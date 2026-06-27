# DeskMemos

> Lightweight desktop sticky notes app. Pure local, no cloud, no account.

A desktop memo tool built with **Tauri 2.x + Vue 3**, designed to stay on your desktop silently and capture ideas instantly. Supports rich text notes, to-do checklists, and a dedicated countdown timer for aquarium lighting.

## Features

- **Rich Text Notes** — Bold, italic, colors, font sizes, bullet/ordered lists. Auto-save. Independent background color per note.
- **To-Do Checklists** — Checkable task items with auto-calculated progress bar (done/total + percentage).
- **Timer Notes** — Countdown timer with start/pause, daily auto-reset, tank-age tracking. Designed for aquarium lighting schedules.
- **Desktop Calendar Widget** — Monthly calendar grid pinned to desktop, showing which notes belong to which dates.
- **Window Controls** — Drag, resize, lock position, toggle always-on-top, per-window opacity (0–100%).
- **System Tray** — Minimizes to tray. Right-click to create notes, toggle all, open management panel.
- **Management Panel** — List/calendar views, groups, full-text search, recycle bin with restore/permanent delete.
- **Reminders** — One-time, daily, or weekly reminders with system notifications.
- **Figma Design System** — Clean black & white editorial frame with pastel color blocks. Inter + JetBrains Mono fonts.
- **Fully Offline** — SQLite local storage. Zero network requests. No telemetry.

## Tech Stack

| Layer | Technology |
|-------|-----------|
| Runtime | Tauri 2.x (Rust) |
| Frontend | Vue 3 + Vite + TypeScript |
| Editor | TipTap (rich text) |
| Database | SQLite (WAL mode, rusqlite) |
| Fonts | Inter + JetBrains Mono |
| Notifications | tauri-plugin-notification |
| Autostart | tauri-plugin-autostart |
| Global Hotkey | tauri-plugin-global-shortcut (Ctrl+Alt+N) |

## Installation

### Pre-built

Download the latest installer from [Releases](https://github.com/LuckL416/desk-memo/releases).

### Build from source

```bash
# Prerequisites: Node.js 18+, Rust 1.70+, VS Build Tools (Windows)

git clone https://github.com/LuckL416/desk-memo.git
cd desk-memo
npm install
npm run tauri build -- --bundles nsis
```

Output: `src-tauri/target/release/bundle/nsis/desk-memo_*_x64-setup.exe` (~3.5 MB)

## Architecture

```
Windows (WebView × N)          # One window per sticky note
    ↓ invoke
Rust Backend                    # All business logic
├── Window Manager              # Create/destroy/restore windows
├── Timer Engine                # Countdown timer (independent thread)
├── Reminder Engine             # Scheduled reminders (independent thread)
├── Backup Engine               # Daily auto-backup (independent thread)
├── System Tray                 # Tray icon + menu
└── Database (SQLite)           # Single-file, WAL mode
```

Each note is an independent Tauri WebViewWindow (no taskbar entries). The Rust backend handles all data persistence and background tasks. Vue components render the UI and communicate via Tauri IPC.

## Data

- All data stored locally at `%APPDATA%/StickyNotes/data.db`
- Daily auto-backups (keeps last 7 copies)
- Manual JSON export/import for migration

## License

MIT
