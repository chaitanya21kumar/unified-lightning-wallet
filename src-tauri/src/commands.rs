use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tauri::State;
use tokio::sync::Mutex;

use ulw_bdk::BdkWallet;

// Application state holding wallet instances
pub struct AppState {
    pub bdk_wallet: Arc<Mutex<Option<BdkWallet>>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            bdk_wallet: Arc::new(Mutex::new(None)),
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

// Lightning commands (placeholders for now)
#[tauri::command]
pub async fn create_lightning_invoice(
    amount_msats: u64,
    description: String,
) -> Result<String, String> {
    log::info!("Creating invoice for {} msats: {}", amount_msats, description);
    // TODO: Implement with LDK
    Ok("lnbc...placeholder_invoice".to_string())
}

#[tauri::command]
pub async fn pay_lightning_invoice(invoice: String) -> Result<String, String> {
    log::info!("Paying invoice: {}", invoice);
    // TODO: Implement with LDK
    Ok("placeholder_payment_hash".to_string())
}
