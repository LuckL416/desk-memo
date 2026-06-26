# 桌面便签工具 — 设计方案规格书

> 版本: v1.0 | 日期: 2026-06-26 | 状态: 已确认

---

## 一、产品定位与边界

### 核心定位
纯本地轻量跨平台桌面便签工具，基于 Tauri 2.x + Vue 3 技术栈，极简无负担。无云同步、无账号体系、无冗余功能。

### 量化目标
| 指标 | 目标值 |
|------|--------|
| 安装包体积 | < 5MB |
| 后台内存占用 | < 100MB |
| 平台优先 | Windows → macOS → Linux |

### 功能边界
- **做**：文本便签、待办清单、计时便签（草缸专用）、窗口管理、提醒、分组、日历视图、回收站、自动备份
- **不做**：云同步、账号体系、图片/视频/表格/代码块、复杂皮肤、专业水族管理

---

## 二、技术架构

### 方案：数据库即真相源 (Database as Single Source of Truth)

```
┌─────────────────────────────────────────────────┐
│  窗口层 (Vue 3 WebView × N)                       │
│  每个便签一个独立 WebViewWindow，无任务栏条目       │
└──────────┬──────────────────────────────────────┘
           │ invoke (进程内 IPC, <1ms)
           ▼
┌─────────────────────────────────────────────────┐
│  Rust 后端 (单一进程)                              │
│  ├─ 窗口管理器 (创建/销毁/恢复/置顶/置底)            │
│  ├─ 计时引擎 (独立线程, 1秒精度)                    │
│  ├─ 提醒引擎 (独立线程, 1分钟精度)                   │
│  ├─ 备份引擎 (独立线程, 每日3:00)                   │
│  ├─ 自动保存 (debounced invoke 写库)               │
│  ├─ 系统托盘 + 全局快捷键                          │
│  └─ 开机自启动                                    │
└──────────┬──────────────────────────────────────┘
           │ SQL
           ▼
┌─────────────────────────────────────────────────┐
│  SQLite (WAL 模式, 单文件)                         │
│  默认路径: %APPDATA%/StickyNotes/data.db          │
│  备份路径: {路径}/backups/ (可独立自定义)            │
└─────────────────────────────────────────────────┘
```

### 数据流
- **Vue → Rust**: Tauri `invoke` 命令（新建、编辑、删除、设置）
- **Rust → Vue**: Tauri `event` 推送（计时tick、提醒触发、便签变更通知）
- **Vue 窗口间**: 不直接通信，通过 Rust 事件广播中转

### 关键设计决策
1. **计时在 Rust 侧**：计时便签倒计时在 Rust 独立线程运行，窗口关闭不影响计时
2. **DB 即真相源**：无 Pinia 跨窗口缓存，每次 invoke 直接读写 SQLite
3. **软删除**：删除便签仅设置 deleted_at，7份自动备份兜底
4. **窗口状态持久化**：X/Y/宽/高/透明度/模式/锁定/可见性存 window_state 表

---

## 三、数据模型

### 表结构

#### groups — 分组表
| 字段 | 类型 | 说明 |
|------|------|------|
| id | TEXT PK | UUID |
| name | TEXT NOT NULL | 分组名称 |
| sort_order | INTEGER | 排序权重 |
| created_at | TEXT | ISO 8601 |
| updated_at | TEXT | ISO 8601 |

#### notes — 便签主表
| 字段 | 类型 | 说明 |
|------|------|------|
| id | TEXT PK | UUID |
| group_id | TEXT FK | → groups.id |
| type | TEXT NOT NULL | 'text' / 'todo' / 'timer' |
| title | TEXT | 便签标题 |
| content | TEXT | TipTap JSON (text/todo) 或 null (timer) |
| bg_color | TEXT | 背景色 hex, null=white |
| default_text_color | TEXT | 默认文字色 hex, null=black |
| default_font_size | INTEGER | 默认字号 px |
| created_at | TEXT | ISO 8601 |
| updated_at | TEXT | ISO 8601 |
| deleted_at | TEXT | 软删除标记，非空=在回收站 |

