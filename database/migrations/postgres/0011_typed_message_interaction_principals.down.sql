BEGIN;

DROP INDEX idx_im_message_reactions_actor;

ALTER TABLE im_message_reactions
    DROP CONSTRAINT chk_im_message_reactions_type,
    DROP CONSTRAINT chk_im_message_reactions_actor_id,
    DROP CONSTRAINT chk_im_message_reactions_actor_kind,
    DROP CONSTRAINT fk_im_message_reactions_message,
    DROP CONSTRAINT pk_im_message_reactions,
    DROP COLUMN actor_principal_kind;

ALTER TABLE im_message_reactions
    RENAME COLUMN actor_principal_id TO user_id;

ALTER TABLE im_message_reactions
    ADD CONSTRAINT pk_im_message_reactions PRIMARY KEY (
        tenant_id, organization_id, conversation_id, message_id, user_id, reaction_type
    );

CREATE INDEX idx_im_message_reactions_user
    ON im_message_reactions (tenant_id, organization_id, user_id, created_at DESC);

DROP INDEX idx_im_message_pins_actor;

ALTER TABLE im_message_pins
    DROP CONSTRAINT chk_im_message_pins_actor_id,
    DROP CONSTRAINT chk_im_message_pins_actor_kind,
    DROP CONSTRAINT fk_im_message_pins_message,
    DROP COLUMN pinned_by_principal_kind;

ALTER TABLE im_message_pins
    RENAME COLUMN pinned_by_principal_id TO pinned_by_user_id;

CREATE INDEX idx_im_message_pins_user
    ON im_message_pins (tenant_id, organization_id, pinned_by_user_id, pinned_at DESC);

COMMIT;
