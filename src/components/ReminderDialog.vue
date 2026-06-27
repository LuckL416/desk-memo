<script setup lang="ts">
import { ref } from 'vue'
import * as api from '../utils/tauri'

const props = defineProps<{ noteId: string }>()
const emit = defineEmits<{ close: [] }>()

const repeatType = ref<'once' | 'daily' | 'weekly'>('once')

// 默认今天
const today = new Date()
const remindDate = ref(today.toISOString().slice(0, 10))
const remindTime = ref('09:00')
const selectedDays = ref<number[]>([1, 3, 5])
const weekLabels = ['一', '二', '三', '四', '五', '六', '日']

function toggleDay(n: number) {
  const idx = selectedDays.value.indexOf(n)
  if (idx >= 0) selectedDays.value.splice(idx, 1)
  else selectedDays.value.push(n)
}

function quickPreset(minutes: number) {
  const d = new Date(Date.now() + minutes * 60000)
  remindDate.value = d.toISOString().slice(0, 10)
  remindTime.value = d.toTimeString().slice(0, 5)
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
      <div class="dialog-title">设置提醒</div>

      <!-- 快捷预设 -->
      <div class="quick-presets">
        <span class="preset" @click="quickPreset(5)">5分钟后</span>
        <span class="preset" @click="quickPreset(15)">15分钟后</span>
        <span class="preset" @click="quickPreset(30)">30分钟后</span>
        <span class="preset" @click="quickPreset(60)">1小时后</span>
      </div>

      <div class="repeat-pills">
        <span class="pill" :class="{ active: repeatType === 'once' }" @click="repeatType = 'once'">一次性</span>
        <span class="pill" :class="{ active: repeatType === 'daily' }" @click="repeatType = 'daily'">每日</span>
        <span class="pill" :class="{ active: repeatType === 'weekly' }" @click="repeatType = 'weekly'">每周</span>
      </div>

      <div class="field">
        <label>日期 & 时间</label>
        <div class="datetime-row">
          <input type="date" v-model="remindDate" />
          <input type="time" v-model="remindTime" />
        </div>
      </div>

      <div v-if="repeatType === 'weekly'" class="field">
        <label>重复日期</label>
        <div class="weekday-picker">
          <span v-for="(l, i) in weekLabels" :key="i" class="day-pill"
            :class="{ active: selectedDays.includes(i + 1) }"
            @click="toggleDay(i + 1)">{{ l }}</span>
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
.dialog { background: #fff; border-radius: var(--rounded-lg); padding: 24px; min-width: 360px; font-size: 14px; max-height: 90vh; overflow-y: auto; }
.dialog-title { font-weight: 700; font-size: 16px; margin-bottom: 14px; }
.quick-presets { display: flex; gap: 6px; margin-bottom: 14px; flex-wrap: wrap; }
.preset { padding: 4px 12px; border-radius: var(--rounded-pill); cursor: pointer; font-size: 12px; background: var(--color-surface-soft); border: 1px solid var(--color-hairline); }
.preset:hover { background: var(--color-primary); color: var(--color-on-primary); border-color: var(--color-primary); }
.repeat-pills { display: flex; gap: 8px; margin-bottom: 14px; }
.pill { padding: 6px 16px; border-radius: var(--rounded-pill); cursor: pointer; border: 1px solid var(--color-hairline); font-size: 13px; }
.pill.active { background: var(--color-primary); color: var(--color-on-primary); border-color: var(--color-primary); }
.field { margin-bottom: 14px; }
.field label { display: block; font-weight: 600; margin-bottom: 4px; }
.datetime-row { display: flex; gap: 8px; }
.datetime-row input { border: 1px solid var(--color-hairline); border-radius: var(--rounded-md); padding: 8px 12px; font-size: 14px; }
.weekday-picker { display: flex; gap: 6px; }
.day-pill { padding: 6px 10px; border-radius: var(--rounded-pill); cursor: pointer; font-size: 12px; border: 1px solid var(--color-hairline); }
.day-pill.active { background: var(--color-primary); color: var(--color-on-primary); border-color: var(--color-primary); }
.dialog-actions { display: flex; gap: 8px; justify-content: flex-end; margin-top: 16px; }
.btn-cancel { border: 1px solid var(--color-hairline); background: #fff; padding: 8px 20px; border-radius: var(--rounded-pill); cursor: pointer; }
.btn-confirm { background: var(--color-primary); color: var(--color-on-primary); border: none; padding: 8px 20px; border-radius: var(--rounded-pill); cursor: pointer; }
</style>
