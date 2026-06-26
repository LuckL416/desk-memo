use tauri::State;
use crate::db::Database;
use crate::models::Group;
use chrono::Utc;
use uuid::Uuid;
use std::sync::Arc;

#[tauri::command]
pub fn create_group(name: String, db: State<Arc<Database>>) -> Result<Group, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();
    conn.execute(
        "INSERT INTO groups (id, name, created_at, updated_at) VALUES (?1, ?2, ?3, ?4)",
        rusqlite::params![id, name, now, now],
    ).map_err(|e| e.to_string())?;
    Ok(Group { id, name, sort_order: 0, created_at: now.clone(), updated_at: now })
}

#[tauri::command]
pub fn list_groups(db: State<Arc<Database>>) -> Result<Vec<Group>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn.prepare(
        "SELECT id, name, sort_order, created_at, updated_at FROM groups ORDER BY sort_order"
    ).map_err(|e| e.to_string())?;
    let groups = stmt.query_map([], |row| {
        Ok(Group {
            id: row.get(0)?, name: row.get(1)?, sort_order: row.get(2)?,
            created_at: row.get(3)?, updated_at: row.get(4)?,
        })
    }).map_err(|e| e.to_string())?
    .filter_map(|r| r.ok())
    .collect();
    Ok(groups)
}

#[tauri::command]
pub fn rename_group(id: String, name: String, db: State<Arc<Database>>) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let now = Utc::now().to_rfc3339();
    conn.execute("UPDATE groups SET name = ?1, updated_at = ?2 WHERE id = ?3",
        rusqlite::params![name, now, id]).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn delete_group(id: String, db: State<Arc<Database>>) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    conn.execute("UPDATE notes SET group_id = NULL WHERE group_id = ?1",
        rusqlite::params![id]).map_err(|e| e.to_string())?;
    conn.execute("DELETE FROM groups WHERE id = ?1",
        rusqlite::params![id]).map_err(|e| e.to_string())?;
    Ok(())
}
