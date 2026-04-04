<template>
  <div class="status-bar">
    <span :class="isConnected ? 'status-bar__connected' : 'status-bar__disconnected'">
      {{ isConnected ? '🟢' : '🔴' }} {{ statusText }}
    </span>
    <span class="flex-1" />
    <span class="text-sm" style="color:#9FA8DA">PayPerCase v5.0.0 — ระบบจัดการค่าตอบแทนพนักงานนวดแผนไทย</span>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useAppStore } from '@/stores/app'

const store = useAppStore()
const isConnected = computed(() => store.isConnected)
const statusText = computed(() => store.statusText)
</script>
