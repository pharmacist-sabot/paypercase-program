// db/local.rs — SQLite local database operations (paypercase.db + hosxp_connection.db)
// ไม่มีการแก้ไขข้อมูลใน HOSxP เป็นอันขาด

use rusqlite::{params, Connection, Result as SqliteResult};

use crate::models::*;

// ─────────────────────────────────────────────────────────────────────────────
//  Database initialisation
// ─────────────────────────────────────────────────────────────────────────────

/// สร้างตารางทั้งหมดใน paypercase.db พร้อมค่า default
pub fn init_config_db(conn: &Connection) -> SqliteResult<()> {
    conn.execute_batch(
        "
        PRAGMA journal_mode=WAL;

        CREATE TABLE IF NOT EXISTS app_settings (
            key   TEXT PRIMARY KEY,
            value TEXT
        );

        CREATE TABLE IF NOT EXISTS configured_pttypes (
            id           INTEGER PRIMARY KEY AUTOINCREMENT,
            pttype       TEXT,
            name         TEXT,
            pcode        TEXT,
            hipdata_code TEXT,
            short_name   TEXT
        );

        CREATE TABLE IF NOT EXISTS configured_procedures (
            id         INTEGER PRIMARY KEY AUTOINCREMENT,
            icode      TEXT UNIQUE,
            name       TEXT,
            short_name TEXT
        );

        CREATE TABLE IF NOT EXISTS configured_drugs (
            id         INTEGER PRIMARY KEY AUTOINCREMENT,
            icode      TEXT UNIQUE,
            name       TEXT,
            short_name TEXT
        );

        CREATE TABLE IF NOT EXISTS configured_providers (
            id                     INTEGER PRIMARY KEY AUTOINCREMENT,
            health_med_provider_id INTEGER UNIQUE,
            full_name              TEXT
        );

        CREATE TABLE IF NOT EXISTS payout_options (
            id     INTEGER PRIMARY KEY AUTOINCREMENT,
            amount REAL,
            label  TEXT
        );

        CREATE TABLE IF NOT EXISTS pending_export (
            id            INTEGER PRIMARY KEY AUTOINCREMENT,
            visit_date    DATE      NOT NULL,
            vn            TEXT      NOT NULL,
            hn            TEXT,
            cid           TEXT,
            first_name    TEXT,
            last_name     TEXT,
            gender        TEXT,
            age           INTEGER,
            rights        TEXT,
            symptoms      TEXT,
            procedure     TEXT,
            therapist     TEXT,
            total_revenue REAL,
            payout_amount REAL,
            created_at    TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            UNIQUE(visit_date, vn)
        );

        CREATE TABLE IF NOT EXISTS hosxp_connection (
            id            INTEGER PRIMARY KEY DEFAULT 1,
            host          TEXT,
            port          INTEGER DEFAULT 3306,
            database_name TEXT,
            username      TEXT,
            password      TEXT
        );
    ",
    )?;

    // Migration: add short_name column to configured_providers if not exists
    let _ = conn.execute(
        "ALTER TABLE configured_providers ADD COLUMN short_name TEXT NOT NULL DEFAULT ''",
        [],
    );

    // อ่านไฟล์ default_config.json ที่ถูกฝังไว้ตอน Compile (ใช้ include_str! เพื่อให้ข้อมูลติดไปกับแอป .exe ไม่ต้องพึ่งไฟล์ภายนอก)
    let default_config_str = include_str!("../../default_config.json");
    if let Ok(config) = serde_json::from_str::<serde_json::Value>(default_config_str) {
        
        let count: i64 = conn.query_row("SELECT COUNT(*) FROM payout_options", [], |row| row.get(0))?;
        if count == 0 {
            if let Some(arr) = config.get("payout_options").and_then(|v| v.as_array()) {
                for item in arr {
                    if let (Some(amount), Some(label)) = (item.get("amount").and_then(|v| v.as_f64()), item.get("label").and_then(|v| v.as_str())) {
                        let _ = conn.execute(
                            "INSERT INTO payout_options (amount, label) VALUES (?1, ?2)",
                            params![amount, label],
                        );
                    }
                }
            }
        }

        let pt_count: i64 = conn.query_row("SELECT COUNT(*) FROM configured_pttypes", [], |row| row.get(0))?;
        if pt_count == 0 {
            if let Some(arr) = config.get("pttypes").and_then(|v| v.as_array()) {
                for item in arr {
                    if let (Some(pt), Some(nm), Some(pc), Some(hip), Some(sh)) = (
                        item.get("pttype").and_then(|v| v.as_str()),
                        item.get("name").and_then(|v| v.as_str()),
                        item.get("pcode").and_then(|v| v.as_str()),
                        item.get("hipdata_code").and_then(|v| v.as_str()),
                        item.get("short_name").and_then(|v| v.as_str()),
                    ) {
                        let _ = conn.execute(
                            "INSERT INTO configured_pttypes (pttype, name, pcode, hipdata_code, short_name) VALUES (?1, ?2, ?3, ?4, ?5)",
                            params![pt, nm, pc, hip, sh],
                        );
                    }
                }
            }
        }

        let pr_count: i64 = conn.query_row("SELECT COUNT(*) FROM configured_procedures", [], |row| row.get(0))?;
        if pr_count == 0 {
            if let Some(arr) = config.get("procedures").and_then(|v| v.as_array()) {
                for item in arr {
                    if let (Some(ic), Some(nm), Some(sh)) = (
                        item.get("icode").and_then(|v| v.as_str()),
                        item.get("name").and_then(|v| v.as_str()),
                        item.get("short_name").and_then(|v| v.as_str()),
                    ) {
                        let _ = conn.execute(
                            "INSERT INTO configured_procedures (icode, name, short_name) VALUES (?1, ?2, ?3)",
                            params![ic, nm, sh],
                        );
                    }
                }
            }
        }

        let dr_count: i64 = conn.query_row("SELECT COUNT(*) FROM configured_drugs", [], |row| row.get(0))?;
        if dr_count == 0 {
            if let Some(arr) = config.get("drugs").and_then(|v| v.as_array()) {
                for item in arr {
                    if let (Some(ic), Some(nm), Some(sh)) = (
                        item.get("icode").and_then(|v| v.as_str()),
                        item.get("name").and_then(|v| v.as_str()),
                        item.get("short_name").and_then(|v| v.as_str()),
                    ) {
                        let _ = conn.execute(
                            "INSERT INTO configured_drugs (icode, name, short_name) VALUES (?1, ?2, ?3)",
                            params![ic, nm, sh],
                        );
                    }
                }
            }
        }

        let pv_count: i64 = conn.query_row("SELECT COUNT(*) FROM configured_providers", [], |row| row.get(0))?;
        if pv_count == 0 {
            if let Some(arr) = config.get("providers").and_then(|v| v.as_array()) {
                for item in arr {
                    if let (Some(pid), Some(nm), Some(sh)) = (
                        item.get("health_med_provider_id").and_then(|v| v.as_i64()),
                        item.get("full_name").and_then(|v| v.as_str()),
                        item.get("short_name").and_then(|v| v.as_str()),
                    ) {
                        let _ = conn.execute(
                            "INSERT INTO configured_providers (health_med_provider_id, full_name, short_name) VALUES (?1, ?2, ?3)",
                            params![pid, nm, sh],
                        );
                    }
                }
            }
        }
    } else {
        // Fallback ถ้า parse json ไม่ผ่าน (ซึ่งไม่ควรเกิดขึ้น)
        let count: i64 = conn.query_row("SELECT COUNT(*) FROM payout_options", [], |row| row.get(0))?;
        if count == 0 {
            conn.execute("INSERT INTO payout_options (amount, label) VALUES (?1, ?2)", params![125.0f64, "ค่าตอบแทน 125 บาท"])?;
            conn.execute("INSERT INTO payout_options (amount, label) VALUES (?1, ?2)", params![20.0f64, "ค่าตอบแทน 20 บาท"])?;
        }
    }

    // ต้องมีอย่างน้อย 1 แถวใน hosxp_connection
    let conn_count: i64 = conn.query_row("SELECT COUNT(*) FROM hosxp_connection", [], |row| {
        row.get(0)
    })?;
    if conn_count == 0 {
        conn.execute(
            "INSERT INTO hosxp_connection (id, host, port, database_name, username, password) VALUES (1, '', 3306, '', '', '')",
            [],
        )?;
    }

    Ok(())
}

