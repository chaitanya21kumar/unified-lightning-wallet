//! Wallet database implementation

use ulw_core::{Error, Result};
use rusqlite::Connection;
use std::path::Path;

pub struct WalletDatabase {
    conn: Connection,
}

impl WalletDatabase {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let conn = Connection::open(path).map_err(|e| Error::Storage(e.to_string()))?;
        Ok(Self { conn })
    }

    pub fn init_schema(&self) -> Result<()> {
        self.conn
            .execute_batch(
                "
                CREATE TABLE IF NOT EXISTS payments (
                    payment_hash TEXT PRIMARY KEY,
                    amount_msat INTEGER NOT NULL,
                    direction TEXT NOT NULL,
                    status TEXT NOT NULL,
                    invoice TEXT,
                    created_at TEXT NOT NULL,
                    settled_at TEXT
                );

                CREATE TABLE IF NOT EXISTS channels (
                    channel_id TEXT PRIMARY KEY,
                    counterparty_node_id TEXT NOT NULL,
                    capacity_sats INTEGER NOT NULL,
                    local_balance_msat INTEGER NOT NULL,
                    remote_balance_msat INTEGER NOT NULL,
                    state TEXT NOT NULL
                );
                ",
            )
            .map_err(|e| Error::Storage(e.to_string()))?;
        Ok(())
    }
}