#### timer_state — 计时便签状态
| 字段 | 类型 | 说明 |
|------|------|------|
| note_id | TEXT PK FK | → notes.id |
| daily_duration_minutes | INTEGER | 每日总时长，默认480(8h) |
| remaining_seconds | INTEGER | 当前剩余秒数 |
| is_running | INTEGER | 是否正在计时 |
| last_resume_at | TEXT | 上次开始/继续时间 |
| tank_start_date | TEXT | 开缸日期 |
| auto_start_time | TEXT | 自动开始时间 'HH:MM' |
| warn_before_minutes | INTEGER | 提前提醒 10/30/null |

#### reminders — 提醒表
| 字段 | 类型 | 说明 |
|------|------|------|
| id | TEXT PK | UUID |
| note_id | TEXT FK | → notes.id |
| remind_at | TEXT | 提醒时间 ISO 8601 |
| repeat_type | TEXT | 'once' / 'daily' / 'weekly' |
| repeat_days | TEXT | weekly 时 '1,3,5' |
| is_active | INTEGER | 默认 1 |

#### window_state — 窗口状态
| 字段 | 类型 | 说明 |
|------|------|------|
| note_id | TEXT PK | → notes.id 或 'management-panel' |
| x, y, width, height | INTEGER | 窗口几何 |
| opacity | REAL | 0.0-1.0 |
| pinned | INTEGER | 锁定位置 |
| mode | TEXT | 'desktop'/'top' |
| is_visible | INTEGER | 窗口是否可见 |

#### settings — 全局设置 (K/V)
| 字段 | 类型 | 说明 |
|------|------|------|
| key | TEXT PK | 设置键 |
| value | TEXT | 设置值 |

典型 key: `data_path`, `backup_path`, `backup_keep_days`, `trash_auto_clean_days`, `autostart`, `language`

### 存储路径
- **默认路径**: `%APPDATA%/StickyNotes/`
- **数据库**: `{路径}/data.db` (WAL 模式)
- **备份**: `{路径}/backups/`
- **路径更改**: 停止引擎 → 复制 data.db → 更新设置 → 重启引擎

---

## 四、设计系统 (Figma Design System 适配)

### 色彩
| Token | Hex | 用途 |
|-------|-----|------|
| primary / ink | #000000 | 文字、主按钮 |
| canvas | #ffffff | 管理面板背景 |
| on-primary | #ffffff | 深色底文字 |
| surface-soft | #f7f7f5 | 卡片/磁贴背景 |
| hairline | #e6e6e6 | 1px 分割线 |
| block-lime | #dceeb1 | 便签背景色 |
| block-lilac | #c5b0f4 | 便签背景色 |
| block-cream | #f4ecd6 | 便签背景色（默认） |
| block-mint | #c8e6cd | 便签背景色 |
| block-pink | #efd4d4 | 便签背景色 |
| block-coral | #f3c9b6 | 便签背景色 |
| block-navy | #1f1d3d | 计时便签背景色 |
| accent-magenta | #ff3d8b | 提醒警示色 |
| semantic-success | #1ea64a | 待办完成/勾选 |

### 文字描边方案
- 暗色文字 → `text-shadow: 0 0 2px rgba(255,255,255,0.5)`
- 亮色文字 → `text-shadow: 0 0 2px rgba(0,0,0,0.4)`
- 解决 pastel 背景与文字对比度不足问题

### 字体
- **Inter** (替代 figmaSans) — 便签正文、管理面板
- **JetBrains Mono** (替代 figmaMono) — 计时数字、代码标签

### 形状
- 按钮: `rounded: pill` (50px)
- 图标按钮: `rounded: full` (9999px)
- 输入框: `rounded: md` (8px)
- 面板: `rounded: lg` (24px)

