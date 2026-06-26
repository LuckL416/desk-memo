<script setup lang="ts">
import { ref, onMounted } from 'vue'
import NoteChrome from './NoteChrome.vue'
import TextNoteEditor from './TextNoteEditor.vue'
import TodoNoteEditor from './TodoNoteEditor.vue'
import TimerNoteView from './TimerNoteView.vue'
import * as api from '../utils/tauri'
import type { Note } from '../types'

const note = ref<Note | null>(null)
const mode = ref<'desktop' | 'top'>('desktop')
const pinned = ref(false)
const opacity = ref(1.0)
const showContextMenu = ref(false)
const contextMenuPos = ref({ x: 0, y: 0 })

const w = window as any
const noteId = w.__noteId as string
const noteType = w.__noteType as string
const noteBg = (w.__noteBg as string) || '#f4ecd6'

onMounted(async () => {
  document.body.style.background = noteBg
  note.value = await api.getNote(noteId)
  const ws = await api.getWindowState(noteId)
  if (ws) {
    mode.value = ws.mode
    pinned.value = ws.pinned
    opacity.value = ws.opacity
  }
})

async function handleClose() {
  await api.saveWindowState({
    note_id: noteId, x: 0, y: 0, width: 320, height: 240,
    opacity: opacity.value, pinned: pinned.value, mode: mode.value, is_visible: false,
  })
  const { getCurrentWindow } = await import('@tauri-apps/api/window')
  await getCurrentWindow().hide()
}

async function handleToggleMode() {
  mode.value = mode.value === 'desktop' ? 'top' : 'desktop'
  const { getCurrentWindow } = await import('@tauri-apps/api/window')
  await getCurrentWindow().setAlwaysOnTop(mode.value === 'top')
}

async function handleTogglePinned() {
  pinned.value = !pinned.value
  const { getCurrentWindow } = await import('@tauri-apps/api/window')
  await getCurrentWindow().setResizable(!pinned.value)
}

async function handleUpdateOpacity(val: number) {
  opacity.value = val
  document.documentElement.style.opacity = String(val)
}

function handleContextMenu(event: MouseEvent) {
  event.preventDefault()
  contextMenuPos.value = { x: event.clientX, y: event.clientY }
  showContextMenu.value = true
}

let saveTimer: ReturnType<typeof setTimeout> | null = null
function debouncedSave(data: Partial<Note>) {
  if (saveTimer) clearTimeout(saveTimer)
  saveTimer = setTimeout(() => {
    api.updateNote(noteId, data)
  }, 500)
}
</script>

<template>
  <div v-if="note" class="note-window" @contextmenu="handleContextMenu">
    <NoteChrome
      :title="note.title || ''" :mode="mode" :pinned="pinned" :opacity="opacity"
      @close="handleClose" @toggle-mode="handleToggleMode"
      @toggle-pinned="handleTogglePinned" @update-opacity="handleUpdateOpacity"
      @show-menu="handleContextMenu"
    />
    <div class="note-body">
      <TextNoteEditor v-if="note.type === 'text'" :note="note" @update="debouncedSave" />
      <TodoNoteEditor v-else-if="note.type === 'todo'" :note="note" @update="debouncedSave" />
      <TimerNoteView v-else-if="note.type === 'timer'" :note="note" />
    </div>
    <div v-if="showContextMenu" class="context-menu" :style="{ left: contextMenuPos.x + 'px', top: contextMenuPos.y + 'px' }">
      <div class="ctx-item" @click="handleToggleMode">切换置顶/置底</div>
      <div class="ctx-item" @click="handleTogglePinned">锁定/解锁</div>
      <div class="ctx-sep"></div>
      <div class="ctx-item">设置提醒</div>
      <div class="ctx-item">复制便签</div>
      <div class="ctx-sep"></div>
      <div class="ctx-item ctx-danger" @click="handleClose">隐藏便签</div>
    </div>
    <div v-if="showContextMenu" class="context-overlay" @click="showContextMenu = false"></div>
  </div>
</template>

<style scoped>
.note-window { display: flex; flex-direction: column; height: 100vh; }
.note-body { flex: 1; overflow: auto; }
.context-menu {
  position: fixed; background: #fff; border: 1px solid var(--color-hairline);
  border-radius: var(--rounded-md); padding: 4px; z-index: 200;
  min-width: 140px; font-size: 12px; box-shadow: 0 2px 12px rgba(0,0,0,0.1);
}
.ctx-item { padding: 6px 12px; border-radius: 4px; cursor: pointer; }
.ctx-item:hover { background: var(--color-surface-soft); }
.ctx-danger { color: var(--color-accent-magenta); }
.ctx-sep { height: 1px; background: var(--color-hairline); margin: 4px 0; }
.context-overlay { position: fixed; inset: 0; z-index: 199; }
</style>
