# 主题系统实施计划

> **For agentic workers:** Use superpowers:subagent-driven-development. Steps use checkbox (`- [ ]`) syntax.

**Goal:** Theme switching system — Macaron (current) + Hacker (Matrix dark + green + digital rain). CSS variable override + Canvas background + SQLite preference storage.

**Architecture:** `<html>` element gets `theme-macaron` or `theme-hacker` class. All CSS uses variables that change per theme. MatrixRain.vue is a Canvas component only active in hacker theme. Theme preference stored in SQLite settings table.

**Tech Stack:** Vue 3 + CSS Variables + Canvas 2D + existing SQLite settings

**Work directory:** `f:/sticky-notes-v2/`

---

### Task 1: Create theme-hacker.css + update tokens.css with CSS variables

**Files:**
- Create: `f:/sticky-notes-v2/src/styles/theme-hacker.css`
- Modify: `f:/sticky-notes-v2/src/styles/tokens.css`

- [ ] **Step 1: Update tokens.css to use CSS variables**

Read `f:/sticky-notes-v2/src/styles/tokens.css`. Replace the hardcoded `:root` values with CSS variables that can be overridden:

```css
:root,
html.theme-macaron {
  --color-bg: #ffffff;
  --color-surface: #f7f7f5;
  --color-primary: #000000;
  --color-on-primary: #ffffff;
  --color-ink: #000000;
  --color-hairline: #e6e6e6;
  --color-accent-magenta: #ff3d8b;
  --color-success: #1ea64a;
  --color-hover: rgba(0,0,0,0.08);
  --color-shadow: rgba(0,0,0,0.12);
  --font-ui: 'Inter', 'FangSong', '仿宋', 'SimSun', '宋体', sans-serif;
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
```

