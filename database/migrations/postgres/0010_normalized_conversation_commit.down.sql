BEGIN;

ALTER TABLE im_conversations
    DROP CONSTRAINT chk_im_conversations_lifecycle,
    DROP COLUMN member_epoch,
    DROP COLUMN commit_seq,
    DROP COLUMN lifecycle_state,
    ADD COLUMN agent_handoff_json JSONB,
    ALTER COLUMN conversation_type DROP NOT NULL;

COMMIT;
