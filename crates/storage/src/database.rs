//! Wallet database implementation

use rusqlite::{params, Connection, OptionalExtension};
use std::path::Path;
use std::sync::Mutex;
use ulw_core::{
    traits::WalletStorage,
    types::{ChannelInfo, Payment, PaymentDirection, PaymentStatus},
    Error, Result,
};

pub struct WalletDatabase {
    conn: Mutex<Connection>,
}

impl WalletDatabase {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let conn = Connection::open(path).map_err(|e| Error::Storage(e.to_string()))?;
        let db = Self {
            conn: Mutex::new(conn),
        };
        db.init_schema()?;
        Ok(db)
    }

    fn init_schema(&self) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute_batch(
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

                CREATE INDEX IF NOT EXISTS idx_payments_created ON payments(created_at DESC);
                CREATE INDEX IF NOT EXISTS idx_channels_state ON channels(state);
                ",
        )
        .map_err(|e| Error::Storage(e.to_string()))?;
        Ok(())
    }
}

#[async_trait::async_trait]
impl WalletStorage for WalletDatabase {
    async fn save_payment(&self, payment: &Payment) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT OR REPLACE INTO payments
                (payment_hash, amount_msat, direction, status, invoice, created_at, settled_at)
                VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                payment.payment_hash,
                payment.amount_msat as i64,
                match payment.direction {
                    PaymentDirection::Inbound => "inbound",
                    PaymentDirection::Outbound => "outbound",
                },
                match payment.status {
                    PaymentStatus::Pending => "pending",
                    PaymentStatus::Succeeded => "succeeded",
                    PaymentStatus::Failed => "failed",
                },
                payment.invoice,
                payment.created_at.to_rfc3339(),
                payment.settled_at.as_ref().map(|t| t.to_rfc3339()),
            ],
        )
        .map_err(|e| Error::Storage(e.to_string()))?;
        Ok(())
    }

    async fn get_payment(&self, payment_hash: &str) -> Result<Option<Payment>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
                "SELECT payment_hash, amount_msat, direction, status, invoice, created_at, settled_at
                FROM payments WHERE payment_hash = ?1",
            )
            .map_err(|e| Error::Storage(e.to_string()))?;

        let payment = stmt
            .query_row(params![payment_hash], |row| {
                Ok(Payment {
                    payment_hash: row.get(0)?,
                    amount_msat: row.get::<_, i64>(1)? as u64,
                    direction: match row.get::<_, String>(2)?.as_str() {
                        "inbound" => PaymentDirection::Inbound,
                        _ => PaymentDirection::Outbound,
                    },
                    status: match row.get::<_, String>(3)?.as_str() {
                        "pending" => PaymentStatus::Pending,
                        "succeeded" => PaymentStatus::Succeeded,
                        _ => PaymentStatus::Failed,
                    },
                    invoice: row.get(4)?,
                    created_at: chrono::DateTime::parse_from_rfc3339(&row.get::<_, String>(5)?)
                        .unwrap()
                        .into(),
                    settled_at: row
                        .get::<_, Option<String>>(6)?
                        .and_then(|s| chrono::DateTime::parse_from_rfc3339(&s).ok())
                        .map(|dt| dt.into()),
                })
            })
            .optional()
            .map_err(|e| Error::Storage(e.to_string()))?;

        Ok(payment)
    }

    async fn list_payments(&self) -> Result<Vec<Payment>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
                "SELECT payment_hash, amount_msat, direction, status, invoice, created_at, settled_at
                FROM payments ORDER BY created_at DESC",
            )
            .map_err(|e| Error::Storage(e.to_string()))?;

        let payments = stmt
            .query_map([], |row| {
                Ok(Payment {
                    payment_hash: row.get(0)?,
                    amount_msat: row.get::<_, i64>(1)? as u64,
                    direction: match row.get::<_, String>(2)?.as_str() {
                        "inbound" => PaymentDirection::Inbound,
                        _ => PaymentDirection::Outbound,
                    },
                    status: match row.get::<_, String>(3)?.as_str() {
                        "pending" => PaymentStatus::Pending,
                        "succeeded" => PaymentStatus::Succeeded,
                        _ => PaymentStatus::Failed,
                    },
                    invoice: row.get(4)?,
                    created_at: chrono::DateTime::parse_from_rfc3339(&row.get::<_, String>(5)?)
                        .unwrap()
                        .into(),
                    settled_at: row
                        .get::<_, Option<String>>(6)?
                        .and_then(|s| chrono::DateTime::parse_from_rfc3339(&s).ok())
                        .map(|dt| dt.into()),
                })
            })
            .map_err(|e| Error::Storage(e.to_string()))?
            .collect::<std::result::Result<Vec<_>, _>>()
            .map_err(|e| Error::Storage(e.to_string()))?;

        Ok(payments)
    }

    async fn save_channel(&self, channel: &ChannelInfo) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
                "INSERT OR REPLACE INTO channels
                (channel_id, counterparty_node_id, capacity_sats, local_balance_msat, remote_balance_msat, state)
                VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                params![
                    channel.channel_id,
                    channel.counterparty_node_id,
                    channel.capacity_sats as i64,
                    channel.local_balance_msat as i64,
                    channel.remote_balance_msat as i64,
                    format!("{:?}", channel.state).to_lowercase(),
                ],
            )
            .map_err(|e| Error::Storage(e.to_string()))?;
        Ok(())
    }

    async fn get_channel(&self, channel_id: &str) -> Result<Option<ChannelInfo>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
                "SELECT channel_id, counterparty_node_id, capacity_sats, local_balance_msat, remote_balance_msat, state
                FROM channels WHERE channel_id = ?1",
            )
            .map_err(|e| Error::Storage(e.to_string()))?;

        let channel = stmt
            .query_row(params![channel_id], |row| {
                Ok(ChannelInfo {
                    channel_id: row.get(0)?,
                    counterparty_node_id: row.get(1)?,
                    capacity_sats: row.get::<_, i64>(2)? as u64,
                    local_balance_msat: row.get::<_, i64>(3)? as u64,
                    remote_balance_msat: row.get::<_, i64>(4)? as u64,
                    state: match row.get::<_, String>(5)?.as_str() {
                        "opening" => ulw_core::types::ChannelState::Opening,
                        "active" => ulw_core::types::ChannelState::Active,
                        "closing" => ulw_core::types::ChannelState::Closing,
                        _ => ulw_core::types::ChannelState::Closed,
                    },
                })
            })
            .optional()
            .map_err(|e| Error::Storage(e.to_string()))?;

        Ok(channel)
    }

    async fn list_channels(&self) -> Result<Vec<ChannelInfo>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
                "SELECT channel_id, counterparty_node_id, capacity_sats, local_balance_msat, remote_balance_msat, state
                FROM channels ORDER BY channel_id",
            )
            .map_err(|e| Error::Storage(e.to_string()))?;

        let channels = stmt
            .query_map([], |row| {
                Ok(ChannelInfo {
                    channel_id: row.get(0)?,
                    counterparty_node_id: row.get(1)?,
                    capacity_sats: row.get::<_, i64>(2)? as u64,
                    local_balance_msat: row.get::<_, i64>(3)? as u64,
                    remote_balance_msat: row.get::<_, i64>(4)? as u64,
                    state: match row.get::<_, String>(5)?.as_str() {
                        "opening" => ulw_core::types::ChannelState::Opening,
                        "active" => ulw_core::types::ChannelState::Active,
                        "closing" => ulw_core::types::ChannelState::Closing,
                        _ => ulw_core::types::ChannelState::Closed,
                    },
                })
            })
            .map_err(|e| Error::Storage(e.to_string()))?
            .collect::<std::result::Result<Vec<_>, _>>()
            .map_err(|e| Error::Storage(e.to_string()))?;

        Ok(channels)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[tokio::test]
    async fn test_payment_crud() {
        let temp_file = NamedTempFile::new().unwrap();
        let db = WalletDatabase::new(temp_file.path()).unwrap();

        let payment = Payment {
            payment_hash: "test_hash".to_string(),
            amount_msat: 1000,
            direction: PaymentDirection::Outbound,
            status: PaymentStatus::Pending,
            invoice: Some("lnbc...".to_string()),
            created_at: chrono::Utc::now(),
            settled_at: None,
        };

        // Save payment
        db.save_payment(&payment).await.unwrap();

        // Retrieve payment
        let retrieved = db.get_payment("test_hash").await.unwrap().unwrap();
        assert_eq!(retrieved.payment_hash, "test_hash");
        assert_eq!(retrieved.amount_msat, 1000);

        // List payments
        let payments = db.list_payments().await.unwrap();
        assert_eq!(payments.len(), 1);
    }
}
