-- Group knowledgebase immutable-binding upgrade for SQLite lifecycle parity.
--
-- The source shape below is the pre-release 0003 table layout. SQLite needs a
-- table rebuild to remove its retired binding_version column and make target
-- fence fields non-null. The copy is intentionally fail-closed: no binding id
-- or UUID is manufactured for historic active links or launch tickets.

BEGIN;

-- Ensure a prior-baseline source relation exists so a clean legacy upgrade and
-- an empty pre-feature installation follow exactly the same conversion path.
CREATE TABLE IF NOT EXISTS im_conversation_knowledge_space_link (
    id INTEGER NOT NULL,
    link_uuid TEXT NOT NULL,
    tenant_id TEXT NOT NULL CHECK (
        tenant_id GLOB '[1-9]*'
        AND tenant_id NOT GLOB '*[^0-9]*'
        AND (
            length(tenant_id) < 19
            OR (
                length(tenant_id) = 19
                AND tenant_id <= '9223372036854775807'
            )
        )
    ),
    organization_id TEXT NOT NULL,
    conversation_id TEXT NOT NULL,
    knowledge_space_id INTEGER,
    knowledge_space_uuid TEXT,
    knowledgebase_binding_id INTEGER,
    lifecycle_state TEXT NOT NULL DEFAULT 'provisioning',
    provisioning_operation_id TEXT,
    creation_idempotency_key TEXT NOT NULL,
    last_source_event_id TEXT,
    membership_epoch INTEGER NOT NULL DEFAULT 0 CHECK (membership_epoch >= 0),
    last_synchronized_membership_epoch INTEGER NOT NULL DEFAULT 0
        CHECK (last_synchronized_membership_epoch >= 0),
    last_error_code TEXT,
    last_error_at TEXT,
    created_by TEXT NOT NULL,
    updated_by TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    archived_at TEXT,
    deleted_at TEXT,
    version INTEGER NOT NULL DEFAULT 1 CHECK (version > 0),
    CONSTRAINT pk_im_conversation_knowledge_space_link
        PRIMARY KEY (tenant_id, organization_id, conversation_id),
    CONSTRAINT uk_im_conversation_knowledge_space_link_id UNIQUE (id),
    CONSTRAINT uk_im_conversation_knowledge_space_link_uuid UNIQUE (link_uuid)
);

CREATE TABLE IF NOT EXISTS im_group_knowledge_launch_tickets (
    id INTEGER NOT NULL PRIMARY KEY,
    ticket_hash TEXT NOT NULL UNIQUE,
    tenant_id TEXT NOT NULL CHECK (
        tenant_id GLOB '[1-9]*'
        AND tenant_id NOT GLOB '*[^0-9]*'
        AND (
            length(tenant_id) < 19
            OR (
                length(tenant_id) = 19
                AND tenant_id <= '9223372036854775807'
            )
        )
    ),
    organization_id TEXT NOT NULL,
    conversation_id TEXT NOT NULL,
    knowledge_space_id INTEGER NOT NULL,
    knowledge_space_uuid TEXT NOT NULL,
    binding_version INTEGER NOT NULL CHECK (binding_version > 0),
    membership_epoch INTEGER NOT NULL CHECK (membership_epoch >= 0),
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
    expires_at TEXT NOT NULL,
    consumed_at TEXT,
    consumed_by_service TEXT,
    consumed_trace_id TEXT,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT uk_im_group_knowledge_launch_tickets_idempotency UNIQUE (
        tenant_id, organization_id, conversation_id, actor_kind, actor_id,
        principal_kind, principal_id, session_id, idempotency_key_hash
    )
);

DROP TRIGGER IF EXISTS trg_im_conversation_knowledge_space_link_org_insert;
DROP TRIGGER IF EXISTS trg_im_conversation_knowledge_space_link_org_update;
DROP TRIGGER IF EXISTS trg_im_group_knowledge_launch_tickets_org_insert;
DROP TRIGGER IF EXISTS trg_im_group_knowledge_launch_tickets_org_update;
DROP TRIGGER IF EXISTS trg_im_conversation_knowledge_space_link_tenant_insert;
DROP TRIGGER IF EXISTS trg_im_conversation_knowledge_space_link_tenant_update;
DROP TRIGGER IF EXISTS trg_im_group_knowledge_launch_tickets_tenant_insert;
DROP TRIGGER IF EXISTS trg_im_group_knowledge_launch_tickets_tenant_update;

