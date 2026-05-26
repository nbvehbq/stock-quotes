use thiserror::Error;

#[derive(Error, Debug)]
pub enum QuoteError {
    #[error("Convert error")]
    Convert(#[from] serde_json::Error),
}
