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
  hide: []
  toggleMode: []
  togglePinned: []
  updateOpacity: [value: number]
  showMenu: [event: MouseEvent]
}>()

const showOpacity = ref(false)
</script>

<template>
  <div
    class="chrome-bar"
    :class="{ 'chrome-locked': pinned }"
    :data-tauri-drag-region="pinned ? undefined : ''"
  >
    <span class="chrome-title">{{ title || '无标题' }}</span>
    <div class="chrome-actions" data-tauri-drag-region="false">
      <div class="opacity-wrap" data-tauri-drag-region="false">
        <button class="chrome-btn" data-tauri-drag-region="false" @click.stop="showOpacity = !showOpacity">
          透明度
        </button>
        <div v-if="showOpacity" class="opacity-dropdown" data-tauri-drag-region="false" @click.stop>
          <input type="range" min="5" max="100" :value="Math.round(opacity * 100)"
            @input="emit('updateOpacity', Number(($event.target as HTMLInputElement).value) / 100)" />
          <span>{{ Math.round(opacity * 100) }}%</span>
        </div>
      </div>
      <button class="chrome-btn" data-tauri-drag-region="false" @click.stop="emit('togglePinned')">
        {{ pinned ? '解锁' : '锁定' }}
      </button>
      <button class="chrome-btn" data-tauri-drag-region="false" @click.stop="emit('toggleMode')">
        {{ mode === 'top' ? '置底' : '置顶' }}
      </button>
      <button class="chrome-btn" data-tauri-drag-region="false" @click.stop="emit('showMenu', $event)">
        ···
      </button>
      <button class="chrome-btn" data-tauri-drag-region="false" @click.stop="emit('hide')" title="隐藏到托盘">
        −
      </button>
      <button class="chrome-btn chrome-close-btn" data-tauri-drag-region="false" @click.stop="emit('close')" title="关闭窗口">
        ×
      </button>
    </div>
  </div>
</template>

<style scoped>
.chrome-bar {
  display: flex;
  align-items: center;
  height: var(--chrome-height);
  padding: 0 6px 0 10px;
  gap: 6px;
  background: rgba(0, 0, 0, 0.12);
  cursor: grab;
  -webkit-app-region: drag;
  user-select: none;
}
.chrome-bar:active {
  cursor: grabbing;
}
.chrome-locked {
  -webkit-app-region: no-drag;
  cursor: default;
}
.chrome-title {
  font-size: 13px;
  font-weight: 600;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  max-width: 130px;
  flex-shrink: 1;
  color: var(--color-ink);
}
.chrome-actions {
  display: flex;
  gap: 3px;
  margin-left: auto;
  flex-shrink: 0;
  -webkit-app-region: no-drag;
}
.chrome-btn {
  background: transparent;
  border: none;
  cursor: pointer;
  font-size: 12px;
  padding: 3px 7px;
  border-radius: var(--rounded-pill);
  color: var(--color-ink);
  line-height: 1.3;
  -webkit-app-region: no-drag;
  transition: background 0.15s;
  white-space: nowrap;
}
.chrome-btn:hover {
  background: rgba(0, 0, 0, 0.12);
}
.chrome-close-btn:hover {
  background: #e81123 !important;
  color: #fff !important;
}
.opacity-wrap {
  position: relative;
  -webkit-app-region: no-drag;
}
.opacity-dropdown {
  position: absolute;
  top: 100%;
  left: 50%;
  transform: translateX(-50%);
  margin-top: 4px;
  background: #fff;
  border: 1px solid var(--color-hairline);
  border-radius: var(--rounded-md);
  padding: 8px 12px;
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 12px;
  z-index: 100;
  box-shadow: 0 2px 12px rgba(0, 0, 0, 0.15);
  -webkit-app-region: no-drag;
  white-space: nowrap;
}
.opacity-dropdown input[type="range"] {
  width: 100px;
}
</style>
