use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Arc;
use tauri::State;
use tokio::sync::Mutex;

use ulw_bdk::BdkWallet;
use ulw_ldk::LdkNode;

// Application state holding wallet instances
pub struct AppState {
    pub bdk_wallet: Arc<Mutex<Option<BdkWallet>>>,
    pub ldk_node: Arc<Mutex<Option<LdkNode>>>,
    pub data_dir: PathBuf,
}

impl AppState {
    pub fn new() -> Self {
        let home_dir = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        let data_dir = home_dir.join(".ulw");

        Self {
            bdk_wallet: Arc::new(Mutex::new(None)),
            ldk_node: Arc::new(Mutex::new(None)),
            data_dir,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct InitWalletParams {
    pub network: String,
    pub descriptor: String,
    pub change_descriptor: String,
    pub electrum_url: String,
}

// Initialize wallet command
#[tauri::command]
pub async fn init_wallet(
    params: InitWalletParams,
    state: State<'_, AppState>,
) -> Result<String, String> {
    log::info!("Initializing wallet with network: {}", params.network);

    let network = match params.network.as_str() {
        "bitcoin" => bitcoin::Network::Bitcoin,
        "testnet" => bitcoin::Network::Testnet,
        "regtest" => bitcoin::Network::Regtest,
        _ => bitcoin::Network::Regtest,
    };

    let wallet = BdkWallet::new(
        network,
        params.descriptor,
        params.change_descriptor,
        params.electrum_url,
    )
    .map_err(|e| e.to_string())?;

    let mut wallet_guard = state.bdk_wallet.lock().await;
    *wallet_guard = Some(wallet);

    Ok("Wallet initialized successfully".to_string())
}

// Get wallet balance
#[tauri::command]
pub async fn get_balance(state: State<'_, AppState>) -> Result<u64, String> {
    let wallet_guard = state.bdk_wallet.lock().await;

    if let Some(wallet) = wallet_guard.as_ref() {
        let balance = wallet.get_balance().await.map_err(|e| e.to_string())?;
        Ok(balance.to_sat())
    } else {
        Err("Wallet not initialized".to_string())
    }
}

// Get new receiving address
#[tauri::command]
pub async fn get_new_address(state: State<'_, AppState>) -> Result<String, String> {
    let wallet_guard = state.bdk_wallet.lock().await;

    if let Some(wallet) = wallet_guard.as_ref() {
        let address = wallet.get_new_address().await.map_err(|e| e.to_string())?;
        Ok(address.to_string())
    } else {
        Err("Wallet not initialized".to_string())
    }
}

// Send Bitcoin
#[tauri::command]
pub async fn send_bitcoin(
    address: String,
    amount_sats: u64,
    state: State<'_, AppState>,
) -> Result<String, String> {
    let wallet_guard = state.bdk_wallet.lock().await;

    if let Some(wallet) = wallet_guard.as_ref() {
        let addr: bitcoin::Address = address
            .parse::<bitcoin::Address<bitcoin::address::NetworkUnchecked>>()
            .map_err(|e| e.to_string())?
            .assume_checked();

        let amount = bitcoin::Amount::from_sat(amount_sats);
        let txid = wallet.send(addr, amount).await.map_err(|e| e.to_string())?;
        Ok(txid.to_string())
    } else {
        Err("Wallet not initialized".to_string())
    }
}

// Sync wallet with blockchain
#[tauri::command]
pub async fn sync_wallet(state: State<'_, AppState>) -> Result<(), String> {
    let wallet_guard = state.bdk_wallet.lock().await;

    if let Some(wallet) = wallet_guard.as_ref() {
        wallet.sync().await.map_err(|e| e.to_string())?;
        Ok(())
    } else {
        Err("Wallet not initialized".to_string())
    }
}

// Get transaction list
#[tauri::command]
pub async fn list_transactions(state: State<'_, AppState>) -> Result<Vec<String>, String> {
    let wallet_guard = state.bdk_wallet.lock().await;

    if let Some(wallet) = wallet_guard.as_ref() {
        let txs = wallet.list_transactions().await.map_err(|e| e.to_string())?;
        let tx_strings: Vec<String> = txs
            .iter()
            .map(|tx| format!("TXID: {} | Received: {} sats | Sent: {} sats",
                tx.txid,
                tx.received.to_sat(),
                tx.sent.to_sat()
            ))
            .collect();
        Ok(tx_strings)
    } else {
        Err("Wallet not initialized".to_string())
    }
}

// Lightning commands
#[tauri::command]
pub async fn create_lightning_invoice(
    amount_msats: u64,
    description: String,
    state: State<'_, AppState>,
) -> Result<String, String> {
    log::info!("Creating invoice for {} msats: {}", amount_msats, description);

    // Initialize LDK node if not already initialized
    let mut node_guard = state.ldk_node.lock().await;

    if node_guard.is_none() {
        // Create Lightning node with demo entropy
        // In production, derive from BDK wallet mnemonic
        let entropy = derive_demo_entropy();
        let lightning_dir = state.data_dir.join("lightning");

        let node = LdkNode::new(
            bitcoin::Network::Regtest,  // TODO: Get from wallet config
            lightning_dir,
            entropy,
        )
        .await
        .map_err(|e| format!("Failed to create LDK node: {}", e))?;

        *node_guard = Some(node);
    }

    // Create invoice
    if let Some(node) = node_guard.as_ref() {
        let invoice = node
            .create_invoice(Some(amount_msats), description, 3600)
            .await
            .map_err(|e| e.to_string())?;

        Ok(invoice)
    } else {
        Err("Lightning node not initialized".to_string())
    }
}

#[tauri::command]
pub async fn pay_lightning_invoice(
    invoice: String,
    state: State<'_, AppState>,
) -> Result<String, String> {
    log::info!("Paying invoice: {}", invoice);

    let node_guard = state.ldk_node.lock().await;

    if let Some(node) = node_guard.as_ref() {
        let payment_hash = node
            .pay_invoice(invoice)
            .await
            .map_err(|e| e.to_string())?;

        Ok(hex::encode(payment_hash.0))
    } else {
        Err("Lightning node not initialized. Create an invoice first.".to_string())
    }
}

// Helper function to derive demo entropy
// WARNING: This is for demo purposes only!
// In production, derive from BDK wallet mnemonic
fn derive_demo_entropy() -> [u8; 32] {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let wallet_name = "tauri_wallet";
    let mut hasher = DefaultHasher::new();
    wallet_name.hash(&mut hasher);
    let hash = hasher.finish();

    let mut entropy = [0u8; 32];
    entropy[0..8].copy_from_slice(&hash.to_le_bytes());
    // Fill rest with derivation
    for (i, byte) in wallet_name.bytes().enumerate() {
        if i + 8 < 32 {
            entropy[i + 8] = byte;
        }
    }
    entropy
}
