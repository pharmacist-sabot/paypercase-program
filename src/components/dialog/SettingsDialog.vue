<template>
    <div class="modal-overlay" @click.self="$emit('close')">
        <div class="modal" style="width: 860px; height: 680px">
            <div class="modal__header">
                <span class="modal__title">⚙️ การตั้งค่าระบบ</span>
                <button class="modal__close" @click="$emit('close')">✕</button>
            </div>

            <div
                class="modal__body"
                style="
                    padding: 0;
                    display: flex;
                    flex-direction: column;
                    flex: 1;
                    overflow: hidden;
                "
            >
                <!-- Sub-tabs -->
                <div
                    class="sub-tabs"
                    style="
                        padding: 0 16px;
                        flex-shrink: 0;
                        border-bottom: 1px solid var(--border-light);
                    "
                >
                    <div
                        v-for="tab in tabs"
                        :key="tab.key"
                        class="sub-tab"
                        :class="{ active: activeSubTab === tab.key }"
                        @click="activeSubTab = tab.key"
                    >
                        {{ tab.label }}
                    </div>
                </div>

                <!-- Tab content -->
                <div style="flex: 1; overflow: auto; padding: 16px">
                    <PttypeTab v-if="activeSubTab === 'pttype'" />
                    <ProcedureTab v-else-if="activeSubTab === 'procedure'" />
                    <DrugTab v-else-if="activeSubTab === 'drug'" />
                    <ProviderTab v-else-if="activeSubTab === 'provider'" />
                    <PayoutTab v-else-if="activeSubTab === 'payout'" />
                </div>
            </div>

            <div class="modal__footer">
                <button class="btn btn-secondary" @click="$emit('close')">
                    ปิด
                </button>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import { ref } from "vue";
import PttypeTab from "@/components/settings/PttypeTab.vue";
import ProcedureTab from "@/components/settings/ProcedureTab.vue";
import DrugTab from "@/components/settings/DrugTab.vue";
import ProviderTab from "@/components/settings/ProviderTab.vue";
import PayoutTab from "@/components/settings/PayoutTab.vue";

defineEmits<{ close: [] }>();

const activeSubTab = ref("pttype");
const tabs = [
    { key: "pttype", label: "💳 สิทธิการรักษา" },
    { key: "procedure", label: "🩺 หัตถการ" },
    { key: "drug", label: "💊 ยาสมุนไพร" },
    { key: "provider", label: "👤 พนักงาน" },
    { key: "payout", label: "💰 ค่าตอบแทน" },
];
</script>
