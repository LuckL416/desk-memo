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

function prevMonth() { currentDate.value = new Date(year.value, month.value - 1, 1); selectedDate.value = null }
function nextMonth() { currentDate.value = new Date(year.value, month.value + 1, 1); selectedDate.value = null }

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
  return notes.value.filter(n => n.created_at.startsWith(ds) || n.updated_at.startsWith(ds))
}

function dotColor(type: string) {
  return { text: '#000', todo: '#1ea64a', timer: '#ff3d8b' }[type] || '#000'
}

function selectDate(day: number) {
  selectedDate.value = dateStr(day)
  dateNotes.value = notesForDay(day)
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
      <template v-for="(week, wi) in weeks" :key="wi">
        <div v-for="(day, di) in week" :key="`${wi}-${di}`" class="cal-cell"
          :class="{ 'cal-today': day && dateStr(day) === today, 'cal-other-month': day === null }"
          @click="day && selectDate(day)">
          <div class="cal-day-num" v-if="day">{{ day }}</div>
          <div class="cal-day-notes" v-if="day">
            <div v-for="n in notesForDay(day).slice(0, 3)" :key="n.id" class="cal-note-dot" :title="n.title || ''">
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
      <div v-for="n in dateNotes" :key="n.id" class="detail-item">
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
.cal-note-dot { display: flex; align-items: center; gap: 3px; font-size: 10px; margin-bottom: 2px; overflow: hidden; white-space: nowrap; }
.dot { width: 5px; height: 5px; border-radius: 50%; flex-shrink: 0; }
.dot-label { overflow: hidden; text-overflow: ellipsis; }
.cal-more { font-size: 10px; color: #888; }
.cal-detail { margin-top: 14px; padding: 12px 16px; background: var(--color-surface-soft); border-radius: var(--rounded-md); }
.detail-title { font-weight: 600; margin-bottom: 6px; font-size: 13px; }
.detail-item { display: flex; align-items: center; gap: 8px; padding: 6px 10px; background: #fff; border-radius: 6px; margin-bottom: 2px; cursor: pointer; font-size: 13px; }
.detail-item:hover { background: #f0f0f0; }
.detail-meta { margin-left: auto; font-size: 11px; color: #888; }
</style>
