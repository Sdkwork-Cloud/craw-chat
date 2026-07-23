-- Group knowledgebase immutable-binding upgrade.
--
-- IM stores only its Conversation-to-Knowledgebase relationship and opaque
-- launch-ticket state. The four remote identifiers are one immutable target
-- fence: space id/uuid and Knowledgebase binding id/uuid. This migration never
-- invents a missing remote UUID. A legacy active link or launch ticket
-- lacking that fence stops cutover and requires explicit pre-launch cleanup.

BEGIN;

CREATE TABLE IF NOT EXISTS im_conversation_knowledge_space_link (
    id BIGINT NOT NULL,
    link_uuid TEXT NOT NULL,
    tenant_id TEXT NOT NULL,
    organization_id TEXT NOT NULL,
    conversation_id TEXT NOT NULL,
    knowledge_space_id BIGINT,
    knowledge_space_uuid TEXT,
    knowledgebase_binding_id BIGINT,
    knowledgebase_binding_uuid TEXT,
    lifecycle_state TEXT NOT NULL DEFAULT 'provisioning',
    provisioning_operation_id TEXT,
    creation_idempotency_key TEXT NOT NULL,
    last_source_event_id TEXT,
    membership_epoch BIGINT NOT NULL DEFAULT 0 CHECK (membership_epoch >= 0),
    last_synchronized_membership_epoch BIGINT NOT NULL DEFAULT 0
        CHECK (last_synchronized_membership_epoch >= 0),
    last_error_code TEXT,
    last_error_at TIMESTAMPTZ,
    created_by TEXT NOT NULL,
    updated_by TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    archived_at TIMESTAMPTZ,
    deleted_at TIMESTAMPTZ,
    version BIGINT NOT NULL DEFAULT 1 CHECK (version > 0),
    CONSTRAINT pk_im_conversation_knowledge_space_link
        PRIMARY KEY (tenant_id, organization_id, conversation_id),
    CONSTRAINT uk_im_conversation_knowledge_space_link_id UNIQUE (id),
    CONSTRAINT uk_im_conversation_knowledge_space_link_uuid UNIQUE (link_uuid),
    CONSTRAINT chk_im_conversation_knowledge_space_link_tenant_id CHECK (
        tenant_id ~ '^[1-9][0-9]*$'
        AND (
            char_length(tenant_id) < 19
            OR (
                char_length(tenant_id) = 19
                AND tenant_id <= '9223372036854775807'
            )
        )
    ),
    CONSTRAINT chk_im_conversation_knowledge_space_link_state CHECK (
        lifecycle_state IN ('provisioning', 'active', 'failed', 'archived', 'deleted')
    ),
    CONSTRAINT chk_im_conversation_knowledge_space_link_active_reference CHECK (
        lifecycle_state <> 'active'
        OR (
            knowledge_space_id > 0
            AND NULLIF(BTRIM(knowledge_space_uuid), '') IS NOT NULL
            AND OCTET_LENGTH(knowledge_space_uuid) <= 256
            AND knowledgebase_binding_id > 0
            AND NULLIF(BTRIM(knowledgebase_binding_uuid), '') IS NOT NULL
            AND OCTET_LENGTH(knowledgebase_binding_uuid) <= 256
        )
    ),
    CONSTRAINT chk_im_conversation_knowledge_space_link_target_reference CHECK (
        (
            knowledge_space_id IS NULL
            AND knowledge_space_uuid IS NULL
            AND knowledgebase_binding_id IS NULL
            AND knowledgebase_binding_uuid IS NULL
        )
        OR (
            knowledge_space_id > 0
            AND NULLIF(BTRIM(knowledge_space_uuid), '') IS NOT NULL
            AND OCTET_LENGTH(knowledge_space_uuid) <= 256
            AND knowledgebase_binding_id > 0
            AND NULLIF(BTRIM(knowledgebase_binding_uuid), '') IS NOT NULL
            AND OCTET_LENGTH(knowledgebase_binding_uuid) <= 256
        )
    ),
    CONSTRAINT chk_im_conversation_knowledge_space_link_archived_at CHECK (
        (lifecycle_state = 'archived') = (archived_at IS NOT NULL)
    ),
    CONSTRAINT chk_im_conversation_knowledge_space_link_deleted_at CHECK (
        (lifecycle_state = 'deleted') = (deleted_at IS NOT NULL)
    ),
    CONSTRAINT chk_im_conversation_knowledge_space_link_membership_sync_epoch CHECK (
        last_synchronized_membership_epoch <= membership_epoch
    )
);

