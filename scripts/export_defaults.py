#!/usr/bin/env python3
# scripts/export_defaults.py
#
# วิธีใช้:
#   1. npm run tauri dev  → ตั้งค่าทุกอย่างผ่าน UI (payout, สิทธิ, หัตถการ, พนักงาน ฯลฯ)
#   2. python3 scripts/export_defaults.py  (หรือ npm run export-defaults)
#   3. npm run tauri build  → ค่าทั้งหมดจะติดไปใน .exe สำหรับการติดตั้งครั้งแรก
#
# หมายเหตุ: HOSxP connection settings จะไม่ถูก export (เหลือว่างเพื่อความปลอดภัย)

import json
import os
import sqlite3
import sys
from pathlib import Path

# ─── หาตำแหน่ง paypercase.db ตาม OS ────────────────────────────────────────


def get_db_path() -> Path:
    platform = sys.platform
    if platform == "win32":
        base = Path(os.environ.get("APPDATA", Path.home() / "AppData" / "Roaming"))
    elif platform == "darwin":
        base = Path.home() / "Library" / "Application Support"
    else:
        base = Path(os.environ.get("XDG_DATA_HOME", Path.home() / ".local" / "share"))
    return base / "PayPerCase" / "paypercase.db"


# ─── Main ────────────────────────────────────────────────────────────────────


def main():
    db_path = get_db_path()

    print(f"📂 กำลังอ่านฐานข้อมูล: {db_path}")

    if not db_path.exists():
        print(f"\n❌ ไม่พบไฟล์ฐานข้อมูล: {db_path}")
        print("   กรุณา run โปรแกรมอย่างน้อยหนึ่งครั้งก่อน แล้วตั้งค่าผ่าน UI")
        sys.exit(1)

    conn = sqlite3.connect(str(db_path))
    conn.row_factory = sqlite3.Row

    try:
        payout_options = [
            dict(r)
            for r in conn.execute(
                "SELECT amount, label FROM payout_options ORDER BY amount DESC"
            )
        ]

        pttypes = [
            dict(r)
            for r in conn.execute(
                "SELECT pttype, name, pcode, hipdata_code, short_name "
                "FROM configured_pttypes ORDER BY id"
            )
        ]

        procedures = [
            dict(r)
            for r in conn.execute(
                "SELECT icode, name, short_name FROM configured_procedures ORDER BY id"
            )
        ]

        drugs = [
            dict(r)
            for r in conn.execute(
                "SELECT icode, name, short_name FROM configured_drugs ORDER BY id"
            )
        ]

        providers = [
            dict(r)
            for r in conn.execute(
                "SELECT health_med_provider_id, full_name, short_name "
                "FROM configured_providers ORDER BY id"
            )
        ]

    finally:
        conn.close()

    # ─── สร้าง config dict ───────────────────────────────────────────────────
    config = {
        "_readme": (
            "สร้างอัตโนมัติโดย scripts/export_defaults.py — "
            "ห้ามแก้ไขโดยตรง ให้ตั้งค่าผ่าน UI แล้วรัน npm run export-defaults ใหม่"
        ),
        "_instructions": {
            "payout_options": "ตัวเลือกค่าตอบแทน เช่น { amount: 125.0, label: 'ค่าตอบแทน 125 บาท' }",
            "pttypes": "สิทธิการรักษา — pttype, name, pcode, hipdata_code, short_name",
            "procedures": "หัตถการ — icode, name, short_name",
            "drugs": "ยาสมุนไพร — icode, name, short_name",
            "providers": "นักนวด/พนักงาน — health_med_provider_id, full_name, short_name",
        },
        "payout_options": payout_options,
        "pttypes": pttypes,
        "procedures": procedures,
        "drugs": drugs,
        "providers": providers,
    }

    # ─── เขียนไปยัง src-tauri/default_config.json ───────────────────────────
    script_dir = Path(__file__).resolve().parent  # paypercase-program/scripts/
    project_root = script_dir.parent  # paypercase-program/
    output_path = project_root / "src-tauri" / "default_config.json"

    output_path.write_text(
        json.dumps(config, ensure_ascii=False, indent=2),
        encoding="utf-8",
    )

    # ─── สรุปผล ─────────────────────────────────────────────────────────────
    print()
    print("✅ บันทึก default_config.json เรียบร้อยแล้ว!")
    print(f"   📄 {output_path}")
    print()
    print(f"   💰 payout_options : {len(payout_options):>3} รายการ")
    print(f"   🏥 pttypes        : {len(pttypes):>3} รายการ")
    print(f"   🩺 procedures     : {len(procedures):>3} รายการ")
    print(f"   💊 drugs          : {len(drugs):>3} รายการ")
    print(f"   👤 providers      : {len(providers):>3} รายการ")
    print()
    print("👉 ขั้นตอนถัดไป: npm run tauri build")


if __name__ == "__main__":
    main()
