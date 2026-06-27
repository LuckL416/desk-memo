use tauri::{AppHandle, WebviewUrl, WebviewWindowBuilder, Manager};

pub fn create_note_window(app: &AppHandle, note_id: &str) -> Result<(), String> {
    let label = format!("note-{}", note_id);
    crate::app_log!("create_note_window: label={}", label);
    if let Some(existing) = app.get_webview_window(&label) {
        crate::app_log!("create_note_window: window exists, visible={}", existing.is_visible().unwrap_or(false));
        if !existing.is_visible().unwrap_or(false) {
            existing.show().map_err(|e| { crate::app_log!("show failed: {}", e); e.to_string() })?;
        }
        existing.set_focus().ok();
        return Ok(());
    }

    crate::app_log!("create_note_window: creating new window");
    let window = WebviewWindowBuilder::new(app, &label, WebviewUrl::App("index.html".into()))
        .title("桌面便签")
        .inner_size(320.0, 240.0)
        .min_inner_size(200.0, 120.0)
        .decorations(false)
        .skip_taskbar(true)
        .visible(true)
        .build()
        .map_err(|e| { crate::app_log!("build failed: {}", e); e.to_string() })?;

    crate::app_log!("create_note_window: window created ok");
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
    let label = format!("note-{}", note_id);
    let _opacity = opacity.max(0.05).min(1.0);
    if let Some(window) = app.get_webview_window(&label) {
        let js = format!("document.documentElement.style.opacity='{}';", _opacity);
        let _ = window.eval(&js);
    }
    Ok(())
}

pub fn create_management_panel(app: &AppHandle) -> Result<(), String> {
    let label = "management-panel";
    if app.get_webview_window(label).is_some() {
        if let Some(w) = app.get_webview_window(label) {
            w.show().ok();
            w.set_focus().ok();
        }
        return Ok(());
    }
    WebviewWindowBuilder::new(app, label, WebviewUrl::App("index.html".into()))
        .title("便签管理")
        .inner_size(800.0, 500.0)
        .min_inner_size(480.0, 360.0)
        .decorations(false)
        .visible(true)
        .build()
        .map_err(|e| e.to_string())?;
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
    WebviewWindowBuilder::new(app, label, WebviewUrl::App("index.html".into()))
        .title("软件设置")
        .inner_size(500.0, 400.0)
        .decorations(false)
        .visible(true)
        .build()
        .map_err(|e| e.to_string())?;
    Ok(())
}

pub fn create_calendar_widget(app: &AppHandle) -> Result<(), String> {
    let label = "calendar-widget";
    if app.get_webview_window(label).is_some() {
        if let Some(w) = app.get_webview_window(label) {
            w.show().ok();
            w.set_focus().ok();
        }
        return Ok(());
    }
    WebviewWindowBuilder::new(app, label, WebviewUrl::App("index.html".into()))
        .title("桌面日历")
        .inner_size(560.0, 420.0)
        .min_inner_size(400.0, 300.0)
        .decorations(false)
        .skip_taskbar(true)
        .visible(true)
        .build()
        .map_err(|e| e.to_string())?;
    Ok(())
}
