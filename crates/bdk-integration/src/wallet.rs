//! BDK wallet implementation

use bdk_electrum::electrum_client::{self, ElectrumApi};
use bdk_wallet::{KeychainKind, Wallet};
use bitcoin::{Address, Amount, FeeRate, Network, Txid};
use std::sync::Arc;
use tokio::sync::Mutex;
use ulw_core::{traits::OnChainTransaction, Error, Result};

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

    /// Sync wallet with blockchain
    pub async fn sync(&self) -> Result<()> {
        // Placeholder - full sync would use Electrum client
        // The BDK 1.0 API has changed significantly
        // This would need to be implemented with the new FullScanRequest/SyncRequest API
        tracing::info!("Sync functionality to be implemented with BDK 1.0 API");
        Ok(())
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

    /// Send transaction
    pub async fn send(&self, address: Address, amount: Amount) -> Result<Txid> {
        let mut wallet = self.wallet.lock().await;

        let mut tx_builder = wallet.build_tx();
        tx_builder
            .add_recipient(address.script_pubkey(), amount)
            .fee_rate(FeeRate::from_sat_per_vb_unchecked(1));

        let mut psbt = tx_builder
            .finish()
            .map_err(|e| Error::Internal(e.to_string()))?;

        wallet
            .sign(&mut psbt, Default::default())
            .map_err(|e| Error::Internal(e.to_string()))?;

        let tx = psbt
            .extract_tx()
            .map_err(|e| Error::Internal(e.to_string()))?;

        let txid = tx.compute_txid();

        // Broadcast via Electrum
        let client = electrum_client::Client::new(&self.electrum_url)
            .map_err(|e| Error::Network(e.to_string()))?;

        client
            .transaction_broadcast(&tx)
            .map_err(|e| Error::Network(e.to_string()))?;

        Ok(txid)
    }

    /// List all transactions
    pub async fn list_transactions(&self) -> Result<Vec<OnChainTransaction>> {
        let wallet = self.wallet.lock().await;
        let mut transactions = Vec::new();

        for canonical_tx in wallet.transactions() {
            // Simplified version - would need proper calculation of amounts
            transactions.push(OnChainTransaction {
                txid: canonical_tx.tx_node.txid,
                received: Amount::ZERO,
                sent: Amount::ZERO,
                fee: None,
                confirmation_time: None, // Simplified for now
            });
        }

        Ok(transactions)
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
