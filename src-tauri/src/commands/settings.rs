use tauri::State;
use std::sync::Arc;
use std::fs;
use crate::db::Database;
use crate::models::{Group, Note, Reminder, Setting};

#[tauri::command]
pub fn set_setting(key: String, value: String, db: State<Arc<Database>>) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    conn.execute(
        "INSERT OR REPLACE INTO settings (key, value) VALUES (?1, ?2)",
        rusqlite::params![key, value],
    ).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn get_setting(key: String, db: State<Arc<Database>>) -> Result<Option<String>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    match conn.query_row(
        "SELECT value FROM settings WHERE key=?1",
        rusqlite::params![key],
        |row| row.get(0),
    ) {
        Ok(v) => Ok(Some(v)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub fn export_all_data(path: String, db: State<Arc<Database>>) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    // Query groups
    let mut stmt = conn.prepare("SELECT id, name, sort_order, created_at, updated_at FROM groups").map_err(|e| e.to_string())?;
    let groups: Vec<Group> = stmt.query_map([], |row| {
        Ok(Group { id: row.get(0)?, name: row.get(1)?, sort_order: row.get(2)?, created_at: row.get(3)?, updated_at: row.get(4)? })
    }).map_err(|e| e.to_string())?.filter_map(|r| r.ok()).collect();

    // Query notes
    let mut stmt = conn.prepare("SELECT id, group_id, type, title, content, bg_color, default_text_color, default_font_size, created_at, updated_at, deleted_at FROM notes").map_err(|e| e.to_string())?;
    let notes: Vec<Note> = stmt.query_map([], |row| {
        Ok(Note { id: row.get(0)?, group_id: row.get(1)?, r#type: row.get(2)?, title: row.get(3)?, content: row.get(4)?, bg_color: row.get(5)?, default_text_color: row.get(6)?, default_font_size: row.get(7)?, created_at: row.get(8)?, updated_at: row.get(9)?, deleted_at: row.get(10)? })
    }).map_err(|e| e.to_string())?.filter_map(|r| r.ok()).collect();

    // Query reminders
    let mut stmt = conn.prepare("SELECT id, note_id, remind_at, repeat_type, repeat_days, is_active FROM reminders").map_err(|e| e.to_string())?;
    let reminders: Vec<Reminder> = stmt.query_map([], |row| {
        Ok(Reminder { id: row.get(0)?, note_id: row.get(1)?, remind_at: row.get(2)?, repeat_type: row.get(3)?, repeat_days: row.get(4)?, is_active: row.get::<_, i32>(5)? != 0 })
    }).map_err(|e| e.to_string())?.filter_map(|r| r.ok()).collect();

    // Query settings
    let mut stmt = conn.prepare("SELECT key, value FROM settings").map_err(|e| e.to_string())?;
    let settings: Vec<Setting> = stmt.query_map([], |row| {
        Ok(Setting { key: row.get(0)?, value: row.get(1)? })
    }).map_err(|e| e.to_string())?.filter_map(|r| r.ok()).collect();

    let export = serde_json::json!({ "groups": groups, "notes": notes, "reminders": reminders, "settings": settings });
    let json = serde_json::to_string_pretty(&export).map_err(|e| e.to_string())?;
    fs::write(&path, json).map_err(|e| e.to_string())?;
    Ok(())
}
