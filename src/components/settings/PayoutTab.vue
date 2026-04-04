<template>
  <div>
    <!-- Add group -->
    <div class="group-box mb-12">
      <span class="group-box__title">เพิ่มตัวเลือกค่าตอบแทน</span>
      <div style="padding-top:12px">
        <div class="flex items-center gap-8 mb-8">
          <label>จำนวนเงิน (บาท) :</label>
          <input type="number" v-model.number="newAmount" placeholder="เช่น 125" style="width:120px" min="0" step="0.01" />
          <label>ป้ายชื่อ :</label>
          <input type="text" v-model="newLabel" placeholder="เช่น ค่านวด 125 บาท" style="width:200px" />
          <button class="btn btn-primary" @click="add">➕ เพิ่ม</button>
        </div>
        <div v-if="msg" class="alert mt-8" :class="msgOk ? 'alert-success' : 'alert-error'">{{ msg }}</div>
      </div>
    </div>

    <!-- Saved list -->
    <div class="group-box">
      <span class="group-box__title">ตัวเลือกค่าตอบแทนที่ตั้งค่าไว้</span>
      <div style="padding-top:10px">
        <p class="note mb-8" style="font-size:11px;color:var(--text-gray)">
          ตัวเลือกเหล่านี้จะปรากฏเป็น checkbox ในหน้าแสดงข้อมูลผู้ป่วย
        </p>
        <div class="table-wrapper" style="max-height:280px">
          <table class="data-table">
            <thead>
              <tr>
                <th style="width:40px">#</th>
                <th>จำนวนเงิน (บาท)</th>
                <th>ป้ายชื่อ</th>
                <th style="width:60px">ลบ</th>
              </tr>
            </thead>
            <tbody>
              <tr v-for="(r, i) in list" :key="r.id">
                <td>{{ i+1 }}</td>
                <td>{{ r.amount.toLocaleString() }}</td>
                <td>{{ r.label }}</td>
                <td><button class="btn btn-danger btn-sm" @click="del(r.id)">🗑️</button></td>
              </tr>
              <tr v-if="list.length === 0">
                <td colspan="4" style="text-align:center;color:var(--text-gray);padding:20px">ยังไม่มีตัวเลือก</td>
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
import type { PayoutOption } from '@/types'

const newAmount = ref<number | null>(null)
const newLabel = ref('')
const list = ref<PayoutOption[]>([])
const msg = ref(''); const msgOk = ref(true)

onMounted(load)

async function load() {
  try { list.value = await cmd.getPayoutOptions() } catch {}
}

async function add() {
  msg.value = ''
  if (!newAmount.value || newAmount.value <= 0) { msg.value = 'กรุณาระบุจำนวนเงิน'; msgOk.value = false; return }
  if (!newLabel.value.trim()) { msg.value = 'กรุณาระบุป้ายชื่อ'; msgOk.value = false; return }
  try {
    await cmd.addPayoutOption(newAmount.value, newLabel.value.trim())
    msg.value = `เพิ่มตัวเลือก "${newLabel.value.trim()}" สำเร็จ`
    msgOk.value = true
    newAmount.value = null; newLabel.value = ''
    await load()
  } catch (e: any) { msg.value = String(e); msgOk.value = false }
}

async function del(id: number) {
  if (!confirm('ต้องการลบตัวเลือกนี้ใช่หรือไม่?')) return
  try { await cmd.deletePayoutOption(id); await load() }
  catch (e: any) { msg.value = String(e); msgOk.value = false }
}
</script>
