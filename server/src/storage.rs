use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::mpsc::Sender;
use std::time::Instant;

use quote::StockQuote;

/// Storage is a alias to complex type
pub type Storage = HashMap<SocketAddr, (Sender<StockQuote>, Instant)>;
