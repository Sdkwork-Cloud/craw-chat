-- sdkwork:migration
-- id: 0006_agents_integration_subject_guard
-- engine: postgres
-- module: im
-- purpose: Enforce tenant, organization, actor, and message identifier guards
-- reversible: true
-- rollback: down-migration
-- transactional: true
-- lock: access-exclusive
-- lock_timeout: 5s
-- statement_timeout: 5m
-- rewrite: none; NOT VALID constraints are validated explicitly
-- backfill: none; validation fails closed on invalid existing rows
-- write_traffic: writes may block briefly while constraints are attached and validated
-- replication_wal: metadata-only apart from validation visibility-map activity
-- observability: monitor validation scans, lock wait, and replica lag
-- cancellation: cancel before commit; PostgreSQL rolls back the complete transaction
-- recovery: remove invalid rows through an approved repair, then rerun; down removes only guards
-- contract_version: 2.0.0

BEGIN;

SET LOCAL lock_timeout = '5s';
SET LOCAL statement_timeout = '5min';

ALTER TABLE im_conversation_agent_assignments
    ADD CONSTRAINT ck_im_conversation_agent_assignments_scope CHECK (
        tenant_id > 0 AND organization_id >= 0 AND assigned_by > 0
    ) NOT VALID;
ALTER TABLE im_conversation_agent_assignments
    VALIDATE CONSTRAINT ck_im_conversation_agent_assignments_scope;

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
