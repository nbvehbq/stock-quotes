use std::{
    sync::{Arc, Mutex, TryLockError},
    time::Duration,
};

use crate::Storage;

pub fn start_monitoring(clients: Arc<Mutex<Storage>>) {
    loop {
        match clients.try_lock() {
            Ok(mut guard) => {
                guard.retain(|_, (tx, last_ping)| {
                    if last_ping.elapsed() > Duration::from_secs(5) {
                        println!("Should droped here! {:?}", tx);
                        false
                    } else {
                        true
                    }
                });
            }
            Err(TryLockError::WouldBlock) => {
                println!("Monitor: Mutex is CURRENTLY locked by another thread.");
            }
            Err(TryLockError::Poisoned(_)) => {
                println!("Monitor: Mutex is poisoned (a thread panicked while holding it).");
            }
        }
        std::thread::sleep(Duration::from_secs(2));
    }
}
