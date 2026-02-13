//! BDK wallet implementation

use bdk_wallet::{KeychainKind, Wallet};
use bitcoin::{Address, Amount, Network, Txid};
use std::sync::Arc;
use tokio::sync::Mutex;
use ulw_core::{Error, Result};

pub struct BdkWallet {
    wallet: Arc<Mutex<Wallet>>,
    electrum_url: String,
}

impl BdkWallet {
    /// Create new wallet from descriptors
    pub fn new(
        network: Network,
        descriptor: String,
        change_descriptor: String,
        electrum_url: String,
    ) -> Result<Self> {
        let wallet = Wallet::create(descriptor, change_descriptor)
            .network(network)
            .create_wallet_no_persist()
            .map_err(|e| Error::Internal(e.to_string()))?;

        Ok(Self {
            wallet: Arc::new(Mutex::new(wallet)),
            electrum_url,
        })
    }

    /// Get new receiving address
    pub async fn get_new_address(&self) -> Result<Address> {
        let mut wallet = self.wallet.lock().await;
        let addr = wallet.reveal_next_address(KeychainKind::External);
        Ok(addr.address)
    }

    /// Get balance
    pub async fn get_balance(&self) -> Result<Amount> {
        let wallet = self.wallet.lock().await;
        Ok(wallet.balance().total())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_wallet_creation() {
        let wallet = BdkWallet::new(
            Network::Regtest,
            "wpkh(tprv8ZgxMBicQKsPeZRHk4rTG6orPS2CRNFX3njhUXx5vj9qGog5ZMH4uGReDWN5kCkY3jmWEtWause41CDvBRXD1shKknAMKxT99o9qUTRVC6m/84'/1'/0'/0/*)".to_string(),
            "wpkh(tprv8ZgxMBicQKsPeZRHk4rTG6orPS2CRNFX3njhUXx5vj9qGog5ZMH4uGReDWN5kCkY3jmWEtWause41CDvBRXD1shKknAMKxT99o9qUTRVC6m/84'/1'/0'/1/*)".to_string(),
            "tcp://localhost:50001".to_string(),
        );

        assert!(wallet.is_ok());
    }
}
