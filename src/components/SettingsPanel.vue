<script setup lang="ts">
import { ref, onMounted } from 'vue'
import * as api from '../utils/tauri'

const dataPath = ref('')
const backupPath = ref('')
const backupKeepDays = ref('7')
const trashAutoCleanDays = ref('30')
const autostart = ref(true)
const exportPath = ref('')

onMounted(async () => {
  dataPath.value = (await api.getSetting('data_path')) || '%APPDATA%/StickyNotes/'
  backupPath.value = (await api.getSetting('backup_path')) || ''
  backupKeepDays.value = (await api.getSetting('backup_keep_days')) || '7'
  trashAutoCleanDays.value = (await api.getSetting('trash_auto_clean_days')) || '30'
  autostart.value = (await api.getSetting('autostart')) !== 'false'
})

async function saveSetting(key: string, value: string) {
  await api.setSetting(key, value)
}

async function handleExport() {
  if (exportPath.value) {
    await api.exportAllData(exportPath.value)
    alert('导出成功！')
  }
}
</script>

<template>
  <div class="settings-panel">
    <h2>⚙️ 软件设置</h2>
    <div class="setting-item">
      <label>数据存储路径</label>
      <input v-model="dataPath" @change="saveSetting('data_path', dataPath)" placeholder="%APPDATA%/StickyNotes/" />
    </div>
    <div class="setting-item">
      <label>备份路径</label>
      <input v-model="backupPath" @change="saveSetting('backup_path', backupPath)" />
    </div>
    <div class="setting-item">
      <label>备份保留天数</label>
      <input type="number" v-model="backupKeepDays" @change="saveSetting('backup_keep_days', backupKeepDays)" />
    </div>
    <div class="setting-item">
      <label>回收站自动清理（天）</label>
      <input type="number" v-model="trashAutoCleanDays" @change="saveSetting('trash_auto_clean_days', trashAutoCleanDays)" placeholder="0=不自动清理" />
    </div>
    <div class="setting-item">
      <label>开机自启动</label>
      <input type="checkbox" :checked="autostart" @change="autostart = !autostart; saveSetting('autostart', String(autostart))" />
    </div>
    <div class="setting-item">
      <label>导出数据</label>
      <input v-model="exportPath" placeholder="选择导出路径..." />
      <button @click="handleExport">导出 JSON</button>
    </div>
  </div>
</template>

<style scoped>
.settings-panel { padding: 24px; font-size: 14px; }
h2 { font-size: 18px; margin-bottom: 20px; }
.setting-item { margin-bottom: 14px; display: flex; align-items: center; gap: 12px; }
.setting-item label { font-weight: 600; width: 160px; flex-shrink: 0; }
.setting-item input { border: 1px solid var(--color-hairline); border-radius: var(--rounded-md); padding: 6px 10px; flex: 1; }
.setting-item button { background: var(--color-primary); color: var(--color-on-primary); border: none; padding: 6px 14px; border-radius: var(--rounded-pill); cursor: pointer; font-size: 13px; }
</style>
