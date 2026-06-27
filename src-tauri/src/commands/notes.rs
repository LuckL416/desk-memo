use tauri::State;
use tauri::AppHandle;
use crate::db::Database;
use crate::models::Note;
use chrono::Utc;
use uuid::Uuid;
use std::sync::Arc;

// Internal helper (no State needed, used by tray menu)
pub fn create_note_inner(db: &Database, group_id: Option<String>, note_type: String, title: Option<String>) -> Result<Note, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();
    conn.execute(
        "INSERT INTO notes (id, group_id, type, title, created_at, updated_at) VALUES (?1,?2,?3,?4,?5,?6)",
        rusqlite::params![id, group_id, note_type, title, now, now],
    ).map_err(|e| e.to_string())?;
    if note_type == "timer" {
        conn.execute("INSERT INTO timer_state (note_id) VALUES (?1)",
            rusqlite::params![id]).map_err(|e| e.to_string())?;
    }
    Ok(Note {
        id: id.clone(), group_id, r#type: note_type, title: title.clone(), content: None,
        bg_color: None, default_text_color: None, default_font_size: None,
        created_at: now.clone(), updated_at: now, deleted_at: None,
    })
}

#[tauri::command]
pub fn create_note(app: AppHandle, group_id: Option<String>, note_type: String, title: Option<String>,
    db: State<Arc<Database>>) -> Result<Note, String> {
    let note = create_note_inner(&db, group_id, note_type, title)?;
    // 前端创建窗口（避免线程池 build() 死锁）
    Ok(note)
}

#[tauri::command]
pub fn update_note(id: String, title: Option<String>, content: Option<String>,
    bg_color: Option<String>, default_text_color: Option<String>,
    default_font_size: Option<i32>, db: State<Arc<Database>>) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let now = Utc::now().to_rfc3339();
    conn.execute(
        "UPDATE notes SET title=COALESCE(?1,title), content=COALESCE(?2,content), bg_color=COALESCE(?3,bg_color), default_text_color=COALESCE(?4,default_text_color), default_font_size=COALESCE(?5,default_font_size), updated_at=?6 WHERE id=?7",
        rusqlite::params![title, content, bg_color, default_text_color, default_font_size, now, id],
    ).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn get_note(id: String, db: State<Arc<Database>>) -> Result<Note, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    conn.query_row("SELECT id, group_id, type, title, content, bg_color, default_text_color, default_font_size, created_at, updated_at, deleted_at FROM notes WHERE id=?1",
        rusqlite::params![id], |row| {
            Ok(Note {
                id: row.get(0)?, group_id: row.get(1)?, r#type: row.get(2)?,
                title: row.get(3)?, content: row.get(4)?, bg_color: row.get(5)?,
                default_text_color: row.get(6)?, default_font_size: row.get(7)?,
                created_at: row.get(8)?, updated_at: row.get(9)?, deleted_at: row.get(10)?,
            })
        }).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn list_notes(group_id: Option<String>, note_type: Option<String>,
    include_deleted: bool, db: State<Arc<Database>>) -> Result<Vec<Note>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let mut sql = String::from(
        "SELECT id, group_id, type, title, content, bg_color, default_text_color, default_font_size, created_at, updated_at, deleted_at FROM notes WHERE 1=1"
    );
    let mut params: Vec<Box<dyn rusqlite::types::ToSql>> = Vec::new();
    if !include_deleted { sql.push_str(" AND deleted_at IS NULL"); }
    if let Some(ref gid) = group_id { sql.push_str(" AND group_id = ?1"); params.push(Box::new(gid.clone())); }
    if let Some(ref t) = note_type {
        let idx = params.len() + 1;
        sql.push_str(&format!(" AND type = ?{}", idx));
        params.push(Box::new(t.clone()));
    }
    sql.push_str(" ORDER BY updated_at DESC");

    let mut stmt = conn.prepare(&sql).map_err(|e| e.to_string())?;
    let param_refs: Vec<&dyn rusqlite::types::ToSql> = params.iter().map(|p| p.as_ref()).collect();
    let notes = stmt.query_map(param_refs.as_slice(), |row| {
        Ok(Note {
            id: row.get(0)?, group_id: row.get(1)?, r#type: row.get(2)?,
            title: row.get(3)?, content: row.get(4)?, bg_color: row.get(5)?,
            default_text_color: row.get(6)?, default_font_size: row.get(7)?,
            created_at: row.get(8)?, updated_at: row.get(9)?, deleted_at: row.get(10)?,
        })
    }).map_err(|e| e.to_string())?
    .filter_map(|r| r.ok())
    .collect();
    Ok(notes)
}

#[tauri::command]
pub fn delete_note(id: String, db: State<Arc<Database>>) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let now = Utc::now().to_rfc3339();
    conn.execute("UPDATE notes SET deleted_at = ?1, updated_at = ?2 WHERE id = ?3",
        rusqlite::params![now, now, id]).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn restore_note(id: String, db: State<Arc<Database>>) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    conn.execute("UPDATE notes SET deleted_at = NULL WHERE id = ?1",
        rusqlite::params![id]).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn permanently_delete_note(id: String, db: State<Arc<Database>>) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    conn.execute("DELETE FROM timer_state WHERE note_id = ?1", rusqlite::params![id]).ok();
    conn.execute("DELETE FROM reminders WHERE note_id = ?1", rusqlite::params![id]).ok();
    conn.execute("DELETE FROM window_state WHERE note_id = ?1", rusqlite::params![id]).ok();
    conn.execute("DELETE FROM notes WHERE id = ?1", rusqlite::params![id]).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn search_notes(query: String, db: State<Arc<Database>>) -> Result<Vec<Note>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let pattern = format!("%{}%", query);
    let mut stmt = conn.prepare(
        "SELECT id, group_id, type, title, content, bg_color, default_text_color, default_font_size, created_at, updated_at, deleted_at FROM notes WHERE deleted_at IS NULL AND (title LIKE ?1 OR content LIKE ?1) ORDER BY updated_at DESC"
    ).map_err(|e| e.to_string())?;
    let notes = stmt.query_map(rusqlite::params![pattern], |row| {
        Ok(Note {
            id: row.get(0)?, group_id: row.get(1)?, r#type: row.get(2)?,
            title: row.get(3)?, content: row.get(4)?, bg_color: row.get(5)?,
            default_text_color: row.get(6)?, default_font_size: row.get(7)?,
            created_at: row.get(8)?, updated_at: row.get(9)?, deleted_at: row.get(10)?,
        })
    }).map_err(|e| e.to_string())?
    .filter_map(|r| r.ok())
    .collect();
    Ok(notes)
}

#[tauri::command]
pub fn open_note_window(app: AppHandle, note_id: String, db: State<Arc<Database>>) -> Result<(), String> {
    crate::app_log!("open_note_window called, note_id={}", note_id);
    let conn = db.conn.lock().map_err(|e| { crate::app_log!("DB lock failed: {}", e); e.to_string() })?;
    let result = conn.query_row("SELECT id FROM notes WHERE id=?1 AND deleted_at IS NULL",
        rusqlite::params![note_id], |_| Ok(()));
    drop(conn);
    match result {
        Ok(()) => {
            crate::app_log!("open_note_window: note found, creating window");
            crate::window_manager::create_note_window(&app, &note_id).ok();
        }
        Err(e) => {
            crate::app_log!("open_note_window: note not found: {}", e);
        }
    }
    Ok(())
}
