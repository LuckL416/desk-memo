# 主题系统设计方案

> 版本: v1.0 | 日期: 2026-06-27 | 状态: 已确认

---

## 一、目标

在管理面板中实现可切换的主题系统。初始提供两个主题：「马卡龙纸胶带」和「极客黑客」。

- 零 Rust 改动
- 纯 CSS + Canvas 实现
- 主题偏好持久化到 SQLite settings 表
- 所有窗口（便签、管理面板、设置、日历组件）统一响应主题

---

## 二、主题定义

### 主题 1：马卡龙纸胶带（默认）

当前样式，保持不变。

| 属性 | 值 |
|------|-----|
| 背景 | 5 色 pastel（粉/紫/绿/蓝/黄） |
| 底色 | `#ffffff` |
| 文字色 | 各配色对应暗色 |
| 装饰 | `::before` 胶带 + SVG 噪点纹理 |
| 侧栏 | 毛玻璃 `rgba(255,255,255,0.7)` + `blur(16px)` |
| 字体 | Inter（英文）+ 仿宋（中文） |

### 主题 2：极客黑客

| 属性 | 值 |
|------|-----|
| 底色 | `#0D1117`（深黑） |
| 卡片色 | `#161B22` |
| 强调色 | `#00FF41`（荧光绿） |
| 文字色 | `#E6EDF3`（浅灰白） |
| 次要文字 | `#8B949E` |
| 边框 | `1px solid rgba(0,255,65,0.2)` |
| 按钮 | 暗底 + 1px 绿边 |
| 输入框 focus | `box-shadow: 0 0 0 2px rgba(0,255,65,0.2)` + 绿色边框 |
| 便签窗口 | 暗色底 + 1px 绿色荧光边框 + 顶部渐变绿线 |
| 侧栏 | 暗色纯底 `#0D1117` + 绿色分隔线 |
| 字体 | JetBrains Mono（英文）+ 仿宋（中文） |
| 背景特效 | Canvas 数字雨 + CSS CRT 扫描线 |

---

## 三、数字雨 + CRT 扫描线背景

### 数字雨（Canvas）
- 字符集：`ｦｧｨｩｪｫｬｭｮｯｱｲｳｴｵｶｷｸｹｺｻｼｽｾｿﾀﾁﾂﾃ`（片假名 + 数字）
- 字号：12px，`JetBrains Mono`
- 颜色：`rgba(0, 255, 65, 0.15)` 低饱和暗绿
- 速度：1-2 像素/帧，缓慢下落
- 密度：约 2-3 列/100px
- Canvas 覆盖整个窗口，`position: fixed; z-index: 0`
- 只在主题为 hacker 时渲染

### CRT 扫描线（CSS）
```css
background: repeating-linear-gradient(
  0deg,
  transparent,
  transparent 2px,
  rgba(0, 0, 0, 0.03) 2px,
  rgba(0, 0, 0, 0.03) 4px
);
```
- 覆盖在数字雨之上，`z-index: 1`
- 只在主题为 hacker 时生效

### 层级结构
```
z-index 2  → 便签内容、按钮、文字
z-index 1  → CRT 扫描线 (::after)
z-index 0  → Canvas 数字雨
```

---

## 四、CSS 变量体系

### 马卡龙主题（默认 `:root`）
```css
--color-bg: #ffffff
--color-surface: #f7f7f5
--color-primary: #000000
--color-ink: #000000
--color-border: #e6e6e6
--color-accent: #ff3d8b
--color-hover: rgba(0,0,0,0.08)
--font-ui: 'Inter', 'FangSong', '仿宋', sans-serif
```

### 极客主题（`html.theme-hacker`）
```css
--color-bg: #0D1117
--color-surface: #161B22
--color-primary: #00FF41
--color-ink: #E6EDF3
--color-border: rgba(0,255,65,0.15)
--color-accent: #00FF41
--color-hover: rgba(0,255,65,0.08)
--font-ui: 'JetBrains Mono', 'FangSong', '仿宋', monospace
```

---

## 五、实现组件

### MatrixRain.vue（新增）
```vue
<canvas ref="canvas" class="matrix-rain" />
```
- `onMounted` 启动动画循环
- `requestAnimationFrame` 逐帧绘制
- `watch` 主题切换时启动/停止
- `onUnmounted` 清理动画

### ThemeSwitcher（修改 ManagementPanel.vue）
在管理面板头部添加主题切换 pill：
```
[马卡龙] [极客黑客]
```

### 主题初始化（修改 main.ts）
```typescript
// 启动时读取 theme 设置，设置 html class
const theme = await getSetting('theme') || 'macaron'
document.documentElement.classList.add(`theme-${theme}`)
```

---

## 六、文件变更

| 操作 | 文件 | 说明 |
|------|------|------|
| 新增 | `src/components/MatrixRain.vue` | Canvas 数字雨组件 |
| 新增 | `src/styles/theme-hacker.css` | 极客主题 CSS 变量覆盖 |
| 修改 | `src/styles/tokens.css` | 统一使用 CSS 变量替代硬编码色值 |
| 修改 | `src/styles/memo-theme.css` | 便签配色适配暗色模式 |
| 修改 | `src/styles/global.css` | CRT 扫描线 |
| 修改 | `src/components/ManagementPanel.vue` | 主题切换 pill |
| 修改 | `src/main.ts` | 启动时加载主题 |
| 修改 | `src/App.vue` | 注入 MatrixRain |
| 不变 | `src-tauri/**` | 零 Rust 改动 |
