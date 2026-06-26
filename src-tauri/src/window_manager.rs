use tauri::{AppHandle, WebviewUrl, WebviewWindowBuilder, Manager};

pub fn create_note_window(app: &AppHandle, note_id: &str, note_type: &str, bg_color: &str, title: &str) -> Result<(), String> {
    let label = format!("note-{}", note_id);
    if app.get_webview_window(&label).is_some() {
        return Ok(());
    }

    let _window = WebviewWindowBuilder::new(app, &label, WebviewUrl::App("index.html".into()))
        .title("桌面便签")
        .inner_size(320.0, 240.0)
        .min_inner_size(200.0, 120.0)
        .decorations(false)
        .skip_taskbar(true)
        .transparent(true)
        .visible(true)
        .build()
        .map_err(|e| e.to_string())?;

    let js = format!(
        "window.__noteId='{}';window.__noteType='{}';window.__noteBg='{}';window.__noteTitle='{}';",
        note_id, note_type, bg_color, title
    );
    let _ = app.get_webview_window(&label).unwrap().eval(&js);
    Ok(())
}

pub fn hide_note_window(app: &AppHandle, note_id: &str) -> Result<(), String> {
    let label = format!("note-{}", note_id);
    if let Some(window) = app.get_webview_window(&label) {
        window.hide().map_err(|e| e.to_string())?;
    }
    Ok(())
}

pub fn show_note_window(app: &AppHandle, note_id: &str) -> Result<(), String> {
    let label = format!("note-{}", note_id);
    if let Some(window) = app.get_webview_window(&label) {
        window.show().map_err(|e| e.to_string())?;
        window.set_focus().ok();
    }
    Ok(())
}

pub fn set_note_window_opacity(app: &AppHandle, note_id: &str, opacity: f64) -> Result<(), String> {
    // Tauri 2 does not expose a cross-platform set_opacity API.
    // Window-level opacity is controlled via CSS on the frontend.
    let label = format!("note-{}", note_id);
    let _opacity = opacity.max(0.05).min(1.0);
    if let Some(window) = app.get_webview_window(&label) {
        let js = format!("document.documentElement.style.opacity='{}';", _opacity);
        let _ = window.eval(&js);
    }
    Ok(())
}

pub fn set_note_window_resizable(app: &AppHandle, note_id: &str, resizable: bool) -> Result<(), String> {
    let label = format!("note-{}", note_id);
    if let Some(window) = app.get_webview_window(&label) {
        window.set_resizable(resizable).ok();
    }
    Ok(())
}

pub fn set_note_window_always_on_top(app: &AppHandle, note_id: &str, on_top: bool) -> Result<(), String> {
    let label = format!("note-{}", note_id);
    if let Some(window) = app.get_webview_window(&label) {
        window.set_always_on_top(on_top).ok();
    }
    Ok(())
}

pub fn create_management_panel(app: &AppHandle) -> Result<(), String> {
    let label = "management-panel";
    if app.get_webview_window(label).is_some() {
        // Already exists, just show it
        if let Some(w) = app.get_webview_window(label) {
            w.show().ok();
            w.set_focus().ok();
        }
        return Ok(());
    }
    let _window = WebviewWindowBuilder::new(app, label, WebviewUrl::App("index.html".into()))
        .title("便签管理")
        .inner_size(800.0, 500.0)
        .min_inner_size(480.0, 360.0)
        .decorations(false)
        .visible(true)
        .build()
        .map_err(|e| e.to_string())?;
    let _ = app.get_webview_window(label).unwrap().eval("window.__windowType='management';");
    Ok(())
}

pub fn create_settings_panel(app: &AppHandle) -> Result<(), String> {
    let label = "settings-panel";
    if app.get_webview_window(label).is_some() {
        if let Some(w) = app.get_webview_window(label) {
            w.show().ok();
            w.set_focus().ok();
        }
        return Ok(());
    }
    let _window = WebviewWindowBuilder::new(app, label, WebviewUrl::App("index.html".into()))
        .title("软件设置")
        .inner_size(500.0, 400.0)
        .decorations(false)
        .visible(true)
        .build()
        .map_err(|e| e.to_string())?;
    let _ = app.get_webview_window(label).unwrap().eval("window.__windowType='settings';");
    Ok(())
}
