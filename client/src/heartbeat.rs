use anyhow::Result;
use std::{
    net::{SocketAddr, UdpSocket},
    sync::Arc,
    thread,
    time::Duration,
};

// Start loop that send PING command to server
pub fn start_heartbeat_loop(socket: Arc<UdpSocket>, addr: SocketAddr) -> Result<()> {
    loop {
        socket.send_to(b"PING", addr)?;
        log::debug!("Send PING to addr: {}", addr);
        thread::sleep(Duration::from_secs(3));
    }
}
