<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { getCurrentWindow, LogicalSize } from '@tauri-apps/api/window'
import NoteChrome from './NoteChrome.vue'
import TextNoteEditor from './TextNoteEditor.vue'
import TodoNoteEditor from './TodoNoteEditor.vue'
import TimerNoteView from './TimerNoteView.vue'
import ReminderDialog from './ReminderDialog.vue'
import * as api from '../utils/tauri'
import type { Note } from '../types'
import { MEMO_COLORS, randomMemoColor } from '../types'

// Match existing bg_color to a memo class, or assign random
function getMemoClass(bgColor: string | null): string {
  if (!bgColor) return `memo-${randomMemoColor().name}`
  const match = MEMO_COLORS.find(c => c.hex === bgColor)
  return match ? `memo-${match.name}` : `memo-${randomMemoColor().name}`
}
const memoClass = computed(() => getMemoClass(note.value?.bg_color ?? null))
const rotateDeg = (Math.random() * 4 - 2).toFixed(1)

const noteId = getCurrentWindow().label.replace('note-', '')

const note = ref<Note | null>(null)
const mode = ref<'desktop' | 'top'>('desktop')
const pinned = ref(false)
const opacity = ref(1.0)
const showContextMenu = ref(false)
const showReminder = ref(false)
const contextMenuPos = ref({ x: 0, y: 0 })
const errorMsg = ref('')
const loading = ref(true)

onMounted(async () => {
  if (!noteId) {
    errorMsg.value = '无法获取便签 ID (label: ' + getCurrentWindow().label + ')'
    loading.value = false
    return
  }
  try {
    note.value = await api.getNote(noteId)
    if (note.value?.bg_color) document.body.style.background = note.value.bg_color
    const ws = await api.getWindowState(noteId)
    if (ws) { mode.value = ws.mode; pinned.value = ws.pinned; opacity.value = ws.opacity }
  } catch (e: any) {
    errorMsg.value = '加载失败: ' + String(e)
  }
  loading.value = false
})

async function handleClose() {
  await api.saveWindowState({ note_id: noteId, x: 0, y: 0, width: 320, height: 240, opacity: opacity.value, pinned: pinned.value, mode: mode.value, is_visible: false })
  await getCurrentWindow().close()
}

async function handleHide() {
  await api.saveWindowState({ note_id: noteId, x: 0, y: 0, width: 320, height: 240, opacity: opacity.value, pinned: pinned.value, mode: mode.value, is_visible: false })
  await getCurrentWindow().hide()
}

async function handleToggleMode() {
  mode.value = mode.value === 'desktop' ? 'top' : 'desktop'
  await getCurrentWindow().setAlwaysOnTop(mode.value === 'top')
}

async function handleTogglePinned() {
  pinned.value = !pinned.value
  const win = getCurrentWindow()
  if (pinned.value) {
    const size = await win.innerSize()
    await win.setMinSize(new LogicalSize(size.width, size.height))
    await win.setMaxSize(new LogicalSize(size.width, size.height))
  } else {
    await win.setMinSize(new LogicalSize(200, 120))
    await win.setMaxSize(new LogicalSize(10000, 10000))
  }
}

let opacityTimer: ReturnType<typeof setTimeout> | null = null
async function handleUpdateOpacity(val: number) {
  opacity.value = val
  api.setWindowOpacity(noteId, val).catch(() => {})
  if (opacityTimer) clearTimeout(opacityTimer)
  opacityTimer = setTimeout(() => {
    api.saveWindowState({ note_id: noteId, x: 0, y: 0, width: 320, height: 240, opacity: val, pinned: pinned.value, mode: mode.value, is_visible: true }).catch(() => {})
  }, 300)
}

function handleContextMenu(event: MouseEvent) {
  contextMenuPos.value = { x: event.clientX, y: event.clientY }
  showContextMenu.value = true
}

let saveTimer: ReturnType<typeof setTimeout> | null = null
function debouncedSave(data: Partial<Note>) {
  if (saveTimer) clearTimeout(saveTimer)
  saveTimer = setTimeout(() => { api.updateNote(noteId, data).catch(console.error) }, 500)
}
</script>

<template>
  <div v-if="loading" class="note-status memo-window">加载中...</div>
  <div v-else-if="errorMsg" class="note-status note-error memo-window">
    <p>{{ errorMsg }}</p>
    <p style="font-size:12px;margin-top:8px;">noteId: {{ noteId }}</p>
  </div>
  <div v-else-if="note" class="note-window memo-window" :class="memoClass" :style="{ transform: `rotate(${rotateDeg}deg)` }" @contextmenu="handleContextMenu">
    <NoteChrome
      :title="note.title || ''" :mode="mode" :pinned="pinned" :opacity="opacity"
      @close="handleClose" @hide="handleHide" @toggle-mode="handleToggleMode"
      @toggle-pinned="handleTogglePinned" @update-opacity="handleUpdateOpacity"
      @show-menu="handleContextMenu"
    />
    <div class="note-body">
      <TextNoteEditor v-if="note.type === 'text'" :note="note" @update="debouncedSave" />
      <TodoNoteEditor v-else-if="note.type === 'todo'" :note="note" @update="debouncedSave" />
      <TimerNoteView v-else-if="note.type === 'timer'" :note="note" />
    </div>
    <Teleport to="body">
      <div v-if="showContextMenu" class="context-overlay" @click="showContextMenu = false">
        <div class="context-menu" :style="{ left: contextMenuPos.x + 'px', top: contextMenuPos.y + 'px' }">
          <div class="ctx-item" @click="handleToggleMode(); showContextMenu = false">{{ mode === 'top' ? '切换到底层' : '切换到置顶' }}</div>
          <div class="ctx-item" @click="handleTogglePinned(); showContextMenu = false">{{ pinned ? '解锁位置' : '锁定位置' }}</div>
          <div class="ctx-sep"></div>
          <div class="ctx-item" @click="showReminder = true; showContextMenu = false">设置提醒</div>
          <div class="ctx-item" @click="showContextMenu = false">复制便签</div>
          <div class="ctx-sep"></div>
          <div class="ctx-item ctx-danger" @click="handleHide(); showContextMenu = false">隐藏窗口</div>
        </div>
      </div>
    </Teleport>

    <ReminderDialog v-if="showReminder" :noteId="noteId" @close="showReminder = false" />
  </div>
  <div v-else class="note-status memo-window">便签数据为空</div>
</template>

<style scoped>
.note-window { display: flex; flex-direction: column; height: 100vh; }
.note-body { flex: 1; overflow: auto; }
.note-status { display: flex; flex-direction: column; align-items: center; justify-content: center; height: 100vh; color: #999; font-size: 15px; padding: 20px; text-align: center; }
.note-error { color: #e81123; }
.context-overlay { position: fixed; inset: 0; z-index: 199; }
.context-menu { position: fixed; background: #fff; border: 1px solid var(--color-hairline); border-radius: var(--rounded-md); padding: 4px; z-index: 200; min-width: 140px; font-size: 13px; box-shadow: 0 2px 12px rgba(0,0,0,0.15); }
.ctx-item { padding: 7px 14px; border-radius: 4px; cursor: pointer; }
.ctx-item:hover { background: var(--color-surface-soft); }
.ctx-danger { color: var(--color-accent-magenta); }
.ctx-sep { height: 1px; background: var(--color-hairline); margin: 4px 0; }
</style>
