use tauri::State;
use std::sync::Arc;
use crate::db::Database;
use crate::models::WindowState;

#[tauri::command]
pub fn save_window_state(state: WindowState, db: State<Arc<Database>>) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    conn.execute(
        "INSERT OR REPLACE INTO window_state (note_id, x, y, width, height, opacity, pinned, mode, is_visible) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9)",
        rusqlite::params![
            state.note_id, state.x, state.y, state.width, state.height,
            state.opacity, state.pinned as i32, state.mode, state.is_visible as i32
        ],
    ).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn get_window_state(note_id: String, db: State<Arc<Database>>) -> Result<Option<WindowState>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let result = conn.query_row(
        "SELECT note_id, x, y, width, height, opacity, pinned, mode, is_visible FROM window_state WHERE note_id=?1",
        rusqlite::params![note_id],
        |row| {
            Ok(WindowState {
                note_id: row.get(0)?,
                x: row.get(1)?,
                y: row.get(2)?,
                width: row.get(3)?,
                height: row.get(4)?,
                opacity: row.get(5)?,
                pinned: row.get::<_, i32>(6)? != 0,
                mode: row.get(7)?,
                is_visible: row.get::<_, i32>(8)? != 0,
            })
        },
    );
    match result {
        Ok(s) => Ok(Some(s)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub fn get_all_window_states(db: State<Arc<Database>>) -> Result<Vec<WindowState>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn.prepare(
        "SELECT note_id, x, y, width, height, opacity, pinned, mode, is_visible FROM window_state"
    ).map_err(|e| e.to_string())?;
    let states = stmt
        .query_map([], |row| {
            Ok(WindowState {
                note_id: row.get(0)?,
                x: row.get(1)?,
                y: row.get(2)?,
                width: row.get(3)?,
                height: row.get(4)?,
                opacity: row.get(5)?,
                pinned: row.get::<_, i32>(6)? != 0,
                mode: row.get(7)?,
                is_visible: row.get::<_, i32>(8)? != 0,
            })
        })
        .map_err(|e| e.to_string())?
        .filter_map(|r| r.ok())
        .collect();
    Ok(states)
}
