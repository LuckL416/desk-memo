<script setup lang="ts">
import { ref } from 'vue'

const props = defineProps<{
  title: string
  mode: 'desktop' | 'top'
  pinned: boolean
  opacity: number
}>()

const emit = defineEmits<{
  close: []
  toggleMode: []
  togglePinned: []
  updateOpacity: [value: number]
  showMenu: [event: MouseEvent]
}>()

const showOpacity = ref(false)
</script>

<template>
  <div class="chrome-bar" data-tauri-drag-region>
    <span class="chrome-title">{{ title || '无标题' }}</span>
    <div class="chrome-actions">
      <button class="chrome-btn" @click="showOpacity = !showOpacity" title="透明度">◐</button>
      <button class="chrome-btn" @click="emit('togglePinned')" :title="pinned ? '解锁' : '锁定'">
        {{ pinned ? '🔒' : '🔓' }}
      </button>
      <button class="chrome-btn" @click="emit('toggleMode')" :title="mode === 'desktop' ? '切换到置顶' : '切换到底层'">
        📌
      </button>
      <button class="chrome-btn" @click="emit('showMenu', $event)" title="更多">⋯</button>
      <button class="chrome-btn chrome-close" @click="emit('close')" title="隐藏">✕</button>
    </div>
    <div v-if="showOpacity" class="opacity-dropdown">
      <input type="range" min="5" max="100" :value="Math.round(opacity * 100)"
        @input="emit('updateOpacity', Number(($event.target as HTMLInputElement).value) / 100)" />
      <span>{{ Math.round(opacity * 100) }}%</span>
    </div>
  </div>
</template>

<style scoped>
.chrome-bar {
  display: flex; align-items: center; height: var(--chrome-height);
  padding: 0 8px; gap: 6px; background: rgba(0,0,0,0.04);
  cursor: move; position: relative;
}
.chrome-title {
  font-size: 13px; font-weight: 600; margin-left: 8px;
  white-space: nowrap; overflow: hidden; text-overflow: ellipsis; max-width: 160px;
}
.chrome-actions { display: flex; gap: 4px; margin-left: auto; }
.chrome-btn {
  background: rgba(0,0,0,0.06); border: none; cursor: pointer;
  font-size: 12px; padding: 2px 6px; border-radius: var(--rounded-pill);
  color: var(--color-ink); line-height: 1.4;
}
.chrome-btn:hover { background: rgba(0,0,0,0.12); }
.chrome-close:hover { background: #ff3d8b; color: #fff; }
.opacity-dropdown {
  position: absolute; top: 100%; right: 8px; background: #fff;
  border: 1px solid var(--color-hairline); border-radius: var(--rounded-md);
  padding: 8px 12px; display: flex; align-items: center; gap: 8px;
  font-size: 12px; z-index: 100; box-shadow: 0 2px 8px rgba(0,0,0,0.08);
}
</style>
