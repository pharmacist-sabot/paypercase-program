<template>
    <nav class="menu-bar">
        <!-- App title -->
        <span class="menu-bar__logo">
            <svg
                width="20"
                height="20"
                viewBox="0 0 64 64"
                xmlns="http://www.w3.org/2000/svg"
                style="
                    display: inline-block;
                    vertical-align: middle;
                    margin-right: 6px;
                "
            >
                <defs>
                    <linearGradient
                        id="mbMainGradient"
                        x1="0%"
                        y1="0%"
                        x2="100%"
                        y2="100%"
                    >
                        <stop
                            offset="0%"
                            style="stop-color: #7986cb; stop-opacity: 1"
                        />
                        <stop
                            offset="100%"
                            style="stop-color: #3f51b5; stop-opacity: 1"
                        />
                    </linearGradient>
                    <linearGradient
                        id="mbCircleFaceGradient"
                        x1="0%"
                        y1="0%"
                        x2="100%"
                        y2="100%"
                    >
                        <stop
                            offset="0%"
                            style="stop-color: #ffe0b2; stop-opacity: 1"
                        />
                        <stop
                            offset="100%"
                            style="stop-color: #ffab91; stop-opacity: 1"
                        />
                    </linearGradient>
                </defs>
                <line
                    x1="48"
                    y1="16"
                    x2="16"
                    y2="48"
                    stroke="url(#mbMainGradient)"
                    stroke-width="7"
                    stroke-linecap="round"
                />
                <circle
                    cx="20"
                    cy="20"
                    r="10"
                    fill="url(#mbCircleFaceGradient)"
                />
                <circle cx="20" cy="20" r="3" fill="white" fill-opacity="0.4" />
                <circle
                    cx="44"
                    cy="44"
                    r="10"
                    fill="url(#mbCircleFaceGradient)"
                />
                <circle
                    cx="44"
                    cy="44"
                    r="3"
                    fill="white"
                    fill-opacity="0.4"
                /></svg
            >PayPerCase
        </span>

        <!-- Menu: การเชื่อมต่อ -->
        <div
            class="menu-bar__item"
            :class="{ active: openMenu === 'conn' }"
            @click="toggleMenu('conn')"
        >
            การเชื่อมต่อ
            <div v-if="openMenu === 'conn'" class="dropdown-menu">
                <div
                    class="dropdown-menu__item"
                    @click.stop="closeAndEmit('open-connection')"
                >
                    ตั้งค่าการเชื่อมต่อ HOSxP
                </div>
                <div class="dropdown-menu__separator" />
                <div
                    class="dropdown-menu__item"
                    @click.stop="closeAndEmit('reconnect')"
                >
                    เชื่อมต่อใหม่
                </div>
                <div
                    class="dropdown-menu__item"
                    @click.stop="closeAndEmit('disconnect')"
                >
                    ตัดการเชื่อมต่อ
                </div>
            </div>
        </div>

        <!-- Menu: การตั้งค่า -->
        <div
            class="menu-bar__item"
            :class="{ active: openMenu === 'settings' }"
            @click="toggleMenu('settings')"
        >
            การตั้งค่า
            <div v-if="openMenu === 'settings'" class="dropdown-menu">
                <div
                    class="dropdown-menu__item"
                    @click.stop="closeAndEmit('open-settings')"
                >
                    ตั้งค่าระบบ
                </div>
            </div>
        </div>

        <!-- Menu: เกี่ยวกับ -->
        <div
            class="menu-bar__item"
            :class="{ active: openMenu === 'about' }"
            @click="toggleMenu('about')"
        >
            เกี่ยวกับ
            <div v-if="openMenu === 'about'" class="dropdown-menu">
                <div
                    class="dropdown-menu__item"
                    @click.stop="closeAndEmit('show-safety')"
                >
                    นโยบายความปลอดภัย HOSxP
                </div>
                <div class="dropdown-menu__item" @click.stop="openDisclaimer">
                    ข้อจำกัดความรับผิดชอบ
                </div>
                <div class="dropdown-menu__separator" />
                <div class="dropdown-menu__item" @click.stop="closeMenu">
                    <span>PayPerCase</span>
                </div>
            </div>
        </div>
    </nav>

    <!-- Disclaimer Modal -->
    <div
        v-if="showDisclaimer"
        class="modal-backdrop"
        @click.self="closeDisclaimer"
    >
        <div
            class="modal"
            role="dialog"
            aria-modal="true"
            aria-label="ข้อจำกัดความรับผิดชอบ"
        >
            <header class="modal__header">ข้อจำกัดความรับผิดชอบ</header>
            <div class="modal__body">
                ผู้พัฒนาไม่รับผิดชอบต่อความเสียหายหรือการสูญเสียใด ๆ
                ที่อาจเกิดขึ้นจากการใช้งานซอฟต์แวร์นี้
            </div>
            <footer class="modal__footer">
                <button class="btn" @click="closeDisclaimer">ปิด</button>
            </footer>
        </div>
    </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from "vue";

const emit = defineEmits<{
    "open-connection": [];
    "open-settings": [];
    "show-safety": [];
    reconnect: [];
    disconnect: [];
    exit: [];
}>();

const openMenu = ref<string | null>(null);
const showDisclaimer = ref(false);

function toggleMenu(name: string) {
    openMenu.value = openMenu.value === name ? null : name;
}

function closeMenu() {
    openMenu.value = null;
}

function closeAndEmit(event: string) {
    closeMenu();
    emit(event as any);
}

function openDisclaimer() {
    closeMenu();
    showDisclaimer.value = true;
}

function closeDisclaimer() {
    showDisclaimer.value = false;
}

function onClickOutside(e: MouseEvent) {
    const target = e.target as HTMLElement;
    if (!target.closest(".menu-bar__item")) {
        closeMenu();
    }
}

onMounted(() => document.addEventListener("click", onClickOutside));
onUnmounted(() => document.removeEventListener("click", onClickOutside));
</script>

<style scoped>
.menu-bar__logo {
    color: #c5cae9;
    font-weight: bold;
    font-size: 13px;
    padding: 4px 12px;
    user-select: none;
}
.dropdown-menu__shortcut {
    margin-left: auto;
    font-size: 10px;
    color: var(--text-gray);
    padding-left: 16px;
}

/* Disclaimer modal */
.modal-backdrop {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.45);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
}
.modal {
    background: var(--bg-surface, #fff);
    padding: 16px;
    border-radius: 8px;
    width: 420px;
    max-width: calc(100% - 40px);
    box-shadow: 0 6px 24px rgba(0, 0, 0, 0.2);
}
.modal__header {
    font-weight: 600;
    margin-bottom: 8px;
}
.modal__body {
    font-size: 14px;
    color: var(--text, #333);
    margin-bottom: 12px;
}
.modal__footer {
    text-align: right;
}
.btn {
    background: var(--primary, #3f51b5);
    color: #fff;
    border: none;
    padding: 6px 10px;
    border-radius: 4px;
    cursor: pointer;
}
</style>
