//! Error types for the wallet

use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Bitcoin error: {0}")]
    Bitcoin(String),

    #[error("Invalid address: {0}")]
    InvalidAddress(String),

    #[error("Insufficient funds: required {required}, available {available}")]
    InsufficientFunds { required: u64, available: u64 },

    #[error("Channel not found: {0}")]
    ChannelNotFound(String),

    #[error("Payment failed: {0}")]
    PaymentFailed(String),

    #[error("Storage error: {0}")]
    Storage(String),

    #[error("Network error: {0}")]
    Network(String),

    #[error("Invalid invoice: {0}")]
    InvalidInvoice(String),

    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),

    #[error("Internal error: {0}")]
    Internal(String),
}

impl From<bitcoin::address::ParseError> for Error {
    fn from(e: bitcoin::address::ParseError) -> Self {
        Error::InvalidAddress(e.to_string())
    }
}
