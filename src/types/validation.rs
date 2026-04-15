//! Validation utilities for transaction types

use crate::constants::*;
use crate::errors::{LighterError, Result};

/// Validate account index
pub fn validate_account_index(index: i64) -> Result<()> {
    if index < MIN_ACCOUNT_INDEX {
        return Err(LighterError::AccountIndexTooLow(index));
    }
    if index > MAX_ACCOUNT_INDEX {
        return Err(LighterError::AccountIndexTooHigh(index));
    }
    Ok(())
}

/// Validate API key index
pub fn validate_api_key_index(index: u8) -> Result<()> {
    if index > MAX_API_KEY_INDEX {
        return Err(LighterError::ApiKeyIndexTooHigh(index));
    }
    Ok(())
}

/// Validate market index
pub fn validate_market_index(index: i16) -> Result<()> {
    if index > MAX_MARKET_INDEX {
        return Err(LighterError::MarketIndexTooHigh(index));
    }
    Ok(())
}
