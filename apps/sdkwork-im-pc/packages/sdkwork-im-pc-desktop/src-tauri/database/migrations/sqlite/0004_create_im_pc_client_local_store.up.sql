-- sdkwork:migration
-- id: 0004_create_im_pc_client_local_store
-- engine: sqlite
-- module: im-pc-client-local
-- purpose: Create the scope-bound encrypted cache and resumable pending-send queue
-- reversible: false
-- rollback: forward-fix
-- transactional: true
-- contract_version: 1.0.0

BEGIN IMMEDIATE;

CREATE TABLE im_local_installation (
    installation_key TEXT NOT NULL PRIMARY KEY
        CHECK (installation_key = 'current'),
    schema_version INTEGER NOT NULL CHECK (schema_version = 4),
    scope_fingerprint TEXT NOT NULL UNIQUE CHECK (length(scope_fingerprint) = 64),
    environment TEXT NOT NULL
        CHECK (environment IN ('development', 'test', 'staging', 'production')),
    deployment_profile TEXT NOT NULL
        CHECK (deployment_profile IN ('standalone', 'cloud')),
    deployment_mode TEXT NOT NULL
        CHECK (deployment_mode IN ('local', 'private', 'saas')),
    api_origin TEXT NOT NULL CHECK (length(api_origin) BETWEEN 8 AND 2048),
    tenant_id TEXT NOT NULL CHECK (length(tenant_id) BETWEEN 1 AND 256),
    organization_id TEXT NOT NULL CHECK (length(organization_id) BETWEEN 1 AND 256),
    account_id TEXT NOT NULL CHECK (length(account_id) BETWEEN 1 AND 256),
    principal_kind TEXT NOT NULL
        CHECK (principal_kind IN ('user', 'agent', 'system', 'service')),
    principal_id TEXT NOT NULL CHECK (length(principal_id) BETWEEN 1 AND 256),
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
) STRICT, WITHOUT ROWID;

CREATE TABLE im_local_conversation_cache (
    tenant_id TEXT NOT NULL,
    organization_id TEXT NOT NULL,
    principal_kind TEXT NOT NULL,
    principal_id TEXT NOT NULL,
    conversation_id TEXT NOT NULL,
    payload_ciphertext TEXT NOT NULL CHECK (payload_ciphertext LIKE 'enc-v1:%'),
    updated_at TEXT NOT NULL,
    cached_at_ms INTEGER NOT NULL CHECK (cached_at_ms >= 0),
    PRIMARY KEY (
        tenant_id, organization_id, principal_kind, principal_id, conversation_id
    )
) STRICT, WITHOUT ROWID;

CREATE INDEX idx_im_local_conversation_cache_scope_updated
    ON im_local_conversation_cache (
        tenant_id, organization_id, principal_kind, principal_id,
        cached_at_ms DESC, conversation_id
    );

CREATE TABLE im_local_message_cache (
    tenant_id TEXT NOT NULL,
    organization_id TEXT NOT NULL,
    principal_kind TEXT NOT NULL,
    principal_id TEXT NOT NULL,
    conversation_id TEXT NOT NULL,
    message_seq INTEGER NOT NULL CHECK (message_seq > 0),
    message_id TEXT NOT NULL,
    payload_ciphertext TEXT NOT NULL CHECK (payload_ciphertext LIKE 'enc-v1:%'),
    updated_at TEXT NOT NULL,
    cached_at_ms INTEGER NOT NULL CHECK (cached_at_ms >= 0),
    PRIMARY KEY (
        tenant_id, organization_id, principal_kind, principal_id,
        conversation_id, message_seq
    )
) STRICT, WITHOUT ROWID;

CREATE INDEX idx_im_local_message_cache_scope_conversation_seq
    ON im_local_message_cache (
        tenant_id, organization_id, principal_kind, principal_id,
        conversation_id, message_seq DESC
    );

CREATE INDEX idx_im_local_message_cache_scope_cached
    ON im_local_message_cache (
        tenant_id, organization_id, principal_kind, principal_id,
        cached_at_ms, message_seq
    );

CREATE TABLE im_local_cache_cursor (
    tenant_id TEXT NOT NULL,
    organization_id TEXT NOT NULL,
    principal_kind TEXT NOT NULL,
    principal_id TEXT NOT NULL,
    cursor_scope TEXT NOT NULL,
    cursor_ciphertext TEXT NOT NULL CHECK (cursor_ciphertext LIKE 'enc-v1:%'),
    updated_at TEXT NOT NULL,
    cached_at_ms INTEGER NOT NULL CHECK (cached_at_ms >= 0),
    PRIMARY KEY (
        tenant_id, organization_id, principal_kind, principal_id, cursor_scope
    )
) STRICT, WITHOUT ROWID;

CREATE INDEX idx_im_local_cache_cursor_scope_cached
    ON im_local_cache_cursor (
        tenant_id, organization_id, principal_kind, principal_id, cached_at_ms
    );

CREATE TABLE im_local_pending_send (
    tenant_id TEXT NOT NULL,
    organization_id TEXT NOT NULL,
    principal_kind TEXT NOT NULL,
    principal_id TEXT NOT NULL,
    client_msg_id TEXT NOT NULL,
    conversation_id TEXT NOT NULL,
    payload_ciphertext TEXT NOT NULL CHECK (payload_ciphertext LIKE 'enc-v1:%'),
    payload_hash TEXT NOT NULL CHECK (length(payload_hash) = 64),
    created_at TEXT NOT NULL,
    created_at_ms INTEGER NOT NULL CHECK (created_at_ms >= 0),
    attempt_count INTEGER NOT NULL DEFAULT 0 CHECK (attempt_count >= 0),
    flush_claim_id TEXT,
    flush_claimed_at_ms INTEGER,
    flush_claim_expires_at_ms INTEGER,
    queue_status TEXT NOT NULL DEFAULT 'pending'
        CHECK (queue_status IN ('pending', 'quarantined')),
    quarantine_reason TEXT,
    quarantined_at_ms INTEGER,
    PRIMARY KEY (
        tenant_id, organization_id, principal_kind, principal_id, client_msg_id
    ),
    CHECK (
        (flush_claim_id IS NULL AND flush_claimed_at_ms IS NULL
            AND flush_claim_expires_at_ms IS NULL)
        OR
        (flush_claim_id IS NOT NULL AND flush_claimed_at_ms IS NOT NULL
            AND flush_claim_expires_at_ms IS NOT NULL)
    ),
    CHECK (
        (queue_status = 'pending' AND quarantine_reason IS NULL
            AND quarantined_at_ms IS NULL)
        OR
        (queue_status = 'quarantined' AND quarantine_reason IS NOT NULL
            AND quarantined_at_ms IS NOT NULL AND flush_claim_id IS NULL
            AND flush_claimed_at_ms IS NULL AND flush_claim_expires_at_ms IS NULL)
    )
) STRICT, WITHOUT ROWID;

CREATE INDEX idx_im_local_pending_send_scope_created
    ON im_local_pending_send (
        tenant_id, organization_id, principal_kind, principal_id,
        created_at_ms, client_msg_id
    );

CREATE INDEX idx_im_local_pending_send_scope_claim
    ON im_local_pending_send (
        tenant_id, organization_id, principal_kind, principal_id,
        flush_claim_expires_at_ms, flush_claim_id
    );

CREATE INDEX idx_im_local_pending_send_scope_status_created
    ON im_local_pending_send (
        tenant_id, organization_id, principal_kind, principal_id,
        queue_status, created_at_ms, client_msg_id
    );

PRAGMA user_version = 4;

COMMIT;

