BEGIN;

ALTER TABLE im_stream_sessions
    ADD COLUMN version INTEGER NOT NULL DEFAULT 1 CHECK (version > 0);

CREATE INDEX IF NOT EXISTS idx_im_stream_sessions_active
    ON im_stream_sessions (tenant_id, organization_id)
    WHERE stream_state NOT IN ('completed', 'aborted', 'expired');

COMMIT;