CREATE TABLE IF NOT EXISTS im_group_knowledge_launch_tickets (
    id BIGINT NOT NULL,
    ticket_hash TEXT NOT NULL,
    tenant_id TEXT NOT NULL,
    organization_id TEXT NOT NULL,
    conversation_id TEXT NOT NULL,
    knowledge_space_id BIGINT NOT NULL,
    knowledge_space_uuid TEXT NOT NULL,
    knowledgebase_binding_id BIGINT NOT NULL,
    knowledgebase_binding_uuid TEXT NOT NULL,
    upstream_link_generation BIGINT NOT NULL,
    membership_epoch BIGINT NOT NULL CHECK (membership_epoch >= 0),
    actor_kind TEXT NOT NULL,
    actor_id TEXT NOT NULL,
    principal_kind TEXT NOT NULL,
    principal_id TEXT NOT NULL,
    session_id TEXT NOT NULL,
    issuing_app_id TEXT,
    issued_by TEXT NOT NULL,
    idempotency_key_hash TEXT NOT NULL,
    request_fingerprint_hash TEXT NOT NULL,
    ticket_ciphertext TEXT NOT NULL,
    expires_at TIMESTAMPTZ NOT NULL,
    consumed_at TIMESTAMPTZ,
    consumed_by_service TEXT,
    consumed_trace_id TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT pk_im_group_knowledge_launch_tickets PRIMARY KEY (id),
    CONSTRAINT uk_im_group_knowledge_launch_tickets_hash UNIQUE (ticket_hash),
    CONSTRAINT uk_im_group_knowledge_launch_tickets_idempotency UNIQUE (
        tenant_id, organization_id, conversation_id, actor_kind, actor_id,
        principal_kind, principal_id, session_id, idempotency_key_hash
    ),
    CONSTRAINT chk_im_group_knowledge_launch_tickets_tenant_id CHECK (
        tenant_id ~ '^[1-9][0-9]*$'
        AND (
            char_length(tenant_id) < 19
            OR (
                char_length(tenant_id) = 19
                AND tenant_id <= '9223372036854775807'
            )
        )
    ),
    CONSTRAINT chk_im_group_knowledge_launch_tickets_delegated_user CHECK (
        actor_kind = 'user'
        AND principal_kind = 'user'
        AND actor_id = principal_id
    ),
    CONSTRAINT chk_im_group_knowledge_launch_tickets_upstream_link_generation CHECK (
        upstream_link_generation > 0
    ),
    CONSTRAINT chk_im_group_knowledge_launch_tickets_target_reference CHECK (
        knowledge_space_id > 0
        AND NULLIF(BTRIM(knowledge_space_uuid), '') IS NOT NULL
        AND OCTET_LENGTH(knowledge_space_uuid) <= 256
        AND knowledgebase_binding_id > 0
        AND NULLIF(BTRIM(knowledgebase_binding_uuid), '') IS NOT NULL
        AND OCTET_LENGTH(knowledgebase_binding_uuid) <= 256
    ),
    CONSTRAINT chk_im_group_knowledge_launch_tickets_expiry CHECK (expires_at > created_at),
    CONSTRAINT chk_im_group_knowledge_launch_tickets_consumer CHECK (
        (consumed_at IS NULL AND consumed_by_service IS NULL AND consumed_trace_id IS NULL)
        OR (consumed_at IS NOT NULL AND consumed_by_service IS NOT NULL AND consumed_trace_id IS NOT NULL)
    )
);

-- A prior development baseline may have created these tables without the UUID
-- target fence. Add columns explicitly; CREATE TABLE IF NOT EXISTS cannot
-- evolve an existing relation.
ALTER TABLE im_conversation_knowledge_space_link
    ADD COLUMN IF NOT EXISTS knowledgebase_binding_id BIGINT;
ALTER TABLE im_conversation_knowledge_space_link
    ADD COLUMN IF NOT EXISTS knowledgebase_binding_uuid TEXT;

ALTER TABLE im_group_knowledge_launch_tickets
    ADD COLUMN IF NOT EXISTS knowledgebase_binding_id BIGINT;
ALTER TABLE im_group_knowledge_launch_tickets
    ADD COLUMN IF NOT EXISTS knowledgebase_binding_uuid TEXT;
ALTER TABLE im_group_knowledge_launch_tickets
    ADD COLUMN IF NOT EXISTS upstream_link_generation BIGINT;

-- `binding_version` was IM's historical physical column, not a KB binding
-- version. Preserve its numeric value only as IM upstream link generation.
DO $$
BEGIN
    IF EXISTS (
        SELECT 1
        FROM information_schema.columns
        WHERE table_schema = current_schema()
          AND table_name = 'im_group_knowledge_launch_tickets'
          AND column_name = 'binding_version'
    ) THEN
        EXECUTE '
            UPDATE im_group_knowledge_launch_tickets
            SET upstream_link_generation = binding_version
            WHERE upstream_link_generation IS NULL
        ';
    END IF;
END
$$;

