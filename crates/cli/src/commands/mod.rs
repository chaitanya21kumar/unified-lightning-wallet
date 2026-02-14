//! CLI command modules

pub mod init;
pub mod lightning;

pub use init::init_wallet;
pub use lightning::{create_invoice, pay_invoice};
