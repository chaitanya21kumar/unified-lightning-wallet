//! Unified Lightning Wallet CLI

mod commands;
mod config;

use clap::{Parser, Subcommand};
use config::WalletConfig;
use tracing_subscriber::EnvFilter;
use ulw_bdk::BdkWallet;
use ulw_core::Result;

#[derive(Parser)]
#[command(name = "ulw")]
#[command(about = "Unified Lightning Wallet - Self-custodial Bitcoin Lightning Network wallet", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Enable verbose logging
    #[arg(short, long, global = true)]
    verbose: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new wallet
    Init,

    /// Get wallet balance
    Balance,

    /// Generate a new receiving address
    Receive,

    /// Send on-chain payment
    Send {
        /// Bitcoin address to send to
        address: String,
        /// Amount in satoshis
        amount: u64,
    },

    /// Sync wallet with blockchain
    Sync,

    /// List transactions
    Transactions {
        /// Number of transactions to show
        #[arg(short, long, default_value = "10")]
        limit: usize,
    },

    /// Manage Lightning channels
    Channels {
        #[command(subcommand)]
        action: Option<ChannelCommands>,
    },

    /// Create a Lightning invoice
    Invoice {
        /// Amount in satoshis
        amount: u64,
        /// Invoice description
        #[arg(short, long)]
        description: Option<String>,
    },

    /// Pay a Lightning invoice
    Pay {
        /// BOLT11 invoice
        invoice: String,
    },
}

#[derive(Subcommand)]
enum ChannelCommands {
    /// List all channels
    List,
    /// Open a new channel
    Open {
        /// Node public key
        node_id: String,
        /// Channel capacity in satoshis
        amount: u64,
    },
    /// Close a channel
    Close {
        /// Channel ID
        channel_id: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Setup logging
    let filter = if cli.verbose {
        EnvFilter::new("debug")
    } else {
        EnvFilter::new("info")
    };

    tracing_subscriber::fmt().with_env_filter(filter).init();

    match cli.command {
        Commands::Init => {
            commands::init_wallet().await?;
        }
        Commands::Balance => {
            let config = load_config()?;
            let wallet = create_bdk_wallet(&config).await?;
            let balance = wallet.get_balance().await?;
            println!("💰 Wallet Balance");
            println!("  Total: {} sats", balance.to_sat());
        }
        Commands::Receive => {
            let config = load_config()?;
            let wallet = create_bdk_wallet(&config).await?;
            let address = wallet.get_new_address().await?;
            println!("📬 New Receiving Address");
            println!("{}", address);
        }
        Commands::Send { address, amount } => {
            let config = load_config()?;
            let wallet = create_bdk_wallet(&config).await?;

            println!("📤 Sending {} satoshis to {}", amount, address);

            let addr: bitcoin::Address = address
                .parse::<bitcoin::Address<bitcoin::address::NetworkUnchecked>>()
                .map_err(|e| ulw_core::Error::InvalidAddress(e.to_string()))?
                .require_network(config.network.network)
                .map_err(|e| ulw_core::Error::InvalidAddress(e.to_string()))?;

            let txid = wallet.send(addr, bitcoin::Amount::from_sat(amount)).await?;
            println!("✅ Transaction broadcast!");
            println!("TXID: {}", txid);
        }
        Commands::Sync => {
            let config = load_config()?;
            let wallet = create_bdk_wallet(&config).await?;
            println!("🔄 Syncing wallet with blockchain...");
            wallet.sync().await?;
            println!("✅ Sync complete!");
        }
        Commands::Transactions { limit } => {
            let config = load_config()?;
            let wallet = create_bdk_wallet(&config).await?;
            let txs = wallet.list_transactions().await?;

            println!("📜 Recent Transactions (showing up to {})", limit);
            for (i, tx) in txs.iter().take(limit).enumerate() {
                println!("\n{}. TXID: {}", i + 1, tx.txid);
                println!("   Received: {} sats", tx.received.to_sat());
                println!("   Sent: {} sats", tx.sent.to_sat());
                if let Some(fee) = tx.fee {
                    println!("   Fee: {} sats", fee.to_sat());
                }
                if let Some(time) = tx.confirmation_time {
                    println!("   Confirmed: {}", time);
                } else {
                    println!("   Status: Unconfirmed");
                }
            }
        }
        Commands::Channels { action } => match action {
            None | Some(ChannelCommands::List) => {
                println!("⚡ Lightning Channels");
                println!("No channels yet. Use 'ulw channels open' to create one.");
            }
            Some(ChannelCommands::Open { node_id, amount }) => {
                println!("Opening channel to {} with {} satoshis", node_id, amount);
                println!("⚠️  Lightning functionality coming soon!");
            }
            Some(ChannelCommands::Close { channel_id }) => {
                println!("Closing channel {}", channel_id);
                println!("⚠️  Lightning functionality coming soon!");
            }
        },
        Commands::Invoice {
            amount,
            description,
        } => {
            println!("Creating invoice for {} satoshis", amount);
            if let Some(desc) = description {
                println!("Description: {}", desc);
            }
            println!("⚠️  Lightning functionality coming soon!");
        }
        Commands::Pay { invoice } => {
            println!("Paying invoice: {}", invoice);
            println!("⚠️  Lightning functionality coming soon!");
        }
    }

    Ok(())
}

fn load_config() -> Result<WalletConfig> {
    let config = WalletConfig::default();
    let config_path = config.config_path();

    if !config_path.exists() {
        return Err(ulw_core::Error::InvalidConfig(
            "Wallet not initialized. Run 'ulw init' first.".to_string(),
        ));
    }

    WalletConfig::load(&config_path)
}

async fn create_bdk_wallet(config: &WalletConfig) -> Result<BdkWallet> {
    // For now, use hardcoded test descriptors
    // In production, these would be derived from a mnemonic stored securely
    let descriptor = "wpkh(tprv8ZgxMBicQKsPeZRHk4rTG6orPS2CRNFX3njhUXx5vj9qGog5ZMH4uGReDWN5kCkY3jmWEtWause41CDvBRXD1shKknAMKxT99o9qUTRVC6m/84'/1'/0'/0/*)".to_string();
    let change_descriptor = "wpkh(tprv8ZgxMBicQKsPeZRHk4rTG6orPS2CRNFX3njhUXx5vj9qGog5ZMH4uGReDWN5kCkY3jmWEtWause41CDvBRXD1shKknAMKxT99o9qUTRVC6m/84'/1'/0'/1/*)".to_string();

    BdkWallet::new(
        config.network.network,
        descriptor,
        change_descriptor,
        config.network.electrum_url.clone(),
    )
}
