-- sdkwork:migration
-- id: 0009_allow_zero_source_aggregate_version
-- engine: postgres
-- module: im
-- purpose: Permit generation-one assignment snapshots emitted from aggregate version zero
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
-- recovery: guarded down migration refuses rollback when zero-version rows exist
-- contract_version: 2.0.0

BEGIN;

SET LOCAL lock_timeout = '5s';
SET LOCAL statement_timeout = '2min';

ALTER TABLE im_conversation_agent_assignments
    DROP CONSTRAINT ck_im_conversation_agent_assignments_generation;
ALTER TABLE im_conversation_agent_assignments
    ADD CONSTRAINT ck_im_conversation_agent_assignments_generation CHECK (
        assignment_generation > 0 AND source_aggregate_version >= 0
    ) NOT VALID;
ALTER TABLE im_conversation_agent_assignments
    VALIDATE CONSTRAINT ck_im_conversation_agent_assignments_generation;

COMMIT;
