// db/hosxp.rs — HOSxP MariaDB connection and queries (READ-ONLY)
// ไม่มีการแก้ไขข้อมูลใน HOSxP เป็นอันขาด — SELECT เท่านั้น

use mysql::prelude::*;
use mysql::{Opts, OptsBuilder, Params, Pool, Value};

use crate::models::*;

// ─────────────────────────────────────────────────────────────────────────────
//  Connection
// ─────────────────────────────────────────────────────────────────────────────

/// สร้าง MySQL Pool สำหรับเชื่อมต่อ HOSxP (read-only)
pub fn create_pool(settings: &HosxpSettings) -> Result<Pool, String> {
    let opts = OptsBuilder::new()
        .ip_or_hostname(Some(settings.host.as_str()))
        .tcp_port(settings.port)
        .db_name(Some(settings.database_name.as_str()))
        .user(Some(settings.username.as_str()))
        .pass(Some(settings.password.as_str()))
        .tcp_connect_timeout(Some(std::time::Duration::from_secs(10)))
        .read_timeout(Some(std::time::Duration::from_secs(60)))
        .write_timeout(Some(std::time::Duration::from_secs(60)))
        // ★ FIX: ระบุ charset เพื่อให้ MySQL แปลงข้อมูลจาก TIS-620/latin1 → UTF-8
        // ป้องกันปัญหา Bytes ที่ไม่ใช่ UTF-8 ทำให้โปรแกรม panic
        .init(vec!["SET NAMES utf8mb4"]);

    Pool::new(Opts::from(opts)).map_err(|e| format!("ไม่สามารถสร้าง connection pool: {}", e))
}

/// ทดสอบการเชื่อมต่อ HOSxP
pub fn test_connection(settings: &HosxpSettings) -> ConnectionTestResult {
    match create_pool(settings) {
        Err(e) => ConnectionTestResult {
            ok: false,
            message: format!("เชื่อมต่อไม่สำเร็จ ❌\n{}", e),
        },
        Ok(pool) => match pool.get_conn() {
            Err(e) => ConnectionTestResult {
                ok: false,
                message: format!("เชื่อมต่อไม่สำเร็จ ❌\n{}", e),
            },
            Ok(mut conn) => match conn.query_first::<String, _>("SELECT VERSION()") {
                Ok(Some(version)) => ConnectionTestResult {
                    ok: true,
                    message: format!(
                        "เชื่อมต่อสำเร็จ ✅\nMySQL/MariaDB Version: {}\nHost: {}:{}\nDatabase: {}",
                        version, settings.host, settings.port, settings.database_name
                    ),
                },
                Ok(None) => ConnectionTestResult {
                    ok: true,
                    message: format!(
                        "เชื่อมต่อสำเร็จ ✅\nHost: {}:{}\nDatabase: {}",
                        settings.host, settings.port, settings.database_name
                    ),
                },
                Err(e) => ConnectionTestResult {
                    ok: false,
                    message: format!("เชื่อมต่อสำเร็จแต่ query ล้มเหลว: {}", e),
                },
            },
        },
    }
}

// ─────────────────────────────────────────────────────────────────────────────
//  Lookup Queries (ใช้ในหน้าตั้งค่า)
// ─────────────────────────────────────────────────────────────────────────────

/// ค้นหาสิทธิการรักษาจาก hipdata_code
/// ★ FIX: ใช้ mysql::Row แทน tuple เพื่อป้องกัน panic จาก FromRow conversion
pub fn fetch_pttype_by_hipdata(
    pool: &Pool,
    hipdata_code: &str,
) -> Result<Option<PttypeLookup>, String> {
    let mut conn = pool.get_conn().map_err(|e| e.to_string())?;
    let result: Option<mysql::Row> = conn
        .exec_first(
            "SELECT pttype, name, pcode, hipdata_code FROM pttype WHERE hipdata_code = ? LIMIT 1",
            (hipdata_code,),
        )
        .map_err(|e| format!("ค้นหาสิทธิการรักษาไม่สำเร็จ: {}", e))?;

    Ok(result.map(|mut row| PttypeLookup {
        pttype: row
            .take_opt("pttype")
            .unwrap_or(Ok(String::new()))
            .unwrap_or_default(),
        name: row
            .take_opt("name")
            .unwrap_or(Ok(String::new()))
            .unwrap_or_default(),
        pcode: row
            .take_opt("pcode")
            .unwrap_or(Ok(String::new()))
            .unwrap_or_default(),
        hipdata_code: row
            .take_opt("hipdata_code")
            .unwrap_or(Ok(String::new()))
            .unwrap_or_default(),
    }))
}

