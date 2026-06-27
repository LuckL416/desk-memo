<script setup lang="ts">
import { MEMO_COLORS } from '../types'
import type { Note } from '../types'

const props = defineProps<{ note: Note }>()
const emit = defineEmits<{ open: [note: Note]; contextmenu: [e: MouseEvent, note: Note] }>()

const color = MEMO_COLORS.find(c => c.hex === props.note.bg_color) || MEMO_COLORS[Math.abs(hashCode(props.note.id)) % 5]
const rotateDeg = ((Math.abs(hashCode(props.note.id) * 7) % 40) - 20) / 10

function hashCode(s: string): number {
  let h = 0
  for (let i = 0; i < s.length; i++) { h = ((h << 5) - h) + s.charCodeAt(i); h |= 0 }
  return Math.abs(h)
}

function getPreview(note: Note): string {
  if (!note.content) return ''
  try {
    const doc = JSON.parse(note.content)
    const texts: string[] = []
    function walk(node: any) { if (node.text) texts.push(node.text); if (node.content) node.content.forEach(walk) }
    walk(doc)
    return texts.join(' ').slice(0, 100)
  } catch { return '' }
}

function getNoteIcon(type: string) { return { text: '📝', todo: '✅', timer: '⏱' }[type] || '📄' }

function formatTime(iso: string) {
  const d = new Date(iso); const now = new Date(); const diff = now.getTime() - d.getTime()
  if (diff < 60000) return '刚刚'
  if (diff < 3600000) return `${Math.floor(diff / 60000)}m`
  if (diff < 86400000) return `${Math.floor(diff / 3600000)}h`
  return d.toLocaleDateString()
}
</script>

<template>
  <div class="memo-card memo-window" :class="`memo-${color.name}`"
    :style="{ transform: `rotate(${rotateDeg}deg)` }"
    @click="emit('open', note)"
    @contextmenu.prevent="emit('contextmenu', $event, note)">
    <div class="memo-card-title">{{ getNoteIcon(note.type) }} {{ note.title || '无标题' }}</div>
    <div class="memo-card-preview" v-if="getPreview(note)">{{ getPreview(note) }}</div>
    <div class="memo-card-meta">
      <span>{{ note.type === 'todo' ? '待办' : note.type === 'timer' ? '计时' : '文本' }}</span>
      <span>{{ formatTime(note.updated_at) }}</span>
    </div>
  </div>
</template>

<style scoped>
.memo-card {
  break-inside: avoid;
  margin-bottom: 16px;
  padding: 24px 14px 14px;
  cursor: pointer;
  transition: transform 0.2s ease, box-shadow 0.2s ease;
}
.memo-card:hover {
  transform: rotate(0deg) translateY(-3px) !important;
  z-index: 10;
  box-shadow: 0 8px 22px rgba(0,0,0,0.16);
}
.memo-card-title { font-weight: 600; font-size: 14px; margin-bottom: 6px; }
.memo-card-preview { font-size: 11px; opacity: 0.75; line-height: 1.4; overflow: hidden; text-overflow: ellipsis; display: -webkit-box; -webkit-line-clamp: 3; -webkit-box-orient: vertical; }
.memo-card-meta { display: flex; justify-content: space-between; margin-top: 10px; font-size: 10px; opacity: 0.6; }
</style>
