<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { getCurrentWindow } from '@tauri-apps/api/window'
import MatrixRain from './components/MatrixRain.vue'
import NoteWindow from './components/NoteWindow.vue'
import ManagementPanel from './components/ManagementPanel.vue'
import SettingsPanel from './components/SettingsPanel.vue'
import CalendarWidget from './components/CalendarWidget.vue'

const isHackerTheme = ref(document.documentElement.classList.contains('theme-hacker'))
const windowType = ref<'note' | 'management' | 'settings' | 'calendar'>('note')

onMounted(() => {
  const label = getCurrentWindow().label
  if (label === 'management-panel') { windowType.value = 'management' }
  else if (label === 'settings-panel') { windowType.value = 'settings' }
  else if (label === 'calendar-widget') { windowType.value = 'calendar' }
  else { windowType.value = 'note' }
})
</script>

<template>
  <MatrixRain :active="isHackerTheme" />
  <NoteWindow v-if="windowType === 'note'" />
  <ManagementPanel v-else-if="windowType === 'management'" />
  <SettingsPanel v-else-if="windowType === 'settings'" />
  <CalendarWidget v-else-if="windowType === 'calendar'" />
</template>
