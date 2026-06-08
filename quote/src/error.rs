use thiserror::Error;

/// Define own errors enum
#[derive(Error, Debug)]
pub enum QuoteError {
    /// Map serde error to own error type
    #[error("Convert error")]
    Convert(#[from] serde_json::Error),
}
