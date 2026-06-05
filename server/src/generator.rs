use anyhow::Result;
use quote::StockQuote;
use rand::RngExt;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
    sync::{Arc, Mutex},
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use crate::Storage;

#[derive(Debug, Clone)]
pub struct QuoteGenerator {
    tickers: HashMap<String, f64>,
}

impl QuoteGenerator {
    pub fn new(path: Option<PathBuf>) -> Result<Self> {
        let mut rng = rand::rng();

        let tikers = match path {
            Some(f) => {
                let file = File::open(f)?;
                let mut reader = BufReader::new(file);
                let mut res = Vec::new();

                let mut buff = String::new();
                while reader.read_line(&mut buff)? > 0 {
                    let ticker = buff.trim().to_owned();
                    buff.clear();

                    if ticker.is_empty() {
                        continue;
                    }
                    res.push(ticker);
                }
                res
            }
            None => vec!["AAPL".into(), "MSFT".into(), "TSLA".into()],
        };

        Ok(Self {
            tickers: tikers.iter().fold(HashMap::new(), |mut acc, ticker| {
                acc.insert(ticker.to_string(), rng.random_range(10.0..=1500.0));
                acc
            }),
        })
    }

    fn generate_quote(&self, ticker: &str) -> Option<StockQuote> {
        let last_price = self.tickers.get(ticker)?;

        let mut rng = rand::rng();
        let suppose_down = rng.random::<bool>();
        let mut delta = *last_price * rng.random::<f64>();
        if suppose_down {
            delta = -delta;
        }

        let volume = match ticker {
            "AAPL" | "MSFT" | "TSLA" => 1000 + (rng.random::<f64>() * 5000.0) as u32,
            _ => 100 + (rng.random::<f64>() * 1000.0) as u32,
        };

        Some(StockQuote {
            ticker: ticker.to_string(),
            price: 0.0f64.max(*last_price + delta),
            volume,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64,
        })
    }

    /// Запускает генерацию тикетов.
    /// Не может завершатся ошибкой, поэтому ждет захвата мьютекса (unwrap())
    pub fn run(self, interval: Duration, clients: Arc<Mutex<Storage>>) {
        loop {
            {
                let mut guard = clients.lock().unwrap();
                log::debug!("generator ticked. Client len: {}", guard.len());

                for ticker in self.tickers.keys() {
                    let Some(quote) = self.generate_quote(ticker) else {
                        // will never happen
                        continue;
                    };

                    guard.retain(|_, (tx, _)| tx.send(quote.clone()).is_ok());
                }
            }
            std::thread::sleep(interval);
        }
    }
}
