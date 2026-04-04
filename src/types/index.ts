// types/index.ts — TypeScript type definitions mirroring Rust models

export interface HosxpSettings {
  host: string
  port: number
  database_name: string
  username: string
  password: string
}

export interface ConnectionTestResult {
  ok: boolean
  message: string
}

export interface PttypeConfig {
  id: number
  pttype: string
  name: string
  pcode: string
  hipdata_code: string
  short_name: string
}

export interface ProcedureConfig {
  id: number
  icode: string
  name: string
  short_name: string
}

export interface DrugConfig {
  id: number
  icode: string
  name: string
  short_name: string
}

export interface ProviderConfig {
  id: number
  health_med_provider_id: number
  full_name: string
  short_name: string
}

export interface PayoutOption {
  id: number
  amount: number
  label: string
}

export interface PatientRow {
  vn: string
  cid: string
  hn: string
  first_name: string
  last_name: string
  gender: string
  age: number | null
  pttype_display_name: string
  pttype_hipdata_code: string
  chief_complaint: string
  item_names: string
  all_icodes: string
  provider_names: string
  total_sum_price: number
  service_date: string
  // UI-only fields (not from Rust)
  _locked?: boolean
  _multi_provider?: boolean
  _selected_payout?: number | null
}

export interface PendingRow {
  id: number
  visit_date: string
  vn: string
  hn: string
  cid: string
  first_name: string
  last_name: string
  gender: string
  age: number | null
  rights: string
  symptoms: string
  procedure: string
  therapist: string
  total_revenue: number
  payout_amount: number
  created_at: string
}

export interface UpsertPendingInput {
  visit_date: string
  vn: string
  hn: string
  cid: string
  first_name: string
  last_name: string
  gender: string
  age: number | null
  rights: string
  symptoms: string
  procedure: string
  therapist: string
  total_revenue: number
  payout_amount: number
}

export interface MonthlyStats {
  month: string
  count: number
  total_revenue: number
  total_payout: number
}

export interface PttypeLookup {
  pttype: string
  name: string
  pcode: string
  hipdata_code: string
}

export interface ItemLookup {
  icode: string
  name: string
}

export interface ProviderLookup {
  health_med_provider_id: number
  health_med_provider_full_name: string
}
