BEGIN;

DO $$
BEGIN
    IF EXISTS (
        SELECT 1
        FROM im_conversation_agent_assignments
        WHERE assigned_by = 0
    ) THEN
        RAISE EXCEPTION
            'rollback refused: system actor assignment projections exist';
    END IF;
END;
$$;

ALTER TABLE im_conversation_agent_assignments
    DROP CONSTRAINT ck_im_conversation_agent_assignments_scope;
ALTER TABLE im_conversation_agent_assignments
    ADD CONSTRAINT ck_im_conversation_agent_assignments_scope CHECK (
        tenant_id > 0 AND organization_id >= 0 AND assigned_by > 0
    ) NOT VALID;
ALTER TABLE im_conversation_agent_assignments
    VALIDATE CONSTRAINT ck_im_conversation_agent_assignments_scope;

COMMIT;
