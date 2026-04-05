<template>
    <div class="app-date-input">
        <select
            class="date-select month-select"
            v-model="selectedMonth"
            @change="onMonthChange"
        >
            <option v-for="(m, i) in thaiMonths" :key="i" :value="i + 1">
                {{ m }}
            </option>
        </select>
        <select
            class="date-select year-select"
            v-model="selectedYear"
            @change="onYearChange"
        >
            <option v-for="y in yearList" :key="y" :value="y">
                {{ y + 543 }}
            </option>
        </select>
        <input
            type="date"
            class="date-native"
            :value="modelValue"
            @change="onDateChange"
        />
    </div>
</template>

<script setup lang="ts">
import { ref, watch, computed } from "vue";

const props = defineProps<{ modelValue: string }>();
const emit = defineEmits<{ (e: "update:modelValue", v: string): void }>();

const thaiMonths = [
    "มกราคม",
    "กุมภาพันธ์",
    "มีนาคม",
    "เมษายน",
    "พฤษภาคม",
    "มิถุนายน",
    "กรกฎาคม",
    "สิงหาคม",
    "กันยายน",
    "ตุลาคม",
    "พฤศจิกายน",
    "ธันวาคม",
];

const currentYear = new Date().getFullYear();

const yearList = computed(() => {
    const years: number[] = [];
    for (let y = currentYear - 5; y <= currentYear + 5; y++) years.push(y);
    return years;
});

function parseDate(iso: string): { year: number; month: number; day: number } {
    if (!iso)
        return { year: currentYear, month: new Date().getMonth() + 1, day: 1 };
    const parts = iso.split("-").map(Number);
    return {
        year: parts[0] || currentYear,
        month: parts[1] || 1,
        day: parts[2] || 1,
    };
}

const parsed = parseDate(props.modelValue);
const selectedMonth = ref<number>(parsed.month);
const selectedYear = ref<number>(parsed.year);

watch(
    () => props.modelValue,
    (val) => {
        const p = parseDate(val);
        selectedMonth.value = p.month;
        selectedYear.value = p.year;
    },
);

function clampDay(year: number, month: number, day: number): number {
    const max = new Date(year, month, 0).getDate();
    return Math.min(day < 1 ? 1 : day, max);
}

function toISO(year: number, month: number, day: number): string {
    const mm = String(month).padStart(2, "0");
    const dd = String(day).padStart(2, "0");
    return `${year}-${mm}-${dd}`;
}

function onMonthChange() {
    const p = parseDate(props.modelValue);
    const day = clampDay(selectedYear.value, selectedMonth.value, p.day);
    emit(
        "update:modelValue",
        toISO(selectedYear.value, selectedMonth.value, day),
    );
}

function onYearChange() {
    const p = parseDate(props.modelValue);
    const day = clampDay(selectedYear.value, selectedMonth.value, p.day);
    emit(
        "update:modelValue",
        toISO(selectedYear.value, selectedMonth.value, day),
    );
}

function onDateChange(e: Event) {
    const val = (e.target as HTMLInputElement).value;
    if (!val) return;
    const p = parseDate(val);
    selectedMonth.value = p.month;
    selectedYear.value = p.year;
    emit("update:modelValue", val);
}
</script>

<style scoped>
.app-date-input {
    display: flex;
    justify-content: center;
    align-items: center;
    gap: 6px;
    font-family: var(--font);
    flex-wrap: nowrap;
    white-space: nowrap;
    width: 100%;
}

.date-select {
    height: 32px;
    border: 1px solid var(--border);
    border-radius: var(--radius);
    background: var(--bg-surface);
    color: var(--text-primary);
    font-family: var(--font);
    font-size: 13px;
    padding: 0 4px;
    cursor: pointer;
    outline: none;
    transition:
        border-color 0.15s,
        box-shadow 0.15s;
    appearance: auto;
}

.date-select:focus {
    border-color: var(--primary);
    box-shadow: 0 0 0 2px rgba(63, 81, 181, 0.15);
}

.month-select {
    width: 116px;
}

.year-select {
    width: 78px;
}

.date-native {
    height: 32px;
    border: 1px solid var(--border);
    border-radius: var(--radius);
    background: var(--bg-surface);
    color: var(--text-primary);
    font-family: var(--font);
    font-size: 13px;
    padding: 0 8px;
    outline: none;
    transition:
        border-color 0.15s,
        box-shadow 0.15s;
    width: 140px;
    cursor: pointer;
}

.date-native:focus {
    border-color: var(--primary);
    box-shadow: 0 0 0 2px rgba(63, 81, 181, 0.15);
}
</style>