Keep existing `--color-block-*` variables unchanged (they're only used in legacy PASTEL_COLORS mode).

Update `body`:
```css
body {
  font-family: var(--font-ui);
  color: var(--color-ink);
  background: var(--color-bg);
  overflow: hidden;
  user-select: none;
}
```

Remove the old color definitions (`--color-primary: #000000;` etc.) that are now in the theme block.

- [ ] **Step 2: Create theme-hacker.css**

Create `f:/sticky-notes-v2/src/styles/theme-hacker.css`:

```css
/* ====== 极客黑客主题 ====== */

html.theme-hacker {
  --color-bg: #0D1117;
  --color-surface: #161B22;
  --color-primary: #00FF41;
  --color-on-primary: #0D1117;
  --color-ink: #E6EDF3;
  --color-hairline: rgba(0, 255, 65, 0.15);
  --color-accent-magenta: #FF4444;
  --color-success: #00FF41;
  --color-hover: rgba(0, 255, 65, 0.08);
  --color-shadow: rgba(0, 255, 65, 0.08);
  --font-ui: 'JetBrains Mono', 'FangSong', '仿宋', monospace;
}

/* CRT 扫描线 */
html.theme-hacker body::after {
  content: '';
  position: fixed;
  inset: 0;
  pointer-events: none;
  z-index: 1;
  background: repeating-linear-gradient(
    0deg,
    transparent,
    transparent 2px,
    rgba(0, 0, 0, 0.04) 2px,
    rgba(0, 0, 0, 0.04) 4px
  );
}

/* 便签卡片暗色化 */
html.theme-hacker .memo-window {
  background: var(--color-surface);
  border: 1px solid var(--color-hairline);
  box-shadow: 0 0 12px rgba(0, 255, 65, 0.05);
}

html.theme-hacker .memo-window::before {
  /* 替代胶带 → 顶部发光横线 */
  background: linear-gradient(90deg, transparent, rgba(0,255,65,0.3), transparent) !important;
  height: 1px !important;
  width: calc(100% - 24px) !important;
  top: -1px !important;
  border-radius: 0 !important;
  filter: none !important;
  box-shadow: none !important;
}

html.theme-hacker .memo-window::after {
  /* 去掉纸张噪点 */
  display: none;
}

/* 覆盖配色类 */
html.theme-hacker .memo-pink,
html.theme-hacker .memo-purple,
html.theme-hacker .memo-green,
html.theme-hacker .memo-blue,
html.theme-hacker .memo-yellow {
  background: var(--color-surface);
  color: var(--color-ink);
}

/* 管理面板 + 侧栏 */
html.theme-hacker .management-panel {
  background: var(--color-bg);
}

html.theme-hacker .sidebar {
  background: var(--color-bg) !important;
  backdrop-filter: none !important;
  -webkit-backdrop-filter: none !important;
  border-right-color: var(--color-hairline) !important;
}

/* 按钮 */
html.theme-hacker .new-btn,
html.theme-hacker .btn-confirm {
  background: transparent;
  color: var(--color-primary);
  border: 1px solid var(--color-primary);
}

html.theme-hacker .chrome-btn:hover {
  background: var(--color-hover);
}

/* 输入框 */
html.theme-hacker input,
html.theme-hacker textarea,
html.theme-hacker select {
  background: var(--color-surface);
  color: var(--color-ink);
  border-color: var(--color-hairline);
}

html.theme-hacker input:focus,
html.theme-hacker textarea:focus {
  border-color: var(--color-primary);
  box-shadow: 0 0 0 2px rgba(0, 255, 65, 0.15);
}

/* 弹窗 */
html.theme-hacker .dialog,
html.theme-hacker .confirm-box,
html.theme-hacker .inline-input-box {
  background: var(--color-surface);
  color: var(--color-ink);
  border: 1px solid var(--color-hairline);
}

/* 上下文菜单 */
html.theme-hacker .context-menu {
  background: var(--color-surface);
  border-color: var(--color-hairline);
  color: var(--color-ink);
}
```

- [ ] **Step 3: Import theme-hacker.css in main.ts**

Read `f:/sticky-notes-v2/src/main.ts`. Add after `import './styles/memo-theme.css'`:
```typescript
import './styles/theme-hacker.css'
```

- [ ] **Step 4: Verify and commit**

```bash
cd f:/sticky-notes-v2 && npx vue-tsc --noEmit 2>&1 | head -5 && npx vite build 2>&1 | tail -3
git add -A && git commit -m "feat: CSS theme variables + hacker theme with CRT scanlines"
```

---

### Task 2: Create MatrixRain.vue Canvas component

**Files:**
- Create: `f:/sticky-notes-v2/src/components/MatrixRain.vue`

- [ ] **Step 1: Create MatrixRain.vue**

```vue
<script setup lang="ts">
import { ref, onMounted, onUnmounted, watch } from 'vue'

const props = defineProps<{ active: boolean }>()
const canvas = ref<HTMLCanvasElement>()
let animId = 0
let drops: number[] = []
let columns = 0

const chars = 'ｦｧｨｩｪｫｬｭｮｯｱｲｳｴｵｶｷｸｹｺｻｼｽｾｿﾀﾁﾂﾃ0123456789'
const fontSize = 13
const color = 'rgba(0, 255, 65, 0.12)'
const headColor = 'rgba(0, 255, 65, 0.25)'

function init() {
  if (!canvas.value) return
  const c = canvas.value
  c.width = window.innerWidth
  c.height = window.innerHeight
  columns = Math.floor(c.width / (fontSize * 0.8))
  drops = Array(columns).fill(0).map(() => Math.random() * -100)
}

function draw() {
  if (!canvas.value) return
  const c = canvas.value
  const ctx = c.getContext('2d')
  if (!ctx) return

  ctx.fillStyle = 'rgba(13, 17, 23, 0.05)'
  ctx.fillRect(0, 0, c.width, c.height)

  ctx.font = `${fontSize}px 'JetBrains Mono'`
  for (let i = 0; i < drops.length; i++) {
    const char = chars[Math.floor(Math.random() * chars.length)]
    const x = i * fontSize * 0.8
    const y = drops[i] * fontSize

    // 头部亮一点
    ctx.fillStyle = headColor
    ctx.fillText(char, x, y)

    // 尾部更暗
    ctx.fillStyle = color
    ctx.fillText(char, x, y - fontSize)

    if (y > c.height && Math.random() > 0.975) {
      drops[i] = 0
    }
    drops[i]++
  }
  animId = requestAnimationFrame(draw)
}

function start() {
  if (!props.active) return
  init()
  animId = requestAnimationFrame(draw)
}

function stop() {
  cancelAnimationFrame(animId)
}

watch(() => props.active, (val) => {
  if (val) start()
  else stop()
})

onMounted(() => { if (props.active) start() })
onUnmounted(() => stop())
</script>

<template>
  <canvas v-if="active" ref="canvas" class="matrix-rain" />
</template>

<style scoped>
.matrix-rain {
  position: fixed;
  inset: 0;
  z-index: 0;
  pointer-events: none;
}
</style>
```

- [ ] **Step 2: Verify and commit**

```bash
cd f:/sticky-notes-v2 && npx vue-tsc --noEmit 2>&1 | head -5 && npx vite build 2>&1 | tail -3
git add -A && git commit -m "feat: MatrixRain canvas component for hacker theme"
```

---

### Task 3: Theme switcher in ManagementPanel + App.vue + main.ts

**Files:**
- Modify: `f:/sticky-notes-v2/src/main.ts`
- Modify: `f:/sticky-notes-v2/src/App.vue`
- Modify: `f:/sticky-notes-v2/src/components/ManagementPanel.vue`

- [ ] **Step 1: Update main.ts — load theme on startup**

Read main.ts. Add after Pinia setup:
```typescript
import { invoke } from '@tauri-apps/api/core'

async function loadTheme() {
  try {
    const theme = await invoke<string | null>('get_setting', { key: 'theme' })
    document.documentElement.className = theme === 'hacker' ? 'theme-hacker' : 'theme-macaron'
  } catch {
    document.documentElement.className = 'theme-macaron'
  }
}
loadTheme()
```

- [ ] **Step 2: Update App.vue — add theme state + MatrixRain**

Read App.vue. Add MatrixRain and theme state:
```typescript
import MatrixRain from './components/MatrixRain.vue'
const isHackerTheme = ref(document.documentElement.classList.contains('theme-hacker'))
```

Add MatrixRain to template:
```html
<MatrixRain :active="isHackerTheme" />
```

- [ ] **Step 3: Update ManagementPanel.vue — theme switcher pill**

Read ManagementPanel.vue. Add after the `closePanel` function:
```typescript
const currentTheme = ref<'macaron' | 'hacker'>(
  document.documentElement.classList.contains('theme-hacker') ? 'hacker' : 'macaron'
)

async function setTheme(theme: 'macaron' | 'hacker') {
  currentTheme.value = theme
  document.documentElement.className = theme === 'hacker' ? 'theme-hacker' : 'theme-macaron'
  await invoke('set_setting', { key: 'theme', value: theme })
}
```

Add theme switcher in the header, before the "+ 新建" button:
```html
<div class="theme-toggle" data-tauri-drag-region="false">
  <span class="view-pill" :class="{ active: currentTheme === 'macaron' }" @click="setTheme('macaron')">马卡龙</span>
  <span class="view-pill" :class="{ active: currentTheme === 'hacker' }" @click="setTheme('hacker')">极客黑客</span>
</div>
```

Add CSS:
```css
.theme-toggle { display: flex; gap: 4px; background: var(--color-surface); padding: 3px; border-radius: var(--rounded-pill); -webkit-app-region: no-drag; margin-right: 8px; }
```

- [ ] **Step 4: Verify and commit**

```bash
cd f:/sticky-notes-v2 && npx vue-tsc --noEmit 2>&1 | head -5 && npx vite build 2>&1 | tail -3
git add -A && git commit -m "feat: theme switcher in management panel + theme init on startup"
```

---

### Task 4: Full build verification

- [ ] **Step 1: Full Tauri build**

```bash
cd f:/sticky-notes-v2 && npx tauri build --bundles nsis 2>&1 | tail -5
```

- [ ] **Step 2: Manual test**

- [ ] 默认启动 = 马卡龙主题 ✓
- [ ] 切换到极客黑客 → 暗色 + 绿色立即生效 ✓
- [ ] 数字雨动画播放 ✓
- [ ] CRT 扫描线可见 ✓
- [ ] 便签窗口暗色 + 绿光边框 ✓
- [ ] 管理面板暗色 ✓
- [ ] 切回马卡龙 → 恢复正常 ✓
- [ ] 关闭重开 → 记住主题偏好 ✓

- [ ] **Step 3: Final commit**

```bash
git add -A && git commit -m "chore: theme system build verification passed"
```
