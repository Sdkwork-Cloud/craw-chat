-- sdkwork:migration
-- id: 0012_conversation_typed_current_state
-- engine: postgres
-- module: im
-- purpose: Add typed Conversation policy, business binding, handoff, archive, and commit identity state
-- reversible: false
-- rollback: forward-fix
-- transactional: true
-- lock: access-exclusive
-- lock_timeout: 5s
-- statement_timeout: 2m
-- rewrite: none because the preflight requires an empty pre-launch Conversation table
-- backfill: forbidden; typed current state must not be reconstructed from journal payloads
-- write_traffic: Conversation writes must be stopped during cutover
-- replication_wal: bounded to DDL because the preflight requires an empty Conversation table
-- observability: monitor preflight failure, lock wait, DDL duration, and replica lag
-- cancellation: cancel before commit; PostgreSQL rolls back the complete transaction
-- recovery: correct through a forward migration and restore only from verified normalized backups
-- contract_version: 2.1.0

BEGIN;

SET LOCAL lock_timeout = '5s';
SET LOCAL statement_timeout = '2min';

DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM im_conversations LIMIT 1) THEN
        RAISE EXCEPTION
            'typed Conversation current-state cutover requires a pre-launch database reset; archive metadata, handoff state, policy, business binding, and commit identity cannot be synthesized from the commit journal';
    END IF;
END;
$$;

ALTER TABLE im_conversations
    ADD COLUMN archived_at TIMESTAMPTZ,
    ADD COLUMN archive_event_id TEXT,
    ADD COLUMN commit_fingerprint TEXT NOT NULL,
    ADD CONSTRAINT chk_im_conversations_archive_metadata CHECK (
        (lifecycle_state = 'active' AND archived_at IS NULL AND archive_event_id IS NULL)
        OR (lifecycle_state = 'archived' AND archived_at IS NOT NULL AND archive_event_id IS NOT NULL)
    );

CREATE TABLE im_conversation_policies (
    tenant_id TEXT NOT NULL,
    organization_id TEXT NOT NULL DEFAULT '0',
    conversation_id TEXT NOT NULL,
    policy_epoch BIGINT NOT NULL CHECK (policy_epoch >= 0),
    policy_version TEXT NOT NULL,
    capability_flags TEXT[],
    history_visibility TEXT NOT NULL,
    retention_policy_ref TEXT NOT NULL,
    max_members INTEGER,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT pk_im_conversation_policies PRIMARY KEY (tenant_id, organization_id, conversation_id),
    CONSTRAINT fk_im_conversation_policies_conversation FOREIGN KEY (
        tenant_id, organization_id, conversation_id
    ) REFERENCES im_conversations (tenant_id, organization_id, conversation_id) ON DELETE CASCADE,
    CONSTRAINT chk_im_conversation_policies_max_members CHECK (max_members IS NULL OR max_members > 0)
);

CREATE TABLE im_conversation_business_bindings (
    tenant_id TEXT NOT NULL,
    organization_id TEXT NOT NULL DEFAULT '0',
    conversation_id TEXT NOT NULL,
    business_type TEXT NOT NULL,
    business_id TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT pk_im_conversation_business_bindings PRIMARY KEY (tenant_id, organization_id, conversation_id),
    CONSTRAINT uk_im_conversation_business_bindings_business UNIQUE (
        tenant_id, organization_id, business_type, business_id
    ),
    CONSTRAINT fk_im_conversation_business_bindings_conversation FOREIGN KEY (
        tenant_id, organization_id, conversation_id
    ) REFERENCES im_conversations (tenant_id, organization_id, conversation_id) ON DELETE CASCADE,
    CONSTRAINT chk_im_conversation_business_bindings_values CHECK (
        length(trim(business_type)) > 0 AND length(trim(business_id)) > 0
    )
);

CREATE TABLE im_conversation_handoffs (
    tenant_id TEXT NOT NULL,
    organization_id TEXT NOT NULL DEFAULT '0',
    conversation_id TEXT NOT NULL,
    handoff_status_epoch BIGINT NOT NULL CHECK (handoff_status_epoch >= 0),
    status TEXT NOT NULL,
    source_principal_kind TEXT NOT NULL,
    source_principal_id TEXT NOT NULL,
    target_principal_kind TEXT NOT NULL,
    target_principal_id TEXT NOT NULL,
    handoff_session_id TEXT NOT NULL,
    handoff_reason TEXT,
    accepted_at TIMESTAMPTZ,
    accepted_by_principal_kind TEXT,
    accepted_by_principal_id TEXT,
    resolved_at TIMESTAMPTZ,
    resolved_by_principal_kind TEXT,
    resolved_by_principal_id TEXT,
    closed_at TIMESTAMPTZ,
    closed_by_principal_kind TEXT,
    closed_by_principal_id TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT pk_im_conversation_handoffs PRIMARY KEY (tenant_id, organization_id, conversation_id),
    CONSTRAINT uk_im_conversation_handoffs_session UNIQUE (tenant_id, organization_id, handoff_session_id),
    CONSTRAINT fk_im_conversation_handoffs_conversation FOREIGN KEY (
        tenant_id, organization_id, conversation_id
    ) REFERENCES im_conversations (tenant_id, organization_id, conversation_id) ON DELETE CASCADE,
    CONSTRAINT chk_im_conversation_handoffs_status CHECK (status IN ('open', 'accepted', 'resolved', 'closed')),
    CONSTRAINT chk_im_conversation_handoffs_accepted_actor CHECK (
        (accepted_by_principal_kind IS NULL) = (accepted_by_principal_id IS NULL)
    ),
    CONSTRAINT chk_im_conversation_handoffs_resolved_actor CHECK (
        (resolved_by_principal_kind IS NULL) = (resolved_by_principal_id IS NULL)
    ),
    CONSTRAINT chk_im_conversation_handoffs_closed_actor CHECK (
        (closed_by_principal_kind IS NULL) = (closed_by_principal_id IS NULL)
    )
);

COMMIT;
