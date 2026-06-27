<script setup lang="ts">
import { ref, onMounted, onUnmounted, watch } from 'vue'

const props = defineProps<{ active: boolean }>()
const canvas = ref<HTMLCanvasElement>()
let animId = 0
let drops: number[] = []
let columns = 0

const chars = 'ｦｧｨｩｪｫｬｭｮｯｱｲｳｴｵｶｷｸｹｺｻｼｽｾｿﾀﾁﾂﾃ0123456789'
const fontSize = 13
const color = 'rgba(0, 255, 65, 0.12)'
const headColor = 'rgba(0, 255, 65, 0.25)'

function init() {
  if (!canvas.value) return
  const c = canvas.value
  c.width = window.innerWidth
  c.height = window.innerHeight
  columns = Math.floor(c.width / (fontSize * 0.8))
  drops = Array(columns).fill(0).map(() => Math.random() * -100)
}

function draw() {
  if (!canvas.value) return
  const c = canvas.value
  const ctx = c.getContext('2d')
  if (!ctx) return

  ctx.fillStyle = 'rgba(13, 17, 23, 0.05)'
  ctx.fillRect(0, 0, c.width, c.height)

  ctx.font = `${fontSize}px 'JetBrains Mono'`
  for (let i = 0; i < drops.length; i++) {
    const char = chars[Math.floor(Math.random() * chars.length)]
    const x = i * fontSize * 0.8
    const y = drops[i] * fontSize

    ctx.fillStyle = headColor
    ctx.fillText(char, x, y)

    ctx.fillStyle = color
    ctx.fillText(char, x, y - fontSize)

    if (y > c.height && Math.random() > 0.975) {
      drops[i] = 0
    }
    drops[i]++
  }
  animId = requestAnimationFrame(draw)
}

function start() {
  if (!props.active) return
  init()
  animId = requestAnimationFrame(draw)
}

function stop() {
  cancelAnimationFrame(animId)
}

watch(() => props.active, (val) => {
  if (val) start()
  else stop()
})

onMounted(() => { if (props.active) start() })
onUnmounted(() => stop())
</script>

<template>
  <canvas v-if="active" ref="canvas" class="matrix-rain" />
</template>

<style scoped>
.matrix-rain {
  position: fixed;
  inset: 0;
  z-index: 0;
  pointer-events: none;
}
</style>
