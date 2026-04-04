// stores/app.ts — Global application state using Pinia

import { defineStore } from 'pinia'
import { ref, computed } from 'vue'

export type MainTab = 'data' | 'pending' | 'delete'

export const useAppStore = defineStore('app', () => {
  // ── Connection state ─────────────────────────────────────────────────
  const isConnected = ref(false)
  const connectionMessage = ref('ยังไม่ได้เชื่อมต่อ HOSxP')
  const connectedHost = ref('')

  function setConnected(host = '', message = 'เชื่อมต่อ HOSxP สำเร็จ') {
    isConnected.value = true
    connectedHost.value = host
    connectionMessage.value = message
  }
  function setDisconnected(message = 'ยังไม่ได้เชื่อมต่อ HOSxP') {
    isConnected.value = false
    connectedHost.value = ''
    connectionMessage.value = message
  }

  // ── Active tab ───────────────────────────────────────────────────────
  const activeTab = ref<MainTab>('data')
  function setTab(tab: MainTab) { activeTab.value = tab }

  // ── Dialog visibility ────────────────────────────────────────────────
  const showConnectionDialog = ref(false)
  const showSettingsDialog = ref(false)
  const showSafetyDialog = ref(false)

  // ── Pending export refresh trigger ───────────────────────────────────
  // Incremented whenever pending_export data changes
  const pendingRefreshKey = ref(0)
  function triggerPendingRefresh() { pendingRefreshKey.value++ }

  // ── Stats ────────────────────────────────────────────────────────────
  const statusText = computed(() => {
    if (isConnected.value) {
      return `เชื่อมต่อ HOSxP: ${connectedHost.value} ✅`
    }
    return 'ไม่ได้เชื่อมต่อ HOSxP ❌'
  })

  return {
    isConnected,
    connectionMessage,
    connectedHost,
    setConnected,
    setDisconnected,
    activeTab,
    setTab,
    showConnectionDialog,
    showSettingsDialog,
    showSafetyDialog,
    pendingRefreshKey,
    triggerPendingRefresh,
    statusText,
  }
})
