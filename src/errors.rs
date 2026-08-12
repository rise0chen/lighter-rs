//! Error types for the Lighter Protocol SDK

use thiserror::Error;

/// Result type alias using LighterError
pub type Result<T> = std::result::Result<T, LighterError>;

/// Main error type for the Lighter SDK
#[derive(Error, Debug)]
pub enum LighterError {
    // Account and API Key Errors
    #[error(
        "Account index {0} is too low, minimum is {min}",
        min = crate::constants::MIN_ACCOUNT_INDEX
    )]
    AccountIndexTooLow(i64),

    #[error(
        "Account index {0} is too high, maximum is {max}",
        max = crate::constants::MAX_ACCOUNT_INDEX
    )]
    AccountIndexTooHigh(i64),

    #[error(
        "API key index {0} is too low, minimum is {min}",
        min = crate::constants::MIN_API_KEY_INDEX
    )]
    ApiKeyIndexTooLow(u8),

    #[error(
        "API key index {0} is too high, maximum is {max}",
        max = crate::constants::MAX_API_KEY_INDEX
    )]
    ApiKeyIndexTooHigh(u8),

    // Market Errors
    #[error(
        "Market index {0} is too low, minimum is {min}",
        min = crate::constants::MIN_MARKET_INDEX
    )]
    MarketIndexTooLow(i16),

    #[error(
        "Market index {0} is too high, maximum is {max}",
        max = crate::constants::MAX_MARKET_INDEX
    )]
    MarketIndexTooHigh(i16),

    #[error("Market index mismatch")]
    MarketIndexMismatch,

    // Order Errors
    #[error(
        "Client order index {0} is too low, minimum is {min}",
        min = crate::constants::MIN_CLIENT_ORDER_INDEX
    )]
    ClientOrderIndexTooLow(i64),

    #[error(
        "Client order index {0} is too high, maximum is {max}",
        max = crate::constants::MAX_CLIENT_ORDER_INDEX
    )]
    ClientOrderIndexTooHigh(i64),

    #[error("Client order index should be nil")]
    ClientOrderIndexNotNil,

    #[error(
        "Order index {0} is too low, minimum is {min}",
        min = crate::constants::MIN_ORDER_INDEX
    )]
    OrderIndexTooLow(i64),

    #[error(
        "Order index {0} is too high, maximum is {max}",
        max = crate::constants::MAX_ORDER_INDEX
    )]
    OrderIndexTooHigh(i64),

    #[error(
        "Base amount {0} is too low, minimum is {min}",
        min = crate::constants::MIN_ORDER_BASE_AMOUNT
    )]
    BaseAmountTooLow(i64),

    #[error(
        "Base amount {0} is too high, maximum is {max}",
        max = crate::constants::MAX_ORDER_BASE_AMOUNT
    )]
    BaseAmountTooHigh(i64),

    #[error("Base amounts are not equal")]
    BaseAmountsNotEqual,

    #[error("Base amount should be nil")]
    BaseAmountNotNil,

    #[error(
        "Order price {0} is too low, minimum is {min}",
        min = crate::constants::MIN_ORDER_PRICE
    )]
    PriceTooLow(u32),

    #[error(
        "Order price {0} is too high, maximum is {max}",
        max = crate::constants::MAX_ORDER_PRICE
    )]
    PriceTooHigh(u32),

    #[error("IsAsk should be 0 or 1")]
    IsAskInvalid,

    #[error("Order type is invalid")]
    OrderTypeInvalid,

    #[error("Order time-in-force is invalid")]
    OrderTimeInForceInvalid,

    #[error("Order reduce-only flag is invalid")]
    OrderReduceOnlyInvalid,

    #[error("Order trigger price is invalid")]
    OrderTriggerPriceInvalid,

    #[error("Order expiry is invalid")]
    OrderExpiryInvalid,

    #[error("Grouping type is invalid")]
    GroupingTypeInvalid,

    #[error("Order group size is invalid")]
    OrderGroupSizeInvalid,

    // Pool Errors
    #[error(
        "Public pool index {0} is too low, minimum is {min}",
        min = crate::constants::MIN_ACCOUNT_INDEX
    )]
    PublicPoolIndexTooLow(i64),

    #[error(
        "Public pool index {0} is too high, maximum is {max}",
        max = crate::constants::MAX_ACCOUNT_INDEX
    )]
    PublicPoolIndexTooHigh(i64),

    #[error(
        "Pool operator fee is invalid, should be 0 to {max}",
        max = crate::constants::FEE_TICK
    )]
    InvalidPoolOperatorFee,

    #[error("Pool status is invalid, should be 0 or 1")]
    InvalidPoolStatus,

    #[error(
        "Pool initial total shares {0} is too low, minimum is {min}",
        min = crate::constants::MIN_INITIAL_TOTAL_SHARES
    )]
    PoolInitialTotalSharesTooLow(i64),

    #[error(
        "Pool initial total shares {0} is too high, maximum is {max}",
        max = crate::constants::MAX_INITIAL_TOTAL_SHARES
    )]
    PoolInitialTotalSharesTooHigh(i64),

    #[error("Pool min operator share rate is too low, should be greater than 0")]
    PoolMinOperatorShareRateTooLow,

    #[error(
        "Pool min operator share rate is too high, maximum is {max}",
        max = crate::constants::SHARE_TICK
    )]
    PoolMinOperatorShareRateTooHigh,

    #[error(
        "Pool mint share amount {0} is too low, minimum is {min}",
        min = crate::constants::MIN_POOL_SHARES_TO_MINT_OR_BURN
    )]
    PoolMintShareAmountTooLow(i64),

    #[error(
        "Pool mint share amount {0} is too high, maximum is {max}",
        max = crate::constants::MAX_POOL_SHARES_TO_MINT_OR_BURN
    )]
    PoolMintShareAmountTooHigh(i64),

    #[error(
        "Pool burn share amount {0} is too low, minimum is {min}",
        min = crate::constants::MIN_POOL_SHARES_TO_MINT_OR_BURN
    )]
    PoolBurnShareAmountTooLow(i64),

    #[error(
        "Pool burn share amount {0} is too high, maximum is {max}",
        max = crate::constants::MAX_POOL_SHARES_TO_MINT_OR_BURN
    )]
    PoolBurnShareAmountTooHigh(i64),

    // Transfer and Withdrawal Errors
    #[error(
        "Withdrawal amount {0} is too low, minimum is {min}",
        min = crate::constants::MIN_WITHDRAWAL_AMOUNT
    )]
    WithdrawalAmountTooLow(u64),

    #[error(
        "Withdrawal amount {0} is too high, maximum is {max}",
        max = crate::constants::MAX_WITHDRAWAL_AMOUNT
    )]
    WithdrawalAmountTooHigh(u64),

    #[error(
        "Transfer amount {0} is too low, minimum is {min}",
        min = crate::constants::MIN_TRANSFER_AMOUNT
    )]
    TransferAmountTooLow(i64),

    #[error(
        "Transfer amount {0} is too high, maximum is {max}",
        max = crate::constants::MAX_TRANSFER_AMOUNT
    )]
    TransferAmountTooHigh(i64),

    #[error("Transfer fee is negative")]
    TransferFeeNegative,

    #[error(
        "Transfer fee is too high, maximum is {max}",
        max = crate::constants::MAX_TRANSFER_AMOUNT
    )]
    TransferFeeTooHigh,

    #[error(
        "To account index {0} is too low, minimum is {min}",
        min = crate::constants::MIN_ACCOUNT_INDEX
    )]
    ToAccountIndexTooLow(i64),

    #[error(
        "To account index {0} is too high, maximum is {max}",
        max = crate::constants::MAX_ACCOUNT_INDEX
    )]
    ToAccountIndexTooHigh(i64),

    #[error(
        "From account index {0} is too low, minimum is {min}",
        min = crate::constants::MIN_ACCOUNT_INDEX
    )]
    FromAccountIndexTooLow(i64),

    #[error(
        "From account index {0} is too high, maximum is {max}",
        max = crate::constants::MAX_ACCOUNT_INDEX
    )]
    FromAccountIndexTooHigh(i64),

    // Margin Errors
    #[error("Initial margin fraction is too low, minimum is 0")]
    InitialMarginFractionTooLow,

    #[error(
        "Initial margin fraction {0} is too high, maximum is {max}",
        max = crate::constants::MARGIN_FRACTION_TICK
    )]
    InitialMarginFractionTooHigh(u16),

    #[error("Margin mode is invalid")]
    InvalidMarginMode,

    #[error("Margin movement direction is invalid")]
    InvalidUpdateMarginDirection,

    // General Errors
    #[error(
        "Nonce {0} is too low, minimum is {min}",
        min = crate::constants::MIN_NONCE
    )]
    NonceTooLow(i64),

    #[error("ExpiredAt is invalid")]
    ExpiredAtInvalid,

    #[error("Public key is invalid")]
    PubKeyInvalid,

    #[error("Transaction signature is invalid")]
    InvalidSignature,

    #[error("Cancel all time-in-force is invalid")]
    InvalidCancelAllTimeInForce,

    #[error("Cancel all time is not in valid range")]
    CancelAllTimeIsNotInRange,

    #[error("Cancel all time should be nil")]
    CancelAllTimeIsNotNil,

    #[error("Cancel mode is invalid")]
    CancelModeInvalid,

    // Cryptographic Errors
    #[error("Invalid private key length: expected {expected}, got {actual}")]
    InvalidPrivateKeyLength { expected: usize, actual: usize },

    #[error("Invalid public key length: expected {expected}, got {actual}")]
    InvalidPublicKeyLength { expected: usize, actual: usize },

    #[error("Failed to parse hex: {0}")]
    HexParseError(#[from] hex::FromHexError),

    #[error("Cryptographic operation failed: {0}")]
    CryptoError(String),

    // HTTP and Network Errors
    #[error("HTTP request failed: {0}")]
    HttpError(#[from] reqwest::Error),

    #[error("API error: {0}")]
    ApiError(String),

    #[error("Invalid response from server: {0}")]
    InvalidResponse(String),

    #[error("Network timeout")]
    Timeout,

    // JSON Errors
    #[error("JSON serialization/deserialization error: {0}")]
    JsonError(#[from] serde_json::Error),

    // Generic Errors
    #[error("Missing required field: {0}")]
    MissingField(String),

    #[error("Invalid configuration: {0}")]
    InvalidConfiguration(String),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("{0}")]
    Other(String),
}

impl From<String> for LighterError {
    fn from(s: String) -> Self {
        LighterError::Other(s)
    }
}

impl From<&str> for LighterError {
    fn from(s: &str) -> Self {
        LighterError::Other(s.to_string())
    }
}
