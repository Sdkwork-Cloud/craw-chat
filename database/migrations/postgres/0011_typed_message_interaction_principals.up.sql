BEGIN;

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