/// ค้นหาหัตถการจาก icode (nondrugitems)
/// ★ FIX: ใช้ mysql::Row แทน tuple เพื่อป้องกัน panic จาก FromRow conversion
pub fn fetch_procedure_by_icode(pool: &Pool, icode: &str) -> Result<Option<ItemLookup>, String> {
    let mut conn = pool.get_conn().map_err(|e| e.to_string())?;
    let result: Option<mysql::Row> = conn
        .exec_first(
            "SELECT icode, name FROM nondrugitems WHERE icode = ? LIMIT 1",
            (icode,),
        )
        .map_err(|e| format!("ค้นหาหัตถการไม่สำเร็จ: {}", e))?;

    Ok(result.map(|mut row| ItemLookup {
        icode: row
            .take_opt("icode")
            .unwrap_or(Ok(String::new()))
            .unwrap_or_default(),
        name: row
            .take_opt("name")
            .unwrap_or(Ok(String::new()))
            .unwrap_or_default(),
    }))
}

/// ค้นหายาสมุนไพรจาก icode (drugitems)
/// ★ FIX: ใช้ mysql::Row แทน tuple เพื่อป้องกัน panic จาก FromRow conversion
pub fn fetch_drug_by_icode(pool: &Pool, icode: &str) -> Result<Option<ItemLookup>, String> {
    let mut conn = pool.get_conn().map_err(|e| e.to_string())?;
    let result: Option<mysql::Row> = conn
        .exec_first(
            "SELECT icode, name FROM drugitems WHERE icode = ? LIMIT 1",
            (icode,),
        )
        .map_err(|e| format!("ค้นหายาสมุนไพรไม่สำเร็จ: {}", e))?;

    Ok(result.map(|mut row| ItemLookup {
        icode: row
            .take_opt("icode")
            .unwrap_or(Ok(String::new()))
            .unwrap_or_default(),
        name: row
            .take_opt("name")
            .unwrap_or(Ok(String::new()))
            .unwrap_or_default(),
    }))
}

/// ค้นหาพนักงานจาก health_med_provider_id
/// ★ FIX: ใช้ mysql::Row แทน tuple เพื่อป้องกัน panic จาก FromRow conversion
pub fn fetch_provider_by_id(
    pool: &Pool,
    provider_id: i64,
) -> Result<Option<ProviderLookup>, String> {
    let mut conn = pool.get_conn().map_err(|e| e.to_string())?;
    let result: Option<mysql::Row> = conn.exec_first(
        "SELECT health_med_provider_id, health_med_provider_full_name FROM health_med_provider WHERE health_med_provider_id = ? LIMIT 1",
        (provider_id,),
    ).map_err(|e| format!("ค้นหาพนักงานไม่สำเร็จ: {}", e))?;

    Ok(result.map(|mut row| ProviderLookup {
        health_med_provider_id: row
            .take_opt("health_med_provider_id")
            .unwrap_or(Ok(0i64))
            .unwrap_or_default(),
        health_med_provider_full_name: row
            .take_opt("health_med_provider_full_name")
            .unwrap_or(Ok(String::new()))
            .unwrap_or_default(),
    }))
}

// ─────────────────────────────────────────────────────────────────────────────
//  Main Patient Query
// ─────────────────────────────────────────────────────────────────────────────

