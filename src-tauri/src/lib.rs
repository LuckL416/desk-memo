pub mod backup_engine;
pub mod commands;
pub mod db;
pub mod models;
pub mod reminder_engine;
pub mod timer_engine;
pub mod tray;
pub mod window_manager;

use db::Database;
use std::path::PathBuf;
use std::sync::Arc;
use tauri::Manager;

fn get_data_dir() -> PathBuf {
    let appdata = std::env::var("APPDATA").unwrap_or_else(|_| ".".into());
    PathBuf::from(appdata).join("StickyNotes")
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let data_dir = get_data_dir();
    let database = Arc::new(Database::new(&data_dir).expect("Failed to initialize database"));

    tauri::Builder::default()
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            None,
        ))
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .plugin(tauri_plugin_shell::init())
        .manage(database.clone())
        .manage(data_dir)
        .setup(|app| {
            let db = app.state::<Arc<Database>>();
            let data_dir = app.state::<PathBuf>();
            timer_engine::start_timer_engine(app.handle().clone(), db.inner().clone());
            reminder_engine::start_reminder_engine(app.handle().clone(), db.inner().clone());
            backup_engine::start_backup_engine(data_dir.inner().clone());

            // Always show management panel on startup
            window_manager::create_management_panel(&app.handle()).ok();

            // Restore visible windows
            let visible: Vec<(String, String, Option<String>, Option<String>)> = {
                let db_conn = db.conn.lock().unwrap();
                let mut stmt = db_conn.prepare(
                    "SELECT n.id, n.type, n.title, n.bg_color FROM notes n \
                     JOIN window_state ws ON n.id = ws.note_id \
                     WHERE ws.is_visible = 1 AND n.deleted_at IS NULL"
                ).unwrap();
                let result = stmt.query_map([], |row| {
                    Ok((
                        row.get::<_, String>(0)?,
                        row.get::<_, String>(1)?,
                        row.get::<_, Option<String>>(2)?,
                        row.get::<_, Option<String>>(3)?,
                    ))
                }).unwrap().filter_map(|r| r.ok()).collect();
                result
            };

            for (id, ntype, title, bg) in visible {
                let bg_color = bg.unwrap_or_else(|| {
                    if ntype == "timer" { "#1f1d3d".into() } else { "#f4ecd6".into() }
                });
                let note_title = title.unwrap_or_default();
                window_manager::create_note_window(&app.handle(), &id, &ntype, &bg_color, &note_title).ok();
            }

            tray::setup_tray(&app.handle()).ok();
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::groups::create_group,
            commands::groups::list_groups,
            commands::groups::rename_group,
            commands::groups::delete_group,
            commands::notes::create_note,
            commands::notes::update_note,
            commands::notes::get_note,
            commands::notes::list_notes,
            commands::notes::delete_note,
            commands::notes::restore_note,
            commands::notes::permanently_delete_note,
            commands::notes::search_notes,
            commands::timers::get_timer_state,
            commands::timers::start_timer,
            commands::timers::pause_timer,
            commands::timers::reset_timer,
            commands::timers::update_timer_settings,
            commands::reminders::set_reminder,
            commands::reminders::get_reminders,
            commands::reminders::delete_reminder,
            commands::windows::set_window_opacity,
            commands::windows::save_window_state,
            commands::windows::get_window_state,
            commands::windows::get_all_window_states,
            commands::settings::set_setting,
            commands::settings::get_setting,
            commands::settings::export_all_data,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
