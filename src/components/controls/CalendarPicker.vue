<template>
    <div class="calendar-picker">
        <div class="input-row">
            <input
                ref="inputEl"
                class="cp-input"
                :placeholder="placeholder"
                :readonly="!allowInput"
                :value="modelValue"
                @keydown.enter.prevent="open"
                aria-label="เลือกวันที่"
            />
            <button
                type="button"
                class="btn-ghost cp-icon"
                @click="open"
                :aria-expanded="isOpen"
                aria-haspopup="dialog"
                title="เปิดปฏิทิน"
            >
                <svg
                    width="18"
                    height="18"
                    viewBox="0 0 24 24"
                    fill="none"
                    aria-hidden="true"
                >
                    <rect
                        x="3"
                        y="5"
                        width="18"
                        height="16"
                        rx="2"
                        stroke="currentColor"
                        stroke-width="1.2"
                    />
                    <path
                        d="M16 3v4M8 3v4"
                        stroke="currentColor"
                        stroke-width="1.2"
                        stroke-linecap="round"
                    />
                </svg>
            </button>
        </div>
    </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onBeforeUnmount, watch } from "vue";
import flatpickr from "flatpickr";
import "flatpickr/dist/flatpickr.min.css";
import { Thai } from "flatpickr/dist/l10n/th.js";

flatpickr.localize(Thai);

// Props & emits
const props = defineProps({
    modelValue: { type: String, default: "" }, // ISO YYYY-MM-DD
    placeholder: { type: String, default: "YYYY-MM-DD" },
    allowInput: { type: Boolean, default: false },
    yearOffsetRange: { type: Number, default: 10 }, // +/- years for dropdown
});

const emit = defineEmits<{
    (e: "update:modelValue", v: string): void;
}>();

const inputEl = ref<HTMLInputElement | null>(null);
let fpInstance: any = null;
const isOpen = ref(false);

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

function open() {
    if (fpInstance) {
        try {
            fpInstance.open();
        } catch {
            // ignore
        }
    }
}

