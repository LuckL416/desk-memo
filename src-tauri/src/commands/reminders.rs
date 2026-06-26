use tauri::State;
use std::sync::Arc;
use crate::db::Database;
use crate::models::Reminder;
use uuid::Uuid;

#[tauri::command]
pub fn set_reminder(
    note_id: String,
    remind_at: String,
    repeat_type: String,
    repeat_days: Option<String>,
    db: State<Arc<Database>>,
) -> Result<Reminder, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let id = Uuid::new_v4().to_string();
    conn.execute(
        "INSERT INTO reminders (id, note_id, remind_at, repeat_type, repeat_days) VALUES (?1,?2,?3,?4,?5)",
        rusqlite::params![id, note_id, remind_at, repeat_type, repeat_days],
    ).map_err(|e| e.to_string())?;
    Ok(Reminder {
        id,
        note_id,
        remind_at,
        repeat_type,
        repeat_days,
        is_active: true,
    })
}

#[tauri::command]
pub fn get_reminders(note_id: String, db: State<Arc<Database>>) -> Result<Vec<Reminder>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn.prepare(
        "SELECT id, note_id, remind_at, repeat_type, repeat_days, is_active FROM reminders WHERE note_id=?1"
    ).map_err(|e| e.to_string())?;
    let reminders = stmt
        .query_map(rusqlite::params![note_id], |row| {
            Ok(Reminder {
                id: row.get(0)?,
                note_id: row.get(1)?,
                remind_at: row.get(2)?,
                repeat_type: row.get(3)?,
                repeat_days: row.get(4)?,
                is_active: row.get::<_, i32>(5)? != 0,
            })
        })
        .map_err(|e| e.to_string())?
        .filter_map(|r| r.ok())
        .collect();
    Ok(reminders)
}

#[tauri::command]
pub fn delete_reminder(id: String, db: State<Arc<Database>>) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    conn.execute("DELETE FROM reminders WHERE id=?1", rusqlite::params![id])
        .map_err(|e| e.to_string())?;
    Ok(())
}
