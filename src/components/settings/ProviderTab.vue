<template>
<div>
    <!-- Search group -->
    <div class="group-box mb-12">
        <span class="group-box__title">ค้นหาพนักงานใน HOSxP</span>
        <div style="padding-top:12px">
            <div class="flex items-center gap-8 mb-8">
                <label>health_med_provider_id :</label>
                <input type="number" v-model.number="searchId" placeholder="เช่น 5" style="width:120px"
                    @keydown.enter="search" />
                <button class="btn btn-secondary" :disabled="searching" @click="search">
                    <span v-if="searching" class="spinner" style="width:12px;height:12px;border-width:2px" />
                    🔍 ค้นหาใน HOSxP
                </button>
            </div>

            <div v-if="lookupResult" class="form-grid" style="max-width:460px;margin-bottom:10px">
                <label>provider_id :</label>
                <input type="text" :value="lookupResult.health_med_provider_id" readonly />
                <label>ชื่อ-นามสกุล :</label>
                <input type="text" :value="lookupResult.health_med_provider_full_name" readonly
                    style="min-width:240px" />
                <label>ชื่อย่อ :</label>
                <input type="text" v-model="shortName" placeholder="ชื่อย่อสำหรับแสดงในโปรแกรม" style="width:220px" />
            </div>

            <div v-if="lookupResult" class="flex gap-8">
                <button class="btn btn-primary" @click="save">💾 บันทึกพนักงาน</button>
            </div>

            <div v-if="msg" class="alert mt-8" :class="msgOk ? 'alert-success' : 'alert-error'">{{ msg }}</div>
            <div v-if="notFound" class="alert alert-warning mt-8">ไม่พบ provider_id "{{ searchId }}" ใน HOSxP</div>
        </div>
    </div>

    <!-- Saved list -->
    <div class="group-box">
        <span class="group-box__title">พนักงานที่บันทึกแล้ว</span>
        <div style="padding-top:10px">
            <div class="table-wrapper" style="max-height:280px">
                <table class="data-table">
                    <thead>
                        <tr>
                            <th style="width:40px">#</th>
                            <th>provider_id (HOSxP)</th>
                            <th>ชื่อ-นามสกุล</th>
                            <th>ชื่อย่อ</th>
                            <th style="width:60px">ลบ</th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr v-for="(r, i) in list" :key="r.id">
                            <td>{{ i + 1 }}</td>
                            <td>{{ r.health_med_provider_id }}</td>
                            <td>{{ r.full_name }}</td>
                            <td>{{ r.short_name }}</td>
                            <td><button class="btn btn-danger btn-sm" @click="del(r.id)">🗑️</button></td>
                        </tr>
                        <tr v-if="list.length === 0">
                            <td colspan="5" style="text-align:center;color:var(--text-gray);padding:20px">ยังไม่มีข้อมูล
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
import { ref, onMounted } from 'vue'
import * as cmd from '@/composables/useCommands'
import type { ProviderConfig, ProviderLookup } from '@/types'

const searchId = ref<number | null>(null)
const lookupResult = ref<ProviderLookup | null>(null)
const shortName = ref('')
const list = ref<ProviderConfig[]>([])
const searching = ref(false)
const notFound = ref(false)
const msg = ref(''); const msgOk = ref(true)

onMounted(load)

async function load() {
    try { list.value = await cmd.getAllProviders() } catch { }
}

async function search() {
    notFound.value = false; lookupResult.value = null; msg.value = ''; shortName.value = ''
    if (!searchId.value) return
    searching.value = true
    try {
        const r = await cmd.lookupProvider(searchId.value)
        if (r) { lookupResult.value = r }
        else notFound.value = true
    } catch (e: any) { msg.value = String(e); msgOk.value = false }
    finally { searching.value = false }
}

async function save() {
    if (!lookupResult.value) return
    msg.value = ''
    try {
        await cmd.saveProvider(lookupResult.value.health_med_provider_id, lookupResult.value.health_med_provider_full_name, shortName.value.trim())
        msg.value = `บันทึกพนักงาน "${lookupResult.value.health_med_provider_full_name}" สำเร็จ`
        msgOk.value = true
        lookupResult.value = null; searchId.value = null; shortName.value = ''
        await load()
    } catch (e: any) { msg.value = String(e); msgOk.value = false }
}

async function del(id: number) {
    if (!confirm('ต้องการลบรายการนี้ใช่หรือไม่?')) return
    try { await cmd.deleteProvider(id); await load() }
    catch (e: any) { msg.value = String(e); msgOk.value = false }
}
</script>
