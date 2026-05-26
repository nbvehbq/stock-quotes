use std::io::{BufRead, BufReader, Write};
use std::net::{SocketAddr, TcpStream, UdpSocket};
use std::sync::{Arc, Mutex, TryLockError, mpsc};
use std::time::Instant;

use quote::StockQuote;

use crate::{Storage, udp::start_send_loop};

pub fn handle_client(stream: TcpStream, socket: Arc<UdpSocket>, clients: Arc<Mutex<Storage>>) {
    let mut writer = stream.try_clone().expect("failed to clone stream");
    let mut reader = BufReader::new(stream);

    let mut line = String::new();
    match reader.read_line(&mut line) {
        Ok(0) => {
            println!("Client close connection");
        }
        Ok(_) => {
            let input = line.trim();
            if input.is_empty() {
                let _ = writer.flush();
                return;
            }

            let mut parts = input.split_whitespace();
            let response = match parts.next() {
                Some("STREAM") => {
                    let addr = parts.next().and_then(|s| s.parse::<SocketAddr>().ok());
                    let tickers = parts.next();

                    if let (Some(addr), Some(tickers)) = (addr, tickers) {
                        println!("STREAM: {:?} - {}", addr, tickers);

                        let (tx, rx) = mpsc::channel::<StockQuote>();
                        match clients.try_lock() {
                            Ok(mut guard) => {
                                guard.insert(addr, (tx, Instant::now()));
                            }
                            Err(TryLockError::WouldBlock) => {
                                println!("Server: Mutex is CURRENTLY locked by another thread.");
                            }
                            Err(TryLockError::Poisoned(_)) => {
                                println!(
                                    "Server: Mutex is poisoned (a thread panicked while holding it)."
                                );
                            }
                        }

                        let tickers: Vec<String> =
                            tickers.split(',').map(|s| s.trim().to_string()).collect();
                        std::thread::spawn(move || {
                            start_send_loop(socket, rx, &addr.to_string(), tickers)
                        });

                        "OK\n".to_string()
                    } else {
                        "Wrong command".to_string()
                    }
                }
                _ => "ERROR: unknown command\n".to_string(),
            };

            let _ = writer.write_all(response.as_bytes());
            let _ = writer.flush();
        }
        Err(e) => {
            println!("Read error: {:?}", e);
        }
    }
}
