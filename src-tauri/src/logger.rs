use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;
use chrono::Utc;

static LOG_PATH: Mutex<Option<PathBuf>> = Mutex::new(None);

pub fn init(data_dir: &PathBuf) {
    let log_path = data_dir.join("app.log");
    *LOG_PATH.lock().unwrap() = Some(log_path.clone());
    // Keep last 5 sessions
    for i in (1..=5).rev() {
        let old = data_dir.join(format!("app.{}.log", i));
        let newer = if i == 1 { log_path.clone() } else { data_dir.join(format!("app.{}.log", i - 1)) };
        if newer.exists() {
            let _ = fs::copy(&newer, &old);
        }
    }
    // Start fresh log
    let _ = fs::write(&log_path, format!("=== 桌面便签启动 {} ===\n", Utc::now().to_rfc3339()));
}

pub fn log(msg: &str) {
    let guard = LOG_PATH.lock().unwrap();
    if let Some(ref path) = *guard {
        let timestamp = Utc::now().format("%H:%M:%S%.3f");
        let line = format!("[{}] {}\n", timestamp, msg);
        let _ = fs::OpenOptions::new().create(true).append(true).open(path)
            .map(|mut f| { use std::io::Write; let _ = f.write_all(line.as_bytes()); });
    }
}

#[macro_export]
macro_rules! app_log {
    ($($arg:tt)*) => {
        crate::logger::log(&format!($($arg)*))
    };
}
