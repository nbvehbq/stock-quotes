use quote::error::QuoteError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum QuoteServerError {
    #[error("Mutex lock error: {0}")]
    Lock(String),

    #[error("Io error")]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Quote(#[from] QuoteError),
}