-- Fail closed instead of manufacturing immutable target identity. This is a
-- pre-launch migration: stale tickets and partial active/archived links
-- must be remediated before the capability is enabled.
DO $$
BEGIN
    IF EXISTS (
        SELECT 1
        FROM im_group_knowledge_launch_tickets
        WHERE knowledgebase_binding_id IS NULL
           OR NULLIF(BTRIM(knowledgebase_binding_uuid), '') IS NULL
           OR upstream_link_generation IS NULL
    ) THEN
        RAISE EXCEPTION
            'group knowledgebase launch tickets require immutable binding id/uuid and upstream link generation before cutover';
    END IF;
END
$$;

ALTER TABLE im_group_knowledge_launch_tickets
    ALTER COLUMN knowledgebase_binding_id SET NOT NULL;
ALTER TABLE im_group_knowledge_launch_tickets
    ALTER COLUMN knowledgebase_binding_uuid SET NOT NULL;
ALTER TABLE im_group_knowledge_launch_tickets
    ALTER COLUMN upstream_link_generation SET NOT NULL;
ALTER TABLE im_group_knowledge_launch_tickets
    DROP COLUMN IF EXISTS binding_version;

ALTER TABLE im_conversation_knowledge_space_link
    DROP CONSTRAINT IF EXISTS chk_im_conversation_knowledge_space_link_active_reference;
ALTER TABLE im_conversation_knowledge_space_link
    DROP CONSTRAINT IF EXISTS chk_im_conversation_knowledge_space_link_target_reference;
ALTER TABLE im_conversation_knowledge_space_link
    ADD CONSTRAINT chk_im_conversation_knowledge_space_link_active_reference CHECK (
        lifecycle_state <> 'active'
        OR (
            knowledge_space_id > 0
            AND NULLIF(BTRIM(knowledge_space_uuid), '') IS NOT NULL
            AND OCTET_LENGTH(knowledge_space_uuid) <= 256
            AND knowledgebase_binding_id > 0
            AND NULLIF(BTRIM(knowledgebase_binding_uuid), '') IS NOT NULL
            AND OCTET_LENGTH(knowledgebase_binding_uuid) <= 256
        )
    ) NOT VALID;
ALTER TABLE im_conversation_knowledge_space_link
    ADD CONSTRAINT chk_im_conversation_knowledge_space_link_target_reference CHECK (
        (
            knowledge_space_id IS NULL
            AND knowledge_space_uuid IS NULL
            AND knowledgebase_binding_id IS NULL
            AND knowledgebase_binding_uuid IS NULL
        )
        OR (
            knowledge_space_id > 0
            AND NULLIF(BTRIM(knowledge_space_uuid), '') IS NOT NULL
            AND OCTET_LENGTH(knowledge_space_uuid) <= 256
            AND knowledgebase_binding_id > 0
            AND NULLIF(BTRIM(knowledgebase_binding_uuid), '') IS NOT NULL
            AND OCTET_LENGTH(knowledgebase_binding_uuid) <= 256
        )
    ) NOT VALID;
ALTER TABLE im_conversation_knowledge_space_link
    VALIDATE CONSTRAINT chk_im_conversation_knowledge_space_link_active_reference;
ALTER TABLE im_conversation_knowledge_space_link
    VALIDATE CONSTRAINT chk_im_conversation_knowledge_space_link_target_reference;

ALTER TABLE im_group_knowledge_launch_tickets
    DROP CONSTRAINT IF EXISTS chk_im_group_knowledge_launch_tickets_upstream_link_generation;
ALTER TABLE im_group_knowledge_launch_tickets
    DROP CONSTRAINT IF EXISTS chk_im_group_knowledge_launch_tickets_target_reference;
ALTER TABLE im_group_knowledge_launch_tickets
    ADD CONSTRAINT chk_im_group_knowledge_launch_tickets_upstream_link_generation CHECK (
        upstream_link_generation > 0
    ) NOT VALID;
ALTER TABLE im_group_knowledge_launch_tickets
    ADD CONSTRAINT chk_im_group_knowledge_launch_tickets_target_reference CHECK (
        knowledge_space_id > 0
        AND NULLIF(BTRIM(knowledge_space_uuid), '') IS NOT NULL
        AND OCTET_LENGTH(knowledge_space_uuid) <= 256
        AND knowledgebase_binding_id > 0
        AND NULLIF(BTRIM(knowledgebase_binding_uuid), '') IS NOT NULL
        AND OCTET_LENGTH(knowledgebase_binding_uuid) <= 256
    ) NOT VALID;
ALTER TABLE im_group_knowledge_launch_tickets
    VALIDATE CONSTRAINT chk_im_group_knowledge_launch_tickets_upstream_link_generation;
ALTER TABLE im_group_knowledge_launch_tickets
    VALIDATE CONSTRAINT chk_im_group_knowledge_launch_tickets_target_reference;

