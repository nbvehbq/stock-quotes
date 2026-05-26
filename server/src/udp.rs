use anyhow::Result;
use std::net::UdpSocket;
use std::sync::TryLockError;
use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex};
use std::time::Instant;

use crate::Storage;
use quote::StockQuote;

pub fn start_receive_loop(socket: Arc<UdpSocket>, clients: Arc<Mutex<Storage>>) -> Result<()> {
    let mut buf = [0u8; 128];
    loop {
        match socket.recv_from(&mut buf) {
            Ok((size, src_addr)) => {
                if &buf[..size] == b"PING" {
                    match clients.try_lock() {
                        Ok(mut guard) => {
                            if let Some((_, last_ping)) = guard.get_mut(&src_addr) {
                                *last_ping = Instant::now();
                            }
                        }
                        Err(TryLockError::WouldBlock) => {
                            println!("Udp: Mutex is CURRENTLY locked by another thread.");
                        }
                        Err(TryLockError::Poisoned(_)) => {
                            println!(
                                "Udp: Mutex is poisoned (a thread panicked while holding it)."
                            );
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("Receive error: {}", e);
            }
        }
    }
}

pub fn start_send_loop(
    socket: Arc<UdpSocket>,
    rx: Receiver<StockQuote>,
    dest_addr: &str,
    tickers: Vec<String>,
) -> Result<()> {
    loop {
        match rx.recv() {
            Ok(msg) => {
                if !tickers.contains(&msg.ticker) {
                    continue;
                }

                let buf = msg.to_json_string()?;
                match socket.send_to(buf.as_bytes(), dest_addr) {
                    Ok(_) => {}
                    Err(e) => {
                        println!("Send error: {}", e);
                    }
                };
            }
            Err(e) => {
                println!("Receive error: {}", e);
                break;
            }
        };
    }
    Ok(())
}
