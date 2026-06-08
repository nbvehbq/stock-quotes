use quote::error::QuoteError;
use thiserror::Error;

/// Define own errors enum
#[derive(Debug, Error)]
pub enum QuoteServerError {
    /// Map lock error to own type
    #[error("Mutex lock error: {0}")]
    Lock(String),

    /// Map io::Error to own type
    #[error("Io error")]
    Io(#[from] std::io::Error),

    /// Map quote own error into local error
    #[error(transparent)]
    Quote(#[from] QuoteError),
}
