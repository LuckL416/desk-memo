use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Group {
    pub id: String,
    pub name: String,
    pub sort_order: i32,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Note {
    pub id: String,
    pub group_id: Option<String>,
    pub r#type: String, // "text" | "todo" | "timer"
    pub title: Option<String>,
    pub content: Option<String>, // TipTap JSON or null
    pub bg_color: Option<String>,
    pub default_text_color: Option<String>,
    pub default_font_size: Option<i32>,
    pub created_at: String,
    pub updated_at: String,
    pub deleted_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimerState {
    pub note_id: String,
    pub daily_duration_minutes: i32,
    pub remaining_seconds: i32,
    pub is_running: bool,
    pub last_resume_at: Option<String>,
    pub tank_start_date: Option<String>,
    pub auto_start_time: Option<String>,
    pub warn_before_minutes: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reminder {
    pub id: String,
    pub note_id: String,
    pub remind_at: String,
    pub repeat_type: String, // "once" | "daily" | "weekly"
    pub repeat_days: Option<String>, // "1,3,5" for weekly
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowState {
    pub note_id: String,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub opacity: f64,
    pub pinned: bool,
    pub mode: String, // "desktop" | "top"
    pub is_visible: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Setting {
    pub key: String,
    pub value: String,
}
