// models.rs — Data structures shared between DB layer and Tauri commands

use serde::{Deserialize, Serialize};

/// การตั้งค่าการเชื่อมต่อ HOSxP
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HosxpSettings {
    pub host: String,
    pub port: u16,
    pub database_name: String,
    pub username: String,
    pub password: String,
}

/// สิทธิการรักษาที่ตั้งค่าไว้
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PttypeConfig {
    pub id: i64,
    pub pttype: String,
    pub name: String,
    pub pcode: String,
    pub hipdata_code: String,
    pub short_name: String,
}

/// หัตถการที่ตั้งค่าไว้
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcedureConfig {
    pub id: i64,
    pub icode: String,
    pub name: String,
    pub short_name: String,
}

/// ยาสมุนไพรที่ตั้งค่าไว้
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DrugConfig {
    pub id: i64,
    pub icode: String,
    pub name: String,
    pub short_name: String,
}

/// พนักงานที่ตั้งค่าไว้
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderConfig {
    pub id: i64,
    pub health_med_provider_id: i64,
    pub full_name: String,
    pub short_name: String,
}

/// ตัวเลือกค่าตอบแทน
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayoutOption {
    pub id: i64,
    pub amount: f64,
    pub label: String,
}

/// ข้อมูลผู้ป่วยจาก HOSxP
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientRow {
    pub vn: String,
    pub cid: String,
    pub hn: String,
    pub first_name: String,
    pub last_name: String,
    pub gender: String,
    pub age: Option<i64>,
    pub pttype_display_name: String,
    pub pttype_hipdata_code: String,
    pub chief_complaint: String,
    pub item_names: String,
    pub all_icodes: String,
    pub provider_names: String,
    pub total_sum_price: f64,
    pub service_date: String,
}

/// ข้อมูลรอการส่งออก (pending_export)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PendingRow {
    pub id: i64,
    pub visit_date: String,
    pub vn: String,
    pub hn: String,
    pub cid: String,
    pub first_name: String,
    pub last_name: String,
    pub gender: String,
    pub age: Option<i64>,
    pub rights: String,
    pub symptoms: String,
    pub procedure: String,
    pub therapist: String,
    pub total_revenue: f64,
    pub payout_amount: f64,
    pub created_at: String,
}

/// สถิติของข้อมูลรอการส่งออกรายเดือน
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonthlyStats {
    pub month: String,
    pub count: i64,
    pub total_revenue: f64,
    pub total_payout: f64,
}

/// ผลลัพธ์ของการทดสอบการเชื่อมต่อ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionTestResult {
    pub ok: bool,
    pub message: String,
}

/// ข้อมูลที่ส่งมาจาก frontend เพื่อบันทึก pending_export
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpsertPendingInput {
    pub visit_date: String,
    pub vn: String,
    pub hn: String,
    pub cid: String,
    pub first_name: String,
    pub last_name: String,
    pub gender: String,
    pub age: Option<i64>,
    pub rights: String,
    pub symptoms: String,
    pub procedure: String,
    pub therapist: String,
    pub total_revenue: f64,
    pub payout_amount: f64,
}

