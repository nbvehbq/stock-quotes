use std::net::UdpSocket;
use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex};
use std::time::Instant;

use crate::Storage;
use crate::error::QuoteServerError;
use quote::StockQuote;

pub fn start_receive_loop(
    socket: Arc<UdpSocket>,
    clients: Arc<Mutex<Storage>>,
) -> Result<(), QuoteServerError> {
    let mut buf = [0u8; 128];
    loop {
        match socket.recv_from(&mut buf) {
            Ok((size, src_addr)) => {
                if &buf[..size] == b"PING" {
                    log::debug!("Got PING from address: {src_addr}");
                    let mut guard = clients
                        .try_lock()
                        .map_err(|e| QuoteServerError::Lock(e.to_string()))?;
                    if let Some((_, last_ping)) = guard.get_mut(&src_addr) {
                        *last_ping = Instant::now();
                    }
                }
            }
            Err(e) => return Err(QuoteServerError::Io(e)),
        }
    }
}

pub fn start_send_loop(
    socket: Arc<UdpSocket>,
    rx: Receiver<StockQuote>,
    dest_addr: &str,
    tickers: Vec<String>,
) -> Result<(), QuoteServerError> {
    // если происходит ошибка чтения, значит канал был закрыт (см start_monitoring)
    // и нужно разорвать цикл
    while let Ok(msg) = rx.recv() {
        if !tickers.contains(&msg.ticker) {
            continue;
        }

        let buf = msg.to_json_string()?;
        let _ = socket.send_to(buf.as_bytes(), dest_addr)?;
    }

    Ok(())
}
