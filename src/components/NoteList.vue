<script setup lang="ts">
import { ref, watch, onMounted } from 'vue'
import * as api from '../utils/tauri'
import type { Note, Group } from '../types'

const props = defineProps<{ group: Group | null }>()
const notes = ref<Note[]>([])
const typeFilter = ref('')

onMounted(() => loadNotes())
watch(() => props.group, () => loadNotes())
watch(typeFilter, () => loadNotes())

async function loadNotes() {
  notes.value = await api.listNotes(props.group?.id, typeFilter.value || undefined)
}

function openNote(note: Note) {
  // Will be wired up later; for now just a placeholder
  console.log('Open note:', note.id)
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
  <div class="note-list">
    <div class="type-filters">
      <span class="filter-pill" :class="{ active: typeFilter === '' }" @click="typeFilter = ''">全部类型</span>
      <span class="filter-pill" :class="{ active: typeFilter === 'text' }" @click="typeFilter = 'text'">📝 文本</span>
      <span class="filter-pill" :class="{ active: typeFilter === 'todo' }" @click="typeFilter = 'todo'">✅ 待办</span>
      <span class="filter-pill" :class="{ active: typeFilter === 'timer' }" @click="typeFilter = 'timer'">⏱ 计时</span>
    </div>
    <div v-for="note in notes" :key="note.id" class="note-item" @click="openNote(note)">
      <span>{{ getNoteIcon(note.type) }}</span>
      <span class="note-title">{{ note.title || '无标题' }}</span>
      <span class="note-meta">{{ note.type === 'todo' ? '待办' : note.type === 'timer' ? '计时' : '文本' }}</span>
      <span class="note-time">{{ formatTime(note.updated_at) }}</span>
    </div>
    <div v-if="notes.length === 0" style="text-align:center;color:#999;padding:40px;">暂无便签</div>
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
</style>
