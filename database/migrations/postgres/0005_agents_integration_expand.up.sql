-- sdkwork:migration
-- id: 0005_agents_integration_expand
-- engine: postgres
-- module: im
-- purpose: Create normalized IM-side Agent assignment, binding, and dispatch tables
-- reversible: true
-- rollback: down-migration
-- transactional: true
-- lock: schema-definition
-- lock_timeout: 5s
-- statement_timeout: 2m
-- rewrite: none; additive tables and indexes only
-- backfill: none; pre-launch runtime writes normalized rows after cutover
-- write_traffic: no existing IM table rewrite
-- replication_wal: bounded by DDL catalog and empty-index creation
-- observability: monitor DDL duration, blocked locks, and replica apply lag
-- cancellation: cancel before commit; PostgreSQL rolls back the complete transaction
-- recovery: guarded down migration is allowed only while all three tables are empty
-- contract_version: 2.0.0

BEGIN;

SET LOCAL lock_timeout = '5s';
SET LOCAL statement_timeout = '2min';

CREATE TABLE im_conversation_agent_assignments (
    id BIGINT NOT NULL PRIMARY KEY,
    uuid VARCHAR(96) NOT NULL UNIQUE,
    tenant_id BIGINT NOT NULL,
    organization_id BIGINT NOT NULL DEFAULT 0,
    conversation_id VARCHAR(128) NOT NULL,
    agent_id VARCHAR(128) NOT NULL,
    agent_revision_ref VARCHAR(128),
    assignment_source SMALLINT NOT NULL,
    assignment_generation BIGINT NOT NULL,
    position INTEGER NOT NULL,
    enabled BOOLEAN NOT NULL DEFAULT TRUE,
    status SMALLINT NOT NULL DEFAULT 0,
    assigned_by BIGINT NOT NULL,
    assigned_at TIMESTAMPTZ NOT NULL,
    source_event_id VARCHAR(128) NOT NULL,
    source_aggregate_version BIGINT NOT NULL,
    payload_hash VARCHAR(128) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL,
    retention_until TIMESTAMPTZ,
    CONSTRAINT uk_im_conversation_agent_assignments_scope
        UNIQUE (tenant_id, organization_id, conversation_id, agent_id),
    CONSTRAINT ck_im_conversation_agent_assignments_source CHECK (
        assignment_source IN (0, 1)
    ),
    CONSTRAINT ck_im_conversation_agent_assignments_generation CHECK (
        assignment_generation > 0 AND source_aggregate_version > 0
    ),
    CONSTRAINT ck_im_conversation_agent_assignments_position CHECK (position >= 0),
    CONSTRAINT ck_im_conversation_agent_assignments_status CHECK (status IN (0, 1, 2))
);

CREATE UNIQUE INDEX uk_im_conversation_agent_assignments_position
    ON im_conversation_agent_assignments (
        tenant_id, organization_id, conversation_id, position
    ) WHERE enabled = TRUE AND status = 0;
CREATE INDEX idx_im_conversation_agent_assignments_list
    ON im_conversation_agent_assignments (
        tenant_id, organization_id, conversation_id, status, position, id
    );
CREATE INDEX idx_im_conversation_agent_assignments_reverse
    ON im_conversation_agent_assignments (
        tenant_id, organization_id, agent_id, status, updated_at DESC, id DESC
    );
CREATE INDEX idx_im_conversation_agent_assignments_retention
    ON im_conversation_agent_assignments (
        tenant_id, organization_id, retention_until, id
    ) WHERE retention_until IS NOT NULL;

CREATE TABLE im_conversation_agent_binding (
    id BIGINT NOT NULL PRIMARY KEY,
    uuid VARCHAR(96) NOT NULL UNIQUE,
    binding_id VARCHAR(128) NOT NULL,
    tenant_id BIGINT NOT NULL,
    organization_id BIGINT NOT NULL DEFAULT 0,
    conversation_id VARCHAR(128) NOT NULL,
    agent_id VARCHAR(128) NOT NULL,
    agent_revision_ref VARCHAR(128),
    assignment_generation BIGINT NOT NULL,
    agents_session_id VARCHAR(128),
    status SMALLINT NOT NULL DEFAULT 0,
    idempotency_key VARCHAR(256) NOT NULL,
    payload_hash VARCHAR(128) NOT NULL,
    created_by BIGINT NOT NULL,
    updated_by BIGINT NOT NULL,
    last_used_at TIMESTAMPTZ,
    closed_at TIMESTAMPTZ,
    last_error_code VARCHAR(128),
    last_error_detail VARCHAR(2048),
    version BIGINT NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL,
    retention_until TIMESTAMPTZ,
    CONSTRAINT uk_im_conversation_agent_binding_scope_id
        UNIQUE (tenant_id, organization_id, binding_id),
    CONSTRAINT uk_im_conversation_agent_binding_generation
        UNIQUE (
            tenant_id, organization_id, conversation_id, agent_id,
            assignment_generation
        ),
    CONSTRAINT uk_im_conversation_agent_binding_idempotency
        UNIQUE (tenant_id, organization_id, idempotency_key),
    CONSTRAINT ck_im_conversation_agent_binding_generation CHECK (
        assignment_generation > 0
    ),
    CONSTRAINT ck_im_conversation_agent_binding_status CHECK (
        status IN (0, 1, 2, 3, 4)
    ),
    CONSTRAINT ck_im_conversation_agent_binding_active_session CHECK (
        status <> 1 OR agents_session_id IS NOT NULL
    ),
    CONSTRAINT ck_im_conversation_agent_binding_version CHECK (version >= 0)
);

