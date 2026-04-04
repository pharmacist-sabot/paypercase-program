// composables/useToast.ts — Lightweight singleton toast manager

import { ref } from 'vue'

export type ToastType = 'success' | 'error' | 'info' | 'warning'

export interface Toast {
  id: number
  type: ToastType
  message: string
}

let _nextId = 0
const _toasts = ref<Toast[]>([])

export function useToast() {
  function show(message: string, type: ToastType = 'info', duration = 3500) {
    const id = ++_nextId
    _toasts.value.push({ id, type, message })
    setTimeout(() => dismiss(id), duration)
  }

  function dismiss(id: number) {
    const idx = _toasts.value.findIndex(t => t.id === id)
    if (idx !== -1) _toasts.value.splice(idx, 1)
  }

  return { toasts: _toasts, show, dismiss }
}
