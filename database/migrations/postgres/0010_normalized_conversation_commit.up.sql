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
-- idempotency: the contract baseline already carries the normalized coordinates; every DDL is
--   guarded so fresh installs bootstrap as a no-op while legacy pre-squash databases still transform
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
    ALTER COLUMN conversation_type SET NOT NULL;

DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_attribute
        WHERE attrelid = 'im_conversations'::regclass AND attname = 'lifecycle_state' AND NOT attisdropped
    ) THEN
        ALTER TABLE im_conversations
            ADD COLUMN lifecycle_state TEXT NOT NULL DEFAULT 'active';
    END IF;

    IF NOT EXISTS (
        SELECT 1 FROM pg_attribute
        WHERE attrelid = 'im_conversations'::regclass AND attname = 'commit_seq' AND NOT attisdropped
    ) THEN
        ALTER TABLE im_conversations
            ADD COLUMN commit_seq BIGINT NOT NULL DEFAULT 0 CHECK (commit_seq >= 0);
    END IF;

    IF NOT EXISTS (
        SELECT 1 FROM pg_attribute
        WHERE attrelid = 'im_conversations'::regclass AND attname = 'member_epoch' AND NOT attisdropped
    ) THEN
        ALTER TABLE im_conversations
            ADD COLUMN member_epoch BIGINT NOT NULL DEFAULT 0 CHECK (member_epoch >= 0);
    END IF;

    IF EXISTS (
        SELECT 1 FROM pg_attribute
        WHERE attrelid = 'im_conversations'::regclass AND attname = 'agent_handoff_json' AND NOT attisdropped
    ) THEN
        ALTER TABLE im_conversations DROP COLUMN agent_handoff_json;
    END IF;
END;
$$;

DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_constraint
        WHERE conname = 'chk_im_conversations_lifecycle'
          AND conrelid = 'im_conversations'::regclass
    ) THEN
        ALTER TABLE im_conversations
            ADD CONSTRAINT chk_im_conversations_lifecycle
            CHECK (lifecycle_state IN ('active', 'archived'));
    END IF;
END;
$$;

COMMIT;
