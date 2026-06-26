# 桌面便签工具 — 实施计划

> **For agentic workers:** 使用 `superpowers:subagent-driven-development` 按任务执行。步骤使用 checkbox (`- [ ]`) 跟踪。

**目标:** 构建 Windows 桌面便签工具——Tauri 2.x + Vue 3 + SQLite，纯本地、极简、支持三类便签与日历视图。

**架构:** Rust 总管所有业务逻辑（计时/提醒/备份为独立线程），Vue 仅做视图渲染，SQLite WAL 模式为单一真相源。每条便签一个独立 WebView 窗口，无任务栏条目，系统托盘管理。

**技术栈:** Tauri 2.x, Vue 3 + Vite + TypeScript, TipTap, SQLite (rusqlite), Pinia, Inter + JetBrains Mono

---

## 文件结构

```
桌面便签/
├── src-tauri/
│   ├── src/
│   │   ├── main.rs              # 入口：托盘 + 窗口恢复 + 引擎启动
│   │   ├── db.rs                # SQLite 初始化、迁移、连接管理
│   │   ├── models.rs            # 所有数据结构定义
│   │   ├── commands/
│   │   │   ├── mod.rs
│   │   │   ├── groups.rs        # 分组 CRUD
│   │   │   ├── notes.rs         # 便签 CRUD + 搜索
│   │   │   ├── timers.rs        # 计时状态读写 + 控制
│   │   │   ├── reminders.rs     # 提醒 CRUD
│   │   │   ├── windows.rs       # 窗口状态 + 显示/隐藏
│   │   │   └── settings.rs      # 设置读写 + 导出/导入
│   │   ├── timer_engine.rs      # 计时引擎（独立线程）
│   │   ├── reminder_engine.rs   # 提醒引擎（独立线程）
│   │   ├── backup_engine.rs     # 备份引擎（独立线程）
│   │   ├── window_manager.rs    # 窗口创建/销毁/恢复/模式切换
│   │   └── tray.rs              # 系统托盘
│   ├── Cargo.toml
│   ├── tauri.conf.json
│   └── icons/                   # 应用图标
├── src/
│   ├── main.ts                  # Vue 入口，挂载 + 事件监听
│   ├── App.vue                  # 根组件（窗口类型路由）
│   ├── components/
│   │   ├── NoteWindow.vue       # 便签窗口容器（Chrome栏 + 编辑区）
│   │   ├── NoteChrome.vue       # 自定义 36px 标题栏
│   │   ├── TextNoteEditor.vue   # TipTap 文本便签编辑器
│   │   ├── TodoNoteEditor.vue   # 待办便签编辑器
│   │   ├── TimerNoteView.vue    # 计时便签视图
│   │   ├── TimerAdvancedSettings.vue  # 高级设置面板
│   │   ├── ManagementPanel.vue  # 管理面板主组件
│   │   ├── NoteList.vue         # 便签列表视图
│   │   ├── CalendarView.vue     # 日历视图
│   │   ├── GroupSidebar.vue     # 分组侧栏
│   │   ├── SettingsPanel.vue    # 设置窗口
│   │   ├── ReminderDialog.vue   # 提醒设置弹窗
│   │   ├── ColorPicker.vue      # 背景色选择器
│   │   └── OpacitySlider.vue    # 透明度滑块
│   ├── stores/
│   │   └── note.ts              # Pinia 本地状态
│   ├── styles/
│   │   ├── tokens.css           # Figma design tokens
│   │   └── global.css           # 全局样式
│   ├── utils/
│   │   └── tauri.ts             # invoke 封装 + 事件订阅
│   └── types/
│       └── index.ts             # TypeScript 类型定义
├── index.html
├── package.json
├── vite.config.ts
└── tsconfig.json
```

---

## Phase 1: 项目脚手架

### Task 1: 初始化 Tauri 2.x + Vue 3 项目

**Create:** 通过脚手架创建项目结构

- [ ] **Step 1: 创建 Vite + Vue 3 + TypeScript 项目**

```bash
cd "f:/学习/桌面便签"
npm create vite@latest . -- --template vue-ts
npm install
```

- [ ] **Step 2: 安装 Tauri CLI 和初始化**

```bash
npm install --save-dev @tauri-apps/cli@^2
npx tauri init
# 配置:
# - App name: 桌面便签
# - Window title: 桌面便签
# - Dev server URL: http://localhost:1420
# - Frontend dist: ../dist
# - Dev command: npm run dev
# - Build command: npm run build
```

- [ ] **Step 3: 安装 Tauri API 和插件**

```bash
npm install @tauri-apps/api@^2
npm install @tauri-apps/plugin-autostart @tauri-apps/plugin-global-shortcut @tauri-apps/plugin-notification @tauri-apps/plugin-window-state @tauri-apps/plugin-shell
```

- [ ] **Step 4: 安装 Vue 生态依赖**

```bash
npm install pinia @tiptap/vue-3 @tiptap/starter-kit @tiptap/extension-task-list @tiptap/extension-task-item @tiptap/extension-font-size @tiptap/extension-text-style @tiptap/extension-color @tiptap/extension-placeholder
```

- [ ] **Step 5: 验证项目能启动**

```bash
npm run tauri dev
# 预期: 出现空白 Tauri 窗口
```

---

## Phase 2: Rust 基础层 — 数据模型与数据库

### Task 2: 定义 Rust 数据结构

**Create:** `src-tauri/src/models.rs`

- [ ] **Step 1: 编写 models.rs**

```rust
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
    pub group_id: String,
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
    pub repeat_days: Option<String>, // "1,3,5"
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
```

- [ ] **Step 2: 在 Cargo.toml 添加依赖**

编辑 `src-tauri/Cargo.toml`：

```toml
[dependencies]
tauri = { version = "2", features = [] }
tauri-plugin-autostart = "2"
tauri-plugin-global-shortcut = "2"
tauri-plugin-notification = "2"
tauri-plugin-window-state = "2"
tauri-plugin-shell = "2"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
rusqlite = { version = "0.31", features = ["bundled"] }
chrono = { version = "0.4", features = ["serde"] }
uuid = { version = "1", features = ["v4"] }
tokio = { version = "1", features = ["full"] }
```

- [ ] **Step 3: 验证编译**

```bash
cd src-tauri && cargo check
# 预期: 编译成功（可能有 unused 警告）
```

---

### Task 3: 数据库初始化与迁移

**Create:** `src-tauri/src/db.rs`

- [ ] **Step 1: 编写 db.rs — 连接管理与建表**

```rust
use rusqlite::{Connection, Result};
use std::path::PathBuf;
use std::sync::Mutex;

pub struct Database {
    pub conn: Mutex<Connection>,
}

impl Database {
    pub fn new(data_dir: &PathBuf) -> Result<Self> {
        std::fs::create_dir_all(data_dir).ok();
        let db_path = data_dir.join("data.db");
        let conn = Connection::open(&db_path)?;
        conn.execute_batch("PRAGMA journal_mode=WAL;")?;
        conn.execute_batch("PRAGMA foreign_keys=ON;")?;
        let db = Database { conn: Mutex::new(conn) };
        db.run_migrations()?;
        Ok(db)
    }

    fn run_migrations(&self) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute_batch("
            CREATE TABLE IF NOT EXISTS groups (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                sort_order INTEGER DEFAULT 0,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS notes (
                id TEXT PRIMARY KEY,
                group_id TEXT,
                type TEXT NOT NULL CHECK(type IN ('text','todo','timer')),
                title TEXT,
                content TEXT,
                bg_color TEXT,
                default_text_color TEXT,
                default_font_size INTEGER,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                deleted_at TEXT,
                FOREIGN KEY (group_id) REFERENCES groups(id)
            );

            CREATE TABLE IF NOT EXISTS timer_state (
                note_id TEXT PRIMARY KEY,
                daily_duration_minutes INTEGER DEFAULT 480,
                remaining_seconds INTEGER DEFAULT 28800,
                is_running INTEGER DEFAULT 0,
                last_resume_at TEXT,
                tank_start_date TEXT,
                auto_start_time TEXT,
                warn_before_minutes INTEGER,
                FOREIGN KEY (note_id) REFERENCES notes(id)
            );

            CREATE TABLE IF NOT EXISTS reminders (
                id TEXT PRIMARY KEY,
                note_id TEXT NOT NULL,
                remind_at TEXT NOT NULL,
                repeat_type TEXT DEFAULT 'once',
                repeat_days TEXT,
                is_active INTEGER DEFAULT 1,
                FOREIGN KEY (note_id) REFERENCES notes(id)
            );

            CREATE TABLE IF NOT EXISTS window_state (
                note_id TEXT PRIMARY KEY,
                x INTEGER DEFAULT 100,
                y INTEGER DEFAULT 100,
                width INTEGER DEFAULT 320,
                height INTEGER DEFAULT 240,
                opacity REAL DEFAULT 1.0,
                pinned INTEGER DEFAULT 0,
                mode TEXT DEFAULT 'desktop',
                is_visible INTEGER DEFAULT 1
            );

            CREATE TABLE IF NOT EXISTS settings (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL
            );

            CREATE INDEX IF NOT EXISTS idx_notes_group ON notes(group_id);
            CREATE INDEX IF NOT EXISTS idx_notes_type ON notes(type);
            CREATE INDEX IF NOT EXISTS idx_notes_deleted ON notes(deleted_at);
            CREATE INDEX IF NOT EXISTS idx_reminders_note ON reminders(note_id);
            CREATE INDEX IF NOT EXISTS idx_reminders_active ON reminders(is_active);
        ")?;
        Ok(())
    }
}
```

- [ ] **Step 2: 在 main.rs 中初始化数据库**

编辑 `src-tauri/src/main.rs`：

```rust
mod db;
mod models;

use db::Database;
use std::path::PathBuf;

fn get_data_dir() -> PathBuf {
    // 先用默认路径，后续可从 settings 读取
    let appdata = std::env::var("APPDATA").unwrap_or_else(|_| ".".into());
    PathBuf::from(appdata).join("StickyNotes")
}

fn main() {
    let data_dir = get_data_dir();
    let database = Database::new(&data_dir).expect("Failed to initialize database");

    tauri::Builder::default()
        .manage(database)
        .manage(data_dir)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

---

## Phase 3: Rust 命令层 — 分组 + 便签 CRUD

### Task 4: 分组 CRUD 命令

**Create:** `src-tauri/src/commands/mod.rs`, `src-tauri/src/commands/groups.rs`

- [ ] **Step 1: commands/mod.rs**

```rust
pub mod groups;
pub mod notes;
pub mod timers;
pub mod reminders;
pub mod windows;
pub mod settings;
```

- [ ] **Step 2: commands/groups.rs — 完整 CRUD**

```rust
use tauri::State;
use crate::db::Database;
use crate::models::Group;
use chrono::Utc;
use uuid::Uuid;

#[tauri::command]
pub fn create_group(name: String, db: State<Database>) -> Result<Group, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();
    conn.execute(
        "INSERT INTO groups (id, name, created_at, updated_at) VALUES (?1, ?2, ?3, ?4)",
        rusqlite::params![id, name, now, now],
    ).map_err(|e| e.to_string())?;
    Ok(Group { id, name, sort_order: 0, created_at: now, updated_at: now })
}

#[tauri::command]
pub fn list_groups(db: State<Database>) -> Result<Vec<Group>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn.prepare(
        "SELECT id, name, sort_order, created_at, updated_at FROM groups ORDER BY sort_order"
    ).map_err(|e| e.to_string())?;
    let groups = stmt.query_map([], |row| {
        Ok(Group {
            id: row.get(0)?, name: row.get(1)?, sort_order: row.get(2)?,
            created_at: row.get(3)?, updated_at: row.get(4)?,
        })
    }).map_err(|e| e.to_string())?
    .filter_map(|r| r.ok())
    .collect();
    Ok(groups)
}

#[tauri::command]
pub fn rename_group(id: String, name: String, db: State<Database>) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let now = Utc::now().to_rfc3339();
    conn.execute("UPDATE groups SET name = ?1, updated_at = ?2 WHERE id = ?3",
        rusqlite::params![name, now, id]).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn delete_group(id: String, db: State<Database>) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    // 将该分组的便签移到未分组 (group_id = null)
    conn.execute("UPDATE notes SET group_id = NULL WHERE group_id = ?1",
        rusqlite::params![id]).map_err(|e| e.to_string())?;
    conn.execute("DELETE FROM groups WHERE id = ?1",
        rusqlite::params![id]).map_err(|e| e.to_string())?;
    Ok(())
}
```

---

### Task 5: 便签 CRUD + 搜索命令

**Create:** `src-tauri/src/commands/notes.rs`

- [ ] **Step 1: 便签创建、更新、查询、软删除**

```rust
use tauri::State;
use crate::db::Database;
use crate::models::Note;
use chrono::Utc;
use uuid::Uuid;

// 内部辅助函数，供托盘等无需 State 的上下文使用
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
        id, group_id, r#type: note_type, title, content: None,
        bg_color: None, default_text_color: None, default_font_size: None,
        created_at: now, updated_at: now, deleted_at: None,
    })
}

