export interface Group {
  id: string
  name: string
  sort_order: number
  created_at: string
  updated_at: string
}

export interface Note {
  id: string
  group_id: string | null
  type: 'text' | 'todo' | 'timer'
  title: string | null
  content: string | null
  bg_color: string | null
  default_text_color: string | null
  default_font_size: number | null
  created_at: string
  updated_at: string
  deleted_at: string | null
}

export interface TimerState {
  note_id: string
  daily_duration_minutes: number
  remaining_seconds: number
  is_running: boolean
  last_resume_at: string | null
  tank_start_date: string | null
  auto_start_time: string | null
  warn_before_minutes: number | null
}

export interface Reminder {
  id: string
  note_id: string
  remind_at: string
  repeat_type: 'once' | 'daily' | 'weekly'
  repeat_days: string | null
  is_active: boolean
}

export interface WindowState {
  note_id: string
  x: number
  y: number
  width: number
  height: number
  opacity: number
  pinned: boolean
  mode: 'desktop' | 'top'
  is_visible: boolean
}

export const PASTEL_COLORS = [
  { name: 'cream', hex: '#f4ecd6', label: '暖米色' },
  { name: 'lime', hex: '#dceeb1', label: '青柠' },
  { name: 'lilac', hex: '#c5b0f4', label: '薰衣草' },
  { name: 'mint', hex: '#c8e6cd', label: '薄荷' },
  { name: 'pink', hex: '#efd4d4', label: '粉红' },
  { name: 'coral', hex: '#f3c9b6', label: '珊瑚' },
  { name: 'navy', hex: '#1f1d3d', label: '深蓝' },
  { name: 'white', hex: '#ffffff', label: '纯白' },
] as const
