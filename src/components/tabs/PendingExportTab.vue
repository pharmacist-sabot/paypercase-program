<template>
<div class="flex-col h-full" style="gap:8px; padding:12px 12px 8px">

    <!-- Filter / Action Panel -->
    <div class="group-box" style="flex-shrink:0">
        <span class="group-box__title">📋 ข้อมูลรอการส่งออก</span>
        <div class="flex items-center gap-12" style="padding-top:8px; flex-wrap:wrap">
            <span class="nowrap">วันที่ :</span>
            <input type="date" v-model="selectedDate" style="width:150px" />

            <span class="nowrap">ผู้ให้บริการ :</span>
            <select v-model="selectedProvider" style="width:220px">
                <option value="">ทั้งหมด</option>
                <option v-for="p in providers" :key="p.id" :value="p.full_name">
                    {{ p.short_name || p.full_name }}
                </option>
            </select>

            <button class="btn btn-secondary" @click="search">🔍 ค้นหา</button>

            <span class="flex-1" />

            <button class="btn btn-warning" :disabled="filteredRows.length === 0" @click="doExport">
                📤 ส่งออก CSV
            </button>
        </div>
    </div>

    <!-- Alert -->
    <div v-if="error" class="alert alert-error" style="flex-shrink:0">⚠️ {{ error }}</div>

    <!-- Table -->
    <div class="table-wrapper flex-1 overflow-auto">
        <table class="data-table">
            <thead>
                <tr>
                    <th style="width:40px">#</th>
                    <th style="width:90px">วันที่</th>
                    <th style="width:75px">HN</th>
                    <th style="width:110px">CID</th>
                    <th style="width:85px">ชื่อ</th>
                    <th style="width:85px">นามสกุล</th>
                    <th style="width:110px">สิทธิ</th>
                    <th style="width:150px">อาการสำคัญ</th>
                    <th>การรักษา</th>
                    <th style="width:130px">ผู้ให้บริการ</th>
                    <th style="width:100px">รายได้รวม (฿)</th>
                    <th style="width:100px">ค่าตอบแทน (฿)</th>
                    <th style="width:60px">ลบ</th>
                </tr>
            </thead>
            <tbody>
                <tr v-for="(row, idx) in filteredRows" :key="row.id">
                    <td style="text-align:center" class="text-sm">{{ idx + 1 }}</td>
                    <td class="text-sm">{{ row.visit_date }}</td>
                    <td class="text-sm">{{ row.hn }}</td>
                    <td class="text-sm">{{ row.cid }}</td>
                    <td>{{ row.first_name }}</td>
                    <td>{{ row.last_name }}</td>

                    <!-- Rights: show short name, tooltip is full/official name -->
                    <td :title="rightsTooltip(row.rights)">{{ shortRightsDisplay(row.rights) }}</td>

                    <td :title="row.symptoms">{{ row.symptoms }}</td>

                    <!-- Procedure displayed as short names, tooltip = full procedure string -->
                    <td :title="row.procedure">{{ shortProcedure(row.procedure) }}</td>

                    <!-- Therapist shown as short names, tooltip = full therapist string -->
                    <td :title="row.therapist">{{ shortTherapist(row.therapist) }}</td>

                    <td style="text-align:right">{{ row.total_revenue.toFixed(2) }}</td>
                    <td style="text-align:right;font-weight:bold;color:var(--highlight)">
                        {{ row.payout_amount.toFixed(2) }}
                    </td>
                    <td style="text-align:center">
                        <button class="btn btn-danger btn-sm" @click="deleteRow(row.id)"
                            title="ลบรายการนี้">🗑️</button>
                    </td>
                </tr>
                <tr v-if="filteredRows.length === 0">
                    <td colspan="13" style="text-align:center;padding:32px;color:var(--text-gray)">
                        กดปุ่ม "ค้นหา" เพื่อแสดงรายการ
                    </td>
                </tr>
            </tbody>
        </table>
    </div>

    <!-- Stats -->
    <div class="flex items-center gap-12" style="font-size:12px;color:var(--text-muted);flex-shrink:0">
        <span>พบ {{ filteredRows.length }} รายการ</span>
        <span v-if="filteredRows.length > 0">|</span>
        <span v-if="filteredRows.length > 0">
            รายได้รวม: {{ totalRevenue.toFixed(2) }} ฿
        </span>
        <span v-if="filteredRows.length > 0">|</span>
        <span v-if="filteredRows.length > 0">
            ค่าตอบแทนรวม: {{ totalPayout.toFixed(2) }} ฿
        </span>
    </div>
</div>
</template>

<script setup lang="ts">
import { ref, computed, watch, onMounted } from 'vue'
import { save } from '@tauri-apps/plugin-dialog'
import * as cmd from '@/composables/useCommands'
import { useAppStore } from '@/stores/app'
import { useToast } from '@/composables/useToast'
import type { PendingRow, ProviderConfig, ProcedureConfig, DrugConfig, PttypeConfig } from '@/types'

const store = useAppStore()
const emit = defineEmits<{ 'record-deleted': [] }>()

const selectedDate = ref(todayStr())
const selectedProvider = ref('')
const allRows = ref<PendingRow[]>([])

const providers = ref<ProviderConfig[]>([])
const procedures = ref<ProcedureConfig[]>([])
const drugs = ref<DrugConfig[]>([])
const pttypes = ref<PttypeConfig[]>([])

const error = ref('')
const { show: showToast } = useToast()

function todayStr() { return new Date().toISOString().slice(0, 10) }

