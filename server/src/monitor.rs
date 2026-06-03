use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use crate::Storage;

/// Запускает мониторинг клиентов.
/// Не может завершатся ошибкой, поэтому ждет захвата мьютекса (unwrap())
/// и не генерит ошибку при неудачном захвате
pub fn start_monitoring(clients: Arc<Mutex<Storage>>) {
    println!("Monitoring started");

    loop {
        {
            let mut guard = clients.lock().unwrap();

            guard.retain(|_, (_, last_ping)| last_ping.elapsed() < Duration::from_secs(5));
        }

        std::thread::sleep(Duration::from_secs(2));
    }
}
