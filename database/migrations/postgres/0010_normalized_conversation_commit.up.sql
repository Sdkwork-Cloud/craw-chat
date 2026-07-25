-- sdkwork:migration
-- id: 0010_normalized_conversation_commit
-- engine: postgres
-- module: im
-- purpose: Make normalized Conversation lifecycle and commit coordinates authoritative
-- reversible: false
-- rollback: forward-fix
-- transactional: true
-- lock: access-exclusive
-- lock_timeout: 5s
-- statement_timeout: 2m
-- rewrite: PostgreSQL-version-dependent fast defaults; no legacy data is accepted
-- backfill: forbidden; pre-launch databases with Conversation rows must reset before cutover
-- write_traffic: Conversation writes must be stopped during cutover
-- replication_wal: bounded to DDL because the preflight requires an empty Conversation table
-- observability: monitor preflight failure, lock wait, WAL growth, and replica lag
-- cancellation: cancel before commit; PostgreSQL rolls back the complete transaction
-- recovery: correct through a forward migration; never recreate agent_handoff_json as authority
-- contract_version: 2.1.0

BEGIN;

SET LOCAL lock_timeout = '5s';
SET LOCAL statement_timeout = '2min';

DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM im_conversations LIMIT 1) THEN
        RAISE EXCEPTION
            'normalized Conversation cutover requires a pre-launch database reset; legacy current state cannot be synthesized from the commit journal';
    END IF;
END;
$$;

ALTER TABLE im_conversations
    ALTER COLUMN conversation_type SET NOT NULL,
    ADD COLUMN lifecycle_state TEXT NOT NULL DEFAULT 'active',
    ADD COLUMN commit_seq BIGINT NOT NULL DEFAULT 0 CHECK (commit_seq >= 0),
    ADD COLUMN member_epoch BIGINT NOT NULL DEFAULT 0 CHECK (member_epoch >= 0),
    DROP COLUMN agent_handoff_json;

ALTER TABLE im_conversations
    ADD CONSTRAINT chk_im_conversations_lifecycle
    CHECK (lifecycle_state IN ('active', 'archived'));

COMMIT;
