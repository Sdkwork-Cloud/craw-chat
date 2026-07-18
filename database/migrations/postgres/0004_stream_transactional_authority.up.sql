BEGIN;

ALTER TABLE im_stream_sessions
    ADD COLUMN IF NOT EXISTS version BIGINT NOT NULL DEFAULT 1;

ALTER TABLE im_stream_sessions
    DROP CONSTRAINT IF EXISTS chk_im_stream_sessions_version;
ALTER TABLE im_stream_sessions
    ADD CONSTRAINT chk_im_stream_sessions_version CHECK (version > 0) NOT VALID;
ALTER TABLE im_stream_sessions
    VALIDATE CONSTRAINT chk_im_stream_sessions_version;

CREATE INDEX IF NOT EXISTS idx_im_stream_sessions_active
    ON im_stream_sessions (tenant_id, organization_id)
    WHERE stream_state NOT IN ('completed', 'aborted', 'expired');

COMMIT;
