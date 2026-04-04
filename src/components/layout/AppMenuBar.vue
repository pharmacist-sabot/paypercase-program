<template>
  <nav class="menu-bar">
    <!-- App title -->
    <span class="menu-bar__logo">💊 PayPerCase</span>

    <!-- Menu: ไฟล์ -->
    <div class="menu-bar__item" :class="{ active: openMenu === 'file' }" @click="toggleMenu('file')">
      ไฟล์
      <div v-if="openMenu === 'file'" class="dropdown-menu">
        <div class="dropdown-menu__item" @click.stop="closeAndEmit('exit')">
          🚪 ออกจากโปรแกรม <span class="dropdown-menu__shortcut">Ctrl+Q</span>
        </div>
      </div>
    </div>

    <!-- Menu: การเชื่อมต่อ -->
    <div class="menu-bar__item" :class="{ active: openMenu === 'conn' }" @click="toggleMenu('conn')">
      🔌 การเชื่อมต่อ
      <div v-if="openMenu === 'conn'" class="dropdown-menu">
        <div class="dropdown-menu__item" @click.stop="closeAndEmit('open-connection')">
          🔌 ตั้งค่าการเชื่อมต่อ HOSxP
        </div>
        <div class="dropdown-menu__separator" />
        <div class="dropdown-menu__item" @click.stop="closeAndEmit('reconnect')">
          🔄 เชื่อมต่อใหม่
        </div>
        <div class="dropdown-menu__item" @click.stop="closeAndEmit('disconnect')">
          ⛔ ตัดการเชื่อมต่อ
        </div>
      </div>
    </div>

    <!-- Menu: การตั้งค่า -->
    <div class="menu-bar__item" :class="{ active: openMenu === 'settings' }" @click="toggleMenu('settings')">
      ⚙️ การตั้งค่า
      <div v-if="openMenu === 'settings'" class="dropdown-menu">
        <div class="dropdown-menu__item" @click.stop="closeAndEmit('open-settings')">
          ⚙️ ตั้งค่าระบบ
        </div>
      </div>
    </div>

    <!-- Menu: เกี่ยวกับ -->
    <div class="menu-bar__item" :class="{ active: openMenu === 'about' }" @click="toggleMenu('about')">
      ℹ️ เกี่ยวกับ
      <div v-if="openMenu === 'about'" class="dropdown-menu">
        <div class="dropdown-menu__item" @click.stop="closeAndEmit('show-safety')">
          🛡️ นโยบายความปลอดภัย HOSxP
        </div>
        <div class="dropdown-menu__separator" />
        <div class="dropdown-menu__item" @click.stop="closeMenu">
          <span>PayPerCase v5.0.0</span>
        </div>
      </div>
    </div>
  </nav>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'

const emit = defineEmits<{
  'open-connection': []
  'open-settings': []
  'show-safety': []
  'reconnect': []
  'disconnect': []
  'exit': []
}>()

const openMenu = ref<string | null>(null)

function toggleMenu(name: string) {
  openMenu.value = openMenu.value === name ? null : name
}

function closeMenu() {
  openMenu.value = null
}

function closeAndEmit(event: string) {
  closeMenu()
  emit(event as any)
}

function onClickOutside(e: MouseEvent) {
  const target = e.target as HTMLElement
  if (!target.closest('.menu-bar__item')) {
    closeMenu()
  }
}

onMounted(() => document.addEventListener('click', onClickOutside))
onUnmounted(() => document.removeEventListener('click', onClickOutside))
</script>

<style scoped>
.menu-bar__logo {
  color: #C5CAE9;
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
</style>
