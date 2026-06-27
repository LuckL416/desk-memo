<script setup lang="ts">
defineProps<{ message: string }>()
const emit = defineEmits<{ confirm: []; cancel: [] }>()
</script>

<template>
  <div class="confirm-overlay" @click.self="emit('cancel')">
    <div class="confirm-box">
      <p>{{ message }}</p>
      <div class="confirm-actions">
        <button class="btn-cancel" @click="emit('cancel')">取消</button>
        <button class="btn-confirm" @click="emit('confirm')">确定</button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.confirm-overlay { position: fixed; inset: 0; background: rgba(0,0,0,0.4); display: flex; align-items: center; justify-content: center; z-index: 600; }
.confirm-box { background: #fff; border-radius: 12px; padding: 24px; min-width: 300px; max-width: 400px; box-shadow: 0 4px 24px rgba(0,0,0,0.15); }
.confirm-box p { font-size: 14px; line-height: 1.5; margin-bottom: 20px; color: var(--color-ink); }
.confirm-actions { display: flex; gap: 8px; justify-content: flex-end; }
.btn-cancel { background: #fff; border: 1px solid var(--color-hairline); padding: 7px 20px; border-radius: var(--rounded-pill); cursor: pointer; font-size: 13px; }
.btn-cancel:hover { background: var(--color-surface-soft); }
.btn-confirm { background: var(--color-primary); color: var(--color-on-primary); border: none; padding: 7px 20px; border-radius: var(--rounded-pill); cursor: pointer; font-size: 13px; }
</style>
