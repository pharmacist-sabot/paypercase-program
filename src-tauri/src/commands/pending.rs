// commands/pending.rs — Pending export operations

use std::io::Write;
use std::sync::Arc;
use tauri::State;

use crate::db::local;
use crate::models::{PendingRow, UpsertPendingInput};
use crate::state::AppState;

/// บันทึก (upsert) ข้อมูลผู้ป่วยไปยัง pending_export
#[tauri::command]
pub fn upsert_pending(
    record: UpsertPendingInput,
    state: State<'_, Arc<AppState>>,
) -> Result<(), String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    local::upsert_pending_export(&db, &record).map_err(|e| e.to_string())
}

/// ดึงข้อมูล pending_export ตามวันที่
#[tauri::command]
pub fn get_pending_export(
    date_from: String,
    date_to: String,
    state: State<'_, Arc<AppState>>,
) -> Result<Vec<PendingRow>, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    local::get_pending_export(&db, &date_from, &date_to).map_err(|e| e.to_string())
}

/// ลบรายการจาก pending_export ด้วย id
#[tauri::command]
pub fn delete_pending_by_id(id: i64, state: State<'_, Arc<AppState>>) -> Result<(), String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    local::delete_pending_by_id(&db, id).map_err(|e| e.to_string())
}

/// Export pending_export เป็น CSV ไปยัง path ที่กำหนด
/// ชื่อไฟล์: PayPerCase_YYYY-MM-DD.csv (ตามวันที่ที่อยู่ในข้อมูล)
#[tauri::command]
pub fn export_pending_csv(
    date_from: String,
    date_to: String,
    file_path: String,
    state: State<'_, Arc<AppState>>,
) -> Result<String, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    let rows = local::get_pending_export(&db, &date_from, &date_to).map_err(|e| e.to_string())?;

    if rows.is_empty() {
        return Err("ไม่มีข้อมูลในช่วงวันที่ที่เลือก".to_string());
    }

    let mut file =
        std::fs::File::create(&file_path).map_err(|e| format!("ไม่สามารถสร้างไฟล์: {}", e))?;

    // Write UTF-8 BOM for Excel compatibility
    file.write_all(&[0xEF, 0xBB, 0xBF])
        .map_err(|e| format!("ไม่สามารถเขียนไฟล์: {}", e))?;

    // Header row
    let header = "visit_date,hn,cid,first_name,last_name,gender,age,rights,symptoms,procedure,therapist,total_revenue,payout_amount\n";
    file.write_all(header.as_bytes())
        .map_err(|e| format!("ไม่สามารถเขียน header: {}", e))?;

    for row in &rows {
        let line = format!(
            "{},{},{},{},{},{},{},{},{},{},{},{},{}\n",
            csv_escape(&row.visit_date),
            csv_escape(&row.hn),
            csv_escape(&row.cid),
            csv_escape(&row.first_name),
            csv_escape(&row.last_name),
            csv_escape(&row.gender),
            row.age.map(|a| a.to_string()).unwrap_or_default(),
            csv_escape(&row.rights),
            csv_escape(&row.symptoms),
            csv_escape(&row.procedure),
            csv_escape(&row.therapist),
            row.total_revenue,
            row.payout_amount,
        );
        file.write_all(line.as_bytes())
            .map_err(|e| format!("ไม่สามารถเขียนข้อมูล: {}", e))?;
    }

    Ok(format!("ส่งออกสำเร็จ {} รายการ → {}", rows.len(), file_path))
}

fn csv_escape(s: &str) -> String {
    if s.contains(',') || s.contains('"') || s.contains('\n') {
        format!("\"{}\"", s.replace('"', "\"\""))
    } else {
        s.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_csv_escape_plain() {
        assert_eq!(csv_escape("hello"), "hello");
    }

    #[test]
    fn test_csv_escape_comma() {
        assert_eq!(csv_escape("a,b"), "\"a,b\"");
    }

    #[test]
    fn test_csv_escape_quotes() {
        assert_eq!(csv_escape("say \"hi\""), "\"say \"\"hi\"\"\"");
    }

    #[test]
    fn test_csv_escape_newline() {
        assert_eq!(csv_escape("line1\nline2"), "\"line1\nline2\"");
    }

    #[test]
    fn test_csv_escape_empty() {
        assert_eq!(csv_escape(""), "");
    }

    #[test]
    fn test_csv_escape_thai() {
        assert_eq!(csv_escape("นวดแผนไทย"), "นวดแผนไทย");
    }

    #[test]
    fn test_csv_escape_combined() {
        // Contains comma, quotes, and newline — should wrap in quotes and double the inner quotes
        let input = "a,\"b\"\nc";
        let result = csv_escape(input);
        assert_eq!(result, "\"a,\"\"b\"\"\nc\"");
    }
}