// ─────────────────────────────────────────────────────────────────────────────
//  HOSxP Connection Settings
// ─────────────────────────────────────────────────────────────────────────────

pub fn save_hosxp_settings(conn: &Connection, settings: &HosxpSettings) -> SqliteResult<()> {
    conn.execute(
        "INSERT INTO hosxp_connection (id, host, port, database_name, username, password)
         VALUES (1, ?1, ?2, ?3, ?4, ?5)
         ON CONFLICT(id) DO UPDATE SET
            host          = excluded.host,
            port          = excluded.port,
            database_name = excluded.database_name,
            username      = excluded.username,
            password      = excluded.password",
        params![
            settings.host,
            settings.port as i64,
            settings.database_name,
            settings.username,
            settings.password,
        ],
    )?;
    Ok(())
}

pub fn get_hosxp_settings(conn: &Connection) -> SqliteResult<HosxpSettings> {
    let result = conn.query_row(
        "SELECT host, port, database_name, username, password FROM hosxp_connection WHERE id = 1",
        [],
        |row| {
            Ok(HosxpSettings {
                host: row.get::<_, String>(0).unwrap_or_default(),
                port: row.get::<_, i64>(1).unwrap_or(3306) as u16,
                database_name: row.get::<_, String>(2).unwrap_or_default(),
                username: row.get::<_, String>(3).unwrap_or_default(),
                password: row.get::<_, String>(4).unwrap_or_default(),
            })
        },
    );
    match result {
        Ok(s) => Ok(s),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(HosxpSettings::default()),
        Err(e) => Err(e),
    }
}

