<template>
<div class="flex-col h-full" style="gap:8px; padding:12px 12px 8px">

    <!-- Filter Panel -->
    <div class="group-box" style="flex-shrink:0">
        <span class="group-box__title">🔍 ตัวกรองข้อมูล</span>
        <div class="flex items-center gap-12" style="padding-top:8px; flex-wrap:wrap">
            <span class="nowrap">วันที่ :</span>
            <input type="date" v-model="selectedDate" style="width:150px" />

            <span class="nowrap">ผู้ให้บริการ :</span>
            <select v-model="selectedProvider" style="width:240px">
                <option value="">ทั้งหมด</option>
                <option v-for="p in providers" :key="p.health_med_provider_id" :value="p.full_name">
                    {{ p.short_name || p.full_name }}
                </option>
            </select>

            <button class="btn btn-secondary" :disabled="loading" @click="loadData">
                <span v-if="loading" class="spinner" style="width:14px;height:14px;border-width:2px" />
                🔄 ดึงข้อมูล
            </button>

            <span class="flex-1" />

            <button class="btn btn-primary" :disabled="!hasSelection" @click="saveSelected">
                💾 บันทึกที่เลือก
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
                    <th style="width:80px">HN</th>
                    <th style="width:115px">CID</th>
                    <th style="width:90px">ชื่อ</th>
                    <th style="width:90px">นามสกุล</th>
                    <th style="width:80px">สิทธิ</th>
                    <th style="width:160px">อาการสำคัญ</th>
                    <th>การรักษา</th>
                    <th style="width:140px">ผู้ให้บริการ</th>
                    <th style="width:90px">ราคา (฿)</th>
                    <th style="width:110px">ค่าตอบแทน</th>
                    <th style="width:70px">บันทึก</th>
                </tr>
            </thead>
            <tbody>
                <tr v-for="(row, idx) in filteredRows" :key="row.vn" :class="{
                    'row-locked': row._locked,
                    'row-warning': row._multi_provider,
                }">
                    <td class="text-sm" style="text-align:center">{{ idx + 1 }}</td>
                    <td class="text-sm">{{ row.service_date }}</td>
                    <td class="text-sm">{{ row.hn }}</td>
                    <td class="text-sm">{{ row.cid }}</td>
                    <td>{{ row.first_name }}</td>
                    <td>{{ row.last_name }}</td>
                    <td>
                        <span :title="row.pttype_display_name">{{ shortPttype(row) }}</span>
                    </td>
                    <td :title="row.chief_complaint">{{ row.chief_complaint }}</td>
                    <td :title="row.item_names" style="max-width:200px">{{ shortProcedureDisplay(row) }}</td>
                    <td>
                        <span v-if="row._multi_provider"
                            :title="'⚠️ พบผู้ให้บริการมากกว่า 1 คน: ' + row.provider_names">
                            ⚠️ {{ shortProviderDisplay(row.provider_names) }}
                        </span>
                        <span v-else :title="row.provider_names">{{ shortProviderDisplay(row.provider_names) }}</span>
                    </td>
                    <td style="text-align:right">{{ row.total_sum_price.toFixed(2) }}</td>
                    <td>
                        <select v-if="!row._locked && !row._multi_provider" v-model="row._selected_payout"
                            class="payout-select" style="width:100%;font-size:12px">
                            <option :value="null">— เลือก —</option>
                            <option v-for="opt in payoutOptions" :key="opt.id" :value="opt.amount">
                                {{ opt.amount.toLocaleString() }} บาท
                            </option>
                        </select>
                        <span v-else-if="row._locked" class="text-sm text-gray">บันทึกแล้ว</span>
                        <span v-else class="text-sm text-gray">N/A</span>
                    </td>
                    <td style="text-align:center">
                        <input v-if="!row._locked && !row._multi_provider" type="checkbox" class="ppc-checkbox"
                            v-model="selectedVns" :value="row.vn" :disabled="row._selected_payout == null" />
                        <span v-else-if="row._locked" title="บันทึกไปแล้ว">🔒</span>
                        <span v-else title="ผู้ให้บริการมากกว่า 1 คน ไม่สามารถบันทึกได้">⛔</span>
                    </td>
                </tr>
                <tr v-if="filteredRows.length === 0 && !loading">
                    <td colspan="13" style="text-align:center;padding:32px;color:var(--text-gray)">
                        {{ rawRows.length === 0 ? 'กดปุ่ม "ดึงข้อมูล" เพื่อแสดงรายการ' : 'ไม่มีข้อมูลตามตัวกรองที่เลือก'
                        }}
                    </td>
                </tr>
            </tbody>
        </table>
    </div>

    <!-- Stats bar -->
    <div class="flex items-center gap-12" style="font-size:12px;color:var(--text-muted);flex-shrink:0">
        <span>พบ {{ rawRows.length }} รายการ</span>
        <span>|</span>
        <span>บันทึกแล้ว {{ lockedCount }} รายการ</span>
        <span>|</span>
        <span>เลือกแล้ว {{ selectedVns.length }} รายการ</span>
    </div>
