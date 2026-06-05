use std::{
    fs::File,
    io::{BufRead, BufReader, Write},
    net::{SocketAddr, TcpStream, UdpSocket},
    path::PathBuf,
    sync::Arc,
};

use anyhow::{Result, bail};
use clap::Parser;
use quote::StockQuote;

use crate::heartbeat::start_heartbeat_loop;

mod heartbeat;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// TCP server address with port
    #[arg(long)]
    tcp_addr: String,

    /// UDP port number
    #[arg(long)]
    udp_port: u16,

    /// Path to tickers file
    #[arg(long)]
    tickers: PathBuf,
}

fn main() -> Result<()> {
    env_logger::init();

    let args = Args::parse();

    let tcp_addr = args.tcp_addr.parse::<SocketAddr>()?;

    let tickers = load_tickers(args.tickers)?;
    if tickers.is_empty() {
        bail!("Tickers file is empty!")
    }
    log::info!("Tickers file processed");

    let command = format!(
        "STREAM {}:{} {}\n",
        tcp_addr.ip(),
        args.udp_port,
        tickers.join(",")
    );

    let mut stream = TcpStream::connect(args.tcp_addr)?;
    stream.write_all(command.as_bytes())?;
    stream.flush()?;

    log::info!("Request to server sended");

    let udp_addr = SocketAddr::new(tcp_addr.ip(), args.udp_port);
    let socket = UdpSocket::bind(udp_addr)?;
    let socket = Arc::new(socket);

    let mut once = Some(true);
    let mut buf = [0u8; 1024];
    while let Ok((n, addr)) = socket.recv_from(&mut buf) {
        if once.take().is_some() {
            log::info!("Start receiving data...\n");
            let ping = Arc::clone(&socket);
            std::thread::spawn(move || {
                if let Err(e) = start_heartbeat_loop(ping, addr) {
                    log::error!("heartbeat thread finished with error: {e}");
                }
            });
        }
        let msg = StockQuote::from_json_str(str::from_utf8(&buf[..n])?)?;
        println!("{msg}")
    }

    Ok(())
}

fn load_tickers(path: PathBuf) -> Result<Vec<String>> {
    let file = File::open(path)?;
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
    Ok(res)
}
