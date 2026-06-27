import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import type { Group, Note, TimerState, Reminder, WindowState } from '../types'

// ── Groups ──
export const createGroup = (name: string) => invoke<Group>('create_group', { name })
export const listGroups = () => invoke<Group[]>('list_groups')
export const renameGroup = (id: string, name: string) => invoke<void>('rename_group', { id, name })
export const deleteGroup = (id: string) => invoke<void>('delete_group', { id })

// ── Notes ──
export const createNote = (groupId: string | null, noteType: string, title: string | null) =>
  invoke<Note>('create_note', { groupId, noteType, title })
export const updateNote = (id: string, data: Partial<Pick<Note, 'title' | 'content' | 'bg_color' | 'default_text_color' | 'default_font_size'>>) =>
  invoke<void>('update_note', { id, ...data })
export const getNote = (id: string) => invoke<Note>('get_note', { id })
export const listNotes = (groupId?: string, noteType?: string, includeDeleted = false) =>
  invoke<Note[]>('list_notes', { groupId: groupId ?? null, noteType: noteType ?? null, includeDeleted })
export const deleteNote = (id: string) => invoke<void>('delete_note', { id })
export const restoreNote = (id: string) => invoke<void>('restore_note', { id })
export const permanentlyDeleteNote = (id: string) => invoke<void>('permanently_delete_note', { id })
export const searchNotes = (query: string) => invoke<Note[]>('search_notes', { query })

// ── Timers ──
export const getTimerState = (noteId: string) => invoke<TimerState>('get_timer_state', { noteId })
export const startTimer = (noteId: string) => invoke<void>('start_timer', { noteId })
export const pauseTimer = (noteId: string) => invoke<void>('pause_timer', { noteId })
export const resetTimer = (noteId: string) => invoke<void>('reset_timer', { noteId })
export const updateTimerSettings = (noteId: string, data: Record<string, unknown>) =>
  invoke<void>('update_timer_settings', { noteId, ...data })

// ── Reminders ──
export const setReminder = (noteId: string, remindAt: string, repeatType: string, repeatDays?: string) =>
  invoke<Reminder>('set_reminder', { noteId, remindAt, repeatType, repeatDays: repeatDays ?? null })
export const getReminders = (noteId: string) => invoke<Reminder[]>('get_reminders', { noteId })
export const deleteReminder = (id: string) => invoke<void>('delete_reminder', { id })

// ── Windows ──
export const saveWindowState = (state: WindowState) => invoke<void>('save_window_state', { state })
export const setWindowOpacity = (noteId: string, opacity: number) => invoke<void>('set_window_opacity', { noteId, opacity })
export const getWindowState = (noteId: string) => invoke<WindowState | null>('get_window_state', { noteId })
export const getAllWindowStates = () => invoke<WindowState[]>('get_all_window_states')

// ── Settings ──
export const setSetting = (key: string, value: string) => invoke<void>('set_setting', { key, value })
export const getSetting = (key: string) => invoke<string | null>('get_setting', { key })
export const exportAllData = (path: string) => invoke<void>('export_all_data', { path })
export const readLog = () => invoke<string>('read_log')

// ── Event Listeners ──
export const onTimerTick = (cb: (data: { note_id: string; remaining_seconds: number; is_running: boolean }) => void) => {
  const unlisten = listen<{ note_id: string; remaining_seconds: number; is_running: boolean }>('timer-tick', (e) => cb(e.payload))
  return () => { unlisten.then(fn => fn()) }
}

export const onTimerFinished = (cb: (data: { note_id: string }) => void) => {
  const unlisten = listen<{ note_id: string }>('timer-finished', (e) => cb(e.payload))
  return () => { unlisten.then(fn => fn()) }
}

export const onTimerWarning = (cb: (data: { note_id: string; minutes_left: number }) => void) => {
  const unlisten = listen<{ note_id: string; minutes_left: number }>('timer-warning', (e) => cb(e.payload))
  return () => { unlisten.then(fn => fn()) }
}

export const onReminderTriggered = (cb: (data: { reminder_id: string; note_id: string }) => void) => {
  const unlisten = listen<{ reminder_id: string; note_id: string }>('reminder-triggered', (e) => cb(e.payload))
  return () => { unlisten.then(fn => fn()) }
}

export const onTankMilestone = (cb: (data: { note_id: string; days: number }) => void) => {
  const unlisten = listen<{ note_id: string; days: number }>('tank-milestone', (e) => cb(e.payload))
  return () => { unlisten.then(fn => fn()) }
}

export const onMidnightReset = (cb: () => void) => {
  const unlisten = listen('midnight-reset', () => cb())
  return () => { unlisten.then(fn => fn()) }
}
