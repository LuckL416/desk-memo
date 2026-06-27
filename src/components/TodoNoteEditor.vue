<script setup lang="ts">
import { computed } from 'vue'
import { useEditor, EditorContent } from '@tiptap/vue-3'
import StarterKit from '@tiptap/starter-kit'
import TaskList from '@tiptap/extension-task-list'
import TaskItem from '@tiptap/extension-task-item'
import Placeholder from '@tiptap/extension-placeholder'
import type { Note } from '../types'

const props = defineProps<{ note: Note }>()
const emit = defineEmits<{ update: [data: Partial<Note>] }>()

const editor = useEditor({
  content: props.note.content ? JSON.parse(props.note.content) : { type: 'doc', content: [{ type: 'taskList', content: [] }] },
  extensions: [
    StarterKit.configure({ heading: false, codeBlock: false, blockquote: false, horizontalRule: false, dropcursor: false, gapcursor: false, bulletList: false, orderedList: false }),
    TaskList, TaskItem.configure({ nested: false }),
    Placeholder.configure({ placeholder: '添加待办项...' }),
  ],
  onUpdate: ({ editor }) => {
    emit('update', { content: JSON.stringify(editor.getJSON()) })
  },
})

function getTaskStats() {
  if (!editor.value) return { done: 0, total: 0 }
  const items: { attrs?: { checked?: boolean } }[] = []
  editor.value.state.doc.descendants((node: { type: { name: string }; attrs?: { checked?: boolean } }) => {
    if (node.type.name === 'taskItem') items.push(node)
  })
  let done = 0
  for (const item of items) {
    if (item.attrs?.checked) done++
  }
  return { done, total: items.length }
}

const stats = computed(() => getTaskStats())
const progressPercent = computed(() => stats.value.total === 0 ? 0 : Math.round((stats.value.done / stats.value.total) * 100))

function addTaskItem() {
  editor.value?.chain().focus().insertContent({ type: 'taskItem', content: [{ type: 'paragraph' }] }).run()
}
</script>

<template>
  <div class="todo-editor">
    <div class="toolbar">
      <button @click="addTaskItem">+ 待办项</button>
    </div>
    <EditorContent :editor="editor" class="editor-content" />
    <div class="progress-bar" v-if="stats.total > 0">
      <span class="progress-text">{{ stats.done }}/{{ stats.total }} 已完成</span>
      <div class="progress-track">
        <div class="progress-fill" :style="{ width: progressPercent + '%' }"></div>
      </div>
      <span>{{ progressPercent }}%</span>
    </div>
  </div>
</template>

<style scoped>
.todo-editor { display: flex; flex-direction: column; height: 100%; }
.toolbar { padding: 4px 12px; background: rgba(255,255,255,0.3); border-bottom: 1px solid rgba(0,0,0,0.06); }
.toolbar button { background: var(--color-primary); color: var(--color-on-primary); border: none; padding: 3px 10px; border-radius: var(--rounded-pill); font-size: 12px; cursor: pointer; }
.editor-content { flex: 1; padding: 14px 16px; min-height: 100px; }
.editor-content :deep(.ProseMirror) { outline: none; min-height: 80px; }
.editor-content :deep(.ProseMirror ul[data-type="taskList"]) { list-style: none; padding: 0; }
.editor-content :deep(.ProseMirror li[data-type="taskItem"]) { display: flex; align-items: flex-start; gap: 8px; margin-bottom: 4px; }
.editor-content :deep(.ProseMirror li[data-type="taskItem"] label) { margin-top: 2px; }
.editor-content :deep(.ProseMirror li[data-type="taskItem"] input[type="checkbox"]) { width: 18px; height: 18px; border: 2px solid var(--color-ink); border-radius: 4px; cursor: pointer; accent-color: var(--color-success); }
.progress-bar { display: flex; align-items: center; gap: 10px; padding: 10px 16px; border-top: 1px solid rgba(0,0,0,0.08); font-size: 12px; color: var(--color-ink); font-weight: 600; }
.progress-track { flex: 1; height: 6px; background: rgba(0,0,0,0.12); border-radius: 3px; overflow: hidden; }
.progress-fill { height: 100%; background: var(--color-success); border-radius: 3px; transition: width 0.3s; }
</style>
