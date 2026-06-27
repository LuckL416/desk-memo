<script setup lang="ts">
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { WebviewWindow } from '@tauri-apps/api/webviewWindow'
import GroupSidebar from './GroupSidebar.vue'
import NoteList from './NoteList.vue'
import CalendarView from './CalendarView.vue'
import MemoWall from './MemoWall.vue'
import InlineInput from './InlineInput.vue'
import * as api from '../utils/tauri'
import type { Group, Note } from '../types'

const selectedGroup = ref<Group | null>(null)
const showTrash = ref(false)
const viewMode = ref<'list' | 'calendar' | 'wall'>('list')
const searchQuery = ref('')
const searchResults = ref<Note[]>([])
const isSearching = ref(false)
const showNewNoteInput = ref(false)
const listRefreshKey = ref(0)

async function onSearch() {
  if (!searchQuery.value.trim()) { isSearching.value = false; return }
  isSearching.value = true
  searchResults.value = await api.searchNotes(searchQuery.value)
}

function onSelectGroup(group: Group | null) {
  selectedGroup.value = group
  showTrash.value = false
}

function onSelectTrash() {
  selectedGroup.value = null
  showTrash.value = true
}

async function onNewNoteConfirm(title: string) {
  showNewNoteInput.value = false
  const note = await api.createNote(selectedGroup.value?.id ?? null, 'text', title.trim() || null)
  // 前端创建窗口（避免 Rust 命令线程池 build 死锁）
  new WebviewWindow(`note-${note.id}`, {
    url: 'index.html',
    title: '桌面便签',
    width: 320, height: 240,
    minWidth: 200, minHeight: 120,
    decorations: false, skipTaskbar: true, visible: true,
  })
  listRefreshKey.value++
}

function onNewNoteCancel() {
  showNewNoteInput.value = false
}

async function openCalendar() {
  const label = 'calendar-widget'
  const existing = await WebviewWindow.getByLabel(label)
  if (existing) { await existing.show(); await existing.setFocus(); return }
  new WebviewWindow(label, {
    url: 'index.html', title: '桌面日历',
    width: 560, height: 420, minWidth: 400, minHeight: 300,
    decorations: false, skipTaskbar: true, visible: true,
  })
}

async function openSearchResult(note: Note) {
  const label = `note-${note.id}`
  const existing = await WebviewWindow.getByLabel(label)
  if (existing) { await existing.show(); await existing.setFocus(); return }
  new WebviewWindow(label, { url: 'index.html', title: '桌面便签', width: 320, height: 240, minWidth: 200, minHeight: 120, decorations: false, skipTaskbar: true, visible: true })
}

async function closePanel() {
  await getCurrentWindow().hide()
}

const currentTheme = ref<'macaron' | 'hacker'>(
  document.documentElement.classList.contains('theme-hacker') ? 'hacker' : 'macaron'
)

async function setTheme(theme: 'macaron' | 'hacker') {
  currentTheme.value = theme
  document.documentElement.className = theme === 'hacker' ? 'theme-hacker' : 'theme-macaron'
  await invoke('set_setting', { key: 'theme', value: theme })
}
</script>

