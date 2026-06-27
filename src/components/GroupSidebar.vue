<script setup lang="ts">
import { ref, onMounted } from 'vue'
import * as api from '../utils/tauri'
import type { Group } from '../types'
import InlineInput from './InlineInput.vue'
import ConfirmDialog from './ConfirmDialog.vue'

const emit = defineEmits<{
  select: [group: Group | null]
  selectTrash: []
}>()
const groups = ref<Group[]>([])
const selectedId = ref<string | null>(null)
const trashSelected = ref(false)

const showCreate = ref(false)
const showRename = ref(false)
const showConfirm = ref(false)
const confirmMessage = ref('')
let confirmAction: (() => void) | null = null

function askConfirm(msg: string, action: () => void) {
  confirmMessage.value = msg
  confirmAction = action
  showConfirm.value = true
}
function onConfirm() { showConfirm.value = false; confirmAction?.(); confirmAction = null }
function onConfirmCancel() { showConfirm.value = false; confirmAction = null }
const contextGroup = ref<Group | null>(null)
const renameTarget = ref<Group | null>(null)
const contextPos = ref({ x: 0, y: 0 })
const showContext = ref(false)

onMounted(async () => { groups.value = await api.listGroups() })

function select(group: Group | null) {
  selectedId.value = group?.id ?? null
  trashSelected.value = false
  emit('select', group)
}

function selectTrash() {
  selectedId.value = null
  trashSelected.value = true
  emit('selectTrash')
}

async function onCreateConfirm(name: string) {
  showCreate.value = false
  if (name.trim()) {
    const g = await api.createGroup(name.trim())
    groups.value.push(g)
  }
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
  renameTarget.value = contextGroup.value
  showRename.value = true
}

async function onRenameConfirm(name: string) {
  showRename.value = false
  hideContext()
  const target = renameTarget.value
  if (target && name.trim()) {
    await api.renameGroup(target.id, name.trim())
    const idx = groups.value.findIndex(x => x.id === target.id)
    if (idx !== -1) groups.value[idx].name = name.trim()
  }
  renameTarget.value = null
}

function onRenameCancel() {
  showRename.value = false
}

function deleteGroup() {
  const target = contextGroup.value
  hideContext()
  if (!target) return
  askConfirm(`删除分组 "${target.name}"？\n该分组下的便签将移至未分组。`, async () => {
    await api.deleteGroup(target.id)
    groups.value = groups.value.filter(x => x.id !== target.id)
    if (selectedId.value === target.id) { selectedId.value = null; emit('select', null) }
  })
}
</script>

<template>
  <div class="sidebar" @click="hideContext">
    <div class="sidebar-label">分组</div>
    <div class="group-item" :class="{ active: selectedId === null && !trashSelected }" @click="select(null)">全部</div>
    <div v-for="g in groups" :key="g.id" class="group-item" :class="{ active: selectedId === g.id }"
      @click="select(g)" @contextmenu="onContextMenu($event, g)">
      {{ g.name }}
    </div>
    <div class="sidebar-label" style="margin-top:8px;">🗑 回收站</div>
    <div class="group-item" :class="{ active: trashSelected }" @click="selectTrash()">已删除便签</div>
    <div v-if="!trashSelected" class="add-group" @click="showCreate = true">+ 新建分组</div>

    <Teleport to="body">
      <div v-if="showContext" class="context-overlay" @click="hideContext" @contextmenu.prevent="hideContext">
        <div class="context-menu" :style="{ left: contextPos.x + 'px', top: contextPos.y + 'px' }">
          <div class="context-item" @click="startRename()">重命名</div>
          <div class="context-item context-danger" @click="deleteGroup()">删除分组</div>
        </div>
      </div>
    </Teleport>

    <InlineInput
      v-if="showCreate"
      placeholder="新建分组名称"
      @confirm="onCreateConfirm"
      @cancel="() => showCreate = false"
    />

    <InlineInput
      v-if="showRename"
      placeholder="新分组名称"
      :initialValue="renameTarget?.name || ''"
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
.sidebar { width: 160px; padding: 16px; border-right: 1px solid rgba(0,0,0,0.06); font-size: 13px; background: rgba(255,255,255,0.7); backdrop-filter: blur(16px); -webkit-backdrop-filter: blur(16px); }
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
