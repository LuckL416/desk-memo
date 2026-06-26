pub mod backup_engine;
pub mod commands;
pub mod db;
pub mod models;
pub mod reminder_engine;
pub mod timer_engine;

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
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
