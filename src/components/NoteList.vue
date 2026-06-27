<script setup lang="ts">
import { ref, watch, onMounted } from 'vue'
import { WebviewWindow } from '@tauri-apps/api/webviewWindow'
import * as api from '../utils/tauri'
import type { Note, Group } from '../types'
import InlineInput from './InlineInput.vue'
import ConfirmDialog from './ConfirmDialog.vue'

const props = defineProps<{ group: Group | null; trash?: boolean }>()
const notes = ref<Note[]>([])
const typeFilter = ref('')

const contextNote = ref<Note | null>(null)
const renameTarget = ref<Note | null>(null)
const contextPos = ref({ x: 0, y: 0 })
const showContext = ref(false)
const showRename = ref(false)
const showConfirm = ref(false)
const confirmMessage = ref('')
let confirmAction: (() => void) | null = null

function askConfirm(msg: string, action: () => void) {
  confirmMessage.value = msg
  confirmAction = action
  showConfirm.value = true
}

function onConfirm() {
  showConfirm.value = false
  confirmAction?.()
  confirmAction = null
}

function onConfirmCancel() {
  showConfirm.value = false
  confirmAction = null
}

onMounted(() => loadNotes())
watch(() => props.group, () => loadNotes())
watch(() => props.trash, () => loadNotes())
watch(typeFilter, () => loadNotes())

async function loadNotes() {
  if (props.trash) {
    notes.value = await api.listNotes(undefined, typeFilter.value || undefined, true)
    // 只显示已删除的
    notes.value = notes.value.filter(n => n.deleted_at)
  } else {
    notes.value = await api.listNotes(props.group?.id, typeFilter.value || undefined)
  }
}

