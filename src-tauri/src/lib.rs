pub mod commands;
pub mod db;
pub mod models;

use db::Database;
use std::path::PathBuf;

fn get_data_dir() -> PathBuf {
    let appdata = std::env::var("APPDATA").unwrap_or_else(|_| ".".into());
    PathBuf::from(appdata).join("StickyNotes")
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let data_dir = get_data_dir();
    let database = Database::new(&data_dir).expect("Failed to initialize database");

    tauri::Builder::default()
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            None,
        ))
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .plugin(tauri_plugin_shell::init())
        .manage(database)
        .manage(data_dir)
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
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