onMounted(() => {
    if (!inputEl.value) return;

    fpInstance = flatpickr(inputEl.value!, {
        locale: Thai,
        dateFormat: "Y-m-d",
        allowInput: props.allowInput,
        clickOpens: false, // open programmatically
        defaultDate: props.modelValue || undefined,
        showMonths: 1,
        onOpen() {
            isOpen.value = true;
        },
        onClose() {
            isOpen.value = false;
        },
        onChange(selectedDates: any, dateStr: string) {
            emit("update:modelValue", dateStr);
            if (inputEl.value) inputEl.value.value = dateStr;
        },
        onReady(selectedDates: any, dateStr: string, instance: any) {
            // Inject month & year dropdowns into header
            const cal = instance.calendarContainer as HTMLElement | null;
            if (!cal) return;

            const header = cal.querySelector(".flatpickr-months");
            if (!header) return;
            // ensure header is positioned so absolute centering works
            (header as HTMLElement).style.position = "relative";
            // hide built-in month title to avoid duplicate month display
            const curMonth = cal.querySelector(
                ".flatpickr-current-month",
            ) as HTMLElement | null;
            if (curMonth) curMonth.style.display = "none";

            // avoid injecting multiple times
            if (cal.querySelector(".cp-month-year")) return;

            // Create wrapper
            const wrapper = document.createElement("div");
            wrapper.className = "cp-month-year";
            // center the selects absolutely in the header
            wrapper.style.position = "absolute";
            wrapper.style.left = "50%";
            wrapper.style.top = "8px";
            wrapper.style.transform = "translateX(-50%)";
            wrapper.style.zIndex = "3";
            // ensure inline layout and prevent wrapping even if header is narrow
            wrapper.style.display = "flex";
            wrapper.style.flexWrap = "nowrap";
            wrapper.style.alignItems = "center";
            wrapper.style.gap = "8px";

            // Create month select
            const monthSelect = document.createElement("select");
            monthSelect.className = "cp-select cp-month";
            // inline style guard to keep month select on the same line
            monthSelect.style.display = "inline-block";
            monthSelect.style.flex = "0 0 auto";
            monthSelect.style.whiteSpace = "nowrap";
            monthSelect.style.margin = "0";
            thaiMonths.forEach((m, idx) => {
                const o = document.createElement("option");
                o.value = String(idx);
                o.text = m;
                monthSelect.appendChild(o);
            });

            // Create year select
            const yearSelect = document.createElement("select");
            yearSelect.className = "cp-select cp-year";
            // inline style guard to keep year select on the same line
            yearSelect.style.display = "inline-block";
            yearSelect.style.flex = "0 0 auto";
            yearSelect.style.whiteSpace = "nowrap";
            yearSelect.style.margin = "0";

            const nowYear = instance.currentYear || new Date().getFullYear();
            const offset = Number(props.yearOffsetRange || 10);
            const start = nowYear - offset;
            const end = nowYear + offset;
            for (let y = start; y <= end; y++) {
                const o = document.createElement("option");
                o.value = String(y);
                // display BE (พ.ศ.) but keep value as CE
                o.text = String(y + 543);
                yearSelect.appendChild(o);
            }

            // Set selects to current month/year
            const setSelects = () => {
                const cy = instance.currentYear ?? new Date().getFullYear();
                const cm = instance.currentMonth ?? new Date().getMonth();
                (monthSelect as HTMLSelectElement).value = String(cm);
                (yearSelect as HTMLSelectElement).value = String(cy);
            };

            setSelects();

            // Change handlers
            monthSelect.addEventListener("change", () => {
                const selMonth = Number(
                    (monthSelect as HTMLSelectElement).value,
                );
                const selYear = Number((yearSelect as HTMLSelectElement).value);
                // Jump to the first day of the chosen month/year
                const dt = new Date(selYear, selMonth, 1);
                try {
                    instance.jumpToDate(dt);
                } catch {
                    // fallback: set month/year directly
                    try {
                        instance.changeMonth(selMonth - instance.currentMonth);
                        instance.currentYear = selYear;
                    } catch {
                        /* ignore */
                    }
                }
            });

            yearSelect.addEventListener("change", () => {
                const selMonth = Number(
                    (monthSelect as HTMLSelectElement).value,
                );
                const selYear = Number((yearSelect as HTMLSelectElement).value);
                const dt = new Date(selYear, selMonth, 1);
                try {
                    instance.jumpToDate(dt);
                } catch {
                    try {
                        instance.changeMonth(selMonth - instance.currentMonth);
                        instance.currentYear = selYear;
                    } catch {
                        /* ignore */
                    }
                }
            });

            // When flatpickr navigates months via arrows, keep selects in sync
            const navObserver = new MutationObserver(() => {
                setTimeout(setSelects, 0);
            });
            const nav = cal.querySelector(".flatpickr-months");
            if (nav)
                navObserver.observe(nav, {
                    subtree: true,
                    childList: true,
                    attributes: true,
                });

            // Insert wrapper into header (replace title area)
            // Try to place inside the month container in a sensible position
            // We append after existing month elements so it appears centered
            wrapper.appendChild(monthSelect);
            wrapper.appendChild(yearSelect);

            // Create Today button (sets the calendar to today's date and closes)
            const todayBtn = document.createElement("button");
            todayBtn.type = "button";
            todayBtn.className = "cp-today-btn";
            todayBtn.textContent = "วันนี้";
            // Inline styles as flatpickr DOM may be outside scoped CSS
            todayBtn.style.display = "inline-block";
            todayBtn.style.height = "30px";
            todayBtn.style.padding = "0 10px";
            todayBtn.style.borderRadius = "6px";
            todayBtn.style.border = "1px solid var(--border)";
            todayBtn.style.background = "var(--bg-surface)";
            todayBtn.style.color = "var(--text-primary)";
            todayBtn.style.cursor = "pointer";
            todayBtn.style.flex = "0 0 auto";
            todayBtn.style.margin = "0";

            todayBtn.addEventListener("click", () => {
                const dt = new Date();
                try {
                    // setDate will update input and selected date
                    instance.setDate(dt, true, "Y-m-d");
                    // ensure the calendar view also jumps to today
                    instance.jumpToDate(dt);
                    // close the calendar after selection
                    instance.close();
                } catch {
                    // Fallback attempts for different flatpickr builds
                    try {
                        instance.setDate(dt);
                        instance.close();
                    } catch {
                        /* ignore */
                    }
                }
            });

            wrapper.appendChild(todayBtn);
            header.appendChild(wrapper);
        },
    });
});

