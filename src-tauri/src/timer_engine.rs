use std::sync::Arc;
use std::thread;
use std::time::Duration;
use tauri::{AppHandle, Emitter};
use chrono::{Utc, Timelike};
use crate::db::Database;

pub fn start_timer_engine(app: AppHandle, db: Arc<Database>) {
    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_secs(1));
            let conn = match db.conn.lock() {
                Ok(c) => c,
                Err(_) => continue,
            };

            // Midnight reset check (at 00:00:00-00:00:02 window)
            let now = Utc::now();
            if now.hour() == 0 && now.minute() == 0 && now.second() < 2 {
                conn.execute_batch(
                    "UPDATE timer_state SET remaining_seconds = daily_duration_minutes * 60, is_running = 0"
                ).ok();
                let _ = app.emit("midnight-reset", ());
            }

            // Get all running timers
            let mut stmt = match conn.prepare(
                "SELECT ts.note_id, ts.remaining_seconds, ts.daily_duration_minutes, ts.warn_before_minutes
                 FROM timer_state ts JOIN notes n ON ts.note_id = n.id
                 WHERE ts.is_running = 1 AND n.deleted_at IS NULL"
            ) {
                Ok(s) => s,
                Err(_) => continue,
            };

            let timers: Vec<(String, i32, i32, Option<i32>)> = stmt
                .query_map([], |row| Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, i32>(1)?,
                    row.get::<_, i32>(2)?,
                    row.get::<_, Option<i32>>(3)?,
                )))
                .ok()
                .map(|r| r.filter_map(|x| x.ok()).collect())
                .unwrap_or_default();

            for (note_id, remaining, _daily_min, warn_min) in timers {
                if remaining <= 0 {
                    // Timer finished
                    conn.execute(
                        "UPDATE timer_state SET is_running = 0, remaining_seconds = 0 WHERE note_id = ?1",
                        rusqlite::params![note_id],
                    ).ok();
                    let _ = app.emit("timer-finished", serde_json::json!({ "note_id": note_id }));
                } else {
                    let new_remaining = remaining - 1;
                    conn.execute(
                        "UPDATE timer_state SET remaining_seconds = ?1 WHERE note_id = ?2",
                        rusqlite::params![new_remaining, note_id],
                    ).ok();

                    let _ = app.emit("timer-tick", serde_json::json!({
                        "note_id": note_id,
                        "remaining_seconds": new_remaining,
                        "is_running": true,
                    }));

                    // Check warning threshold
                    if let Some(wm) = warn_min {
                        let warn_seconds = wm * 60;
                        if new_remaining == warn_seconds {
                            let _ = app.emit("timer-warning", serde_json::json!({
                                "note_id": note_id,
                                "minutes_left": wm,
                            }));
                        }
                    }
                }
            }

            // Check tank milestones once per day at 8:00
            if now.hour() == 8 && now.minute() == 0 && now.second() < 2 {
                check_milestones(&conn, &app);
            }
        }
    });
}

fn check_milestones(conn: &std::sync::MutexGuard<rusqlite::Connection>, app: &AppHandle) {
    let today = Utc::now().date_naive();
    let mut stmt = match conn.prepare(
        "SELECT note_id, tank_start_date FROM timer_state WHERE tank_start_date IS NOT NULL"
    ) {
        Ok(s) => s,
        Err(_) => return,
    };

    let items: Vec<(String, String)> = stmt
        .query_map([], |row| Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?)))
        .ok()
        .map(|r| r.filter_map(|x| x.ok()).collect())
        .unwrap_or_default();

    for (note_id, start_str) in items {
        if let Ok(start_date) = chrono::NaiveDate::parse_from_str(&start_str, "%Y-%m-%d") {
            let days = (today - start_date).num_days();
            if days > 0 && [30i64, 100i64, 365i64].contains(&days) {
                let _ = app.emit("tank-milestone", serde_json::json!({
                    "note_id": note_id,
                    "days": days,
                }));
            }
        }
    }
}
