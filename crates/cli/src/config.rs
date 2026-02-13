//! Wallet configuration and management

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use ulw_core::{types::NetworkConfig, Error, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletConfig {
    pub data_dir: PathBuf,
    pub network: NetworkConfig,
    pub wallet_name: String,
}

impl WalletConfig {
    pub fn new(data_dir: PathBuf, network: NetworkConfig) -> Self {
        Self {
            data_dir,
            network,
            wallet_name: "default".to_string(),
        }
    }

    pub fn load(config_path: &Path) -> Result<Self> {
        let content = std::fs::read_to_string(config_path)
            .map_err(|e| Error::InvalidConfig(e.to_string()))?;
        serde_json::from_str(&content).map_err(|e| Error::InvalidConfig(e.to_string()))
    }

    pub fn save(&self, config_path: &Path) -> Result<()> {
        let content =
            serde_json::to_string_pretty(self).map_err(|e| Error::Internal(e.to_string()))?;
        std::fs::write(config_path, content).map_err(|e| Error::Internal(e.to_string()))?;
        Ok(())
    }

    pub fn database_path(&self) -> PathBuf {
        self.data_dir.join(format!("{}.db", self.wallet_name))
    }

    pub fn config_path(&self) -> PathBuf {
        self.data_dir.join("config.json")
    }
}

impl Default for WalletConfig {
    fn default() -> Self {
        let home_dir = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        Self {
            data_dir: home_dir.join(".ulw"),
            network: NetworkConfig::default(),
            wallet_name: "default".to_string(),
        }
    }
}
