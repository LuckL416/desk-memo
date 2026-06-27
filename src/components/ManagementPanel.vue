<script setup lang="ts">
import { ref } from 'vue'
import GroupSidebar from './GroupSidebar.vue'
import NoteList from './NoteList.vue'
import CalendarView from './CalendarView.vue'
import InlineInput from './InlineInput.vue'
import * as api from '../utils/tauri'
import type { Group, Note } from '../types'

const selectedGroup = ref<Group | null>(null)
const viewMode = ref<'list' | 'calendar'>('list')
const searchQuery = ref('')
const searchResults = ref<Note[]>([])
const isSearching = ref(false)
const showNewNoteInput = ref(false)

async function onSearch() {
  if (!searchQuery.value.trim()) { isSearching.value = false; return }
  isSearching.value = true
  searchResults.value = await api.searchNotes(searchQuery.value)
}

async function onNewNoteConfirm(title: string) {
  showNewNoteInput.value = false
  await api.createNote(selectedGroup.value?.id ?? null, 'text', title.trim() || null)
  location.reload()
}

function onNewNoteCancel() {
  showNewNoteInput.value = false
}

async function closePanel() {
  const { getCurrentWindow } = await import('@tauri-apps/api/window')
  await getCurrentWindow().hide()
}
</script>

<template>
  <div class="management-panel">
    <div class="header" data-tauri-drag-region>
      <span class="panel-title">📋 便签管理</span>
      <div class="view-toggle" data-tauri-drag-region="false">
        <span class="view-pill" :class="{ active: viewMode === 'list' }" @click="viewMode = 'list'">列表</span>
        <span class="view-pill" :class="{ active: viewMode === 'calendar' }" @click="viewMode = 'calendar'">日历</span>
      </div>
      <div class="search-wrap" data-tauri-drag-region="false">
        <input v-model="searchQuery" placeholder="🔍 搜索便签..." @input="onSearch" class="search-input" data-tauri-drag-region="false" />
      </div>
      <button class="new-btn" data-tauri-drag-region="false" @click="showNewNoteInput = true">+ 新建</button>
      <button class="close-panel-btn" data-tauri-drag-region="false" @click="closePanel" title="隐藏面板">✕</button>
    </div>
    <div class="body">
      <GroupSidebar @select="selectedGroup = $event" />
      <NoteList v-if="viewMode === 'list' && !isSearching" :group="selectedGroup" />
      <div v-else-if="isSearching" class="search-results">
        <div v-for="n in searchResults" :key="n.id" class="search-item">{{ n.title || '无标题' }}</div>
        <div v-if="searchResults.length === 0" style="color:#999;padding:20px;text-align:center;">无匹配结果</div>
      </div>
      <CalendarView v-else />
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
.view-pill { padding: 5px 14px; border-radius: var(--rounded-pill); font-size: 13px; cursor: pointer; }
.view-pill.active { background: var(--color-primary); color: var(--color-on-primary); }
.search-wrap { flex: 1; display: flex; justify-content: center; -webkit-app-region: no-drag; }
.search-input { border: 1px solid var(--color-hairline); border-radius: var(--rounded-md); padding: 8px 14px; font-size: 14px; width: 360px; background: var(--color-surface-soft); text-align: center; outline: none; }
.new-btn { background: var(--color-primary); color: var(--color-on-primary); border: none; padding: 8px 18px; border-radius: var(--rounded-pill); font-size: 14px; font-weight: 500; cursor: pointer; white-space: nowrap; -webkit-app-region: no-drag; }
.close-panel-btn { background: rgba(0,0,0,0.06); border: 1px solid rgba(0,0,0,0.1); cursor: pointer; font-size: 14px; padding: 4px 10px; border-radius: var(--rounded-pill); color: var(--color-ink); -webkit-app-region: no-drag; }
.close-panel-btn:hover { background: #ff3d8b; color: #fff; border-color: #ff3d8b; }
.body { display: flex; flex: 1; overflow: hidden; }
.search-results { flex: 1; padding: 12px 16px; }
.search-item { padding: 8px 12px; border-radius: 6px; margin-bottom: 4px; cursor: pointer; }
.search-item:hover { background: var(--color-surface-soft); }
</style>
