use thiserror::Error as ThisError;

/// Defines Error types.
#[derive(ThisError, Debug)]
pub enum MarketErrors {
    /// Contents doesn't match the Structure elements  
    #[error("SERDE-JSON-ERROR: {0}")]
    JSONError(#[from] serde_json::Error),
}
/// Alias Type for Results with Error Handler
pub type MarketResults<T> = Result<T, MarketErrors>;
