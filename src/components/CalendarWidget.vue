<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { getCurrentWindow, LogicalSize } from '@tauri-apps/api/window'
import * as api from '../utils/tauri'
import type { Note } from '../types'

const calId = 'calendar-widget'
const currentDate = ref(new Date())
const notes = ref<Note[]>([])
const pinned = ref(false)
const opacity = ref(1.0)
const showOpacity = ref(false)

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
  const startOffset = (firstDay.getDay() + 6) % 7
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
  return notes.value.filter(n => n.created_at?.startsWith(ds) || n.updated_at?.startsWith(ds))
}

function dotColor(type: string) {
  return { text: '#000', todo: '#1ea64a', timer: '#ff3d8b' }[type] || '#000'
}

const noteCount = computed(() => notes.value.length)

async function handleClose() { await getCurrentWindow().close() }
async function handleHide() { await getCurrentWindow().hide() }

// 锁定 — 控制固定大小 + 禁止拖动（与 NoteChrome 完全相同的方案）
async function handleTogglePinned() {
  pinned.value = !pinned.value
  const win = getCurrentWindow()
  if (pinned.value) {
    const size = await win.innerSize()
    await win.setMinSize(new LogicalSize(size.width, size.height))
    await win.setMaxSize(new LogicalSize(size.width, size.height))
  } else {
    await win.setMinSize(new LogicalSize(400, 300))
    await win.setMaxSize(new LogicalSize(10000, 10000))
  }
}

// 透明度 — 与便签完全相同的方案
async function handleUpdateOpacity(val: number) {
  opacity.value = val
  api.setWindowOpacity(calId, val).catch(() => {})
}

onMounted(async () => {
  notes.value = await api.listNotes(undefined, undefined)
})
</script>

<template>
  <div class="cal-widget">
    <div class="cal-chrome" :class="{ 'chrome-locked': pinned }" :data-tauri-drag-region="pinned ? undefined : ''">
      <span class="cal-title">📅 桌面日历 — {{ year }}年{{ month + 1 }}月</span>
      <div class="cal-actions" data-tauri-drag-region="false">
        <div class="opacity-wrap" data-tauri-drag-region="false">
          <button data-tauri-drag-region="false" @click.stop="showOpacity = !showOpacity">透明度</button>
          <div v-if="showOpacity" class="opacity-dropdown" data-tauri-drag-region="false" @click.stop>
            <input type="range" min="5" max="100" :value="Math.round(opacity * 100)"
              @input="handleUpdateOpacity(Number(($event.target as HTMLInputElement).value) / 100)" />
            <span>{{ Math.round(opacity * 100) }}%</span>
          </div>
        </div>
        <button data-tauri-drag-region="false" @click.stop="handleTogglePinned">{{ pinned ? '解锁' : '锁定' }}</button>
        <button data-tauri-drag-region="false" @click.stop="handleHide">−</button>
        <button data-tauri-drag-region="false" class="cal-close" @click.stop="handleClose">×</button>
      </div>
    </div>
    <div class="cal-body">
      <div class="cal-nav">
        <span class="cal-arrow" @click="prevMonth">◂</span>
        <span class="cal-month">{{ year }}年{{ month + 1 }}月</span>
        <span class="cal-arrow" @click="nextMonth">▸</span>
        <span class="cal-count">共 {{ noteCount }} 条</span>
        <span class="cal-legend">
          <span style="color:#000;">● 文本</span>
          <span style="color:var(--color-success);">● 待办</span>
          <span style="color:var(--color-accent-magenta);">● 计时</span>
        </span>
      </div>
      <div class="cal-grid">
        <div class="cal-dh" v-for="d in ['一','二','三','四','五','六','日']" :key="d">{{ d }}</div>
        <template v-for="(week, wi) in weeks" :key="wi">
          <div v-for="(day, di) in week" :key="`${wi}-${di}`" class="cal-cell"
            :class="{ 'cal-today': day && dateStr(day) === today, 'cal-empty': day === null }">
            <div class="cal-day-num" v-if="day">{{ day }}</div>
            <div class="cal-day-notes" v-if="day">
              <div v-for="n in notesForDay(day).slice(0, 3)" :key="n.id" class="cal-note-item">
                <span class="dot" :style="{ background: dotColor(n.type) }"></span>
                <span class="dot-label">{{ n.title || '无标题' }}</span>
              </div>
              <div v-if="notesForDay(day).length > 3" class="cal-more">+{{ notesForDay(day).length - 3 }}</div>
            </div>
          </div>
        </template>
      </div>
    </div>
  </div>