#[tauri::command]
pub fn create_note(group_id: Option<String>, note_type: String, title: Option<String>,
    db: State<Database>) -> Result<Note, String> {
    create_note_inner(&db, group_id, note_type, title)
}

#[tauri::command]
pub fn update_note(id: String, title: Option<String>, content: Option<String>,
    bg_color: Option<String>, default_text_color: Option<String>,
    default_font_size: Option<i32>, db: State<Database>) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let now = Utc::now().to_rfc3339();
    conn.execute(
        "UPDATE notes SET title=?1, content=?2, bg_color=?3, default_text_color=?4, default_font_size=?5, updated_at=?6 WHERE id=?7",
        rusqlite::params![title, content, bg_color, default_text_color, default_font_size, now, id],
    ).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn get_note(id: String, db: State<Database>) -> Result<Note, String> {
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
    include_deleted: bool, db: State<Database>) -> Result<Vec<Note>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let mut sql = String::from(
        "SELECT id, group_id, type, title, content, bg_color, default_text_color, default_font_size, created_at, updated_at, deleted_at FROM notes WHERE 1=1"
    );
    let mut params: Vec<Box<dyn rusqlite::types::ToSql>> = Vec::new();
    if !include_deleted { sql.push_str(" AND deleted_at IS NULL"); }
    if let Some(ref gid) = group_id { sql.push_str(" AND group_id = ?"); params.push(Box::new(gid.clone())); }
    if let Some(ref t) = note_type { sql.push_str(" AND type = ?"); params.push(Box::new(t.clone())); }
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
pub fn delete_note(id: String, db: State<Database>) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let now = Utc::now().to_rfc3339();
    conn.execute("UPDATE notes SET deleted_at = ?1, updated_at = ?2 WHERE id = ?3",
        rusqlite::params![now, now, id]).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn restore_note(id: String, db: State<Database>) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    conn.execute("UPDATE notes SET deleted_at = NULL WHERE id = ?1",
        rusqlite::params![id]).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn permanently_delete_note(id: String, db: State<Database>) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    conn.execute("DELETE FROM timer_state WHERE note_id = ?1", rusqlite::params![id]).ok();
    conn.execute("DELETE FROM reminders WHERE note_id = ?1", rusqlite::params![id]).ok();
    conn.execute("DELETE FROM window_state WHERE note_id = ?1", rusqlite::params![id]).ok();
    conn.execute("DELETE FROM notes WHERE id = ?1", rusqlite::params![id]).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn search_notes(query: String, db: State<Database>) -> Result<Vec<Note>, String> {
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
```

---

### Task 6: 注册所有命令到 main.rs

**Modify:** `src-tauri/src/main.rs`

- [ ] **Step 1: 更新 main.rs 注册命令**

```rust
mod commands;

// ... 在 tauri::Builder 中添加:
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
```

---

## Phase 4: Rust 引擎层 — 计时、提醒、备份

### Task 7: 计时引擎

**Create:** `src-tauri/src/timer_engine.rs`

- [ ] **Step 1: 计时引擎实现**

```rust
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use tauri::{AppHandle, Emitter};
use chrono::{Utc, Timelike};
use crate::db::Database;

pub fn start_timer_engine(app: AppHandle, db: Arc<Database>) {
    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_secs(1));
            let conn = match db.conn.lock() {
                Ok(c) => c,
                Err(_) => continue,
            };

            // 检查是否跨日，重置所有活跃计时
            let now = Utc::now();
            if now.hour() == 0 && now.minute() == 0 && now.second() < 2 {
                conn.execute_batch(
                    "UPDATE timer_state SET remaining_seconds = daily_duration_minutes * 60, is_running = 0"
                ).ok();
                // 发送重置事件
                let _ = app.emit("midnight-reset", ());
            }

            // 获取所有正在运行的计时
            let mut stmt = match conn.prepare(
                "SELECT ts.note_id, ts.remaining_seconds, ts.daily_duration_minutes, ts.warn_before_minutes, ts.tank_start_date
                 FROM timer_state ts JOIN notes n ON ts.note_id = n.id
                 WHERE ts.is_running = 1 AND n.deleted_at IS NULL"
            ) { Ok(s) => s, Err(_) => continue };

            let timers: Vec<(String, i32, i32, Option<i32>, Option<String>)> = stmt
                .query_map([], |row| Ok((
                    row.get::<_,String>(0)?, row.get(1)?, row.get(2)?, row.get(3)?, row.get(4)?
                ))).ok().map(|r| r.filter_map(|x| x.ok()).collect()).unwrap_or_default();

            for (note_id, remaining, daily_min, warn_min, tank_date) in timers {
                if remaining <= 0 {
                    // 倒计时结束
                    conn.execute("UPDATE timer_state SET is_running = 0, remaining_seconds = 0 WHERE note_id = ?1",
                        rusqlite::params![note_id]).ok();
                    let _ = app.emit("timer-finished", serde_json::json!({ "note_id": note_id }));
                } else {
                    let new_remaining = remaining - 1;
                    conn.execute("UPDATE timer_state SET remaining_seconds = ?1 WHERE note_id = ?2",
                        rusqlite::params![new_remaining, note_id]).ok();

                    let _ = app.emit("timer-tick", serde_json::json!({
                        "note_id": note_id,
                        "remaining_seconds": new_remaining,
                        "is_running": true,
                    }));

                    // 检查提前提醒阈值
                    if let Some(wm) = warn_min {
                        let warn_seconds = wm * 60;
                        if new_remaining == warn_seconds {
                            let _ = app.emit("timer-warning", serde_json::json!({
                                "note_id": note_id,
                                "minutes_left": wm,
                            }));
                        }
                    }
                }
            }

            // 检查开缸里程碑
            if now.hour() == 8 && now.minute() == 0 {
                check_milestones(&conn, &app);
            }
        }
    });
}

fn check_milestones(conn: &std::sync::MutexGuard<rusqlite::Connection>, app: &AppHandle) {
    let today = Utc::now().date_naive();
    let mut stmt = match conn.prepare(
        "SELECT note_id, tank_start_date FROM timer_state WHERE tank_start_date IS NOT NULL"
    ) { Ok(s) => s, Err(_) => return };

    let milestones: Vec<(String, i64)> = stmt.query_map([], |row| {
        let note_id: String = row.get(0)?;
        let start_date_str: String = row.get(1)?;
        if let Ok(start_date) = chrono::NaiveDate::parse_from_str(&start_date_str, "%Y-%m-%d") {
            let days = (today - start_date).num_days();
            Ok((note_id, days))
        } else {
            Err(rusqlite::Error::InvalidQuery)
        }
    }).ok().map(|r| r.filter_map(|x| x.ok()).collect()).unwrap_or_default();

    for (note_id, days) in milestones {
        if [30, 100, 365].contains(&days) {
            let _ = app.emit("tank-milestone", serde_json::json!({
                "note_id": note_id,
                "days": days,
            }));
        }
    }
}
```

---

### Task 8: 计时便签控制命令

**Create:** `src-tauri/src/commands/timers.rs`

- [ ] **Step 1: 计时状态读写命令**

```rust
use tauri::State;
use crate::db::Database;
use crate::models::TimerState;
use chrono::Utc;

#[tauri::command]
pub fn get_timer_state(note_id: String, db: State<Database>) -> Result<TimerState, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    conn.query_row("SELECT note_id, daily_duration_minutes, remaining_seconds, is_running, last_resume_at, tank_start_date, auto_start_time, warn_before_minutes FROM timer_state WHERE note_id=?1",
        rusqlite::params![note_id], |row| {
            Ok(TimerState {
                note_id: row.get(0)?,
                daily_duration_minutes: row.get(1)?,
                remaining_seconds: row.get(2)?,
                is_running: row.get::<_,i32>(3)? != 0,
                last_resume_at: row.get(4)?,
                tank_start_date: row.get(5)?,
                auto_start_time: row.get(6)?,
                warn_before_minutes: row.get(7)?,
            })
        }).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn start_timer(note_id: String, db: State<Database>) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let now = Utc::now().to_rfc3339();
    conn.execute("UPDATE timer_state SET is_running = 1, last_resume_at = ?1 WHERE note_id = ?2",
        rusqlite::params![now, note_id]).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn pause_timer(note_id: String, db: State<Database>) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    conn.execute("UPDATE timer_state SET is_running = 0 WHERE note_id = ?1",
        rusqlite::params![note_id]).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn reset_timer(note_id: String, db: State<Database>) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    conn.execute(
        "UPDATE timer_state SET remaining_seconds = daily_duration_minutes * 60, is_running = 0 WHERE note_id = ?1",
        rusqlite::params![note_id]).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn update_timer_settings(note_id: String, daily_duration_minutes: Option<i32>,
    tank_start_date: Option<String>, auto_start_time: Option<String>,
    warn_before_minutes: Option<i32>, db: State<Database>) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    if let Some(d) = daily_duration_minutes {
        conn.execute("UPDATE timer_state SET daily_duration_minutes = ?1, remaining_seconds = ?2 WHERE note_id = ?3",
            rusqlite::params![d, d * 60, note_id]).map_err(|e| e.to_string())?;
    }
    if let Some(ref d) = tank_start_date {
        conn.execute("UPDATE timer_state SET tank_start_date = ?1 WHERE note_id = ?2",
            rusqlite::params![d, note_id]).map_err(|e| e.to_string())?;
    }
    if let Some(ref t) = auto_start_time {
        conn.execute("UPDATE timer_state SET auto_start_time = ?1 WHERE note_id = ?2",
            rusqlite::params![t, note_id]).map_err(|e| e.to_string())?;
    }
    if let Some(w) = warn_before_minutes {
        conn.execute("UPDATE timer_state SET warn_before_minutes = ?1 WHERE note_id = ?2",
            rusqlite::params![w, note_id]).map_err(|e| e.to_string())?;
    }
    Ok(())
}
```

---

### Task 9: 提醒引擎 + 备份引擎

**Create:** `src-tauri/src/reminder_engine.rs`, `src-tauri/src/backup_engine.rs`, `src-tauri/src/commands/reminders.rs`

- [ ] **Step 1: reminder_engine.rs**

```rust
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use tauri::{AppHandle, Emitter};
use chrono::Utc;
use crate::db::Database;

pub fn start_reminder_engine(app: AppHandle, db: Arc<Database>) {
    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_secs(60));
            let conn = match db.conn.lock() { Ok(c) => c, Err(_) => continue };
            let now = Utc::now().to_rfc3339();

            let mut stmt = match conn.prepare(
                "SELECT id, note_id, repeat_type, remind_at FROM reminders WHERE is_active = 1 AND remind_at <= ?1"
            ) { Ok(s) => s, Err(_) => continue };

            let due: Vec<(String, String, String, String)> = stmt.query_map(
                rusqlite::params![now], |row| Ok((
                    row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?
                ))
            ).ok().map(|r| r.filter_map(|x| x.ok()).collect()).unwrap_or_default();

            for (id, note_id, repeat_type, _remind_at) in due {
                let _ = app.emit("reminder-triggered", serde_json::json!({
                    "reminder_id": id,
                    "note_id": note_id,
                }));

                match repeat_type.as_str() {
                    "once" => {
                        conn.execute("UPDATE reminders SET is_active = 0 WHERE id = ?1",
                            rusqlite::params![id]).ok();
                    }
                    "daily" => {
                        let next = Utc::now() + chrono::Duration::days(1);
                        conn.execute("UPDATE reminders SET remind_at = ?1 WHERE id = ?2",
                            rusqlite::params![next.to_rfc3339(), id]).ok();
                    }
                    "weekly" => {
                        let next = Utc::now() + chrono::Duration::weeks(1);
                        conn.execute("UPDATE reminders SET remind_at = ?1 WHERE id = ?2",
                            rusqlite::params![next.to_rfc3339(), id]).ok();
                    }
                    _ => {}
                }
            }
        }
    });
}
```

- [ ] **Step 2: backup_engine.rs**

```rust
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use chrono::Utc;

pub fn start_backup_engine(data_dir: PathBuf) {
    thread::spawn(move || {
        loop {
            // 每小时检查一次是否到凌晨3点
            thread::sleep(Duration::from_secs(3600));
            let now = Utc::now();
            if now.hour() == 3 {
                let backup_dir = data_dir.join("backups");
                fs::create_dir_all(&backup_dir).ok();
                let db_path = data_dir.join("data.db");
                let backup_name = format!("backup-{}.db", now.format("%Y-%m-%d"));
                let backup_path = backup_dir.join(&backup_name);
                fs::copy(&db_path, &backup_path).ok();

                // 保留最近 7 份
                if let Ok(entries) = fs::read_dir(&backup_dir) {
                    let mut files: Vec<_> = entries.filter_map(|e| e.ok())
                        .filter(|e| e.file_name().to_string_lossy().starts_with("backup-"))
                        .collect();
                    files.sort_by_key(|e| e.metadata().and_then(|m| m.modified()).ok());
                    while files.len() > 7 {
                        if let Some(old) = files.first() {
                            fs::remove_file(old.path()).ok();
                            files.remove(0);
                        }
                    }
                }
            }
        }
    });
}
```

- [ ] **Step 3: commands/reminders.rs**

```rust
use tauri::State;
use crate::db::Database;
use crate::models::Reminder;
use uuid::Uuid;

