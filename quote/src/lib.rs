#![warn(missing_docs)]

//! This crate describe stock quote object and methods for Serialize/Deserialize it

use core::fmt;

use crate::error::QuoteError;
use serde::{Deserialize, Serialize};

/// Own errors
pub mod error;

/// Describe stock quote object
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockQuote {
    /// Ticker name
    pub ticker: String,
    /// Ticker price
    pub price: f64,
    /// Ticker volume
    pub volume: u32,
    /// Price timestamp
    pub timestamp: u64,
}

impl StockQuote {
    /// Deserialize from json string
    pub fn from_json_str(data: &str) -> Result<Self, QuoteError> {
        serde_json::from_str(data).map_err(QuoteError::Convert)
    }

    /// Serialize to json string
    pub fn to_json_string(&self) -> Result<String, QuoteError> {
        serde_json::to_string(self).map_err(QuoteError::Convert)
    }
}

impl fmt::Display for StockQuote {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}: price: {} volume: {}, time: {}",
            self.ticker, self.price, self.volume, self.timestamp
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_json_str_success() {
        let data = r#"{"ticker":"AAPL","price":0.12,"volume":100,"timestamp":1780655430369}"#;
        let res = StockQuote::from_json_str(data);
        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(res.ticker, "AAPL".to_string());
        assert_eq!(res.price, 0.12);
        assert_eq!(res.volume, 100);
        assert_eq!(res.timestamp, 1780655430369);
    }

    #[test]
    fn test_from_json_str_fail() {
        let data = "BAD";
        assert!(matches!(
            StockQuote::from_json_str(data),
            Err(QuoteError::Convert(_))
        ));
    }

    #[test]
    fn test_to_json_string() {
        let data = StockQuote {
            ticker: "AAPL".to_string(),
            price: 3.45,
            volume: 100,
            timestamp: 1780655430369,
        };
        let expected = r#"{"ticker":"AAPL","price":3.45,"volume":100,"timestamp":1780655430369}"#;

        let res = data.to_json_string().unwrap();

        assert_eq!(res, expected);
    }

    #[test]
    fn test_to_string() {
        let data = StockQuote {
            ticker: "AAPL".to_string(),
            price: 3.45,
            volume: 100,
            timestamp: 1780655430369,
        };
        let expected = r#"AAPL: price: 3.45 volume: 100, time: 1780655430369"#;

        assert_eq!(expected, data.to_string());
    }
}
