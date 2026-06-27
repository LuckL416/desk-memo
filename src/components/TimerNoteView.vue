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
const unlisteners: (() => void)[] = []

onMounted(async () => {
  timerState.value = await api.getTimerState(props.note.id)
  const u1 = await api.onTimerTick((data) => {
    if (data.note_id === props.note.id) {
      timerState.value.remaining_seconds = data.remaining_seconds
      timerState.value.is_running = data.is_running
    }
  })
  const u2 = await api.onTimerFinished((data) => {
    if (data.note_id === props.note.id) {
      timerState.value.is_running = false
      timerState.value.remaining_seconds = 0
    }
  })
  const u3 = await api.onMidnightReset(() => {
    timerState.value.remaining_seconds = timerState.value.daily_duration_minutes * 60
    timerState.value.is_running = false
  })
  unlisteners.push(u1, u2, u3)
})

onUnmounted(() => unlisteners.forEach(f => { try { f() } catch { /* ignore */ } }))

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

async function handleSettingsUpdate(settings: Record<string, unknown>) {
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
.timer-view { display: flex; flex-direction: column; align-items: center; padding: 20px; height: 100%; }
.timer-display { font-family: var(--font-mono); font-size: 56px; font-weight: 340; letter-spacing: -1px; }
.timer-subtitle { font-size: 14px; opacity: 0.5; margin-top: 4px; }
.timer-controls { display: flex; gap: 10px; margin-top: 16px; }
.btn-primary { background: #fff; color: #000; border: none; padding: 10px 32px; border-radius: var(--rounded-pill); font-size: 16px; font-weight: 500; cursor: pointer; }
.btn-secondary { background: rgba(255,255,255,0.12); border: 1px solid rgba(255,255,255,0.2); padding: 10px 16px; border-radius: var(--rounded-pill); font-size: 14px; cursor: pointer; color: var(--color-ink); }
.tank-info { margin-top: 14px; font-size: 13px; }
.advanced-toggle { margin-top: 16px; background: none; border: none; font-size: 12px; cursor: pointer; color: var(--color-ink); }
</style>