// ─────────────────────────────────────────────────────────────────────────────
//  Pttypes (สิทธิการรักษา)
// ─────────────────────────────────────────────────────────────────────────────

pub fn save_pttype(
    conn: &Connection,
    pttype: &str,
    name: &str,
    pcode: &str,
    hipdata_code: &str,
    short_name: &str,
) -> SqliteResult<()> {
    conn.execute(
        "INSERT INTO configured_pttypes (pttype, name, pcode, hipdata_code, short_name) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![pttype, name, pcode, hipdata_code, short_name],
    )?;
    Ok(())
}

pub fn get_all_pttypes(conn: &Connection) -> SqliteResult<Vec<PttypeConfig>> {
    let mut stmt = conn.prepare(
        "SELECT id, pttype, name, pcode, hipdata_code, short_name FROM configured_pttypes ORDER BY id"
    )?;
    let rows = stmt.query_map([], |row| {
        Ok(PttypeConfig {
            id: row.get(0)?,
            pttype: row.get::<_, String>(1).unwrap_or_default(),
            name: row.get::<_, String>(2).unwrap_or_default(),
            pcode: row.get::<_, String>(3).unwrap_or_default(),
            hipdata_code: row.get::<_, String>(4).unwrap_or_default(),
            short_name: row.get::<_, String>(5).unwrap_or_default(),
        })
    })?;
    rows.collect()
}

pub fn delete_pttype(conn: &Connection, id: i64) -> SqliteResult<()> {
    conn.execute("DELETE FROM configured_pttypes WHERE id = ?1", params![id])?;
    Ok(())
}

// ─────────────────────────────────────────────────────────────────────────────
//  Procedures (หัตถการ)
// ─────────────────────────────────────────────────────────────────────────────

pub fn save_procedure(
    conn: &Connection,
    icode: &str,
    name: &str,
    short_name: &str,
) -> SqliteResult<()> {
    conn.execute(
        "INSERT INTO configured_procedures (icode, name, short_name) VALUES (?1, ?2, ?3)
         ON CONFLICT(icode) DO UPDATE SET name = excluded.name, short_name = excluded.short_name",
        params![icode, name, short_name],
    )?;
    Ok(())
}

pub fn get_all_procedures(conn: &Connection) -> SqliteResult<Vec<ProcedureConfig>> {
    let mut stmt =
        conn.prepare("SELECT id, icode, name, short_name FROM configured_procedures ORDER BY id")?;
    let rows = stmt.query_map([], |row| {
        Ok(ProcedureConfig {
            id: row.get(0)?,
            icode: row.get::<_, String>(1).unwrap_or_default(),
            name: row.get::<_, String>(2).unwrap_or_default(),
            short_name: row.get::<_, String>(3).unwrap_or_default(),
        })
    })?;
    rows.collect()
}

pub fn delete_procedure(conn: &Connection, id: i64) -> SqliteResult<()> {
    conn.execute(
        "DELETE FROM configured_procedures WHERE id = ?1",
        params![id],
    )?;
    Ok(())
}

// ─────────────────────────────────────────────────────────────────────────────
//  Drugs (ยาสมุนไพร)
// ─────────────────────────────────────────────────────────────────────────────

pub fn save_drug(conn: &Connection, icode: &str, name: &str, short_name: &str) -> SqliteResult<()> {
    conn.execute(
        "INSERT INTO configured_drugs (icode, name, short_name) VALUES (?1, ?2, ?3)
         ON CONFLICT(icode) DO UPDATE SET name = excluded.name, short_name = excluded.short_name",
        params![icode, name, short_name],
    )?;
    Ok(())
}

pub fn get_all_drugs(conn: &Connection) -> SqliteResult<Vec<DrugConfig>> {
    let mut stmt =
        conn.prepare("SELECT id, icode, name, short_name FROM configured_drugs ORDER BY id")?;
    let rows = stmt.query_map([], |row| {
        Ok(DrugConfig {
            id: row.get(0)?,
            icode: row.get::<_, String>(1).unwrap_or_default(),
            name: row.get::<_, String>(2).unwrap_or_default(),
            short_name: row.get::<_, String>(3).unwrap_or_default(),
        })
    })?;
    rows.collect()
}

pub fn delete_drug(conn: &Connection, id: i64) -> SqliteResult<()> {
    conn.execute("DELETE FROM configured_drugs WHERE id = ?1", params![id])?;
    Ok(())
}

// ─────────────────────────────────────────────────────────────────────────────
//  Providers (พนักงาน)
// ─────────────────────────────────────────────────────────────────────────────

pub fn save_provider(
    conn: &Connection,
    health_med_provider_id: i64,
    full_name: &str,
    short_name: &str,
) -> SqliteResult<()> {
    conn.execute(
        "INSERT INTO configured_providers (health_med_provider_id, full_name, short_name) VALUES (?1, ?2, ?3)
         ON CONFLICT(health_med_provider_id) DO UPDATE SET full_name = excluded.full_name, short_name = excluded.short_name",
        params![health_med_provider_id, full_name, short_name],
    )?;
    Ok(())
}