CREATE UNIQUE INDEX IF NOT EXISTS uk_im_conversation_knowledge_space_link_space
    ON im_conversation_knowledge_space_link (knowledge_space_id)
    WHERE knowledge_space_id IS NOT NULL
      AND lifecycle_state IN ('provisioning', 'active', 'archived');
CREATE UNIQUE INDEX IF NOT EXISTS uk_im_conversation_knowledge_space_link_binding
    ON im_conversation_knowledge_space_link (knowledgebase_binding_id)
    WHERE knowledgebase_binding_id IS NOT NULL
      AND lifecycle_state IN ('provisioning', 'active', 'archived');
CREATE INDEX IF NOT EXISTS idx_im_conversation_knowledge_space_link_state
    ON im_conversation_knowledge_space_link (
        tenant_id, organization_id, lifecycle_state, updated_at, conversation_id
    );
CREATE INDEX IF NOT EXISTS idx_im_group_knowledge_launch_tickets_expiry
    ON im_group_knowledge_launch_tickets (tenant_id, organization_id, expires_at)
    WHERE consumed_at IS NULL;
CREATE INDEX IF NOT EXISTS idx_im_group_knowledge_launch_tickets_actor
    ON im_group_knowledge_launch_tickets (
        tenant_id, organization_id, actor_kind, actor_id, principal_kind,
        principal_id, session_id, created_at DESC
    );

-- Group Knowledgebase shares the existing KB/Drive signed BIGINT scope.
-- The text storage boundary preserves the wider IM schema convention without
-- accepting ids that cannot cross the generated internal RPC boundary.
ALTER TABLE im_conversation_knowledge_space_link
    DROP CONSTRAINT IF EXISTS chk_im_conversation_knowledge_space_link_tenant_id;
ALTER TABLE im_conversation_knowledge_space_link
    ADD CONSTRAINT chk_im_conversation_knowledge_space_link_tenant_id CHECK (
        tenant_id ~ '^[1-9][0-9]*$'
        AND (
            char_length(tenant_id) < 19
            OR (
                char_length(tenant_id) = 19
                AND tenant_id <= '9223372036854775807'
            )
        )
    ) NOT VALID;
ALTER TABLE im_conversation_knowledge_space_link
    VALIDATE CONSTRAINT chk_im_conversation_knowledge_space_link_tenant_id;

ALTER TABLE im_conversation_knowledge_space_link
    ALTER COLUMN organization_id DROP DEFAULT;
ALTER TABLE im_conversation_knowledge_space_link
    DROP CONSTRAINT IF EXISTS chk_im_conversation_knowledge_space_link_organization_id;
ALTER TABLE im_conversation_knowledge_space_link
    ADD CONSTRAINT chk_im_conversation_knowledge_space_link_organization_id CHECK (
        organization_id ~ '^[1-9][0-9]*$'
        AND (
            char_length(organization_id) < 19
            OR (
                char_length(organization_id) = 19
                AND organization_id <= '9223372036854775807'
            )
        )
    ) NOT VALID;
ALTER TABLE im_conversation_knowledge_space_link
    VALIDATE CONSTRAINT chk_im_conversation_knowledge_space_link_organization_id;

ALTER TABLE im_group_knowledge_launch_tickets
    DROP CONSTRAINT IF EXISTS chk_im_group_knowledge_launch_tickets_tenant_id;
ALTER TABLE im_group_knowledge_launch_tickets
    ADD CONSTRAINT chk_im_group_knowledge_launch_tickets_tenant_id CHECK (
        tenant_id ~ '^[1-9][0-9]*$'
        AND (
            char_length(tenant_id) < 19
            OR (
                char_length(tenant_id) = 19
                AND tenant_id <= '9223372036854775807'
            )
        )
    ) NOT VALID;
ALTER TABLE im_group_knowledge_launch_tickets
    VALIDATE CONSTRAINT chk_im_group_knowledge_launch_tickets_tenant_id;

ALTER TABLE im_group_knowledge_launch_tickets
    ALTER COLUMN organization_id DROP DEFAULT;
ALTER TABLE im_group_knowledge_launch_tickets
    DROP CONSTRAINT IF EXISTS chk_im_group_knowledge_launch_tickets_organization_id;
ALTER TABLE im_group_knowledge_launch_tickets
    ADD CONSTRAINT chk_im_group_knowledge_launch_tickets_organization_id CHECK (
        organization_id ~ '^[1-9][0-9]*$'
        AND (
            char_length(organization_id) < 19
            OR (
                char_length(organization_id) = 19
                AND organization_id <= '9223372036854775807'
            )
        )
    ) NOT VALID;
ALTER TABLE im_group_knowledge_launch_tickets
    VALIDATE CONSTRAINT chk_im_group_knowledge_launch_tickets_organization_id;

COMMIT;