/// สร้าง SQL query แบบ dynamic สำหรับดึงข้อมูลผู้ป่วย
fn build_patient_sql(
    pttype_count: usize,
    provider_count: usize,
    icode_count: usize,
    procedure_count: usize,
) -> String {
    let pttype_ph = vec!["?"; pttype_count].join(", ");
    let provider_ph = vec!["?"; provider_count].join(", ");
    let icode_ph = vec!["?"; icode_count].join(", ");
    let proc_ph = vec!["?"; procedure_count].join(", ");

    format!(
        r#"
SELECT
    v.vn AS vn,
    p.cid AS cid,
    v.hn AS hn,
    p.fname AS first_name,
    p.lname AS last_name,
    s.name AS gender,
    v.age_y AS age,
    CASE
        WHEN pt.hipdata_code IN ({pttype_ph}) THEN pt.name
        ELSE 'HIPDATA_CODE_NOT_DEFINED'
    END AS pttype_display_name,
    pt.hipdata_code AS pttype_hipdata_code,
    os.cc AS chief_complaint,
    bill.item_names AS item_names_from_opitemrece,
    bill.icode_list AS all_icodes,
    GROUP_CONCAT(DISTINCT
        CASE
            WHEN prov.health_med_provider_id IN ({provider_ph}) THEN prov.health_med_provider_full_name
            ELSE 'PROVIDER_ID_NOT_DEFINED'
        END
    ORDER BY prov.health_med_provider_full_name SEPARATOR ', ') AS provider_names,
    bill.final_total_price AS total_sum_price,
    hm.service_date AS service_date
FROM vn_stat v
INNER JOIN patient p ON v.hn = p.hn
LEFT JOIN sex s ON p.sex = s.code
LEFT JOIN pttype pt ON v.pttype = pt.pttype
LEFT JOIN opdscreen os ON v.vn = os.vn
INNER JOIN health_med_service hm ON v.vn = hm.vn
INNER JOIN health_med_service_operation hmo ON hm.health_med_service_id = hmo.health_med_service_id
LEFT JOIN health_med_provider prov ON hmo.health_med_provider_id = prov.health_med_provider_id
INNER JOIN (
    SELECT
        res.vn,
        GROUP_CONCAT(DISTINCT res.item_name ORDER BY res.item_name SEPARATOR ', ') AS item_names,
        GROUP_CONCAT(DISTINCT res.icode ORDER BY res.icode SEPARATOR ', ')          AS icode_list,
        SUM(res.max_item_price)                                                      AS final_total_price
    FROM (
        SELECT
            opi.vn,
            opi.icode,
            IFNULL(d.name, nd.name) AS item_name,
            MAX(opi.sum_price)       AS max_item_price
        FROM opitemrece opi
        LEFT JOIN drugitems    d  ON opi.icode = d.icode
        LEFT JOIN nondrugitems nd ON opi.icode = nd.icode
        WHERE opi.vstdate BETWEEN ? AND ?
          AND opi.icode IN ({icode_ph})
        GROUP BY opi.vn, opi.icode
    ) AS res
    GROUP BY res.vn
) AS bill ON v.vn = bill.vn
WHERE hm.service_date BETWEEN ? AND ?
  AND hmo.service_icode IN ({proc_ph})
GROUP BY v.vn
ORDER BY v.vn ASC
"#,
        pttype_ph = pttype_ph,
        provider_ph = provider_ph,
        icode_ph = icode_ph,
        proc_ph = proc_ph
    )
}

/// ดึงข้อมูลผู้ป่วยจาก HOSxP (READ-ONLY)
pub fn fetch_patient_data(
    pool: &Pool,
    date_from: &str,
    date_to: &str,
    pttype_codes: &[String],
    provider_ids: &[i64],
    all_icodes: &[String],
    procedure_icodes: &[String],
) -> Result<Vec<PatientRow>, String> {
    // Fallback for empty lists
    let pt_codes: Vec<String> = if pttype_codes.is_empty() {
        vec!["__NONE__".to_string()]
    } else {
        pttype_codes.to_vec()
    };
    let prov_ids: Vec<i64> = if provider_ids.is_empty() {
        vec![-1]
    } else {
        provider_ids.to_vec()
    };
    let icodes: Vec<String> = if all_icodes.is_empty() {
        vec!["__NONE__".to_string()]
    } else {
        all_icodes.to_vec()
    };
    let proc_codes: Vec<String> = if procedure_icodes.is_empty() {
        vec!["__NONE__".to_string()]
    } else {
        procedure_icodes.to_vec()
    };

    let sql = build_patient_sql(
        pt_codes.len(),
        prov_ids.len(),
        icodes.len(),
        proc_codes.len(),
    );

    // Build positional params in exact order of ? placeholders:
    // 1. pttype_hipdata_codes (CASE WHEN pt.hipdata_code IN (...))
    // 2. provider_ids (CASE WHEN prov.health_med_provider_id IN (...))
    // 3. date_from, date_to (opi.vstdate BETWEEN ? AND ?)
    // 4. all_icodes (opi.icode IN (...))
    // 5. date_from, date_to (hm.service_date BETWEEN ? AND ?)
    // 6. procedure_icodes (hmo.service_icode IN (...))
    let mut params: Vec<Value> = Vec::new();
    for c in &pt_codes {
        params.push(Value::from(c.as_str()));
    }
    for id in &prov_ids {
        params.push(Value::from(*id));
    }
    params.push(Value::from(date_from));
    params.push(Value::from(date_to));
    for ic in &icodes {
        params.push(Value::from(ic.as_str()));
    }
    params.push(Value::from(date_from));
    params.push(Value::from(date_to));
    for ic in &proc_codes {
        params.push(Value::from(ic.as_str()));
    }

    let mut conn = pool
        .get_conn()
        .map_err(|e| format!("ไม่สามารถเชื่อมต่อ HOSxP: {}", e))?;

    let rows: Vec<mysql::Row> = conn
        .exec(sql.trim(), Params::Positional(params))
        .map_err(|e| format!("ไม่สามารถดึงข้อมูลผู้ป่วย: {}", e))?;

    let mut result = Vec::with_capacity(rows.len());
    for mut row in rows {
        // Extract nullable values safely
        let vn: String = row
            .take_opt("vn")
            .unwrap_or(Ok(String::new()))
            .unwrap_or_default();
        let cid: String = row
            .take_opt("cid")
            .unwrap_or(Ok(String::new()))
            .unwrap_or_default();
        let hn: String = row
            .take_opt("hn")
            .unwrap_or(Ok(String::new()))
            .unwrap_or_default();
        let first_name: String = row
            .take_opt("first_name")
            .unwrap_or(Ok(String::new()))
            .unwrap_or_default();
        let last_name: String = row
            .take_opt("last_name")
            .unwrap_or(Ok(String::new()))
            .unwrap_or_default();
        let gender: String = row
            .take_opt("gender")
            .unwrap_or(Ok(String::new()))
            .unwrap_or_default();
        let age: Option<i64> = row.take_opt("age").unwrap_or(Ok(None)).unwrap_or(None);
        let pttype_display_name: String = row
            .take_opt("pttype_display_name")
            .unwrap_or(Ok(String::new()))
            .unwrap_or_default();
        let pttype_hipdata_code: String = row
            .take_opt("pttype_hipdata_code")
            .unwrap_or(Ok(String::new()))
            .unwrap_or_default();
        let chief_complaint: String = row
            .take_opt("chief_complaint")
            .unwrap_or(Ok(String::new()))
            .unwrap_or_default();
        let item_names: String = row
            .take_opt("item_names_from_opitemrece")
            .unwrap_or(Ok(String::new()))
            .unwrap_or_default();
        let all_icodes_val: String = row
            .take_opt("all_icodes")
            .unwrap_or(Ok(String::new()))
            .unwrap_or_default();
        let provider_names: String = row
            .take_opt("provider_names")
            .unwrap_or(Ok(String::new()))
            .unwrap_or_default();
        let total_sum_price: f64 = row
            .take_opt("total_sum_price")
            .unwrap_or(Ok(0.0f64))
            .unwrap_or_default();
        let service_date: String = row
            .take_opt::<String, _>("service_date")
            .unwrap_or(Ok(String::new()))
            .unwrap_or_default();

        result.push(PatientRow {
            vn,
            cid,
            hn,
            first_name,
            last_name,
            gender,
            age,
            pttype_display_name,
            pttype_hipdata_code,
            chief_complaint,
            item_names,
            all_icodes: all_icodes_val,
            provider_names,
            total_sum_price,
            service_date,
        });
    }

    Ok(result)
}

