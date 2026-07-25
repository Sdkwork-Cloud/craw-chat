-- sdkwork:migration
-- id: 0008_allow_system_assignment_actor
-- engine: postgres
-- module: im
-- purpose: Permit the canonical system actor in normalized Agent assignments
-- reversible: true
-- rollback: down-migration
-- transactional: true
-- lock: access-exclusive
-- lock_timeout: 5s
-- statement_timeout: 2m
-- rewrite: none; replaces one CHECK constraint
-- backfill: none
-- write_traffic: assignment writes may block while the constraint is replaced and validated
-- replication_wal: metadata-only apart from validation visibility-map activity
-- observability: monitor validation scan duration and lock wait
-- cancellation: cancel before commit; PostgreSQL rolls back the complete transaction
-- recovery: guarded down migration refuses rollback when system-authored rows exist
-- contract_version: 2.0.0

BEGIN;

SET LOCAL lock_timeout = '5s';
SET LOCAL statement_timeout = '2min';

ALTER TABLE im_conversation_agent_assignments
    DROP CONSTRAINT ck_im_conversation_agent_assignments_scope;
ALTER TABLE im_conversation_agent_assignments
    ADD CONSTRAINT ck_im_conversation_agent_assignments_scope CHECK (
        tenant_id > 0 AND organization_id >= 0 AND assigned_by >= 0
    ) NOT VALID;
ALTER TABLE im_conversation_agent_assignments
    VALIDATE CONSTRAINT ck_im_conversation_agent_assignments_scope;

COMMIT;
