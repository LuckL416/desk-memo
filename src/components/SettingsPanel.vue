<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { invoke } from '@tauri-apps/api/core'
import { enable, disable, isEnabled } from '@tauri-apps/plugin-autostart'

const dataPath = ref('')
const backupPath = ref('')
const backupKeepDays = ref('7')
const trashAutoCleanDays = ref('30')
const autostart = ref(false)
const exportPath = ref('')

onMounted(async () => {
  dataPath.value = (await invoke<string | null>('get_setting', { key: 'data_path' })) || ''
  backupPath.value = (await invoke<string | null>('get_setting', { key: 'backup_path' })) || ''
  backupKeepDays.value = (await invoke<string | null>('get_setting', { key: 'backup_keep_days' })) || '7'
  trashAutoCleanDays.value = (await invoke<string | null>('get_setting', { key: 'trash_auto_clean_days' })) || '30'
  try { autostart.value = await isEnabled() } catch { autostart.value = false }
})

async function saveSetting(key: string, value: string) {
  await invoke('set_setting', { key, value })
}

async function pickDataPath() {
  const path = await invoke<string | null>('pick_folder')
  if (path) { dataPath.value = path; await saveSetting('data_path', path) }
}

async function pickBackupPath() {
  const path = await invoke<string | null>('pick_folder')
  if (path) { backupPath.value = path; await saveSetting('backup_path', path) }
}

async function pickExportPath() {
  const path = await invoke<string | null>('pick_save_file', { defaultName: 'sticky-notes-export.json' })
  if (path) { exportPath.value = path }
}

async function doExport() {
  if (!exportPath.value) return
  await invoke('export_all_data', { path: exportPath.value })
  alert('导出成功！')
}

async function toggleAutostart() {
  autostart.value = !autostart.value
  try {
    if (autostart.value) { await enable() } else { await disable() }
  } catch { autostart.value = !autostart.value }
}

async function closePanel() {
  await getCurrentWindow().hide()
}
</script>

<template>
  <div class="settings-panel">
    <div class="s-header" data-tauri-drag-region>
      <h2>软件设置</h2>
      <button class="close-btn" data-tauri-drag-region="false" @click="closePanel">✕</button>
    </div>
    <div class="s-body">
      <div class="setting-item">
        <label>数据存储路径</label>
        <div class="path-row">
          <span class="path-text">{{ dataPath || '默认 (%APPDATA%/StickyNotes/)' }}</span>
          <button @click="pickDataPath">选择文件夹</button>
        </div>
      </div>
      <div class="setting-item">
        <label>备份路径</label>
        <div class="path-row">
          <span class="path-text">{{ backupPath || '默认 (数据目录/backups/)' }}</span>
          <button @click="pickBackupPath">选择文件夹</button>
        </div>
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
        <input type="checkbox" :checked="autostart" @change="toggleAutostart" />
      </div>
      <div class="setting-item">
        <label>导出数据</label>
        <div class="path-row">
          <span class="path-text">{{ exportPath || '选择保存位置...' }}</span>
          <button @click="pickExportPath">选择文件</button>
          <button v-if="exportPath" class="btn-export" @click="doExport">导出</button>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.settings-panel { display: flex; flex-direction: column; height: 100vh; background: var(--color-canvas); }
.s-header { display: flex; align-items: center; justify-content: space-between; padding: 14px 20px; border-bottom: 1px solid var(--color-hairline); cursor: grab; -webkit-app-region: drag; user-select: none; }
.s-header h2 { font-size: 18px; font-weight: 700; }
.close-btn { background: rgba(0,0,0,0.06); border: 1px solid rgba(0,0,0,0.1); cursor: pointer; font-size: 14px; padding: 4px 10px; border-radius: var(--rounded-pill); color: var(--color-ink); -webkit-app-region: no-drag; }
.close-btn:hover { background: #e81123; color: #fff; border-color: #e81123; }
.s-body { flex: 1; overflow-y: auto; padding: 20px 24px; }
.setting-item { margin-bottom: 18px; }
.setting-item label { display: block; font-weight: 600; margin-bottom: 6px; font-size: 14px; }
.setting-item input[type="number"] { border: 1px solid var(--color-hairline); border-radius: var(--rounded-md); padding: 6px 10px; width: 80px; font-size: 14px; }
.path-row { display: flex; align-items: center; gap: 8px; }
.path-text { flex: 1; font-size: 13px; color: #666; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; background: var(--color-surface-soft); padding: 6px 10px; border-radius: var(--rounded-md); }
.path-row button { background: var(--color-primary); color: var(--color-on-primary); border: none; padding: 6px 14px; border-radius: var(--rounded-pill); font-size: 13px; cursor: pointer; white-space: nowrap; }
.btn-export { background: var(--color-success) !important; }
</style>