<template>
  <div class="management-panel">
    <div class="header" data-tauri-drag-region>
      <span class="panel-title">📋 便签管理</span>
      <div class="view-toggle" data-tauri-drag-region="false">
        <span class="view-pill" :class="{ active: viewMode === 'list' }" @click="viewMode = 'list'">列表</span>
        <span class="view-pill" :class="{ active: viewMode === 'calendar' }" @click="viewMode = 'calendar'">日历</span>
        <span class="view-pill" :class="{ active: viewMode === 'wall' }" @click="viewMode = 'wall'">便签墙</span>
      </div>
      <div class="theme-toggle" data-tauri-drag-region="false">
        <span class="view-pill" :class="{ active: currentTheme === 'macaron' }" @click="setTheme('macaron')">马卡龙</span>
        <span class="view-pill" :class="{ active: currentTheme === 'hacker' }" @click="setTheme('hacker')">极客黑客</span>
      </div>
      <div class="search-wrap" data-tauri-drag-region="false">
        <input v-model="searchQuery" placeholder="🔍 搜索便签..." @input="onSearch" class="search-input" data-tauri-drag-region="false" />
      </div>
      <button v-if="!showTrash" class="cal-btn" data-tauri-drag-region="false" @click="openCalendar">📅 桌面日历</button>
      <button v-if="!showTrash" class="new-btn" data-tauri-drag-region="false" @click="showNewNoteInput = true">+ 新建</button>
      <button class="close-panel-btn" data-tauri-drag-region="false" @click="closePanel" title="隐藏面板">✕</button>
    </div>
    <div class="body">
      <GroupSidebar @select="onSelectGroup" @select-trash="onSelectTrash" />
      <NoteList v-if="viewMode === 'list' && !isSearching" :group="selectedGroup" :trash="showTrash" :key="listRefreshKey" />
      <div v-else-if="isSearching" class="search-results">
        <div v-for="n in searchResults" :key="n.id" class="search-item" @click="openSearchResult(n)">
          <span>{{ n.title || '无标题' }}</span>
          <span style="font-size:11px;color:#999;margin-left:8px;">{{ n.type === 'todo' ? '待办' : n.type === 'timer' ? '计时' : '文本' }}</span>
        </div>
        <div v-if="searchResults.length === 0" style="color:#999;padding:20px;text-align:center;">无匹配结果</div>
      </div>
      <CalendarView v-else-if="viewMode === 'calendar'" />
      <MemoWall v-else-if="viewMode === 'wall'" :group="selectedGroup" />
    </div>
    <InlineInput
      v-if="showNewNoteInput"
      placeholder="便签标题（可选）"
      @confirm="onNewNoteConfirm"
      @cancel="onNewNoteCancel"
    />
  </div>
</template>

<style scoped>
.management-panel { display: flex; flex-direction: column; height: 100vh; background: var(--color-canvas); }
.header {
  display: flex; align-items: center; padding: 14px 20px;
  border-bottom: 1px solid var(--color-hairline); gap: 12px;
  cursor: grab; -webkit-app-region: drag; user-select: none;
}
.header:active { cursor: grabbing; }
.panel-title { font-weight: 700; font-size: 17px; }
.view-toggle { display: flex; gap: 4px; background: var(--color-surface-soft); padding: 3px; border-radius: var(--rounded-pill); -webkit-app-region: no-drag; }
.theme-toggle { display: flex; gap: 4px; background: var(--color-surface); padding: 3px; border-radius: var(--rounded-pill); -webkit-app-region: no-drag; }
.view-pill { padding: 5px 14px; border-radius: var(--rounded-pill); font-size: 13px; cursor: pointer; }
.view-pill.active { background: var(--color-primary); color: var(--color-on-primary); }
.search-wrap { flex: 1; display: flex; justify-content: center; -webkit-app-region: no-drag; }
.search-input { border: 1px solid var(--color-hairline); border-radius: var(--rounded-md); padding: 8px 14px; font-size: 14px; width: 360px; background: var(--color-surface-soft); text-align: center; outline: none; }
.cal-btn { background: var(--color-surface-soft); color: var(--color-ink); border: 1px solid var(--color-hairline); padding: 8px 14px; border-radius: var(--rounded-pill); font-size: 13px; cursor: pointer; white-space: nowrap; -webkit-app-region: no-drag; }
.cal-btn:hover { background: #e6e6e6; }
.new-btn { background: var(--color-primary); color: var(--color-on-primary); border: none; padding: 8px 18px; border-radius: var(--rounded-pill); font-size: 14px; font-weight: 500; cursor: pointer; white-space: nowrap; -webkit-app-region: no-drag; }
.close-panel-btn { background: rgba(0,0,0,0.06); border: 1px solid rgba(0,0,0,0.1); cursor: pointer; font-size: 14px; padding: 4px 10px; border-radius: var(--rounded-pill); color: var(--color-ink); -webkit-app-region: no-drag; }
.close-panel-btn:hover { background: #ff3d8b; color: #fff; border-color: #ff3d8b; }
.body { display: flex; flex: 1; overflow: hidden; }
.search-results { flex: 1; padding: 12px 16px; }
.search-item { padding: 8px 12px; border-radius: 6px; margin-bottom: 4px; cursor: pointer; }
.search-item:hover { background: var(--color-surface-soft); }
</style>
