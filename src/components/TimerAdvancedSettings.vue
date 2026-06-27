<script setup lang="ts">
import { ref } from 'vue'
import type { TimerState } from '../types'

const props = defineProps<{ timerState: TimerState }>()
const emit = defineEmits<{ update: [settings: Record<string, unknown>] }>()

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
.advanced-settings { margin-top: 12px; padding: 14px; border-radius: var(--rounded-md); font-size: 13px; width: 100%; max-width: 280px; }
.setting-row { display: flex; justify-content: space-between; align-items: center; margin-bottom: 8px; }
.setting-row input, .setting-row select { padding: 4px 8px; border-radius: 4px; font-size: 12px; border: 1px solid rgba(255,255,255,0.2); background: rgba(255,255,255,0.12); color: var(--color-ink); }
.setting-row input[type="number"] { width: 50px; text-align: center; }
.time-inputs { display: flex; align-items: center; gap: 4px; }
</style>