</div>

<!-- Multi-provider warning popup (auto-show) -->
<div v-if="showMultiWarning" class="modal-overlay" @click.self="showMultiWarning = false">
    <div class="modal" style="width:480px">
        <div class="modal__header">
            <span class="modal__title">⚠️ พบผู้ให้บริการมากกว่า 1 คน</span>
            <button class="modal__close" @click="showMultiWarning = false">✕</button>
        </div>
        <div class="modal__body">
            <p style="margin-bottom:12px">พบ VN ต่อไปนี้มีผู้ให้บริการมากกว่า 1 คน กรุณาตรวจสอบข้อมูลใน HOSxP :</p>
            <ul style="padding-left:20px;line-height:1.8">
                <li v-for="vn in multiProviderVns" :key="vn" class="text-sm">{{ vn }}</li>
            </ul>
            <p class="alert alert-warning" style="margin-top:12px">
                แถวเหล่านี้จะไม่สามารถบันทึกค่าตอบแทนได้ จนกว่าจะแก้ไขข้อมูลใน HOSxP
            </p>
        </div>
        <div class="modal__footer">
            <button class="btn btn-primary" @click="showMultiWarning = false">รับทราบ</button>
        </div>
    </div>
</div>
</template>

<script setup lang="ts">
import { ref, computed, watch, onMounted } from 'vue'
import { useAppStore } from '@/stores/app'
import * as cmd from '@/composables/useCommands'
import type { PatientRow, PayoutOption, ProviderConfig, PttypeConfig, ProcedureConfig, DrugConfig } from '@/types'
import { useToast } from '@/composables/useToast'

const store = useAppStore()
const emit = defineEmits<{ 'pending-changed': [] }>()
const { show: showToast } = useToast()

// ── State ──────────────────────────────────────────────────────────────
const selectedDate = ref(todayStr())
const selectedProvider = ref('')
const rawRows = ref<PatientRow[]>([])
const lockedVns = ref<Set<string>>(new Set())
const selectedVns = ref<string[]>([])
const payoutOptions = ref<PayoutOption[]>([])
const providers = ref<ProviderConfig[]>([])
const pttypes = ref<PttypeConfig[]>([])
const procedures = ref<ProcedureConfig[]>([])
const drugs = ref<DrugConfig[]>([])
const loading = ref(false)
const error = ref('')
const showMultiWarning = ref(false)
const multiProviderVns = ref<string[]>([])

// ── Computed ───────────────────────────────────────────────────────────
const filteredRows = computed(() => {
    if (!selectedProvider.value) return rawRows.value
    return rawRows.value.filter(r =>
        r.provider_names.includes(selectedProvider.value)
    )
})

const lockedCount = computed(() => rawRows.value.filter(r => r._locked).length)
const hasSelection = computed(() => selectedVns.value.length > 0)

// ── Lifecycle ──────────────────────────────────────────────────────────
onMounted(async () => {
    await loadSettings()
})

// Refresh when pending data changes (from PendingExportTab deletion)
watch(() => store.pendingRefreshKey, () => {
    if (selectedDate.value) refreshLocks()
})

// ── Methods ────────────────────────────────────────────────────────────
function todayStr() {
    return new Date().toISOString().slice(0, 10)
}

async function loadSettings() {
    try {
        payoutOptions.value = await cmd.getPayoutOptions()
        providers.value = await cmd.getAllProviders()
        pttypes.value = await cmd.getAllPttypes()
        procedures.value = await cmd.getAllProcedures()
        drugs.value = await cmd.getAllDrugs()
    } catch (e: any) {
        console.error('loadSettings:', e)
    }
}

