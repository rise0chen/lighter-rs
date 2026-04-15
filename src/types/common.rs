//! Common types and structures used across transactions

use crate::errors::Result;
use serde::{Deserialize, Serialize};

/// Transaction options for customizing transaction parameters
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransactOpts {
    pub from_account_index: Option<i64>,
    pub api_key_index: Option<u8>,
    #[serde(default)]
    pub expired_at: i64,
    pub nonce: Option<i64>,
    #[serde(default)]
    pub dry_run: bool,
}

/// Trait that all transaction types must implement
pub trait TxInfo {
    /// Get the transaction type identifier
    fn get_tx_type(&self) -> u8;

    /// Get transaction info as JSON string
    fn get_tx_info(&self) -> Result<String>;

    /// Get the transaction hash (if signed)
    fn get_tx_hash(&self) -> Option<String>;

    /// Validate the transaction
    fn validate(&self) -> Result<()>;

    /// Hash the transaction for signing
    fn hash(&self, lighter_chain_id: u32) -> Result<Vec<u8>>;
}

/// Order information structure used in order-related transactions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderInfo {
    pub market_index: i16,
    pub client_order_index: i64,
    pub base_amount: i64,
    pub price: u32,
    pub is_ask: u8,
    pub order_type: u8,
    pub time_in_force: u8,
    pub reduce_only: u8,
    pub trigger_price: u32,
    pub order_expiry: i64,
}
