<script setup lang="ts">
import { ref } from 'vue'
import * as api from '../utils/tauri'

const props = defineProps<{ noteId: string }>()
const emit = defineEmits<{ close: [] }>()

const repeatType = ref<'once' | 'daily' | 'weekly'>('once')
const remindDate = ref('')
const remindTime = ref('09:00')
const selectedDays = ref<number[]>([1, 3, 5])
const weekLabels = ['一', '二', '三', '四', '五', '六', '日']

function toggleDay(n: number) {
  const idx = selectedDays.value.indexOf(n)
  if (idx >= 0) selectedDays.value.splice(idx, 1)
  else selectedDays.value.push(n)
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
      <div class="dialog-title">🔔 设置提醒</div>
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
.dialog { background: #fff; border-radius: var(--rounded-lg); padding: 24px; min-width: 360px; font-size: 14px; }
.dialog-title { font-weight: 700; font-size: 16px; margin-bottom: 16px; }
.repeat-pills { display: flex; gap: 8px; margin-bottom: 14px; }
.pill { padding: 6px 16px; border-radius: var(--rounded-pill); cursor: pointer; border: 1px solid var(--color-hairline); font-size: 13px; }
.pill.active { background: var(--color-primary); color: var(--color-on-primary); border-color: var(--color-primary); }
.field { margin-bottom: 14px; }
.field label { display: block; font-weight: 600; margin-bottom: 4px; }
.datetime-row { display: flex; gap: 8px; }
.datetime-row input { border: 1px solid var(--color-hairline); border-radius: var(--rounded-md); padding: 8px 12px; font-size: 14px; }
.weekday-picker { display: flex; gap: 6px; }
.day-pill { padding: 6px 10px; border-radius: var(--rounded-pill); cursor: pointer; font-size: 12px; border: 1px solid var(--color-hairline); }
.day-pill.active { background: var(--color-primary); color: var(--color-on-primary); }
.dialog-actions { display: flex; gap: 8px; justify-content: flex-end; margin-top: 16px; }
.btn-cancel { border: 1px solid var(--color-hairline); background: #fff; padding: 8px 20px; border-radius: var(--rounded-pill); cursor: pointer; }
.btn-confirm { background: var(--color-primary); color: var(--color-on-primary); border: none; padding: 8px 20px; border-radius: var(--rounded-pill); cursor: pointer; }
</style>
