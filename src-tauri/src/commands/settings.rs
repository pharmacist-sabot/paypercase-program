// commands/settings.rs — Settings CRUD commands

use std::sync::Arc;
use tauri::State;

use crate::db::{hosxp, local};
use crate::models::*;
use crate::state::AppState;

// ─────────────────────────────────────────────────────────────────────────────
//  HOSxP Lookup (ใช้ในหน้าตั้งค่า เพื่อค้นหาข้อมูลจาก HOSxP)
// ─────────────────────────────────────────────────────────────────────────────

#[tauri::command]
pub fn lookup_pttype(
    hipdata_code: String,
    state: State<'_, Arc<AppState>>,
) -> Result<Option<PttypeLookup>, String> {
    let pool_guard = state.hosxp_pool.lock().map_err(|e| e.to_string())?;
    let pool = pool_guard
        .as_ref()
        .ok_or("ยังไม่ได้เชื่อมต่อ HOSxP กรุณาตั้งค่าการเชื่อมต่อก่อน")?;
    hosxp::fetch_pttype_by_hipdata(pool, &hipdata_code)
}

#[tauri::command]
pub fn lookup_procedure(
    icode: String,
    state: State<'_, Arc<AppState>>,
) -> Result<Option<ItemLookup>, String> {
    let pool_guard = state.hosxp_pool.lock().map_err(|e| e.to_string())?;
    let pool = pool_guard
        .as_ref()
        .ok_or("ยังไม่ได้เชื่อมต่อ HOSxP กรุณาตั้งค่าการเชื่อมต่อก่อน")?;
    hosxp::fetch_procedure_by_icode(pool, &icode)
}

#[tauri::command]
pub fn lookup_drug(
    icode: String,
    state: State<'_, Arc<AppState>>,
) -> Result<Option<ItemLookup>, String> {
    let pool_guard = state.hosxp_pool.lock().map_err(|e| e.to_string())?;
    let pool = pool_guard
        .as_ref()
        .ok_or("ยังไม่ได้เชื่อมต่อ HOSxP กรุณาตั้งค่าการเชื่อมต่อก่อน")?;
    hosxp::fetch_drug_by_icode(pool, &icode)
}

#[tauri::command]
pub fn lookup_provider(
    provider_id: i64,
    state: State<'_, Arc<AppState>>,
) -> Result<Option<ProviderLookup>, String> {
    let pool_guard = state.hosxp_pool.lock().map_err(|e| e.to_string())?;
    let pool = pool_guard
        .as_ref()
        .ok_or("ยังไม่ได้เชื่อมต่อ HOSxP กรุณาตั้งค่าการเชื่อมต่อก่อน")?;
    hosxp::fetch_provider_by_id(pool, provider_id)
}

// ─────────────────────────────────────────────────────────────────────────────
//  Pttypes
// ─────────────────────────────────────────────────────────────────────────────

#[tauri::command]
pub fn get_all_pttypes(state: State<'_, Arc<AppState>>) -> Result<Vec<PttypeConfig>, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    local::get_all_pttypes(&db).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn save_pttype(
    pttype: String,
    name: String,
    pcode: String,
    hipdata_code: String,
    short_name: String,
    state: State<'_, Arc<AppState>>,
) -> Result<(), String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    local::save_pttype(&db, &pttype, &name, &pcode, &hipdata_code, &short_name)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_pttype(id: i64, state: State<'_, Arc<AppState>>) -> Result<(), String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    local::delete_pttype(&db, id).map_err(|e| e.to_string())
}

// ─────────────────────────────────────────────────────────────────────────────
//  Procedures
// ─────────────────────────────────────────────────────────────────────────────

#[tauri::command]
pub fn get_all_procedures(state: State<'_, Arc<AppState>>) -> Result<Vec<ProcedureConfig>, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    local::get_all_procedures(&db).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn save_procedure(
    icode: String,
    name: String,
    short_name: String,
    state: State<'_, Arc<AppState>>,
) -> Result<(), String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    local::save_procedure(&db, &icode, &name, &short_name).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_procedure(id: i64, state: State<'_, Arc<AppState>>) -> Result<(), String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    local::delete_procedure(&db, id).map_err(|e| e.to_string())
}

// ─────────────────────────────────────────────────────────────────────────────
//  Drugs
// ─────────────────────────────────────────────────────────────────────────────

#[tauri::command]
pub fn get_all_drugs(state: State<'_, Arc<AppState>>) -> Result<Vec<DrugConfig>, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    local::get_all_drugs(&db).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn save_drug(
    icode: String,
    name: String,
    short_name: String,
    state: State<'_, Arc<AppState>>,
) -> Result<(), String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    local::save_drug(&db, &icode, &name, &short_name).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_drug(id: i64, state: State<'_, Arc<AppState>>) -> Result<(), String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    local::delete_drug(&db, id).map_err(|e| e.to_string())
}

// ─────────────────────────────────────────────────────────────────────────────
//  Providers
// ─────────────────────────────────────────────────────────────────────────────

#[tauri::command]
pub fn get_all_providers(state: State<'_, Arc<AppState>>) -> Result<Vec<ProviderConfig>, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    local::get_all_providers(&db).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn save_provider(
    health_med_provider_id: i64,
    full_name: String,
    short_name: String,
    state: State<'_, Arc<AppState>>,
) -> Result<(), String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    local::save_provider(&db, health_med_provider_id, &full_name, &short_name)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_provider(id: i64, state: State<'_, Arc<AppState>>) -> Result<(), String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    local::delete_provider(&db, id).map_err(|e| e.to_string())
}

// ─────────────────────────────────────────────────────────────────────────────
//  Payout Options
// ─────────────────────────────────────────────────────────────────────────────

#[tauri::command]
pub fn get_payout_options(state: State<'_, Arc<AppState>>) -> Result<Vec<PayoutOption>, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    local::get_payout_options(&db).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn add_payout_option(
    amount: f64,
    label: String,
    state: State<'_, Arc<AppState>>,
) -> Result<(), String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    local::add_payout_option(&db, amount, &label).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_payout_option(id: i64, state: State<'_, Arc<AppState>>) -> Result<(), String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    local::delete_payout_option(&db, id).map_err(|e| e.to_string())
}
