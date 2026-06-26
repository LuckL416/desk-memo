<script setup lang="ts">
import { ref, onMounted } from 'vue'
import NoteWindow from './components/NoteWindow.vue'
import ManagementPanel from './components/ManagementPanel.vue'
import SettingsPanel from './components/SettingsPanel.vue'

const windowType = ref<'note' | 'management' | 'settings'>('note')

onMounted(() => {
  const w = window as any
  if (w.__windowType === 'management') {
    windowType.value = 'management'
  } else if (w.__windowType === 'settings') {
    windowType.value = 'settings'
  }
})
</script>

<template>
  <NoteWindow v-if="windowType === 'note'" />
  <ManagementPanel v-else-if="windowType === 'management'" />
  <SettingsPanel v-else-if="windowType === 'settings'" />
</template>
