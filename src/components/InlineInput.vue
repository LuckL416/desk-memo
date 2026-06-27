<script setup lang="ts">
import { ref, nextTick, onMounted } from 'vue'

const props = defineProps<{ placeholder?: string; initialValue?: string }>()
const emit = defineEmits<{ confirm: [value: string]; cancel: [] }>()

const value = ref(props.initialValue || '')
const inputRef = ref<HTMLInputElement>()

onMounted(() => {
  nextTick(() => inputRef.value?.focus())
})

function onKeydown(e: KeyboardEvent) {
  if (e.key === 'Enter') emit('confirm', value.value)
  if (e.key === 'Escape') emit('cancel')
}
</script>

<template>
  <div class="inline-input-overlay" @click.self="emit('cancel')">
    <div class="inline-input-box">
      <input ref="inputRef" v-model="value" :placeholder="placeholder || '输入...'" @keydown="onKeydown" />
      <div class="inline-input-actions">
        <button class="btn-cancel" @click.stop="emit('cancel')">取消</button>
        <button class="btn-confirm" @click.stop="emit('confirm', value)">确定</button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.inline-input-overlay { position: fixed; inset: 0; background: rgba(0,0,0,0.3); display: flex; align-items: center; justify-content: center; z-index: 500; }
.inline-input-box { background: #fff; border-radius: 16px; padding: 20px; min-width: 300px; box-shadow: 0 4px 24px rgba(0,0,0,0.15); }
.inline-input-box input { width: 100%; border: 1px solid #e6e6e6; border-radius: 8px; padding: 10px 14px; font-size: 15px; outline: none; margin-bottom: 12px; }
.inline-input-box input:focus { border-color: var(--color-primary); box-shadow: 0 0 0 2px rgba(0,0,0,0.08); }
.inline-input-actions { display: flex; gap: 8px; justify-content: flex-end; }
.btn-cancel { background: #fff; border: 1px solid #e6e6e6; padding: 6px 18px; border-radius: 50px; cursor: pointer; font-size: 13px; }
.btn-confirm { background: #000; color: #fff; border: none; padding: 6px 18px; border-radius: 50px; cursor: pointer; font-size: 13px; }
</style>