// 前端直接创建/显示窗口 — 避免 Rust 命令的线程池死锁
async function openNote(note: Note) {
  const label = `note-${note.id}`

  // 先查是否已有窗口
  const existing = await WebviewWindow.getByLabel(label)
  if (existing) {
    await existing.show()
    await existing.setFocus()
    return
  }

  // 创建新窗口
  new WebviewWindow(label, {
    url: 'index.html',
    title: '桌面便签',
    width: 320,
    height: 240,
    minWidth: 200,
    minHeight: 120,
    decorations: false,
    skipTaskbar: true,
    visible: true,
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

function doDelete() {
  if (!contextNote.value) return
  const note = contextNote.value
  hideContext()
  askConfirm(`确定删除便签 "${note.title || '无标题'}"？`, async () => {
    await api.deleteNote(note.id)
    await loadNotes()
  })
}

function doRename() {
  renameTarget.value = contextNote.value
  showRename.value = true
}

async function onRenameConfirm(value: string) {
  const note = renameTarget.value
  if (note && value.trim()) {
    await api.updateNote(note.id, { title: value.trim() })
  }
  showRename.value = false
  renameTarget.value = null
  hideContext()
  await loadNotes()
}

function onRenameCancel() {
  showRename.value = false
}

async function doRestore() {
  if (!contextNote.value) return
  await api.restoreNote(contextNote.value.id)
  hideContext()
  await loadNotes()
}

function doPermanentDelete() {
  if (!contextNote.value) return
  const note = contextNote.value
  hideContext()
  askConfirm('永久删除此便签？此操作不可恢复。', async () => {
    await api.permanentlyDeleteNote(note.id)
    await loadNotes()
  })
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

function getContentPreview(note: Note): string {
  if (!note.content) return ''
  try {
    const doc = JSON.parse(note.content)
    const texts: string[] = []
    function walk(node: any) {
      if (node.text) texts.push(node.text)
      if (node.content) node.content.forEach(walk)
    }
    walk(doc)
    return texts.join(' ').slice(0, 120)
  } catch { return '' }
}

function getTodoStats(note: Note): string {
  if (!note.content) return ''
  try {
    const doc = JSON.parse(note.content)
    let done = 0, total = 0
    function walk(node: any) {
      if (node.type === 'taskItem') {
        total++
        if (node.attrs?.checked) done++
      }
      if (node.content) node.content.forEach(walk)
    }
    walk(doc)
    if (total === 0) return ''
    return `${done}/${total} 已完成`
  } catch { return '' }
}
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
      <div class="note-top">
        <span class="note-icon">{{ getNoteIcon(note.type) }}</span>
        <span class="note-title">{{ note.title || '无标题' }}</span>
        <span class="note-meta">{{ note.type === 'todo' ? '待办' : note.type === 'timer' ? '计时' : '文本' }}</span>
        <span class="note-time">{{ formatTime(note.updated_at) }}</span>
      </div>
      <div class="note-preview" v-if="getContentPreview(note)">{{ getContentPreview(note) }}</div>
      <div v-if="note.type === 'todo'" class="note-todo-stats">
        {{ getTodoStats(note) }}
      </div>
    </div>
    <div v-if="notes.length === 0" style="text-align:center;color:#999;padding:40px;">暂无便签</div>

    <Teleport to="body">
      <div v-if="showContext" class="context-overlay" @click="hideContext" @contextmenu.prevent="hideContext">
        <div class="context-menu" :style="{ left: contextPos.x + 'px', top: contextPos.y + 'px' }">
          <template v-if="trash">
            <div class="context-item" @click.stop="doRestore()">恢复便签</div>
            <div class="context-item context-danger" @click.stop="doPermanentDelete()">永久删除</div>
          </template>
          <template v-else>
            <div class="context-item" @click.stop="openNote(contextNote!); hideContext()">打开便签</div>
            <div class="context-item" @click.stop="doRename()">重命名</div>
            <div class="context-item context-danger" @click.stop="doDelete()">删除</div>
          </template>
        </div>
      </div>
    </Teleport>

    <InlineInput
      v-if="showRename"
      placeholder="输入新标题..."
      :initialValue="renameTarget?.title || ''"
      @confirm="onRenameConfirm"
      @cancel="onRenameCancel"
    />

    <ConfirmDialog
      v-if="showConfirm"
      :message="confirmMessage"
      @confirm="onConfirm"
      @cancel="onConfirmCancel"
    />
  </div>
</template>

<style scoped>
.note-list { flex: 1; padding: 12px 16px; }
.type-filters { display: flex; gap: 8px; margin-bottom: 12px; font-size: 12px; }
.filter-pill { padding: 3px 10px; border-radius: var(--rounded-pill); cursor: pointer; border: 1px solid var(--color-hairline); }
.filter-pill.active { background: var(--color-primary); color: var(--color-on-primary); border-color: var(--color-primary); }
.note-item { display: flex; flex-direction: column; padding: 12px 14px; border: 1px solid #f1f1f1; border-radius: var(--rounded-md); margin-bottom: 6px; cursor: pointer; border-left: 3px solid var(--color-block-cream); }
.note-item:hover { background: var(--color-surface-soft); }
.note-top { display: flex; align-items: center; gap: 8px; }
.note-icon { flex-shrink: 0; }
.note-title { font-weight: 600; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; max-width: 200px; }
.note-meta { font-size: 10px; color: #888; background: var(--color-surface-soft); padding: 1px 6px; border-radius: 4px; flex-shrink: 0; }
.note-time { font-size: 11px; color: #aaa; margin-left: auto; flex-shrink: 0; }
.note-preview { font-size: 12px; color: #666; margin-top: 6px; line-height: 1.4; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
.note-todo-stats { font-size: 11px; color: var(--color-success); margin-top: 4px; font-weight: 500; }

.context-overlay { position: fixed; inset: 0; z-index: 1000; }
.context-menu { position: fixed; background: #fff; border-radius: 10px; box-shadow: 0 4px 24px rgba(0,0,0,0.15); min-width: 140px; padding: 6px 0; z-index: 1001; }
.context-item { padding: 9px 18px; font-size: 14px; cursor: pointer; white-space: nowrap; }
.context-item:hover { background: var(--color-surface-soft); }
.context-danger { color: #ff3d8b; }
</style>
