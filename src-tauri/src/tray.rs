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
    let calendar = MenuItemBuilder::with_id("calendar", "桌面日历").build(app)?;
    let management = MenuItemBuilder::with_id("management", "打开管理面板").build(app)?;
    let settings = MenuItemBuilder::with_id("settings", "软件设置").build(app)?;
    let quit = MenuItemBuilder::with_id("quit", "退出程序").build(app)?;

    let menu = MenuBuilder::new(app)
        .item(&new_text)
        .item(&new_todo)
        .item(&new_timer)
        .separator()
        .item(&calendar)
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
                        crate::window_manager::create_note_window(app, &note.id).ok();
                    }
                }
                "new_todo" => {
                    if let Ok(note) = crate::commands::notes::create_note_inner(&db, None, "todo".into(), None) {
                        crate::window_manager::create_note_window(app, &note.id).ok();
                    }
                }
                "new_timer" => {
                    if let Ok(note) = crate::commands::notes::create_note_inner(&db, None, "timer".into(), Some("草缸开灯".into())) {
                        crate::window_manager::create_note_window(app, &note.id).ok();
                    }
                }
                "calendar" => { crate::window_manager::create_calendar_widget(app).ok(); }
                "toggle_all" => {
                    let db = app.state::<Arc<Database>>();
                    let conn = db.conn.lock().unwrap();
                    let mut stmt = conn.prepare(
                        "SELECT id, type, title, bg_color FROM notes WHERE deleted_at IS NULL"
                    ).unwrap();
                    let notes: Vec<(String, String, Option<String>, Option<String>)> = stmt.query_map([], |row| {
                        Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?))
                    }).unwrap().filter_map(|r| r.ok()).collect();
                    drop(stmt);
                    drop(conn);

                    let any_visible = notes.iter().any(|(id, _, _, _)| {
                        let label = format!("note-{}", id);
                        app.get_webview_window(&label).map(|w| w.is_visible().unwrap_or(false)).unwrap_or(false)
                    });

                    for (id, ntype, title, bg) in &notes {
                        let bg_color = bg.clone().unwrap_or_else(|| if ntype == "timer" { "#1f1d3d".into() } else { "#f4ecd6".into() });
                        let note_title = title.clone().unwrap_or_default();
                        if any_visible {
                            crate::window_manager::hide_note_window(app, id).ok();
                        } else {
                            crate::window_manager::create_note_window(app, id).ok();
                        }
                    }
                }
                "management" => { crate::window_manager::create_management_panel(app).ok(); }
                "settings" => { crate::window_manager::create_settings_panel(app).ok(); }
                "quit" => { app.exit(0); }
                _ => {}
            }
        })
        .build(app)?;

    Ok(())
}
