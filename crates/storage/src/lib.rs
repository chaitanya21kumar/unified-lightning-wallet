//! Storage layer for wallet persistence

pub mod database;
pub mod migrations;

pub use database::WalletDatabase;