pub fn get_all_providers(conn: &Connection) -> SqliteResult<Vec<ProviderConfig>> {
    let mut stmt = conn.prepare(
        "SELECT id, health_med_provider_id, full_name, short_name FROM configured_providers ORDER BY id",
    )?;
    let rows = stmt.query_map([], |row| {
        Ok(ProviderConfig {
            id: row.get(0)?,
            health_med_provider_id: row.get(1)?,
            full_name: row.get::<_, String>(2).unwrap_or_default(),
            short_name: row.get::<_, String>(3).unwrap_or_default(),
        })
    })?;
    rows.collect()
}

pub fn delete_provider(conn: &Connection, id: i64) -> SqliteResult<()> {
    conn.execute(
        "DELETE FROM configured_providers WHERE id = ?1",
        params![id],
    )?;
    Ok(())
}

// ─────────────────────────────────────────────────────────────────────────────
//  Payout Options (ค่าตอบแทน)
// ─────────────────────────────────────────────────────────────────────────────

pub fn get_payout_options(conn: &Connection) -> SqliteResult<Vec<PayoutOption>> {
    let mut stmt =
        conn.prepare("SELECT id, amount, label FROM payout_options ORDER BY amount DESC")?;
    let rows = stmt.query_map([], |row| {
        Ok(PayoutOption {
            id: row.get(0)?,
            amount: row.get(1)?,
            label: row.get::<_, String>(2).unwrap_or_default(),
        })
    })?;
    rows.collect()
}

pub fn add_payout_option(conn: &Connection, amount: f64, label: &str) -> SqliteResult<()> {
    conn.execute(
        "INSERT INTO payout_options (amount, label) VALUES (?1, ?2)",
        params![amount, label],
    )?;
    Ok(())
}

pub fn delete_payout_option(conn: &Connection, id: i64) -> SqliteResult<()> {
    conn.execute("DELETE FROM payout_options WHERE id = ?1", params![id])?;
    Ok(())
}

// ─────────────────────────────────────────────────────────────────────────────
//  Pending Export
// ─────────────────────────────────────────────────────────────────────────────

pub fn upsert_pending_export(conn: &Connection, r: &UpsertPendingInput) -> SqliteResult<()> {
    conn.execute(
        "INSERT INTO pending_export
            (visit_date, vn, hn, cid, first_name, last_name, gender, age,
             rights, symptoms, procedure, therapist, total_revenue, payout_amount)
         VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13,?14)
         ON CONFLICT(visit_date, vn) DO UPDATE SET
            hn            = excluded.hn,
            cid           = excluded.cid,
            first_name    = excluded.first_name,
            last_name     = excluded.last_name,
            gender        = excluded.gender,
            age           = excluded.age,
            rights        = excluded.rights,
            symptoms      = excluded.symptoms,
            procedure     = excluded.procedure,
            therapist     = excluded.therapist,
            total_revenue = excluded.total_revenue,
            payout_amount = excluded.payout_amount",
        params![
            r.visit_date,
            r.vn,
            r.hn,
            r.cid,
            r.first_name,
            r.last_name,
            r.gender,
            r.age,
            r.rights,
            r.symptoms,
            r.procedure,
            r.therapist,
            r.total_revenue,
            r.payout_amount,
        ],
    )?;
    Ok(())
}

pub fn get_pending_export(
    conn: &Connection,
    date_from: &str,
    date_to: &str,
) -> SqliteResult<Vec<PendingRow>> {
    let mut stmt = conn.prepare(
        "SELECT id, visit_date, vn, hn, cid, first_name, last_name, gender, age,
                rights, symptoms, procedure, therapist, total_revenue, payout_amount, created_at
         FROM pending_export
         WHERE visit_date BETWEEN ?1 AND ?2
         ORDER BY visit_date ASC, vn ASC",
    )?;
    let rows = stmt.query_map(params![date_from, date_to], |row| {
        Ok(PendingRow {
            id: row.get(0)?,
            visit_date: row.get::<_, String>(1).unwrap_or_default(),
            vn: row.get::<_, String>(2).unwrap_or_default(),
            hn: row.get::<_, String>(3).unwrap_or_default(),
            cid: row.get::<_, String>(4).unwrap_or_default(),
            first_name: row.get::<_, String>(5).unwrap_or_default(),
            last_name: row.get::<_, String>(6).unwrap_or_default(),
            gender: row.get::<_, String>(7).unwrap_or_default(),
            age: row.get(8)?,
            rights: row.get::<_, String>(9).unwrap_or_default(),
            symptoms: row.get::<_, String>(10).unwrap_or_default(),
            procedure: row.get::<_, String>(11).unwrap_or_default(),
            therapist: row.get::<_, String>(12).unwrap_or_default(),
            total_revenue: row.get::<_, f64>(13).unwrap_or_default(),
            payout_amount: row.get::<_, f64>(14).unwrap_or_default(),
            created_at: row.get::<_, String>(15).unwrap_or_default(),
        })
    })?;
    rows.collect()
}

