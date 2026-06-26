<script setup lang="ts">
import { ref, onMounted } from 'vue'
import * as api from '../utils/tauri'
import type { Group } from '../types'

const emit = defineEmits<{ select: [group: Group | null] }>()
const groups = ref<Group[]>([])
const selectedId = ref<string | null>(null)

onMounted(async () => { groups.value = await api.listGroups() })

function select(group: Group | null) {
  selectedId.value = group?.id ?? null
  emit('select', group)
}

async function createGroup() {
  const name = prompt('新建分组名称:')
  if (name) { const g = await api.createGroup(name); groups.value.push(g) }
}

async function deleteGroup(g: Group) {
  if (confirm(`删除分组 "${g.name}"?\n该分组下的便签将移至未分组。`)) {
    await api.deleteGroup(g.id)
    groups.value = groups.value.filter(x => x.id !== g.id)
    if (selectedId.value === g.id) { selectedId.value = null; emit('select', null) }
  }
}
</script>

<template>
  <div class="sidebar">
    <div class="sidebar-label">分组</div>
    <div class="group-item" :class="{ active: selectedId === null }" @click="select(null)">全部</div>
    <div v-for="g in groups" :key="g.id" class="group-item" :class="{ active: selectedId === g.id }"
      @click="select(g)" @contextmenu.prevent="deleteGroup(g)">
      {{ g.name }}
    </div>
    <div class="sidebar-label" style="margin-top:8px;">🗑 回收站</div>
    <div class="add-group" @click="createGroup">+ 新建分组</div>
  </div>
</template>

<style scoped>
.sidebar { width: 160px; padding: 16px; border-right: 1px solid var(--color-hairline); font-size: 13px; }
.sidebar-label { font-weight: 600; margin-bottom: 10px; font-size: 11px; color: #888; }
.group-item { padding: 6px 10px; border-radius: var(--rounded-pill); margin-bottom: 4px; cursor: pointer; }
.group-item:hover { background: var(--color-surface-soft); }
.group-item.active { background: var(--color-primary); color: var(--color-on-primary); }
.add-group { margin-top: 8px; font-size: 12px; color: #888; cursor: pointer; }
</style>