</template>

<style scoped>
.cal-widget { display: flex; flex-direction: column; height: 100vh; background: #fff; font-size: 12px; }
.cal-chrome { display: flex; align-items: center; height: 36px; padding: 0 6px 0 10px; background: rgba(0,0,0,0.12); cursor: grab; -webkit-app-region: drag; user-select: none; }
.chrome-locked { -webkit-app-region: no-drag; cursor: default; }
.cal-title { font-size: 13px; font-weight: 600; }
.cal-actions { margin-left: auto; display: flex; gap: 3px; -webkit-app-region: no-drag; }
.cal-actions button { background: transparent; border: none; cursor: pointer; font-size: 12px; padding: 3px 7px; border-radius: var(--rounded-pill); -webkit-app-region: no-drag; }
.cal-actions button:hover { background: rgba(0,0,0,0.12); }
.cal-close:hover { background: #e81123 !important; color: #fff; }
.opacity-wrap { position: relative; -webkit-app-region: no-drag; }
.opacity-dropdown { position: absolute; top: 100%; left: 50%; transform: translateX(-50%); margin-top: 4px; background: #fff; border: 1px solid var(--color-hairline); border-radius: var(--rounded-md); padding: 8px 12px; display: flex; align-items: center; gap: 8px; font-size: 12px; z-index: 100; box-shadow: 0 2px 12px rgba(0,0,0,0.15); -webkit-app-region: no-drag; white-space: nowrap; }
.opacity-dropdown input[type="range"] { width: 100px; }
.cal-body { flex: 1; padding: 12px 14px; overflow: hidden; display: flex; flex-direction: column; }
.cal-nav { display: flex; align-items: center; gap: 8px; margin-bottom: 8px; }
.cal-month { font-weight: 600; font-size: 14px; }
.cal-arrow { cursor: pointer; font-size: 13px; }
.cal-count { font-size: 11px; color: #888; }
.cal-legend { margin-left: auto; display: flex; gap: 8px; font-size: 10px; }
.cal-grid { display: grid; grid-template-columns: repeat(7, 1fr); border-top: 1px solid #e6e6e6; border-left: 1px solid #e6e6e6; flex: 1; }
.cal-dh { padding: 4px 4px; text-align: center; font-weight: 600; color: #888; font-size: 11px; border-right: 1px solid #e6e6e6; border-bottom: 1px solid #e6e6e6; background: #f7f7f5; }
.cal-cell { padding: 4px; border-right: 1px solid #e6e6e6; border-bottom: 1px solid #e6e6e6; overflow: hidden; }
.cal-empty { background: #fafafa; }
.cal-today { background: #fffdf5; }
.cal-today .cal-day-num { background: #000; color: #fff; border-radius: 50%; width: 18px; height: 18px; display: inline-flex; align-items: center; justify-content: center; font-size: 10px; }
.cal-day-num { font-size: 11px; font-weight: 500; margin-bottom: 2px; }
.cal-day-notes { display: flex; flex-direction: column; gap: 1px; }
.cal-note-item { display: flex; align-items: center; gap: 3px; font-size: 10px; overflow: hidden; white-space: nowrap; }
.dot { width: 5px; height: 5px; border-radius: 50%; flex-shrink: 0; }
.dot-label { overflow: hidden; text-overflow: ellipsis; }
.cal-more { font-size: 10px; color: #888; }
</style>
