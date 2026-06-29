-- ============================================================================
-- Database Index Optimization Migration
-- ============================================================================
--
-- Adds query-path indexes that are not already created by the baseline DDL.

-- ============================================================================
-- 1. Message Store Queries (message_store.rs)
-- ============================================================================

CREATE INDEX IF NOT EXISTS idx_messages_window_live
    ON im_conversation_messages (tenant_id, organization_id, conversation_id, message_seq)
    WHERE deleted_at IS NULL AND retention_until IS NULL;

CREATE INDEX IF NOT EXISTS idx_messages_client_id
    ON im_conversation_messages (tenant_id, organization_id, conversation_id, sender_principal_kind, sender_principal_id, client_msg_id);

CREATE INDEX IF NOT EXISTS idx_messages_tenant_id
    ON im_conversation_messages (tenant_id, message_id);

CREATE INDEX IF NOT EXISTS idx_messages_watermark
    ON im_conversation_messages (tenant_id, organization_id, conversation_id, message_seq DESC);

-- ============================================================================
-- 2. RTC Session Store (im_rtc_sessions)
-- ============================================================================

CREATE INDEX IF NOT EXISTS idx_rtc_sessions_tenant
    ON im_rtc_sessions (tenant_id, organization_id, rtc_session_id, session_state);

CREATE INDEX IF NOT EXISTS idx_rtc_sessions_state_activity
    ON im_rtc_sessions (session_state, updated_at)
    WHERE session_state IN ('started', 'accepted');

-- ============================================================================
-- 3. RTC Signal Store (im_rtc_signals)
-- ============================================================================

CREATE INDEX IF NOT EXISTS idx_rtc_signals_session_seq
    ON im_rtc_signals (tenant_id, organization_id, rtc_session_id, signal_seq);

-- ============================================================================
-- 4. Realtime Device Events (im_realtime_device_events)
-- ============================================================================

CREATE INDEX IF NOT EXISTS idx_realtime_events_device_seq
    ON im_realtime_device_events (tenant_id, principal_id, device_id, realtime_seq);

-- ============================================================================
-- 5. Commit Journal (im_commit_journal)
-- ============================================================================

CREATE INDEX IF NOT EXISTS idx_commit_journal_aggregate
    ON im_commit_journal (tenant_id, aggregate_type, aggregate_id, commit_offset);

CREATE INDEX IF NOT EXISTS idx_commit_journal_type
    ON im_commit_journal (tenant_id, organization_id, aggregate_type, occurred_at);

-- ============================================================================
-- 6. Idempotency Keys (im_idempotency_keys)
-- ============================================================================

CREATE INDEX IF NOT EXISTS idx_idempotency_expiry
    ON im_idempotency_keys (tenant_id, organization_id, expires_at);

-- ============================================================================
-- 7. Audit Records (im_audit_records)
-- ============================================================================

CREATE INDEX IF NOT EXISTS idx_audit_records_tenant_occurred
    ON im_audit_records (tenant_id, organization_id, occurred_at DESC);

CREATE INDEX IF NOT EXISTS idx_audit_records_target
    ON im_audit_records (target_type, target_id, occurred_at DESC);

CREATE INDEX IF NOT EXISTS idx_audit_records_actor
    ON im_audit_records (actor_id, actor_kind, occurred_at DESC);

-- ============================================================================
-- 8. Conversation Membership (im_projection_conversation_members)
-- ============================================================================

CREATE INDEX IF NOT EXISTS idx_projection_conversation_members_principal
    ON im_projection_conversation_members (tenant_id, organization_id, principal_kind, principal_id);
