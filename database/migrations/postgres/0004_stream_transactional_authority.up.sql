-- sdkwork:migration
-- id: 0004_stream_transactional_authority
-- engine: postgres
-- module: im
-- purpose: Add optimistic versioning and an active-session index to stream authority
-- reversible: false
-- rollback: forward-fix
-- transactional: true
-- lock: access-exclusive
-- lock_timeout: 5s
-- statement_timeout: 2m
-- rewrite: PostgreSQL-version-dependent fast default; validate on supported versions
-- backfill: existing rows receive version 1 in the schema transaction
-- write_traffic: stream-session writes may block while table metadata changes
-- replication_wal: bounded by the version backfill and index build
-- observability: monitor lock wait, index build duration, WAL growth, and replica lag
-- cancellation: cancel before commit; PostgreSQL rolls back the complete transaction
-- recovery: correct the failed statement and ship a forward migration
-- contract_version: 1.3.0

BEGIN;

SET LOCAL lock_timeout = '5s';
SET LOCAL statement_timeout = '2min';

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
