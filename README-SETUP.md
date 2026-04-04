paypercase-program/README-SETUP.md
# PayPerCase — Deployment / Build Setup (เก็บค่าเริ่มต้นจากเครื่อง dev)

วัตถุประสงค์
- ให้คุณสามารถตั้งค่าทั้งหมดผ่านโหมดพัฒนา (dev) บนเครื่องของคุณ
- จากนั้น export การตั้งค่าเหล่านั้นเป็นไฟล์ `default_config.json`
- เมื่อ build โปรแกรม (release) ค่าใน `default_config.json` จะถูกฝังเข้าไปใน binary ทำให้ผู้ใช้ที่ติดตั้งใหม่ได้ค่าเริ่มต้นพร้อมใช้ทันที
- สร้าง workflow ง่าย 3 ขั้นตอน เพื่อให้ทำซ้ำได้ในอนาคต

สรุป 3 ขั้นตอนที่สำคัญ (สั้น)
1. เปิด dev และตั้งค่าผ่าน UI
2. รัน `npm run export-defaults` → จะอ่าน DB dev ของคุณ และเขียน `src-tauri/default_config.json`
3. รัน `npm run tauri build` → ค่าใน `default_config.json` จะฝังเข้า binary

ไฟล์สำคัญที่เกี่ยวข้อง
- สคริปต์ export: `scripts/export_defaults.py`
```paypercase-program/scripts/export_defaults.py#L1-40
#!/usr/bin/env python3
# scripts/export_defaults.py
# (อ่าน DB ของ dev แล้วเขียน src-tauri/default_config.json)
```
- ไฟล์ config เริ่มต้นที่ถูกฝัง: `src-tauri/default_config.json`
```paypercase-program/src-tauri/default_config.json#L1-40
{
  "_readme": "...",
  "payout_options": [ ... ],
  "pttypes": [...],
  "procedures": [...],
  "drugs": [...],
  "providers": [...]
}
```
- โค้ดใน Rust ที่โหลดไฟล์นี้และ seed ครั้งแรก: `src-tauri/src/lib.rs`
```paypercase-program/src-tauri/src/lib.rs#L1-80
const DEFAULT_CONFIG_JSON: &str = include_str!("../default_config.json");
// ... เมื่อ run ครั้งแรก: อ่าน DEFAULT_CONFIG_JSON และ seed DB ...
```
- ฟังก์ชัน seed / flag: `src-tauri/src/db/local.rs`
```paypercase-program/src-tauri/src/db/local.rs#L450-680
// is_first_run, mark_seeded, seed_from_default_config
```

คำสั่งที่ใช้ (ตัวอย่าง)
- เปิด development (แนะนำให้ใช้คำสั่งนี้ เพื่อให้ tauri/หน้าต่าง desktop เปิดพร้อมกับ vite dev server)
```/dev/null/commands.sh#L1-3
npm run tauri dev
# หรือ ถ้าต้องการแยก:
npm run dev         # เรียก vite
npm run tauri -- dev
```

- หลังตั้งค่าผ่าน UI เสร็จแล้ว ให้ export ค่าจาก DB ลงไฟล์ default_config.json:
```/dev/null/commands.sh#L1-3
npm run export-defaults
# (script จะอ่านจาก DB ของคุณ แล้วเขียนทับ src-tauri/default_config.json)
```

- สร้าง build สำหรับแจกจ่าย:
```/dev/null/commands.sh#L1-2
npm run tauri build
```

รายละเอียดแต่ละขั้นตอน (ละเอียด)

1) ตั้งค่าทุกอย่างผ่าน UI (Dev)
- เป้าหมาย: กำหนดค่าที่คุณต้องการให้เป็นค่าเริ่มต้นสำหรับผู้ติดตั้งใหม่ เช่น:
  - Payout options (ค่าตอบแทน)
  - Pttypes (สิทธิการรักษา)
  - Procedures (หัตถการ / รหัส icode)
  - Drugs (ยา / รหัส icode)
  - Providers (พนักงาน / นักนวด)
