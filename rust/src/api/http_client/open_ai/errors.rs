use thiserror::Error as ThisError;

/// Defines Error types for Chat-Service.
#[derive(ThisError, Debug)]
pub enum ChatErrors {
    /// Contents doesn't match the Structure elements  
    #[error("SERDE-JSON-ERROR: {0}")]
    JSONError(#[from] serde_json::Error),
}

/// Alias Type for Chat-Service Results with Error Handler
pub type ChatResults<T> = Result<T, ChatErrors>;
