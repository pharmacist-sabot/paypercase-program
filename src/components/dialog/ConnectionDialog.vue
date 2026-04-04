<template>
<div class="modal-overlay" @click.self="$emit('close')">
    <div class="modal" style="width:600px">
        <div class="modal__header">
            <span class="modal__title">🔌 การเชื่อมต่อฐานข้อมูล HOSxP</span>
            <button class="modal__close" @click="$emit('close')">✕</button>
        </div>

        <div class="modal__body" style="min-width:540px">
            <!-- Title -->
            <p style="color:var(--text-gray);font-size:12px;margin-bottom:16px">
                กรอกข้อมูลการเชื่อมต่อ MySQL ของระบบ HOSxP เพื่อดึงข้อมูลผู้ป่วย
            </p>

            <!-- Form -->
            <div class="group-box" style="margin-bottom:16px">
                <span class="group-box__title">ข้อมูลการเชื่อมต่อ MySQL (HOSxP)</span>
                <div class="form-grid" style="padding-top:12px">
                    <label>Host / IP :</label>
                    <input type="text" v-model="form.host" placeholder="เช่น 192.168.1.100 หรือ localhost"
                        style="width:300px" autocapitalize="off" autocomplete="off" autocorrect="off"
                        spellcheck="false" />

                    <label>Port :</label>
                    <input type="number" v-model.number="form.port" min="1" max="65535" style="width:110px"
                        autocomplete="off" />

                    <label>Database :</label>
                    <input type="text" v-model="form.database_name" placeholder="เช่น hospdb" style="width:300px"
                        autocapitalize="off" autocomplete="off" autocorrect="off" spellcheck="false" />

                    <label>Username :</label>
                    <input type="text" v-model="form.username" placeholder="ชื่อผู้ใช้ MySQL" style="width:300px"
                        autocapitalize="off" autocomplete="off" autocorrect="off" spellcheck="false" />

                    <label>Password :</label>
                    <input type="password" v-model="form.password" placeholder="รหัสผ่าน MySQL" style="width:300px"
                        autocomplete="off" />
                </div>
            </div>

            <!-- Result -->
            <div v-if="resultMsg" class="alert" :class="resultOk ? 'alert-success' : 'alert-error'"
                style="white-space:pre-wrap;font-size:12px">{{ resultMsg }}</div>

            <!-- Loading -->
            <div v-if="testing" class="flex items-center gap-8" style="color:var(--text-muted);font-size:13px">
                <span class="spinner" />
                กำลังทดสอบการเชื่อมต่อ...
            </div>
        </div>

        <div class="modal__footer">
            <button class="btn btn-ghost" :disabled="testing" @click="testOnly">🔍 ทดสอบการเชื่อมต่อ</button>
            <button class="btn btn-warning" :disabled="testing" @click="saveAndConnect">💾 บันทึกและเชื่อมต่อ</button>
            <button class="btn btn-secondary" @click="$emit('close')">ปิด</button>
        </div>
    </div>
</div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useAppStore } from '@/stores/app'
import * as cmd from '@/composables/useCommands'
import type { HosxpSettings } from '@/types'

const emit = defineEmits<{ close: []; connected: [host: string] }>()
const store = useAppStore()

const form = ref<HosxpSettings>({
    host: '', port: 3306, database_name: '', username: '', password: '',
})
const testing = ref(false)
const resultMsg = ref('')
const resultOk = ref(false)

onMounted(async () => {
    try {
        const s = await cmd.getHosxpSettings()
        form.value = { ...s }
    } catch { }
})

async function testOnly() {
    testing.value = true; resultMsg.value = ''
    try {
        const result = await cmd.testHosxpConnection(form.value)
        resultOk.value = result.ok
        resultMsg.value = result.message
    } catch (e: any) {
        resultOk.value = false
        resultMsg.value = String(e)
    } finally {
        testing.value = false
    }
}

async function saveAndConnect() {
    testing.value = true; resultMsg.value = ''
    try {
        const result = await cmd.connectHosxp(form.value)
        resultOk.value = result.ok
        resultMsg.value = result.message
        if (result.ok) {
            store.setConnected(form.value.host, result.message)
            emit('connected', form.value.host)
            setTimeout(() => emit('close'), 1200)
        }
    } catch (e: any) {
        resultOk.value = false
        resultMsg.value = String(e)
    } finally {
        testing.value = false
    }
}
</script>
