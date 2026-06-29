-- ============================================================================
-- RTC State Machine Expansion
-- ============================================================================
-- Extends im_rtc_sessions.session_state from 4 states to 11 states, aligning
-- with industry-standard call lifecycles (Discord/Teams/Zoom/Twilio/Agora).
--
-- State map (backward compatible):
--   initiating  (new, supersedes 'started' for outbound leg before ringing)
--   ringing     (new, callee device(s) are being alerted)
--   connecting  (new, ICE/DTLS/SRTP handshake in progress)
--   connected   (new, media flowing; supersedes 'accepted')
--   on_hold     (new, media paused by either party)
--   reconnecting(new, media dropped, ICE restart in progress)
--   ended       (existing, normal termination)
--   canceled    (new, initiator canceled before callee accepted)
--   rejected    (existing, callee explicitly declined)
--   failed      (new, media/signaling failure, non-recoverable)
--   timeout     (new, ringing exceeded callee-answer deadline)
--
-- Backward compatibility:
--   'started'  remains valid, treated as alias for 'initiating'
--   'accepted' remains valid, treated as alias for 'connected'
-- This allows zero-downtime rollout: existing rows keep their state values,
-- new code writes the new values, readers normalize via RtcSessionState::from_str.
-- ============================================================================

-- 1. Expand session_state CHECK constraint
ALTER TABLE im_rtc_sessions
    DROP CONSTRAINT IF EXISTS chk_im_rtc_sessions_state;

ALTER TABLE im_rtc_sessions
    ADD CONSTRAINT chk_im_rtc_sessions_state CHECK (session_state IN (
        'started', 'accepted', 'rejected', 'ended',
        'initiating', 'ringing', 'connecting', 'connected',
        'on_hold', 'reconnecting', 'canceled', 'failed', 'timeout'
    ));

-- 2. Add lifecycle timestamp columns for SLA / quality analytics
ALTER TABLE im_rtc_sessions
    ADD COLUMN IF NOT EXISTS initiating_at   TIMESTAMPTZ,
    ADD COLUMN IF NOT EXISTS ringing_at      TIMESTAMPTZ,
    ADD COLUMN IF NOT EXISTS connecting_at   TIMESTAMPTZ,
    ADD COLUMN IF NOT EXISTS connected_at    TIMESTAMPTZ,
    ADD COLUMN IF NOT EXISTS on_hold_since   TIMESTAMPTZ,
    ADD COLUMN IF NOT EXISTS reconnecting_since TIMESTAMPTZ,
    ADD COLUMN IF NOT EXISTS canceled_at     TIMESTAMPTZ,
    ADD COLUMN IF NOT EXISTS failed_at       TIMESTAMPTZ,
    ADD COLUMN IF NOT EXISTS timeout_at      TIMESTAMPTZ,
    ADD COLUMN IF NOT EXISTS ended_reason    TEXT,
    ADD COLUMN IF NOT EXISTS failure_reason  TEXT;

-- 3. Backfill lifecycle timestamps from existing columns for historical rows
UPDATE im_rtc_sessions
    SET initiating_at = started_at
    WHERE initiating_at IS NULL AND started_at IS NOT NULL;

UPDATE im_rtc_sessions
    SET connected_at = COALESCE(connected_at, updated_at)
    WHERE connected_at IS NULL AND session_state IN ('accepted', 'connected', 'ended');

UPDATE im_rtc_sessions
    SET ended_reason = CASE
        WHEN session_state = 'rejected' THEN 'rejected'
        WHEN session_state = 'ended'    THEN 'normal'
        ELSE NULL
    END
    WHERE ended_reason IS NULL AND session_state IN ('rejected', 'ended');

-- 4. Add CHECK constraint ensuring ended_reason is present for terminal states
ALTER TABLE im_rtc_sessions
    DROP CONSTRAINT IF EXISTS chk_im_rtc_sessions_terminal_reason;

ALTER TABLE im_rtc_sessions
    ADD CONSTRAINT chk_im_rtc_sessions_terminal_reason CHECK (
        session_state NOT IN ('ended', 'canceled', 'rejected', 'failed', 'timeout')
        OR ended_reason IS NOT NULL
    );

-- 5. Expand im_rtc_signals.signal_type CHECK to cover new signaling types
ALTER TABLE im_rtc_signals
    DROP CONSTRAINT IF EXISTS chk_im_rtc_signals_signal_type;

ALTER TABLE im_rtc_signals
    ADD CONSTRAINT chk_im_rtc_signals_signal_type CHECK (signal_type IN (
        'offer', 'answer', 'ice_candidate', 'renegotiate',
        'add_participant', 'remove_participant', 'kick_participant',
        'mute', 'unmute', 'screen_share_start', 'screen_share_stop',
        'hold', 'resume', 'reconnect', 'quality_report',
        'recording_start', 'recording_stop', 'recording_status'
    ));

-- 6. Add client_signal_id column for signal idempotency (dedup on retry)
ALTER TABLE im_rtc_signals
    ADD COLUMN IF NOT EXISTS client_signal_id TEXT;

CREATE UNIQUE INDEX IF NOT EXISTS uk_im_rtc_signals_client_signal_id
    ON im_rtc_signals (tenant_id, organization_id, rtc_session_id, sender_principal_kind, sender_principal_id, client_signal_id)
    WHERE client_signal_id IS NOT NULL;

-- 7. Indexes for the new lifecycle timestamps (SLA dashboards, cleanup jobs)
CREATE INDEX IF NOT EXISTS idx_im_rtc_sessions_ringing_at
    ON im_rtc_sessions (tenant_id, organization_id, ringing_at)
    WHERE ringing_at IS NOT NULL AND session_state = 'ringing';

CREATE INDEX IF NOT EXISTS idx_im_rtc_sessions_active_lifecycle
    ON im_rtc_sessions (tenant_id, organization_id, session_state, updated_at)
    WHERE session_state IN (
        'initiating', 'ringing', 'connecting', 'connected',
        'on_hold', 'reconnecting', 'started', 'accepted'
    );

-- 8. Index for cleanup jobs: stale sessions older than threshold
CREATE INDEX IF NOT EXISTS idx_im_rtc_sessions_stale_cleanup
    ON im_rtc_sessions (tenant_id, organization_id, updated_at, rtc_session_id)
    WHERE session_state NOT IN ('ended', 'canceled', 'rejected', 'failed', 'timeout');

-- 9. Comment for documentation
COMMENT ON TABLE im_rtc_sessions IS
    'RTC call session lifecycle. State machine: initiating -> ringing -> connecting -> connected -> (on_hold|reconnecting)* -> ended|canceled|rejected|failed|timeout. Legacy states started/accepted retained as aliases.';
COMMENT ON COLUMN im_rtc_sessions.ended_reason IS
    'Required for terminal states. Values: normal|rejected|canceled|timeout|failed|media_drop|signaling_error|participant_left';