#[tauri::command]
pub fn set_reminder(note_id: String, remind_at: String, repeat_type: String,
    repeat_days: Option<String>, db: State<Database>) -> Result<Reminder, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let id = Uuid::new_v4().to_string();
    conn.execute(
        "INSERT INTO reminders (id, note_id, remind_at, repeat_type, repeat_days) VALUES (?1,?2,?3,?4,?5)",
        rusqlite::params![id, note_id, remind_at, repeat_type, repeat_days],
    ).map_err(|e| e.to_string())?;
    Ok(Reminder { id, note_id, remind_at, repeat_type, repeat_days, is_active: true })
}

#[tauri::command]
pub fn get_reminders(note_id: String, db: State<Database>) -> Result<Vec<Reminder>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn.prepare(
        "SELECT id, note_id, remind_at, repeat_type, repeat_days, is_active FROM reminders WHERE note_id=?1"
    ).map_err(|e| e.to_string())?;
    let reminders = stmt.query_map(rusqlite::params![note_id], |row| {
        Ok(Reminder {
            id: row.get(0)?, note_id: row.get(1)?, remind_at: row.get(2)?,
            repeat_type: row.get(3)?, repeat_days: row.get(4)?,
            is_active: row.get::<_,i32>(5)? != 0,
        })
    }).map_err(|e| e.to_string())?.filter_map(|r| r.ok()).collect();
    Ok(reminders)
}

#[tauri::command]
pub fn delete_reminder(id: String, db: State<Database>) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    conn.execute("DELETE FROM reminders WHERE id=?1", rusqlite::params![id]).map_err(|e| e.to_string())?;
    Ok(())
}
```

---

### Task 10: 窗口管理 + 系统托盘

**Create:** `src-tauri/src/window_manager.rs`, `src-tauri/src/tray.rs`, `src-tauri/src/commands/windows.rs`

- [ ] **Step 1: window_manager.rs — 创建/销毁/模式切换**

```rust
use tauri::{AppHandle, WebviewUrl, WebviewWindowBuilder, Manager};
use crate::models::{Note, WindowState};

