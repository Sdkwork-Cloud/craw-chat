BEGIN;

DO $$
BEGIN
    IF EXISTS (
        SELECT 1
        FROM im_projection_conversation_agent
        WHERE assigned_by = 0
    ) THEN
        RAISE EXCEPTION
            'rollback refused: system actor assignment projections exist';
    END IF;
END;
$$;

ALTER TABLE im_projection_conversation_agent
    DROP CONSTRAINT ck_im_projection_conversation_agent_scope;
ALTER TABLE im_projection_conversation_agent
    ADD CONSTRAINT ck_im_projection_conversation_agent_scope CHECK (
        tenant_id > 0 AND organization_id >= 0 AND assigned_by > 0
    ) NOT VALID;
ALTER TABLE im_projection_conversation_agent
    VALIDATE CONSTRAINT ck_im_projection_conversation_agent_scope;

COMMIT;