/// ผลลัพธ์สำหรับ lookup ใน HOSxP
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PttypeLookup {
    pub pttype: String,
    pub name: String,
    pub pcode: String,
    pub hipdata_code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemLookup {
    pub icode: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderLookup {
    pub health_med_provider_id: i64,
    pub health_med_provider_full_name: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_hosxp_settings_default() {
        let settings = HosxpSettings::default();
        assert_eq!(settings.host, "");
        assert_eq!(settings.port, 0);
        assert_eq!(settings.database_name, "");
        assert_eq!(settings.username, "");
        assert_eq!(settings.password, "");
    }

    #[test]
    fn test_hosxp_settings_serialization() {
        let settings = HosxpSettings {
            host: "localhost".to_string(),
            port: 3306,
            database_name: "hosxp".to_string(),
            username: "admin".to_string(),
            password: "secret".to_string(),
        };
        let json = serde_json::to_string(&settings).unwrap();
        let deserialized: HosxpSettings = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.host, "localhost");
        assert_eq!(deserialized.port, 3306);
        assert_eq!(deserialized.database_name, "hosxp");
        assert_eq!(deserialized.username, "admin");
        assert_eq!(deserialized.password, "secret");
    }

    #[test]
    fn test_patient_row_serialization() {
        let row = PatientRow {
            vn: "VN001".to_string(),
            cid: "1234567890123".to_string(),
            hn: "HN001".to_string(),
            first_name: "สมชาย".to_string(),
            last_name: "ใจดี".to_string(),
            gender: "M".to_string(),
            age: Some(45),
            pttype_display_name: "UC".to_string(),
            pttype_hipdata_code: "T01".to_string(),
            chief_complaint: "ปวดหลัง".to_string(),
            item_names: "นวดแผนไทย".to_string(),
            all_icodes: "IC001".to_string(),
            provider_names: "หมอสม".to_string(),
            total_sum_price: 500.0,
            service_date: "2024-01-15".to_string(),
        };
        let json = serde_json::to_string(&row).unwrap();
        let deserialized: PatientRow = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.vn, "VN001");
        assert_eq!(deserialized.cid, "1234567890123");
        assert_eq!(deserialized.hn, "HN001");
        assert_eq!(deserialized.first_name, "สมชาย");
        assert_eq!(deserialized.last_name, "ใจดี");
        assert_eq!(deserialized.gender, "M");
        assert_eq!(deserialized.age, Some(45));
        assert_eq!(deserialized.pttype_display_name, "UC");
        assert_eq!(deserialized.pttype_hipdata_code, "T01");
        assert_eq!(deserialized.chief_complaint, "ปวดหลัง");
        assert_eq!(deserialized.item_names, "นวดแผนไทย");
        assert_eq!(deserialized.all_icodes, "IC001");
        assert_eq!(deserialized.provider_names, "หมอสม");
        assert_eq!(deserialized.total_sum_price, 500.0);
        assert_eq!(deserialized.service_date, "2024-01-15");
    }

    #[test]
    fn test_pending_row_serialization() {
        // Test with age = None
        let row_none_age = PendingRow {
            id: 1,
            visit_date: "2024-01-15".to_string(),
            vn: "VN001".to_string(),
            hn: "HN001".to_string(),
            cid: "1234567890123".to_string(),
            first_name: "สมชาย".to_string(),
            last_name: "ใจดี".to_string(),
            gender: "M".to_string(),
            age: None,
            rights: "UC".to_string(),
            symptoms: "ปวดหลัง".to_string(),
            procedure: "นวดแผนไทย".to_string(),
            therapist: "หมอสม".to_string(),
            total_revenue: 500.0,
            payout_amount: 300.0,
            created_at: "2024-01-15T10:00:00".to_string(),
        };
        let json_none = serde_json::to_string(&row_none_age).unwrap();
        assert!(json_none.contains("\"age\":null"));
        let deserialized_none: PendingRow = serde_json::from_str(&json_none).unwrap();
        assert_eq!(deserialized_none.age, None);

        // Test with age = Some(25)
        let row_some_age = PendingRow {
            id: 2,
            visit_date: "2024-02-20".to_string(),
            vn: "VN002".to_string(),
            hn: "HN002".to_string(),
            cid: "9876543210987".to_string(),
            first_name: "สมหญิง".to_string(),
            last_name: "รักดี".to_string(),
            gender: "F".to_string(),
            age: Some(25),
            rights: "ประกันสังคม".to_string(),
            symptoms: "ปวดคอ".to_string(),
            procedure: "ประคบสมุนไพร".to_string(),
            therapist: "หมอหญิง".to_string(),
            total_revenue: 800.0,
            payout_amount: 500.0,
            created_at: "2024-02-20T14:30:00".to_string(),
        };
        let json_some = serde_json::to_string(&row_some_age).unwrap();
        assert!(json_some.contains("\"age\":25"));
        let deserialized_some: PendingRow = serde_json::from_str(&json_some).unwrap();
        assert_eq!(deserialized_some.age, Some(25));
        assert_eq!(deserialized_some.id, 2);
        assert_eq!(deserialized_some.first_name, "สมหญิง");
        assert_eq!(deserialized_some.total_revenue, 800.0);
        assert_eq!(deserialized_some.payout_amount, 500.0);
    }

    #[test]
    fn test_upsert_pending_input_serialization() {
        let input = UpsertPendingInput {
            visit_date: "2024-03-10".to_string(),
            vn: "VN003".to_string(),
            hn: "HN003".to_string(),
            cid: "1111111111111".to_string(),
            first_name: "ทดสอบ".to_string(),
            last_name: "ระบบ".to_string(),
            gender: "M".to_string(),
            age: Some(30),
            rights: "UC".to_string(),
            symptoms: "ปวดเข่า".to_string(),
            procedure: "นวดแผนไทย".to_string(),
            therapist: "หมอทด".to_string(),
            total_revenue: 1000.0,
            payout_amount: 600.0,
        };
        let json = serde_json::to_string(&input).unwrap();
        let deserialized: UpsertPendingInput = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.visit_date, "2024-03-10");
        assert_eq!(deserialized.vn, "VN003");
        assert_eq!(deserialized.hn, "HN003");
        assert_eq!(deserialized.cid, "1111111111111");
        assert_eq!(deserialized.first_name, "ทดสอบ");
        assert_eq!(deserialized.last_name, "ระบบ");
        assert_eq!(deserialized.gender, "M");
        assert_eq!(deserialized.age, Some(30));
        assert_eq!(deserialized.rights, "UC");
        assert_eq!(deserialized.symptoms, "ปวดเข่า");
        assert_eq!(deserialized.procedure, "นวดแผนไทย");
        assert_eq!(deserialized.therapist, "หมอทด");
        assert_eq!(deserialized.total_revenue, 1000.0);
        assert_eq!(deserialized.payout_amount, 600.0);
    }

    #[test]
    fn test_connection_test_result() {
        let result = ConnectionTestResult {
            ok: true,
            message: "เชื่อมต่อสำเร็จ".to_string(),
        };
        let json = serde_json::to_string(&result).unwrap();
        let deserialized: ConnectionTestResult = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.ok, true);
        assert_eq!(deserialized.message, "เชื่อมต่อสำเร็จ");

        let fail_result = ConnectionTestResult {
            ok: false,
            message: "Connection refused".to_string(),
        };
        let json_fail = serde_json::to_string(&fail_result).unwrap();
        let deserialized_fail: ConnectionTestResult = serde_json::from_str(&json_fail).unwrap();
        assert_eq!(deserialized_fail.ok, false);
        assert_eq!(deserialized_fail.message, "Connection refused");
    }

    #[test]
    fn test_pttype_lookup_serialization() {
        let lookup = PttypeLookup {
            pttype: "T1".to_string(),
            name: "สิทธิ UC".to_string(),
            pcode: "P001".to_string(),
            hipdata_code: "H001".to_string(),
        };
        let json = serde_json::to_string(&lookup).unwrap();
        let deserialized: PttypeLookup = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.pttype, "T1");
        assert_eq!(deserialized.name, "สิทธิ UC");
        assert_eq!(deserialized.pcode, "P001");
        assert_eq!(deserialized.hipdata_code, "H001");
    }

    #[test]
    fn test_monthly_stats_serialization() {
        let stats = MonthlyStats {
            month: "2024-01".to_string(),
            count: 42,
            total_revenue: 21000.50,
            total_payout: 12600.30,
        };
        let json = serde_json::to_string(&stats).unwrap();
        let deserialized: MonthlyStats = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.month, "2024-01");
        assert_eq!(deserialized.count, 42);
        assert_eq!(deserialized.total_revenue, 21000.50);
        assert_eq!(deserialized.total_payout, 12600.30);
    }
}