pub fn get_locked_vns(conn: &Connection, date: &str) -> SqliteResult<Vec<String>> {
    let mut stmt = conn.prepare("SELECT vn FROM pending_export WHERE visit_date = ?1")?;
    let rows = stmt.query_map(params![date], |row| row.get::<_, String>(0))?;
    rows.collect()
}

pub fn delete_pending_by_id(conn: &Connection, id: i64) -> SqliteResult<()> {
    conn.execute("DELETE FROM pending_export WHERE id = ?1", params![id])?;
    Ok(())
}

pub fn delete_pending_by_range(
    conn: &Connection,
    date_from: &str,
    date_to: &str,
) -> SqliteResult<usize> {
    let n = conn.execute(
        "DELETE FROM pending_export WHERE visit_date BETWEEN ?1 AND ?2",
        params![date_from, date_to],
    )?;
    Ok(n)
}

pub fn get_monthly_stats(conn: &Connection) -> SqliteResult<Vec<MonthlyStats>> {
    let mut stmt = conn.prepare(
        "SELECT strftime('%Y-%m', visit_date) AS month,
                COUNT(*) AS cnt,
                COALESCE(SUM(total_revenue), 0) AS total_revenue,
                COALESCE(SUM(payout_amount), 0) AS total_payout
         FROM pending_export
         GROUP BY strftime('%Y-%m', visit_date)
         ORDER BY month DESC",
    )?;
    let rows = stmt.query_map([], |row| {
        Ok(MonthlyStats {
            month: row.get::<_, String>(0).unwrap_or_default(),
            count: row.get(1)?,
            total_revenue: row.get::<_, f64>(2).unwrap_or_default(),
            total_payout: row.get::<_, f64>(3).unwrap_or_default(),
        })
    })?;
    rows.collect()
}

pub fn preview_delete_range(
    conn: &Connection,
    date_from: &str,
    date_to: &str,
) -> SqliteResult<Vec<PendingRow>> {
    get_pending_export(conn, date_from, date_to)
}