CREATE UNIQUE INDEX uk_im_conversation_agent_binding_active
    ON im_conversation_agent_binding (
        tenant_id, organization_id, conversation_id, agent_id
    ) WHERE status = 1;
CREATE INDEX idx_im_conversation_agent_binding_resolve
    ON im_conversation_agent_binding (
        tenant_id, organization_id, conversation_id, agent_id,
        status, assignment_generation DESC, id DESC
    );
CREATE INDEX idx_im_conversation_agent_binding_session
    ON im_conversation_agent_binding (
        tenant_id, organization_id, agents_session_id
    ) WHERE agents_session_id IS NOT NULL;
CREATE INDEX idx_im_conversation_agent_binding_lifecycle
    ON im_conversation_agent_binding (
        tenant_id, organization_id, status, updated_at, retention_until, id
    );

CREATE TABLE im_agent_dispatch (
    id BIGINT NOT NULL PRIMARY KEY,
    uuid VARCHAR(96) NOT NULL UNIQUE,
    dispatch_id VARCHAR(128) NOT NULL,
    tenant_id BIGINT NOT NULL,
    organization_id BIGINT NOT NULL DEFAULT 0,
    conversation_id VARCHAR(128) NOT NULL,
    source_message_id BIGINT NOT NULL,
    source_message_seq BIGINT NOT NULL,
    agent_id VARCHAR(128) NOT NULL,
    agent_revision_ref VARCHAR(128),
    assignment_generation BIGINT NOT NULL,
    binding_id VARCHAR(128),
    agents_session_id VARCHAR(128),
    agents_turn_id VARCHAR(128),
    status SMALLINT NOT NULL DEFAULT 0,
    idempotency_key VARCHAR(256) NOT NULL,
    payload_hash VARCHAR(128) NOT NULL,
    attempt_count INTEGER NOT NULL DEFAULT 0,
    max_attempts INTEGER NOT NULL DEFAULT 10,
    lease_owner VARCHAR(128),
    lease_expires_at TIMESTAMPTZ,
    next_attempt_at TIMESTAMPTZ NOT NULL,
    last_error_code VARCHAR(128),
    last_error_detail VARCHAR(2048),
    requested_by BIGINT NOT NULL,
    reply_message_id BIGINT,
    reply_message_seq BIGINT,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL,
    started_at TIMESTAMPTZ,
    completed_at TIMESTAMPTZ,
    cancelled_at TIMESTAMPTZ,
    retention_until TIMESTAMPTZ,
    CONSTRAINT uk_im_agent_dispatch_scope_id
        UNIQUE (tenant_id, organization_id, dispatch_id),
    CONSTRAINT uk_im_agent_dispatch_source_target
        UNIQUE (
            tenant_id, organization_id, conversation_id, source_message_id,
            agent_id, assignment_generation
        ),
    CONSTRAINT uk_im_agent_dispatch_idempotency
        UNIQUE (tenant_id, organization_id, idempotency_key),
    CONSTRAINT ck_im_agent_dispatch_message_seq CHECK (
        source_message_seq > 0 AND (reply_message_seq IS NULL OR reply_message_seq > 0)
    ),
    CONSTRAINT ck_im_agent_dispatch_generation CHECK (assignment_generation > 0),
    CONSTRAINT ck_im_agent_dispatch_status CHECK (status IN (0, 1, 2, 3, 4, 5, 6, 7)),
    CONSTRAINT ck_im_agent_dispatch_attempts CHECK (
        attempt_count >= 0 AND max_attempts > 0 AND attempt_count <= max_attempts
    ),
    CONSTRAINT ck_im_agent_dispatch_reply_pair CHECK (
        (reply_message_id IS NULL AND reply_message_seq IS NULL)
        OR (reply_message_id IS NOT NULL AND reply_message_seq IS NOT NULL)
    ),
    CONSTRAINT ck_im_agent_dispatch_reply_distinct CHECK (
        reply_message_id IS NULL OR reply_message_id <> source_message_id
    ),
    CONSTRAINT fk_im_agent_dispatch_binding
        FOREIGN KEY (tenant_id, organization_id, binding_id)
        REFERENCES im_conversation_agent_binding (
            tenant_id, organization_id, binding_id
        ) ON DELETE RESTRICT DEFERRABLE INITIALLY DEFERRED
);

CREATE INDEX idx_im_agent_dispatch_worker
    ON im_agent_dispatch (
        tenant_id, organization_id, status, next_attempt_at,
        lease_expires_at, id
    ) WHERE status IN (0, 1, 2, 3, 4);
CREATE INDEX idx_im_agent_dispatch_source
    ON im_agent_dispatch (
        tenant_id, organization_id, conversation_id, source_message_seq,
        status, id
    );
CREATE INDEX idx_im_agent_dispatch_turn
    ON im_agent_dispatch (tenant_id, organization_id, agents_turn_id)
    WHERE agents_turn_id IS NOT NULL;
CREATE INDEX idx_im_agent_dispatch_reply
    ON im_agent_dispatch (
        tenant_id, organization_id, conversation_id, reply_message_seq
    ) WHERE reply_message_seq IS NOT NULL;
CREATE INDEX idx_im_agent_dispatch_retention
    ON im_agent_dispatch (tenant_id, organization_id, retention_until, id)
    WHERE retention_until IS NOT NULL;

COMMIT;