### 设计原则
- 无阴影设计，用颜色做层次区分
- pill 按钮为唯一 CTA 形状
- 纯黑文字 (#000)，不引入灰色层级（靠字重区分）
- 管理面板黑白框架，便签窗口 pastel 色块

---

## 五、窗口交互体系

### 自定义 Chrome 栏
- 高度 36px，替代 Windows 原生标题栏
- 左侧：便签标题（可编辑）
- 右侧：透明度 ◐ / 锁定 🔒 / 模式切换 📌 / 更多 ⋯ / 关闭 ✕
- 拖动区域：Chrome 栏整体可拖动

### 双模式
- **桌面置底**：窗口在图标层之上、壁纸层之上，不被遮挡，点击"显示桌面"不消失
- **悬浮置顶**：悬浮于所有应用之上，可自由拖动
- 一键切换：Chrome 栏按钮或右键菜单

### 窗口操控
- 四角自由缩放，最小尺寸 200×120px
- 锁定位置：禁用拖动 + 缩放
- 无级透明度：0%-100%，每条便签独立，最低 5% 可见度边框
- 状态记忆：debounced 写入 window_state 表

### 右键菜单
**便签窗口右键**：切换模式 / 锁定 / 透明度 / 背景色 / 设置提醒 / 移动到分组 / 复制便签 / 删除便签

**系统托盘右键**：新建便签(Ctrl+Alt+N) / 新建待办 / 新建计时 / 显示-隐藏全部 / 管理面板 / 设置 / 退出

### 关闭行为
- 关闭 = 隐藏窗口（数据保留），仅管理面板"删除"才入回收站

---

## 六、三类便签设计

### 1. 普通文本便签
- TipTap 富文本编辑器
- 启用扩展：Bold, Italic, TextStyle, FontSize, OrderedList, BulletList, History
- 工具栏：B / I / 文字颜色 🎨 / 字号下拉 / 无序列表 / 有序列表 / 背景色
- 独立设置背景色、默认文字颜色、默认字号
- 实时自动保存（debounced invoke）

### 2. 待办清单便签
- 复用文本便签格式能力 + TaskList 扩展
- 专属"+ 待办项"按钮
- 底部自动统计：已完成 X/Y · 进度条 · 百分比
- 完成项：文字删除线 + 绿色勾选框

### 3. 计时便签（草缸专用）
- 独立窗口，navy 深色背景
- 主体：JetBrains Mono 大字号倒计时 HH:MM:SS
- 按钮：开始/暂停 + 重置
- 小字：已开缸 XX 天 · YYYY-MM-DD
- 高级设置（可折叠）：每日总时长、开缸日期、自动开始时间、提前提醒
- 倒计时结束 → 系统通知 + 提示音
- 跨日 00:00 → Rust 线程自动重置剩余时长
- 里程碑：30/100/365 天通知

---

## 七、提醒系统

### 通用提醒
- 类型：一次性 / 每日重复 / 每周重复（可选周几）
- 触发：Rust 提醒线程每分钟检查 → 系统通知 + 可选提示音
- 交互：点击通知 → 打开对应便签窗口
- 重复型：触发后自动计算下次时间

### 计时便签专属提醒
- 倒计时结束提醒
- 提前预告：10/30 分钟可选
- 开缸里程碑：30/100/365 天

### 提醒设置 UI
- 便签窗口内弹窗
- Pill 切换：一次性 / 每日 / 每周
- 日期选择 + 时间选择
- 周几选择器（仅 weekly 模式显示）

---

## 八、管理面板

### 双视图
- **列表视图**（默认）：分组侧栏 + 便签列表 + 类型筛选
- **日历视图**：月历网格，借鉴 Memos 设计

### 日历视图
- 7 列标准月历，今天黑色圆圈高亮
- 每格显示便签标题（圆点颜色区分类型：黑=文本 / 绿=待办 / 品红=计时）
- 每格最多 3 条，超出显示"+N"
- 单击日期 → 底部展开当日便签列表
- 双击日期 → 打开对应便签窗口
- 数据来源：created_at / updated_at / remind_at

### 分组管理
- 新建 / 重命名 / 删除分组
- 右击分组 → 一键显示/隐藏该组全部便签

### 全局搜索
- 匹配标题 + 正文纯文本
- 实时过滤结果列表
- 点击结果 → 定位并高亮便签窗口边框
- **搜索框位置**：顶部居中，360px 宽

### 回收站
- 软删除便签 → 可恢复或永久删除
- 可选自动清理周期（如 30 天）

---

## 九、数据安全

### 自动本地备份
- 频率：每日凌晨 3:00
- 保留：最近 7 份
- 格式：SQLite 完整副本 `backup-YYYY-MM-DD.db`
- 路径：可自定义

### 手动导出/导入
- 导出：全量 JSON 文件（所有表）
- 导入：JSON → 验证结构 → 合并/覆盖模式 → 写入 SQLite

### 首次启动
- 首次安装 → 弹出管理面板
- 后续启动 → 静默后台，仅托盘图标

### 隐私
- 纯本地运行，零网络请求
- 无遥测、无分析、无统计上报

---

## 十、技术栈

| 类别 | 技术 | 说明 |
|------|------|------|
| 运行时 | Tauri 2.x (Rust) | 跨平台桌面框架 |
| 前端 | Vue 3 + Vite + TypeScript | SPA 开发 |
| 状态管理 | Pinia | 窗口内本地状态 |
| 富文本 | TipTap 核心版 | 8 个启用扩展 |
| 数据库 | SQLite (rusqlite) | WAL 模式 |
| 字体 | Inter + JetBrains Mono | 开源替代 |
| 自启动 | tauri-plugin-autostart | 开机自启 |
| 快捷键 | tauri-plugin-global-shortcut | Ctrl+Alt+N |
| 通知 | tauri-plugin-notification | 系统原生通知 |
| 窗口状态 | tauri-plugin-window-state | 辅助插件 |

### TipTap 扩展清单
**启用**: starter-kit (Bold, Italic, History, OrderedList, BulletList, Document, Paragraph, Text) + TaskList + TaskItem + FontSize + TextStyle + Color

**禁做**: Image, Table, CodeBlock, Link (仅自动识别不渲染超链接), Blockquote, HorizontalRule, File/Attachment

### 项目目录结构
```
桌面便签/
├── src-tauri/              # Rust 后端
│   ├── src/
│   │   ├── main.rs         # 入口
│   │   ├── db.rs           # SQLite
│   │   ├── commands/       # Tauri 命令
│   │   ├── timer.rs        # 计时引擎
│   │   ├── reminder.rs     # 提醒引擎
│   │   ├── backup.rs       # 备份引擎
│   │   ├── window_manager.rs
│   │   └── tray.rs
│   ├── Cargo.toml
│   └── tauri.conf.json
├── src/                    # Vue 3 前端
│   ├── main.ts
│   ├── components/
│   │   ├── NoteWindow.vue
│   │   ├── TextNoteEditor.vue
│   │   ├── TodoNoteEditor.vue
│   │   ├── TimerNoteView.vue
│   │   ├── ManagementPanel.vue
│   │   ├── CalendarView.vue
│   │   ├── SettingsPanel.vue
│   │   └── ReminderDialog.vue
│   ├── stores/note.ts
│   ├── styles/tokens.css
│   └── utils/tauri.ts
├── package.json
├── vite.config.ts
└── tsconfig.json
```

---

## 十一、设计决策记录

| # | 决策 | 选项 | 结论 |
|----|------|------|------|
| 1 | 平台优先 | A:Windows/B:同步/C:Win+Mac | **A: Windows 优先** |
| 2 | 交付节奏 | A:全量/B:MVP/C:分批 | **A: 全量交付** |
| 3 | 窗口架构 | A:任务栏/B:无任务栏/C:混合 | **B: 无任务栏多窗口** |
| 4 | 关闭行为 | A:隐藏/B:最小化/C:无按钮 | **A: 关闭=隐藏** |
| 5 | 首次启动 | A:空便签/B:管理面板/C:静默 | **B: 弹出管理面板** |
| 6 | 架构方案 | A:DB真相源/B:总管+缓存/C:极简壳 | **A: DB即真相源** |
| 7 | 视觉主题 | Figma Design System | **Inter + pastel + pill + 无阴影** |
| 8 | 文字对比度 | 描边 vs 校验 vs 智能默认 | **文字描边方案** |
| 9 | 搜索框位置 | 右 vs 中 | **居中，360px** |
| 10 | 日期展示 | 日历表格（借鉴 Memos） | **月历网格 + 日期标记** |

---

## 十二、附录：设计系统 Token 映射

详见 `DESIGN-figma.md`，关键映射：

- `{colors.primary}` → `#000000`
- `{colors.block-lime}` → `#dceeb1`
- `{colors.block-lilac}` → `#c5b0f4`
- `{colors.block-cream}` → `#f4ecd6`
- `{colors.block-mint}` → `#c8e6cd`
- `{colors.block-pink}` → `#efd4d4`
- `{colors.block-coral}` → `#f3c9b6`
- `{colors.block-navy}` → `#1f1d3d`
- `{colors.accent-magenta}` → `#ff3d8b`
- `{colors.semantic-success}` → `#1ea64a`
- Font: Inter (figmaSans substitute) + JetBrains Mono (figmaMono substitute)
- Pill buttons, no shadows, flat design
