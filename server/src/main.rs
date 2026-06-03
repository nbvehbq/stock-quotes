use anyhow::Result;
use std::{
    net::{TcpListener, UdpSocket},
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
const TCP_ADDRESS: &str = "127.0.0.1:3000";

fn main() -> Result<()> {
    let holder = Arc::new(Mutex::new(Storage::new()));

    let clients = Arc::clone(&holder);
    let generator = QuoteGenerator::new(None)?;
    let h1 = thread::spawn(move || generator.run(Duration::from_millis(1000), clients));

    let socket = UdpSocket::bind(UDP_ADDRESS)?;
    let socket = Arc::new(socket);

    let rcv = Arc::clone(&socket);
    let clients = Arc::clone(&holder);
    let h2 = thread::spawn(move || start_receive_loop(rcv, clients));

    let clients = Arc::clone(&holder);
    let h3 = thread::spawn(move || start_monitoring(clients));

    let listener = TcpListener::bind(TCP_ADDRESS)?;
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let clients = Arc::clone(&holder);

                let snd = Arc::clone(&socket);
                thread::spawn(move || {
                    if let Err(e) = handle_client(stream, snd, clients) {
                        println!("client finished with error: {:?}", e);
                    }
                });
            }
            Err(e) => eprintln!("Connection failed: {}", e),
        }
    }

    h1.join().unwrap();
    let _ = h2.join().unwrap();
    h3.join().unwrap();

    Ok(())
}
