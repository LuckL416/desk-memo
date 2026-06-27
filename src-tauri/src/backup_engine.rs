use std::fs;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;
use chrono::{Timelike, Utc};

pub fn start_backup_engine(data_dir: PathBuf) {
    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_secs(3600)); // check every hour
            let now = Utc::now();
            if now.hour() == 3 {
                let backup_dir = data_dir.join("backups");
                fs::create_dir_all(&backup_dir).ok();
                let db_path = data_dir.join("data.db");
                let backup_name = format!("backup-{}.db", now.format("%Y-%m-%d"));
                let backup_path = backup_dir.join(&backup_name);
                fs::copy(&db_path, &backup_path).ok();

                // Keep only last 7 backups
                if let Ok(entries) = fs::read_dir(&backup_dir) {
                    let mut files: Vec<_> = entries
                        .filter_map(|e| e.ok())
                        .filter(|e| e.file_name().to_string_lossy().starts_with("backup-"))
                        .collect();
                    files.sort_by_key(|e| e.metadata().and_then(|m| m.modified()).ok());
                    while files.len() > 7 {
                        if let Some(old) = files.first() {
                            fs::remove_file(old.path()).ok();
                            files.remove(0);
                        }
                    }
                }
            }
        }
    });
}
