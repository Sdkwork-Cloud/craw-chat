-- sdkwork:migration
-- id: 0011_typed_message_interaction_principals
-- engine: postgres
-- module: im
-- purpose: Replace user-only reaction and pin principals with typed IM principals
-- reversible: false
-- rollback: forward-fix
-- transactional: true
-- lock: access-exclusive
-- lock_timeout: 5s
-- statement_timeout: 5m
-- rewrite: existing user principals are explicitly typed as user
-- backfill: bounded in-transaction updates over reaction and pin rows
-- write_traffic: reaction and pin writes must be stopped during cutover
-- replication_wal: proportional to existing reaction and pin rows plus replacement indexes
-- observability: monitor updated row counts, index builds, lock wait, WAL growth, and replica lag
-- cancellation: cancel before commit; PostgreSQL rolls back the complete transaction
-- recovery: correct through a forward migration; principal kinds must never be discarded
-- contract_version: 2.1.0

BEGIN;

SET LOCAL lock_timeout = '5s';
SET LOCAL statement_timeout = '5min';

ALTER TABLE im_message_reactions
    DROP CONSTRAINT pk_im_message_reactions;

ALTER TABLE im_message_reactions
    RENAME COLUMN user_id TO actor_principal_id;

ALTER TABLE im_message_reactions
    ADD COLUMN actor_principal_kind TEXT;

UPDATE im_message_reactions
SET actor_principal_kind = 'user';

ALTER TABLE im_message_reactions
    ALTER COLUMN actor_principal_kind SET NOT NULL,
    ADD CONSTRAINT pk_im_message_reactions PRIMARY KEY (
        tenant_id, organization_id, conversation_id, message_id,
        actor_principal_kind, actor_principal_id, reaction_type
    ),
    ADD CONSTRAINT fk_im_message_reactions_message FOREIGN KEY (tenant_id, message_id)
        REFERENCES im_conversation_messages (tenant_id, message_id) ON DELETE CASCADE,
    ADD CONSTRAINT chk_im_message_reactions_actor_kind
        CHECK (btrim(actor_principal_kind) <> ''),
    ADD CONSTRAINT chk_im_message_reactions_actor_id
        CHECK (btrim(actor_principal_id) <> ''),
    ADD CONSTRAINT chk_im_message_reactions_type
        CHECK (btrim(reaction_type) <> '');

DROP INDEX idx_im_message_reactions_user;

CREATE INDEX idx_im_message_reactions_actor
    ON im_message_reactions (
        tenant_id, organization_id, actor_principal_kind, actor_principal_id, created_at DESC
    );

ALTER TABLE im_message_pins
    RENAME COLUMN pinned_by_user_id TO pinned_by_principal_id;

ALTER TABLE im_message_pins
    ADD COLUMN pinned_by_principal_kind TEXT;

UPDATE im_message_pins
SET pinned_by_principal_kind = 'user';

ALTER TABLE im_message_pins
    ALTER COLUMN pinned_by_principal_kind SET NOT NULL,
    ADD CONSTRAINT fk_im_message_pins_message FOREIGN KEY (tenant_id, message_id)
        REFERENCES im_conversation_messages (tenant_id, message_id) ON DELETE CASCADE,
    ADD CONSTRAINT chk_im_message_pins_actor_kind
        CHECK (btrim(pinned_by_principal_kind) <> ''),
    ADD CONSTRAINT chk_im_message_pins_actor_id
        CHECK (btrim(pinned_by_principal_id) <> '');

DROP INDEX idx_im_message_pins_user;

CREATE INDEX idx_im_message_pins_actor
    ON im_message_pins (
        tenant_id, organization_id, pinned_by_principal_kind, pinned_by_principal_id, pinned_at DESC
    );

COMMIT;
