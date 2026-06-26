<script setup lang="ts">
import { ref, watch, onMounted } from 'vue'
import { WebviewWindow } from '@tauri-apps/api/webviewWindow'
import * as api from '../utils/tauri'
import type { Note, Group } from '../types'
import InlineInput from './InlineInput.vue'

const props = defineProps<{ group: Group | null }>()
const notes = ref<Note[]>([])
const typeFilter = ref('')

const contextNote = ref<Note | null>(null)
const contextPos = ref({ x: 0, y: 0 })
const showContext = ref(false)
const showRename = ref(false)

onMounted(() => loadNotes())
watch(() => props.group, () => loadNotes())
watch(typeFilter, () => loadNotes())

async function loadNotes() {
  notes.value = await api.listNotes(props.group?.id, typeFilter.value || undefined)
}

async function openNote(note: Note) {
  const label = `note-${note.id}`
  const existing = WebviewWindow.getByLabel(label)
  if (existing) {
    await existing.show()
    await existing.setFocus()
    return
  }
  const bg = note.bg_color || (note.type === 'timer' ? '#1f1d3d' : '#f4ecd6')
  const w = new WebviewWindow(label, {
    url: 'index.html',
    title: '桌面便签',
    width: 320,
    height: 240,
    decorations: false,
    skipTaskbar: true,
    visible: true,
  })
  w.once('tauri://created', () => {
    const js = `window.__noteId='${note.id}';window.__noteType='${note.type}';window.__noteBg='${bg}';window.__noteTitle='${note.title || ''}';`
    w.eval(js)
  })
}

function onContextMenu(e: MouseEvent, note: Note) {
  e.preventDefault()
  contextNote.value = note
  contextPos.value = { x: e.clientX, y: e.clientY }
  showContext.value = true
}

function hideContext() {
  showContext.value = false
  contextNote.value = null
}

async function doDelete() {
  if (!contextNote.value) return
  if (confirm(`确定删除便签 "${contextNote.value.title || '无标题'}"？`)) {
    await api.deleteNote(contextNote.value.id)
    hideContext()
    await loadNotes()
  }
}

function doRename() {
  showRename.value = true
}

async function onRenameConfirm(value: string) {
  if (contextNote.value && value.trim()) {
    await api.updateNote(contextNote.value.id, { title: value.trim() })
  }
  showRename.value = false
  hideContext()
  await loadNotes()
}

function onRenameCancel() {
  showRename.value = false
}

function formatTime(iso: string) {
  const d = new Date(iso)
  const now = new Date()
  const diff = now.getTime() - d.getTime()
  if (diff < 60000) return '刚刚'
  if (diff < 3600000) return `${Math.floor(diff / 60000)}分钟前`
  if (diff < 86400000) return `${Math.floor(diff / 3600000)}小时前`
  return d.toLocaleDateString()
}

function getNoteIcon(type: string) { return { text: '📝', todo: '✅', timer: '⏱' }[type] || '📄' }
</script>

<template>
  <div class="note-list" @click="hideContext">
    <div class="type-filters">
      <span class="filter-pill" :class="{ active: typeFilter === '' }" @click="typeFilter = ''">全部类型</span>
      <span class="filter-pill" :class="{ active: typeFilter === 'text' }" @click="typeFilter = 'text'">📝 文本</span>
      <span class="filter-pill" :class="{ active: typeFilter === 'todo' }" @click="typeFilter = 'todo'">✅ 待办</span>
      <span class="filter-pill" :class="{ active: typeFilter === 'timer' }" @click="typeFilter = 'timer'">⏱ 计时</span>
    </div>
    <div v-for="note in notes" :key="note.id" class="note-item" @click="openNote(note)" @contextmenu="onContextMenu($event, note)">
      <span>{{ getNoteIcon(note.type) }}</span>
      <span class="note-title">{{ note.title || '无标题' }}</span>
      <span class="note-meta">{{ note.type === 'todo' ? '待办' : note.type === 'timer' ? '计时' : '文本' }}</span>
      <span class="note-time">{{ formatTime(note.updated_at) }}</span>
    </div>
    <div v-if="notes.length === 0" style="text-align:center;color:#999;padding:40px;">暂无便签</div>

    <!-- Context Menu -->
    <Teleport to="body">
      <div v-if="showContext" class="context-overlay" @click="hideContext" @contextmenu.prevent="hideContext">
        <div class="context-menu" :style="{ left: contextPos.x + 'px', top: contextPos.y + 'px' }">
          <div class="context-item" @click="openNote(contextNote!); hideContext()">打开便签</div>
          <div class="context-item" @click="doRename()">重命名</div>
          <div class="context-item context-danger" @click="doDelete()">删除</div>
        </div>
      </div>
    </Teleport>

    <!-- Rename Input -->
    <InlineInput
      v-if="showRename"
      placeholder="输入新标题..."
      :initialValue="contextNote?.title || ''"
      @confirm="onRenameConfirm"
      @cancel="onRenameCancel"
    />
  </div>
</template>

<style scoped>
.note-list { flex: 1; padding: 12px 16px; }
.type-filters { display: flex; gap: 8px; margin-bottom: 12px; font-size: 12px; }
.filter-pill { padding: 3px 10px; border-radius: var(--rounded-pill); cursor: pointer; border: 1px solid var(--color-hairline); }
.filter-pill.active { background: var(--color-primary); color: var(--color-on-primary); border-color: var(--color-primary); }
.note-item { display: flex; align-items: center; gap: 10px; padding: 10px 14px; border: 1px solid #f1f1f1; border-radius: var(--rounded-md); margin-bottom: 4px; cursor: pointer; border-left: 3px solid var(--color-block-cream); }
.note-item:hover { background: var(--color-surface-soft); }
.note-title { font-weight: 500; flex: 1; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
.note-meta { font-size: 11px; color: #888; }
.note-time { font-size: 11px; color: #aaa; }

.context-overlay { position: fixed; inset: 0; z-index: 1000; }
.context-menu {
  position: fixed;
  background: #fff;
  border-radius: 10px;
  box-shadow: 0 4px 24px rgba(0,0,0,0.15);
  min-width: 140px;
  padding: 6px 0;
  z-index: 1001;
}
.context-item {
  padding: 9px 18px;
  font-size: 14px;
  cursor: pointer;
  white-space: nowrap;
}
.context-item:hover { background: var(--color-surface-soft); }
.context-danger { color: #ff3d8b; }
</style>
