// composables/useCommands.ts — Typed wrappers around Tauri invoke calls

import { invoke } from '@tauri-apps/api/core'
import type {
  HosxpSettings,
  ConnectionTestResult,
  PttypeConfig,
  ProcedureConfig,
  DrugConfig,
  ProviderConfig,
  PayoutOption,
  PatientRow,
  PendingRow,
  UpsertPendingInput,
  MonthlyStats,
  PttypeLookup,
  ItemLookup,
  ProviderLookup,
} from '@/types'

// ── Connection ────────────────────────────────────────────────────────────
export const getHosxpSettings = () =>
  invoke<HosxpSettings>('get_hosxp_settings')

export const saveHosxpSettings = (settings: HosxpSettings) =>
  invoke<void>('save_hosxp_settings', { settings })

export const testHosxpConnection = (settings: HosxpSettings) =>
  invoke<ConnectionTestResult>('test_hosxp_connection', { settings })

export const connectHosxp = (settings: HosxpSettings) =>
  invoke<ConnectionTestResult>('connect_hosxp', { settings })

export const disconnectHosxp = () =>
  invoke<void>('disconnect_hosxp')

export const isHosxpConnected = () =>
  invoke<boolean>('is_hosxp_connected')

export const autoConnect = () =>
  invoke<ConnectionTestResult>('auto_connect')

// ── HOSxP Lookups (Settings) ──────────────────────────────────────────────
export const lookupPttype = (hipdataCode: string) =>
  invoke<PttypeLookup | null>('lookup_pttype', { hipdataCode })

export const lookupProcedure = (icode: string) =>
  invoke<ItemLookup | null>('lookup_procedure', { icode })

export const lookupDrug = (icode: string) =>
  invoke<ItemLookup | null>('lookup_drug', { icode })

export const lookupProvider = (providerId: number) =>
  invoke<ProviderLookup | null>('lookup_provider', { providerId })

// ── Pttypes ───────────────────────────────────────────────────────────────
export const getAllPttypes = () =>
  invoke<PttypeConfig[]>('get_all_pttypes')

export const savePttype = (
  pttype: string, name: string, pcode: string,
  hipdataCode: string, shortName: string
) =>
  invoke<void>('save_pttype', { pttype, name, pcode, hipdataCode, shortName })

export const deletePttype = (id: number) =>
  invoke<void>('delete_pttype', { id })

// ── Procedures ────────────────────────────────────────────────────────────
export const getAllProcedures = () =>
  invoke<ProcedureConfig[]>('get_all_procedures')

export const saveProcedure = (icode: string, name: string, shortName: string) =>
  invoke<void>('save_procedure', { icode, name, shortName })

export const deleteProcedure = (id: number) =>
  invoke<void>('delete_procedure', { id })

// ── Drugs ─────────────────────────────────────────────────────────────────
export const getAllDrugs = () =>
  invoke<DrugConfig[]>('get_all_drugs')

export const saveDrug = (icode: string, name: string, shortName: string) =>
  invoke<void>('save_drug', { icode, name, shortName })

export const deleteDrug = (id: number) =>
  invoke<void>('delete_drug', { id })

// ── Providers ─────────────────────────────────────────────────────────────
export const getAllProviders = () =>
  invoke<ProviderConfig[]>('get_all_providers')

export const saveProvider = (healthMedProviderId: number, fullName: string, shortName: string) =>
  invoke<void>('save_provider', { healthMedProviderId, fullName, shortName })

export const deleteProvider = (id: number) =>
  invoke<void>('delete_provider', { id })

// ── Payout Options ────────────────────────────────────────────────────────
export const getPayoutOptions = () =>
  invoke<PayoutOption[]>('get_payout_options')

export const addPayoutOption = (amount: number, label: string) =>
  invoke<void>('add_payout_option', { amount, label })

export const deletePayoutOption = (id: number) =>
  invoke<void>('delete_payout_option', { id })

// ── Patient Data ──────────────────────────────────────────────────────────
export const fetchPatientData = (date: string) =>
  invoke<PatientRow[]>('fetch_patient_data', { date })

export const getLockedVns = (date: string) =>
  invoke<string[]>('get_locked_vns', { date })

// ── Pending Export ────────────────────────────────────────────────────────
export const upsertPending = (record: UpsertPendingInput) =>
  invoke<void>('upsert_pending', { record })

export const getPendingExport = (dateFrom: string, dateTo: string) =>
  invoke<PendingRow[]>('get_pending_export', { dateFrom, dateTo })

export const deletePendingById = (id: number) =>
  invoke<void>('delete_pending_by_id', { id })

export const exportPendingCsv = (dateFrom: string, dateTo: string, filePath: string) =>
  invoke<string>('export_pending_csv', { dateFrom, dateTo, filePath })

// ── Delete Data ───────────────────────────────────────────────────────────
export const previewDeleteRange = (dateFrom: string, dateTo: string) =>
  invoke<PendingRow[]>('preview_delete_range', { dateFrom, dateTo })

export const deletePendingRange = (dateFrom: string, dateTo: string) =>
  invoke<number>('delete_pending_range', { dateFrom, dateTo })

export const getMonthlyStats = () =>
  invoke<MonthlyStats[]>('get_monthly_stats')
