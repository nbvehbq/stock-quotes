use anyhow::Result;
use clap::Parser;
use std::{
    net::{TcpListener, UdpSocket},
    path::PathBuf,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use crate::{
    generator::QuoteGenerator, monitor::start_monitoring, server::handle_client,
    udp::start_receive_loop,
};

mod error;
mod generator;
mod monitor;
mod server;
mod storage;
mod udp;

pub use self::storage::Storage;

const UDP_ADDRESS: &str = "127.0.0.1:0";

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// TCP address with port
    #[arg(long, default_value = "127.0.0.1:3000")]
    addr: String,

    /// Geenerator interval in ms
    #[arg(long, default_value = "1000")]
    interval: u64,

    /// Optional path to tickers file
    #[arg(long)]
    tickers: Option<PathBuf>,
}

fn main() -> Result<()> {
    env_logger::init();

    let args = Args::parse();

    let holder = Arc::new(Mutex::new(Storage::new()));

    let clients = Arc::clone(&holder);
    let generator = QuoteGenerator::new(args.tickers)?;
    let h1 = thread::spawn(move || generator.run(Duration::from_millis(args.interval), clients));
    log::info!("generator started");

    let socket = UdpSocket::bind(UDP_ADDRESS)?;
    log::info!("UDP socket binded");
    let socket = Arc::new(socket);

    let rcv = Arc::clone(&socket);
    let clients = Arc::clone(&holder);
    let h2 = thread::spawn(move || start_receive_loop(rcv, clients));
    log::info!("PING receiver started");

    let clients = Arc::clone(&holder);
    let h3 = thread::spawn(move || start_monitoring(clients));
    log::info!("monitoring thread started");

    let listener = TcpListener::bind(args.addr)?;
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                log::info!("new client arrived");
                let clients = Arc::clone(&holder);

                let snd = Arc::clone(&socket);
                thread::spawn(move || {
                    if let Err(e) = handle_client(stream, snd, clients) {
                        log::error!("client finished with error: {:?}", e);
                    }
                });
            }
            Err(e) => log::error!("Connection failed: {}", e),
        }
    }

    h1.join().unwrap();
    let _ = h2.join().unwrap();
    h3.join().unwrap();

    Ok(())
}
