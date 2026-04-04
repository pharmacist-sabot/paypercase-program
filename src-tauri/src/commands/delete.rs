// commands/delete.rs — Delete data operations

use std::sync::Arc;
use tauri::State;

use crate::models::{MonthlyStats, PendingRow};
use crate::state::AppState;
use crate::db::local;

/// ดูรายการที่จะถูกลบในช่วงวันที่ที่เลือก
#[tauri::command]
pub fn preview_delete_range(
    date_from: String,
    date_to: String,
    state: State<'_, Arc<AppState>>,
) -> Result<Vec<PendingRow>, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    local::preview_delete_range(&db, &date_from, &date_to).map_err(|e| e.to_string())
}

/// ลบข้อมูล pending_export ในช่วงวันที่ที่เลือก
#[tauri::command]
pub fn delete_pending_range(
    date_from: String,
    date_to: String,
    state: State<'_, Arc<AppState>>,
) -> Result<usize, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    local::delete_pending_by_range(&db, &date_from, &date_to).map_err(|e| e.to_string())
}

/// ดึงสถิติรายเดือน
#[tauri::command]
pub fn get_monthly_stats(state: State<'_, Arc<AppState>>) -> Result<Vec<MonthlyStats>, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    local::get_monthly_stats(&db).map_err(|e| e.to_string())
}
