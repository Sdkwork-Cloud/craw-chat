-- ============================================================================
-- RTC State Machine Expansion (SQLite)
-- ============================================================================
-- SQLite adaptation of postgres migration 0008.
-- SQLite does not support ALTER TABLE ADD/DROP CONSTRAINT, so CHECK
-- constraints are NOT modified here; they remain as the original 4-state
-- CHECK from baseline. Application-layer validation (RtcSessionState::from_str)
-- enforces the full 11-state machine. The columns are added so the schema
-- matches postgres for cross-engine compatibility.
-- ============================================================================

-- 1. Add lifecycle timestamp columns
ALTER TABLE im_rtc_sessions ADD COLUMN initiating_at      TEXT;
ALTER TABLE im_rtc_sessions ADD COLUMN ringing_at         TEXT;
ALTER TABLE im_rtc_sessions ADD COLUMN connecting_at      TEXT;
ALTER TABLE im_rtc_sessions ADD COLUMN connected_at       TEXT;
ALTER TABLE im_rtc_sessions ADD COLUMN on_hold_since      TEXT;
ALTER TABLE im_rtc_sessions ADD COLUMN reconnecting_since TEXT;
ALTER TABLE im_rtc_sessions ADD COLUMN canceled_at        TEXT;
ALTER TABLE im_rtc_sessions ADD COLUMN failed_at          TEXT;
ALTER TABLE im_rtc_sessions ADD COLUMN timeout_at         TEXT;
ALTER TABLE im_rtc_sessions ADD COLUMN ended_reason       TEXT;
ALTER TABLE im_rtc_sessions ADD COLUMN failure_reason     TEXT;

-- 2. Add client_signal_id to im_rtc_signals for idempotency
ALTER TABLE im_rtc_signals ADD COLUMN client_signal_id TEXT;

CREATE UNIQUE INDEX IF NOT EXISTS uk_im_rtc_signals_client_signal_id
    ON im_rtc_signals (tenant_id, organization_id, rtc_session_id, sender_principal_kind, sender_principal_id, client_signal_id)
    WHERE client_signal_id IS NOT NULL;

-- 3. Indexes for lifecycle queries
CREATE INDEX IF NOT EXISTS idx_im_rtc_sessions_ringing_at
    ON im_rtc_sessions (tenant_id, organization_id, ringing_at)
    WHERE ringing_at IS NOT NULL AND session_state = 'ringing';

CREATE INDEX IF NOT EXISTS idx_im_rtc_sessions_active_lifecycle
    ON im_rtc_sessions (tenant_id, organization_id, session_state, updated_at)
    WHERE session_state IN (
        'initiating', 'ringing', 'connecting', 'connected',
        'on_hold', 'reconnecting', 'started', 'accepted'
    );

CREATE INDEX IF NOT EXISTS idx_im_rtc_sessions_stale_cleanup
    ON im_rtc_sessions (tenant_id, organization_id, updated_at, rtc_session_id)
    WHERE session_state NOT IN ('ended', 'canceled', 'rejected', 'failed', 'timeout');
