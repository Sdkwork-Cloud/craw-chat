-- Rollback: Remove indexes created in migration 0007.

DROP INDEX IF EXISTS idx_messages_window_live;
DROP INDEX IF EXISTS idx_messages_client_id;
DROP INDEX IF EXISTS idx_messages_tenant_id;
DROP INDEX IF EXISTS idx_messages_watermark;
DROP INDEX IF EXISTS idx_rtc_sessions_tenant;
DROP INDEX IF EXISTS idx_rtc_sessions_state_activity;
DROP INDEX IF EXISTS idx_rtc_signals_session_seq;
DROP INDEX IF EXISTS idx_realtime_events_device_seq;
DROP INDEX IF EXISTS idx_commit_journal_aggregate;
DROP INDEX IF EXISTS idx_commit_journal_type;
DROP INDEX IF EXISTS idx_idempotency_expiry;
DROP INDEX IF EXISTS idx_audit_records_tenant_occurred;
DROP INDEX IF EXISTS idx_audit_records_target;
DROP INDEX IF EXISTS idx_audit_records_actor;
DROP INDEX IF EXISTS idx_projection_conversation_members_principal;
