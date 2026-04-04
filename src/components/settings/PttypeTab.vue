<template>
  <div>
    <!-- Search group -->
    <div class="group-box mb-12">
      <span class="group-box__title">ค้นหาสิทธิการรักษาใน HOSxP</span>
      <div style="padding-top:12px">
        <div class="flex items-center gap-8 mb-8">
          <label>hipdata_code :</label>
          <input type="text" v-model="searchInput" placeholder="เช่น UCS, OFC, LGO" style="width:180px"
            @keydown.enter="search" />
          <button class="btn btn-secondary" :disabled="searching" @click="search">
            <span v-if="searching" class="spinner" style="width:12px;height:12px;border-width:2px" />
            🔍 ค้นหาใน HOSxP
          </button>
        </div>

        <div v-if="lookupResult" class="form-grid" style="max-width:460px;margin-bottom:10px">
          <label>hipdata_code :</label>
          <input type="text" :value="lookupResult.hipdata_code" readonly />
          <label>ชื่อย่อ :</label>
          <input type="text" v-model="shortName" placeholder="ชื่อย่อสำหรับแสดงในโปรแกรม" style="width:220px" />
        </div>

        <div v-if="lookupResult" class="flex gap-8">
          <button class="btn btn-primary" @click="save">💾 บันทึกสิทธิการรักษา</button>
        </div>

        <div v-if="msg" class="alert mt-8" :class="msgOk ? 'alert-success' : 'alert-error'">{{ msg }}</div>
        <div v-if="notFound" class="alert alert-warning mt-8">ไม่พบ hipdata_code "{{ searchInput }}" ใน HOSxP</div>
      </div>
    </div>

    <!-- Saved list -->
    <div class="group-box">
      <span class="group-box__title">สิทธิการรักษาที่บันทึกแล้ว</span>
      <div style="padding-top:10px">
        <div class="flex items-center gap-8 mb-8">
          <label style="font-size:12px">แสดงเฉพาะ hipdata_code :</label>
          <select v-model="filterCode" style="width:180px;font-size:12px">
            <option value="">ทั้งหมด</option>
            <option v-for="c in uniqueCodes" :key="c" :value="c">{{ c }}</option>
          </select>
        </div>
        <div class="table-wrapper" style="max-height:260px">
          <table class="data-table">
            <thead>
              <tr>
                <th style="width:40px">#</th>
                <th>hipdata_code</th>
                <th>ชื่อย่อ</th>
                <th style="width:60px">ลบ</th>
              </tr>
            </thead>
            <tbody>
              <tr v-for="(r, i) in filteredList" :key="r.id">
                <td>{{ i+1 }}</td>
                <td>{{ r.hipdata_code }}</td>
                <td>{{ r.short_name }}</td>
                <td><button class="btn btn-danger btn-sm" @click="del(r.id)">🗑️</button></td>
              </tr>
              <tr v-if="filteredList.length === 0">
                <td colspan="4" style="text-align:center;color:var(--text-gray);padding:20px">ยังไม่มีข้อมูล</td>
              </tr>
            </tbody>
          </table>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import * as cmd from '@/composables/useCommands'
import type { PttypeConfig, PttypeLookup } from '@/types'

const searchInput = ref('')
const shortName = ref('')
const lookupResult = ref<PttypeLookup | null>(null)
const list = ref<PttypeConfig[]>([])
const filterCode = ref('')
const searching = ref(false)
const notFound = ref(false)
const msg = ref(''); const msgOk = ref(true)

const uniqueCodes = computed(() => [...new Set(list.value.map(r => r.hipdata_code))])
const filteredList = computed(() =>
  filterCode.value ? list.value.filter(r => r.hipdata_code === filterCode.value) : list.value
)

onMounted(load)

async function load() {
  try { list.value = await cmd.getAllPttypes() } catch {}
}

async function search() {
  notFound.value = false; lookupResult.value = null; msg.value = ''
  if (!searchInput.value.trim()) return
  searching.value = true
  try {
    const r = await cmd.lookupPttype(searchInput.value.trim())
    if (r) { lookupResult.value = r; shortName.value = '' }
    else notFound.value = true
  } catch (e: any) { msg.value = String(e); msgOk.value = false }
  finally { searching.value = false }
}

async function save() {
  if (!lookupResult.value) return
  msg.value = ''
  try {
    await cmd.savePttype(
      lookupResult.value.pttype,
      lookupResult.value.name,
      lookupResult.value.pcode,
      lookupResult.value.hipdata_code,
      shortName.value.trim(),
    )
    msg.value = `บันทึกสิทธิ "${lookupResult.value.hipdata_code}" สำเร็จ`
    msgOk.value = true
    lookupResult.value = null; searchInput.value = ''; shortName.value = ''
    await load()
  } catch (e: any) { msg.value = String(e); msgOk.value = false }
}

async function del(id: number) {
  if (!confirm('ต้องการลบรายการนี้ใช่หรือไม่?')) return
  try { await cmd.deletePttype(id); await load() }
  catch (e: any) { msg.value = String(e); msgOk.value = false }
}
</script>
