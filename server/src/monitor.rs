use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use crate::Storage;

const MONITOR_INTERVAL: Duration = Duration::from_secs(2);
const ALLOWED_PING_GAP: Duration = Duration::from_secs(5);

/// Запускает мониторинг клиентов.
/// Не может завершатся ошибкой, поэтому ждет захвата мьютекса (unwrap())
pub fn start_monitoring(clients: Arc<Mutex<Storage>>) {
    loop {
        log::debug!("monitoring ticked");
        {
            let mut guard = clients.lock().unwrap();
            guard.retain(|_, (_, last_ping)| last_ping.elapsed() < ALLOWED_PING_GAP);
        }

        std::thread::sleep(MONITOR_INTERVAL);
    }
}
