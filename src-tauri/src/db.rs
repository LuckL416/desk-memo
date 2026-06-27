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