ALTER TABLE im_conversation_knowledge_space_link
    RENAME TO im_conversation_knowledge_space_link_legacy_0003;

CREATE TABLE im_conversation_knowledge_space_link_next_0003 (
    id INTEGER NOT NULL,
    link_uuid TEXT NOT NULL,
    tenant_id TEXT NOT NULL CHECK (
        tenant_id GLOB '[1-9]*'
        AND tenant_id NOT GLOB '*[^0-9]*'
        AND (
            length(tenant_id) < 19
            OR (
                length(tenant_id) = 19
                AND tenant_id <= '9223372036854775807'
            )
        )
    ),
    organization_id TEXT NOT NULL CHECK (
        organization_id GLOB '[1-9]*'
        AND organization_id NOT GLOB '*[^0-9]*'
        AND (
            length(organization_id) < 19
            OR (
                length(organization_id) = 19
                AND organization_id <= '9223372036854775807'
            )
        )
    ),
    conversation_id TEXT NOT NULL,
    knowledge_space_id INTEGER,
    knowledge_space_uuid TEXT,
    knowledgebase_binding_id INTEGER,
    knowledgebase_binding_uuid TEXT,
    lifecycle_state TEXT NOT NULL DEFAULT 'provisioning',
    provisioning_operation_id TEXT,
    creation_idempotency_key TEXT NOT NULL,
    last_source_event_id TEXT,
    membership_epoch INTEGER NOT NULL DEFAULT 0 CHECK (membership_epoch >= 0),
    last_synchronized_membership_epoch INTEGER NOT NULL DEFAULT 0
        CHECK (last_synchronized_membership_epoch >= 0),
    last_error_code TEXT,
    last_error_at TEXT,
    created_by TEXT NOT NULL,
    updated_by TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    archived_at TEXT,
    deleted_at TEXT,
    version INTEGER NOT NULL DEFAULT 1 CHECK (version > 0),
    CONSTRAINT pk_im_conversation_knowledge_space_link
        PRIMARY KEY (tenant_id, organization_id, conversation_id),
    CONSTRAINT uk_im_conversation_knowledge_space_link_id UNIQUE (id),
    CONSTRAINT uk_im_conversation_knowledge_space_link_uuid UNIQUE (link_uuid),
    CONSTRAINT chk_im_conversation_knowledge_space_link_state CHECK (
        lifecycle_state IN ('provisioning', 'active', 'failed', 'archived', 'deleted')
    ),
    CONSTRAINT chk_im_conversation_knowledge_space_link_active_reference CHECK (
        lifecycle_state <> 'active'
        OR (
            knowledge_space_id > 0
            AND NULLIF(TRIM(knowledge_space_uuid), '') IS NOT NULL
            AND length(CAST(knowledge_space_uuid AS BLOB)) <= 256
            AND knowledgebase_binding_id > 0
            AND NULLIF(TRIM(knowledgebase_binding_uuid), '') IS NOT NULL
            AND length(CAST(knowledgebase_binding_uuid AS BLOB)) <= 256
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
            AND NULLIF(TRIM(knowledge_space_uuid), '') IS NOT NULL
            AND length(CAST(knowledge_space_uuid AS BLOB)) <= 256
            AND knowledgebase_binding_id > 0
            AND NULLIF(TRIM(knowledgebase_binding_uuid), '') IS NOT NULL
            AND length(CAST(knowledgebase_binding_uuid AS BLOB)) <= 256
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

-- NULL is deliberate. Any legacy remote reference is incomplete because the
-- old table had no binding UUID. Active/archived rows therefore reject rather
-- than silently targeting an unverified Knowledgebase binding.
INSERT INTO im_conversation_knowledge_space_link_next_0003 (
    id, link_uuid, tenant_id, organization_id, conversation_id,
    knowledge_space_id, knowledge_space_uuid, knowledgebase_binding_id,
    knowledgebase_binding_uuid, lifecycle_state, provisioning_operation_id,
    creation_idempotency_key, last_source_event_id, membership_epoch,
    last_synchronized_membership_epoch, last_error_code, last_error_at,
    created_by, updated_by, created_at, updated_at, archived_at, deleted_at,
    version
)
SELECT
    id, link_uuid, tenant_id, organization_id, conversation_id,
    knowledge_space_id, knowledge_space_uuid, knowledgebase_binding_id,
    NULL, lifecycle_state, provisioning_operation_id, creation_idempotency_key,
    last_source_event_id, membership_epoch, last_synchronized_membership_epoch,
    last_error_code, last_error_at, created_by, updated_by, created_at,
    updated_at, archived_at, deleted_at, version
FROM im_conversation_knowledge_space_link_legacy_0003;

DROP TABLE im_conversation_knowledge_space_link_legacy_0003;
ALTER TABLE im_conversation_knowledge_space_link_next_0003
    RENAME TO im_conversation_knowledge_space_link;

ALTER TABLE im_group_knowledge_launch_tickets
    RENAME TO im_group_knowledge_launch_tickets_legacy_0003;

CREATE TABLE im_group_knowledge_launch_tickets_next_0003 (
    id INTEGER NOT NULL PRIMARY KEY,
    ticket_hash TEXT NOT NULL UNIQUE,
    tenant_id TEXT NOT NULL CHECK (
        tenant_id GLOB '[1-9]*'
        AND tenant_id NOT GLOB '*[^0-9]*'
        AND (
            length(tenant_id) < 19
            OR (
                length(tenant_id) = 19
                AND tenant_id <= '9223372036854775807'
            )
        )
    ),
    organization_id TEXT NOT NULL CHECK (
        organization_id GLOB '[1-9]*'
        AND organization_id NOT GLOB '*[^0-9]*'
        AND (
            length(organization_id) < 19
            OR (
                length(organization_id) = 19
                AND organization_id <= '9223372036854775807'
            )
        )
    ),
    conversation_id TEXT NOT NULL,
    knowledge_space_id INTEGER NOT NULL,
    knowledge_space_uuid TEXT NOT NULL,
    knowledgebase_binding_id INTEGER NOT NULL CHECK (knowledgebase_binding_id > 0),
    knowledgebase_binding_uuid TEXT NOT NULL,
    upstream_link_generation INTEGER NOT NULL CHECK (upstream_link_generation > 0),
    membership_epoch INTEGER NOT NULL CHECK (membership_epoch >= 0),
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
    expires_at TEXT NOT NULL,
    consumed_at TEXT,
    consumed_by_service TEXT,
    consumed_trace_id TEXT,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT uk_im_group_knowledge_launch_tickets_idempotency UNIQUE (
        tenant_id, organization_id, conversation_id, actor_kind, actor_id,
        principal_kind, principal_id, session_id, idempotency_key_hash
    ),
    CONSTRAINT chk_im_group_knowledge_launch_tickets_delegated_user CHECK (
        actor_kind = 'user' AND principal_kind = 'user' AND actor_id = principal_id
    ),
    CONSTRAINT chk_im_group_knowledge_launch_tickets_target_reference CHECK (
        knowledge_space_id > 0
        AND NULLIF(TRIM(knowledge_space_uuid), '') IS NOT NULL
        AND length(CAST(knowledge_space_uuid AS BLOB)) <= 256
        AND knowledgebase_binding_id > 0
        AND NULLIF(TRIM(knowledgebase_binding_uuid), '') IS NOT NULL
        AND length(CAST(knowledgebase_binding_uuid AS BLOB)) <= 256
    ),
    CONSTRAINT chk_im_group_knowledge_launch_tickets_expiry CHECK (expires_at > created_at),
    CONSTRAINT chk_im_group_knowledge_launch_tickets_consumer CHECK (
        (consumed_at IS NULL AND consumed_by_service IS NULL AND consumed_trace_id IS NULL)
        OR (consumed_at IS NOT NULL AND consumed_by_service IS NOT NULL AND consumed_trace_id IS NOT NULL)
    )
);

-- The folded baseline already has the final ticket columns, while a legacy
-- ticket table has only the retired `binding_version` column. SQLite cannot
-- conditionally reference one of those two physical columns in static SQL.
-- Both layouts share the fields selected below, so no legacy generation is
-- copied. A non-empty legacy ticket table therefore fails the immutable target
-- fence atomically instead of manufacturing an unverified KB target. An empty
-- folded-baseline table upgrades without relying on the retired column.
INSERT INTO im_group_knowledge_launch_tickets_next_0003 (
    id, ticket_hash, tenant_id, organization_id, conversation_id,
    knowledge_space_id, knowledge_space_uuid, knowledgebase_binding_id,
    knowledgebase_binding_uuid, upstream_link_generation, membership_epoch,
    actor_kind, actor_id, principal_kind, principal_id, session_id,
    issuing_app_id, issued_by, idempotency_key_hash, request_fingerprint_hash,
    ticket_ciphertext, expires_at, consumed_at, consumed_by_service,
    consumed_trace_id, created_at
)
SELECT
    id, ticket_hash, tenant_id, organization_id, conversation_id,
    knowledge_space_id, knowledge_space_uuid, NULL, NULL, NULL,
    membership_epoch, actor_kind, actor_id, principal_kind, principal_id,
    session_id, issuing_app_id, issued_by, idempotency_key_hash,
    request_fingerprint_hash, ticket_ciphertext, expires_at, consumed_at,
    consumed_by_service, consumed_trace_id, created_at
FROM im_group_knowledge_launch_tickets_legacy_0003;

DROP TABLE im_group_knowledge_launch_tickets_legacy_0003;
ALTER TABLE im_group_knowledge_launch_tickets_next_0003
    RENAME TO im_group_knowledge_launch_tickets;

CREATE UNIQUE INDEX uk_im_conversation_knowledge_space_link_space
    ON im_conversation_knowledge_space_link (knowledge_space_id)
    WHERE knowledge_space_id IS NOT NULL
      AND lifecycle_state IN ('provisioning', 'active', 'archived');
CREATE UNIQUE INDEX uk_im_conversation_knowledge_space_link_binding
    ON im_conversation_knowledge_space_link (knowledgebase_binding_id)
    WHERE knowledgebase_binding_id IS NOT NULL
      AND lifecycle_state IN ('provisioning', 'active', 'archived');
CREATE INDEX idx_im_conversation_knowledge_space_link_state
    ON im_conversation_knowledge_space_link (
        tenant_id, organization_id, lifecycle_state, updated_at, conversation_id
    );
CREATE INDEX idx_im_group_knowledge_launch_tickets_expiry
    ON im_group_knowledge_launch_tickets (tenant_id, organization_id, expires_at)
    WHERE consumed_at IS NULL;
CREATE INDEX idx_im_group_knowledge_launch_tickets_actor
    ON im_group_knowledge_launch_tickets (
        tenant_id, organization_id, actor_kind, actor_id, principal_kind,
        principal_id, session_id, created_at DESC
    );

CREATE TRIGGER trg_im_conversation_knowledge_space_link_tenant_insert
BEFORE INSERT ON im_conversation_knowledge_space_link
WHEN NEW.tenant_id IS NULL
    OR NEW.tenant_id NOT GLOB '[1-9]*'
    OR NEW.tenant_id GLOB '*[^0-9]*'
    OR length(NEW.tenant_id) > 19
    OR (
        length(NEW.tenant_id) = 19
        AND NEW.tenant_id > '9223372036854775807'
    )
BEGIN
    SELECT RAISE(ABORT, 'im_conversation_knowledge_space_link.tenant_id must be a canonical positive signed i64');
END;

CREATE TRIGGER trg_im_conversation_knowledge_space_link_tenant_update
BEFORE UPDATE OF tenant_id ON im_conversation_knowledge_space_link
WHEN NEW.tenant_id IS NULL
    OR NEW.tenant_id NOT GLOB '[1-9]*'
    OR NEW.tenant_id GLOB '*[^0-9]*'
    OR length(NEW.tenant_id) > 19
    OR (
        length(NEW.tenant_id) = 19
        AND NEW.tenant_id > '9223372036854775807'
    )
BEGIN
    SELECT RAISE(ABORT, 'im_conversation_knowledge_space_link.tenant_id must be a canonical positive signed i64');
END;

CREATE TRIGGER trg_im_group_knowledge_launch_tickets_tenant_insert
BEFORE INSERT ON im_group_knowledge_launch_tickets
WHEN NEW.tenant_id IS NULL
    OR NEW.tenant_id NOT GLOB '[1-9]*'
    OR NEW.tenant_id GLOB '*[^0-9]*'
    OR length(NEW.tenant_id) > 19
    OR (
        length(NEW.tenant_id) = 19
        AND NEW.tenant_id > '9223372036854775807'
    )
BEGIN
    SELECT RAISE(ABORT, 'im_group_knowledge_launch_tickets.tenant_id must be a canonical positive signed i64');
END;

CREATE TRIGGER trg_im_group_knowledge_launch_tickets_tenant_update
BEFORE UPDATE OF tenant_id ON im_group_knowledge_launch_tickets
WHEN NEW.tenant_id IS NULL
    OR NEW.tenant_id NOT GLOB '[1-9]*'
    OR NEW.tenant_id GLOB '*[^0-9]*'
    OR length(NEW.tenant_id) > 19
    OR (
        length(NEW.tenant_id) = 19
        AND NEW.tenant_id > '9223372036854775807'
    )
BEGIN
    SELECT RAISE(ABORT, 'im_group_knowledge_launch_tickets.tenant_id must be a canonical positive signed i64');
END;

CREATE TRIGGER trg_im_conversation_knowledge_space_link_org_insert
BEFORE INSERT ON im_conversation_knowledge_space_link
WHEN NEW.organization_id IS NULL
    OR NEW.organization_id NOT GLOB '[1-9]*'
    OR NEW.organization_id GLOB '*[^0-9]*'
    OR length(NEW.organization_id) > 19
    OR (
        length(NEW.organization_id) = 19
        AND NEW.organization_id > '9223372036854775807'
    )
BEGIN
    SELECT RAISE(ABORT, 'im_conversation_knowledge_space_link.organization_id must be a canonical positive signed i64');
END;

CREATE TRIGGER trg_im_conversation_knowledge_space_link_org_update
BEFORE UPDATE OF organization_id ON im_conversation_knowledge_space_link
WHEN NEW.organization_id IS NULL
    OR NEW.organization_id NOT GLOB '[1-9]*'
    OR NEW.organization_id GLOB '*[^0-9]*'
    OR length(NEW.organization_id) > 19
    OR (
        length(NEW.organization_id) = 19
        AND NEW.organization_id > '9223372036854775807'
    )
BEGIN
    SELECT RAISE(ABORT, 'im_conversation_knowledge_space_link.organization_id must be a canonical positive signed i64');
END;

CREATE TRIGGER trg_im_group_knowledge_launch_tickets_org_insert
BEFORE INSERT ON im_group_knowledge_launch_tickets
WHEN NEW.organization_id IS NULL
    OR NEW.organization_id NOT GLOB '[1-9]*'
    OR NEW.organization_id GLOB '*[^0-9]*'
    OR length(NEW.organization_id) > 19
    OR (
        length(NEW.organization_id) = 19
        AND NEW.organization_id > '9223372036854775807'
    )
BEGIN
    SELECT RAISE(ABORT, 'im_group_knowledge_launch_tickets.organization_id must be a canonical positive signed i64');
END;

CREATE TRIGGER trg_im_group_knowledge_launch_tickets_org_update
BEFORE UPDATE OF organization_id ON im_group_knowledge_launch_tickets
WHEN NEW.organization_id IS NULL
    OR NEW.organization_id NOT GLOB '[1-9]*'
    OR NEW.organization_id GLOB '*[^0-9]*'
    OR length(NEW.organization_id) > 19
    OR (
        length(NEW.organization_id) = 19
        AND NEW.organization_id > '9223372036854775807'
    )
BEGIN
    SELECT RAISE(ABORT, 'im_group_knowledge_launch_tickets.organization_id must be a canonical positive signed i64');
END;

COMMIT;
