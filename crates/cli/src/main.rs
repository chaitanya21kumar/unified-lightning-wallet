//! Unified Lightning Wallet CLI

use clap::Parser;

#[derive(Parser)]
#[command(name = "ulw")]
#[command(about = "Unified Lightning Wallet - Self-custodial Bitcoin Lightning Network wallet", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(clap::Subcommand)]
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
    /// Manage Lightning channels
    Channels,
    /// Create a Lightning invoice
    Invoice {
        /// Amount in satoshis
        amount: u64,
    },
    /// Pay a Lightning invoice
    Pay {
        /// BOLT11 invoice
        invoice: String,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Init) => {
            println!("Initializing wallet...");
            Ok(())
        }
        Some(Commands::Balance) => {
            println!("Fetching balance...");
            Ok(())
        }
        Some(Commands::Receive) => {
            println!("Generating address...");
            Ok(())
        }
        Some(Commands::Send { address, amount }) => {
            println!("Sending {} satoshis to {}", amount, address);
            Ok(())
        }
        Some(Commands::Channels) => {
            println!("Listing channels...");
            Ok(())
        }
        Some(Commands::Invoice { amount }) => {
            println!("Creating invoice for {} satoshis", amount);
            Ok(())
        }
        Some(Commands::Pay { invoice }) => {
            println!("Paying invoice {}", invoice);
            Ok(())
        }
        None => {
            println!("Use --help for usage information");
            Ok(())
        }
    }
}