// ─────────────────────────────────────────────────────────────────────────────
//  Tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;

    /// Helper: create an in-memory DB and initialise it.
    fn setup() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        init_config_db(&conn).unwrap();
        conn
    }

    /// Helper: build an `UpsertPendingInput` with sensible defaults.
    fn make_pending(visit_date: &str, vn: &str) -> UpsertPendingInput {
        UpsertPendingInput {
            visit_date: visit_date.to_string(),
            vn: vn.to_string(),
            hn: "HN001".to_string(),
            cid: "1234567890123".to_string(),
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
            gender: "M".to_string(),
            age: Some(30),
            rights: "UCS".to_string(),
            symptoms: "headache".to_string(),
            procedure: "acupuncture".to_string(),
            therapist: "Dr. Smith".to_string(),
            total_revenue: 500.0,
            payout_amount: 125.0,
        }
    }

    // ── init ─────────────────────────────────────────────────────────────

    #[test]
    fn test_init_config_db() {
        let conn = setup();

        // 2 default payout options
        let opts = get_payout_options(&conn).unwrap();
        assert_eq!(opts.len(), 2);

        // 1 default hosxp_connection row
        let count: i64 = conn
            .query_row("SELECT COUNT(*) FROM hosxp_connection", [], |r| r.get(0))
            .unwrap();
        assert_eq!(count, 1);

        // Tables should exist (quick smoke check via SELECT)
        conn.execute_batch(
            "SELECT 1 FROM configured_pttypes LIMIT 0;
             SELECT 1 FROM configured_procedures LIMIT 0;
             SELECT 1 FROM configured_drugs LIMIT 0;
             SELECT 1 FROM configured_providers LIMIT 0;
             SELECT 1 FROM payout_options LIMIT 0;
             SELECT 1 FROM pending_export LIMIT 0;
             SELECT 1 FROM hosxp_connection LIMIT 0;
             SELECT 1 FROM app_settings LIMIT 0;",
        )
        .unwrap();
    }

    // ── hosxp settings ──────────────────────────────────────────────────

    #[test]
    fn test_save_and_get_hosxp_settings() {
        let conn = setup();

        let settings = HosxpSettings {
            host: "192.168.1.100".to_string(),
            port: 3307,
            database_name: "hosxp_db".to_string(),
            username: "admin".to_string(),
            password: "secret".to_string(),
        };
        save_hosxp_settings(&conn, &settings).unwrap();

        let got = get_hosxp_settings(&conn).unwrap();
        assert_eq!(got.host, "192.168.1.100");
        assert_eq!(got.port, 3307);
        assert_eq!(got.database_name, "hosxp_db");
        assert_eq!(got.username, "admin");
        assert_eq!(got.password, "secret");
    }

    #[test]
    fn test_get_hosxp_settings_default() {
        let conn = setup();

        let got = get_hosxp_settings(&conn).unwrap();
        assert_eq!(got.host, "");
        assert_eq!(got.port, 3306);
        assert_eq!(got.database_name, "");
        assert_eq!(got.username, "");
        assert_eq!(got.password, "");
    }

    #[test]
    fn test_hosxp_settings_preserves_case() {
        let conn = setup();

        let settings = HosxpSettings {
            host: "myServer".to_string(),
            port: 3306,
            database_name: "hospDB".to_string(),
            username: "rootUser".to_string(),
            password: "SeCrEt123".to_string(),
        };
        save_hosxp_settings(&conn, &settings).unwrap();

        let got = get_hosxp_settings(&conn).unwrap();
        assert_eq!(got.host, "myServer");
        assert_eq!(got.database_name, "hospDB");
        assert_eq!(got.username, "rootUser");
        assert_eq!(got.password, "SeCrEt123");
    }

    // ── pttypes ─────────────────────────────────────────────────────────

    #[test]
    fn test_save_and_get_pttypes() {
        let conn = setup();

        save_pttype(&conn, "T1", "สิทธิ UCS", "P01", "H01", "UCS").unwrap();
        save_pttype(&conn, "T2", "สิทธิ OFC", "P02", "H02", "OFC").unwrap();

        let all = get_all_pttypes(&conn).unwrap();
        assert_eq!(all.len(), 2);

        assert_eq!(all[0].pttype, "T1");
        assert_eq!(all[0].name, "สิทธิ UCS");
        assert_eq!(all[0].pcode, "P01");
        assert_eq!(all[0].hipdata_code, "H01");
        assert_eq!(all[0].short_name, "UCS");

        assert_eq!(all[1].pttype, "T2");
        assert_eq!(all[1].name, "สิทธิ OFC");
    }

    #[test]
    fn test_delete_pttype() {
        let conn = setup();

        save_pttype(&conn, "T1", "Name1", "P01", "H01", "S1").unwrap();
        let all = get_all_pttypes(&conn).unwrap();
        assert_eq!(all.len(), 1);

        delete_pttype(&conn, all[0].id).unwrap();
        let all = get_all_pttypes(&conn).unwrap();
        assert_eq!(all.len(), 0);
    }

    // ── procedures ──────────────────────────────────────────────────────

    #[test]
    fn test_save_and_get_procedures() {
        let conn = setup();

        save_procedure(&conn, "IC001", "Acupuncture", "ACU").unwrap();
        save_procedure(&conn, "IC002", "Massage", "MAS").unwrap();

        let all = get_all_procedures(&conn).unwrap();
        assert_eq!(all.len(), 2);
        assert_eq!(all[0].icode, "IC001");
        assert_eq!(all[0].name, "Acupuncture");
        assert_eq!(all[0].short_name, "ACU");
        assert_eq!(all[1].icode, "IC002");

        // ON CONFLICT UPDATE: same icode, new name/short_name
        save_procedure(&conn, "IC001", "Acupuncture v2", "ACU2").unwrap();
        let all = get_all_procedures(&conn).unwrap();
        assert_eq!(all.len(), 2); // still 2, not 3
        assert_eq!(all[0].name, "Acupuncture v2");
        assert_eq!(all[0].short_name, "ACU2");
    }

    #[test]
    fn test_delete_procedure() {
        let conn = setup();

        save_procedure(&conn, "IC001", "Acupuncture", "ACU").unwrap();
        let all = get_all_procedures(&conn).unwrap();
        assert_eq!(all.len(), 1);

        delete_procedure(&conn, all[0].id).unwrap();
        let all = get_all_procedures(&conn).unwrap();
        assert_eq!(all.len(), 0);
    }

    // ── drugs ───────────────────────────────────────────────────────────

    #[test]
    fn test_save_and_get_drugs() {
        let conn = setup();

        save_drug(&conn, "D001", "Herbal Tea", "HT").unwrap();
        save_drug(&conn, "D002", "Ginger Extract", "GE").unwrap();

        let all = get_all_drugs(&conn).unwrap();
        assert_eq!(all.len(), 2);
        assert_eq!(all[0].icode, "D001");
        assert_eq!(all[0].name, "Herbal Tea");
        assert_eq!(all[0].short_name, "HT");
        assert_eq!(all[1].icode, "D002");

        // ON CONFLICT UPDATE
        save_drug(&conn, "D001", "Herbal Tea v2", "HT2").unwrap();
        let all = get_all_drugs(&conn).unwrap();
        assert_eq!(all.len(), 2);
        assert_eq!(all[0].name, "Herbal Tea v2");
        assert_eq!(all[0].short_name, "HT2");
    }

    #[test]
    fn test_delete_drug() {
        let conn = setup();

        save_drug(&conn, "D001", "Herbal Tea", "HT").unwrap();
        let all = get_all_drugs(&conn).unwrap();
        assert_eq!(all.len(), 1);

        delete_drug(&conn, all[0].id).unwrap();
        let all = get_all_drugs(&conn).unwrap();
        assert_eq!(all.len(), 0);
    }

    // ── providers ───────────────────────────────────────────────────────

    #[test]
    fn test_save_and_get_providers() {
        let conn = setup();

        save_provider(&conn, 101, "Dr. Alice", "Alice").unwrap();
        save_provider(&conn, 102, "Dr. Bob", "Bob").unwrap();

        let all = get_all_providers(&conn).unwrap();
        assert_eq!(all.len(), 2);
        assert_eq!(all[0].health_med_provider_id, 101);
        assert_eq!(all[0].full_name, "Dr. Alice");
        assert_eq!(all[1].health_med_provider_id, 102);

        // ON CONFLICT UPDATE
        save_provider(&conn, 101, "Dr. Alice Updated", "Alice U").unwrap();
        let all = get_all_providers(&conn).unwrap();
        assert_eq!(all.len(), 2);
        assert_eq!(all[0].full_name, "Dr. Alice Updated");
    }

    #[test]
    fn test_delete_provider() {
        let conn = setup();

        save_provider(&conn, 101, "Dr. Alice", "Alice").unwrap();
        let all = get_all_providers(&conn).unwrap();
        assert_eq!(all.len(), 1);

        delete_provider(&conn, all[0].id).unwrap();
        let all = get_all_providers(&conn).unwrap();
        assert_eq!(all.len(), 0);
    }

    // ── payout options ──────────────────────────────────────────────────

    #[test]
    fn test_payout_options_defaults() {
        let conn = setup();

        let opts = get_payout_options(&conn).unwrap();
        assert_eq!(opts.len(), 2);

        // ORDER BY amount DESC  →  125 first, 20 second
        assert!((opts[0].amount - 125.0).abs() < f64::EPSILON);
        assert!((opts[1].amount - 20.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_add_and_delete_payout_option() {
        let conn = setup();

        add_payout_option(&conn, 50.0, "ค่าตอบแทน 50 บาท").unwrap();

        let opts = get_payout_options(&conn).unwrap();
        assert_eq!(opts.len(), 3);

        // find the newly added one (amount = 50)
        let new_opt = opts
            .iter()
            .find(|o| (o.amount - 50.0).abs() < f64::EPSILON)
            .unwrap();
        assert_eq!(new_opt.label, "ค่าตอบแทน 50 บาท");

        delete_payout_option(&conn, new_opt.id).unwrap();
        let opts = get_payout_options(&conn).unwrap();
        assert_eq!(opts.len(), 2);
    }

    // ── pending export ──────────────────────────────────────────────────

    #[test]
    fn test_upsert_pending_export() {
        let conn = setup();

        let input = make_pending("2024-06-15", "VN001");
        upsert_pending_export(&conn, &input).unwrap();

        let rows = get_pending_export(&conn, "2024-06-01", "2024-06-30").unwrap();
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].vn, "VN001");
        assert_eq!(rows[0].hn, "HN001");
        assert_eq!(rows[0].first_name, "John");
        assert_eq!(rows[0].last_name, "Doe");
        assert_eq!(rows[0].gender, "M");
        assert_eq!(rows[0].age, Some(30));
        assert_eq!(rows[0].rights, "UCS");
        assert_eq!(rows[0].symptoms, "headache");
        assert_eq!(rows[0].procedure, "acupuncture");
        assert_eq!(rows[0].therapist, "Dr. Smith");
        assert!((rows[0].total_revenue - 500.0).abs() < f64::EPSILON);
        assert!((rows[0].payout_amount - 125.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_upsert_pending_export_update() {
        let conn = setup();

        let input = make_pending("2024-06-15", "VN001");
        upsert_pending_export(&conn, &input).unwrap();

        // Upsert same (visit_date, vn) with different values
        let updated = UpsertPendingInput {
            visit_date: "2024-06-15".to_string(),
            vn: "VN001".to_string(),
            hn: "HN999".to_string(),
            cid: "9999999999999".to_string(),
            first_name: "Jane".to_string(),
            last_name: "Smith".to_string(),
            gender: "F".to_string(),
            age: Some(25),
            rights: "OFC".to_string(),
            symptoms: "back pain".to_string(),
            procedure: "massage".to_string(),
            therapist: "Dr. Jones".to_string(),
            total_revenue: 800.0,
            payout_amount: 200.0,
        };
        upsert_pending_export(&conn, &updated).unwrap();

        let rows = get_pending_export(&conn, "2024-06-01", "2024-06-30").unwrap();
        assert_eq!(rows.len(), 1); // still 1 row, not 2
        assert_eq!(rows[0].hn, "HN999");
        assert_eq!(rows[0].first_name, "Jane");
        assert_eq!(rows[0].last_name, "Smith");
        assert_eq!(rows[0].gender, "F");
        assert_eq!(rows[0].age, Some(25));
        assert_eq!(rows[0].rights, "OFC");
        assert_eq!(rows[0].symptoms, "back pain");
        assert_eq!(rows[0].procedure, "massage");
        assert_eq!(rows[0].therapist, "Dr. Jones");
        assert!((rows[0].total_revenue - 800.0).abs() < f64::EPSILON);
        assert!((rows[0].payout_amount - 200.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_get_pending_export_date_range() {
        let conn = setup();

        upsert_pending_export(&conn, &make_pending("2024-05-01", "VN-MAY")).unwrap();
        upsert_pending_export(&conn, &make_pending("2024-06-10", "VN-JUN1")).unwrap();
        upsert_pending_export(&conn, &make_pending("2024-06-20", "VN-JUN2")).unwrap();
        upsert_pending_export(&conn, &make_pending("2024-07-05", "VN-JUL")).unwrap();

        // Query June only
        let rows = get_pending_export(&conn, "2024-06-01", "2024-06-30").unwrap();
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].vn, "VN-JUN1");
        assert_eq!(rows[1].vn, "VN-JUN2");

        // Query all
        let rows = get_pending_export(&conn, "2024-01-01", "2024-12-31").unwrap();
        assert_eq!(rows.len(), 4);
    }

    #[test]
    fn test_get_locked_vns() {
        let conn = setup();

        upsert_pending_export(&conn, &make_pending("2024-06-15", "VN001")).unwrap();
        upsert_pending_export(&conn, &make_pending("2024-06-15", "VN002")).unwrap();
        upsert_pending_export(&conn, &make_pending("2024-06-16", "VN003")).unwrap();

        let vns = get_locked_vns(&conn, "2024-06-15").unwrap();
        assert_eq!(vns.len(), 2);
        assert!(vns.contains(&"VN001".to_string()));
        assert!(vns.contains(&"VN002".to_string()));

        // Different date
        let vns = get_locked_vns(&conn, "2024-06-16").unwrap();
        assert_eq!(vns.len(), 1);
        assert_eq!(vns[0], "VN003");

        // No records
        let vns = get_locked_vns(&conn, "2024-01-01").unwrap();
        assert_eq!(vns.len(), 0);
    }

    #[test]
    fn test_delete_pending_by_id() {
        let conn = setup();

        upsert_pending_export(&conn, &make_pending("2024-06-15", "VN001")).unwrap();
        let rows = get_pending_export(&conn, "2024-06-01", "2024-06-30").unwrap();
        assert_eq!(rows.len(), 1);

        delete_pending_by_id(&conn, rows[0].id).unwrap();

        let rows = get_pending_export(&conn, "2024-06-01", "2024-06-30").unwrap();
        assert_eq!(rows.len(), 0);
    }

    #[test]
    fn test_delete_pending_by_range() {
        let conn = setup();

        upsert_pending_export(&conn, &make_pending("2024-06-10", "VN-A")).unwrap();
        upsert_pending_export(&conn, &make_pending("2024-06-20", "VN-B")).unwrap();
        upsert_pending_export(&conn, &make_pending("2024-07-05", "VN-C")).unwrap();

        let deleted = delete_pending_by_range(&conn, "2024-06-01", "2024-06-30").unwrap();
        assert_eq!(deleted, 2);

        // Only the July record should remain
        let rows = get_pending_export(&conn, "2024-01-01", "2024-12-31").unwrap();
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].vn, "VN-C");
    }

    // ── stats / preview ─────────────────────────────────────────────────

    #[test]
    fn test_get_monthly_stats() {
        let conn = setup();

        // June: 2 records
        let mut jun1 = make_pending("2024-06-10", "VN-J1");
        jun1.total_revenue = 500.0;
        jun1.payout_amount = 125.0;
        upsert_pending_export(&conn, &jun1).unwrap();

        let mut jun2 = make_pending("2024-06-20", "VN-J2");
        jun2.total_revenue = 300.0;
        jun2.payout_amount = 20.0;
        upsert_pending_export(&conn, &jun2).unwrap();

        // July: 1 record
        let mut jul1 = make_pending("2024-07-05", "VN-JL1");
        jul1.total_revenue = 1000.0;
        jul1.payout_amount = 125.0;
        upsert_pending_export(&conn, &jul1).unwrap();

        let stats = get_monthly_stats(&conn).unwrap();
        assert_eq!(stats.len(), 2);

        // ORDER BY month DESC  →  July first
        assert_eq!(stats[0].month, "2024-07");
        assert_eq!(stats[0].count, 1);
        assert!((stats[0].total_revenue - 1000.0).abs() < f64::EPSILON);
        assert!((stats[0].total_payout - 125.0).abs() < f64::EPSILON);

        assert_eq!(stats[1].month, "2024-06");
        assert_eq!(stats[1].count, 2);
        assert!((stats[1].total_revenue - 800.0).abs() < f64::EPSILON);
        assert!((stats[1].total_payout - 145.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_preview_delete_range() {
        let conn = setup();

        upsert_pending_export(&conn, &make_pending("2024-06-10", "VN-A")).unwrap();
        upsert_pending_export(&conn, &make_pending("2024-06-20", "VN-B")).unwrap();
        upsert_pending_export(&conn, &make_pending("2024-07-05", "VN-C")).unwrap();

        let preview = preview_delete_range(&conn, "2024-06-01", "2024-06-30").unwrap();
        let actual = get_pending_export(&conn, "2024-06-01", "2024-06-30").unwrap();

        assert_eq!(preview.len(), actual.len());
        assert_eq!(preview.len(), 2);
        for (p, a) in preview.iter().zip(actual.iter()) {
            assert_eq!(p.id, a.id);
            assert_eq!(p.vn, a.vn);
            assert_eq!(p.visit_date, a.visit_date);
        }
    }
}
