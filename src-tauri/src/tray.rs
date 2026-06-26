use tauri::{
    AppHandle, Manager,
    menu::{MenuBuilder, MenuItemBuilder},
    tray::{TrayIconBuilder},
};
use crate::db::Database;
use std::sync::Arc;

pub fn setup_tray(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    let new_text = MenuItemBuilder::with_id("new_text", "新建普通便签 (Ctrl+Alt+N)").build(app)?;
    let new_todo = MenuItemBuilder::with_id("new_todo", "新建待办便签").build(app)?;
    let new_timer = MenuItemBuilder::with_id("new_timer", "新建计时便签").build(app)?;
    let toggle_all = MenuItemBuilder::with_id("toggle_all", "显示/隐藏全部便签").build(app)?;
    let management = MenuItemBuilder::with_id("management", "打开管理面板").build(app)?;
    let settings = MenuItemBuilder::with_id("settings", "软件设置").build(app)?;
    let quit = MenuItemBuilder::with_id("quit", "退出程序").build(app)?;

    let menu = MenuBuilder::new(app)
        .item(&new_text)
        .item(&new_todo)
        .item(&new_timer)
        .separator()
        .item(&toggle_all)
        .item(&management)
        .item(&settings)
        .separator()
        .item(&quit)
        .build()?;

    let _tray = TrayIconBuilder::new()
        .menu(&menu)
        .on_menu_event(move |app, event| {
            let db = app.state::<Arc<Database>>();
            match event.id().as_ref() {
                "new_text" => {
                    if let Ok(note) = crate::commands::notes::create_note_inner(&db, None, "text".into(), None) {
                        crate::window_manager::create_note_window(app, &note.id, &note.r#type, "#f4ecd6", "").ok();
                    }
                }
                "new_todo" => {
                    if let Ok(note) = crate::commands::notes::create_note_inner(&db, None, "todo".into(), None) {
                        crate::window_manager::create_note_window(app, &note.id, &note.r#type, "#f4ecd6", "").ok();
                    }
                }
                "new_timer" => {
                    if let Ok(note) = crate::commands::notes::create_note_inner(&db, None, "timer".into(), Some("草缸开灯".into())) {
                        crate::window_manager::create_note_window(app, &note.id, &note.r#type, "#1f1d3d", "草缸开灯").ok();
                    }
                }
                "toggle_all" => { /* Handled by frontend */ }
                "management" => { crate::window_manager::create_management_panel(app).ok(); }
                "settings" => { crate::window_manager::create_settings_panel(app).ok(); }
                "quit" => { app.exit(0); }
                _ => {}
            }
        })
        .build(app)?;

    Ok(())
}
