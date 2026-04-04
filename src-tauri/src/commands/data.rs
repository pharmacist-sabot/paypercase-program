// commands/data.rs — Patient data fetch from HOSxP

use std::sync::Arc;
use tauri::State;

use crate::models::PatientRow;
use crate::state::AppState;
use crate::db::{hosxp, local};

/// ดึงข้อมูลผู้ป่วยจาก HOSxP สำหรับวันที่กำหนด (READ-ONLY)
#[tauri::command]
pub fn fetch_patient_data(
    date: String,
    state: State<'_, Arc<AppState>>,
) -> Result<Vec<PatientRow>, String> {
    // อ่าน pool HOSxP
    let pool_guard = state.hosxp_pool.lock().map_err(|e| e.to_string())?;
    let pool = pool_guard.as_ref().ok_or("ยังไม่ได้เชื่อมต่อ HOSxP\nกรุณาตั้งค่าการเชื่อมต่อในเมนู การเชื่อมต่อ")?;

    // อ่าน config จาก SQLite
    let db = state.db.lock().map_err(|e| e.to_string())?;
    let pttypes = local::get_all_pttypes(&db).map_err(|e| e.to_string())?;
    let procedures = local::get_all_procedures(&db).map_err(|e| e.to_string())?;
    let drugs = local::get_all_drugs(&db).map_err(|e| e.to_string())?;
    let providers = local::get_all_providers(&db).map_err(|e| e.to_string())?;

    if pttypes.is_empty() {
        return Err("ยังไม่ได้ตั้งค่าสิทธิการรักษา กรุณาตั้งค่าก่อน".to_string());
    }
    if providers.is_empty() {
        return Err("ยังไม่ได้ตั้งค่าพนักงาน กรุณาตั้งค่าก่อน".to_string());
    }
    if procedures.is_empty() && drugs.is_empty() {
        return Err("ยังไม่ได้ตั้งค่าหัตถการหรือยาสมุนไพร กรุณาตั้งค่าก่อน".to_string());
    }

    let pttype_codes: Vec<String> = pttypes.iter().map(|p| p.hipdata_code.clone()).collect();
    let provider_ids: Vec<i64> = providers.iter().map(|p| p.health_med_provider_id).collect();

    let mut all_icodes: Vec<String> = Vec::new();
    all_icodes.extend(procedures.iter().map(|p| p.icode.clone()));
    all_icodes.extend(drugs.iter().map(|d| d.icode.clone()));

    // procedure icodes คือ icodes ที่ขึ้นต้นด้วย '3'
    let procedure_icodes: Vec<String> = procedures.iter().map(|p| p.icode.clone()).collect();
    let procedure_icodes = if procedure_icodes.is_empty() {
        all_icodes.clone()
    } else {
        procedure_icodes
    };

    // ต้อง drop db lock ก่อนเรียก hosxp query เพื่อป้องกัน deadlock
    drop(db);

    hosxp::fetch_patient_data(
        pool,
        &date,
        &date,
        &pttype_codes,
        &provider_ids,
        &all_icodes,
        &procedure_icodes,
    )
}

/// ดึงรายชื่อ VN ที่ถูกล็อก (บันทึกใน pending_export) สำหรับวันที่กำหนด
#[tauri::command]
pub fn get_locked_vns(
    date: String,
    state: State<'_, Arc<AppState>>,
) -> Result<Vec<String>, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    local::get_locked_vns(&db, &date).map_err(|e| e.to_string())
}
