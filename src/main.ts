import { createApp } from 'vue'
import { createPinia } from 'pinia'
import { invoke } from '@tauri-apps/api/core'
import App from './App.vue'
import './styles/tokens.css'
import './styles/theme-hacker.css'
import './styles/memo-theme.css'
import './styles/global.css'

async function loadTheme() {
  try {
    const theme = await invoke<string | null>('get_setting', { key: 'theme' })
    document.documentElement.className = theme === 'hacker' ? 'theme-hacker' : 'theme-macaron'
  } catch {
    document.documentElement.className = 'theme-macaron'
  }
}
loadTheme()

const app = createApp(App)
app.use(createPinia())
app.mount('#app')