const filteredRows = computed(() => {
    if (!selectedProvider.value) return allRows.value
    return allRows.value.filter(r => r.therapist.includes(selectedProvider.value))
})

const totalRevenue = computed(() => filteredRows.value.reduce((s, r) => s + r.total_revenue, 0))
const totalPayout = computed(() => filteredRows.value.reduce((s, r) => s + r.payout_amount, 0))

onMounted(async () => {
    try {
        // Load everything used to map to short names
        const results = await Promise.allSettled([
            cmd.getAllProviders(),
            cmd.getAllProcedures(),
            cmd.getAllDrugs(),
            cmd.getAllPttypes(),
        ])

        if (results[0].status === 'fulfilled') providers.value = results[0].value as ProviderConfig[]
        if (results[1].status === 'fulfilled') procedures.value = results[1].value as ProcedureConfig[]
        if (results[2].status === 'fulfilled') drugs.value = results[2].value as DrugConfig[]
        if (results[3].status === 'fulfilled') pttypes.value = results[3].value as PttypeConfig[]
    } catch (e) {
        // non-fatal: mapping will fall back to raw strings
        console.error('load lookups:', e)
    }
})

// Refresh when pending data changes
watch(() => store.pendingRefreshKey, () => search())

// Map provider full name to short name
function shortTherapist(names: string): string {
    return (names || '').split(', ').map(name => {
        const found = providers.value.find(p => p.full_name === name || String(p.health_med_provider_id) === name)
        return found?.short_name || name
    }).filter(Boolean).join(', ')
}

// Map stored item names (full) to short names (procedure/drug). We try procedures (by name) first then drugs.
function shortProcedure(procedure: string): string {
    if (!procedure) return ''
    return procedure.split(', ').map(name => {
        const proc = procedures.value.find(p => p.name === name || p.icode === name)
        if (proc?.short_name) return proc.short_name
        const drug = drugs.value.find(d => d.name === name || d.icode === name)
        if (drug?.short_name) return drug.short_name
        return name
    }).filter(Boolean).join(', ')
}

// Rights: show short name (if known) but tooltip full name
function shortRightsDisplay(rights: string): string {
    if (!rights) return ''
    // try to match by short_name, name, hipdata_code or pttype
    const found = pttypes.value.find(p =>
        p.short_name === rights ||
        p.name === rights ||
        p.hipdata_code === rights ||
        p.pttype === rights
    )
    return found?.short_name || rights
}
function rightsTooltip(rights: string): string {
    const found = pttypes.value.find(p =>
        p.short_name === rights ||
        p.name === rights ||
        p.hipdata_code === rights ||
        p.pttype === rights
    )
    return found?.name || rights
}

// CSV escape helper
function csvEscape(s: string): string {
    if (s == null) return ''
    const v = String(s)
    if (v.includes(',') || v.includes('"') || v.includes('\n')) {
        return `"${v.replace(/"/g, '""')}"`
    }
    return v
}

async function search() {
    error.value = ''
    try {
        allRows.value = await cmd.getPendingExport(selectedDate.value, selectedDate.value)
    } catch (e: any) {
        error.value = String(e)
    }
}

async function deleteRow(id: number) {
    if (!confirm('ต้องการลบรายการนี้ออกจากข้อมูลรอการส่งออกใช่หรือไม่?\n(การลบจะปลดล็อกรายการในหน้าแสดงข้อมูล)')) return
    try {
        await cmd.deletePendingById(id)
        allRows.value = allRows.value.filter(r => r.id !== id)
        emit('record-deleted')
        store.triggerPendingRefresh()
        showToast('ลบรายการสำเร็จ', 'success')
    } catch (e: any) {
        error.value = String(e)
    }
}

async function doExport() {
    error.value = ''
    try {
        const defaultName = `PayPerCase_${selectedDate.value}.csv`
        const filePath = await save({
            defaultPath: defaultName,
            filters: [{ name: 'CSV', extensions: ['csv'] }],
        })
        if (!filePath) return

        // Build CSV with short names matching what's displayed
        const headers = ['visit_date', 'hn', 'cid', 'first_name', 'last_name', 'gender', 'age',
            'rights', 'symptoms', 'procedure', 'therapist', 'total_revenue', 'payout_amount']

        const dataRows = filteredRows.value.map(r => [
            r.visit_date,
            r.hn,
            r.cid,
            r.first_name,
            r.last_name,
            r.gender,
            r.age?.toString() ?? '',
            shortRightsDisplay(r.rights),
            r.symptoms,
            shortProcedure(r.procedure),
            shortTherapist(r.therapist),
            r.total_revenue.toFixed(2),
            r.payout_amount.toFixed(2),
        ])

        const csvLines = [headers.join(','), ...dataRows.map(row => row.map(csvEscape).join(','))]
        const csvContent = '\uFEFF' + csvLines.join('\n') // UTF-8 BOM for Excel

        const { writeTextFile } = await import('@tauri-apps/plugin-fs')
        await writeTextFile(filePath, csvContent)
        showToast(`ส่งออกสำเร็จ ${filteredRows.value.length} รายการ → ${filePath}`, 'success', 5000)
    } catch (e: any) {
        error.value = String(e)
    }
}
</script>

<style scoped>
/* small adjustments specific to pending export tab */
.table-wrapper {
    min-height: 120px;
}

.data-table td[title] {
    cursor: help;
    text-decoration: underline dotted rgba(0, 0, 0, 0.15);
    text-underline-offset: 2px;
}
</style>