onBeforeUnmount(() => {
    if (fpInstance) {
        try {
            fpInstance.destroy();
        } catch {
            // ignore
        }
        fpInstance = null;
    }
});

// Keep external modelValue in sync (parent -> input)
watch(
    () => props.modelValue,
    (val) => {
        if (!fpInstance) return;
        if (!val) {
            try {
                fpInstance.clear();
            } catch {}
            if (inputEl.value) inputEl.value.value = "";
            return;
        }
        const current = fpInstance.input?.value || "";
        if (val !== current) {
            try {
                fpInstance.setDate(val, true, "Y-m-d");
            } catch {
                if (inputEl.value) inputEl.value.value = val;
            }
        }
    },
);
</script>

<style scoped>
/* Integrate with PayPerCase theme variables */
.calendar-picker {
    display: inline-block;
    font-family: var(--font);
}

.input-row {
    display: inline-flex;
    align-items: center;
    gap: 8px;
}

.cp-input {
    padding: 10px 12px;
    border-radius: 6px;
    border: 1px solid var(--border);
    font-size: 14px;
    width: 170px;
    background: var(--bg-surface);
    color: var(--text-primary);
    height: 36px;
    line-height: 1;
}

.cp-icon {
    width: 36px;
    height: 36px;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    border-radius: 6px;
    background: var(--bg-surface);
    border: 1px solid transparent;
    cursor: pointer;
}

/* Flatpickr header injection */
.cp-month-year {
    position: absolute;
    left: 50%;
    top: 8px;
    transform: translateX(-50%);
    display: inline-flex;
    gap: 8px;
    align-items: center;
    /* absolutely centered by script */
    z-index: 3;
    white-space: nowrap; /* prevent month/year wrapping to two lines */
}

.cp-select {
    display: inline-block;
    height: 30px;
    border-radius: 6px;
    border: 1px solid var(--border);
    background: var(--bg-surface);
    color: var(--text-primary);
    font-size: 13px;
    padding: 0 6px;
    vertical-align: middle;
}
.cp-month {
    min-width: 116px; /* fixed-ish width so month label fits on one line */
    max-width: 140px;
}
.cp-year {
    min-width: 72px;
    max-width: 92px;
}
.cp-today-btn {
    height: 30px;
    border-radius: 6px;
    border: 1px solid var(--border);
    background: var(--bg-surface);
    color: var(--text-primary);
    font-size: 13px;
    padding: 0 10px;
    cursor: pointer;
    display: inline-block;
    vertical-align: middle;
}

/* tweak calendar layout to accommodate dropdowns */
.flatpickr-months {
    position: relative;
    display: flex;
    align-items: center;
    gap: 8px;
    justify-content: center;
}

/* keep calendar look consistent */
.flatpickr-calendar {
    border-radius: 8px;
    box-shadow: var(--shadow);
    border: 1px solid var(--border);
    padding: 8px;
    font-family: var(--font);
    color: var(--text-primary);
    z-index: 9999;
}

/* selected/ today styles */
.flatpickr-day.today {
    color: var(--primary);
    font-weight: 600;
}
.flatpickr-day.selected {
    background: var(--primary) !important;
    color: #fff !important;
    border-radius: 6px;
}
</style>
