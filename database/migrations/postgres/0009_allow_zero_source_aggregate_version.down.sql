BEGIN;

DO $$
BEGIN
    IF EXISTS (
        SELECT 1
        FROM im_projection_conversation_agent
        WHERE source_aggregate_version = 0
    ) THEN
        RAISE EXCEPTION
            'rollback refused: zero-based source aggregate projections exist';
    END IF;
END;
$$;

ALTER TABLE im_projection_conversation_agent
    DROP CONSTRAINT ck_im_projection_conversation_agent_generation;
ALTER TABLE im_projection_conversation_agent
    ADD CONSTRAINT ck_im_projection_conversation_agent_generation CHECK (
        assignment_generation > 0 AND source_aggregate_version > 0
    ) NOT VALID;
ALTER TABLE im_projection_conversation_agent
    VALIDATE CONSTRAINT ck_im_projection_conversation_agent_generation;

COMMIT;
