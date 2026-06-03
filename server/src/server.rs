use std::io::{BufRead, BufReader, Write};
use std::net::{SocketAddr, TcpStream, UdpSocket};
use std::sync::{Arc, Mutex, mpsc};
use std::time::Instant;

use quote::StockQuote;

use crate::error::QuoteServerError;
use crate::{Storage, udp::start_send_loop};

pub fn handle_client(
    stream: TcpStream,
    socket: Arc<UdpSocket>,
    clients: Arc<Mutex<Storage>>,
) -> Result<(), QuoteServerError> {
    let mut writer = stream.try_clone()?;
    let mut reader = BufReader::new(stream);

    let mut line = String::new();
    match reader.read_line(&mut line) {
        Ok(0) => return Ok(()),
        Ok(_) => {
            let input = line.trim();
            if input.is_empty() {
                let _ = writer.flush();
                return Ok(());
            }

            let mut parts = input.split_whitespace();
            let response = match parts.next() {
                Some("STREAM") => {
                    let addr = parts.next().and_then(|s| s.parse::<SocketAddr>().ok());
                    let tickers = parts.next();

                    if let (Some(addr), Some(tickers)) = (addr, tickers) {
                        println!("STREAM: {:?} - {}", addr, tickers);

                        let (tx, rx) = mpsc::channel::<StockQuote>();
                        let mut guard = clients.lock().unwrap();
                        // .map_err(|e| QuoteServerError::Lock("handle_client".to_string()))?;

                        guard.insert(addr, (tx, Instant::now()));

                        let tickers: Vec<String> =
                            tickers.split(',').map(|s| s.trim().to_string()).collect();

                        std::thread::spawn(move || {
                            start_send_loop(socket, rx, &addr.to_string(), tickers)
                        });

                        "OK\n".to_string()
                    } else {
                        "NOK".to_string()
                    }
                }
                _ => "ERR\n".to_string(),
            };

            let _ = writer.write_all(response.as_bytes());
            let _ = writer.flush();
        }
        Err(e) => return Err(QuoteServerError::Io(e)),
    }

    Ok(())
}
