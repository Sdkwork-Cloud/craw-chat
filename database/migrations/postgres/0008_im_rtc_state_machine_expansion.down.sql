-- Rollback: RTC state machine expansion
-- WARNING: This drops new columns and constraints; data in them will be lost.

DROP INDEX IF EXISTS idx_im_rtc_sessions_stale_cleanup;
DROP INDEX IF EXISTS idx_im_rtc_sessions_active_lifecycle;
DROP INDEX IF EXISTS idx_im_rtc_sessions_ringing_at;
DROP INDEX IF EXISTS uk_im_rtc_signals_client_signal_id;

ALTER TABLE im_rtc_signals
    DROP COLUMN IF EXISTS client_signal_id;

ALTER TABLE im_rtc_signals
    DROP CONSTRAINT IF EXISTS chk_im_rtc_signals_signal_type;

ALTER TABLE im_rtc_sessions
    DROP CONSTRAINT IF EXISTS chk_im_rtc_sessions_terminal_reason;

ALTER TABLE im_rtc_sessions
    DROP COLUMN IF EXISTS initiating_at,
    DROP COLUMN IF EXISTS ringing_at,
    DROP COLUMN IF EXISTS connecting_at,
    DROP COLUMN IF EXISTS connected_at,
    DROP COLUMN IF EXISTS on_hold_since,
    DROP COLUMN IF EXISTS reconnecting_since,
    DROP COLUMN IF EXISTS canceled_at,
    DROP COLUMN IF EXISTS failed_at,
    DROP COLUMN IF EXISTS timeout_at,
    DROP COLUMN IF EXISTS ended_reason,
    DROP COLUMN IF EXISTS failure_reason;

ALTER TABLE im_rtc_sessions
    DROP CONSTRAINT IF EXISTS chk_im_rtc_sessions_state;

ALTER TABLE im_rtc_sessions
    ADD CONSTRAINT chk_im_rtc_sessions_state CHECK (session_state IN ('started', 'accepted', 'rejected', 'ended'));
