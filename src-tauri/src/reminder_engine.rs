use std::sync::Arc;
use std::thread;
use std::time::Duration;
use tauri::{AppHandle, Emitter};
use chrono::Utc;
use crate::db::Database;

pub fn start_reminder_engine(app: AppHandle, db: Arc<Database>) {
    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_secs(60));
            let conn = match db.conn.lock() {
                Ok(c) => c,
                Err(_) => continue,
            };
            let now = Utc::now().to_rfc3339();

            let mut stmt = match conn.prepare(
                "SELECT id, note_id, repeat_type FROM reminders WHERE is_active = 1 AND remind_at <= ?1"
            ) {
                Ok(s) => s,
                Err(_) => continue,
            };

            let due: Vec<(String, String, String)> = stmt
                .query_map(rusqlite::params![now], |row| {
                    Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?, row.get::<_, String>(2)?))
                })
                .ok()
                .map(|r| r.filter_map(|x| x.ok()).collect())
                .unwrap_or_default();

            for (id, note_id, repeat_type) in due {
                let _ = app.emit("reminder-triggered", serde_json::json!({
                    "reminder_id": id,
                    "note_id": note_id,
                }));

                match repeat_type.as_str() {
                    "once" => {
                        conn.execute(
                            "UPDATE reminders SET is_active = 0 WHERE id = ?1",
                            rusqlite::params![id],
                        ).ok();
                    }
                    "daily" => {
                        let next = Utc::now() + chrono::Duration::days(1);
                        conn.execute(
                            "UPDATE reminders SET remind_at = ?1 WHERE id = ?2",
                            rusqlite::params![next.to_rfc3339(), id],
                        ).ok();
                    }
                    "weekly" => {
                        let next = Utc::now() + chrono::Duration::weeks(1);
                        conn.execute(
                            "UPDATE reminders SET remind_at = ?1 WHERE id = ?2",
                            rusqlite::params![next.to_rfc3339(), id],
                        ).ok();
                    }
                    _ => {}
                }
            }
        }
    });
}
