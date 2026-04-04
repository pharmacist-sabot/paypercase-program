// lib.rs — PayPerCase v5.0.0 Tauri Application Setup
// ไม่มีการแก้ไขข้อมูลใน HOSxP เป็นอันขาด

mod commands;
mod db;
mod models;
mod state;

use state::AppState;
use db::local;

fn get_db_path() -> std::path::PathBuf {
    if let Some(data_dir) = dirs::data_dir() {
        let app_dir = data_dir.join("PayPerCase");
        std::fs::create_dir_all(&app_dir).ok();
        app_dir.join("paypercase.db")
    } else {
        std::path::PathBuf::from("paypercase.db")
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // เปิด SQLite database
    let db_path = get_db_path();
    let conn = rusqlite::Connection::open(&db_path)
        .expect("ไม่สามารถเปิดฐานข้อมูลท้องถิ่น paypercase.db");

    // สร้างตารางและค่า default
    local::init_config_db(&conn).expect("ไม่สามารถเริ่มต้นฐานข้อมูลท้องถิ่น");

    // สร้าง AppState
    let state = AppState::new(conn);

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .manage(state)
        .invoke_handler(tauri::generate_handler![
            // Connection
            commands::connection::get_hosxp_settings,
            commands::connection::save_hosxp_settings,
            commands::connection::test_hosxp_connection,
            commands::connection::connect_hosxp,
            commands::connection::disconnect_hosxp,
            commands::connection::is_hosxp_connected,
            commands::connection::auto_connect,
            // Settings — Lookup
            commands::settings::lookup_pttype,
            commands::settings::lookup_procedure,
            commands::settings::lookup_drug,
            commands::settings::lookup_provider,
            // Settings — Pttypes
            commands::settings::get_all_pttypes,
            commands::settings::save_pttype,
            commands::settings::delete_pttype,
            // Settings — Procedures
            commands::settings::get_all_procedures,
            commands::settings::save_procedure,
            commands::settings::delete_procedure,
            // Settings — Drugs
            commands::settings::get_all_drugs,
            commands::settings::save_drug,
            commands::settings::delete_drug,
            // Settings — Providers
            commands::settings::get_all_providers,
            commands::settings::save_provider,
            commands::settings::delete_provider,
            // Settings — Payout Options
            commands::settings::get_payout_options,
            commands::settings::add_payout_option,
            commands::settings::delete_payout_option,
            // Data View
            commands::data::fetch_patient_data,
            commands::data::get_locked_vns,
            // Pending Export
            commands::pending::upsert_pending,
            commands::pending::get_pending_export,
            commands::pending::delete_pending_by_id,
            commands::pending::export_pending_csv,
            // Delete Data
            commands::delete::preview_delete_range,
            commands::delete::delete_pending_range,
            commands::delete::get_monthly_stats,
        ])
        .run(tauri::generate_context!())
        .expect("error while running PayPerCase application");
}
