BEGIN;

ALTER TABLE im_projection_conversation_agent
    ADD CONSTRAINT ck_im_projection_conversation_agent_scope CHECK (
        tenant_id > 0 AND organization_id >= 0 AND assigned_by > 0
    ) NOT VALID;
ALTER TABLE im_projection_conversation_agent
    VALIDATE CONSTRAINT ck_im_projection_conversation_agent_scope;

ALTER TABLE im_conversation_agent_binding
    ADD CONSTRAINT ck_im_conversation_agent_binding_scope CHECK (
        tenant_id > 0 AND organization_id >= 0
        AND created_by > 0 AND updated_by > 0
    ) NOT VALID;
ALTER TABLE im_conversation_agent_binding
    VALIDATE CONSTRAINT ck_im_conversation_agent_binding_scope;

ALTER TABLE im_agent_dispatch
    ADD CONSTRAINT ck_im_agent_dispatch_scope CHECK (
        tenant_id > 0 AND organization_id >= 0 AND requested_by > 0
    ) NOT VALID;
ALTER TABLE im_agent_dispatch
    ADD CONSTRAINT ck_im_agent_dispatch_message_ids CHECK (
        source_message_id > 0 AND (reply_message_id IS NULL OR reply_message_id > 0)
    ) NOT VALID;
ALTER TABLE im_agent_dispatch
    VALIDATE CONSTRAINT ck_im_agent_dispatch_scope;
ALTER TABLE im_agent_dispatch
    VALIDATE CONSTRAINT ck_im_agent_dispatch_message_ids;

COMMIT;
