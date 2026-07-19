BEGIN;

ALTER TABLE im_agent_dispatch
    DROP CONSTRAINT IF EXISTS ck_im_agent_dispatch_message_ids;
ALTER TABLE im_agent_dispatch
    DROP CONSTRAINT IF EXISTS ck_im_agent_dispatch_scope;
ALTER TABLE im_conversation_agent_binding
    DROP CONSTRAINT IF EXISTS ck_im_conversation_agent_binding_scope;
ALTER TABLE im_projection_conversation_agent
    DROP CONSTRAINT IF EXISTS ck_im_projection_conversation_agent_scope;

COMMIT;
