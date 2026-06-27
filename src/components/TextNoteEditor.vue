<script setup lang="ts">
import { ref, watch, onBeforeUnmount } from 'vue'
import { useEditor, EditorContent } from '@tiptap/vue-3'
import StarterKit from '@tiptap/starter-kit'
import { TextStyle } from '@tiptap/extension-text-style'
import { Color } from '@tiptap/extension-color'
import Placeholder from '@tiptap/extension-placeholder'
import type { Note } from '../types'
import { MEMO_COLORS } from '../types'

const props = defineProps<{ note: Note }>()
const emit = defineEmits<{ update: [data: Partial<Note>] }>()

const currentBg = ref(props.note.bg_color || '#ffffff')
const currentTextColor = ref(props.note.default_text_color || '#000000')
const currentFontSize = ref(String(props.note.default_font_size || 16))
const showColorPicker = ref(false)
const showBgPicker = ref(false)

function getTextShadow(hex: string): string {
  const r = parseInt(hex.slice(1,3), 16)
  const g = parseInt(hex.slice(3,5), 16)
  const b = parseInt(hex.slice(5,7), 16)
  const luminance = (0.299 * r + 0.587 * g + 0.114 * b) / 255
  return luminance > 0.5 ? '0 0 2px rgba(0,0,0,0.4)' : '0 0 2px rgba(255,255,255,0.5)'
}

const editor = useEditor({
  content: props.note.content ? JSON.parse(props.note.content) : '',
  extensions: [
    StarterKit.configure({ heading: false, codeBlock: false, blockquote: false, horizontalRule: false, dropcursor: false, gapcursor: false }),
    TextStyle, Color,
    Placeholder.configure({ placeholder: '输入内容...' }),
  ],
  onUpdate: ({ editor }) => {
    emit('update', { content: JSON.stringify(editor.getJSON()) })
  },
  editorProps: {
    attributes: {
      style: `font-size: ${currentFontSize.value}px; color: ${currentTextColor.value}; text-shadow: ${getTextShadow(currentTextColor.value)};`,
    },
  },
})

function updateTextColor(hex: string) {
  currentTextColor.value = hex
  editor.value?.chain().setColor(hex).run()
  emit('update', { default_text_color: hex })
  showColorPicker.value = false
}

function updateBgColor(hex: string) {
  currentBg.value = hex
  document.body.style.background = hex
  emit('update', { bg_color: hex })
  showBgPicker.value = false
}

function updateFontSize(size: string) {
  currentFontSize.value = size
  emit('update', { default_font_size: Number(size) })
}

watch(() => props.note.bg_color, (val) => {
  if (val) { currentBg.value = val; document.body.style.background = val }
})

onBeforeUnmount(() => { editor.value?.destroy() })
</script>

<template>
  <div class="text-editor">
    <div class="toolbar">
      <button @click="editor?.chain().toggleBold().run()" :class="{ active: editor?.isActive('bold') }"><b>B</b></button>
      <button @click="editor?.chain().toggleItalic().run()" :class="{ active: editor?.isActive('italic') }"><i>I</i></button>
      <span class="sep">|</span>
      <div class="dropdown-wrap">
        <button @click="showColorPicker = !showColorPicker" title="文字颜色">🎨</button>
        <div v-if="showColorPicker" class="color-dropdown">
          <div v-for="c in ['#000000','#ffffff','#ff3d8b','#1ea64a','#1f1d3d']" :key="c"
            class="color-swatch" :style="{ background: c }" @click="updateTextColor(c)"></div>
        </div>
      </div>
      <select :value="currentFontSize" @change="updateFontSize(($event.target as HTMLSelectElement).value)">
        <option v-for="s in ['12','14','16','18','20','24','28']" :key="s" :value="s">{{ s }}px</option>
      </select>
      <span class="sep">|</span>
      <button @click="editor?.chain().toggleBulletList().run()" :class="{ active: editor?.isActive('bulletList') }">•≡</button>
      <button @click="editor?.chain().toggleOrderedList().run()" :class="{ active: editor?.isActive('orderedList') }">1.≡</button>
      <span class="sep">|</span>
      <div class="dropdown-wrap">
        <button @click="showBgPicker = !showBgPicker" title="背景色">🎨 背景</button>
        <div v-if="showBgPicker" class="color-dropdown">
          <div v-for="c in MEMO_COLORS" :key="c.hex"
            class="color-swatch" :style="{ background: c.hex, border: currentBg === c.hex ? '2px solid #000' : '1px solid #ccc' }"
            :title="c.label" @click="updateBgColor(c.hex)"></div>
        </div>
      </div>
    </div>
    <EditorContent :editor="editor" class="editor-content" />
  </div>
</template>

<style scoped>
.text-editor { display: flex; flex-direction: column; height: 100%; }
.toolbar { display: flex; gap: 4px; padding: 4px 12px; align-items: center; background: rgba(255,255,255,0.4); border-bottom: 1px solid rgba(0,0,0,0.06); flex-wrap: wrap; }
.toolbar button { background: none; border: none; cursor: pointer; padding: 2px 6px; border-radius: 4px; font-size: 13px; color: var(--color-ink); }
.toolbar button:hover, .toolbar button.active { background: rgba(0,0,0,0.08); }
.toolbar select { background: none; border: 1px solid rgba(0,0,0,0.15); border-radius: 4px; font-size: 12px; padding: 1px 4px; }
.sep { color: #ccc; font-size: 14px; }
.editor-content { flex: 1; padding: 14px 16px; min-height: 100px; }
.editor-content :deep(.ProseMirror) { outline: none; min-height: 100px; line-height: 1.6; }
.editor-content :deep(.ProseMirror p.is-editor-empty:first-child::before) { content: attr(data-placeholder); color: #999; float: left; pointer-events: none; height: 0; }
.dropdown-wrap { position: relative; }
.color-dropdown { position: absolute; top: 100%; left: 0; display: flex; gap: 4px; background: #fff; border: 1px solid var(--color-hairline); border-radius: var(--rounded-md); padding: 6px 8px; z-index: 100; box-shadow: 0 2px 8px rgba(0,0,0,0.08); flex-wrap: wrap; width: 140px; }
.color-swatch { width: 20px; height: 20px; border-radius: 50%; cursor: pointer; flex-shrink: 0; }
</style>
