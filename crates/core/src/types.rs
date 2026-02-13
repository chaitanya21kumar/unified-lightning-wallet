//! Core domain types

use bitcoin::{Amount, Network};
use serde::{Deserialize, Serialize};

/// Wallet balance information
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct Balance {
    /// Confirmed on-chain balance
    pub confirmed: Amount,
    /// Unconfirmed on-chain balance (in mempool)
    pub unconfirmed: Amount,
    /// Available Lightning balance across all channels
    pub lightning_available: Amount,
    /// Total Lightning balance (including pending)
    pub lightning_total: Amount,
}

impl Balance {
    /// Total balance across on-chain and Lightning
    pub fn total(&self) -> Amount {
        self.confirmed + self.unconfirmed + self.lightning_total
    }

    /// Spendable balance (confirmed on-chain + Lightning available)
    pub fn spendable(&self) -> Amount {
        self.confirmed + self.lightning_available
    }
}

impl Default for Balance {
    fn default() -> Self {
        Self {
            confirmed: Amount::ZERO,
            unconfirmed: Amount::ZERO,
            lightning_available: Amount::ZERO,
            lightning_total: Amount::ZERO,
        }
    }
}

/// Payment direction
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum PaymentDirection {
    Inbound,
    Outbound,
}

/// Payment status
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum PaymentStatus {
    Pending,
    Succeeded,
    Failed,
}

/// Payment record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Payment {
    pub payment_hash: String,
    pub amount_msat: u64,
    pub direction: PaymentDirection,
    pub status: PaymentStatus,
    pub invoice: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub settled_at: Option<chrono::DateTime<chrono::Utc>>,
}

/// Channel state
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ChannelState {
    Opening,
    Active,
    Closing,
    Closed,
}

/// Channel information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelInfo {
    pub channel_id: String,
    pub counterparty_node_id: String,
    pub capacity_sats: u64,
    pub local_balance_msat: u64,
    pub remote_balance_msat: u64,
    pub state: ChannelState,
}

/// Network configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    pub network: Network,
    pub electrum_url: String,
    pub lightning_port: u16,
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            network: Network::Regtest,
            electrum_url: "tcp://localhost:50001".to_string(),
            lightning_port: 9735,
        }
    }
}

/// Wallet configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletConfig {
    pub network: NetworkConfig,
    pub data_dir: String,
}

impl Default for WalletConfig {
    fn default() -> Self {
        Self {
            network: NetworkConfig::default(),
            data_dir: ".ulw".to_string(),
        }
    }
}
