BEGIN;

ALTER TABLE im_conversation_agent_assignments
    DROP CONSTRAINT ck_im_conversation_agent_assignments_generation;
ALTER TABLE im_conversation_agent_assignments
    ADD CONSTRAINT ck_im_conversation_agent_assignments_generation CHECK (
        assignment_generation > 0 AND source_aggregate_version >= 0
    ) NOT VALID;
ALTER TABLE im_conversation_agent_assignments
    VALIDATE CONSTRAINT ck_im_conversation_agent_assignments_generation;

COMMIT;
