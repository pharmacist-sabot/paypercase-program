<template>
<Teleport to="body">
    <div class="toast-container">
        <TransitionGroup name="toast">
            <div v-for="t in toasts" :key="t.id" class="toast" :class="`toast--${t.type}`" @click="dismiss(t.id)">
                <span class="toast__icon">{{ icons[t.type] }}</span>
                <span class="toast__msg">{{ t.message }}</span>
                <button class="toast__close" @click.stop="dismiss(t.id)">✕</button>
            </div>
        </TransitionGroup>
    </div>
</Teleport>
</template>

<script setup lang="ts">
import { useToast } from '@/composables/useToast'

const { toasts, dismiss } = useToast()

const icons: Record<string, string> = {
    success: '✅',
    error: '❌',
    warning: '⚠️',
    info: 'ℹ️',
}
</script>

<style scoped>
.toast-container {
    position: fixed;
    bottom: 28px;
    right: 24px;
    z-index: 99999;
    display: flex;
    flex-direction: column;
    gap: 10px;
    pointer-events: none;
}

.toast {
    display: flex;
    align-items: center;
    gap: 10px;
    min-width: 260px;
    max-width: 400px;
    padding: 12px 14px;
    border-radius: 8px;
    box-shadow: 0 4px 16px rgba(0, 0, 0, 0.18);
    font-family: var(--font);
    font-size: 13px;
    font-weight: 500;
    cursor: pointer;
    pointer-events: all;
    user-select: none;
    line-height: 1.4;
}

.toast--success {
    background: #2e7d32;
    color: #fff;
    border-left: 4px solid #1b5e20;
}

.toast--error {
    background: #c62828;
    color: #fff;
    border-left: 4px solid #b71c1c;
}

.toast--warning {
    background: #e65100;
    color: #fff;
    border-left: 4px solid #bf360c;
}

.toast--info {
    background: #1565c0;
    color: #fff;
    border-left: 4px solid #0d47a1;
}

.toast__icon {
    font-size: 16px;
    flex-shrink: 0;
}

.toast__msg {
    flex: 1;
}

.toast__close {
    background: none;
    border: none;
    color: rgba(255, 255, 255, 0.75);
    cursor: pointer;
    font-size: 14px;
    padding: 0 2px;
    line-height: 1;
    flex-shrink: 0;
}

.toast__close:hover {
    color: #fff;
}

/* Transition */
.toast-enter-active {
    transition: all 0.28s cubic-bezier(0.34, 1.3, 0.64, 1);
}

.toast-leave-active {
    transition: all 0.22s ease-in;
}

.toast-enter-from {
    opacity: 0;
    transform: translateX(60px);
}

.toast-leave-to {
    opacity: 0;
    transform: translateX(60px);
}
</style>
