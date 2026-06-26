<script setup lang="ts">
import { ref, onMounted } from 'vue'
import * as api from '../utils/tauri'
import type { Group } from '../types'
import InlineInput from './InlineInput.vue'

const emit = defineEmits<{ select: [group: Group | null] }>()
const groups = ref<Group[]>([])
const selectedId = ref<string | null>(null)

const showCreate = ref(false)
const showRename = ref(false)
const contextGroup = ref<Group | null>(null)
const contextPos = ref({ x: 0, y: 0 })
const showContext = ref(false)

onMounted(async () => { groups.value = await api.listGroups() })

function select(group: Group | null) {
  selectedId.value = group?.id ?? null
  emit('select', group)
}

async function onCreateConfirm(name: string) {
  showCreate.value = false
  if (name.trim()) {
    const g = await api.createGroup(name.trim())
    groups.value.push(g)
  }
}

function onCreateCancel() {
  showCreate.value = false
}

function onContextMenu(e: MouseEvent, g: Group) {
  e.preventDefault()
  contextGroup.value = g
  contextPos.value = { x: e.clientX, y: e.clientY }
  showContext.value = true
}

function hideContext() {
  showContext.value = false
  contextGroup.value = null
}

function startRename() {
  showRename.value = true
}

async function onRenameConfirm(name: string) {
  showRename.value = false
  hideContext()
  if (contextGroup.value && name.trim()) {
    await api.renameGroup(contextGroup.value.id, name.trim())
    // Refresh group list
    const idx = groups.value.findIndex(x => x.id === contextGroup.value!.id)
    if (idx !== -1) groups.value[idx].name = name.trim()
  }
}

function onRenameCancel() {
  showRename.value = false
}

async function deleteGroup() {
  if (!contextGroup.value) return
  if (confirm(`删除分组 "${contextGroup.value.name}"?\n该分组下的便签将移至未分组。`)) {
    await api.deleteGroup(contextGroup.value.id)
    groups.value = groups.value.filter(x => x.id !== contextGroup.value!.id)
    if (selectedId.value === contextGroup.value.id) { selectedId.value = null; emit('select', null) }
  }
  hideContext()
}
</script>

<template>
  <div class="sidebar" @click="hideContext">
    <div class="sidebar-label">分组</div>
    <div class="group-item" :class="{ active: selectedId === null }" @click="select(null)">全部</div>
    <div v-for="g in groups" :key="g.id" class="group-item" :class="{ active: selectedId === g.id }"
      @click="select(g)" @contextmenu="onContextMenu($event, g)">
      {{ g.name }}
    </div>
    <div class="sidebar-label" style="margin-top:8px;">🗑 回收站</div>
    <div class="add-group" @click="showCreate = true">+ 新建分组</div>

    <!-- Context Menu -->
    <Teleport to="body">
      <div v-if="showContext" class="context-overlay" @click="hideContext" @contextmenu.prevent="hideContext">
        <div class="context-menu" :style="{ left: contextPos.x + 'px', top: contextPos.y + 'px' }">
          <div class="context-item" @click="startRename()">重命名</div>
          <div class="context-item context-danger" @click="deleteGroup()">删除</div>
        </div>
      </div>
    </Teleport>

    <!-- Create Input -->
    <InlineInput
      v-if="showCreate"
      placeholder="新建分组名称"
      @confirm="onCreateConfirm"
      @cancel="onCreateCancel"
    />

    <!-- Rename Input -->
    <InlineInput
      v-if="showRename"
      placeholder="新分组名称"
      :initialValue="contextGroup?.name || ''"
      @confirm="onRenameConfirm"
      @cancel="onRenameCancel"
    />
  </div>
</template>

<style scoped>
.sidebar { width: 160px; padding: 16px; border-right: 1px solid var(--color-hairline); font-size: 13px; }
.sidebar-label { font-weight: 600; margin-bottom: 10px; font-size: 11px; color: #888; }
.group-item { padding: 6px 10px; border-radius: var(--rounded-pill); margin-bottom: 4px; cursor: pointer; }
.group-item:hover { background: var(--color-surface-soft); }
.group-item.active { background: var(--color-primary); color: var(--color-on-primary); }
.add-group { margin-top: 8px; font-size: 12px; color: #888; cursor: pointer; }

.context-overlay { position: fixed; inset: 0; z-index: 1000; }
.context-menu {
  position: fixed;
  background: #fff;
  border-radius: 10px;
  box-shadow: 0 4px 24px rgba(0,0,0,0.15);
  min-width: 120px;
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
