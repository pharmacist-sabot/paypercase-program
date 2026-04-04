<template>
    <div>
        <!-- Search group: search box + lookup result + save -->
        <div class="group-box mb-12">
            <span class="group-box__title">🩺 ค้นหาหัตถการใน HOSxP</span>

            <div style="padding-top: 12px">
                <div class="flex items-center gap-8 mb-8">
                    <label>icode :</label>
                    <input
                        type="text"
                        v-model="searchInput"
                        placeholder="เช่น 3003038"
                        style="width: 180px"
                        @keydown.enter="search"
                    />
                    <button
                        class="btn btn-secondary"
                        :disabled="searching"
                        @click="search"
                    >
                        <span
                            v-if="searching"
                            class="spinner"
                            style="width: 12px; height: 12px; border-width: 2px"
                        />
                        🔍 ค้นหาใน HOSxP
                    </button>
                </div>

                <!-- Lookup result (only shown when a lookup exists) -->
                <div
                    v-if="lookupResult"
                    class="form-grid"
                    style="max-width: 680px; margin-bottom: 10px"
                >
                    <label>icode :</label>
                    <input type="text" :value="lookupResult.icode" readonly />
                    <label>ชื่อ (HOSxP) :</label>
                    <input
                        type="text"
                        :value="lookupResult.name"
                        readonly
                        style="min-width: 360px"
                    />
                    <label>ชื่อย่อ :</label>
                    <input
                        type="text"
                        v-model="shortName"
                        placeholder="ชื่อย่อสำหรับแสดงในโปรแกรม"
                        style="width: 260px"
                    />

                    <!-- action row for the lookup -->
                    <div style="grid-column: 1 / -1; margin-top: 8px">
                        <div class="flex items-center gap-8">
                            <button class="btn btn-primary" @click="save">
                                🩺 💾 บันทึกหัตถการ
                            </button>
                            <button class="btn btn-ghost" @click="cancelLookup">
                                ยกเลิก
                            </button>
                        </div>
                    </div>
                </div>

                <div
                    v-if="msg"
                    class="alert mt-8"
                    :class="msgOk ? 'alert-success' : 'alert-error'"
                >
                    {{ msg }}
                </div>
                <div v-if="notFound" class="alert alert-warning mt-8">
                    ไม่พบ icode "{{ searchInput }}" ในตาราง nondrugitems
                </div>
            </div>
        </div>

        <!-- Single Saved list below -->
        <div class="group-box">
            <span class="group-box__title">🩺 หัตถการที่บันทึกแล้ว</span>
            <div style="padding-top: 10px">
                <div class="table-wrapper" style="max-height: 320px">
                    <!-- add small vertical spacing between rows and allow wrapping for long names -->
                    <table
                        class="data-table"
                        style="border-collapse: separate; border-spacing: 0 6px"
                    >
                        <thead>
                            <tr>
                                <th style="width: 40px">#</th>
                                <th style="width: 120px">icode</th>
                                <th style="min-width: 300px">ชื่อ (HOSxP)</th>
                                <th style="width: 140px">ชื่อย่อ</th>
                                <th style="width: 80px">ลบ</th>
                            </tr>
                        </thead>
                        <tbody>
                            <tr
                                v-for="(r, i) in list"
                                :key="r.id"
                                style="background: transparent"
                            >
                                <td style="vertical-align: middle">
                                    {{ i + 1 }}
                                </td>
                                <td
                                    style="width: 120px; vertical-align: middle"
                                >
                                    {{ r.icode }}
                                </td>
                                <td
                                    style="
                                        white-space: normal;
                                        word-break: break-word;
                                        max-width: 560px;
                                        vertical-align: middle;
                                    "
                                >
                                    {{ r.name }}
                                </td>
                                <td
                                    style="
                                        width: 140px;
                                        white-space: nowrap;
                                        vertical-align: middle;
                                    "
                                >
                                    {{ r.short_name }}
                                </td>
                                <td style="width: 80px; vertical-align: middle">
                                    <button
                                        class="btn btn-danger btn-sm"
                                        @click="del(r.id)"
                                        style="
                                            min-width: 36px;
                                            padding: 6px 8px;
                                        "
                                    >
                                        🗑️
                                    </button>
                                </td>
                            </tr>
                            <tr v-if="list.length === 0">
                                <td
                                    colspan="5"
                                    style="
                                        text-align: center;
                                        color: var(--text-gray);
                                        padding: 20px;
                                    "
                                >
                                    ยังไม่มีข้อมูล
                                </td>
                            </tr>
                        </tbody>
                    </table>
                </div>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from "vue";
import * as cmd from "@/composables/useCommands";
import type { ProcedureConfig, ItemLookup } from "@/types";

const searchInput = ref("");
const shortName = ref("");
const lookupResult = ref<ItemLookup | null>(null);
const list = ref<ProcedureConfig[]>([]);
const searching = ref(false);
const notFound = ref(false);
const msg = ref("");
const msgOk = ref(true);

onMounted(load);

async function load() {
    try {
        list.value = await cmd.getAllProcedures();
    } catch {}
}

async function search() {
    notFound.value = false;
    lookupResult.value = null;
    msg.value = "";
    if (!searchInput.value.trim()) return;
    searching.value = true;
    try {
        const r = await cmd.lookupProcedure(searchInput.value.trim());
        if (r) {
            lookupResult.value = r;
            shortName.value = "";
        } else {
            notFound.value = true;
        }
    } catch (e: any) {
        msg.value = String(e);
        msgOk.value = false;
    } finally {
        searching.value = false;
    }
}

function cancelLookup() {
    lookupResult.value = null;
    shortName.value = "";
    notFound.value = false;
    msg.value = "";
}

async function save() {
    if (!lookupResult.value) return;
    msg.value = "";
    try {
        await cmd.saveProcedure(
            lookupResult.value.icode,
            lookupResult.value.name,
            shortName.value.trim(),
        );
        msg.value = `บันทึกหัตถการ icode "${lookupResult.value.icode}" สำเร็จ`;
        msgOk.value = true;
        lookupResult.value = null;
        searchInput.value = "";
        shortName.value = "";
        await load();
    } catch (e: any) {
        msg.value = String(e);
        msgOk.value = false;
    }
}

async function del(id: number) {
    if (!confirm("ต้องการลบรายการนี้ใช่หรือไม่?")) return;
    try {
        await cmd.deleteProcedure(id);
        await load();
    } catch (e: any) {
        msg.value = String(e);
        msgOk.value = false;
    }
}
</script>