pub fn create_note_window(app: &AppHandle, note: &Note) -> Result<(), String> {
    let label = format!("note-{}", note.id);
    if app.get_webview_window(&label).is_some() { return Ok(()); }

    let bg = note.bg_color.clone().unwrap_or_else(|| "#ffffff".into());
    let title = note.title.clone().unwrap_or_else(|| "".into());

    let window = WebviewWindowBuilder::new(app, &label, WebviewUrl::App("index.html".into()))
        .title("桌面便签")
        .inner_size(320.0, 240.0)
        .min_inner_size(200.0, 120.0)
        .decorations(false) // 无原生标题栏
        .skip_taskbar(true) // 无任务栏条目
        .transparent(true)
        .visible(true)
        .build()
        .map_err(|e| e.to_string())?;

    // 使用 JavaScript 初始化便签数据
    let _ = window.eval(&format!(
        "window.__noteId = '{}'; window.__noteType = '{}'; window.__noteBg = '{}'; window.__noteTitle = '{}';",
        note.id, note.r#type, bg, title
    ));

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

pub fn toggle_note_window_mode(app: &AppHandle, note_id: &str, mode: &str) -> Result<(), String> {
    let label = format!("note-{}", note_id);
    if let Some(window) = app.get_webview_window(&label) {
        match mode {
            "desktop" => { window.set_always_on_top(false).ok(); }
            "top" => { window.set_always_on_top(true).ok(); }
            _ => {}
        }
    }
    Ok(())
}

pub fn set_note_window_opacity(app: &AppHandle, note_id: &str, opacity: f64) -> Result<(), String> {
    let label = format!("note-{}", note_id);
    if let Some(window) = app.get_webview_window(&label) {
        let clamped = opacity.max(0.05).min(1.0);
        window.set_opacity(clamped).ok();
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

pub fn create_management_panel(app: &AppHandle) -> Result<(), String> {
    let label = "management-panel";
    if app.get_webview_window(label).is_some() { return Ok(()); }
    let _window = WebviewWindowBuilder::new(app, label, WebviewUrl::App("index.html".into()))
        .title("便签管理")
        .inner_size(800.0, 500.0)
        .min_inner_size(480.0, 360.0)
        .decorations(false)
        .visible(true)
        .build()
        .map_err(|e| e.to_string())?;
    let _ = app.get_webview_window(label).unwrap().eval("window.__windowType = 'management';");
    Ok(())
}

pub fn create_settings_panel(app: &AppHandle) -> Result<(), String> {
    let label = "settings-panel";
    if app.get_webview_window(label).is_some() { return Ok(()); }
    let _window = WebviewWindowBuilder::new(app, label, WebviewUrl::App("index.html".into()))
        .title("软件设置")
        .inner_size(500.0, 400.0)
        .decorations(false)
        .visible(true)
        .build()
        .map_err(|e| e.to_string())?;
    let _ = app.get_webview_window(label).unwrap().eval("window.__windowType = 'settings';");
    Ok(())
}

pub fn show_management_panel(app: &AppHandle) -> Result<(), String> {
    create_management_panel(app)?;
    let label = "management-panel";
    if let Some(window) = app.get_webview_window(label) {
        window.show().ok(); window.set_focus().ok();
    }
    Ok(())
}
```

- [ ] **Step 2: commands/windows.rs**

```rust
use tauri::State;
use crate::db::Database;
use crate::models::WindowState;

#[tauri::command]
pub fn save_window_state(state: WindowState, db: State<Database>) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    conn.execute(
        "INSERT OR REPLACE INTO window_state (note_id, x, y, width, height, opacity, pinned, mode, is_visible) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9)",
        rusqlite::params![state.note_id, state.x, state.y, state.width, state.height, state.opacity, state.pinned as i32, state.mode, state.is_visible as i32],
    ).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn get_window_state(note_id: String, db: State<Database>) -> Result<Option<WindowState>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let result = conn.query_row(
        "SELECT note_id, x, y, width, height, opacity, pinned, mode, is_visible FROM window_state WHERE note_id=?1",
        rusqlite::params![note_id], |row| {
            Ok(WindowState {
                note_id: row.get(0)?, x: row.get(1)?, y: row.get(2)?,
                width: row.get(3)?, height: row.get(4)?, opacity: row.get(5)?,
                pinned: row.get::<_,i32>(6)? != 0, mode: row.get(7)?,
                is_visible: row.get::<_,i32>(8)? != 0,
            })
        });
    match result {
        Ok(s) => Ok(Some(s)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub fn get_all_window_states(db: State<Database>) -> Result<Vec<WindowState>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn.prepare(
        "SELECT note_id, x, y, width, height, opacity, pinned, mode, is_visible FROM window_state"
    ).map_err(|e| e.to_string())?;
    let states = stmt.query_map([], |row| {
        Ok(WindowState {
            note_id: row.get(0)?, x: row.get(1)?, y: row.get(2)?,
            width: row.get(3)?, height: row.get(4)?, opacity: row.get(5)?,
            pinned: row.get::<_,i32>(6)? != 0, mode: row.get(7)?,
            is_visible: row.get::<_,i32>(8)? != 0,
        })
    }).map_err(|e| e.to_string())?.filter_map(|r| r.ok()).collect();
    Ok(states)
}
```

- [ ] **Step 3: commands/settings.rs — 设置和导出/导入**

```rust
use tauri::State;
use crate::db::Database;
use std::fs;

#[tauri::command]
pub fn set_setting(key: String, value: String, db: State<Database>) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    conn.execute("INSERT OR REPLACE INTO settings (key, value) VALUES (?1, ?2)",
        rusqlite::params![key, value]).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn get_setting(key: String, db: State<Database>) -> Result<Option<String>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    match conn.query_row("SELECT value FROM settings WHERE key=?1",
        rusqlite::params![key], |row| row.get(0)) {
        Ok(v) => Ok(Some(v)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub fn export_all_data(path: String, db: State<Database>) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    // 手动查询所有表数据并构建 JSON
    let groups: Vec<crate::models::Group> = {
        let mut stmt = conn.prepare("SELECT id, name, sort_order, created_at, updated_at FROM groups").map_err(|e| e.to_string())?;
        stmt.query_map([], |row| {
            Ok(crate::models::Group { id: row.get(0)?, name: row.get(1)?, sort_order: row.get(2)?, created_at: row.get(3)?, updated_at: row.get(4)? })
        }).map_err(|e| e.to_string())?.filter_map(|r| r.ok()).collect()
    };
    let notes: Vec<crate::models::Note> = {
        let mut stmt = conn.prepare("SELECT id, group_id, type, title, content, bg_color, default_text_color, default_font_size, created_at, updated_at, deleted_at FROM notes").map_err(|e| e.to_string())?;
        stmt.query_map([], |row| Ok(crate::models::Note {
            id: row.get(0)?, group_id: row.get(1)?, r#type: row.get(2)?, title: row.get(3)?, content: row.get(4)?,
            bg_color: row.get(5)?, default_text_color: row.get(6)?, default_font_size: row.get(7)?,
            created_at: row.get(8)?, updated_at: row.get(9)?, deleted_at: row.get(10)?,
        })).map_err(|e| e.to_string())?.filter_map(|r| r.ok()).collect()
    };
    let reminders: Vec<crate::models::Reminder> = {
        let mut stmt = conn.prepare("SELECT id, note_id, remind_at, repeat_type, repeat_days, is_active FROM reminders").map_err(|e| e.to_string())?;
        stmt.query_map([], |row| Ok(crate::models::Reminder {
            id: row.get(0)?, note_id: row.get(1)?, remind_at: row.get(2)?, repeat_type: row.get(3)?, repeat_days: row.get(4)?, is_active: row.get::<_,i32>(5)? != 0,
        })).map_err(|e| e.to_string())?.filter_map(|r| r.ok()).collect()
    };
    let settings: Vec<crate::models::Setting> = {
        let mut stmt = conn.prepare("SELECT key, value FROM settings").map_err(|e| e.to_string())?;
        stmt.query_map([], |row| Ok(crate::models::Setting { key: row.get(0)?, value: row.get(1)? })).map_err(|e| e.to_string())?.filter_map(|r| r.ok()).collect()
    };

    let export = serde_json::json!({ "groups": groups, "notes": notes, "reminders": reminders, "settings": settings });
    let json = serde_json::to_string_pretty(&export).map_err(|e| e.to_string())?;
    fs::write(&path, json).map_err(|e| e.to_string())?;
    Ok(())
}
```

---

### Task 11: 系统托盘

**Create:** `src-tauri/src/tray.rs`

- [ ] **Step 1: 构建系统托盘菜单**

```rust
use tauri::{
    AppHandle, Manager,
    menu::{MenuBuilder, MenuItemBuilder},
    tray::{TrayIconBuilder, MouseButton, MouseButtonState, TrayIconEvent},
};

pub fn setup_tray(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    let new_text = MenuItemBuilder::with_id("new_text", "新建普通便签").build(app)?;
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
            match event.id().as_ref() {
                "new_text" => {
                    let db = app.state::<crate::db::Database>();
                    if let Ok(note) = crate::commands::notes::create_note_inner(&db, None, "text".into(), None) {
                        crate::window_manager::create_note_window(app, &note).ok();
                    }
                }
                "new_todo" => {
                    let db = app.state::<crate::db::Database>();
                    if let Ok(note) = crate::commands::notes::create_note_inner(&db, None, "todo".into(), None) {
                        crate::window_manager::create_note_window(app, &note).ok();
                    }
                }
                "new_timer" => {
                    let db = app.state::<crate::db::Database>();
                    if let Ok(note) = crate::commands::notes::create_note_inner(&db, None, "timer".into(), None) {
                        crate::window_manager::create_note_window(app, &note).ok();
                    }
                }
                "toggle_all" => { /* 由前端管理面板处理 */ }
                "management" => { crate::window_manager::show_management_panel(app).ok(); }
                "settings" => { crate::window_manager::create_settings_panel(app).ok(); }
                "quit" => { app.exit(0); }
                _ => {}
            }
        })
        .build(app)?;

    Ok(())
}
```

---

## Phase 5: Vue 前端 — 基础设施

### Task 12: TypeScript 类型 + Design Tokens + invoke 封装

**Create:** `src/types/index.ts`, `src/styles/tokens.css`, `src/utils/tauri.ts`

- [ ] **Step 1: src/types/index.ts**

```typescript
export interface Group {
  id: string
  name: string
  sort_order: number
  created_at: string
  updated_at: string
}

export interface Note {
  id: string
  group_id: string | null
  type: 'text' | 'todo' | 'timer'
  title: string | null
  content: string | null
  bg_color: string | null
  default_text_color: string | null
  default_font_size: number | null
  created_at: string
  updated_at: string
  deleted_at: string | null
}

export interface TimerState {
  note_id: string
  daily_duration_minutes: number
  remaining_seconds: number
  is_running: boolean
  last_resume_at: string | null
  tank_start_date: string | null
  auto_start_time: string | null
  warn_before_minutes: number | null
}

export interface Reminder {
  id: string
  note_id: string
  remind_at: string
  repeat_type: 'once' | 'daily' | 'weekly'
  repeat_days: string | null
  is_active: boolean
}

export interface WindowState {
  note_id: string
  x: number
  y: number
  width: number
  height: number
  opacity: number
  pinned: boolean
  mode: 'desktop' | 'top'
  is_visible: boolean
}

export const PASTEL_COLORS = [
  { name: 'cream', hex: '#f4ecd6', label: '暖米色' },
  { name: 'lime', hex: '#dceeb1', label: '青柠' },
  { name: 'lilac', hex: '#c5b0f4', label: '薰衣草' },
  { name: 'mint', hex: '#c8e6cd', label: '薄荷' },
  { name: 'pink', hex: '#efd4d4', label: '粉红' },
  { name: 'coral', hex: '#f3c9b6', label: '珊瑚' },
  { name: 'navy', hex: '#1f1d3d', label: '深蓝' },
  { name: 'white', hex: '#ffffff', label: '纯白' },
] as const
```

- [ ] **Step 2: src/styles/tokens.css**

```css
:root {
  --color-primary: #000000;
  --color-on-primary: #ffffff;
  --color-canvas: #ffffff;
  --color-ink: #000000;
  --color-surface-soft: #f7f7f5;
  --color-hairline: #e6e6e6;
  --color-block-lime: #dceeb1;
  --color-block-lilac: #c5b0f4;
  --color-block-cream: #f4ecd6;
  --color-block-mint: #c8e6cd;
  --color-block-pink: #efd4d4;
  --color-block-coral: #f3c9b6;
  --color-block-navy: #1f1d3d;
  --color-accent-magenta: #ff3d8b;
  --color-success: #1ea64a;

  --font-sans: 'Inter', system-ui, -apple-system, sans-serif;
  --font-mono: 'JetBrains Mono', 'SF Mono', monospace;

  --rounded-xs: 2px;
  --rounded-sm: 6px;
  --rounded-md: 8px;
  --rounded-lg: 24px;
  --rounded-pill: 50px;
  --rounded-full: 9999px;

  --spacing-xs: 8px;
  --spacing-sm: 12px;
  --spacing-md: 16px;
  --spacing-lg: 24px;
  --spacing-xl: 32px;

  --chrome-height: 36px;
}

* { margin: 0; padding: 0; box-sizing: border-box; }
body { font-family: var(--font-sans); color: var(--color-ink); background: transparent; overflow: hidden; user-select: none; }
```

- [ ] **Step 3: src/utils/tauri.ts**

```typescript
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import type { Group, Note, TimerState, Reminder, WindowState } from '../types'

// ── Groups ──
export const createGroup = (name: string) => invoke<Group>('create_group', { name })
export const listGroups = () => invoke<Group[]>('list_groups')
export const renameGroup = (id: string, name: string) => invoke('rename_group', { id, name })
export const deleteGroup = (id: string) => invoke('delete_group', { id })

// ── Notes ──
export const createNote = (groupId: string | null, noteType: string, title: string | null) =>
  invoke<Note>('create_note', { groupId, noteType, title })
export const updateNote = (id: string, data: Partial<Pick<Note, 'title' | 'content' | 'bg_color' | 'default_text_color' | 'default_font_size'>>) =>
  invoke('update_note', { id, ...data })
export const getNote = (id: string) => invoke<Note>('get_note', { id })
export const listNotes = (groupId?: string, noteType?: string, includeDeleted = false) =>
  invoke<Note[]>('list_notes', { groupId: groupId ?? null, noteType: noteType ?? null, includeDeleted })
export const deleteNote = (id: string) => invoke('delete_note', { id })
export const restoreNote = (id: string) => invoke('restore_note', { id })
export const permanentlyDeleteNote = (id: string) => invoke('permanently_delete_note', { id })
export const searchNotes = (query: string) => invoke<Note[]>('search_notes', { query })

// ── Timers ──
export const getTimerState = (noteId: string) => invoke<TimerState>('get_timer_state', { noteId })
export const startTimer = (noteId: string) => invoke('start_timer', { noteId })
export const pauseTimer = (noteId: string) => invoke('pause_timer', { noteId })
export const resetTimer = (noteId: string) => invoke('reset_timer', { noteId })
export const updateTimerSettings = (noteId: string, data: any) => invoke('update_timer_settings', { noteId, ...data })

// ── Reminders ──
export const setReminder = (noteId: string, remindAt: string, repeatType: string, repeatDays?: string) =>
  invoke<Reminder>('set_reminder', { noteId, remindAt, repeatType, repeatDays: repeatDays ?? null })
export const getReminders = (noteId: string) => invoke<Reminder[]>('get_reminders', { noteId })
export const deleteReminder = (id: string) => invoke('delete_reminder', { id })

// ── Windows ──
export const saveWindowState = (state: WindowState) => invoke('save_window_state', { state })
export const getWindowState = (noteId: string) => invoke<WindowState | null>('get_window_state', { noteId })
export const getAllWindowStates = () => invoke<WindowState[]>('get_all_window_states')

// ── Settings ──
export const setSetting = (key: string, value: string) => invoke('set_setting', { key, value })
export const getSetting = (key: string) => invoke<string | null>('get_setting', { key })
export const exportAllData = (path: string) => invoke('export_all_data', { path })

// ── Event listeners ──
export const onTimerTick = (cb: (data: { note_id: string; remaining_seconds: number; is_running: boolean }) => void) =>
  listen<{ note_id: string; remaining_seconds: number; is_running: boolean }>('timer-tick', (e) => cb(e.payload))

export const onTimerFinished = (cb: (data: { note_id: string }) => void) =>
  listen<{ note_id: string }>('timer-finished', (e) => cb(e.payload))

export const onTimerWarning = (cb: (data: { note_id: string; minutes_left: number }) => void) =>
  listen<{ note_id: string; minutes_left: number }>('timer-warning', (e) => cb(e.payload))

export const onReminderTriggered = (cb: (data: { reminder_id: string; note_id: string }) => void) =>
  listen<{ reminder_id: string; note_id: string }>('reminder-triggered', (e) => cb(e.payload))

export const onTankMilestone = (cb: (data: { note_id: string; days: number }) => void) =>
  listen<{ note_id: string; days: number }>('tank-milestone', (e) => cb(e.payload))

export const onMidnightReset = (cb: () => void) =>
  listen('midnight-reset', () => cb())
```

---

## Phase 6: Vue — 便签窗口核心组件

### Task 13: NoteWindow.vue + NoteChrome.vue + App.vue

**Create:** `src/App.vue`, `src/components/NoteWindow.vue`, `src/components/NoteChrome.vue`

- [ ] **Step 1: App.vue — 窗口入口路由**

```vue
<script setup lang="ts">
import { ref, onMounted } from 'vue'
import NoteWindow from './components/NoteWindow.vue'
import ManagementPanel from './components/ManagementPanel.vue'
import SettingsPanel from './components/SettingsPanel.vue'

const windowType = ref<'note' | 'management' | 'settings'>('note')

onMounted(() => {
  const w = window as any
  if (w.__noteId) {
    windowType.value = 'note'
  } else if (w.__windowType === 'management') {
    windowType.value = 'management'
  } else if (w.__windowType === 'settings') {
    windowType.value = 'settings'
  }
})
</script>

<template>
  <NoteWindow v-if="windowType === 'note'" />
  <ManagementPanel v-else-if="windowType === 'management'" />
  <SettingsPanel v-else-if="windowType === 'settings'" />
</template>
```

- [ ] **Step 2: NoteChrome.vue — 36px 自定义标题栏**

```vue
<script setup lang="ts">
import { ref } from 'vue'

const props = defineProps<{
  title: string
  mode: 'desktop' | 'top'
  pinned: boolean
  opacity: number
}>()

const emit = defineEmits<{
  close: []
  toggleMode: []
  togglePinned: []
  updateOpacity: [value: number]
  showMenu: [event: MouseEvent]
}>()

const showOpacity = ref(false)
</script>

<template>
  <div class="chrome-bar" data-tauri-drag-region>
    <span class="chrome-title">{{ title || '无标题' }}</span>
    <div class="chrome-actions">
      <button class="chrome-btn" @click="showOpacity = !showOpacity" title="透明度">◐</button>
      <button class="chrome-btn" @click="emit('togglePinned')" :title="pinned ? '解锁' : '锁定'">
        {{ pinned ? '🔒' : '🔓' }}
      </button>
      <button class="chrome-btn" @click="emit('toggleMode')" :title="mode === 'desktop' ? '切换到置顶' : '切换到底层'">
        📌
      </button>
      <button class="chrome-btn" @click="emit('showMenu', $event)" title="更多">⋯</button>
      <button class="chrome-btn chrome-close" @click="emit('close')" title="隐藏">✕</button>
    </div>
    <div v-if="showOpacity" class="opacity-dropdown">
      <input type="range" min="5" max="100" :value="Math.round(opacity * 100)"
        @input="emit('updateOpacity', Number(($event.target as HTMLInputElement).value) / 100)" />
      <span>{{ Math.round(opacity * 100) }}%</span>
    </div>
  </div>
</template>

<style scoped>
.chrome-bar {
  display: flex; align-items: center; height: var(--chrome-height);
  padding: 0 8px; gap: 6px; background: rgba(0,0,0,0.04);
  cursor: move; position: relative;
}
.chrome-title {
  font-size: 13px; font-weight: 600; margin-left: 8px;
  white-space: nowrap; overflow: hidden; text-overflow: ellipsis; max-width: 160px;
}
.chrome-actions { display: flex; gap: 4px; margin-left: auto; }
.chrome-btn {
  background: rgba(0,0,0,0.06); border: none; cursor: pointer;
  font-size: 12px; padding: 2px 6px; border-radius: var(--rounded-pill);
  color: var(--color-ink); line-height: 1.4;
}
.chrome-btn:hover { background: rgba(0,0,0,0.12); }
.chrome-close:hover { background: #ff3d8b; color: #fff; }
.opacity-dropdown {
  position: absolute; top: 100%; right: 8px; background: #fff;
  border: 1px solid var(--color-hairline); border-radius: var(--rounded-md);
  padding: 8px 12px; display: flex; align-items: center; gap: 8px;
  font-size: 12px; z-index: 100; box-shadow: 0 2px 8px rgba(0,0,0,0.08);
}
</style>
```

- [ ] **Step 3: NoteWindow.vue — 便签窗口容器**

```vue
<script setup lang="ts">
import { ref, onMounted, onUnmounted, watch } from 'vue'
import NoteChrome from './NoteChrome.vue'
import TextNoteEditor from './TextNoteEditor.vue'
import TodoNoteEditor from './TodoNoteEditor.vue'
import TimerNoteView from './TimerNoteView.vue'
import * as api from '../utils/tauri'
import type { Note, WindowState } from '../types'

const note = ref<Note | null>(null)
const mode = ref<'desktop' | 'top'>('desktop')
const pinned = ref(false)
const opacity = ref(1.0)
const showContextMenu = ref(false)
const contextMenuPos = ref({ x: 0, y: 0 })

const w = window as any
const noteId = w.__noteId as string
const noteType = w.__noteType as string

onMounted(async () => {
  note.value = await api.getNote(noteId)
  const ws = await api.getWindowState(noteId)
  if (ws) {
    mode.value = ws.mode
    pinned.value = ws.pinned
    opacity.value = ws.opacity
  }
  document.body.style.background = (note.value as any)?.bg_color || '#ffffff'
  document.body.style.opacity = String(opacity.value)
})

// 关闭 = 隐藏窗口
async function handleClose() {
  await api.saveWindowState({
    note_id: noteId, x: 0, y: 0, width: 320, height: 240,
    opacity: opacity.value, pinned: pinned.value, mode: mode.value, is_visible: false,
  })
  // 通过 Tauri window API 隐藏
  const { getCurrentWindow } = await import('@tauri-apps/api/window')
  await getCurrentWindow().hide()
}

async function handleToggleMode() {
  mode.value = mode.value === 'desktop' ? 'top' : 'desktop'
  const { getCurrentWindow } = await import('@tauri-apps/api/window')
  if (mode.value === 'top') {
    await getCurrentWindow().setAlwaysOnTop(true)
  } else {
    await getCurrentWindow().setAlwaysOnTop(false)
  }
}

async function handleTogglePinned() {
  pinned.value = !pinned.value
  const { getCurrentWindow } = await import('@tauri-apps/api/window')
  await getCurrentWindow().setResizable(!pinned.value)
}

async function handleUpdateOpacity(val: number) {
  opacity.value = val
  const { getCurrentWindow } = await import('@tauri-apps/api/window')
  await getCurrentWindow().setOpacity(val)
}

function handleContextMenu(event: MouseEvent) {
  event.preventDefault()
  contextMenuPos.value = { x: event.clientX, y: event.clientY }
  showContextMenu.value = true
}

// 自动保存 (debounced)
let saveTimer: ReturnType<typeof setTimeout> | null = null
function debouncedSave(data: Partial<Note>) {
  if (saveTimer) clearTimeout(saveTimer)
  saveTimer = setTimeout(() => {
    api.updateNote(noteId, data)
  }, 500)
}
</script>

<template>
  <div v-if="note" class="note-window" @contextmenu="handleContextMenu">
    <NoteChrome
      :title="note.title || ''" :mode="mode" :pinned="pinned" :opacity="opacity"
      @close="handleClose" @toggle-mode="handleToggleMode"
      @toggle-pinned="handleTogglePinned" @update-opacity="handleUpdateOpacity"
      @show-menu="handleContextMenu"
    />
    <div class="note-body">
      <TextNoteEditor v-if="note.type === 'text'" :note="note" @update="debouncedSave" />
      <TodoNoteEditor v-else-if="note.type === 'todo'" :note="note" @update="debouncedSave" />
      <TimerNoteView v-else-if="note.type === 'timer'" :note="note" />
    </div>

    <!-- 右键菜单 -->
    <div v-if="showContextMenu" class="context-menu" :style="{ left: contextMenuPos.x + 'px', top: contextMenuPos.y + 'px' }">
      <div class="ctx-item" @click="handleToggleMode">切换置顶/置底</div>
      <div class="ctx-item" @click="handleTogglePinned">锁定/解锁</div>
      <div class="ctx-sep"></div>
      <div class="ctx-item">设置提醒</div>
      <div class="ctx-item">复制便签</div>
      <div class="ctx-sep"></div>
      <div class="ctx-item ctx-danger" @click="handleClose">删除便签</div>
    </div>
    <div v-if="showContextMenu" class="context-overlay" @click="showContextMenu = false"></div>
  </div>
</template>

<style scoped>
.note-window { display: flex; flex-direction: column; height: 100vh; }
.note-body { flex: 1; overflow: auto; }
.context-menu {
  position: fixed; background: #fff; border: 1px solid var(--color-hairline);
  border-radius: var(--rounded-md); padding: 4px; z-index: 200;
  min-width: 140px; font-size: 12px; box-shadow: 0 2px 12px rgba(0,0,0,0.1);
}
.ctx-item { padding: 6px 12px; border-radius: 4px; cursor: pointer; }
.ctx-item:hover { background: var(--color-surface-soft); }
.ctx-danger { color: var(--color-accent-magenta); }
.ctx-sep { height: 1px; background: var(--color-hairline); margin: 4px 0; }
.context-overlay { position: fixed; inset: 0; z-index: 199; }
</style>
```

---

## Phase 7: 三类便签编辑器

### Task 14: TextNoteEditor.vue — TipTap 富文本编辑器

**Create:** `src/components/TextNoteEditor.vue`

- [ ] **Step 1: 完整 TipTap 编辑器组件**

```vue
<script setup lang="ts">
import { ref, watch, onBeforeUnmount } from 'vue'
import { useEditor, EditorContent } from '@tiptap/vue-3'
import StarterKit from '@tiptap/starter-kit'
import TextStyle from '@tiptap/extension-text-style'
import Color from '@tiptap/extension-color'
import FontSize from '@tiptap/extension-font-size'
import Placeholder from '@tiptap/extension-placeholder'
import type { Note } from '../types'
import { PASTEL_COLORS } from '../types'

const props = defineProps<{ note: Note }>()
const emit = defineEmits<{ update: [data: Partial<Note>] }>()

const currentBg = ref(props.note.bg_color || '#ffffff')
const currentTextColor = ref(props.note.default_text_color || '#000000')
const currentFontSize = ref(String(props.note.default_font_size || 16))
const showColorPicker = ref(false)
const showBgPicker = ref(false)

const editor = useEditor({
  content: props.note.content ? JSON.parse(props.note.content) : '',
  extensions: [
    StarterKit.configure({ heading: false, codeBlock: false, blockquote: false, horizontalRule: false, dropcursor: false, gapcursor: false }),
    TextStyle, Color, FontSize,
    Placeholder.configure({ placeholder: '输入内容...' }),
  ],
  onUpdate: ({ editor }) => {
    const json = JSON.stringify(editor.getJSON())
    emit('update', { content: json })
  },
  editorProps: {
    attributes: {
      style: `font-size: ${currentFontSize.value}px; color: ${currentTextColor.value}; text-shadow: ${getTextShadow(currentTextColor.value)};`,
    },
  },
})

// 文字描边
function getTextShadow(hex: string): string {
  const r = parseInt(hex.slice(1,3), 16)
  const g = parseInt(hex.slice(3,5), 16)
  const b = parseInt(hex.slice(5,7), 16)
  const luminance = (0.299 * r + 0.587 * g + 0.114 * b) / 255
  return luminance > 0.5
    ? '0 0 2px rgba(0,0,0,0.4)'
    : '0 0 2px rgba(255,255,255,0.5)'
}

function updateTextColor(hex: string) {
  currentTextColor.value = hex
  editor.value?.chain().setColor(hex).run()
  emit('update', { default_text_color: hex })
}

function updateBgColor(hex: string) {
  currentBg.value = hex
  document.body.style.background = hex
  emit('update', { bg_color: hex })
}

function updateFontSize(size: string) {
  currentFontSize.value = size
  editor.value?.chain().setFontSize(`${size}px`).run()
  emit('update', { default_font_size: Number(size) })
}

watch(() => props.note.bg_color, (val) => {
  if (val) { currentBg.value = val; document.body.style.background = val }
})

onBeforeUnmount(() => { editor.value?.destroy() })
</script>

<template>
  <div class="text-editor">
    <div class="toolbar">
      <button @click="editor?.chain().toggleBold().run()" :class="{ active: editor?.isActive('bold') }"><b>B</b></button>
      <button @click="editor?.chain().toggleItalic().run()" :class="{ active: editor?.isActive('italic') }"><i>I</i></button>
      <span class="sep">|</span>
      <div class="color-btn-wrap">
        <button @click="showColorPicker = !showColorPicker" title="文字颜色">🎨</button>
        <div v-if="showColorPicker" class="color-picker-dropdown">
          <div v-for="c in ['#000000','#ffffff','#ff3d8b','#1ea64a','#1f1d3d']" :key="c"
            class="color-swatch" :style="{ background: c }"
            @click="updateTextColor(c); showColorPicker = false">
          </div>
        </div>
      </div>
      <select :value="currentFontSize" @change="updateFontSize(($event.target as HTMLSelectElement).value)">
        <option v-for="s in ['12','14','16','18','20','24','28']" :key="s" :value="s">{{ s }}px</option>
      </select>
      <span class="sep">|</span>
      <button @click="editor?.chain().toggleBulletList().run()" :class="{ active: editor?.isActive('bulletList') }">•≡</button>
      <button @click="editor?.chain().toggleOrderedList().run()" :class="{ active: editor?.isActive('orderedList') }">1.≡</button>
      <span class="sep">|</span>
      <div class="color-btn-wrap">
        <button @click="showBgPicker = !showBgPicker" title="背景色">🎨 背景</button>
        <div v-if="showBgPicker" class="color-picker-dropdown">
          <div v-for="c in PASTEL_COLORS" :key="c.hex"
            class="color-swatch" :style="{ background: c.hex, border: currentBg === c.hex ? '2px solid #000' : '1px solid #ccc' }"
            :title="c.label" @click="updateBgColor(c.hex); showBgPicker = false">
          </div>
        </div>
      </div>
    </div>
    <EditorContent :editor="editor" class="editor-content" />
  </div>
</template>

<style scoped>
.text-editor { display: flex; flex-direction: column; height: 100%; }
.toolbar {
  display: flex; gap: 4px; padding: 4px 12px; align-items: center;
  background: rgba(255,255,255,0.4); border-bottom: 1px solid rgba(0,0,0,0.06);
  flex-wrap: wrap;
}
.toolbar button {
  background: none; border: none; cursor: pointer; padding: 2px 6px;
  border-radius: 4px; font-size: 13px; color: var(--color-ink);
}
.toolbar button:hover, .toolbar button.active { background: rgba(0,0,0,0.08); }
.toolbar select { background: none; border: 1px solid rgba(0,0,0,0.15); border-radius: 4px; font-size: 12px; padding: 1px 4px; }
.sep { color: #ccc; font-size: 14px; }
.editor-content {
  flex: 1; padding: 14px 16px; min-height: 100px;
}
.editor-content :deep(.ProseMirror) {
  outline: none; min-height: 100px; line-height: 1.6;
}
.editor-content :deep(.ProseMirror p.is-editor-empty:first-child::before) {
  content: attr(data-placeholder); color: #999; float: left; pointer-events: none; height: 0;
}
.color-btn-wrap { position: relative; }
.color-picker-dropdown {
  position: absolute; top: 100%; left: 0; display: flex; gap: 4px;
  background: #fff; border: 1px solid var(--color-hairline); border-radius: var(--rounded-md);
  padding: 6px 8px; z-index: 100; box-shadow: 0 2px 8px rgba(0,0,0,0.08); flex-wrap: wrap; width: 140px;
}
.color-swatch {
  width: 20px; height: 20px; border-radius: 50%; cursor: pointer; flex-shrink: 0;
}
</style>
```

---

### Task 15: TodoNoteEditor.vue — 待办便签编辑器

**Create:** `src/components/TodoNoteEditor.vue`

- [ ] **Step 1: 待办编辑器 + 进度条**

```vue
<script setup lang="ts">
import { computed } from 'vue'
import { useEditor, EditorContent } from '@tiptap/vue-3'
import StarterKit from '@tiptap/starter-kit'
import TaskList from '@tiptap/extension-task-list'
import TaskItem from '@tiptap/extension-task-item'
import TextStyle from '@tiptap/extension-text-style'
import Color from '@tiptap/extension-color'
import FontSize from '@tiptap/extension-font-size'
import Placeholder from '@tiptap/extension-placeholder'
import type { Note } from '../types'

const props = defineProps<{ note: Note }>()
const emit = defineEmits<{ update: [data: Partial<Note>] }>()

const editor = useEditor({
  content: props.note.content ? JSON.parse(props.note.content) : {
    type: 'doc', content: [{ type: 'taskList', content: [] }]
  },
  extensions: [
    StarterKit.configure({ heading: false, codeBlock: false, blockquote: false, horizontalRule: false, dropcursor: false, gapcursor: false, bulletList: false, orderedList: false }),
    TaskList, TaskItem.configure({ nested: false }),
    TextStyle, Color, FontSize,
    Placeholder.configure({ placeholder: '添加待办项...' }),
  ],
  onUpdate: ({ editor }) => {
    const json = JSON.stringify(editor.getJSON())
    const stats = getTaskStats()
    emit('update', { content: json })
  },
})

function getTaskStats() {
  if (!editor.value) return { done: 0, total: 0 }
  const items = editor.value.state.doc.descendants((node: any) => node.type.name === 'taskItem')
  let done = 0, total = 0
  for (const item of items) {
    total++
    if (item.attrs?.checked) done++
  }
  return { done, total }
}

const stats = computed(() => getTaskStats())
const progressPercent = computed(() => stats.value.total === 0 ? 0 : Math.round((stats.value.done / stats.value.total) * 100))

function addTaskItem() {
  editor.value?.chain().focus().insertContent({ type: 'taskItem', content: [{ type: 'paragraph' }] }).run()
}
</script>

<template>
  <div class="todo-editor">
    <div class="toolbar">
      <button @click="addTaskItem">+ 待办项</button>
    </div>
    <EditorContent :editor="editor" class="editor-content" />
    <div class="progress-bar" v-if="stats.total > 0">
      <span class="progress-text">{{ stats.done }}/{{ stats.total }} 已完成</span>
      <div class="progress-track">
        <div class="progress-fill" :style="{ width: progressPercent + '%' }"></div>
      </div>
      <span>{{ progressPercent }}%</span>
    </div>
  </div>
</template>

<style scoped>
.todo-editor { display: flex; flex-direction: column; height: 100%; }
.toolbar {
  padding: 4px 12px; background: rgba(255,255,255,0.3);
  border-bottom: 1px solid rgba(0,0,0,0.06);
}
.toolbar button {
  background: var(--color-primary); color: var(--color-on-primary);
  border: none; padding: 3px 10px; border-radius: var(--rounded-pill);
  font-size: 12px; cursor: pointer;
}
.editor-content {
  flex: 1; padding: 14px 16px; min-height: 100px;
}
.editor-content :deep(.ProseMirror) { outline: none; min-height: 80px; }
.editor-content :deep(.ProseMirror ul[data-type="taskList"]) { list-style: none; padding: 0; }
.editor-content :deep(.ProseMirror li[data-type="taskItem"]) { display: flex; align-items: flex-start; gap: 8px; margin-bottom: 4px; }
.editor-content :deep(.ProseMirror li[data-type="taskItem"] label) { margin-top: 2px; }
.editor-content :deep(.ProseMirror li[data-type="taskItem"] input[type="checkbox"]) {
  width: 18px; height: 18px; border: 2px solid var(--color-ink); border-radius: 4px;
  cursor: pointer; accent-color: var(--color-success);
}
.progress-bar {
  display: flex; align-items: center; gap: 10px;
  padding: 10px 16px; border-top: 1px solid rgba(0,0,0,0.08);
  font-size: 12px; color: var(--color-ink); font-weight: 600;
}
.progress-track {
  flex: 1; height: 6px; background: rgba(0,0,0,0.12); border-radius: 3px; overflow: hidden;
}
.progress-fill { height: 100%; background: var(--color-success); border-radius: 3px; transition: width 0.3s; }
</style>
```

---

### Task 16: TimerNoteView.vue — 计时便签视图

**Create:** `src/components/TimerNoteView.vue`, `src/components/TimerAdvancedSettings.vue`

- [ ] **Step 1: TimerNoteView.vue**

```vue
<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue'
import * as api from '../utils/tauri'
import TimerAdvancedSettings from './TimerAdvancedSettings.vue'
import type { Note, TimerState } from '../types'

const props = defineProps<{ note: Note }>()

const timerState = ref<TimerState>({
  note_id: props.note.id, daily_duration_minutes: 480, remaining_seconds: 28800,
  is_running: false, last_resume_at: null, tank_start_date: null,
  auto_start_time: null, warn_before_minutes: null,
})

const showAdvanced = ref(false)
let unlisteners: (() => void)[] = []

onMounted(async () => {
  timerState.value = await api.getTimerState(props.note.id)
  unlisteners.push(await api.onTimerTick((data) => {
    if (data.note_id === props.note.id) {
      timerState.value.remaining_seconds = data.remaining_seconds
      timerState.value.is_running = data.is_running
    }
  }) as any)
  unlisteners.push(await api.onTimerFinished((data) => {
    if (data.note_id === props.note.id) {
      timerState.value.is_running = false
      timerState.value.remaining_seconds = 0
    }
  }) as any)
  unlisteners.push(await api.onMidnightReset(() => {
    timerState.value.remaining_seconds = timerState.value.daily_duration_minutes * 60
    timerState.value.is_running = false
  }) as any)
})

onUnmounted(() => { unlisteners.forEach(f => f()) })

const displayTime = computed(() => {
  const s = timerState.value.remaining_seconds
  const h = Math.floor(s / 3600)
  const m = Math.floor((s % 3600) / 60)
  const sec = s % 60
  return `${String(h).padStart(2, '0')}:${String(m).padStart(2, '0')}:${String(sec).padStart(2, '0')}`
})

const tankDays = computed(() => {
  if (!timerState.value.tank_start_date) return null
  const start = new Date(timerState.value.tank_start_date)
  const today = new Date()
  return Math.floor((today.getTime() - start.getTime()) / 86400000)
})

async function toggleTimer() {
  if (timerState.value.is_running) {
    await api.pauseTimer(props.note.id)
    timerState.value.is_running = false
  } else {
    await api.startTimer(props.note.id)
    timerState.value.is_running = true
  }
}

async function handleReset() {
  await api.resetTimer(props.note.id)
  timerState.value.remaining_seconds = timerState.value.daily_duration_minutes * 60
  timerState.value.is_running = false
}

async function handleSettingsUpdate(settings: any) {
  await api.updateTimerSettings(props.note.id, settings)
  timerState.value = await api.getTimerState(props.note.id)
}
</script>

<template>
  <div class="timer-view">
    <div class="timer-display">{{ displayTime }}</div>
    <div class="timer-subtitle">剩余时间 / 今日总长 {{ Math.floor(timerState.daily_duration_minutes / 60) }}h</div>
    <div class="timer-controls">
      <button class="btn-primary" @click="toggleTimer">
        {{ timerState.is_running ? '⏸ 暂停' : '▶ 开始' }}
      </button>
      <button class="btn-secondary" @click="handleReset">↺ 重置</button>
    </div>
    <div v-if="tankDays !== null" class="tank-info">
      🌱 已开缸 <strong>{{ tankDays }}</strong> 天 · {{ timerState.tank_start_date }}
    </div>
    <button class="advanced-toggle" @click="showAdvanced = !showAdvanced">
      {{ showAdvanced ? '收起设置 ▲' : '高级设置 ▼' }}
    </button>
    <TimerAdvancedSettings v-if="showAdvanced" :timerState="timerState" @update="handleSettingsUpdate" />
  </div>
</template>

<style scoped>
.timer-view { display: flex; flex-direction: column; align-items: center; padding: 20px; color: #fff; height: 100%; }
.timer-display {
  font-family: var(--font-mono); font-size: 56px; font-weight: 340;
  letter-spacing: -1px; text-shadow: 0 1px 3px rgba(0,0,0,0.5);
}
.timer-subtitle { font-size: 14px; opacity: 0.5; margin-top: 4px; }
.timer-controls { display: flex; gap: 10px; margin-top: 16px; }
.btn-primary {
  background: #fff; color: #000; border: none;
  padding: 10px 32px; border-radius: var(--rounded-pill);
  font-size: 16px; font-weight: 500; cursor: pointer;
}
.btn-secondary {
  background: rgba(255,255,255,0.12); color: #fff;
  border: 1px solid rgba(255,255,255,0.2);
  padding: 10px 16px; border-radius: var(--rounded-pill); font-size: 14px; cursor: pointer;
}
.tank-info { margin-top: 14px; font-size: 13px; opacity: 0.45; }
.tank-info strong { color: #fff; font-weight: 600; }
.advanced-toggle {
  margin-top: 16px; background: none; border: none; color: rgba(255,255,255,0.5);
  font-size: 12px; cursor: pointer;
}
</style>
```

- [ ] **Step 2: TimerAdvancedSettings.vue**

```vue
<script setup lang="ts">
import { ref } from 'vue'
import type { TimerState } from '../types'

const props = defineProps<{ timerState: TimerState }>()
const emit = defineEmits<{ update: [settings: Record<string, any>] }>()

const dailyHours = ref(Math.floor(props.timerState.daily_duration_minutes / 60))
const dailyMinutes = ref(props.timerState.daily_duration_minutes % 60)
const tankDate = ref(props.timerState.tank_start_date || '')
const autoStart = ref(props.timerState.auto_start_time || '')
const warnBefore = ref(props.timerState.warn_before_minutes?.toString() || '')

function save() {
  emit('update', {
    daily_duration_minutes: dailyHours.value * 60 + dailyMinutes.value,
    tank_start_date: tankDate.value || null,
    auto_start_time: autoStart.value || null,
    warn_before_minutes: warnBefore.value ? Number(warnBefore.value) : null,
  })
}
</script>

<template>
  <div class="advanced-settings">
    <div class="setting-row">
      <label>每日总时长</label>
      <div class="time-inputs">
        <input type="number" v-model="dailyHours" min="0" max="24" @change="save" /> 小时
        <input type="number" v-model="dailyMinutes" min="0" max="59" @change="save" /> 分钟
      </div>
    </div>
    <div class="setting-row">
      <label>开缸日期</label>
      <input type="date" v-model="tankDate" @change="save" />
    </div>
    <div class="setting-row">
      <label>自动开始时间</label>
      <input type="time" v-model="autoStart" @change="save" />
    </div>
    <div class="setting-row">
      <label>提前提醒</label>
      <select v-model="warnBefore" @change="save">
        <option value="">不提醒</option>
        <option value="10">10 分钟前</option>
        <option value="30">30 分钟前</option>
      </select>
    </div>
  </div>
</template>

<style scoped>
.advanced-settings {
  margin-top: 12px; padding: 14px; background: rgba(255,255,255,0.08);
  border-radius: var(--rounded-md); font-size: 13px; color: rgba(255,255,255,0.8);
  width: 100%; max-width: 280px;
}
.setting-row {
  display: flex; justify-content: space-between; align-items: center; margin-bottom: 8px;
}
.setting-row input, .setting-row select {
  background: rgba(255,255,255,0.12); border: 1px solid rgba(255,255,255,0.2);
  color: #fff; padding: 4px 8px; border-radius: 4px; font-size: 12px;
}
.setting-row input[type="number"] { width: 50px; text-align: center; }
.time-inputs { display: flex; align-items: center; gap: 4px; }
</style>
```

---

## Phase 8: 管理面板

### Task 17: ManagementPanel.vue + GroupSidebar.vue + NoteList.vue

**Create:** `src/components/ManagementPanel.vue`, `src/components/GroupSidebar.vue`, `src/components/NoteList.vue`

- [ ] **Step 1: GroupSidebar.vue**

```vue
<script setup lang="ts">
import { ref, onMounted } from 'vue'
import * as api from '../utils/tauri'
import type { Group } from '../types'

const emit = defineEmits<{ select: [group: Group | null] }>()
const groups = ref<Group[]>([])
const selectedId = ref<string | null>(null)
const editingName = ref('')

onMounted(async () => { groups.value = await api.listGroups() })

function select(group: Group | null) {
  selectedId.value = group?.id ?? null
  emit('select', group)
}

async function createGroup() {
  const name = prompt('新建分组名称:')
  if (name) {
    const g = await api.createGroup(name)
    groups.value.push(g)
  }
}

async function renameGroup(g: Group) {
  const name = prompt('重命名:', g.name)
  if (name) {
    await api.renameGroup(g.id, name)
    g.name = name
  }
}

async function deleteGroup(g: Group) {
  if (confirm(`删除分组 "${g.name}"?\n该分组下的便签将移至未分组。`)) {
    await api.deleteGroup(g.id)
    groups.value = groups.value.filter(x => x.id !== g.id)
    if (selectedId.value === g.id) { selectedId.value = null; emit('select', null) }
  }
}
</script>

<template>
  <div class="sidebar">
    <div class="sidebar-label">分组</div>
    <div class="group-item" :class="{ active: selectedId === null }" @click="select(null)">全部</div>
    <div v-for="g in groups" :key="g.id" class="group-item" :class="{ active: selectedId === g.id }"
      @click="select(g)" @contextmenu.prevent="deleteGroup(g)">
      {{ g.name }}
    </div>
    <div class="sidebar-label" style="margin-top:8px;">🗑 回收站</div>
    <div class="add-group" @click="createGroup">+ 新建分组</div>
  </div>
</template>

<style scoped>
.sidebar { width: 160px; padding: 16px; border-right: 1px solid var(--color-hairline); font-size: 13px; }
.sidebar-label { font-weight: 600; margin-bottom: 10px; font-size: 11px; color: #888; }
.group-item {
  padding: 6px 10px; border-radius: var(--rounded-pill); margin-bottom: 4px; cursor: pointer;
}
.group-item:hover { background: var(--color-surface-soft); }
.group-item.active { background: var(--color-primary); color: var(--color-on-primary); }
.add-group { margin-top: 8px; font-size: 12px; color: #888; cursor: pointer; }
</style>
```

- [ ] **Step 2: NoteList.vue — 便签列表 + 类型筛选**

```vue
<script setup lang="ts">
import { ref, watch, onMounted } from 'vue'
import * as api from '../utils/tauri'
import type { Note, Group } from '../types'

const props = defineProps<{ group: Group | null }>()
const notes = ref<Note[]>([])
const typeFilter = ref<string>('')
const viewMode = ref<'list' | 'calendar'>('list')

onMounted(() => loadNotes())
watch(() => props.group, () => loadNotes())

async function loadNotes() {
  notes.value = await api.listNotes(props.group?.id, typeFilter.value || undefined)
}

watch(typeFilter, () => loadNotes())

function openNote(note: Note) {
  // 发送事件给 Rust 侧以显示便签窗口
  api.getWindowState(note.id) // 触发侧边栏显示
}

function formatTime(iso: string) {
  const d = new Date(iso)
  const now = new Date()
  const diff = now.getTime() - d.getTime()
  if (diff < 60000) return '刚刚'
  if (diff < 3600000) return `${Math.floor(diff / 60000)}分钟前`
  if (diff < 86400000) return `${Math.floor(diff / 3600000)}小时前`
  return d.toLocaleDateString()
}

function getNoteIcon(type: string) {
  return { text: '📝', todo: '✅', timer: '⏱' }[type] || '📄'
}
</script>

<template>
  <div class="note-list">
    <div class="type-filters">
      <span class="filter-pill" :class="{ active: typeFilter === '' }" @click="typeFilter = ''">全部类型</span>
      <span class="filter-pill" :class="{ active: typeFilter === 'text' }" @click="typeFilter = 'text'">📝 文本</span>
      <span class="filter-pill" :class="{ active: typeFilter === 'todo' }" @click="typeFilter = 'todo'">✅ 待办</span>
      <span class="filter-pill" :class="{ active: typeFilter === 'timer' }" @click="typeFilter = 'timer'">⏱ 计时</span>
    </div>
    <div v-for="note in notes" :key="note.id" class="note-item" @click="openNote(note)">
      <span>{{ getNoteIcon(note.type) }}</span>
      <span class="note-title">{{ note.title || '无标题' }}</span>
      <span class="note-meta">{{ note.type === 'todo' ? '待办' : note.type === 'timer' ? '计时' : '文本' }}</span>
      <span class="note-time">{{ formatTime(note.updated_at) }}</span>
    </div>
  </div>
</template>

<style scoped>
.note-list { flex: 1; padding: 12px 16px; }
.type-filters { display: flex; gap: 8px; margin-bottom: 12px; font-size: 12px; }
.filter-pill {
  padding: 3px 10px; border-radius: var(--rounded-pill); cursor: pointer;
  border: 1px solid var(--color-hairline);
}
.filter-pill.active { background: var(--color-primary); color: var(--color-on-primary); border-color: var(--color-primary); }
.note-item {
  display: flex; align-items: center; gap: 10px; padding: 10px 14px;
  border: 1px solid #f1f1f1; border-radius: var(--rounded-md); margin-bottom: 4px;
  cursor: pointer; border-left: 3px solid var(--color-block-cream);
}
.note-item:hover { background: var(--color-surface-soft); }
.note-title { font-weight: 500; flex: 1; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
.note-meta { font-size: 11px; color: #888; }
.note-time { font-size: 11px; color: #aaa; }
</style>
```

- [ ] **Step 3: ManagementPanel.vue — 组装管理面板**

```vue
<script setup lang="ts">
import { ref } from 'vue'
import GroupSidebar from './GroupSidebar.vue'
import NoteList from './NoteList.vue'
import CalendarView from './CalendarView.vue'
import * as api from '../utils/tauri'
import type { Group, Note } from '../types'

const selectedGroup = ref<Group | null>(null)
const viewMode = ref<'list' | 'calendar'>('list')
const searchQuery = ref('')
const searchResults = ref<Note[]>([])
const isSearching = ref(false)

async function onSearch() {
  if (!searchQuery.value.trim()) {
    isSearching.value = false
    return
  }
  isSearching.value = true
  searchResults.value = await api.searchNotes(searchQuery.value)
}

async function createNewNote() {
  const note = await api.createNote(selectedGroup.value?.id ?? null, 'text', null)
  // 触发窗口显示（通过 Rust 事件）
}
</script>

<template>
  <div class="management-panel">
    <div class="header">
      <span class="panel-title">📋 便签管理</span>
      <div class="view-toggle">
        <span class="view-pill" :class="{ active: viewMode === 'list' }" @click="viewMode = 'list'">列表</span>
        <span class="view-pill" :class="{ active: viewMode === 'calendar' }" @click="viewMode = 'calendar'">日历</span>
      </div>
      <div class="search-wrap">
        <input v-model="searchQuery" placeholder="🔍 搜索便签..." @input="onSearch" class="search-input" />
      </div>
      <button class="new-btn" @click="createNewNote">+ 新建</button>
    </div>
    <div class="body">
      <GroupSidebar @select="selectedGroup = $event" />
      <NoteList v-if="viewMode === 'list'" :group="selectedGroup" />
      <CalendarView v-else />
    </div>
  </div>
</template>

<style scoped>
.management-panel { display: flex; flex-direction: column; height: 100vh; background: var(--color-canvas); }
.header {
  display: flex; align-items: center; padding: 14px 20px;
  border-bottom: 1px solid var(--color-hairline); gap: 12px;
}
.panel-title { font-weight: 700; font-size: 17px; }
.view-toggle { display: flex; gap: 4px; background: var(--color-surface-soft); padding: 3px; border-radius: var(--rounded-pill); }
.view-pill { padding: 5px 14px; border-radius: var(--rounded-pill); font-size: 13px; cursor: pointer; }
.view-pill.active { background: var(--color-primary); color: var(--color-on-primary); }
.search-wrap { flex: 1; display: flex; justify-content: center; }
.search-input {
  border: 1px solid var(--color-hairline); border-radius: var(--rounded-md);
  padding: 8px 14px; font-size: 14px; width: 360px; background: var(--color-surface-soft);
  text-align: center; outline: none;
}
.new-btn {
  background: var(--color-primary); color: var(--color-on-primary);
  border: none; padding: 8px 18px; border-radius: var(--rounded-pill);
  font-size: 14px; font-weight: 500; cursor: pointer;
}
.body { display: flex; flex: 1; overflow: hidden; }
</style>
```

---

### Task 18: CalendarView.vue — 月历网格

**Create:** `src/components/CalendarView.vue`

- [ ] **Step 1: 日历视图组件**

```vue
<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import * as api from '../utils/tauri'
import type { Note } from '../types'

const currentDate = ref(new Date())
const notes = ref<Note[]>([])
const selectedDate = ref<string | null>(null)
const dateNotes = ref<Note[]>([])

const year = computed(() => currentDate.value.getFullYear())
const month = computed(() => currentDate.value.getMonth())

const today = computed(() => {
  const d = new Date()
  return `${d.getFullYear()}-${String(d.getMonth()+1).padStart(2,'0')}-${String(d.getDate()).padStart(2,'0')}`
})

function prevMonth() { currentDate.value = new Date(year.value, month.value - 1, 1) }
function nextMonth() { currentDate.value = new Date(year.value, month.value + 1, 1) }

const weeks = computed(() => {
  const firstDay = new Date(year.value, month.value, 1)
  const lastDay = new Date(year.value, month.value + 1, 0)
  const startOffset = (firstDay.getDay() + 6) % 7 // 周一开始
  const days: (number | null)[][] = []
  let week: (number | null)[] = Array(startOffset).fill(null)

  for (let d = 1; d <= lastDay.getDate(); d++) {
    week.push(d)
    if (week.length === 7) { days.push(week); week = [] }
  }
  if (week.length > 0) {
    while (week.length < 7) week.push(null)
    days.push(week)
  }
  return days
})

function dateStr(day: number) {
  return `${year.value}-${String(month.value+1).padStart(2,'0')}-${String(day).padStart(2,'0')}`
}

function notesForDay(day: number): Note[] {
  const ds = dateStr(day)
  return notes.value.filter(n => n.created_at.startsWith(ds) || n.updated_at.startsWith(ds))
}

const dotColor = (type: string) => ({ text: '#000', todo: '#1ea64a', timer: '#ff3d8b' }[type] || '#000')

function selectDate(day: number) {
  selectedDate.value = dateStr(day)
  dateNotes.value = notesForDay(day)
}

function openNote(note: Note) {
  // 通过事件通知 Rust 显示便签窗口
}

onMounted(async () => {
  notes.value = await api.listNotes(undefined, undefined)
})
</script>

<template>
  <div class="calendar-view">
    <div class="cal-header">
      <span class="cal-nav" @click="prevMonth">◂</span>
      <span class="cal-title">{{ year }}年{{ month + 1 }}月</span>
      <span class="cal-nav" @click="nextMonth">▸</span>
      <span class="cal-count">{{ notes.length }} 条便签</span>
      <span class="cal-legend">
        <span style="color:#000;">● 文本</span>
        <span style="color:var(--color-success);">● 待办</span>
        <span style="color:var(--color-accent-magenta);">● 计时</span>
      </span>
    </div>

    <div class="cal-grid">
      <div class="cal-day-header" v-for="d in ['一','二','三','四','五','六','日']" :key="d">{{ d }}</div>
      <template v-for="week in weeks">
        <div v-for="(day, i) in week" :key="`${week}-${i}`" class="cal-cell" :class="{
          'cal-today': day && dateStr(day) === today,
          'cal-other-month': day === null,
        }" @click="day && selectDate(day)">
          <div class="cal-day-num" v-if="day">{{ day }}</div>
          <div class="cal-day-notes" v-if="day">
            <div v-for="(n, ni) in notesForDay(day).slice(0, 3)" :key="n.id" class="cal-note-dot" :title="n.title || ''">
              <span class="dot" :style="{ background: dotColor(n.type) }"></span>
              <span class="dot-label">{{ n.title || '无标题' }}</span>
            </div>
            <div v-if="notesForDay(day).length > 3" class="cal-more">+{{ notesForDay(day).length - 3 }}</div>
          </div>
        </div>
      </template>
    </div>

    <div v-if="selectedDate" class="cal-detail">
      <div class="detail-title">📅 {{ selectedDate }} — {{ dateNotes.length }} 条便签</div>
      <div v-for="n in dateNotes" :key="n.id" class="detail-item" @click="openNote(n)">
        <span class="dot" :style="{ background: dotColor(n.type) }"></span>
        <span>{{ n.type === 'todo' ? '✅' : n.type === 'timer' ? '⏱' : '📝' }} {{ n.title || '无标题' }}</span>
        <span class="detail-meta">{{ n.type }}</span>
      </div>
    </div>
  </div>
</template>

<style scoped>
.calendar-view { flex: 1; padding: 16px 20px; overflow-y: auto; }
.cal-header { display: flex; align-items: center; gap: 8px; margin-bottom: 14px; font-size: 13px; }
.cal-title { font-size: 15px; font-weight: 600; }
.cal-nav { cursor: pointer; font-size: 14px; }
.cal-count { font-size: 12px; color: #888; }
.cal-legend { margin-left: auto; display: flex; gap: 10px; font-size: 11px; }
.cal-grid { display: grid; grid-template-columns: repeat(7, 1fr); border-top: 1px solid var(--color-hairline); border-left: 1px solid var(--color-hairline); }
.cal-day-header { padding: 6px 8px; text-align: center; font-weight: 600; color: #888; font-size: 12px; border-right: 1px solid var(--color-hairline); border-bottom: 1px solid var(--color-hairline); background: var(--color-surface-soft); }
.cal-cell { min-height: 72px; padding: 6px; border-right: 1px solid var(--color-hairline); border-bottom: 1px solid var(--color-hairline); cursor: pointer; }
.cal-cell:hover { background: #fafafa; }
.cal-other-month { background: #fafafa; }
.cal-today { background: #fffdf5; }
.cal-today .cal-day-num { background: var(--color-primary); color: var(--color-on-primary); border-radius: 50%; width: 20px; height: 20px; display: inline-flex; align-items: center; justify-content: center; font-size: 11px; }
.cal-day-num { font-weight: 500; margin-bottom: 3px; font-size: 12px; }
.cal-note-dot { display: flex; align-items: center; gap: 3px; font-size: 10px; margin-bottom: 2px; overflow: hidden; white-space: nowrap; text-overflow: ellipsis; }
.dot { width: 5px; height: 5px; border-radius: 50%; flex-shrink: 0; }
.dot-label { overflow: hidden; text-overflow: ellipsis; }
.cal-more { font-size: 10px; color: #888; }
.cal-detail { margin-top: 14px; padding: 12px 16px; background: var(--color-surface-soft); border-radius: var(--rounded-md); }
.detail-title { font-weight: 600; margin-bottom: 6px; font-size: 13px; }
.detail-item { display: flex; align-items: center; gap: 8px; padding: 6px 10px; background: #fff; border-radius: 6px; margin-bottom: 2px; cursor: pointer; font-size: 13px; }
.detail-item:hover { background: #f0f0f0; }
.detail-meta { margin-left: auto; font-size: 11px; color: #888; }
</style>
```

---

## Phase 9: 提醒 + 设置 + 最后集成

### Task 19: ReminderDialog.vue

**Create:** `src/components/ReminderDialog.vue`

- [ ] **Step 1: 提醒设置弹窗**

```vue
<script setup lang="ts">
import { ref } from 'vue'
import * as api from '../utils/tauri'

const props = defineProps<{ noteId: string }>()
const emit = defineEmits<{ close: [] }>()

const repeatType = ref<'once' | 'daily' | 'weekly'>('once')
const remindDate = ref('')
const remindTime = ref('09:00')
const selectedDays = ref<number[]>([1, 3, 5])

const weekLabels = ['一','二','三','四','五','六','日']

function toggleDay(n: number) {
  const idx = selectedDays.value.indexOf(n)
  if (idx >= 0) selectedDays.value.splice(idx, 1)
  else selectedDays.value.push(n)
}

async function confirm() {
  const remindAt = `${remindDate.value}T${remindTime.value}:00`
  const repeatDays = repeatType.value === 'weekly' ? selectedDays.value.join(',') : undefined
  await api.setReminder(props.noteId, remindAt, repeatType.value, repeatDays)
  emit('close')
}
</script>

<template>
  <div class="dialog-overlay" @click.self="emit('close')">
    <div class="dialog">
      <div class="dialog-title">🔔 设置提醒</div>
      <div class="repeat-pills">
        <span class="pill" :class="{ active: repeatType === 'once' }" @click="repeatType = 'once'">一次性</span>
        <span class="pill" :class="{ active: repeatType === 'daily' }" @click="repeatType = 'daily'">每日</span>
        <span class="pill" :class="{ active: repeatType === 'weekly' }" @click="repeatType = 'weekly'">每周</span>
      </div>
      <div class="field">
        <label>日期 & 时间</label>
        <input type="date" v-model="remindDate" />
        <input type="time" v-model="remindTime" />
      </div>
      <div v-if="repeatType === 'weekly'" class="field">
        <label>重复日期</label>
        <div class="weekday-picker">
          <span v-for="(l, i) in weekLabels" :key="i" class="day-pill"
            :class="{ active: selectedDays.includes(i+1) }"
            @click="toggleDay(i+1)">{{ l }}</span>
        </div>
      </div>
      <div class="dialog-actions">
        <button class="btn-cancel" @click="emit('close')">取消</button>
        <button class="btn-confirm" @click="confirm">确认</button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.dialog-overlay { position: fixed; inset: 0; background: rgba(0,0,0,0.6); display: flex; align-items: center; justify-content: center; z-index: 300; }
.dialog { background: #fff; border-radius: var(--rounded-lg); padding: 24px; min-width: 360px; font-size: 14px; }
.dialog-title { font-weight: 700; font-size: 16px; margin-bottom: 16px; }
.repeat-pills { display: flex; gap: 8px; margin-bottom: 14px; }
.pill { padding: 6px 16px; border-radius: var(--rounded-pill); cursor: pointer; border: 1px solid var(--color-hairline); font-size: 13px; }
.pill.active { background: var(--color-primary); color: var(--color-on-primary); border-color: var(--color-primary); }
.field { margin-bottom: 14px; }
.field label { display: block; font-weight: 600; margin-bottom: 4px; }
.field input { border: 1px solid var(--color-hairline); border-radius: var(--rounded-md); padding: 8px 12px; font-size: 14px; margin-right: 8px; }
.weekday-picker { display: flex; gap: 6px; }
.day-pill { padding: 6px 10px; border-radius: var(--rounded-pill); cursor: pointer; font-size: 12px; border: 1px solid var(--color-hairline); }
.day-pill.active { background: var(--color-primary); color: var(--color-on-primary); }
.dialog-actions { display: flex; gap: 8px; justify-content: flex-end; margin-top: 16px; }
.btn-cancel { border: 1px solid var(--color-hairline); background: #fff; padding: 8px 20px; border-radius: var(--rounded-pill); cursor: pointer; }
.btn-confirm { background: var(--color-primary); color: var(--color-on-primary); border: none; padding: 8px 20px; border-radius: var(--rounded-pill); cursor: pointer; }
</style>
```

---

### Task 20: SettingsPanel.vue + 首次启动检测

**Create:** `src/components/SettingsPanel.vue`

- [ ] **Step 1: 设置窗口**

```vue
<script setup lang="ts">
import { ref, onMounted } from 'vue'
import * as api from '../utils/tauri'

const dataPath = ref('')
const backupPath = ref('')
const backupKeepDays = ref('7')
const trashAutoCleanDays = ref('30')
const autostart = ref(true)
const exportPath = ref('')

onMounted(async () => {
  dataPath.value = (await api.getSetting('data_path')) || ''
  backupPath.value = (await api.getSetting('backup_path')) || ''
  backupKeepDays.value = (await api.getSetting('backup_keep_days')) || '7'
  trashAutoCleanDays.value = (await api.getSetting('trash_auto_clean_days')) || '30'
  autostart.value = (await api.getSetting('autostart')) !== 'false'
})

async function saveSetting(key: string, value: string) {
  await api.setSetting(key, value)
}

async function handleExport() {
  if (exportPath.value) {
    await api.exportAllData(exportPath.value)
    alert('导出成功！')
  }
}
</script>

<template>
  <div class="settings-panel">
    <h2>⚙️ 软件设置</h2>
    <div class="setting-item">
      <label>数据存储路径</label>
      <input v-model="dataPath" @change="saveSetting('data_path', dataPath)" placeholder="%APPDATA%/StickyNotes/" />
    </div>
    <div class="setting-item">
      <label>备份路径</label>
      <input v-model="backupPath" @change="saveSetting('backup_path', backupPath)" />
    </div>
    <div class="setting-item">
      <label>备份保留天数</label>
      <input type="number" v-model="backupKeepDays" @change="saveSetting('backup_keep_days', backupKeepDays)" />
    </div>
    <div class="setting-item">
      <label>回收站自动清理（天）</label>
      <input type="number" v-model="trashAutoCleanDays" @change="saveSetting('trash_auto_clean_days', trashAutoCleanDays)" placeholder="0=不自动清理" />
    </div>
    <div class="setting-item">
      <label>开机自启动</label>
      <input type="checkbox" :checked="autostart" @change="autostart = !autostart; saveSetting('autostart', String(autostart))" />
    </div>
    <div class="setting-item">
      <label>导出数据</label>
      <input v-model="exportPath" placeholder="选择导出路径..." />
      <button @click="handleExport">导出 JSON</button>
    </div>
  </div>
</template>

<style scoped>
.settings-panel { padding: 24px; font-size: 14px; }
h2 { font-size: 18px; margin-bottom: 20px; }
.setting-item { margin-bottom: 14px; display: flex; align-items: center; gap: 12px; }
.setting-item label { font-weight: 600; width: 160px; flex-shrink: 0; }
.setting-item input { border: 1px solid var(--color-hairline); border-radius: var(--rounded-md); padding: 6px 10px; flex: 1; }
.setting-item button {
  background: var(--color-primary); color: var(--color-on-primary);
  border: none; padding: 6px 14px; border-radius: var(--rounded-pill); cursor: pointer; font-size: 13px;
}
</style>
```

---

### Task 21: main.ts + Vite 配置 + 全局快捷键集成

**Modify:** `src/main.ts`, `vite.config.ts`, `src-tauri/tauri.conf.json`

- [ ] **Step 1: src/main.ts**

```typescript
import { createApp } from 'vue'
import { createPinia } from 'pinia'
import App from './App.vue'
import './styles/tokens.css'
import './styles/global.css'

const app = createApp(App)
app.use(createPinia())
app.mount('#app')
```

- [ ] **Step 2: vite.config.ts**

```typescript
import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'

export default defineConfig({
  plugins: [vue()],
  clearScreen: false,
  server: { port: 1420, strictPort: true },
  envPrefix: ['VITE_', 'TAURI_'],
})
```

- [ ] **Step 3: tauri.conf.json — 关键配置**

```json
{
  "$schema": "https://raw.githubusercontent.com/nickolay/tauri-plugin-autostart-ts/main/schema.json",
  "productName": "桌面便签",
  "version": "1.0.0",
  "identifier": "com.stickynotes.desktop",
  "build": {
    "frontendDist": "../dist",
    "devUrl": "http://localhost:1420",
    "beforeDevCommand": "npm run dev",
    "beforeBuildCommand": "npm run build"
  },
  "app": {
    "windows": [],
    "withGlobalTauri": true
  },
  "plugins": {
    "autostart": {},
    "global-shortcut": {
      "shortcuts": [
        { "modifiers": "Control+Alt", "key": "N", "handler": "new-text-note" }
      ]
    },
    "notification": {
      "all": true
    }
  }
}
```

---

### Task 22: 最终 main.rs — 启动时恢复窗口

**Modify:** `src-tauri/src/main.rs`

- [ ] **Step 1: 完整 main.rs — 启动流程**

```rust
mod db;
mod models;
mod commands;
mod timer_engine;
mod reminder_engine;
mod backup_engine;
mod window_manager;
mod tray;

use db::Database;
use std::path::PathBuf;
use std::sync::Arc;
use tauri::Manager;

fn get_data_dir(app: &tauri::AppHandle) -> PathBuf {
    let default = std::env::var("APPDATA")
        .unwrap_or_else(|_| ".".into());
    PathBuf::from(default).join("StickyNotes")
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let data_dir = get_data_dir(&app.handle());
            let db = Arc::new(Database::new(&data_dir).expect("DB init failed"));

            // 检查是否首次启动
            let is_first_launch = {
                let conn = db.conn.lock().unwrap();
                conn.query_row::<String, _, _>(
                    "SELECT value FROM settings WHERE key='first_launch'", [], |r| r.get(0)
                ).is_err()
            };

            if is_first_launch {
                let conn = db.conn.lock().unwrap();
                conn.execute("INSERT OR REPLACE INTO settings (key,value) VALUES ('first_launch','false')",
                    rusqlite::params![]).ok();
                // 首次启动：创建管理面板窗口
                let _ = window_manager::create_management_panel(&app.handle());
            } else {
                // 恢复所有可见窗口
                let visible_notes: Vec<String> = {
                    let conn = db.conn.lock().unwrap();
                    let mut stmt = conn.prepare(
                        "SELECT ws.note_id FROM window_state ws JOIN notes n ON ws.note_id = n.id WHERE ws.is_visible = 1 AND n.deleted_at IS NULL"
                    ).unwrap();
                    stmt.query_map([], |row| row.get::<_,String>(0))
                        .unwrap().filter_map(|r| r.ok()).collect()
                };
                for note_id in visible_notes {
                    let note = {
                        let conn = db.conn.lock().unwrap();
                        conn.query_row("SELECT id, group_id, type, title FROM notes WHERE id=?1",
                            rusqlite::params![note_id], |row| {
                                Ok(crate::models::Note {
                                    id: row.get(0)?, group_id: row.get(1)?, r#type: row.get(2)?,
                                    title: row.get(3)?, content: None, bg_color: None,
                                    default_text_color: None, default_font_size: None,
                                    created_at: String::new(), updated_at: String::new(), deleted_at: None,
                                })
                            }).ok()
                    };
                    if let Some(note) = note {
                        window_manager::create_note_window(&app.handle(), &note).ok();
                    }
                }
            }

            // 设置系统托盘
            tray::setup_tray(&app.handle()).ok();

            // 启动后台引擎
            let db_clone = db.clone();
            timer_engine::start_timer_engine(app.handle().clone(), db_clone);
            let db_clone2 = db.clone();
            reminder_engine::start_reminder_engine(app.handle().clone(), db_clone2);
            backup_engine::start_backup_engine(data_dir.clone());

            // 管理数据库和路径
            app.manage(db);
            app.manage(data_dir);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::groups::create_group, commands::groups::list_groups,
            commands::groups::rename_group, commands::groups::delete_group,
            commands::notes::create_note, commands::notes::update_note,
            commands::notes::get_note, commands::notes::list_notes,
            commands::notes::delete_note, commands::notes::restore_note,
            commands::notes::permanently_delete_note, commands::notes::search_notes,
            commands::timers::get_timer_state, commands::timers::start_timer,
            commands::timers::pause_timer, commands::timers::reset_timer,
            commands::timers::update_timer_settings,
            commands::reminders::set_reminder, commands::reminders::get_reminders,
            commands::reminders::delete_reminder,
            commands::windows::save_window_state, commands::windows::get_window_state,
            commands::windows::get_all_window_states,
            commands::settings::set_setting, commands::settings::get_setting,
            commands::settings::export_all_data,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

---

## Phase 10: 验证与收尾

### Task 23: 编译验证 + 安装字体 + 图标

- [ ] **Step 1: 编译 Rust 后端**

```bash
cd src-tauri && cargo check
# 预期: 0 errors
```

- [ ] **Step 2: 安装字体**

```bash
# 下载 Inter 和 JetBrains Mono，放入 src/assets/fonts/
# 在 index.html 中通过 @font-face 或 CDN 引入
```

在 `index.html` 中添加：

```html
<link rel="preconnect" href="https://fonts.googleapis.com">
<link href="https://fonts.googleapis.com/css2?family=Inter:wght@320;340;400;500;600;700&family=JetBrains+Mono:wght@400;500&display=swap" rel="stylesheet">
```

- [ ] **Step 3: 构建验证**

```bash
npm run tauri build -- --debug
# 预期: 生成 Windows .msi/.exe, 大小 < 10MB (debug)
```

- [ ] **Step 4: 验证关键功能**

```bash
# 启动应用
# 验证: 托盘图标出现 → 新建便签 → 编辑 → 关闭隐藏 → 新建计时 → 开始暂停 → 管理面板 → 日历视图
```

---

## 附录: 实施顺序依赖图

```
Phase 1 (脚手架)
  └─> Phase 2 (数据模型)
        └─> Phase 3 (CRUD命令)
              ├─> Phase 4 (计时引擎) ─┐
              ├─> Phase 4 (提醒引擎) ─┤
              └─> Phase 4 (备份引擎) ─┤
                                      ├─> Phase 5 (Vue基础设施)
                                      │     └─> Phase 6 (NoteWindow + Chrome)
                                      │           ├─> Phase 7 (三类编辑器)
                                      │           ├─> Phase 8 (管理面板)
                                      │           └─> Phase 9 (提醒 + 设置)
                                      └─> Phase 10 (最终集成 + 验证)
```

---

> 计划完成。请审阅后选择执行方式。
