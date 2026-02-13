//! Storage layer usage example
//!
//! This example demonstrates how to:
//! - Create a wallet database
//! - Save and retrieve payments
//! - Save and retrieve channel information
//!
//! To run this example:
//! ```bash
//! cargo run --example storage_example
//! ```

use ulw_core::{
    traits::WalletStorage,
    types::{ChannelInfo, ChannelState, Payment, PaymentDirection, PaymentStatus},
    Result,
};
use ulw_storage::WalletDatabase;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🗄️  Unified Lightning Wallet - Storage Example\n");

    // Create a temporary database for this example
    let temp_dir = std::env::temp_dir();
    let db_path = temp_dir.join("ulw_storage_example.db");

    println!("📂 Creating database at: {}", db_path.display());
    let db = WalletDatabase::new(&db_path)?;
    println!("✅ Database created!\n");

    // Create a sample payment
    println!("💳 Creating sample payment...");
    let payment = Payment {
        payment_hash: "a1b2c3d4e5f6".to_string(),
        amount_msat: 50_000_000, // 50,000 sats
        direction: PaymentDirection::Outbound,
        status: PaymentStatus::Pending,
        invoice: None,
        created_at: chrono::Utc::now(),
        settled_at: None,
    };

    // Save the payment
    db.save_payment(&payment).await?;
    println!("✅ Payment saved!");
    println!("   Hash: {}", payment.payment_hash);
    println!(
        "   Amount: {} msat ({} sats)",
        payment.amount_msat,
        payment.amount_msat / 1000
    );
    println!("   Direction: {:?}", payment.direction);
    println!("   Status: {:?}\n", payment.status);

    // Retrieve all payments
    println!("📋 Retrieving all payments...");
    let payments = db.list_payments().await?;
    println!("Found {} payment(s)\n", payments.len());

    // Get a specific payment
    println!("🔍 Looking up payment by hash...");
    let retrieved = db.get_payment(&payment.payment_hash).await?;
    match retrieved {
        Some(p) => {
            println!("✅ Payment found!");
            println!("   Hash: {}", p.payment_hash);
            println!("   Amount: {} sats", p.amount_msat / 1000);
        }
        None => println!("❌ Payment not found"),
    }
    println!();

    // Create a sample channel
    println!("⚡ Creating sample channel...");
    let channel = ChannelInfo {
        channel_id: "ch_abc123".to_string(),
        counterparty_node_id: "03abcdef...".to_string(),
        capacity_sats: 1_000_000,
        local_balance_msat: 500_000_000,
        remote_balance_msat: 500_000_000,
        state: ChannelState::Active,
    };

    // Save the channel
    db.save_channel(&channel).await?;
    println!("✅ Channel saved!");
    println!("   Channel ID: {}", channel.channel_id);
    println!("   Capacity: {} sats", channel.capacity_sats);
    println!(
        "   Local Balance: {} sats",
        channel.local_balance_msat / 1000
    );
    println!("   State: {:?}\n", channel.state);

    // List all channels
    println!("📋 Retrieving all channels...");
    let channels = db.list_channels().await?;
    println!("Found {} channel(s)\n", channels.len());

    // Clean up
    println!("🧹 Cleaning up...");
    std::fs::remove_file(&db_path).map_err(|e| ulw_core::Error::Internal(e.to_string()))?;
    println!("✅ Database removed\n");

    println!("✨ Storage example complete!");
    println!("\n💡 The storage layer provides:");
    println!("  - Persistent payment tracking");
    println!("  - Channel state management");
    println!("  - Thread-safe SQLite operations");
    println!("  - Automatic schema migrations");

    Ok(())
}
