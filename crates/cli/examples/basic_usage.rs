//! Basic usage example for Unified Lightning Wallet
//!
//! This example demonstrates how to:
//! - Create a new BDK wallet
//! - Generate a receiving address
//! - Check balance
//! - Send an on-chain transaction
//!
//! To run this example:
//! ```bash
//! cargo run --example basic_usage
//! ```

use bitcoin::Network;
use ulw_bdk::BdkWallet;
use ulw_core::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt().with_env_filter("info").init();

    println!("🌩️  Unified Lightning Wallet - Basic Usage Example\n");

    // For this example, we'll use regtest network
    let network = Network::Regtest;
    let electrum_url = "tcp://localhost:50001".to_string();

    // Test descriptors (DO NOT use in production!)
    // In a real application, generate these from a BIP39 mnemonic
    let descriptor = "wpkh(tprv8ZgxMBicQKsPeZRHk4rTG6orPS2CRNFX3njhUXx5vj9qGog5ZMH4uGReDWN5kCkY3jmWEtWause41CDvBRXD1shKknAMKxT99o9qUTRVC6m/84'/1'/0'/0/*)".to_string();
    let change_descriptor = "wpkh(tprv8ZgxMBicQKsPeZRHk4rTG6orPS2CRNFX3njhUXx5vj9qGog5ZMH4uGReDWN5kCkY3jmWEtWause41CDvBRXD1shKknAMKxT99o9qUTRVC6m/84'/1'/0'/1/*)".to_string();

    println!("📝 Creating wallet...");
    let wallet = BdkWallet::new(network, descriptor, change_descriptor, electrum_url)?;
    println!("✅ Wallet created successfully!\n");

    // Generate a receiving address
    println!("📬 Generating receiving address...");
    let address = wallet.get_new_address().await?;
    println!("Address: {}\n", address);

    // Check balance
    println!("💰 Checking balance...");
    let balance = wallet.get_balance().await?;
    println!("Total balance: {} sats", balance.to_sat());
    println!("  (This will be 0 for a new wallet)\n");

    // Sync with blockchain
    println!("🔄 Syncing with blockchain...");
    wallet.sync().await?;
    println!("✅ Sync complete!\n");

    // Example: How to send (commented out as it requires funds)
    println!("📤 To send Bitcoin:");
    println!("  let recipient = \"bcrt1q...\".parse().unwrap();");
    println!("  let amount = Amount::from_sat(10000);");
    println!("  let txid = wallet.send(recipient, amount).await?;");
    println!("  println!(\"Transaction sent: {{}}\", txid);\n");

    // List transactions
    println!("📜 Listing transactions...");
    let txs = wallet.list_transactions().await?;
    if txs.is_empty() {
        println!("No transactions yet.\n");
    } else {
        for (i, tx) in txs.iter().enumerate() {
            println!("{}. TXID: {}", i + 1, tx.txid);
            println!("   Received: {} sats", tx.received.to_sat());
            println!("   Sent: {} sats\n", tx.sent.to_sat());
        }
    }

    println!("✨ Example complete!");
    println!("\n💡 Next steps:");
    println!("  1. Run 'ulw init' to create your own wallet");
    println!("  2. Use 'ulw receive' to get an address");
    println!("  3. Send testnet/regtest coins to the address");
    println!("  4. Check balance with 'ulw balance'");
    println!("  5. Send coins with 'ulw send <address> <amount>'");

    Ok(())
}
