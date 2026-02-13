//! Wallet initialization command

use crate::config::WalletConfig;
use bitcoin::Network;
use dialoguer::{Input, Select};
use ulw_core::{types::NetworkConfig, Result};

pub async fn init_wallet() -> Result<()> {
    println!("🌩️  Unified Lightning Wallet - Initialization\n");

    // Select network
    let networks = vec!["Bitcoin (Mainnet)", "Testnet", "Regtest"];
    let network_index = Select::new()
        .with_prompt("Select Bitcoin network")
        .items(&networks)
        .default(2) // Default to Regtest for development
        .interact()
        .map_err(|e| ulw_core::Error::Internal(e.to_string()))?;

    let network = match network_index {
        0 => Network::Bitcoin,
        1 => Network::Testnet,
        2 => Network::Regtest,
        _ => Network::Regtest,
    };

    // Get Electrum URL
    let default_electrum = match network {
        Network::Bitcoin => "ssl://electrum.blockstream.info:50002",
        Network::Testnet => "ssl://electrum.blockstream.info:60002",
        Network::Regtest => "tcp://localhost:50001",
        _ => "tcp://localhost:50001",
    };

    let electrum_url: String = Input::new()
        .with_prompt("Electrum server URL")
        .default(default_electrum.to_string())
        .interact_text()
        .map_err(|e| ulw_core::Error::Internal(e.to_string()))?;

    // Get Lightning port
    let lightning_port: u16 = Input::new()
        .with_prompt("Lightning Network port")
        .default(9735)
        .interact_text()
        .map_err(|e| ulw_core::Error::Internal(e.to_string()))?;

    // Create config
    let mut config = WalletConfig::default();
    config.network = NetworkConfig {
        network,
        electrum_url,
        lightning_port,
    };

    // Ensure data directory exists
    std::fs::create_dir_all(&config.data_dir)
        .map_err(|e| ulw_core::Error::Internal(e.to_string()))?;

    // Save config
    config.save(&config.config_path())?;

    println!("\n✅ Wallet initialized successfully!");
    println!("📁 Data directory: {}", config.data_dir.to_string_lossy());
    println!("🌐 Network: {:?}", config.network.network);
    println!("⚡ Electrum: {}", config.network.electrum_url);
    println!("\nNext steps:");
    println!("  1. Generate a new address: ulw receive");
    println!("  2. Check balance: ulw balance");
    println!("  3. Send funds: ulw send <address> <amount>");

    Ok(())
}