// ─────────────────────────────────────────────────────────────────────────────
//  Unit Tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_patient_sql_single_params() {
        let sql = build_patient_sql(1, 1, 1, 1);
        // Should contain single ? for each IN clause
        assert!(sql.contains("pt.hipdata_code IN (?)"));
        assert!(sql.contains("prov.health_med_provider_id IN (?)"));
        assert!(sql.contains("opi.icode IN (?)"));
        assert!(sql.contains("hmo.service_icode IN (?)"));
        // Should contain date range placeholders
        assert!(sql.contains("opi.vstdate BETWEEN ? AND ?"));
        assert!(sql.contains("hm.service_date BETWEEN ? AND ?"));
    }

    #[test]
    fn test_build_patient_sql_multiple_params() {
        let sql = build_patient_sql(3, 2, 4, 2);
        assert!(sql.contains("pt.hipdata_code IN (?, ?, ?)"));
        assert!(sql.contains("prov.health_med_provider_id IN (?, ?)"));
        assert!(sql.contains("opi.icode IN (?, ?, ?, ?)"));
        assert!(sql.contains("hmo.service_icode IN (?, ?)"));
    }

    #[test]
    fn test_build_patient_sql_structure() {
        let sql = build_patient_sql(1, 1, 1, 1);
        // Should be a SELECT statement
        assert!(sql.contains("SELECT"));
        // Should have proper JOINs
        assert!(sql.contains("INNER JOIN patient p ON v.hn = p.hn"));
        assert!(sql.contains("LEFT JOIN sex s ON p.sex = s.code"));
        assert!(sql.contains("LEFT JOIN pttype pt ON v.pttype = pt.pttype"));
        // Should have GROUP BY and ORDER BY
        assert!(sql.contains("GROUP BY v.vn"));
        assert!(sql.contains("ORDER BY v.vn ASC"));
    }

    #[test]
    fn test_connection_test_result_serialization() {
        let result = ConnectionTestResult {
            ok: true,
            message: "เชื่อมต่อสำเร็จ".to_string(),
        };
        let json = serde_json::to_string(&result).unwrap();
        assert!(json.contains("\"ok\":true"));
        assert!(json.contains("เชื่อมต่อสำเร็จ"));
    }
}
