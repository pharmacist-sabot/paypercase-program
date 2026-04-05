<script setup lang="ts">
import { ref, onMounted } from "vue";
import { useAppStore } from "@/stores/app";
import AppMenuBar from "@/components/layout/AppMenuBar.vue";
import AppStatusBar from "@/components/layout/AppStatusBar.vue";
import DataViewTab from "@/components/tabs/DataViewTab.vue";
import PendingExportTab from "@/components/tabs/PendingExportTab.vue";
import DeleteDataTab from "@/components/tabs/DeleteDataTab.vue";
import ConnectionDialog from "@/components/dialog/ConnectionDialog.vue";
import SettingsDialog from "@/components/dialog/SettingsDialog.vue";
import * as cmd from "@/composables/useCommands";
import AppToast from "@/components/layout/AppToast.vue";

const store = useAppStore();

const showConnection = ref(false);
const showSettings = ref(false);
const showSafetyInfo = ref(false);

onMounted(async () => {
    try {
        const result = await cmd.autoConnect();
        if (result.ok) {
            const settings = await cmd.getHosxpSettings().catch(() => null);
            store.setConnected(settings?.host ?? "", result.message);
        }
    } catch {}
});

async function handleMenuEvent(event: string) {
    if (event === "open-connection") showConnection.value = true;
    else if (event === "open-settings") showSettings.value = true;
    else if (event === "show-safety") showSafetyInfo.value = true;
    else if (event === "reconnect") {
        try {
            const result = await cmd.autoConnect();
            if (result.ok) {
                const settings = await cmd.getHosxpSettings().catch(() => null);
                store.setConnected(settings?.host ?? "", result.message);
            }
        } catch {}
    } else if (event === "disconnect") {
        try {
            await cmd.disconnectHosxp();
            store.setDisconnected();
        } catch {}
    } else if (event === "exit") {
        const { exit } = await import("@tauri-apps/plugin-process");
        await exit(0);
    }
}
</script>

<template>
    <div class="app-layout">
        <!-- Menu bar -->
        <AppMenuBar
            @open-connection="handleMenuEvent('open-connection')"
            @open-settings="handleMenuEvent('open-settings')"
            @show-safety="handleMenuEvent('show-safety')"
            @reconnect="handleMenuEvent('reconnect')"
            @disconnect="handleMenuEvent('disconnect')"
            @exit="handleMenuEvent('exit')"
        />

        <!-- Tab bar -->
        <div class="tab-bar">
            <button
                class="tab-btn"
                :class="{ active: store.activeTab === 'data' }"
                @click="store.activeTab = 'data'"
            >
                📋 ข้อมูลผู้ป่วย
            </button>
            <button
                class="tab-btn"
                :class="{ active: store.activeTab === 'pending' }"
                @click="store.activeTab = 'pending'"
            >
                📤 ข้อมูลรอส่งออก
            </button>
            <button
                class="tab-btn"
                :class="{ active: store.activeTab === 'delete' }"
                @click="store.activeTab = 'delete'"
            >
                🗑️ ลบข้อมูล
            </button>
        </div>

        <!-- Main panel — wrapper divs keep components alive so dates & filters are preserved -->
        <div class="main-panel">
            <div class="tab-pane" v-show="store.activeTab === 'data'">
                <DataViewTab />
            </div>
            <div class="tab-pane" v-show="store.activeTab === 'pending'">
                <PendingExportTab />
            </div>
            <div class="tab-pane" v-show="store.activeTab === 'delete'">
                <DeleteDataTab />
            </div>
        </div>

        <!-- Status bar -->
        <AppStatusBar />

        <!-- Dialogs (v-if is fine — they are lightweight) -->
        <ConnectionDialog
            v-if="showConnection"
            @close="showConnection = false"
        />
        <SettingsDialog v-if="showSettings" @close="showSettings = false" />

        <!-- Safety info modal -->
        <div
            v-if="showSafetyInfo"
            class="modal-overlay"
            @click.self="showSafetyInfo = false"
        >
            <div class="modal" style="max-width: 620px">
                <div class="modal__header">
                    <span class="modal__title"
                        >🛡️ ความปลอดภัยของฐานข้อมูล HOSxP</span
                    >
                    <button
                        class="modal__close"
                        @click="showSafetyInfo = false"
                    >
                        ✕
                    </button>
                </div>
                <div
                    class="modal__body"
                    style="font-size: 13px; line-height: 1.8"
                >
                    <p>
                        <strong
                            >โปรแกรม PayPerCase อ่านข้อมูลจาก HOSxP แบบ
                            READ-ONLY เท่านั้น</strong
                        >
                    </p>
                    <ul style="margin-top: 10px; padding-left: 20px">
                        <li>
                            ไม่มีการ INSERT, UPDATE หรือ DELETE ข้อมูลใดๆ ใน
                            HOSxP
                        </li>
                        <li>ใช้เพียง SELECT เพื่อดึงข้อมูลมาแสดงผลเท่านั้น</li>
                        <li>
                            ข้อมูลที่บันทึกทั้งหมดจะถูกเก็บในฐานข้อมูลของโปรแกรมนี้แยกต่างหาก
                            (paypercase.db)
                        </li>
                        <li>ไม่กระทบต่อการทำงานของ HOSxP แต่อย่างใด</li>
                    </ul>
                </div>
                <div class="modal__footer">
                    <button
                        class="btn btn-primary"
                        @click="showSafetyInfo = false"
                    >
                        รับทราบ
                    </button>
                </div>
            </div>
        </div>

        <!-- Toast notifications -->
        <AppToast />
    </div>
</template>

<style>
/* ── Global Reset (supplement theme.css) ──────────────────────────────── */
*,
*::before,
*::after {
    box-sizing: border-box;
    margin: 0;
    padding: 0;
}

html,
body,
#app {
    height: 100%;
    font-family: var(--font);
    font-size: 13px;
    color: var(--text-primary);
    background: var(--bg-main);
    overflow: hidden;
    -webkit-font-smoothing: antialiased;
    text-rendering: optimizeLegibility;
}

/* ── App Shell ─────────────────────────────────────────────────────────── */
.app-layout {
    display: flex;
    flex-direction: column;
    height: 100vh;
    width: 100vw;
    background: var(--bg-main);
}

/* ── Tab Bar ───────────────────────────────────────────────────────────── */
.tab-bar {
    display: flex;
    background: #e8eaf6;
    border-bottom: 2px solid var(--border);
    padding: 0 8px;
    flex-shrink: 0;
    gap: 2px;
}

.tab-btn {
    padding: 8px 20px;
    border: none;
    border-bottom: 3px solid transparent;
    margin-bottom: -2px;
    background: transparent;
    color: var(--text-primary);
    cursor: pointer;
    font-family: var(--font);
    font-size: 13px;
    white-space: nowrap;
    transition:
        background 0.15s,
        border-color 0.15s,
        color 0.15s;
    border-radius: var(--radius) var(--radius) 0 0;
}

.tab-btn:hover {
    background: #c5cae9;
    border-bottom-color: var(--border);
}

.tab-btn.active {
    background: #fff;
    border-bottom-color: var(--primary);
    font-weight: 600;
    color: var(--primary);
}

/* ── Main Panel ────────────────────────────────────────────────────────── */
.main-panel {
    flex: 1;
    overflow: hidden;
    background: var(--bg-main);
}

/* Each tab-pane wrapper fills the entire panel height */
.tab-pane {
    height: 100%;
    overflow: hidden;
}
</style>