- วิธี:
  - รันแอปในโหมด dev (`npm run tauri dev`)
  - เข้าเมนู Settings → ใส่ข้อมูลตามต้องการ → บันทึก
  - ปิดโปรแกรมเมื่อเสร็จ

สำคัญ: *อย่าใส่ข้อมูลการเชื่อมต่อ HOSxP ไว้ในไฟล์เริ่มต้น* — เรา intentionally ทิ้งค่า HOSxP เป็นค่าว่างเพื่อความปลอดภัย (ผู้ใช้ต้องตั้งค่า connection ด้วยตัวเอง)

2) Export ค่าเป็น `default_config.json`
- สคริปต์ที่เตรียมไว้จะอ่านฐานข้อมูล local (ไฟล์ SQLite ที่ dev ใช้งานอยู่) แล้ว extract ตารางที่เกี่ยวกับการตั้งค่า แล้วเขียน JSON ไปที่:
  - `src-tauri/default_config.json`
- คำสั่ง:
```/dev/null/commands.sh#L1-2
npm run export-defaults
# (เรียก python3 scripts/export_defaults.py)
```
- สิ่งที่สคริปต์จะ export:
  - `payout_options` ← ค่า amount + label
  - `pttypes` ← pttype, name, pcode, hipdata_code, short_name
  - `procedures` ← icode, name, short_name
  - `drugs` ← icode, name, short_name
  - `providers` ← health_med_provider_id, full_name, short_name

- ตรวจสอบไฟล์ที่ถูกเขียน:
```paypercase-program/src-tauri/default_config.json#L1-80
# เปิดไฟล์นี้เพื่อตรวจสอบว่าค่าเป็นไปตามที่ต้องการก่อน build
```

3) Build (ฝังค่าเข้า binary)
- เมื่อ `src-tauri/default_config.json` ถูกกำหนดเรียบร้อยแล้ว ให้ build:
```/dev/null/commands.sh#L1-2
npm run tauri build
```
- ระบบจะ compile Rust และฝังค่า `default_config.json` โดยใช้ `include_str!("../default_config.json")` — ดังนั้นไฟล์ต้องอยู่ก่อนทำการ build
- ผลลัพธ์: binary ที่แจกจ่ายให้ผู้ใช้ จะเติมค่าเริ่มต้นลงในฐานข้อมูล `paypercase.db` เมื่อผู้ใช้รันโปรแกรมเป็นครั้งแรก (seed เฉพาะครั้งแรก)

การตรวจสอบหลัง build / ติดตั้ง
- รันโปรแกรมบนเครื่องที่ติดตั้งใหม่ → ตรวจสอบ Settings ว่าค่าที่ export มากำลังถูกแสดง หากไม่มี ให้ตรวจสอบ log ของแอป (stderr) ว่ามี error ในการ parse JSON หรือ seed DB หรือไม่
- หากต้องการยืนยันในไฟล์ฐานข้อมูลของเครื่องที่ติดตั้ง:
  - ตำแหน่งไฟล์ DB (ตัวอย่าง)
    - macOS: ~/Library/Application Support/PayPerCase/paypercase.db
    - Windows: %APPDATA%\PayPerCase\paypercase.db
    - Linux: ${XDG_DATA_HOME:-$HOME/.local/share}/PayPerCase/paypercase.db
  - ตัวอย่างเช็คตาราง payout_options:
```/dev/null/commands.sh#L1-2
sqlite3 "/path/to/paypercase.db" "SELECT * FROM payout_options;"
```

วิธี re-seed / อัพเดต default แล้ว rebuild (กรณีต้องการเปลี่ยนไฟล์ default)
- หากคุณต้องการเปลี่ยนค่า default (เช่น เพิ่ม pttype ใหม่) ให้ทำตาม:
  1. เปิด dev → ปรับค่าใน UI → ปิดโปรแกรม
  2. `npm run export-defaults` → ไฟล์ `src-tauri/default_config.json` จะถูกเขียนทับ
  3. `npm run tauri build` → rebuild เพื่อฝังค่าใหม่
