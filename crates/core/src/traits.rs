//! Core traits for wallet components

use crate::{types::*, Result};
use async_trait::async_trait;
use bitcoin::{Address, Txid};

/// Trait for on-chain wallet operations
#[async_trait]
pub trait OnChainWallet: Send + Sync {
    /// Get a new receiving address
    async fn get_new_address(&self) -> Result<Address>;

    /// Get the current balance
    async fn get_balance(&self) -> Result<bitcoin::Amount>;

    /// Send on-chain transaction
    async fn send(&self, address: Address, amount: bitcoin::Amount) -> Result<Txid>;

    /// Sync with the blockchain
    async fn sync(&self) -> Result<()>;

    /// List all transactions
    async fn list_transactions(&self) -> Result<Vec<OnChainTransaction>>;
}

/// On-chain transaction record
#[derive(Debug, Clone)]
pub struct OnChainTransaction {
    pub txid: Txid,
    pub received: bitcoin::Amount,
    pub sent: bitcoin::Amount,
    pub fee: Option<bitcoin::Amount>,
    pub confirmation_time: Option<chrono::DateTime<chrono::Utc>>,
}

/// Trait for Lightning Network operations
#[async_trait]
pub trait LightningNode: Send + Sync {
    /// Get node public key
    fn node_id(&self) -> String;

    /// Connect to a peer
    async fn connect_peer(&self, node_id: String, addr: String) -> Result<()>;

    /// Open a new channel
    async fn open_channel(&self, node_id: String, amount_sats: u64) -> Result<String>;

    /// Close a channel
    async fn close_channel(&self, channel_id: String) -> Result<()>;

    /// List all channels
    async fn list_channels(&self) -> Result<Vec<ChannelInfo>>;

    /// Create an invoice
    async fn create_invoice(&self, amount_msat: u64, description: String) -> Result<String>;

    /// Pay an invoice
    async fn pay_invoice(&self, invoice: String) -> Result<Payment>;

    /// Get Lightning balance
    async fn get_balance(&self) -> Result<(bitcoin::Amount, bitcoin::Amount)>;
}

/// Trait for wallet storage
#[async_trait]
pub trait WalletStorage: Send + Sync {
    /// Save payment record
    async fn save_payment(&self, payment: &Payment) -> Result<()>;

    /// Get payment by hash
    async fn get_payment(&self, payment_hash: &str) -> Result<Option<Payment>>;

    /// List all payments
    async fn list_payments(&self) -> Result<Vec<Payment>>;

    /// Save channel info
    async fn save_channel(&self, channel: &ChannelInfo) -> Result<()>;

    /// Get channel by ID
    async fn get_channel(&self, channel_id: &str) -> Result<Option<ChannelInfo>>;

    /// List all channels
    async fn list_channels(&self) -> Result<Vec<ChannelInfo>>;
}
