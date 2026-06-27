use tauri::State;
use crate::db::Database;
use crate::models::TimerState;
use chrono::Utc;
use std::sync::Arc;

#[tauri::command]
pub fn get_timer_state(note_id: String, db: State<Arc<Database>>) -> Result<TimerState, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    conn.query_row(
        "SELECT note_id, daily_duration_minutes, remaining_seconds, is_running, last_resume_at, tank_start_date, auto_start_time, warn_before_minutes FROM timer_state WHERE note_id=?1",
        rusqlite::params![note_id],
        |row| {
            Ok(TimerState {
                note_id: row.get(0)?,
                daily_duration_minutes: row.get(1)?,
                remaining_seconds: row.get(2)?,
                is_running: row.get::<_, i32>(3)? != 0,
                last_resume_at: row.get(4)?,
                tank_start_date: row.get(5)?,
                auto_start_time: row.get(6)?,
                warn_before_minutes: row.get(7)?,
            })
        },
    ).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn start_timer(note_id: String, db: State<Arc<Database>>) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let now = Utc::now().to_rfc3339();
    conn.execute(
        "UPDATE timer_state SET is_running = 1, last_resume_at = ?1 WHERE note_id = ?2",
        rusqlite::params![now, note_id],
    ).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn pause_timer(note_id: String, db: State<Arc<Database>>) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    conn.execute(
        "UPDATE timer_state SET is_running = 0 WHERE note_id = ?1",
        rusqlite::params![note_id],
    ).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn reset_timer(note_id: String, db: State<Arc<Database>>) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    conn.execute(
        "UPDATE timer_state SET remaining_seconds = daily_duration_minutes * 60, is_running = 0 WHERE note_id = ?1",
        rusqlite::params![note_id],
    ).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn update_timer_settings(
    note_id: String,
    daily_duration_minutes: Option<i32>,
    tank_start_date: Option<String>,
    auto_start_time: Option<String>,
    warn_before_minutes: Option<i32>,
    db: State<Arc<Database>>,
) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    if let Some(d) = daily_duration_minutes {
        conn.execute(
            "UPDATE timer_state SET daily_duration_minutes = ?1, remaining_seconds = ?2 WHERE note_id = ?3",
            rusqlite::params![d, d * 60, note_id],
        ).map_err(|e| e.to_string())?;
    }
    if let Some(ref d) = tank_start_date {
        conn.execute(
            "UPDATE timer_state SET tank_start_date = ?1 WHERE note_id = ?2",
            rusqlite::params![d, note_id],
        ).map_err(|e| e.to_string())?;
    }
    if let Some(ref t) = auto_start_time {
        conn.execute(
            "UPDATE timer_state SET auto_start_time = ?1 WHERE note_id = ?2",
            rusqlite::params![t, note_id],
        ).map_err(|e| e.to_string())?;
    }
    if let Some(w) = warn_before_minutes {
        conn.execute(
            "UPDATE timer_state SET warn_before_minutes = ?1 WHERE note_id = ?2",
            rusqlite::params![w, note_id],
        ).map_err(|e| e.to_string())?;
    }
    Ok(())
}
