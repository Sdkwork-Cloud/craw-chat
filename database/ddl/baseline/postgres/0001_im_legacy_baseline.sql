-- Consolidated legacy baseline imported by bootstrap-database-module.mjs
-- Review and replace with contract-first migrations.

-- source: deployments/database/postgres/migrations/001_im_core_schema.sql
-- Core IM PostgreSQL schema.
-- This migration is intentionally append-first and cursor-indexed. It defines
-- the durable contracts that the Rust in-memory/local stores must preserve.

CREATE TABLE IF NOT EXISTS im_commit_journal (
    partition_key TEXT NOT NULL,
    commit_offset BIGINT NOT NULL CHECK (commit_offset > 0),
    event_id TEXT NOT NULL,
    tenant_id TEXT NOT NULL,
    aggregate_type TEXT NOT NULL,
    aggregate_id TEXT NOT NULL,
    aggregate_seq BIGINT NOT NULL CHECK (aggregate_seq > 0),
    event_type TEXT NOT NULL,
    payload_json JSONB NOT NULL,
    payload_hash TEXT NOT NULL,
    idempotency_key TEXT,
    occurred_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    retention_until TIMESTAMPTZ,
    CONSTRAINT pk_im_commit_journal PRIMARY KEY (partition_key, commit_offset),
    CONSTRAINT uk_im_commit_journal_event UNIQUE (event_id)
);

CREATE INDEX IF NOT EXISTS idx_im_commit_journal_tenant_aggregate_seq
    ON im_commit_journal (tenant_id, aggregate_type, aggregate_id, aggregate_seq);

CREATE INDEX IF NOT EXISTS idx_im_commit_journal_tenant_occurred
    ON im_commit_journal (tenant_id, occurred_at, event_id);

CREATE INDEX IF NOT EXISTS idx_im_commit_journal_retention_until
    ON im_commit_journal (retention_until)
    WHERE retention_until IS NOT NULL;

CREATE TABLE IF NOT EXISTS im_outbox_events (
    tenant_id TEXT NOT NULL,
    outbox_id TEXT NOT NULL,
    aggregate_type TEXT NOT NULL,
    aggregate_id TEXT NOT NULL,
    event_id TEXT NOT NULL,
    event_type TEXT NOT NULL,
    payload_json JSONB NOT NULL,
    payload_hash TEXT NOT NULL,
    publish_status TEXT NOT NULL,
    attempt_count INTEGER NOT NULL DEFAULT 0 CHECK (attempt_count >= 0),
    available_at TIMESTAMPTZ NOT NULL,
    published_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL,
    retention_until TIMESTAMPTZ,
    CONSTRAINT pk_im_outbox_events PRIMARY KEY (tenant_id, outbox_id),
    CONSTRAINT uk_im_outbox_events_event UNIQUE (tenant_id, event_id),
    CONSTRAINT chk_im_outbox_events_publish_status CHECK (publish_status IN ('pending', 'published', 'failed'))
);

CREATE INDEX IF NOT EXISTS idx_im_outbox_events_status_available
    ON im_outbox_events (tenant_id, publish_status, available_at, outbox_id);

CREATE INDEX IF NOT EXISTS idx_im_outbox_events_retention_until
    ON im_outbox_events (retention_until)
    WHERE retention_until IS NOT NULL;

CREATE TABLE IF NOT EXISTS im_inbox_events (
    tenant_id TEXT NOT NULL,
    inbox_id TEXT NOT NULL,
    source_system TEXT NOT NULL,
    source_event_id TEXT NOT NULL,
    consumer_name TEXT NOT NULL,
    event_type TEXT NOT NULL,
    payload_json JSONB NOT NULL,
    payload_hash TEXT NOT NULL,
    process_status TEXT NOT NULL,
    received_at TIMESTAMPTZ NOT NULL,
    processed_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL,
    retention_until TIMESTAMPTZ,
    CONSTRAINT pk_im_inbox_events PRIMARY KEY (tenant_id, inbox_id),
    CONSTRAINT uk_im_inbox_events_source UNIQUE (tenant_id, source_system, source_event_id),
    CONSTRAINT chk_im_inbox_events_process_status CHECK (process_status IN ('pending', 'processed', 'failed'))
);

CREATE INDEX IF NOT EXISTS idx_im_inbox_events_status_received
    ON im_inbox_events (tenant_id, consumer_name, process_status, received_at, inbox_id);

CREATE INDEX IF NOT EXISTS idx_im_inbox_events_retention_until
    ON im_inbox_events (retention_until)
    WHERE retention_until IS NOT NULL;

CREATE TABLE IF NOT EXISTS im_idempotency_keys (
    tenant_id TEXT NOT NULL,
    request_scope TEXT NOT NULL,
    idempotency_key TEXT NOT NULL,
    request_hash TEXT NOT NULL,
    response_json JSONB NOT NULL,
    first_seen_at TIMESTAMPTZ NOT NULL,
    last_seen_at TIMESTAMPTZ NOT NULL,
    expires_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL,
    CONSTRAINT pk_im_idempotency_keys PRIMARY KEY (tenant_id, request_scope, idempotency_key)
);

CREATE INDEX IF NOT EXISTS idx_im_idempotency_keys_expires
    ON im_idempotency_keys (expires_at);

DO $$
BEGIN
    IF EXISTS (
        SELECT 1
        FROM pg_constraint
        WHERE conname = 'uk_im_idempotency_keys_scope'
          AND conrelid = 'im_idempotency_keys'::regclass
    ) THEN
        ALTER TABLE im_idempotency_keys
            DROP CONSTRAINT uk_im_idempotency_keys_scope;
    END IF;
END $$;

CREATE TABLE IF NOT EXISTS im_conversation_messages (
    tenant_id TEXT NOT NULL,
    conversation_id TEXT NOT NULL,
    message_seq BIGINT NOT NULL CHECK (message_seq > 0),
    message_id TEXT NOT NULL,
    sender_principal_kind TEXT NOT NULL,
    sender_principal_id TEXT NOT NULL,
    sender_device_id TEXT,
    client_msg_id TEXT,
    message_type TEXT NOT NULL,
    payload_json JSONB NOT NULL,
    payload_hash TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL,
    deleted_at TIMESTAMPTZ,
    retention_until TIMESTAMPTZ,
    CONSTRAINT pk_im_conversation_messages PRIMARY KEY (tenant_id, conversation_id, message_seq),
    CONSTRAINT uk_im_conversation_messages_message UNIQUE (tenant_id, message_id)
);

CREATE UNIQUE INDEX IF NOT EXISTS uk_im_conversation_messages_client
    ON im_conversation_messages (tenant_id, conversation_id, sender_principal_kind, sender_principal_id, client_msg_id)
    WHERE client_msg_id IS NOT NULL;

CREATE INDEX IF NOT EXISTS idx_im_messages_tenant_conversation_seq
    ON im_conversation_messages (tenant_id, conversation_id, message_seq DESC);

CREATE INDEX IF NOT EXISTS idx_im_messages_sender_created
    ON im_conversation_messages (tenant_id, sender_principal_kind, sender_principal_id, created_at DESC);

CREATE INDEX IF NOT EXISTS idx_im_conversation_messages_retention_until
    ON im_conversation_messages (retention_until)
    WHERE retention_until IS NOT NULL;

CREATE TABLE IF NOT EXISTS im_message_media_refs (
    tenant_id TEXT NOT NULL,
    conversation_id TEXT NOT NULL,
    message_seq BIGINT NOT NULL CHECK (message_seq > 0),
    message_id TEXT NOT NULL,
    part_index INTEGER NOT NULL CHECK (part_index >= 0),
    media_role TEXT NOT NULL,
    drive_space_id TEXT NOT NULL,
    drive_node_id TEXT NOT NULL,
    drive_uri TEXT NOT NULL,
    drive_node_version TEXT,
    media_kind TEXT NOT NULL,
    media_source TEXT NOT NULL,
    mime_type TEXT,
    size_bytes TEXT,
    checksum_algorithm TEXT,
    checksum_value TEXT,
    object_blob_id TEXT,
    media_resource_snapshot JSONB NOT NULL,
    resource_hash TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL,
    retention_until TIMESTAMPTZ,
    CONSTRAINT pk_im_message_media_refs PRIMARY KEY (tenant_id, conversation_id, message_seq, part_index),
    CONSTRAINT uk_im_message_media_refs_message_part UNIQUE (tenant_id, message_id, part_index),
    CONSTRAINT fk_im_message_media_refs_message FOREIGN KEY (tenant_id, conversation_id, message_seq)
        REFERENCES im_conversation_messages (tenant_id, conversation_id, message_seq)
        ON DELETE CASCADE,
    CONSTRAINT chk_im_message_media_refs_drive_uri CHECK (
        drive_uri = ('drive://spaces/' || drive_space_id || '/nodes/' || drive_node_id)
    ),
    CONSTRAINT chk_im_message_media_refs_media_source CHECK (
        media_source IN ('drive', 'external_url', 'data_url', 'provider_asset', 'generated')
    ),
    CONSTRAINT chk_im_message_media_refs_size_bytes CHECK (
        size_bytes IS NULL OR size_bytes ~ '^[0-9]+$'
    )
);

CREATE INDEX IF NOT EXISTS idx_im_message_media_refs_drive_node
    ON im_message_media_refs (tenant_id, drive_space_id, drive_node_id, message_seq DESC);

CREATE INDEX IF NOT EXISTS idx_im_message_media_refs_role
    ON im_message_media_refs (tenant_id, conversation_id, media_role, message_seq DESC, part_index);

CREATE INDEX IF NOT EXISTS idx_im_message_media_refs_retention_until
    ON im_message_media_refs (retention_until)
    WHERE retention_until IS NOT NULL;

-- Drop legacy realtime tables before recreate so bootstrap survives pre-route-scope schemas.
DROP TABLE IF EXISTS im_realtime_subscription_scopes CASCADE;
DROP TABLE IF EXISTS im_realtime_device_events CASCADE;
DROP TABLE IF EXISTS im_realtime_subscriptions CASCADE;
DROP TABLE IF EXISTS im_realtime_checkpoints CASCADE;

CREATE TABLE IF NOT EXISTS im_realtime_device_events (
    tenant_id TEXT NOT NULL,
    client_route_scope_key TEXT NOT NULL,
    realtime_seq BIGINT NOT NULL CHECK (realtime_seq > 0),
    principal_kind TEXT NOT NULL,
    principal_id TEXT NOT NULL,
    device_id TEXT NOT NULL,
    scope_type TEXT NOT NULL,
    scope_id TEXT NOT NULL,
    event_type TEXT NOT NULL,
    delivery_class TEXT NOT NULL,
    payload_json JSONB NOT NULL,
    payload_hash TEXT NOT NULL,
    occurred_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    retention_until TIMESTAMPTZ,
    CONSTRAINT pk_im_realtime_device_events PRIMARY KEY (tenant_id, client_route_scope_key, realtime_seq)
);

CREATE INDEX IF NOT EXISTS idx_im_realtime_device_events_scope_seq
    ON im_realtime_device_events (tenant_id, client_route_scope_key, realtime_seq);

CREATE INDEX IF NOT EXISTS idx_im_realtime_device_events_scope_fanout
    ON im_realtime_device_events (tenant_id, scope_type, scope_id, event_type, realtime_seq);

CREATE INDEX IF NOT EXISTS idx_im_realtime_device_events_retention_until
    ON im_realtime_device_events (retention_until)
    WHERE retention_until IS NOT NULL;

CREATE TABLE IF NOT EXISTS im_realtime_checkpoints (
    tenant_id TEXT NOT NULL,
    client_route_scope_key TEXT NOT NULL,
    principal_kind TEXT NOT NULL,
    principal_id TEXT NOT NULL,
    device_id TEXT NOT NULL,
    latest_realtime_seq BIGINT NOT NULL DEFAULT 0 CHECK (latest_realtime_seq >= 0),
    acked_through_seq BIGINT NOT NULL DEFAULT 0 CHECK (acked_through_seq >= 0),
    trimmed_through_seq BIGINT NOT NULL DEFAULT 0 CHECK (trimmed_through_seq >= 0),
    capacity_trimmed_event_count BIGINT NOT NULL DEFAULT 0 CHECK (capacity_trimmed_event_count >= 0),
    capacity_trimmed_through_seq BIGINT NOT NULL DEFAULT 0 CHECK (capacity_trimmed_through_seq >= 0),
    last_capacity_trimmed_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL,
    CONSTRAINT pk_im_realtime_checkpoints PRIMARY KEY (tenant_id, client_route_scope_key),
    CONSTRAINT chk_im_realtime_checkpoints_order CHECK (
        acked_through_seq <= latest_realtime_seq
        AND trimmed_through_seq <= latest_realtime_seq
        AND capacity_trimmed_through_seq <= trimmed_through_seq
    ),
    CONSTRAINT chk_im_realtime_checkpoints_capacity_trim_meta CHECK (
        (
            capacity_trimmed_event_count = 0
            AND capacity_trimmed_through_seq = 0
            AND last_capacity_trimmed_at IS NULL
        )
        OR (
            capacity_trimmed_event_count > 0
            AND capacity_trimmed_through_seq > 0
            AND last_capacity_trimmed_at IS NOT NULL
        )
    )
);

CREATE INDEX IF NOT EXISTS idx_im_realtime_checkpoints_capacity_trimmed
    ON im_realtime_checkpoints (
        tenant_id,
        last_capacity_trimmed_at DESC,
        capacity_trimmed_through_seq DESC,
        client_route_scope_key
    )
    WHERE capacity_trimmed_event_count > 0;

DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1
        FROM pg_constraint
        WHERE conname = 'fk_im_realtime_device_events_checkpoint'
          AND conrelid = 'im_realtime_device_events'::regclass
    ) THEN
        ALTER TABLE im_realtime_device_events
            ADD CONSTRAINT fk_im_realtime_device_events_checkpoint
            FOREIGN KEY (tenant_id, client_route_scope_key)
            REFERENCES im_realtime_checkpoints (tenant_id, client_route_scope_key)
            ON DELETE CASCADE
            DEFERRABLE INITIALLY DEFERRED
            NOT VALID;
    END IF;
END $$;

CREATE TABLE IF NOT EXISTS im_realtime_subscriptions (
    tenant_id TEXT NOT NULL,
    client_route_scope_key TEXT NOT NULL,
    principal_kind TEXT NOT NULL,
    principal_id TEXT NOT NULL,
    device_id TEXT NOT NULL,
    subscriptions_json JSONB NOT NULL,
    subscription_count INTEGER NOT NULL DEFAULT 0 CHECK (subscription_count >= 0),
    synced_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL,
    retention_until TIMESTAMPTZ,
    CONSTRAINT pk_im_realtime_subscriptions PRIMARY KEY (tenant_id, client_route_scope_key)
);

CREATE INDEX IF NOT EXISTS idx_im_realtime_subscriptions_principal
    ON im_realtime_subscriptions (tenant_id, principal_kind, principal_id, device_id);

CREATE INDEX IF NOT EXISTS idx_im_realtime_subscriptions_synced_at
    ON im_realtime_subscriptions (tenant_id, client_route_scope_key, synced_at);

CREATE INDEX IF NOT EXISTS idx_im_realtime_subscriptions_items_gin
    ON im_realtime_subscriptions USING GIN (subscriptions_json);

CREATE INDEX IF NOT EXISTS idx_im_realtime_subscriptions_retention_until
    ON im_realtime_subscriptions (retention_until)
    WHERE retention_until IS NOT NULL;