- หากต้องการ force ให้ผู้ติดตั้งเดิม (ที่ติดตั้งไปแล้ว) ใช้ค่า default ใหม่: ผู้ติดตั้งเดิมต้องลบ flag `seeded_from_default` ใน DB ของเครื่องนั้นด้วยตนเอง (หรือ uninstall/reinstall)
  - ลบ flag ด้วย sqlite3:
```/dev/null/commands.sh#L1-2
sqlite3 "/path/to/paypercase.db" "DELETE FROM app_settings WHERE key='seeded_from_default';"
```
  - แล้วเมื่อรันโปรแกรม มันจะ seed ใหม่จาก config ที่ฝังไว้

สำคัญด้านความปลอดภัย
- ห้ามฝังข้อมูลรหัสผ่าน/credentials ใดๆ ลงใน `default_config.json` หรือใน binary
- HOSxP connection จะถูกเว้นว่างใน workflow ปกติ — ผู้ใช้แต่ละคนต้องกรอกเอง
- ไฟล์ `default_config.json` ควรถูกจัดการจากแหล่งที่เชื่อถือได้เท่านั้น (หาก commit เข้า repo ให้ระวังการเผยแพร่ค่าที่ไม่ควรเปิดเผย)

ปัญหาทั่วไป & วิธีแก้
- Python3 ไม่ติดตั้ง → `npm run export-defaults` จะล้มเหลว
  - ติดตั้ง Python3 (macOS: Homebrew, Windows: installer, Linux: apt/yum)
- ไม่พบ DB (สคริปต์ออก error ว่าไม่พบไฟล์)
  - ให้สั่งรัน dev อย่างน้อยหนึ่งครั้ง แล้วตั้งค่าผ่าน UI เพื่อให้ DB ถูกสร้าง (ตารางและข้อมูลบางอย่างจะถูกสร้างโดย init)
- JSON ที่ถูกเขียนมีค่าผิดพลาด (ไม่ parse)
  - เปิด `src-tauri/default_config.json` ตรวจสอบ syntax (utf-8, ไม่มี trailing comma)
- ต้องการ revert default_config.json → ใช้ git เพื่อคืนค่า หรือแก้ไฟล์ด้วย editor ก่อน build

คำแนะนำการจัดเก็บเวอร์ชัน / การปล่อย
- ถ้าต้องการให้ release ชุดเดียวกันสำหรับหน่วยงาน/โรงพยาบาล:
  - ทำการตั้งค่าทั้งหมดบนเครื่อง dev
  - `npm run export-defaults` → ตรวจสอบ `src-tauri/default_config.json`
  - คอมมิตไฟล์นี้ใน branch release แล้ว `npm run tauri build`
- หากไฟล์ default ควรเป็นเฉพาะสำหรับแต่ละหน่วยงาน ให้ไม่ commit ลง repo กลาง แต่เก็บใน release artifact หรือทางเอกสารแจกให้กับผู้ติดตั้ง

บันทึกทางเทคนิค (เพิ่มเติม)
- ฟังก์ชัน seed จะตรวจสอบ `app_settings` key `'seeded_from_default'` เพื่อป้องกันการ seed ซ้ำ
- Seed จะทำการ insert เฉพาะตารางที่ว่าง (idempotent per-table)
- หากมีผู้ใช้งานต้องการ reset ให้ลบ row `app_settings.key = 'seeded_from_default'` แล้ว run app ใหม่

การช่วยเหลือเพิ่มเติม
- ถ้าคุณต้องการ ผมสามารถ:
  - สร้างคำสั่ง npm เพิ่มเติมเพื่อรัน build แบบ clean / sign
  - เพิ่มปุ่มใน UI เพื่อ "Export default config" จากตัวโปรแกรมโดยตรง (ถ้าต้องการ workflow ที่ไม่ต้องใช้ terminal)
  - เพิ่ม script สำหรับสร้างตัวติดตั้ง (installer) พร้อมไฟล์ default แยกสำหรับแต่ละโรงพยาบาล

ขอให้การ deploy ราบรื่น! หากต้องการให้ผมเขียนตัวอย่างเอกสารสำหรับผู้ติดตั้ง (end-user README) หรือสคริปต์ตรวจสอบก่อน build แจ้งได้เลย :)