async function loadData() {
    if (!store.isConnected) {
        error.value = 'ยังไม่ได้เชื่อมต่อ HOSxP กรุณาตั้งค่าการเชื่อมต่อก่อน'
        return
    }
    error.value = ''
    loading.value = true
    selectedVns.value = []

    try {
        // Reload settings in case they changed
        await loadSettings()

        const [rows, locked] = await Promise.all([
            cmd.fetchPatientData(selectedDate.value),
            cmd.getLockedVns(selectedDate.value),
        ])

        lockedVns.value = new Set(locked)

        // Annotate rows
        const annotated: PatientRow[] = rows.map(r => {
            const providerCount = (r.provider_names || '')
                .split(', ')
                .filter(p => p && p !== 'PROVIDER_ID_NOT_DEFINED').length
            return {
                ...r,
                _locked: lockedVns.value.has(r.vn),
                _multi_provider: providerCount > 1,
                _selected_payout: null,
            }
        })

        rawRows.value = annotated

        // Check for multi-provider VNs
        const multiVns = annotated.filter(r => r._multi_provider).map(r => r.vn)
        if (multiVns.length > 0) {
            multiProviderVns.value = multiVns
            showMultiWarning.value = true
        }

    } catch (e: any) {
        error.value = String(e)
    } finally {
        loading.value = false
    }
}

async function refreshLocks() {
    try {
        const locked = await cmd.getLockedVns(selectedDate.value)
        lockedVns.value = new Set(locked)
        rawRows.value = rawRows.value.map(r => ({
            ...r,
            _locked: lockedVns.value.has(r.vn),
        }))
        // Deselect any newly locked VNs
        selectedVns.value = selectedVns.value.filter(vn => !lockedVns.value.has(vn))
    } catch (e) { console.error('refreshLocks:', e) }
}

async function saveSelected() {
    if (selectedVns.value.length === 0) return
    error.value = ''

    const rowsToSave = rawRows.value.filter(r =>
        selectedVns.value.includes(r.vn) && r._selected_payout != null
    )

    if (rowsToSave.length === 0) {
        error.value = 'กรุณาเลือกค่าตอบแทนสำหรับทุกรายการที่ต้องการบันทึก'
        return
    }

    try {
        for (const row of rowsToSave) {
            await cmd.upsertPending({
                visit_date: row.service_date || selectedDate.value,
                vn: row.vn,
                hn: row.hn,
                cid: row.cid,
                first_name: row.first_name,
                last_name: row.last_name,
                gender: row.gender,
                age: row.age,
                rights: resolveRightsShort(row),
                symptoms: row.chief_complaint,
                procedure: resolveProcedureShort(row),
                therapist: cleanProviderName(row.provider_names),
                total_revenue: row.total_sum_price,
                payout_amount: row._selected_payout!,
            })
        }

        showToast(`บันทึก ${rowsToSave.length} รายการสำเร็จ`, 'success')
        selectedVns.value = []
        await refreshLocks()
        emit('pending-changed')
        store.triggerPendingRefresh()
    } catch (e: any) {
        error.value = String(e)
    }
}

// ── Helpers ────────────────────────────────────────────────────────────
function shortPttype(row: PatientRow): string {
    const found = pttypes.value.find(
        p => p.hipdata_code === row.pttype_hipdata_code
    )
    if (found?.short_name) return found.short_name
    return row.pttype_display_name.length > 10
        ? row.pttype_display_name.slice(0, 10) + '…'
        : row.pttype_display_name
}

function resolveRightsShort(row: PatientRow): string {
    const found = pttypes.value.find(p => p.hipdata_code === row.pttype_hipdata_code)
    return found?.short_name || row.pttype_display_name
}

// Show short name for provider, full name in tooltip
function shortProviderDisplay(names: string): string {
    return (names || '')
        .split(', ')
        .map(name => {
            if (!name || name === 'PROVIDER_ID_NOT_DEFINED') return null
            const found = providers.value.find(p => p.full_name === name)
            return found?.short_name || name
        })
        .filter((n): n is string => n !== null)
        .join(', ')
}

// Show short name for procedures/drugs by icode, full item_names in tooltip
function shortProcedureDisplay(row: PatientRow): string {
    const icodes = (row.all_icodes || '').split(', ').filter(Boolean)
    if (icodes.length === 0) return row.item_names
    return icodes.map(icode => {
        const proc = procedures.value.find(p => p.icode === icode)
        if (proc?.short_name) return proc.short_name
        const drug = drugs.value.find(d => d.icode === icode)
        if (drug?.short_name) return drug.short_name
        return icode
    }).join(', ')
}

function resolveProcedureShort(row: PatientRow): string {
    return row.item_names
}

function cleanProviderName(names: string): string {
    return (names || '')
        .split(', ')
        .filter(n => n && n !== 'PROVIDER_ID_NOT_DEFINED')
        .join(', ')
}
</script>

<style scoped>
.payout-select {
    height: 26px;
    border-radius: 3px;
}
</style>