CREATE TABLE IF NOT EXISTS im_realtime_subscription_scopes (
    tenant_id TEXT NOT NULL,
    principal_kind TEXT NOT NULL,
    principal_id TEXT NOT NULL,
    scope_type TEXT NOT NULL,
    scope_id TEXT NOT NULL,
    event_type TEXT NOT NULL DEFAULT '*',
    client_route_scope_key TEXT NOT NULL,
    device_id TEXT NOT NULL,
    synced_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL,
    CONSTRAINT pk_im_realtime_subscription_scopes PRIMARY KEY (
        tenant_id,
        principal_kind,
        principal_id,
        scope_type,
        scope_id,
        event_type,
        client_route_scope_key
    ),
    CONSTRAINT fk_im_realtime_subscription_scopes_device FOREIGN KEY (tenant_id, client_route_scope_key)
        REFERENCES im_realtime_subscriptions (tenant_id, client_route_scope_key)
        ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_im_realtime_subscription_scopes_fanout
    ON im_realtime_subscription_scopes (
        tenant_id,
        principal_kind,
        principal_id,
        scope_type,
        scope_id,
        event_type,
        device_id
    );

CREATE INDEX IF NOT EXISTS idx_im_realtime_subscription_scopes_device
    ON im_realtime_subscription_scopes (tenant_id, client_route_scope_key, synced_at);

CREATE TABLE IF NOT EXISTS im_presence_states (
    tenant_id TEXT NOT NULL,
    principal_kind TEXT NOT NULL,
    principal_id TEXT NOT NULL,
    device_id TEXT NOT NULL,
    session_id TEXT,
    presence_status TEXT NOT NULL,
    last_sync_seq BIGINT NOT NULL DEFAULT 0 CHECK (last_sync_seq >= 0),
    last_resume_at TIMESTAMPTZ,
    last_seen_at TIMESTAMPTZ,
    resume_required BOOLEAN NOT NULL DEFAULT FALSE,
    payload_json JSONB NOT NULL,
    payload_hash TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL,
    retention_until TIMESTAMPTZ,
    CONSTRAINT pk_im_presence_states PRIMARY KEY (tenant_id, principal_kind, principal_id, device_id),
    CONSTRAINT chk_im_presence_states_status CHECK (presence_status IN ('online', 'offline'))
);

CREATE INDEX IF NOT EXISTS idx_im_presence_states_principal
    ON im_presence_states (tenant_id, principal_kind, principal_id, device_id);

CREATE INDEX IF NOT EXISTS idx_im_presence_states_online_seen_at
    ON im_presence_states (
        last_seen_at,
        tenant_id,
        principal_kind,
        principal_id,
        device_id
    )
    WHERE presence_status = 'online' AND last_seen_at IS NOT NULL;

CREATE INDEX IF NOT EXISTS idx_im_presence_states_retention_until
    ON im_presence_states (retention_until)
    WHERE retention_until IS NOT NULL;

CREATE TABLE IF NOT EXISTS im_route_bindings (
    tenant_id TEXT NOT NULL,
    principal_kind TEXT NOT NULL,
    principal_id TEXT NOT NULL,
    device_id TEXT NOT NULL,
    owner_node_id TEXT NOT NULL,
    session_id TEXT,
    connection_kind TEXT NOT NULL,
    route_epoch BIGINT NOT NULL CHECK (route_epoch > 0),
    bound_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL,
    CONSTRAINT pk_im_route_bindings PRIMARY KEY (tenant_id, principal_kind, principal_id, device_id),
    CONSTRAINT chk_im_route_bindings_connection_kind CHECK (connection_kind IN (
        'websocket',
        'http',
        'ccp/ws/1',
        'ccp/tcp/1',
        'ccp/udp/1',
        'ccp/quic/1',
        'ccp/sse/1',
        'ccp/mqtt/1'
    ))
);

CREATE INDEX IF NOT EXISTS idx_im_route_bindings_owner_node
    ON im_route_bindings (owner_node_id, tenant_id, principal_kind, principal_id, device_id);

CREATE TABLE IF NOT EXISTS im_realtime_disconnect_fences (
    tenant_id TEXT NOT NULL,
    principal_kind TEXT NOT NULL,
    principal_id TEXT NOT NULL,
    device_id TEXT NOT NULL,
    session_id TEXT,
    owner_node_id TEXT NOT NULL,
    disconnected_at TIMESTAMPTZ NOT NULL,
    fence_token TEXT NOT NULL,
    payload_json JSONB NOT NULL,
    payload_hash TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL,
    retention_until TIMESTAMPTZ,
    CONSTRAINT pk_im_realtime_disconnect_fences PRIMARY KEY (tenant_id, principal_kind, principal_id, device_id),
    CONSTRAINT uk_im_realtime_disconnect_fences_token UNIQUE (tenant_id, fence_token)
);

CREATE INDEX IF NOT EXISTS idx_im_realtime_disconnect_fences_disconnected_at
    ON im_realtime_disconnect_fences (tenant_id, disconnected_at, principal_kind, principal_id, device_id);

CREATE INDEX IF NOT EXISTS idx_im_realtime_disconnect_fences_retention_until
    ON im_realtime_disconnect_fences (retention_until)
    WHERE retention_until IS NOT NULL;

CREATE TABLE IF NOT EXISTS im_rtc_sessions (
    tenant_id TEXT NOT NULL,
    rtc_session_id TEXT NOT NULL,
    conversation_id TEXT,
    rtc_mode TEXT NOT NULL,
    initiator_principal_kind TEXT NOT NULL,
    initiator_principal_id TEXT NOT NULL,
    provider_plugin_id TEXT,
    provider_session_id TEXT,
    provider_region TEXT,
    access_endpoint TEXT,
    session_state TEXT NOT NULL,
    latest_signal_seq BIGINT NOT NULL DEFAULT 0 CHECK (latest_signal_seq >= 0),
    signaling_stream_id TEXT,
    artifact_message_id TEXT,
    started_at TIMESTAMPTZ NOT NULL,
    ended_at TIMESTAMPTZ,
    payload_json JSONB NOT NULL,
    payload_hash TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL,
    retention_until TIMESTAMPTZ,
    CONSTRAINT pk_im_rtc_sessions PRIMARY KEY (tenant_id, rtc_session_id),
    CONSTRAINT chk_im_rtc_sessions_state CHECK (session_state IN ('started', 'accepted', 'rejected', 'ended'))
);

CREATE INDEX IF NOT EXISTS idx_im_rtc_sessions_conversation
    ON im_rtc_sessions (tenant_id, conversation_id, updated_at DESC)
    WHERE conversation_id IS NOT NULL;

CREATE INDEX IF NOT EXISTS idx_im_rtc_sessions_state
    ON im_rtc_sessions (tenant_id, session_state, updated_at DESC, rtc_session_id);

CREATE INDEX IF NOT EXISTS idx_im_rtc_sessions_provider_session
    ON im_rtc_sessions (tenant_id, provider_plugin_id, provider_session_id)
    WHERE provider_plugin_id IS NOT NULL AND provider_session_id IS NOT NULL;

CREATE INDEX IF NOT EXISTS idx_im_rtc_sessions_retention_until
    ON im_rtc_sessions (retention_until)
    WHERE retention_until IS NOT NULL;

CREATE TABLE IF NOT EXISTS im_rtc_signals (
    tenant_id TEXT NOT NULL,
    rtc_session_id TEXT NOT NULL,
    signal_seq BIGINT NOT NULL CHECK (signal_seq > 0),
    sender_principal_kind TEXT NOT NULL,
    sender_principal_id TEXT NOT NULL,
    receiver_principal_kind TEXT,
    receiver_principal_id TEXT,
    signal_type TEXT NOT NULL,
    payload_json JSONB NOT NULL,
    payload_hash TEXT NOT NULL,
    occurred_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    retention_until TIMESTAMPTZ,
    CONSTRAINT pk_im_rtc_signals PRIMARY KEY (tenant_id, rtc_session_id, signal_seq)
);

CREATE INDEX IF NOT EXISTS idx_im_rtc_signals_session_seq
    ON im_rtc_signals (tenant_id, rtc_session_id, signal_seq);

CREATE INDEX IF NOT EXISTS idx_im_rtc_signals_retention_until
    ON im_rtc_signals (retention_until)
    WHERE retention_until IS NOT NULL;

CREATE TABLE IF NOT EXISTS im_audit_records (
    tenant_id TEXT NOT NULL,
    organization_id TEXT NOT NULL DEFAULT '0',
    audit_seq BIGINT NOT NULL CHECK (audit_seq > 0),
    record_id TEXT NOT NULL,
    aggregate_type TEXT NOT NULL,
    aggregate_id TEXT NOT NULL,
    action TEXT NOT NULL,
    actor_id TEXT NOT NULL,
    actor_kind TEXT NOT NULL,
    actor_session_id TEXT,
    payload TEXT,
    recorded_at TEXT NOT NULL,
    chain_prev_hash TEXT,
    chain_hash TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    retention_until TIMESTAMPTZ,
    CONSTRAINT pk_im_audit_records PRIMARY KEY (tenant_id, organization_id, audit_seq),
    CONSTRAINT uk_im_audit_records_record_id UNIQUE (tenant_id, organization_id, record_id)
);

CREATE INDEX IF NOT EXISTS idx_im_audit_records_tenant_seq
    ON im_audit_records (tenant_id, organization_id, audit_seq);

CREATE INDEX IF NOT EXISTS idx_im_audit_records_aggregate
    ON im_audit_records (tenant_id, organization_id, aggregate_type, aggregate_id, audit_seq);

CREATE INDEX IF NOT EXISTS idx_im_audit_records_retention_until
    ON im_audit_records (retention_until)
    WHERE retention_until IS NOT NULL;

CREATE TABLE IF NOT EXISTS im_notification_tasks (
    tenant_id TEXT NOT NULL,
    notification_id TEXT NOT NULL,
    source_event_id TEXT NOT NULL,
    source_event_type TEXT NOT NULL,
    category TEXT NOT NULL,
    channel TEXT NOT NULL,
    recipient_kind TEXT NOT NULL,
    recipient_id TEXT NOT NULL,
    notification_status TEXT NOT NULL,
    title TEXT,
    body TEXT,
    payload_json JSONB NOT NULL,
    payload_hash TEXT NOT NULL,
    requested_at TIMESTAMPTZ NOT NULL,
    dispatched_at TIMESTAMPTZ,
    failure_reason TEXT,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL,
    retention_until TIMESTAMPTZ,
    CONSTRAINT pk_im_notification_tasks PRIMARY KEY (tenant_id, notification_id),
    CONSTRAINT uk_im_notification_tasks_source UNIQUE (tenant_id, source_event_id, recipient_kind, recipient_id, category, channel),
    CONSTRAINT chk_im_notification_tasks_status CHECK (notification_status IN ('requested', 'dispatched', 'failed'))
);

CREATE INDEX IF NOT EXISTS idx_im_notification_tasks_recipient_updated
    ON im_notification_tasks (tenant_id, recipient_kind, recipient_id, updated_at DESC, notification_id);

CREATE INDEX IF NOT EXISTS idx_im_notification_tasks_status
    ON im_notification_tasks (tenant_id, notification_status, updated_at DESC);

CREATE INDEX IF NOT EXISTS idx_im_notification_tasks_retention_until
    ON im_notification_tasks (retention_until)
    WHERE retention_until IS NOT NULL;

CREATE TABLE IF NOT EXISTS im_automation_executions (
    tenant_id TEXT NOT NULL,
    principal_kind TEXT NOT NULL,
    principal_id TEXT NOT NULL,
    execution_id TEXT NOT NULL,
    trigger_type TEXT NOT NULL,
    target_kind TEXT NOT NULL,
    target_ref TEXT NOT NULL,
    request_hash TEXT NOT NULL,
    input_payload_json JSONB,
    input_payload_hash TEXT,
    output_payload_json JSONB,
    output_payload_hash TEXT,
    execution_state TEXT NOT NULL,
    retry_count INTEGER NOT NULL DEFAULT 0 CHECK (retry_count >= 0),
    requested_at TIMESTAMPTZ NOT NULL,
    completed_at TIMESTAMPTZ,
    failure_reason TEXT,
    payload_json JSONB NOT NULL,
    payload_hash TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL,
    retention_until TIMESTAMPTZ,
    CONSTRAINT pk_im_automation_executions PRIMARY KEY (tenant_id, principal_kind, principal_id, execution_id),
    CONSTRAINT uk_im_automation_executions_request UNIQUE (tenant_id, principal_kind, principal_id, execution_id, request_hash),
    CONSTRAINT chk_im_automation_executions_state CHECK (execution_state IN ('requested', 'running', 'succeeded', 'failed'))
);

CREATE INDEX IF NOT EXISTS idx_im_automation_executions_principal_updated
    ON im_automation_executions (tenant_id, principal_kind, principal_id, updated_at DESC, execution_id);

CREATE INDEX IF NOT EXISTS idx_im_automation_executions_state
    ON im_automation_executions (tenant_id, execution_state, updated_at DESC);

CREATE INDEX IF NOT EXISTS idx_im_automation_executions_retention_until
    ON im_automation_executions (retention_until)
    WHERE retention_until IS NOT NULL;

CREATE TABLE IF NOT EXISTS im_projection_timeline_entries (
    tenant_id TEXT NOT NULL,
    conversation_id TEXT NOT NULL,
    message_seq BIGINT NOT NULL CHECK (message_seq > 0),
    message_id TEXT NOT NULL,
    summary TEXT,
    payload_json JSONB NOT NULL,
    payload_hash TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL,
    retention_until TIMESTAMPTZ,
    CONSTRAINT pk_im_projection_timeline_entries PRIMARY KEY (tenant_id, conversation_id, message_seq),
    CONSTRAINT uk_im_projection_timeline_entries_message UNIQUE (tenant_id, message_id)
);

CREATE INDEX IF NOT EXISTS idx_im_projection_timeline_entries_message
    ON im_projection_timeline_entries (tenant_id, message_id);

CREATE INDEX IF NOT EXISTS idx_im_projection_timeline_entries_retention_until
    ON im_projection_timeline_entries (retention_until)
    WHERE retention_until IS NOT NULL;

CREATE TABLE IF NOT EXISTS im_projection_conversation_summaries (
    tenant_id TEXT NOT NULL,
    conversation_id TEXT NOT NULL,
    conversation_type TEXT,
    message_count BIGINT NOT NULL DEFAULT 0 CHECK (message_count >= 0),
    last_message_id TEXT,
    last_message_seq BIGINT NOT NULL DEFAULT 0 CHECK (last_message_seq >= 0),
    last_sender_kind TEXT,
    last_sender_id TEXT,
    last_summary TEXT,
    last_message_at TIMESTAMPTZ,
    last_activity_at TIMESTAMPTZ NOT NULL,
    agent_handoff_json JSONB,
    payload_json JSONB NOT NULL,
    payload_hash TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL,
    retention_until TIMESTAMPTZ,
    CONSTRAINT pk_im_projection_conversation_summaries PRIMARY KEY (tenant_id, conversation_id)
);

CREATE INDEX IF NOT EXISTS idx_im_projection_conversation_summaries_activity
    ON im_projection_conversation_summaries (tenant_id, last_activity_at DESC, conversation_id);

CREATE INDEX IF NOT EXISTS idx_im_projection_conversation_summaries_retention_until
    ON im_projection_conversation_summaries (retention_until)
    WHERE retention_until IS NOT NULL;

CREATE TABLE IF NOT EXISTS im_projection_conversation_members (
    tenant_id TEXT NOT NULL,
    conversation_id TEXT NOT NULL,
    principal_kind TEXT NOT NULL,
    principal_id TEXT NOT NULL,
    member_id TEXT NOT NULL,
    membership_role TEXT NOT NULL,
    membership_state TEXT NOT NULL,
    invited_by TEXT,
    joined_at TIMESTAMPTZ NOT NULL,
    removed_at TIMESTAMPTZ,
    attributes_json JSONB NOT NULL DEFAULT '{}'::JSONB,
    payload_json JSONB NOT NULL,
    payload_hash TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL,
    retention_until TIMESTAMPTZ,
    CONSTRAINT pk_im_projection_conversation_members PRIMARY KEY (tenant_id, conversation_id, principal_kind, principal_id),
    CONSTRAINT uk_im_projection_conversation_members_member UNIQUE (tenant_id, conversation_id, member_id),
    CONSTRAINT chk_im_projection_conversation_members_state CHECK (membership_state IN ('invited', 'joined', 'linked', 'removed', 'left'))
);

DO $$
BEGIN
    IF EXISTS (
        SELECT 1
        FROM pg_constraint
        WHERE conname = 'chk_im_projection_conversation_members_state'
          AND conrelid = 'im_projection_conversation_members'::regclass
    ) THEN
        ALTER TABLE im_projection_conversation_members
            DROP CONSTRAINT chk_im_projection_conversation_members_state;
    END IF;

    ALTER TABLE im_projection_conversation_members
        ADD CONSTRAINT chk_im_projection_conversation_members_state
        CHECK (membership_state IN ('invited', 'joined', 'linked', 'removed', 'left'));
END $$;

CREATE INDEX IF NOT EXISTS idx_im_projection_conversation_members_principal
    ON im_projection_conversation_members (tenant_id, principal_kind, principal_id, membership_state, conversation_id);

CREATE INDEX IF NOT EXISTS idx_im_projection_conversation_members_active
    ON im_projection_conversation_members (tenant_id, conversation_id, principal_kind, principal_id)
    WHERE membership_state = 'joined';

CREATE INDEX IF NOT EXISTS idx_im_projection_conversation_members_retention_until
    ON im_projection_conversation_members (retention_until)
    WHERE retention_until IS NOT NULL;

CREATE TABLE IF NOT EXISTS im_projection_read_cursors (
    tenant_id TEXT NOT NULL,
    conversation_id TEXT NOT NULL,
    member_id TEXT NOT NULL,
    principal_kind TEXT NOT NULL,
    principal_id TEXT NOT NULL,
    read_seq BIGINT NOT NULL DEFAULT 0 CHECK (read_seq >= 0),
    last_read_message_id TEXT,
    payload_json JSONB NOT NULL,
    payload_hash TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL,
    retention_until TIMESTAMPTZ,
    CONSTRAINT pk_im_projection_read_cursors PRIMARY KEY (tenant_id, conversation_id, member_id)
);

CREATE INDEX IF NOT EXISTS idx_im_projection_read_cursors_principal
    ON im_projection_read_cursors (tenant_id, principal_kind, principal_id, conversation_id);

CREATE INDEX IF NOT EXISTS idx_im_projection_read_cursors_retention_until
    ON im_projection_read_cursors (retention_until)
    WHERE retention_until IS NOT NULL;

CREATE TABLE IF NOT EXISTS im_projection_registered_client_routes (
    tenant_id TEXT NOT NULL,
    principal_kind TEXT NOT NULL,
    principal_id TEXT NOT NULL,
    device_id TEXT NOT NULL,
    registered_at TIMESTAMPTZ NOT NULL,
    payload_json JSONB NOT NULL,
    payload_hash TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL,
    retention_until TIMESTAMPTZ,
    CONSTRAINT pk_im_projection_registered_client_routes PRIMARY KEY (tenant_id, principal_kind, principal_id, device_id)
);

CREATE INDEX IF NOT EXISTS idx_im_projection_registered_client_routes_retention_until
    ON im_projection_registered_client_routes (retention_until)
    WHERE retention_until IS NOT NULL;

CREATE TABLE IF NOT EXISTS im_projection_client_route_sync_feeds (
    tenant_id TEXT NOT NULL,
    principal_kind TEXT NOT NULL,
    principal_id TEXT NOT NULL,
    device_id TEXT NOT NULL,
    sync_seq BIGINT NOT NULL CHECK (sync_seq > 0),
    origin_event_id TEXT NOT NULL,
    origin_event_type TEXT NOT NULL,
    conversation_id TEXT,
    message_id TEXT,
    message_seq BIGINT CHECK (message_seq IS NULL OR message_seq > 0),
    member_id TEXT,
    read_seq BIGINT CHECK (read_seq IS NULL OR read_seq >= 0),
    last_read_message_id TEXT,
    actor_kind TEXT,
    actor_id TEXT,
    actor_device_id TEXT,
    summary TEXT,
    payload_json JSONB NOT NULL,
    payload_hash TEXT NOT NULL,
    occurred_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    retention_until TIMESTAMPTZ,
    CONSTRAINT pk_im_projection_client_route_sync_feeds PRIMARY KEY (tenant_id, principal_kind, principal_id, device_id, sync_seq)
);

CREATE INDEX IF NOT EXISTS idx_im_projection_client_route_sync_feeds_window
    ON im_projection_client_route_sync_feeds (tenant_id, principal_kind, principal_id, device_id, sync_seq);

CREATE INDEX IF NOT EXISTS idx_im_projection_client_route_sync_feeds_conversation
    ON im_projection_client_route_sync_feeds (tenant_id, conversation_id, sync_seq)
    WHERE conversation_id IS NOT NULL;

CREATE INDEX IF NOT EXISTS idx_im_projection_client_route_sync_feeds_retention_until
    ON im_projection_client_route_sync_feeds (retention_until)
    WHERE retention_until IS NOT NULL;

CREATE TABLE IF NOT EXISTS im_projection_client_route_sync_checkpoints (
    tenant_id TEXT NOT NULL,
    principal_kind TEXT NOT NULL,
    principal_id TEXT NOT NULL,
    device_id TEXT NOT NULL,
    latest_sync_seq BIGINT NOT NULL DEFAULT 0 CHECK (latest_sync_seq >= 0),
    trimmed_through_seq BIGINT NOT NULL DEFAULT 0 CHECK (trimmed_through_seq >= 0),
    payload_json JSONB NOT NULL,
    payload_hash TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL,
    retention_until TIMESTAMPTZ,
    CONSTRAINT pk_im_projection_client_route_sync_checkpoints PRIMARY KEY (tenant_id, principal_kind, principal_id, device_id),
    CONSTRAINT chk_im_projection_client_route_sync_checkpoints_order CHECK (trimmed_through_seq <= latest_sync_seq)
);

CREATE INDEX IF NOT EXISTS idx_im_projection_client_route_sync_checkpoints_retention_until
    ON im_projection_client_route_sync_checkpoints (retention_until)
    WHERE retention_until IS NOT NULL;

CREATE TABLE IF NOT EXISTS im_projection_contacts (
    tenant_id TEXT NOT NULL,
    owner_user_id TEXT NOT NULL,
    contact_type TEXT NOT NULL,
    target_user_id TEXT NOT NULL,
    relationship_state TEXT NOT NULL,
    friendship_id TEXT NOT NULL,
    direct_chat_id TEXT,
    conversation_id TEXT,
    established_at TIMESTAMPTZ NOT NULL,
    last_interaction_at TIMESTAMPTZ NOT NULL,
    payload_json JSONB NOT NULL,
    payload_hash TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL,
    retention_until TIMESTAMPTZ,
    CONSTRAINT pk_im_projection_contacts PRIMARY KEY (tenant_id, owner_user_id, contact_type, target_user_id)
);

CREATE INDEX IF NOT EXISTS idx_im_projection_contacts_owner_activity
    ON im_projection_contacts (tenant_id, owner_user_id, last_interaction_at DESC, target_user_id);

CREATE INDEX IF NOT EXISTS idx_im_projection_contacts_retention_until
    ON im_projection_contacts (retention_until)
    WHERE retention_until IS NOT NULL;

CREATE TABLE IF NOT EXISTS im_projection_direct_chat_bindings (
    tenant_id TEXT NOT NULL,
    direct_chat_id TEXT NOT NULL,
    conversation_id TEXT NOT NULL,
    direct_chat_status TEXT NOT NULL,
    bound_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL,
    payload_json JSONB NOT NULL,
    payload_hash TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    retention_until TIMESTAMPTZ,
    CONSTRAINT pk_im_projection_direct_chat_bindings PRIMARY KEY (tenant_id, direct_chat_id),
    CONSTRAINT uk_im_projection_direct_chat_bindings_conversation UNIQUE (tenant_id, conversation_id),
    CONSTRAINT chk_im_projection_direct_chat_bindings_status CHECK (direct_chat_status IN ('active', 'archived'))
);

CREATE INDEX IF NOT EXISTS idx_im_projection_direct_chat_bindings_conversation
    ON im_projection_direct_chat_bindings (tenant_id, conversation_id, direct_chat_status);

CREATE INDEX IF NOT EXISTS idx_im_projection_direct_chat_bindings_retention_until
    ON im_projection_direct_chat_bindings (retention_until)
    WHERE retention_until IS NOT NULL;

CREATE TABLE IF NOT EXISTS im_stream_sessions (
    tenant_id TEXT NOT NULL,
    stream_id TEXT NOT NULL,
    owner_principal_kind TEXT NOT NULL,
    owner_principal_id TEXT NOT NULL,
    stream_type TEXT NOT NULL,
    scope_kind TEXT NOT NULL,
    scope_id TEXT NOT NULL,
    durability_class TEXT NOT NULL,
    ordering_scope TEXT NOT NULL,
    schema_ref TEXT,
    stream_state TEXT NOT NULL,
    last_frame_seq BIGINT NOT NULL DEFAULT 0 CHECK (last_frame_seq >= 0),
    last_checkpoint_seq BIGINT CHECK (last_checkpoint_seq >= 0),
    result_message_id TEXT,
    complete_frame_seq BIGINT CHECK (complete_frame_seq >= 0),
    abort_frame_seq BIGINT CHECK (abort_frame_seq >= 0),
    abort_reason TEXT,
    opened_at TIMESTAMPTZ NOT NULL,
    closed_at TIMESTAMPTZ,
    expires_at TIMESTAMPTZ,
    payload_json JSONB NOT NULL,
    payload_hash TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL,
    retention_until TIMESTAMPTZ,
    CONSTRAINT pk_im_stream_sessions PRIMARY KEY (tenant_id, stream_id),
    CONSTRAINT chk_im_stream_sessions_state CHECK (stream_state IN ('created', 'opened', 'active', 'checkpointed', 'completed', 'aborted', 'expired')),
    CONSTRAINT chk_im_stream_sessions_seq_order CHECK (
        COALESCE(last_checkpoint_seq, 0) <= last_frame_seq
        AND COALESCE(complete_frame_seq, 0) <= last_frame_seq
        AND COALESCE(abort_frame_seq, 0) <= last_frame_seq
    )
);

CREATE INDEX IF NOT EXISTS idx_im_stream_sessions_scope
    ON im_stream_sessions (tenant_id, scope_kind, scope_id, updated_at DESC);

CREATE INDEX IF NOT EXISTS idx_im_stream_sessions_updated
    ON im_stream_sessions (tenant_id, updated_at DESC, stream_id);

CREATE INDEX IF NOT EXISTS idx_im_stream_sessions_retention_until
    ON im_stream_sessions (retention_until)
    WHERE retention_until IS NOT NULL;

CREATE TABLE IF NOT EXISTS im_stream_frames (
    tenant_id TEXT NOT NULL,
    stream_id TEXT NOT NULL,
    frame_seq BIGINT NOT NULL CHECK (frame_seq > 0),
    producer_principal_kind TEXT NOT NULL,
    producer_principal_id TEXT NOT NULL,
    schema_ref TEXT,
    payload_json JSONB NOT NULL,
    payload_hash TEXT NOT NULL,
    occurred_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    retention_until TIMESTAMPTZ,
    CONSTRAINT pk_im_stream_frames PRIMARY KEY (tenant_id, stream_id, frame_seq)
);

CREATE INDEX IF NOT EXISTS idx_im_stream_frames_stream_seq
    ON im_stream_frames (tenant_id, stream_id, frame_seq);

CREATE INDEX IF NOT EXISTS idx_im_stream_frames_retention_until
    ON im_stream_frames (retention_until)
    WHERE retention_until IS NOT NULL;

-- source: deployments/database/postgres/migrations/010_im_tenant_organization_isolation.sql
-- Migration 010: Tenant + Organization Dual Isolation
-- 为所有 im_* 业务表引入 organization_id，实现租户组织双重隔离
-- 新应用零用户，直接重建终端 schema，不保留 001 迁移的兼容性

-- ============================================================
-- 核心设计决策：
-- 1. organization_id 为 TEXT NOT NULL DEFAULT '0'
-- 2. 主键与索引统一前置 (tenant_id, organization_id, ...)
-- 3. 所有查询强制携带 organization_id 过滤
-- ============================================================

-- ============================================================
-- 1. 消息真值层
-- ============================================================

-- 重建 im_conversation_messages（消息真值表）
-- 主键改为 Snowflake message_id，但保留 message_seq 作为会话内序号
DROP TABLE IF EXISTS im_conversation_messages CASCADE;
CREATE TABLE im_conversation_messages (
    tenant_id           TEXT NOT NULL,
    organization_id     TEXT NOT NULL DEFAULT '0',
    conversation_id     TEXT NOT NULL,
    message_id          BIGINT NOT NULL,           -- Snowflake ID，全局唯一
    message_seq         BIGINT NOT NULL,           -- 会话内严格递增
    sender_principal_kind TEXT NOT NULL,
    sender_principal_id TEXT NOT NULL,
    sender_device_id    TEXT,
    client_msg_id       TEXT,
    message_type        TEXT NOT NULL,
    payload_json        JSONB NOT NULL,
    payload_hash        TEXT NOT NULL,
    created_at           TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at           TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at           TIMESTAMPTZ,
    retention_until     TIMESTAMPTZ,
    CONSTRAINT pk_im_conversation_messages PRIMARY KEY (tenant_id, organization_id, conversation_id, message_seq),
    CONSTRAINT uk_im_conversation_messages_id UNIQUE (tenant_id, message_id),
    CONSTRAINT chk_im_conversation_messages_seq CHECK (message_seq > 0)
);

-- 客户端幂等键（会话 + 发送者 + client_msg_id 唯一性）
CREATE UNIQUE INDEX IF NOT EXISTS uk_im_conversation_messages_client
    ON im_conversation_messages (tenant_id, organization_id, conversation_id, sender_principal_kind, sender_principal_id, client_msg_id)
    WHERE client_msg_id IS NOT NULL;

-- timeline 读取索引
CREATE INDEX IF NOT EXISTS idx_im_messages_tenant_conv_seq
    ON im_conversation_messages (tenant_id, organization_id, conversation_id, message_seq DESC);

-- 发送者消息索引
CREATE INDEX IF NOT EXISTS idx_im_messages_sender_created
    ON im_conversation_messages (tenant_id, organization_id, sender_principal_kind, sender_principal_id, created_at DESC);

-- retention 索引
CREATE INDEX IF NOT EXISTS idx_im_conversation_messages_retention_until
    ON im_conversation_messages (tenant_id, organization_id, retention_until)
    WHERE retention_until IS NOT NULL;

-- ============================================================
-- 2. 消息序号分配器（会话级原子）
-- ============================================================

DROP TABLE IF EXISTS im_conversation_seq_counters CASCADE;
CREATE TABLE im_conversation_seq_counters (
    tenant_id       TEXT NOT NULL,
    organization_id TEXT NOT NULL DEFAULT '0',
    conversation_id TEXT NOT NULL,
    next_seq        BIGINT NOT NULL DEFAULT 1,
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT pk_im_conversation_seq_counters PRIMARY KEY (tenant_id, organization_id, conversation_id),
    CONSTRAINT chk_im_conversation_seq_counters_seq CHECK (next_seq > 0)
);

-- ============================================================
-- 3. 消息媒体引用
-- ============================================================

DROP TABLE IF EXISTS im_message_media_refs CASCADE;
CREATE TABLE im_message_media_refs (
    tenant_id TEXT NOT NULL,
    organization_id TEXT NOT NULL DEFAULT '0',
    conversation_id TEXT NOT NULL,
    message_seq BIGINT NOT NULL,
    message_id BIGINT NOT NULL,
    part_index INTEGER NOT NULL CHECK (part_index >= 0),
    media_role TEXT NOT NULL,
    drive_space_id TEXT NOT NULL,
    drive_node_id TEXT NOT NULL,
    drive_uri TEXT NOT NULL,
    drive_node_version TEXT,
    media_kind TEXT NOT NULL,
    media_source TEXT NOT NULL,
    mime_type TEXT,
    size_bytes TEXT,
    checksum_algorithm TEXT,
    checksum_value TEXT,
    object_blob_id TEXT,
    media_resource_snapshot JSONB NOT NULL,
    resource_hash TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    retention_until TIMESTAMPTZ,
    CONSTRAINT pk_im_message_media_refs PRIMARY KEY (tenant_id, organization_id, conversation_id, message_seq, part_index),
    CONSTRAINT uk_im_message_media_refs_message_part UNIQUE (tenant_id, message_id, part_index),
    CONSTRAINT fk_im_message_media_refs_message FOREIGN KEY (tenant_id, organization_id, conversation_id, message_seq)
        REFERENCES im_conversation_messages (tenant_id, organization_id, conversation_id, message_seq)
        ON DELETE CASCADE,
    CONSTRAINT chk_im_message_media_refs_drive_uri CHECK (
        drive_uri = ('drive://spaces/' || drive_space_id || '/nodes/' || drive_node_id)
    ),
    CONSTRAINT chk_im_message_media_refs_media_source CHECK (
        media_source IN ('drive', 'external_url', 'data_url', 'provider_asset', 'generated')
    ),
    CONSTRAINT chk_im_message_media_refs_size_bytes CHECK (
        size_bytes IS NULL OR size_bytes ~ '^[0-9]+$'
    )
);

CREATE INDEX IF NOT EXISTS idx_im_message_media_refs_drive_node
    ON im_message_media_refs (tenant_id, organization_id, drive_space_id, drive_node_id, message_seq DESC);

CREATE INDEX IF NOT EXISTS idx_im_message_media_refs_role
    ON im_message_media_refs (tenant_id, organization_id, conversation_id, media_role, message_seq DESC, part_index);

CREATE INDEX IF NOT EXISTS idx_im_message_media_refs_retention_until
    ON im_message_media_refs (tenant_id, organization_id, retention_until)
    WHERE retention_until IS NOT NULL;

-- ============================================================
-- 4. Outbox 事件表（重建，支持 FOR UPDATE SKIP LOCKED）
-- ============================================================

DROP TABLE IF EXISTS im_outbox_events CASCADE;
CREATE TABLE im_outbox_events (
    tenant_id TEXT NOT NULL,
    organization_id TEXT NOT NULL DEFAULT '0',
    outbox_id TEXT NOT NULL,              -- Snowflake ID
    aggregate_type TEXT NOT NULL,
    aggregate_id TEXT NOT NULL,
    event_id TEXT NOT NULL,
    event_type TEXT NOT NULL,
    payload_json JSONB NOT NULL,
    payload_hash TEXT NOT NULL,
    publish_status TEXT NOT NULL DEFAULT 'pending',
    attempt_count INTEGER NOT NULL DEFAULT 0 CHECK (attempt_count >= 0),
    available_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    published_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    retention_until TIMESTAMPTZ,
    CONSTRAINT pk_im_outbox_events PRIMARY KEY (tenant_id, organization_id, outbox_id),
    CONSTRAINT uk_im_outbox_events_event UNIQUE (tenant_id, organization_id, event_id),
    CONSTRAINT chk_im_outbox_events_publish_status CHECK (publish_status IN ('pending', 'published', 'failed'))
);

-- relay worker 用索引：FOR UPDATE SKIP LOCKED
CREATE INDEX IF NOT EXISTS idx_im_outbox_events_status_available
    ON im_outbox_events (tenant_id, organization_id, publish_status, available_at, outbox_id);

CREATE INDEX IF NOT EXISTS idx_im_outbox_events_retention_until
    ON im_outbox_events (tenant_id, organization_id, retention_until)
    WHERE retention_until IS NOT NULL;

-- ============================================================
-- 5. Inbox 事件表（消费幂等性）
-- ============================================================

DROP TABLE IF EXISTS im_inbox_events CASCADE;
CREATE TABLE im_inbox_events (
    tenant_id TEXT NOT NULL,
    organization_id TEXT NOT NULL DEFAULT '0',
    inbox_id TEXT NOT NULL,
    source_system TEXT NOT NULL,
    source_event_id TEXT NOT NULL,
    consumer_name TEXT NOT NULL,
    event_type TEXT NOT NULL,
    payload_json JSONB NOT NULL,
    payload_hash TEXT NOT NULL,
    process_status TEXT NOT NULL DEFAULT 'pending',
    received_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    processed_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    retention_until TIMESTAMPTZ,
    CONSTRAINT pk_im_inbox_events PRIMARY KEY (tenant_id, organization_id, inbox_id),
    CONSTRAINT uk_im_inbox_events_source UNIQUE (tenant_id, organization_id, source_system, source_event_id),
    CONSTRAINT chk_im_inbox_events_process_status CHECK (process_status IN ('pending', 'processed', 'failed'))
);

CREATE INDEX IF NOT EXISTS idx_im_inbox_events_status_received
    ON im_inbox_events (tenant_id, organization_id, consumer_name, process_status, received_at, inbox_id);

CREATE INDEX IF NOT EXISTS idx_im_inbox_events_retention_until
    ON im_inbox_events (tenant_id, organization_id, retention_until)
    WHERE retention_until IS NOT NULL;

-- ============================================================
-- 6. Commit Journal（重建，offset 独立于 aggregate_seq）
-- ============================================================

DROP TABLE IF EXISTS im_commit_journal CASCADE;
CREATE TABLE im_commit_journal (
    partition_key TEXT NOT NULL,           -- (tenant_id:organization_id:aggregate_type:aggregate_id)
    commit_offset BIGINT NOT NULL,         -- Snowflake ID，全局唯一，非业务序号
    event_id TEXT NOT NULL,                -- Snowflake ID
    tenant_id TEXT NOT NULL,
    organization_id TEXT NOT NULL DEFAULT '0',
    aggregate_type TEXT NOT NULL,
    aggregate_id TEXT NOT NULL,
    aggregate_seq BIGINT NOT NULL CHECK (aggregate_seq > 0),  -- 业务聚合版本号
    event_type TEXT NOT NULL,
    payload_json JSONB NOT NULL,
    payload_hash TEXT NOT NULL,
    idempotency_key TEXT,
    occurred_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    retention_until TIMESTAMPTZ,
    CONSTRAINT pk_im_commit_journal PRIMARY KEY (partition_key, commit_offset),
    CONSTRAINT uk_im_commit_journal_event UNIQUE (event_id)
);

CREATE INDEX IF NOT EXISTS idx_im_commit_journal_tenant_aggregate_seq
    ON im_commit_journal (tenant_id, organization_id, aggregate_type, aggregate_id, aggregate_seq);

CREATE INDEX IF NOT EXISTS idx_im_commit_journal_tenant_occurred
    ON im_commit_journal (tenant_id, organization_id, occurred_at, event_id);

CREATE INDEX IF NOT EXISTS idx_im_commit_journal_retention_until
    ON im_commit_journal (tenant_id, organization_id, retention_until)
    WHERE retention_until IS NOT NULL;

-- ============================================================
-- 7. 幂等键表
-- ============================================================

DROP TABLE IF EXISTS im_idempotency_keys CASCADE;
CREATE TABLE im_idempotency_keys (
    tenant_id TEXT NOT NULL,
    organization_id TEXT NOT NULL DEFAULT '0',
    request_scope TEXT NOT NULL,
    idempotency_key TEXT NOT NULL,
    request_hash TEXT NOT NULL,
    response_json JSONB NOT NULL,
    first_seen_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_seen_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    expires_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT pk_im_idempotency_keys PRIMARY KEY (tenant_id, organization_id, request_scope, idempotency_key)
);

CREATE INDEX IF NOT EXISTS idx_im_idempotency_keys_expires
    ON im_idempotency_keys (tenant_id, organization_id, expires_at);

-- ============================================================
-- 8. 实时设备事件
-- ============================================================

DROP TABLE IF EXISTS im_realtime_device_events CASCADE;
CREATE TABLE im_realtime_device_events (
    tenant_id TEXT NOT NULL,
    organization_id TEXT NOT NULL DEFAULT '0',
    client_route_scope_key TEXT NOT NULL,
    realtime_seq BIGINT NOT NULL CHECK (realtime_seq > 0),
    principal_kind TEXT NOT NULL,
    principal_id TEXT NOT NULL,
    device_id TEXT NOT NULL,
    scope_type TEXT NOT NULL,
    scope_id TEXT NOT NULL,
    event_type TEXT NOT NULL,
    delivery_class TEXT NOT NULL,
    payload_json JSONB NOT NULL,
    payload_hash TEXT NOT NULL,
    occurred_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    retention_until TIMESTAMPTZ,
    CONSTRAINT pk_im_realtime_device_events PRIMARY KEY (tenant_id, organization_id, client_route_scope_key, realtime_seq)
);

CREATE INDEX IF NOT EXISTS idx_im_realtime_device_events_scope_seq
    ON im_realtime_device_events (tenant_id, organization_id, client_route_scope_key, realtime_seq);

CREATE INDEX IF NOT EXISTS idx_im_realtime_device_events_scope_fanout
    ON im_realtime_device_events (tenant_id, organization_id, scope_type, scope_id, event_type, realtime_seq);

CREATE INDEX IF NOT EXISTS idx_im_realtime_device_events_retention_until
    ON im_realtime_device_events (tenant_id, organization_id, retention_until)
    WHERE retention_until IS NOT NULL;

-- ============================================================
-- 9. 实时检查点
-- ============================================================

DROP TABLE IF EXISTS im_realtime_checkpoints CASCADE;
CREATE TABLE im_realtime_checkpoints (
    tenant_id TEXT NOT NULL,
    organization_id TEXT NOT NULL DEFAULT '0',
    client_route_scope_key TEXT NOT NULL,
    principal_kind TEXT NOT NULL,
    principal_id TEXT NOT NULL,
    device_id TEXT NOT NULL,
    latest_realtime_seq BIGINT NOT NULL DEFAULT 0 CHECK (latest_realtime_seq >= 0),
    acked_through_seq BIGINT NOT NULL DEFAULT 0 CHECK (acked_through_seq >= 0),
    trimmed_through_seq BIGINT NOT NULL DEFAULT 0 CHECK (trimmed_through_seq >= 0),
    capacity_trimmed_event_count BIGINT NOT NULL DEFAULT 0 CHECK (capacity_trimmed_event_count >= 0),
    capacity_trimmed_through_seq BIGINT NOT NULL DEFAULT 0 CHECK (capacity_trimmed_through_seq >= 0),
    last_capacity_trimmed_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT pk_im_realtime_checkpoints PRIMARY KEY (tenant_id, organization_id, client_route_scope_key),
    CONSTRAINT chk_im_realtime_checkpoints_order CHECK (
        acked_through_seq <= latest_realtime_seq
        AND trimmed_through_seq <= latest_realtime_seq
        AND capacity_trimmed_through_seq <= trimmed_through_seq
    ),
    CONSTRAINT chk_im_realtime_checkpoints_capacity_trim_meta CHECK (
        (
            capacity_trimmed_event_count = 0
            AND capacity_trimmed_through_seq = 0
            AND last_capacity_trimmed_at IS NULL
        )
        OR (
            capacity_trimmed_event_count > 0
            AND capacity_trimmed_through_seq > 0
            AND last_capacity_trimmed_at IS NOT NULL
        )
    )
);

CREATE INDEX IF NOT EXISTS idx_im_realtime_checkpoints_capacity_trimmed
    ON im_realtime_checkpoints (
        tenant_id,
        organization_id,
        last_capacity_trimmed_at DESC,
        capacity_trimmed_through_seq DESC,
        client_route_scope_key
    )
    WHERE capacity_trimmed_event_count > 0;

-- ============================================================
-- 10. 实时订阅
-- ============================================================

DROP TABLE IF EXISTS im_realtime_subscriptions CASCADE;
CREATE TABLE im_realtime_subscriptions (
    tenant_id TEXT NOT NULL,
    organization_id TEXT NOT NULL DEFAULT '0',
    client_route_scope_key TEXT NOT NULL,
    principal_kind TEXT NOT NULL,
    principal_id TEXT NOT NULL,
    device_id TEXT NOT NULL,
    subscriptions_json JSONB NOT NULL,
    subscription_count INTEGER NOT NULL DEFAULT 0 CHECK (subscription_count >= 0),
    synced_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    retention_until TIMESTAMPTZ,
    CONSTRAINT pk_im_realtime_subscriptions PRIMARY KEY (tenant_id, organization_id, client_route_scope_key)
);

CREATE INDEX IF NOT EXISTS idx_im_realtime_subscriptions_principal
    ON im_realtime_subscriptions (tenant_id, organization_id, principal_kind, principal_id, device_id);

CREATE INDEX IF NOT EXISTS idx_im_realtime_subscriptions_synced_at
    ON im_realtime_subscriptions (tenant_id, organization_id, client_route_scope_key, synced_at);

CREATE INDEX IF NOT EXISTS idx_im_realtime_subscriptions_items_gin
    ON im_realtime_subscriptions USING GIN (subscriptions_json);

CREATE INDEX IF NOT EXISTS idx_im_realtime_subscriptions_retention_until
    ON im_realtime_subscriptions (tenant_id, organization_id, retention_until)
    WHERE retention_until IS NOT NULL;

-- ============================================================
-- 11. 实时订阅范围
-- ============================================================

DROP TABLE IF EXISTS im_realtime_subscription_scopes CASCADE;
CREATE TABLE im_realtime_subscription_scopes (
    tenant_id TEXT NOT NULL,
    organization_id TEXT NOT NULL DEFAULT '0',
    principal_kind TEXT NOT NULL,
    principal_id TEXT NOT NULL,
    scope_type TEXT NOT NULL,
    scope_id TEXT NOT NULL,
    event_type TEXT NOT NULL DEFAULT '*',
    client_route_scope_key TEXT NOT NULL,
    device_id TEXT NOT NULL,
    synced_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT pk_im_realtime_subscription_scopes PRIMARY KEY (
        tenant_id,
        organization_id,
        principal_kind,
        principal_id,
        scope_type,
        scope_id,
        event_type,
        client_route_scope_key
    ),
    CONSTRAINT fk_im_realtime_subscription_scopes_device
        FOREIGN KEY (tenant_id, organization_id, client_route_scope_key)
        REFERENCES im_realtime_subscriptions (tenant_id, organization_id, client_route_scope_key)
        ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_im_realtime_subscription_scopes_fanout
    ON im_realtime_subscription_scopes (
        tenant_id,
        organization_id,
        principal_kind,
        principal_id,
        scope_type,
        scope_id,
        event_type,
        device_id
    );

CREATE INDEX IF NOT EXISTS idx_im_realtime_subscription_scopes_device
    ON im_realtime_subscription_scopes (tenant_id, organization_id, client_route_scope_key, synced_at);

-- ============================================================
-- 12. Presence 状态
-- ============================================================

DROP TABLE IF EXISTS im_presence_states CASCADE;
CREATE TABLE im_presence_states (
    tenant_id TEXT NOT NULL,
    organization_id TEXT NOT NULL DEFAULT '0',
    principal_kind TEXT NOT NULL,
    principal_id TEXT NOT NULL,
    device_id TEXT NOT NULL,
    session_id TEXT,
    presence_status TEXT NOT NULL,
    last_sync_seq BIGINT NOT NULL DEFAULT 0 CHECK (last_sync_seq >= 0),
    last_resume_at TIMESTAMPTZ,
    last_seen_at TIMESTAMPTZ,
    resume_required BOOLEAN NOT NULL DEFAULT FALSE,
    payload_json JSONB NOT NULL,
    payload_hash TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    retention_until TIMESTAMPTZ,
    CONSTRAINT pk_im_presence_states PRIMARY KEY (tenant_id, organization_id, principal_kind, principal_id, device_id),
    CONSTRAINT chk_im_presence_states_status CHECK (presence_status IN ('online', 'offline'))
);

CREATE INDEX IF NOT EXISTS idx_im_presence_states_principal
    ON im_presence_states (tenant_id, organization_id, principal_kind, principal_id, device_id);

CREATE INDEX IF NOT EXISTS idx_im_presence_states_online_seen_at
    ON im_presence_states (
        last_seen_at,
        tenant_id,
        organization_id,
        principal_kind,
        principal_id,
        device_id
    )
    WHERE presence_status = 'online' AND last_seen_at IS NOT NULL;

CREATE INDEX IF NOT EXISTS idx_im_presence_states_retention_until
    ON im_presence_states (tenant_id, organization_id, retention_until)
    WHERE retention_until IS NOT NULL;

-- ============================================================
-- 13. 路由绑定
-- ============================================================

DROP TABLE IF EXISTS im_route_bindings CASCADE;
CREATE TABLE im_route_bindings (
    tenant_id TEXT NOT NULL,
    organization_id TEXT NOT NULL DEFAULT '0',
    principal_kind TEXT NOT NULL,
    principal_id TEXT NOT NULL,
    device_id TEXT NOT NULL,
    owner_node_id TEXT NOT NULL,
    session_id TEXT,
    connection_kind TEXT NOT NULL,
    route_epoch BIGINT NOT NULL CHECK (route_epoch > 0),
    bound_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT pk_im_route_bindings PRIMARY KEY (tenant_id, organization_id, principal_kind, principal_id, device_id),
    CONSTRAINT chk_im_route_bindings_connection_kind CHECK (connection_kind IN (
        'websocket',
        'http',
        'ccp/ws/1',
        'ccp/tcp/1',
        'ccp/udp/1',
        'ccp/quic/1',
        'ccp/sse/1',
        'ccp/mqtt/1'
    ))
);

CREATE INDEX IF NOT EXISTS idx_im_route_bindings_owner_node
    ON im_route_bindings (owner_node_id, tenant_id, organization_id, principal_kind, principal_id, device_id);

-- ============================================================
-- 14. 断线围栏
-- ============================================================

DROP TABLE IF EXISTS im_realtime_disconnect_fences CASCADE;
CREATE TABLE im_realtime_disconnect_fences (
    tenant_id TEXT NOT NULL,
    organization_id TEXT NOT NULL DEFAULT '0',
    principal_kind TEXT NOT NULL,
    principal_id TEXT NOT NULL,
    device_id TEXT NOT NULL,
    session_id TEXT,
    owner_node_id TEXT NOT NULL,
    disconnected_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    fence_token TEXT NOT NULL,
    payload_json JSONB NOT NULL,
    payload_hash TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    retention_until TIMESTAMPTZ,
    CONSTRAINT pk_im_realtime_disconnect_fences PRIMARY KEY (tenant_id, organization_id, principal_kind, principal_id, device_id),
    CONSTRAINT uk_im_realtime_disconnect_fences_token UNIQUE (tenant_id, organization_id, fence_token)
);

CREATE INDEX IF NOT EXISTS idx_im_realtime_disconnect_fences_disconnected_at
    ON im_realtime_disconnect_fences (tenant_id, organization_id, disconnected_at, principal_kind, principal_id, device_id);

CREATE INDEX IF NOT EXISTS idx_im_realtime_disconnect_fences_retention_until
    ON im_realtime_disconnect_fences (tenant_id, organization_id, retention_until)
    WHERE retention_until IS NOT NULL;

-- source: deployments/database/postgres/migrations/011_im_projections_rtc_streams.sql
-- Migration 011: RTC Sessions, Signals, Audit, Notifications, Automations, Projections
-- 继续重建剩余表，引入 organization_id

-- ============================================================
-- 15. RTC 会话
-- ============================================================

DROP TABLE IF EXISTS im_rtc_sessions CASCADE;
CREATE TABLE im_rtc_sessions (
    tenant_id TEXT NOT NULL,
    organization_id TEXT NOT NULL DEFAULT '0',
    rtc_session_id TEXT NOT NULL,
    conversation_id TEXT,
    rtc_mode TEXT NOT NULL,
    initiator_principal_kind TEXT NOT NULL,
    initiator_principal_id TEXT NOT NULL,
    provider_plugin_id TEXT,
    provider_session_id TEXT,
    provider_region TEXT,
    access_endpoint TEXT,
    session_state TEXT NOT NULL,
    latest_signal_seq BIGINT NOT NULL DEFAULT 0 CHECK (latest_signal_seq >= 0),
    signaling_stream_id TEXT,
    artifact_message_id TEXT,
    started_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    ended_at TIMESTAMPTZ,
    -- Lifecycle timestamps for SLA / quality analytics (migration 0008)
    initiating_at       TIMESTAMPTZ,
    ringing_at          TIMESTAMPTZ,
    connecting_at       TIMESTAMPTZ,
    connected_at        TIMESTAMPTZ,
    on_hold_since       TIMESTAMPTZ,
    reconnecting_since  TIMESTAMPTZ,
    canceled_at         TIMESTAMPTZ,
    failed_at           TIMESTAMPTZ,
    timeout_at          TIMESTAMPTZ,
    ended_reason        TEXT,
    failure_reason      TEXT,
    payload_json JSONB NOT NULL,
    payload_hash TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    retention_until TIMESTAMPTZ,
    CONSTRAINT pk_im_rtc_sessions PRIMARY KEY (tenant_id, organization_id, rtc_session_id),
    CONSTRAINT chk_im_rtc_sessions_state CHECK (session_state IN (
        'started', 'accepted', 'rejected', 'ended',
        'initiating', 'ringing', 'connecting', 'connected',
        'on_hold', 'reconnecting', 'canceled', 'failed', 'timeout'
    )),
    CONSTRAINT chk_im_rtc_sessions_terminal_reason CHECK (
        session_state NOT IN ('ended', 'canceled', 'rejected', 'failed', 'timeout')
        OR ended_reason IS NOT NULL
    )
);

CREATE INDEX IF NOT EXISTS idx_im_rtc_sessions_conversation
    ON im_rtc_sessions (tenant_id, organization_id, conversation_id, updated_at DESC)
    WHERE conversation_id IS NOT NULL;

CREATE INDEX IF NOT EXISTS idx_im_rtc_sessions_state
    ON im_rtc_sessions (tenant_id, organization_id, session_state, updated_at DESC, rtc_session_id);

CREATE INDEX IF NOT EXISTS idx_im_rtc_sessions_provider_session
    ON im_rtc_sessions (tenant_id, organization_id, provider_plugin_id, provider_session_id)
    WHERE provider_plugin_id IS NOT NULL AND provider_session_id IS NOT NULL;

CREATE INDEX IF NOT EXISTS idx_im_rtc_sessions_retention_until
    ON im_rtc_sessions (tenant_id, organization_id, retention_until)
    WHERE retention_until IS NOT NULL;

CREATE INDEX IF NOT EXISTS idx_im_rtc_sessions_ringing_at
    ON im_rtc_sessions (tenant_id, organization_id, ringing_at)
    WHERE ringing_at IS NOT NULL AND session_state = 'ringing';

CREATE INDEX IF NOT EXISTS idx_im_rtc_sessions_active_lifecycle
    ON im_rtc_sessions (tenant_id, organization_id, session_state, updated_at)
    WHERE session_state IN (
        'initiating', 'ringing', 'connecting', 'connected',
        'on_hold', 'reconnecting', 'started', 'accepted'
    );

CREATE INDEX IF NOT EXISTS idx_im_rtc_sessions_stale_cleanup
    ON im_rtc_sessions (tenant_id, organization_id, updated_at, rtc_session_id)
    WHERE session_state NOT IN ('ended', 'canceled', 'rejected', 'failed', 'timeout');

COMMENT ON TABLE im_rtc_sessions IS
    'RTC call session lifecycle. State machine: initiating -> ringing -> connecting -> connected -> (on_hold|reconnecting)* -> ended|canceled|rejected|failed|timeout. Legacy states started/accepted retained as aliases.';
COMMENT ON COLUMN im_rtc_sessions.ended_reason IS
    'Required for terminal states. Values: normal|rejected|canceled|timeout|failed|media_drop|signaling_error|participant_left';

-- ============================================================
-- 16. RTC 信令
-- ============================================================

DROP TABLE IF EXISTS im_rtc_signals CASCADE;
CREATE TABLE im_rtc_signals (
    tenant_id TEXT NOT NULL,
    organization_id TEXT NOT NULL DEFAULT '0',
    rtc_session_id TEXT NOT NULL,
    signal_seq BIGINT NOT NULL CHECK (signal_seq > 0),
    sender_principal_kind TEXT NOT NULL,
    sender_principal_id TEXT NOT NULL,
    receiver_principal_kind TEXT,
    receiver_principal_id TEXT,
    signal_type TEXT NOT NULL,
    client_signal_id TEXT,
    payload_json JSONB NOT NULL,
    payload_hash TEXT NOT NULL,
    occurred_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    retention_until TIMESTAMPTZ,
    CONSTRAINT pk_im_rtc_signals PRIMARY KEY (tenant_id, organization_id, rtc_session_id, signal_seq),
    CONSTRAINT chk_im_rtc_signals_signal_type CHECK (signal_type IN (
        'offer', 'answer', 'ice_candidate', 'renegotiate',
        'add_participant', 'remove_participant', 'kick_participant',
        'mute', 'unmute', 'screen_share_start', 'screen_share_stop',
        'hold', 'resume', 'reconnect', 'quality_report',
        'recording_start', 'recording_stop', 'recording_status'
    ))
);

CREATE INDEX IF NOT EXISTS idx_im_rtc_signals_session_seq
    ON im_rtc_signals (tenant_id, organization_id, rtc_session_id, signal_seq);

CREATE INDEX IF NOT EXISTS idx_im_rtc_signals_retention_until
    ON im_rtc_signals (tenant_id, organization_id, retention_until)
    WHERE retention_until IS NOT NULL;

CREATE UNIQUE INDEX IF NOT EXISTS uk_im_rtc_signals_client_signal_id
    ON im_rtc_signals (tenant_id, organization_id, rtc_session_id, sender_principal_kind, sender_principal_id, client_signal_id)
    WHERE client_signal_id IS NOT NULL;

-- ============================================================
-- 17. 审计记录
-- ============================================================

DROP TABLE IF EXISTS im_audit_records CASCADE;
CREATE TABLE im_audit_records (
    tenant_id TEXT NOT NULL,
    organization_id TEXT NOT NULL DEFAULT '0',
    audit_seq BIGINT NOT NULL CHECK (audit_seq > 0),
    record_id TEXT NOT NULL,
    aggregate_type TEXT NOT NULL,
    aggregate_id TEXT NOT NULL,
    action TEXT NOT NULL,
    actor_id TEXT NOT NULL,
    actor_kind TEXT NOT NULL,
    actor_session_id TEXT,
    payload TEXT,
    recorded_at TEXT NOT NULL,
    occurred_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    target_type TEXT,
    target_id TEXT,
    retention_class TEXT NOT NULL DEFAULT 'access',
    integrity_anchor TEXT,
    integrity_anchored_at TIMESTAMPTZ,
    chain_prev_hash TEXT,
    chain_hash TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    retention_until TIMESTAMPTZ,
    CONSTRAINT pk_im_audit_records PRIMARY KEY (tenant_id, organization_id, audit_seq),
    CONSTRAINT uk_im_audit_records_record_id UNIQUE (tenant_id, organization_id, record_id),
    CONSTRAINT chk_im_audit_records_retention_class CHECK (retention_class IN (
        'security', 'access', 'admin', 'data_lifecycle'
    ))
);

CREATE INDEX IF NOT EXISTS idx_im_audit_records_tenant_seq
    ON im_audit_records (tenant_id, organization_id, audit_seq);

CREATE INDEX IF NOT EXISTS idx_im_audit_records_aggregate
    ON im_audit_records (tenant_id, organization_id, aggregate_type, aggregate_id, audit_seq);

CREATE INDEX IF NOT EXISTS idx_im_audit_records_tenant_occurred
    ON im_audit_records (tenant_id, organization_id, occurred_at DESC);

CREATE INDEX IF NOT EXISTS idx_im_audit_records_target
    ON im_audit_records (tenant_id, organization_id, target_type, target_id, occurred_at DESC)
    WHERE target_type IS NOT NULL AND target_id IS NOT NULL;

CREATE INDEX IF NOT EXISTS idx_im_audit_records_actor
    ON im_audit_records (tenant_id, organization_id, actor_id, actor_kind, occurred_at DESC);

CREATE INDEX IF NOT EXISTS idx_im_audit_records_retention_class
    ON im_audit_records (tenant_id, organization_id, retention_class, occurred_at DESC);

CREATE INDEX IF NOT EXISTS idx_im_audit_records_integrity_anchor_pending
    ON im_audit_records (tenant_id, organization_id, audit_seq)
    WHERE integrity_anchor IS NULL;

CREATE INDEX IF NOT EXISTS idx_im_audit_records_retention_until
    ON im_audit_records (tenant_id, organization_id, retention_until)
    WHERE retention_until IS NOT NULL;

COMMENT ON TABLE im_audit_records IS
    'L3 compliance audit log. occurred_at is authoritative timestamp. integrity_anchor links to external notary for WORM-like tamper evidence. retention_class drives differentiated retention (security=2y, access=180d, admin=1y, data_lifecycle=3y).';

-- ============================================================
-- 18. 通知任务
-- ============================================================

DROP TABLE IF EXISTS im_notification_tasks CASCADE;
CREATE TABLE im_notification_tasks (
    tenant_id TEXT NOT NULL,
    organization_id TEXT NOT NULL DEFAULT '0',
    notification_id TEXT NOT NULL,
    source_event_id TEXT NOT NULL,
    source_event_type TEXT NOT NULL,
    category TEXT NOT NULL,
    channel TEXT NOT NULL,
    recipient_kind TEXT NOT NULL,
    recipient_id TEXT NOT NULL,
    notification_status TEXT NOT NULL DEFAULT 'requested',
    title TEXT,
    body TEXT,
    payload_json JSONB NOT NULL,
    payload_hash TEXT NOT NULL,
    requested_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    dispatched_at TIMESTAMPTZ,
    failure_reason TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    retention_until TIMESTAMPTZ,
    CONSTRAINT pk_im_notification_tasks PRIMARY KEY (tenant_id, organization_id, notification_id),
    CONSTRAINT uk_im_notification_tasks_source UNIQUE (tenant_id, organization_id, source_event_id, recipient_kind, recipient_id, category, channel),
    CONSTRAINT chk_im_notification_tasks_status CHECK (notification_status IN ('requested', 'dispatched', 'failed'))
);

CREATE INDEX IF NOT EXISTS idx_im_notification_tasks_recipient_updated
    ON im_notification_tasks (tenant_id, organization_id, recipient_kind, recipient_id, updated_at DESC, notification_id);

CREATE INDEX IF NOT EXISTS idx_im_notification_tasks_status
    ON im_notification_tasks (tenant_id, organization_id, notification_status, updated_at DESC);

CREATE INDEX IF NOT EXISTS idx_im_notification_tasks_retention_until
    ON im_notification_tasks (tenant_id, organization_id, retention_until)
    WHERE retention_until IS NOT NULL;

-- ============================================================
-- 19. 自动化执行
-- ============================================================

DROP TABLE IF EXISTS im_automation_executions CASCADE;
CREATE TABLE im_automation_executions (
    tenant_id TEXT NOT NULL,
    organization_id TEXT NOT NULL DEFAULT '0',
    principal_kind TEXT NOT NULL,
    principal_id TEXT NOT NULL,
    execution_id TEXT NOT NULL,
    trigger_type TEXT NOT NULL,
    target_kind TEXT NOT NULL,
    target_ref TEXT NOT NULL,
    request_hash TEXT NOT NULL,
    input_payload_json JSONB,
    input_payload_hash TEXT,
    output_payload_json JSONB,
    output_payload_hash TEXT,
    execution_state TEXT NOT NULL DEFAULT 'requested',
    retry_count INTEGER NOT NULL DEFAULT 0 CHECK (retry_count >= 0),
    requested_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    completed_at TIMESTAMPTZ,
    failure_reason TEXT,
    payload_json JSONB NOT NULL,
    payload_hash TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    retention_until TIMESTAMPTZ,
    CONSTRAINT pk_im_automation_executions PRIMARY KEY (tenant_id, organization_id, principal_kind, principal_id, execution_id),
    CONSTRAINT uk_im_automation_executions_request UNIQUE (tenant_id, organization_id, principal_kind, principal_id, execution_id, request_hash),
    CONSTRAINT chk_im_automation_executions_state CHECK (execution_state IN ('requested', 'running', 'succeeded', 'failed'))
);

CREATE INDEX IF NOT EXISTS idx_im_automation_executions_principal_updated
    ON im_automation_executions (tenant_id, organization_id, principal_kind, principal_id, updated_at DESC, execution_id);

CREATE INDEX IF NOT EXISTS idx_im_automation_executions_state
    ON im_automation_executions (tenant_id, organization_id, execution_state, updated_at DESC);

CREATE INDEX IF NOT EXISTS idx_im_automation_executions_retention_until
    ON im_automation_executions (tenant_id, organization_id, retention_until)
    WHERE retention_until IS NOT NULL;

-- ============================================================
-- 20. 投影：Timeline 条目
-- ============================================================

DROP TABLE IF EXISTS im_projection_timeline_entries CASCADE;
CREATE TABLE im_projection_timeline_entries (
    tenant_id TEXT NOT NULL,
    organization_id TEXT NOT NULL DEFAULT '0',
    conversation_id TEXT NOT NULL,
    message_seq BIGINT NOT NULL CHECK (message_seq > 0),
    message_id BIGINT NOT NULL,
    summary TEXT,
    payload_json JSONB NOT NULL,
    payload_hash TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    retention_until TIMESTAMPTZ,
    CONSTRAINT pk_im_projection_timeline_entries PRIMARY KEY (tenant_id, organization_id, conversation_id, message_seq),
    CONSTRAINT uk_im_projection_timeline_entries_message UNIQUE (tenant_id, organization_id, message_id)
);

CREATE INDEX IF NOT EXISTS idx_im_projection_timeline_entries_message
    ON im_projection_timeline_entries (tenant_id, organization_id, message_id);

CREATE INDEX IF NOT EXISTS idx_im_projection_timeline_entries_retention_until
    ON im_projection_timeline_entries (tenant_id, organization_id, retention_until)
    WHERE retention_until IS NOT NULL;

-- ============================================================
-- 21. 投影：会话摘要
-- ============================================================

DROP TABLE IF EXISTS im_projection_conversation_summaries CASCADE;
CREATE TABLE im_projection_conversation_summaries (
    tenant_id TEXT NOT NULL,
    organization_id TEXT NOT NULL DEFAULT '0',
    conversation_id TEXT NOT NULL,
    conversation_type TEXT,
    message_count BIGINT NOT NULL DEFAULT 0 CHECK (message_count >= 0),
    last_message_id BIGINT,
    last_message_seq BIGINT NOT NULL DEFAULT 0 CHECK (last_message_seq >= 0),
    last_sender_kind TEXT,
    last_sender_id TEXT,
    last_summary TEXT,
    last_message_at TIMESTAMPTZ,
    last_activity_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    agent_handoff_json JSONB,
    payload_json JSONB NOT NULL,
    payload_hash TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    retention_until TIMESTAMPTZ,
    CONSTRAINT pk_im_projection_conversation_summaries PRIMARY KEY (tenant_id, organization_id, conversation_id)
);

CREATE INDEX IF NOT EXISTS idx_im_projection_conversation_summaries_activity
    ON im_projection_conversation_summaries (tenant_id, organization_id, last_activity_at DESC, conversation_id);

CREATE INDEX IF NOT EXISTS idx_im_projection_conversation_summaries_retention_until
    ON im_projection_conversation_summaries (tenant_id, organization_id, retention_until)
    WHERE retention_until IS NOT NULL;

-- ============================================================
-- 22. 投影：会话成员
-- ============================================================

DROP TABLE IF EXISTS im_projection_conversation_members CASCADE;
CREATE TABLE im_projection_conversation_members (
    tenant_id TEXT NOT NULL,
    organization_id TEXT NOT NULL DEFAULT '0',
    conversation_id TEXT NOT NULL,
    principal_kind TEXT NOT NULL,
    principal_id TEXT NOT NULL,
    member_id BIGINT NOT NULL,             -- Snowflake ID
    membership_role TEXT NOT NULL,
    membership_state TEXT NOT NULL,
    invited_by TEXT,
    joined_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    removed_at TIMESTAMPTZ,
    attributes_json JSONB NOT NULL DEFAULT '{}'::JSONB,
    payload_json JSONB NOT NULL,
    payload_hash TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    retention_until TIMESTAMPTZ,
    CONSTRAINT pk_im_projection_conversation_members PRIMARY KEY (tenant_id, organization_id, conversation_id, principal_kind, principal_id),
    CONSTRAINT uk_im_projection_conversation_members_member UNIQUE (tenant_id, organization_id, conversation_id, member_id),
    CONSTRAINT chk_im_projection_conversation_members_state CHECK (membership_state IN ('invited', 'joined', 'linked', 'removed', 'left'))
);

CREATE INDEX IF NOT EXISTS idx_im_projection_conversation_members_principal
    ON im_projection_conversation_members (tenant_id, organization_id, principal_kind, principal_id, membership_state, conversation_id);

CREATE INDEX IF NOT EXISTS idx_im_projection_conversation_members_active
    ON im_projection_conversation_members (tenant_id, organization_id, conversation_id, principal_kind, principal_id)
    WHERE membership_state = 'joined';

CREATE INDEX IF NOT EXISTS idx_im_projection_conversation_members_retention_until
    ON im_projection_conversation_members (tenant_id, organization_id, retention_until)
    WHERE retention_until IS NOT NULL;

-- ============================================================
-- 23. 投影：已读游标
-- ============================================================

DROP TABLE IF EXISTS im_projection_read_cursors CASCADE;
CREATE TABLE im_projection_read_cursors (
    tenant_id TEXT NOT NULL,
    organization_id TEXT NOT NULL DEFAULT '0',
    conversation_id TEXT NOT NULL,
    member_id BIGINT NOT NULL,
    principal_kind TEXT NOT NULL,
    principal_id TEXT NOT NULL,
    read_seq BIGINT NOT NULL DEFAULT 0 CHECK (read_seq >= 0),
    last_read_message_id BIGINT,
    payload_json JSONB NOT NULL,
    payload_hash TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    retention_until TIMESTAMPTZ,
    CONSTRAINT pk_im_projection_read_cursors PRIMARY KEY (tenant_id, organization_id, conversation_id, member_id)
);

CREATE INDEX IF NOT EXISTS idx_im_projection_read_cursors_principal
    ON im_projection_read_cursors (tenant_id, organization_id, principal_kind, principal_id, conversation_id);

CREATE INDEX IF NOT EXISTS idx_im_projection_read_cursors_retention_until
    ON im_projection_read_cursors (tenant_id, organization_id, retention_until)
    WHERE retention_until IS NOT NULL;

-- ============================================================
-- 24. 投影：注册客户端路由
-- ============================================================

DROP TABLE IF EXISTS im_projection_registered_client_routes CASCADE;
CREATE TABLE im_projection_registered_client_routes (
    tenant_id TEXT NOT NULL,
    organization_id TEXT NOT NULL DEFAULT '0',
    principal_kind TEXT NOT NULL,
    principal_id TEXT NOT NULL,
    device_id TEXT NOT NULL,
    registered_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    payload_json JSONB NOT NULL,
    payload_hash TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    retention_until TIMESTAMPTZ,
    CONSTRAINT pk_im_projection_registered_client_routes PRIMARY KEY (tenant_id, organization_id, principal_kind, principal_id, device_id)
);

CREATE INDEX IF NOT EXISTS idx_im_projection_registered_client_routes_retention_until
    ON im_projection_registered_client_routes (tenant_id, organization_id, retention_until)
    WHERE retention_until IS NOT NULL;

-- ============================================================
-- 25. 投影：客户端路由同步 Feed
-- ============================================================

DROP TABLE IF EXISTS im_projection_client_route_sync_feeds CASCADE;
CREATE TABLE im_projection_client_route_sync_feeds (
    tenant_id TEXT NOT NULL,
    organization_id TEXT NOT NULL DEFAULT '0',
    principal_kind TEXT NOT NULL,
    principal_id TEXT NOT NULL,
    device_id TEXT NOT NULL,
    sync_seq BIGINT NOT NULL CHECK (sync_seq > 0),
    origin_event_id TEXT NOT NULL,
    origin_event_type TEXT NOT NULL,
    conversation_id TEXT,
    message_id BIGINT,
    message_seq BIGINT CHECK (message_seq IS NULL OR message_seq > 0),
    member_id BIGINT,
    read_seq BIGINT CHECK (read_seq IS NULL OR read_seq >= 0),
    last_read_message_id BIGINT,
    actor_kind TEXT,
    actor_id TEXT,
    actor_device_id TEXT,
    summary TEXT,
    payload_json JSONB NOT NULL,
    payload_hash TEXT NOT NULL,
    occurred_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    retention_until TIMESTAMPTZ,
    CONSTRAINT pk_im_projection_client_route_sync_feeds PRIMARY KEY (tenant_id, organization_id, principal_kind, principal_id, device_id, sync_seq)
);

CREATE INDEX IF NOT EXISTS idx_im_projection_client_route_sync_feeds_window
    ON im_projection_client_route_sync_feeds (tenant_id, organization_id, principal_kind, principal_id, device_id, sync_seq);

CREATE INDEX IF NOT EXISTS idx_im_projection_client_route_sync_feeds_conversation
    ON im_projection_client_route_sync_feeds (tenant_id, organization_id, conversation_id, sync_seq)
    WHERE conversation_id IS NOT NULL;

CREATE INDEX IF NOT EXISTS idx_im_projection_client_route_sync_feeds_retention_until
    ON im_projection_client_route_sync_feeds (tenant_id, organization_id, retention_until)
    WHERE retention_until IS NOT NULL;

-- ============================================================
-- 26. 投影：客户端路由同步检查点
-- ============================================================

DROP TABLE IF EXISTS im_projection_client_route_sync_checkpoints CASCADE;
CREATE TABLE im_projection_client_route_sync_checkpoints (
    tenant_id TEXT NOT NULL,
    organization_id TEXT NOT NULL DEFAULT '0',
    principal_kind TEXT NOT NULL,
    principal_id TEXT NOT NULL,
    device_id TEXT NOT NULL,
    latest_sync_seq BIGINT NOT NULL DEFAULT 0 CHECK (latest_sync_seq >= 0),
    trimmed_through_seq BIGINT NOT NULL DEFAULT 0 CHECK (trimmed_through_seq >= 0),
    payload_json JSONB NOT NULL,
    payload_hash TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    retention_until TIMESTAMPTZ,
    CONSTRAINT pk_im_projection_client_route_sync_checkpoints PRIMARY KEY (tenant_id, organization_id, principal_kind, principal_id, device_id),
    CONSTRAINT chk_im_projection_client_route_sync_checkpoints_order CHECK (trimmed_through_seq <= latest_sync_seq)
);

CREATE INDEX IF NOT EXISTS idx_im_projection_client_route_sync_checkpoints_retention_until
    ON im_projection_client_route_sync_checkpoints (tenant_id, organization_id, retention_until)
    WHERE retention_until IS NOT NULL;

-- ============================================================
-- 27. 投影：联系人
-- ============================================================

DROP TABLE IF EXISTS im_projection_contacts CASCADE;
CREATE TABLE im_projection_contacts (
    tenant_id TEXT NOT NULL,
    organization_id TEXT NOT NULL DEFAULT '0',
    owner_user_id TEXT NOT NULL,
    contact_type TEXT NOT NULL,
    target_user_id TEXT NOT NULL,
    relationship_state TEXT NOT NULL,
    friendship_id TEXT NOT NULL,
    direct_chat_id TEXT,
    conversation_id TEXT,
    established_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_interaction_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    payload_json JSONB NOT NULL,
    payload_hash TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    retention_until TIMESTAMPTZ,
    CONSTRAINT pk_im_projection_contacts PRIMARY KEY (tenant_id, organization_id, owner_user_id, contact_type, target_user_id)
);

CREATE INDEX IF NOT EXISTS idx_im_projection_contacts_owner_activity
    ON im_projection_contacts (tenant_id, organization_id, owner_user_id, last_interaction_at DESC, target_user_id);

CREATE INDEX IF NOT EXISTS idx_im_projection_contacts_retention_until
    ON im_projection_contacts (tenant_id, organization_id, retention_until)
    WHERE retention_until IS NOT NULL;

-- ============================================================
-- 28. 投影：直接聊天绑定
-- ============================================================

DROP TABLE IF EXISTS im_projection_direct_chat_bindings CASCADE;
CREATE TABLE im_projection_direct_chat_bindings (
    tenant_id TEXT NOT NULL,
    organization_id TEXT NOT NULL DEFAULT '0',
    direct_chat_id TEXT NOT NULL,
    conversation_id TEXT NOT NULL,
    direct_chat_status TEXT NOT NULL DEFAULT 'active',
    bound_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    payload_json JSONB NOT NULL,
    payload_hash TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    retention_until TIMESTAMPTZ,
    CONSTRAINT pk_im_projection_direct_chat_bindings PRIMARY KEY (tenant_id, organization_id, direct_chat_id),
    CONSTRAINT uk_im_projection_direct_chat_bindings_conversation UNIQUE (tenant_id, organization_id, conversation_id),
    CONSTRAINT chk_im_projection_direct_chat_bindings_status CHECK (direct_chat_status IN ('active', 'archived'))
);

CREATE INDEX IF NOT EXISTS idx_im_projection_direct_chat_bindings_conversation
    ON im_projection_direct_chat_bindings (tenant_id, organization_id, conversation_id, direct_chat_status);

CREATE INDEX IF NOT EXISTS idx_im_projection_direct_chat_bindings_retention_until
    ON im_projection_direct_chat_bindings (tenant_id, organization_id, retention_until)
    WHERE retention_until IS NOT NULL;

-- ============================================================
-- 29. Stream Sessions
-- ============================================================

DROP TABLE IF EXISTS im_stream_sessions CASCADE;
CREATE TABLE im_stream_sessions (
    tenant_id TEXT NOT NULL,
    organization_id TEXT NOT NULL DEFAULT '0',
    stream_id TEXT NOT NULL,
    owner_principal_kind TEXT NOT NULL,
    owner_principal_id TEXT NOT NULL,
    stream_type TEXT NOT NULL,
    scope_kind TEXT NOT NULL,
    scope_id TEXT NOT NULL,
    durability_class TEXT NOT NULL,
    ordering_scope TEXT NOT NULL,
    schema_ref TEXT,
    stream_state TEXT NOT NULL DEFAULT 'created',
    last_frame_seq BIGINT NOT NULL DEFAULT 0 CHECK (last_frame_seq >= 0),
    last_checkpoint_seq BIGINT CHECK (last_checkpoint_seq >= 0),
    result_message_id BIGINT,
    complete_frame_seq BIGINT CHECK (complete_frame_seq >= 0),
    abort_frame_seq BIGINT CHECK (abort_frame_seq >= 0),
    abort_reason TEXT,
    opened_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    closed_at TIMESTAMPTZ,
    expires_at TIMESTAMPTZ,
    payload_json JSONB NOT NULL,
    payload_hash TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    retention_until TIMESTAMPTZ,
    CONSTRAINT pk_im_stream_sessions PRIMARY KEY (tenant_id, organization_id, stream_id),
    CONSTRAINT chk_im_stream_sessions_state CHECK (stream_state IN ('created', 'opened', 'active', 'checkpointed', 'completed', 'aborted', 'expired')),
    CONSTRAINT chk_im_stream_sessions_seq_order CHECK (
        COALESCE(last_checkpoint_seq, 0) <= last_frame_seq
        AND COALESCE(complete_frame_seq, 0) <= last_frame_seq
        AND COALESCE(abort_frame_seq, 0) <= last_frame_seq
    )
);

CREATE INDEX IF NOT EXISTS idx_im_stream_sessions_scope
    ON im_stream_sessions (tenant_id, organization_id, scope_kind, scope_id, updated_at DESC);

CREATE INDEX IF NOT EXISTS idx_im_stream_sessions_updated
    ON im_stream_sessions (tenant_id, organization_id, updated_at DESC, stream_id);

CREATE INDEX IF NOT EXISTS idx_im_stream_sessions_retention_until
    ON im_stream_sessions (tenant_id, organization_id, retention_until)
    WHERE retention_until IS NOT NULL;

-- ============================================================
-- 30. Stream Frames
-- ============================================================

DROP TABLE IF EXISTS im_stream_frames CASCADE;
CREATE TABLE im_stream_frames (
    tenant_id TEXT NOT NULL,
    organization_id TEXT NOT NULL DEFAULT '0',
    stream_id TEXT NOT NULL,
    frame_seq BIGINT NOT NULL CHECK (frame_seq > 0),
    producer_principal_kind TEXT NOT NULL,
    producer_principal_id TEXT NOT NULL,
    schema_ref TEXT,
    payload_json JSONB NOT NULL,
    payload_hash TEXT NOT NULL,
    occurred_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    retention_until TIMESTAMPTZ,
    CONSTRAINT pk_im_stream_frames PRIMARY KEY (tenant_id, organization_id, stream_id, frame_seq)
);

CREATE INDEX IF NOT EXISTS idx_im_stream_frames_stream_seq
    ON im_stream_frames (tenant_id, organization_id, stream_id, frame_seq);

CREATE INDEX IF NOT EXISTS idx_im_stream_frames_retention_until
    ON im_stream_frames (tenant_id, organization_id, retention_until)
    WHERE retention_until IS NOT NULL;

-- source: deployments/database/postgres/migrations/012_im_social_org_interactions.sql
-- Migration 012: Social Relations, Organization Model, Message Interactions
-- 对齐行业最专业 IM（微信 Telegram/Discord/Slack）的数据库设计
-- 所有 ID 统一使用 Snowflake ID (BIGINT)

-- ============================================================
-- 设计原则：
-- 1. 所有主键 ID 使用 Snowflake BIGINT
-- 2. 租户和用户引入 IAM 系统（iam_tenant, iam_user）
-- 3. 组织模型（Space/Group/Channel）是 IM 专有
-- 4. 社交关系独立持久化，不依赖内部事件溯源
-- 5. 消息互动（Reaction/Pin/Thread）独立表
-- ============================================================

-- ============================================================
-- 第一部分：社交关系真值表
-- ============================================================

-- 1. 好友请求表
CREATE TABLE IF NOT EXISTS im_friend_requests (
    tenant_id           TEXT NOT NULL,
    organization_id     TEXT NOT NULL DEFAULT '0',
    request_id          BIGINT NOT NULL,           -- Snowflake ID
    requester_user_id   TEXT NOT NULL,              -- 引用 iam_user.user_id
    target_user_id      TEXT NOT NULL,              -- 引用 iam_user.user_id
    request_message     TEXT,
    status              TEXT NOT NULL DEFAULT 'pending',
    expired_at          TIMESTAMPTZ,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT pk_im_friend_requests PRIMARY KEY (tenant_id, organization_id, request_id),
    CONSTRAINT uk_im_friend_requests_pair UNIQUE (tenant_id, organization_id, requester_user_id, target_user_id, status),
    CONSTRAINT chk_im_friend_requests_status CHECK (status IN ('pending', 'accepted', 'declined', 'canceled', 'expired')),
    CONSTRAINT chk_im_friend_requests_not_self CHECK (requester_user_id != target_user_id)
);

CREATE INDEX IF NOT EXISTS idx_im_friend_requests_requester
    ON im_friend_requests (tenant_id, organization_id, requester_user_id, status, created_at DESC);

CREATE INDEX IF NOT EXISTS idx_im_friend_requests_target
    ON im_friend_requests (tenant_id, organization_id, target_user_id, status, created_at DESC);

CREATE INDEX IF NOT EXISTS idx_im_friend_requests_expired
    ON im_friend_requests (tenant_id, organization_id, expired_at)
    WHERE expired_at IS NOT NULL AND status = 'pending';

-- 2. 好友关系表
CREATE TABLE IF NOT EXISTS im_friendships (
    tenant_id           TEXT NOT NULL,
    organization_id     TEXT NOT NULL DEFAULT '0',
    friendship_id       BIGINT NOT NULL,           -- Snowflake ID
user_low_id         TEXT NOT NULL,              -- 规范化：较小的 user_id
user_high_id        TEXT NOT NULL,              -- 规范化：较大的 user_id
initiator_user_id   TEXT NOT NULL,              -- 发起好友请求的用户
    status              TEXT NOT NULL DEFAULT 'active',
    established_at      TIMESTAMPTZ,
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT pk_im_friendships PRIMARY KEY (tenant_id, organization_id, friendship_id),
    CONSTRAINT uk_im_friendships_pair UNIQUE (tenant_id, organization_id, user_low_id, user_high_id),
    CONSTRAINT chk_im_friendships_status CHECK (status IN ('active', 'removed')),
    CONSTRAINT chk_im_friendships_not_self CHECK (user_low_id < user_high_id)
);

CREATE INDEX IF NOT EXISTS idx_im_friendships_user_low
    ON im_friendships (tenant_id, organization_id, user_low_id, status, established_at DESC);

CREATE INDEX IF NOT EXISTS idx_im_friendships_user_high
    ON im_friendships (tenant_id, organization_id, user_high_id, status, established_at DESC);

-- 3. 用户屏蔽表
CREATE TABLE IF NOT EXISTS im_user_blocks (
    tenant_id           TEXT NOT NULL,
    organization_id     TEXT NOT NULL DEFAULT '0',
    block_id            BIGINT NOT NULL,           -- Snowflake ID
blocker_user_id     TEXT NOT NULL,              -- 屏蔽者
blocked_user_id     TEXT NOT NULL,              -- 被屏蔽者
    scope               TEXT NOT NULL DEFAULT 'all',
    direct_chat_id      BIGINT,                    -- 关联 direct_chat 作用域
    reason              TEXT,
    expires_at          TIMESTAMPTZ,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT pk_im_user_blocks PRIMARY KEY (tenant_id, organization_id, block_id),
    CONSTRAINT uk_im_user_blocks_pair UNIQUE (tenant_id, organization_id, blocker_user_id, blocked_user_id, scope),
    CONSTRAINT chk_im_user_blocks_scope CHECK (scope IN ('all', 'friendship', 'direct_chat')),
    CONSTRAINT chk_im_user_blocks_not_self CHECK (blocker_user_id != blocked_user_id)
);

CREATE INDEX IF NOT EXISTS idx_im_user_blocks_blocker
    ON im_user_blocks (tenant_id, organization_id, blocker_user_id, scope, created_at DESC);

CREATE INDEX IF NOT EXISTS idx_im_user_blocks_blocked
    ON im_user_blocks (tenant_id, organization_id, blocked_user_id, scope, created_at DESC);

CREATE INDEX IF NOT EXISTS idx_im_user_blocks_expires
    ON im_user_blocks (tenant_id, organization_id, expires_at)
    WHERE expires_at IS NOT NULL;

-- 4. 单聊会话表
CREATE TABLE IF NOT EXISTS im_direct_chats (
    tenant_id           TEXT NOT NULL,
    organization_id     TEXT NOT NULL DEFAULT '0',
    direct_chat_id      BIGINT NOT NULL,           -- Snowflake ID
    left_actor_kind     TEXT NOT NULL,
    left_actor_id       TEXT NOT NULL,
    right_actor_kind    TEXT NOT NULL,
    right_actor_id      TEXT NOT NULL,
    pair_hash           TEXT NOT NULL,              -- 规范化后的哈希
    status              TEXT NOT NULL DEFAULT 'active',
    conversation_id     TEXT,                       -- 关联的会话 ID
    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT pk_im_direct_chats PRIMARY KEY (tenant_id, organization_id, direct_chat_id),
    CONSTRAINT uk_im_direct_chats_pair UNIQUE (tenant_id, organization_id, pair_hash),
    CONSTRAINT chk_im_direct_chats_status CHECK (status IN ('active', 'archived', 'closed'))
);

CREATE INDEX IF NOT EXISTS idx_im_direct_chats_left_actor
    ON im_direct_chats (tenant_id, organization_id, left_actor_kind, left_actor_id, status);

CREATE INDEX IF NOT EXISTS idx_im_direct_chats_right_actor
    ON im_direct_chats (tenant_id, organization_id, right_actor_kind, right_actor_id, status);

CREATE INDEX IF NOT EXISTS idx_im_direct_chats_conversation
    ON im_direct_chats (tenant_id, organization_id, conversation_id)
    WHERE conversation_id IS NOT NULL;

-- 5. 外部连接表
CREATE TABLE IF NOT EXISTS im_external_connections (
    tenant_id           TEXT NOT NULL,
    organization_id     TEXT NOT NULL DEFAULT '0',
    connection_id       BIGINT NOT NULL,           -- Snowflake ID
    external_tenant_id  TEXT NOT NULL,
    external_org_name   TEXT,
    connection_kind     TEXT NOT NULL DEFAULT 'shared_channel',
    status              TEXT NOT NULL DEFAULT 'active',
    established_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT pk_im_external_connections PRIMARY KEY (tenant_id, organization_id, connection_id),
    CONSTRAINT uk_im_external_connections_pair UNIQUE (tenant_id, organization_id, external_tenant_id),
    CONSTRAINT chk_im_external_connections_kind CHECK (connection_kind IN ('shared_channel')),
    CONSTRAINT chk_im_external_connections_status CHECK (status IN ('active', 'suspended', 'revoked')),
    CONSTRAINT chk_im_external_connections_not_self CHECK (tenant_id != external_tenant_id)
);

-- 6. 外部成员链接表
CREATE TABLE IF NOT EXISTS im_external_member_links (
    tenant_id               TEXT NOT NULL,
    organization_id         TEXT NOT NULL DEFAULT '0',
    link_id                 BIGINT NOT NULL,           -- Snowflake ID
    connection_id           BIGINT NOT NULL,
    local_actor_kind        TEXT NOT NULL,
    local_actor_id          TEXT NOT NULL,
    external_member_id      TEXT NOT NULL,
    external_display_name   TEXT,
    status                  TEXT NOT NULL DEFAULT 'active',
    linked_at               TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at              TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT pk_im_external_member_links PRIMARY KEY (tenant_id, organization_id, link_id),
    CONSTRAINT uk_im_external_member_links_mapping UNIQUE (tenant_id, organization_id, connection_id, local_actor_id, external_member_id),
    CONSTRAINT chk_im_external_member_links_status CHECK (status IN ('active', 'revoked'))
);

CREATE INDEX IF NOT EXISTS idx_im_external_member_links_connection
    ON im_external_member_links (tenant_id, organization_id, connection_id, status);

CREATE INDEX IF NOT EXISTS idx_im_external_member_links_local_actor
    ON im_external_member_links (tenant_id, organization_id, local_actor_kind, local_actor_id, status);

-- 7. 共享频道策略表
CREATE TABLE IF NOT EXISTS im_shared_channel_policies (
    tenant_id               TEXT NOT NULL,
    organization_id         TEXT NOT NULL DEFAULT '0',
    policy_id               BIGINT NOT NULL,           -- Snowflake ID
    connection_id           BIGINT NOT NULL,
    channel_id              TEXT NOT NULL,
    conversation_id         TEXT,
    policy_version          BIGINT NOT NULL DEFAULT 1,
    history_visibility      TEXT NOT NULL DEFAULT 'shared',
    status                  TEXT NOT NULL DEFAULT 'active',
    applied_at              TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at              TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT pk_im_shared_channel_policies PRIMARY KEY (tenant_id, organization_id, policy_id),
    CONSTRAINT uk_im_shared_channel_policies_target UNIQUE (tenant_id, organization_id, connection_id, channel_id),
    CONSTRAINT chk_im_shared_channel_policies_visibility CHECK (history_visibility IN ('shared', 'isolated')),
    CONSTRAINT chk_im_shared_channel_policies_status CHECK (status IN ('active', 'suspended'))
);

CREATE INDEX IF NOT EXISTS idx_im_shared_channel_policies_connection
    ON im_shared_channel_policies (tenant_id, organization_id, connection_id, status);

-- ============================================================
-- 第二部分：组织模型（IM 专有）
-- ============================================================

-- 8. 空间/组织表
CREATE TABLE IF NOT EXISTS im_spaces (
    tenant_id           TEXT NOT NULL,
    organization_id     TEXT NOT NULL DEFAULT '0',
    space_id            BIGINT NOT NULL,           -- Snowflake ID
    space_name          TEXT NOT NULL,
    space_type          TEXT NOT NULL DEFAULT 'organization',
    owner_user_id       TEXT NOT NULL,              -- 引用 iam_user.user_id
    description         TEXT,
    avatar_url          TEXT,
    max_members         INTEGER NOT NULL DEFAULT 10000,
    settings_json       JSONB NOT NULL DEFAULT '{}',
    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT pk_im_spaces PRIMARY KEY (tenant_id, organization_id, space_id),
    CONSTRAINT chk_im_spaces_type CHECK (space_type IN ('organization', 'team', 'project', 'community'))
);

CREATE INDEX IF NOT EXISTS idx_im_spaces_owner
    ON im_spaces (tenant_id, organization_id, owner_user_id, created_at DESC);

CREATE INDEX IF NOT EXISTS idx_im_spaces_type
    ON im_spaces (tenant_id, organization_id, space_type, created_at DESC);

-- 9. 空间成员表
CREATE TABLE IF NOT EXISTS im_space_members (
    tenant_id           TEXT NOT NULL,
    organization_id     TEXT NOT NULL DEFAULT '0',
    space_id            BIGINT NOT NULL,
    user_id             TEXT NOT NULL,              -- 引用 iam_user.user_id
    role                TEXT NOT NULL DEFAULT 'member',
    nickname            TEXT,
    joined_at           TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT pk_im_space_members PRIMARY KEY (tenant_id, organization_id, space_id, user_id),
    CONSTRAINT chk_im_space_members_role CHECK (role IN ('owner', 'admin', 'member', 'guest'))
);

CREATE INDEX IF NOT EXISTS idx_im_space_members_user
    ON im_space_members (tenant_id, organization_id, user_id, role);

-- 10. 群组表
CREATE TABLE IF NOT EXISTS im_chat_groups (
    tenant_id           TEXT NOT NULL,
    organization_id     TEXT NOT NULL DEFAULT '0',
    group_id            BIGINT NOT NULL,           -- Snowflake ID
    space_id            BIGINT,                    -- 所属空间（可选）
    group_name          TEXT NOT NULL,
    group_type          TEXT NOT NULL DEFAULT 'normal',
    owner_user_id       TEXT NOT NULL,              -- 引用 iam_user.user_id
    conversation_id     TEXT,                       -- 关联的会话 ID
    max_members         INTEGER NOT NULL DEFAULT 500,
    description         TEXT,
    avatar_url          TEXT,
    announcement        TEXT,                       -- 群公告
    settings_json       JSONB NOT NULL DEFAULT '{}',
    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT pk_im_chat_groups PRIMARY KEY (tenant_id, organization_id, group_id),
    CONSTRAINT chk_im_chat_groups_type CHECK (group_type IN ('normal', 'announcement', 'project', 'department'))
);

CREATE INDEX IF NOT EXISTS idx_im_chat_groups_space
    ON im_chat_groups (tenant_id, organization_id, space_id, created_at DESC)
    WHERE space_id IS NOT NULL;

CREATE INDEX IF NOT EXISTS idx_im_chat_groups_owner
    ON im_chat_groups (tenant_id, organization_id, owner_user_id, created_at DESC);

CREATE INDEX IF NOT EXISTS idx_im_chat_groups_conversation
    ON im_chat_groups (tenant_id, organization_id, conversation_id)
    WHERE conversation_id IS NOT NULL;

-- 11. 群组成员表
CREATE TABLE IF NOT EXISTS im_group_members (
    tenant_id           TEXT NOT NULL,
    organization_id     TEXT NOT NULL DEFAULT '0',
    group_id            BIGINT NOT NULL,
    user_id             TEXT NOT NULL,              -- 引用 iam_user.user_id
    role                TEXT NOT NULL DEFAULT 'member',
    nickname            TEXT,                       -- 群内昵称
    mute_until          TIMESTAMPTZ,               -- 禁言截止时间
    joined_at           TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT pk_im_group_members PRIMARY KEY (tenant_id, organization_id, group_id, user_id),
    CONSTRAINT chk_im_group_members_role CHECK (role IN ('owner', 'admin', 'member', 'muted'))
);

CREATE INDEX IF NOT EXISTS idx_im_group_members_user
    ON im_group_members (tenant_id, organization_id, user_id, role);

CREATE INDEX IF NOT EXISTS idx_im_group_members_role
    ON im_group_members (tenant_id, organization_id, group_id, role, joined_at);

-- 12. 频道表
CREATE TABLE IF NOT EXISTS im_chat_channels (
    tenant_id           TEXT NOT NULL,
    organization_id     TEXT NOT NULL DEFAULT '0',
    channel_id          BIGINT NOT NULL,           -- Snowflake ID
    space_id            BIGINT NOT NULL,
    channel_name        TEXT NOT NULL,
    channel_type        TEXT NOT NULL DEFAULT 'text',
    description         TEXT,
    conversation_id     TEXT,                       -- 关联的会话 ID
    position            INTEGER NOT NULL DEFAULT 0,
    is_nsfw             BOOLEAN NOT NULL DEFAULT FALSE,
    is_pinned           BOOLEAN NOT NULL DEFAULT FALSE,
    topic               TEXT,                       -- 频道话题
    settings_json       JSONB NOT NULL DEFAULT '{}',
    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT pk_im_chat_channels PRIMARY KEY (tenant_id, organization_id, channel_id),
    CONSTRAINT chk_im_chat_channels_type CHECK (channel_type IN ('text', 'voice', 'announcement', 'forum'))
);

CREATE INDEX IF NOT EXISTS idx_im_chat_channels_space
    ON im_chat_channels (tenant_id, organization_id, space_id, position, channel_name);

CREATE INDEX IF NOT EXISTS idx_im_chat_channels_conversation
    ON im_chat_channels (tenant_id, organization_id, conversation_id)
    WHERE conversation_id IS NOT NULL;

-- 13. 频道访问规则表
CREATE TABLE IF NOT EXISTS im_channel_access_rules (
    tenant_id           TEXT NOT NULL,
    organization_id     TEXT NOT NULL DEFAULT '0',
    rule_id             BIGINT NOT NULL,           -- Snowflake ID
    channel_id          BIGINT NOT NULL,
    rule_type           TEXT NOT NULL,
    principal_kind      TEXT,                       -- user/role/group
    principal_id        TEXT,
    permission          TEXT NOT NULL,              -- view/send/manage
    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT pk_im_channel_access_rules PRIMARY KEY (tenant_id, organization_id, rule_id),
    CONSTRAINT uk_im_channel_access_rules_target UNIQUE (tenant_id, organization_id, channel_id, rule_type, principal_kind, principal_id, permission),
    CONSTRAINT chk_im_channel_access_rules_type CHECK (rule_type IN ('allow', 'deny'))
);

CREATE INDEX IF NOT EXISTS idx_im_channel_access_rules_channel
    ON im_channel_access_rules (tenant_id, organization_id, channel_id, rule_type);

-- ============================================================
-- 第三部分：消息互动表
-- ============================================================

-- 14. 消息 Reaction 表
CREATE TABLE IF NOT EXISTS im_message_reactions (
    tenant_id           TEXT NOT NULL,
    organization_id     TEXT NOT NULL DEFAULT '0',
    conversation_id     TEXT NOT NULL,
    message_id          BIGINT NOT NULL,
    user_id             TEXT NOT NULL,              -- 引用 iam_user.user_id
    reaction_type       TEXT NOT NULL,              -- emoji 类型（如 👍, ❤️, 😂）
    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT pk_im_message_reactions PRIMARY KEY (tenant_id, organization_id, conversation_id, message_id, user_id, reaction_type)
);

CREATE INDEX IF NOT EXISTS idx_im_message_reactions_message
    ON im_message_reactions (tenant_id, organization_id, conversation_id, message_id, reaction_type);

CREATE INDEX IF NOT EXISTS idx_im_message_reactions_user
    ON im_message_reactions (tenant_id, organization_id, user_id, created_at DESC);

-- 15. 消息 Pin 表
CREATE TABLE IF NOT EXISTS im_message_pins (
    tenant_id           TEXT NOT NULL,
    organization_id     TEXT NOT NULL DEFAULT '0',
    conversation_id     TEXT NOT NULL,
    message_id          BIGINT NOT NULL,
    pinned_by_user_id   TEXT NOT NULL,              -- 引用 iam_user.user_id
    pin_reason          TEXT,
    pinned_at           TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT pk_im_message_pins PRIMARY KEY (tenant_id, organization_id, conversation_id, message_id)
);

CREATE INDEX IF NOT EXISTS idx_im_message_pins_conversation
    ON im_message_pins (tenant_id, organization_id, conversation_id, pinned_at DESC);

CREATE INDEX IF NOT EXISTS idx_im_message_pins_user
    ON im_message_pins (tenant_id, organization_id, pinned_by_user_id, pinned_at DESC);

-- 16. Thread 表
CREATE TABLE IF NOT EXISTS im_threads (
    tenant_id           TEXT NOT NULL,
    organization_id     TEXT NOT NULL DEFAULT '0',
    thread_id           BIGINT NOT NULL,           -- Snowflake ID
    conversation_id     TEXT NOT NULL,
    root_message_id     BIGINT NOT NULL,
    thread_title        TEXT,
    reply_count         INTEGER NOT NULL DEFAULT 0 CHECK (reply_count >= 0),
    last_reply_at       TIMESTAMPTZ,
    last_reply_user_id  TEXT,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT pk_im_threads PRIMARY KEY (tenant_id, organization_id, thread_id),
    CONSTRAINT uk_im_threads_root UNIQUE (tenant_id, organization_id, conversation_id, root_message_id)
);

CREATE INDEX IF NOT EXISTS idx_im_threads_conversation
    ON im_threads (tenant_id, organization_id, conversation_id, last_reply_at DESC);

CREATE INDEX IF NOT EXISTS idx_im_threads_root_message
    ON im_threads (tenant_id, organization_id, root_message_id);

-- 17. Thread 订阅表
CREATE TABLE IF NOT EXISTS im_thread_subscriptions (
    tenant_id           TEXT NOT NULL,
    organization_id     TEXT NOT NULL DEFAULT '0',
    thread_id           BIGINT NOT NULL,
    user_id             TEXT NOT NULL,              -- 引用 iam_user.user_id
    last_read_seq       BIGINT NOT NULL DEFAULT 0,
    notification_level  TEXT NOT NULL DEFAULT 'all',
    subscribed_at       TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT pk_im_thread_subscriptions PRIMARY KEY (tenant_id, organization_id, thread_id, user_id),
    CONSTRAINT chk_im_thread_subscriptions_level CHECK (notification_level IN ('all', 'mentions', 'none'))
);

CREATE INDEX IF NOT EXISTS idx_im_thread_subscriptions_user
    ON im_thread_subscriptions (tenant_id, organization_id, user_id, subscribed_at DESC);

-- ============================================================
-- 第四部分：IM 用户扩展表
-- ============================================================

-- 18. IM 用户资料扩展表
CREATE TABLE IF NOT EXISTS im_user_profiles (
    tenant_id               TEXT NOT NULL,
    organization_id         TEXT NOT NULL DEFAULT '0',
    user_id                 TEXT NOT NULL,              -- 引用 iam_user.user_id
    im_nickname             TEXT,                       -- IM 专属昵称
    im_avatar_url           TEXT,                       -- IM 专属头像
    im_status_message       TEXT,                       -- 状态消息
    im_notification_prefs   JSONB NOT NULL DEFAULT '{}', -- 通知偏好
    im_mute_settings        JSONB NOT NULL DEFAULT '{}', -- 免打扰设置
    im_privacy_settings     JSONB NOT NULL DEFAULT '{}', -- 隐私设置
    im_online_status        TEXT NOT NULL DEFAULT 'online',
    last_active_at          TIMESTAMPTZ,
    created_at              TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at              TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT pk_im_user_profiles PRIMARY KEY (tenant_id, organization_id, user_id),
    CONSTRAINT chk_im_user_profiles_online_status CHECK (im_online_status IN ('online', 'away', 'busy', 'invisible', 'offline'))
);

-- 19. 用户设置表
CREATE TABLE IF NOT EXISTS im_user_settings (
    tenant_id           TEXT NOT NULL,
    organization_id     TEXT NOT NULL DEFAULT '0',
    user_id             TEXT NOT NULL,
    setting_key         TEXT NOT NULL,
    setting_value       JSONB NOT NULL,
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT pk_im_user_settings PRIMARY KEY (tenant_id, organization_id, user_id, setting_key)
);

-- 20. 会话设置表（用户对特定会话的设置）
CREATE TABLE IF NOT EXISTS im_conversation_settings (
    tenant_id           TEXT NOT NULL,
    organization_id     TEXT NOT NULL DEFAULT '0',
    conversation_id     TEXT NOT NULL,
    user_id             TEXT NOT NULL,
    is_muted            BOOLEAN NOT NULL DEFAULT FALSE,
    mute_until          TIMESTAMPTZ,
    is_pinned           BOOLEAN NOT NULL DEFAULT FALSE,
    is_archived         BOOLEAN NOT NULL DEFAULT FALSE,
    is_blocked          BOOLEAN NOT NULL DEFAULT FALSE,
    notification_level  TEXT NOT NULL DEFAULT 'all',
    custom_name         TEXT,                       -- 用户自定义会话名称
    settings_json       JSONB NOT NULL DEFAULT '{}',
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT pk_im_conversation_settings PRIMARY KEY (tenant_id, organization_id, conversation_id, user_id),
    CONSTRAINT chk_im_conversation_settings_notification CHECK (notification_level IN ('all', 'mentions', 'none'))
);

CREATE INDEX IF NOT EXISTS idx_im_conversation_settings_user
    ON im_conversation_settings (tenant_id, organization_id, user_id, is_pinned DESC, updated_at DESC);

-- ============================================================
-- 第五部分：消息搜索索引
-- ============================================================

-- 21. 消息搜索向量表
ALTER TABLE im_conversation_messages ADD COLUMN IF NOT EXISTS search_vector tsvector;

-- 22. 消息搜索索引
CREATE INDEX IF NOT EXISTS idx_im_messages_search
    ON im_conversation_messages USING GIN(search_vector)
    WHERE deleted_at IS NULL;

-- 23. 消息搜索触发器
CREATE OR REPLACE FUNCTION im_messages_search_trigger() RETURNS trigger AS $$
BEGIN
    NEW.search_vector := to_tsvector('simple',
        COALESCE(NEW.payload_json->>'text', '') || ' ' ||
        COALESCE(NEW.payload_json->>'caption', '') || ' ' ||
        COALESCE(NEW.payload_json->>'description', '')
    );
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS im_messages_search_update ON im_conversation_messages;
CREATE TRIGGER im_messages_search_update
    BEFORE INSERT OR UPDATE ON im_conversation_messages
    FOR EACH ROW EXECUTE FUNCTION im_messages_search_trigger();

-- ============================================================
-- 第六部分：邀请和封禁
-- ============================================================

-- 24. 邀请表
CREATE TABLE IF NOT EXISTS im_invitations (
    tenant_id           TEXT NOT NULL,
    organization_id     TEXT NOT NULL DEFAULT '0',
    invitation_id       BIGINT NOT NULL,           -- Snowflake ID
    inviter_user_id     TEXT NOT NULL,
    invitee_user_id     TEXT,
    invitee_email       TEXT,
    invitee_phone       TEXT,
    target_type         TEXT NOT NULL,              -- space/group/channel
    target_id           BIGINT NOT NULL,
    role                TEXT NOT NULL DEFAULT 'member',
    status              TEXT NOT NULL DEFAULT 'pending',
    message             TEXT,
    expires_at          TIMESTAMPTZ,
    accepted_at         TIMESTAMPTZ,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT pk_im_invitations PRIMARY KEY (tenant_id, organization_id, invitation_id),
    CONSTRAINT chk_im_invitations_target_type CHECK (target_type IN ('space', 'group', 'channel')),
    CONSTRAINT chk_im_invitations_status CHECK (status IN ('pending', 'accepted', 'declined', 'expired', 'canceled'))
);

CREATE INDEX IF NOT EXISTS idx_im_invitations_invitee
    ON im_invitations (tenant_id, organization_id, invitee_user_id, status, created_at DESC)
    WHERE invitee_user_id IS NOT NULL;

CREATE INDEX IF NOT EXISTS idx_im_invitations_target
    ON im_invitations (tenant_id, organization_id, target_type, target_id, status);

-- 25. 封禁记录表
CREATE TABLE IF NOT EXISTS im_ban_records (
    tenant_id           TEXT NOT NULL,
    organization_id     TEXT NOT NULL DEFAULT '0',
    ban_id              BIGINT NOT NULL,           -- Snowflake ID
    target_type         TEXT NOT NULL,              -- space/group/channel
    target_id           BIGINT NOT NULL,
    banned_user_id      TEXT NOT NULL,
    banned_by_user_id   TEXT NOT NULL,
    reason              TEXT,
    expires_at          TIMESTAMPTZ,
    unbanned_at         TIMESTAMPTZ,
    unbanned_by_user_id TEXT,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT pk_im_ban_records PRIMARY KEY (tenant_id, organization_id, ban_id),
    CONSTRAINT chk_im_ban_records_target_type CHECK (target_type IN ('space', 'group', 'channel'))
);

CREATE INDEX IF NOT EXISTS idx_im_ban_records_target
    ON im_ban_records (tenant_id, organization_id, target_type, target_id, banned_user_id)
    WHERE unbanned_at IS NULL;

CREATE INDEX IF NOT EXISTS idx_im_ban_records_user
    ON im_ban_records (tenant_id, organization_id, banned_user_id, created_at DESC);

-- ============================================================
-- 完成
-- ============================================================

-- 注册新表到 database-table-registry.json
-- 注册新表到 database-prefix-registry.json

-- source: deployments/database/postgres/migrations/014_im_search_cjk.sql
-- Migration 014: Chinese / CJK Full-Text Search
-- ============================================================
-- Replaces the simple `to_tsvector('simple', ...)` trigger with
-- proper CJK tokenization using zhparser or pg_bigm extensions.
--
-- Strategy:
--   1. If zhparser is installed → use 'chinese_zh' text search config
--   2. If pg_bigm is installed  → use bigram-based similarity + GIN trigram index
--   3. Otherwise                  → keep 'simple' config (no CJK support)
--
-- Risk: LOW (non-destructive → only modifies the search trigger function)
-- ============================================================

-- ============================================================
-- Option A: zhparser (Chinese word segmentation)
-- ============================================================
-- zhparser provides Chinese word segmentation for PostgreSQL full-text search.
-- Installation: https://github.com/amutu/zhparser
--
-- After installing zhparser, run:
--   CREATE EXTENSION IF NOT EXISTS zhparser;
--   CREATE TEXT SEARCH CONFIGURATION chinese_zh (PARSER = zhparser);
--   ALTER TEXT SEARCH CONFIGURATION chinese_zh ADD MAPPING FOR n,v,a,i,e,l WITH simple;

-- ============================================================
-- Option B: pg_bigm / pg_trgm (bigram/trigram similarity)
-- ============================================================
-- pg_bigm provides 2-gram indexing for full-text search on CJK text.
-- pg_trgm ships with PostgreSQL and provides trigram matching.
--
-- After installing pg_bigm:
--   CREATE EXTENSION IF NOT EXISTS pg_bigm;
--   CREATE INDEX IF NOT EXISTS idx_im_messages_search_bigm
--       ON im_conversation_messages USING gin (payload_json_text gin_bigm_ops);
--
-- With pg_trgm (bundled with PostgreSQL):
--   CREATE EXTENSION IF NOT EXISTS pg_trgm;
--   CREATE INDEX IF NOT EXISTS idx_im_messages_search_trgm
--       ON im_conversation_messages USING gin (
--           (payload_json->>'text') gin_trgm_ops,
--           (payload_json->>'caption') gin_trgm_ops
--       );

-- ============================================================
-- Update the search trigger to handle Chinese text
-- ============================================================

CREATE OR REPLACE FUNCTION im_messages_search_trigger() RETURNS trigger AS $$
DECLARE
    raw_text text;
BEGIN
    raw_text := COALESCE(NEW.payload_json->>'text', '') || ' ' ||
                COALESCE(NEW.payload_json->>'caption', '') || ' ' ||
                COALESCE(NEW.payload_json->>'description', '');

    -- Use zhparser if available, otherwise fall back to simple
    -- (zhparser must be installed and 'chinese_zh' config created)
    BEGIN
        NEW.search_vector := to_tsvector('chinese_zh', raw_text);
    EXCEPTION WHEN OTHERS THEN
        -- Fallback: simple config (no CJK segmentation, but works for ASCII)
        NEW.search_vector := to_tsvector('simple', raw_text);
    END;

    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Recreate the trigger (replace the one from migration 012)
DROP TRIGGER IF EXISTS im_messages_search_update ON im_conversation_messages;
CREATE TRIGGER im_messages_search_update
    BEFORE INSERT OR UPDATE ON im_conversation_messages
    FOR EACH ROW EXECUTE FUNCTION im_messages_search_trigger();

-- ============================================================
-- CJK search index using pg_trgm (bundled with PostgreSQL 9.4+)
-- ============================================================
-- Provides fuzzy search for Chinese/Japanese/Korean without zhparser.
-- Enable with: CREATE EXTENSION IF NOT EXISTS pg_trgm;
--
-- CREATE INDEX IF NOT EXISTS idx_im_messages_search_cjk
--     ON im_conversation_messages USING gin (
--         (COALESCE(payload_json->>'text', '') || ' ' ||
--          COALESCE(payload_json->>'caption', '') || ' ' ||
--          COALESCE(payload_json->>'description', '')) gin_trgm_ops
--     )
--     WHERE deleted_at IS NULL;

-- ============================================================
-- 搜索架构说明
-- ============================================================
-- 默认使用 PostgreSQL 原生全文搜索。后续可通过 Provider 模式
-- （参考 PushProvider / RTC adapter）扩展为可插拔的搜索后端。
--
--   trait SearchProvider {
--       fn index_message(&self, message: &StoredMessageRecord) -> Result;
--       fn search(&self, tenant: &str, query: &str) -> Result<Vec<message_id>>;
--   }
--
-- PostgreSQL 实现即为本迁移的 search_vector + GIN 索引方案。
-- 如需切换到其他后端（如 Elasticsearch），实现 SearchProvider，
-- 通过 ProviderRegistry 切换即可，无需修改消息写入/查询路径。

-- ============================================================
-- Migration checklist (MIGRATION_SPEC §2):
--   id: MIG-2026-0014
--   type: database
--   strategy: expand-contract (new trigger coexists with old index)
--   rollback: revert trigger to 'simple' config
--   verification:
--     - SELECT to_tsvector('chinese_zh', '你好世界') @@ to_tsquery('chinese_zh', '世界');
--     - EXPLAIN ANALYZE SELECT * FROM im_conversation_messages WHERE search_vector @@ plainto_tsquery('chinese_zh', '你好');
-- ============================================================

-- source: database/migrations/postgres/0002_im_projection_metadata_snapshots.up.sql

CREATE TABLE IF NOT EXISTS im_projection_metadata_snapshots (
    snapshot_scope TEXT NOT NULL,
    snapshot_key TEXT NOT NULL,
    payload_json JSONB NOT NULL,
    payload_hash TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT pk_im_projection_metadata_snapshots PRIMARY KEY (snapshot_scope, snapshot_key)
);

CREATE INDEX IF NOT EXISTS idx_im_projection_metadata_snapshots_key
    ON im_projection_metadata_snapshots (snapshot_key);

-- ============================================================
-- RTC Lifecycle Tables (migrations 0008-0010 consolidated)
-- ============================================================

-- RTC Outbox Events
CREATE TABLE IF NOT EXISTS im_rtc_outbox_events (
    tenant_id           TEXT NOT NULL,
    organization_id     TEXT NOT NULL DEFAULT '0',
    outbox_id           TEXT NOT NULL,
    rtc_session_id      TEXT NOT NULL,
    event_id            TEXT NOT NULL,
    event_type          TEXT NOT NULL,
    actor_principal_kind TEXT NOT NULL,
    actor_principal_id  TEXT NOT NULL,
    payload_json        JSONB NOT NULL,
    payload_hash        TEXT NOT NULL,
    publish_status      TEXT NOT NULL DEFAULT 'pending',
    attempt_count       INTEGER NOT NULL DEFAULT 0 CHECK (attempt_count >= 0),
    available_at        TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    published_at        TIMESTAMPTZ,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    retention_until     TIMESTAMPTZ,
    CONSTRAINT pk_im_rtc_outbox_events PRIMARY KEY (tenant_id, organization_id, outbox_id),
    CONSTRAINT uk_im_rtc_outbox_events_event UNIQUE (tenant_id, organization_id, event_id),
    CONSTRAINT chk_im_rtc_outbox_events_status CHECK (publish_status IN ('pending', 'published', 'failed')),
    CONSTRAINT chk_im_rtc_outbox_events_type CHECK (event_type IN (
        'session.created', 'session.ringing', 'session.connected',
        'session.ended', 'session.canceled', 'session.rejected',
        'session.failed', 'session.timeout', 'session.hold', 'session.resumed',
        'participant.invited', 'participant.joined', 'participant.left',
        'participant.kicked', 'participant.credential_issued',
        'participant.credential_revoked',
        'recording.started', 'recording.stopped', 'recording.failed'
    ))
);

CREATE INDEX IF NOT EXISTS idx_im_rtc_outbox_events_status_available
    ON im_rtc_outbox_events (tenant_id, organization_id, publish_status, available_at, outbox_id);

CREATE INDEX IF NOT EXISTS idx_im_rtc_outbox_events_session
    ON im_rtc_outbox_events (tenant_id, organization_id, rtc_session_id, created_at);

CREATE INDEX IF NOT EXISTS idx_im_rtc_outbox_events_retention_until
    ON im_rtc_outbox_events (tenant_id, organization_id, retention_until)
    WHERE retention_until IS NOT NULL;

-- RTC Quality Reports
CREATE TABLE IF NOT EXISTS im_rtc_quality_reports (
    tenant_id               TEXT NOT NULL,
    organization_id         TEXT NOT NULL DEFAULT '0',
    report_id               TEXT NOT NULL,
    rtc_session_id          TEXT NOT NULL,
    participant_principal_kind TEXT NOT NULL,
    participant_principal_id   TEXT NOT NULL,
    participant_device_id     TEXT NOT NULL,
    reported_at             TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    mos_score               DOUBLE PRECISION CHECK (mos_score IS NULL OR (mos_score >= 1.0 AND mos_score <= 4.5)),
    rtt_ms                  DOUBLE PRECISION CHECK (rtt_ms IS NULL OR rtt_ms >= 0),
    jitter_ms               DOUBLE PRECISION CHECK (jitter_ms IS NULL OR jitter_ms >= 0),
    packet_loss_rate        DOUBLE PRECISION CHECK (packet_loss_rate IS NULL OR (packet_loss_rate >= 0 AND packet_loss_rate <= 1.0)),
    packets_sent            BIGINT CHECK (packets_sent IS NULL OR packets_sent >= 0),
    packets_received        BIGINT CHECK (packets_received IS NULL OR packets_received >= 0),
    packets_lost            BIGINT CHECK (packets_lost IS NULL OR packets_lost >= 0),
    bytes_sent              BIGINT CHECK (bytes_sent IS NULL OR bytes_sent >= 0),
    bytes_received          BIGINT CHECK (bytes_received IS NULL OR bytes_received >= 0),
    audio_bitrate_kbps      INTEGER CHECK (audio_bitrate_kbps IS NULL OR audio_bitrate_kbps >= 0),
    video_bitrate_kbps      INTEGER CHECK (video_bitrate_kbps IS NULL OR video_bitrate_kbps >= 0),
    audio_codec             TEXT,
    video_codec             TEXT,
    resolution_width        INTEGER CHECK (resolution_width IS NULL OR resolution_width >= 0),
    resolution_height       INTEGER CHECK (resolution_height IS NULL OR resolution_height >= 0),
    frame_rate_fps          DOUBLE PRECISION CHECK (frame_rate_fps IS NULL OR frame_rate_fps >= 0),
    quality_grade           TEXT CHECK (quality_grade IN ('excellent', 'good', 'fair', 'poor', 'bad')),
    payload_json            JSONB,
    payload_hash            TEXT,
    created_at              TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    retention_until         TIMESTAMPTZ,
    CONSTRAINT pk_im_rtc_quality_reports PRIMARY KEY (tenant_id, organization_id, report_id),
    CONSTRAINT uk_im_rtc_quality_reports_session_report UNIQUE (tenant_id, organization_id, rtc_session_id, participant_principal_kind, participant_principal_id, participant_device_id, reported_at)
);

CREATE INDEX IF NOT EXISTS idx_im_rtc_quality_reports_session_time
    ON im_rtc_quality_reports (tenant_id, organization_id, rtc_session_id, reported_at);

CREATE INDEX IF NOT EXISTS idx_im_rtc_quality_reports_participant
    ON im_rtc_quality_reports (tenant_id, organization_id, participant_principal_kind, participant_principal_id, reported_at);

CREATE INDEX IF NOT EXISTS idx_im_rtc_quality_reports_grade
    ON im_rtc_quality_reports (tenant_id, organization_id, quality_grade, reported_at)
    WHERE quality_grade IN ('poor', 'bad');

CREATE INDEX IF NOT EXISTS idx_im_rtc_quality_reports_retention_until
    ON im_rtc_quality_reports (tenant_id, organization_id, retention_until)
    WHERE retention_until IS NOT NULL;

-- RTC Participant Credentials
CREATE TABLE IF NOT EXISTS im_rtc_participant_credentials (
    tenant_id                   TEXT NOT NULL,
    organization_id             TEXT NOT NULL DEFAULT '0',
    credential_id               TEXT NOT NULL,
    rtc_session_id              TEXT NOT NULL,
    participant_principal_kind  TEXT NOT NULL,
    participant_principal_id    TEXT NOT NULL,
    participant_device_id       TEXT,
    provider_plugin_id          TEXT NOT NULL,
    provider_token_id           TEXT,
    credential_state            TEXT NOT NULL DEFAULT 'active',
    issued_at                   TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    expires_at                  TIMESTAMPTZ NOT NULL,
    rotated_from_credential_id  TEXT,
    rotated_at                  TIMESTAMPTZ,
    revoked_at                  TIMESTAMPTZ,
    revoked_reason              TEXT,
    revoked_by_principal_kind   TEXT,
    revoked_by_principal_id     TEXT,
    credential_payload_json     JSONB NOT NULL,
    credential_payload_hash     TEXT NOT NULL,
    created_at                  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at                  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    retention_until             TIMESTAMPTZ,
    CONSTRAINT pk_im_rtc_participant_credentials PRIMARY KEY (tenant_id, organization_id, credential_id),
    CONSTRAINT uk_im_rtc_participant_credentials_session_part UNIQUE (
        tenant_id, organization_id, rtc_session_id,
        participant_principal_kind, participant_principal_id, participant_device_id,
        credential_state
    ),
    CONSTRAINT chk_im_rtc_participant_credentials_state CHECK (credential_state IN (
        'active', 'expired', 'revoked', 'rotated'
    )),
    CONSTRAINT chk_im_rtc_participant_credentials_revocation CHECK (
        (credential_state = 'revoked') = (revoked_at IS NOT NULL)
    ),
    CONSTRAINT chk_im_rtc_participant_credentials_rotation CHECK (
        (credential_state = 'rotated') = (rotated_at IS NOT NULL AND rotated_from_credential_id IS NOT NULL)
    )
);

CREATE INDEX IF NOT EXISTS idx_im_rtc_participant_credentials_session
    ON im_rtc_participant_credentials (tenant_id, organization_id, rtc_session_id, participant_principal_kind, participant_principal_id);

CREATE INDEX IF NOT EXISTS idx_im_rtc_participant_credentials_active
    ON im_rtc_participant_credentials (tenant_id, organization_id, rtc_session_id, credential_state, expires_at)
    WHERE credential_state = 'active';

CREATE INDEX IF NOT EXISTS idx_im_rtc_participant_credentials_expiry
    ON im_rtc_participant_credentials (tenant_id, organization_id, expires_at)
    WHERE credential_state = 'active';

CREATE INDEX IF NOT EXISTS idx_im_rtc_participant_credentials_retention_until
    ON im_rtc_participant_credentials (tenant_id, organization_id, retention_until)
    WHERE retention_until IS NOT NULL;

