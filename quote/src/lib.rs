use crate::error::QuoteError;
use serde::{Deserialize, Serialize};

mod error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockQuote {
    pub ticker: String,
    pub price: f64,
    pub volume: u32,
    pub timestamp: u64,
}

impl StockQuote {
    pub fn from_json_str(data: &str) -> Result<Self, QuoteError> {
        serde_json::from_str(data).map_err(QuoteError::Convert)
    }

    pub fn to_json_string(&self) -> Result<String, QuoteError> {
        serde_json::to_string(self).map_err(QuoteError::Convert)
    }
}
