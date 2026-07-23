BEGIN;

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
