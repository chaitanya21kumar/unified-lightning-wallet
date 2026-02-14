//! Lightning Network command implementations

use bitcoin::Network;
use std::path::PathBuf;
use ulw_core::Result;
use ulw_ldk::LdkNode;

use crate::config::WalletConfig;

/// Create a Lightning node from wallet configuration
async fn create_ldk_node(config: &WalletConfig) -> Result<LdkNode> {
    let ldk_storage = config.data_dir.join("lightning");

    // Use a deterministic seed based on wallet name for now
    // In production, this would be derived from the wallet's mnemonic
    let entropy_seed = derive_entropy_from_name(&config.wallet_name);

    // Create node
    LdkNode::new(
        config.network.network,
        ldk_storage,
        entropy_seed,
    ).await
}

/// Derive a deterministic 32-byte seed from wallet name
/// WARNING: This is for demo purposes only!
/// In production, derive from the BIP39 mnemonic
fn derive_entropy_from_name(wallet_name: &str) -> [u8; 32] {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut hasher = DefaultHasher::new();
    wallet_name.hash(&mut hasher);
    let hash = hasher.finish();

    let mut entropy = [0u8; 32];
    entropy[0..8].copy_from_slice(&hash.to_le_bytes());
    // Fill rest with derivation from wallet name chars
    for (i, byte) in wallet_name.bytes().enumerate() {
        if i + 8 < 32 {
            entropy[i + 8] = byte;
        }
    }
    entropy
}

/// Create a Lightning invoice
pub async fn create_invoice(
    config: &WalletConfig,
    amount_sats: u64,
    description: Option<String>,
) -> Result<()> {
    println!("⚡ Creating Lightning Invoice");

    let node = create_ldk_node(config).await?;

    let amount_msats = amount_sats * 1000;
    let desc = description.unwrap_or_else(|| "Payment request".to_string());

    let invoice = node.create_invoice(Some(amount_msats), desc, 3600).await?;

    println!("\n✅ Invoice created!");
    println!("Amount: {} sats ({} msats)", amount_sats, amount_msats);
    println!("Node ID: {}", node.get_node_id());
    println!("\nInvoice:");
    println!("{}", invoice);
    println!("\n💡 Share this invoice to receive payment");

    Ok(())
}

/// Pay a Lightning invoice
pub async fn pay_invoice(config: &WalletConfig, invoice_str: String) -> Result<()> {
    println!("⚡ Paying Lightning Invoice");

    let node = create_ldk_node(config).await?;

    println!("Parsing invoice...");
    let payment_hash = node.pay_invoice(invoice_str.clone()).await?;

    println!("\n✅ Payment initiated!");
    println!("Payment Hash: {}", hex::encode(payment_hash.0));
    println!("\n⚠️  Note: Full payment routing requires active channels");
    println!("   This is a simplified implementation for testing");

    Ok(())
}

/// List Lightning payment history
pub async fn list_payments(config: &WalletConfig) -> Result<()> {
    println!("⚡ Lightning Payment History");

    let node = create_ldk_node(config).await?;

    let payments = node.list_payments().await?;

    if payments.is_empty() {
        println!("No Lightning payments yet");
        return Ok(());
    }

    println!("Found {} payment(s):\n", payments.len());
    for (i, payment) in payments.iter().enumerate() {
        println!("{}. Payment Hash: {}", i + 1, hex::encode(payment.payment_hash));
        if let Some(amt) = payment.amount_msat {
            println!("   Amount: {} msats ({} sats)", amt, amt / 1000);
        }
        println!("   Status: {:?}", payment.status);
        println!();
    }

    Ok(())
}

/// Get Lightning node information
pub async fn get_node_info(config: &WalletConfig) -> Result<()> {
    println!("⚡ Lightning Node Information");

    let node = create_ldk_node(config).await?;

    let info = node.get_info();

    println!("\nNode ID: {}", info.node_id);
    println!("Network: {:?}", info.network);
    println!("Version: {}", info.version);
    println!("\n💡 Use this Node ID to receive channel open requests");

    Ok(())
}
