// commands/connection.rs — HOSxP connection management commands

use std::sync::Arc;
use tauri::State;

use crate::models::{ConnectionTestResult, HosxpSettings};
use crate::state::AppState;
use crate::db::{hosxp, local};

/// โหลดค่าการเชื่อมต่อที่บันทึกไว้
#[tauri::command]
pub fn get_hosxp_settings(state: State<'_, Arc<AppState>>) -> Result<HosxpSettings, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    local::get_hosxp_settings(&db).map_err(|e| e.to_string())
}

/// บันทึกค่าการเชื่อมต่อ HOSxP
#[tauri::command]
pub fn save_hosxp_settings(
    settings: HosxpSettings,
    state: State<'_, Arc<AppState>>,
) -> Result<(), String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    local::save_hosxp_settings(&db, &settings).map_err(|e| e.to_string())
}

/// ทดสอบการเชื่อมต่อ HOSxP (ไม่บันทึก pool)
#[tauri::command]
pub fn test_hosxp_connection(settings: HosxpSettings) -> ConnectionTestResult {
    hosxp::test_connection(&settings)
}

/// เชื่อมต่อ HOSxP และบันทึก pool ไว้ใน state
#[tauri::command]
pub fn connect_hosxp(
    settings: HosxpSettings,
    state: State<'_, Arc<AppState>>,
) -> Result<ConnectionTestResult, String> {
    // ทดสอบก่อน
    let test_result = hosxp::test_connection(&settings);
    if !test_result.ok {
        return Ok(test_result);
    }

    // สร้าง pool
    let pool = hosxp::create_pool(&settings)?;

    // บันทึก pool ใน state
    let mut pool_guard = state.hosxp_pool.lock().map_err(|e| e.to_string())?;
    *pool_guard = Some(pool);

    // บันทึกค่าการเชื่อมต่อใน SQLite
    let db = state.db.lock().map_err(|e| e.to_string())?;
    local::save_hosxp_settings(&db, &settings).map_err(|e| e.to_string())?;

    Ok(test_result)
}

/// ตัดการเชื่อมต่อ HOSxP
#[tauri::command]
pub fn disconnect_hosxp(state: State<'_, Arc<AppState>>) -> Result<(), String> {
    let mut pool_guard = state.hosxp_pool.lock().map_err(|e| e.to_string())?;
    *pool_guard = None;
    Ok(())
}

/// ตรวจสอบว่าเชื่อมต่อ HOSxP อยู่หรือไม่
#[tauri::command]
pub fn is_hosxp_connected(state: State<'_, Arc<AppState>>) -> Result<bool, String> {
    let pool_guard = state.hosxp_pool.lock().map_err(|e| e.to_string())?;
    Ok(pool_guard.is_some())
}

/// Auto-connect โดยใช้ค่าที่บันทึกไว้
#[tauri::command]
pub fn auto_connect(state: State<'_, Arc<AppState>>) -> Result<ConnectionTestResult, String> {
    let settings = {
        let db = state.db.lock().map_err(|e| e.to_string())?;
        local::get_hosxp_settings(&db).map_err(|e| e.to_string())?
    };

    if settings.host.is_empty() {
        return Ok(ConnectionTestResult {
            ok: false,
            message: "ยังไม่ได้ตั้งค่าการเชื่อมต่อ HOSxP".to_string(),
        });
    }

    let test_result = hosxp::test_connection(&settings);
    if test_result.ok {
        let pool = hosxp::create_pool(&settings)?;
        let mut pool_guard = state.hosxp_pool.lock().map_err(|e| e.to_string())?;
        *pool_guard = Some(pool);
    }

    Ok(test_result)
}
