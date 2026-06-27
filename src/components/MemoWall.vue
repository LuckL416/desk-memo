<script setup lang="ts">
import { ref, watch, onMounted } from 'vue'
import { WebviewWindow } from '@tauri-apps/api/webviewWindow'
import * as api from '../utils/tauri'
import type { Note, Group } from '../types'
import MemoCard from './MemoCard.vue'

const props = defineProps<{ group: Group | null }>()
const notes = ref<Note[]>([])
const contextNote = ref<Note | null>(null)
const contextPos = ref({ x: 0, y: 0 })
const showContext = ref(false)

onMounted(() => loadNotes())
watch(() => props.group, () => loadNotes())

async function loadNotes() {
  notes.value = await api.listNotes(props.group?.id, undefined)
}

async function openNote(note: Note) {
  const label = `note-${note.id}`
  const existing = await WebviewWindow.getByLabel(label)
  if (existing) { await existing.show(); await existing.setFocus(); return }
  new WebviewWindow(label, { url: 'index.html', title: '桌面便签', width: 320, height: 240, minWidth: 200, minHeight: 120, decorations: false, skipTaskbar: true, visible: true })
}

function onContextMenu(e: MouseEvent, note: Note) {
  e.preventDefault()
  contextNote.value = note
  contextPos.value = { x: e.clientX, y: e.clientY }
  showContext.value = true
}

function hideContext() { showContext.value = false; contextNote.value = null }

async function doDelete() {
  const note = contextNote.value
  if (!note) return
  hideContext()
  if (confirm(`删除 "${note.title || '无标题'}"？`)) {
    await api.deleteNote(note.id)
    await loadNotes()
  }
}

async function doRename() {
  const note = contextNote.value
  if (!note) return
  hideContext()
  const name = prompt('新标题:', note.title || '')
  if (name) { await api.updateNote(note.id, { title: name }); await loadNotes() }
}
</script>

<template>
  <div class="memo-wall" @click="hideContext">
    <MemoCard v-for="n in notes" :key="n.id" :note="n" @open="openNote" @contextmenu="onContextMenu" />
    <div v-if="notes.length === 0" style="text-align:center;color:#999;padding:60px;">暂无便签</div>

    <Teleport to="body">
      <div v-if="showContext" class="context-overlay" @click="hideContext" @contextmenu.prevent="hideContext">
        <div class="context-menu" :style="{ left: contextPos.x + 'px', top: contextPos.y + 'px' }">
          <div class="context-item" @click.stop="openNote(contextNote!); hideContext()">打开便签</div>
          <div class="context-item" @click.stop="doRename()">重命名</div>
          <div class="context-item context-danger" @click.stop="doDelete()">删除</div>
        </div>
      </div>
    </Teleport>
  </div>
</template>

<style scoped>
.memo-wall { column-count: 3; column-gap: 16px; padding: 16px; flex: 1; overflow-y: auto; }
@media (max-width: 960px) { .memo-wall { column-count: 2; } }

.context-overlay { position: fixed; inset: 0; z-index: 1000; }
.context-menu { position: fixed; background: #fff; border-radius: 12px; box-shadow: 0 4px 24px rgba(0,0,0,0.15); min-width: 140px; padding: 6px 0; z-index: 1001; }
.context-item { padding: 9px 18px; font-size: 14px; cursor: pointer; white-space: nowrap; }
.context-item:hover { background: var(--color-surface-soft); }
.context-item:first-child { border-radius: 8px 8px 0 0; }
.context-item:last-child { border-radius: 0 0 8px 8px; }
.context-danger { color: #ff3d8b; }
</style>
