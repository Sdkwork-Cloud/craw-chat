-- SDKWork IM consolidated initialization baseline (SQLite)
-- SQLite-compatible schema for contract parity, desktop offline cache, and gateway webstore.
-- All JSONB columns are represented as TEXT with json_valid() CHECK constraints.
-- All TIMESTAMPTZ columns are represented as TEXT (ISO 8601 format).
--
-- Keep the folded baseline as one atomic SQLite batch. It contains trigger
-- bodies whose internal semicolons must not be split by the lifecycle runner.
BEGIN;







-- source: deployments/database/postgres/migrations/010_im_tenant_organization_isolation.sql
-- Migration 010: Tenant + Organization Dual Isolation
-- 为所�?im_* 业务表引�?organization_id，实现租�?组织双重隔离
-- 新应用零用户，直接重建终�?schema，不保留 001 迁移的兼容�?

-- ============================================================
-- 核心设计决策�?
-- 1. organization_id �?TEXT NOT NULL DEFAULT '0'
-- 2. 主键与索引统一前置 (tenant_id, organization_id, ...)
-- 3. 所有查询强制携�?organization_id 过滤
-- ============================================================

-- ============================================================
-- 1. 消息真值层
-- ============================================================

-- 重建 im_conversation_messages（消息真值表�?
-- 主键改为 Snowflake message_id，但保留 message_seq 作为会话内序�?
DROP TABLE IF EXISTS im_conversation_messages;
CREATE TABLE im_conversation_messages (
    tenant_id           TEXT NOT NULL,
    organization_id     TEXT NOT NULL DEFAULT '0',
    conversation_id     TEXT NOT NULL,
    message_id          INTEGER NOT NULL,           -- Snowflake ID，全局唯一
    message_seq         INTEGER NOT NULL,           -- 会话内严格递增
    sender_principal_kind TEXT NOT NULL,
    sender_principal_id TEXT NOT NULL,
    sender_device_id    TEXT,
    client_msg_id       TEXT,
    message_type        TEXT NOT NULL,
    payload_json TEXT NOT NULL CHECK (json_valid(payload_json)),
    payload_hash        TEXT NOT NULL,
    created_at           TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at           TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at           TEXT,
    retention_until     TEXT,
    CONSTRAINT pk_im_conversation_messages PRIMARY KEY (tenant_id, organization_id, conversation_id, message_seq),
    CONSTRAINT uk_im_conversation_messages_id UNIQUE (tenant_id, message_id),
    CONSTRAINT chk_im_conversation_messages_seq CHECK (message_seq > 0)
);

-- 客户端幂等键（会�?+ 发送�?+ client_msg_id 唯一�?
CREATE UNIQUE INDEX IF NOT EXISTS uk_im_conversation_messages_client
    ON im_conversation_messages (tenant_id, organization_id, conversation_id, sender_principal_kind, sender_principal_id, client_msg_id)
    WHERE client_msg_id IS NOT NULL;

-- timeline 读取索引
CREATE INDEX IF NOT EXISTS idx_im_messages_tenant_conv_seq
    ON im_conversation_messages (tenant_id, organization_id, conversation_id, message_seq DESC);

-- 发送者消息索�?
CREATE INDEX IF NOT EXISTS idx_im_messages_sender_created
    ON im_conversation_messages (tenant_id, organization_id, sender_principal_kind, sender_principal_id, created_at DESC);

-- retention 索引
CREATE INDEX IF NOT EXISTS idx_im_conversation_messages_retention_until
    ON im_conversation_messages (tenant_id, organization_id, retention_until)
    WHERE retention_until IS NOT NULL;

-- ============================================================
-- 2. 消息序号分配器（会话级原子）
-- ============================================================

DROP TABLE IF EXISTS im_conversation_seq_counters;
CREATE TABLE im_conversation_seq_counters (
    tenant_id       TEXT NOT NULL,
    organization_id TEXT NOT NULL DEFAULT '0',
    conversation_id TEXT NOT NULL,
    next_seq        INTEGER NOT NULL DEFAULT 1,
    updated_at      TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT pk_im_conversation_seq_counters PRIMARY KEY (tenant_id, organization_id, conversation_id),
    CONSTRAINT chk_im_conversation_seq_counters_seq CHECK (next_seq > 0)
);

-- ============================================================
-- 3. 消息媒体引用
-- ============================================================

DROP TABLE IF EXISTS im_message_media_refs;
CREATE TABLE im_message_media_refs (
    tenant_id TEXT NOT NULL,
    organization_id TEXT NOT NULL DEFAULT '0',
    conversation_id TEXT NOT NULL,
    message_seq INTEGER NOT NULL,
    message_id INTEGER NOT NULL,
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
    media_resource_snapshot TEXT NOT NULL CHECK (json_valid(media_resource_snapshot)),
    resource_hash TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    retention_until TEXT,
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
        size_bytes IS NULL OR size_bytes GLOB '[0-9]*'
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
-- 4. Outbox 事件表（重建，支�?-- PostgreSQL: FOR UPDATE SKIP LOCKED (not supported in SQLite)�?
-- ============================================================

DROP TABLE IF EXISTS im_outbox_events;
CREATE TABLE im_outbox_events (
    tenant_id TEXT NOT NULL,
    organization_id TEXT NOT NULL DEFAULT '0',
    outbox_id TEXT NOT NULL,              -- Snowflake ID
    aggregate_type TEXT NOT NULL,
    aggregate_id TEXT NOT NULL,
    event_id TEXT NOT NULL,
    event_type TEXT NOT NULL,
    payload_json TEXT NOT NULL CHECK (json_valid(payload_json)),
    payload_hash TEXT NOT NULL,
    publish_status TEXT NOT NULL DEFAULT 'pending',
    attempt_count INTEGER NOT NULL DEFAULT 0 CHECK (attempt_count >= 0),
    available_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    published_at TEXT,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    retention_until TEXT,
    CONSTRAINT pk_im_outbox_events PRIMARY KEY (tenant_id, organization_id, outbox_id),
    CONSTRAINT uk_im_outbox_events_event UNIQUE (tenant_id, organization_id, event_id),
    CONSTRAINT chk_im_outbox_events_publish_status CHECK (publish_status IN ('pending', 'published', 'failed'))
);

-- relay worker 用索引：-- PostgreSQL: FOR UPDATE SKIP LOCKED (not supported in SQLite)
CREATE INDEX IF NOT EXISTS idx_im_outbox_events_status_available
    ON im_outbox_events (tenant_id, organization_id, publish_status, available_at, outbox_id);

CREATE INDEX IF NOT EXISTS idx_im_outbox_events_retention_until
    ON im_outbox_events (tenant_id, organization_id, retention_until)
    WHERE retention_until IS NOT NULL;

-- ============================================================
-- 5. Inbox 事件表（消费幂等�?
-- ============================================================

DROP TABLE IF EXISTS im_inbox_events;
CREATE TABLE im_inbox_events (
    tenant_id TEXT NOT NULL,
    organization_id TEXT NOT NULL DEFAULT '0',
    inbox_id TEXT NOT NULL,
    source_system TEXT NOT NULL,
    source_event_id TEXT NOT NULL,
    consumer_name TEXT NOT NULL,
    event_type TEXT NOT NULL,
    payload_json TEXT NOT NULL CHECK (json_valid(payload_json)),
    payload_hash TEXT NOT NULL,
    process_status TEXT NOT NULL DEFAULT 'pending',
    received_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    processed_at TEXT,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    retention_until TEXT,
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
-- 6. Commit Journal（重建，offset 独立�?aggregate_seq�?
-- ============================================================

DROP TABLE IF EXISTS im_commit_journal;
CREATE TABLE im_commit_journal (
    partition_key TEXT NOT NULL,           -- (tenant_id:organization_id:aggregate_type:aggregate_id)
    commit_offset INTEGER NOT NULL,         -- Snowflake ID，全局唯一，非业务序号
    event_id TEXT NOT NULL,                -- Snowflake ID
    tenant_id TEXT NOT NULL,
    organization_id TEXT NOT NULL DEFAULT '0',
    aggregate_type TEXT NOT NULL,
    aggregate_id TEXT NOT NULL,
    aggregate_seq INTEGER NOT NULL CHECK (aggregate_seq > 0),  -- 业务聚合版本�?
    event_type TEXT NOT NULL,
    payload_json TEXT NOT NULL CHECK (json_valid(payload_json)),
    payload_hash TEXT NOT NULL,
    idempotency_key TEXT,
    occurred_at TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    retention_until TEXT,
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

DROP TABLE IF EXISTS im_idempotency_keys;
CREATE TABLE im_idempotency_keys (
    tenant_id TEXT NOT NULL,
    organization_id TEXT NOT NULL DEFAULT '0',
    request_scope TEXT NOT NULL,
    idempotency_key TEXT NOT NULL,
    request_hash TEXT NOT NULL,
    response_json TEXT NOT NULL CHECK (json_valid(response_json)),
    first_seen_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    last_seen_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    expires_at TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT pk_im_idempotency_keys PRIMARY KEY (tenant_id, organization_id, request_scope, idempotency_key)
);

CREATE INDEX IF NOT EXISTS idx_im_idempotency_keys_expires
    ON im_idempotency_keys (tenant_id, organization_id, expires_at);

-- ============================================================
-- 8. 实时设备事件
-- ============================================================

DROP TABLE IF EXISTS im_realtime_device_events;
CREATE TABLE im_realtime_device_events (
    tenant_id TEXT NOT NULL,
    organization_id TEXT NOT NULL DEFAULT '0',
    client_route_scope_key TEXT NOT NULL,
    realtime_seq INTEGER NOT NULL CHECK (realtime_seq > 0),
    principal_kind TEXT NOT NULL,
    principal_id TEXT NOT NULL,
    device_id TEXT NOT NULL,
    scope_type TEXT NOT NULL,
    scope_id TEXT NOT NULL,
    event_type TEXT NOT NULL,
    delivery_class TEXT NOT NULL,
    payload_json TEXT NOT NULL CHECK (json_valid(payload_json)),
    payload_hash TEXT NOT NULL,
    occurred_at TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    retention_until TEXT,
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

DROP TABLE IF EXISTS im_realtime_checkpoints;
CREATE TABLE im_realtime_checkpoints (
    tenant_id TEXT NOT NULL,
    organization_id TEXT NOT NULL DEFAULT '0',
    client_route_scope_key TEXT NOT NULL,
    principal_kind TEXT NOT NULL,
    principal_id TEXT NOT NULL,
    device_id TEXT NOT NULL,
    latest_realtime_seq INTEGER NOT NULL DEFAULT 0 CHECK (latest_realtime_seq >= 0),
    acked_through_seq INTEGER NOT NULL DEFAULT 0 CHECK (acked_through_seq >= 0),
    trimmed_through_seq INTEGER NOT NULL DEFAULT 0 CHECK (trimmed_through_seq >= 0),
    capacity_trimmed_event_count INTEGER NOT NULL DEFAULT 0 CHECK (capacity_trimmed_event_count >= 0),
    capacity_trimmed_through_seq INTEGER NOT NULL DEFAULT 0 CHECK (capacity_trimmed_through_seq >= 0),
    last_capacity_trimmed_at TEXT,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
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

DROP TABLE IF EXISTS im_realtime_subscriptions;
CREATE TABLE im_realtime_subscriptions (
    tenant_id TEXT NOT NULL,
    organization_id TEXT NOT NULL DEFAULT '0',
    client_route_scope_key TEXT NOT NULL,
    principal_kind TEXT NOT NULL,
    principal_id TEXT NOT NULL,
    device_id TEXT NOT NULL,
    subscriptions_json TEXT NOT NULL CHECK (json_valid(subscriptions_json)),
    subscription_count INTEGER NOT NULL DEFAULT 0 CHECK (subscription_count >= 0),
    synced_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    retention_until TEXT,
    CONSTRAINT pk_im_realtime_subscriptions PRIMARY KEY (tenant_id, organization_id, client_route_scope_key)
);

CREATE INDEX IF NOT EXISTS idx_im_realtime_subscriptions_principal
    ON im_realtime_subscriptions (tenant_id, organization_id, principal_kind, principal_id, device_id);

CREATE INDEX IF NOT EXISTS idx_im_realtime_subscriptions_synced_at
    ON im_realtime_subscriptions (tenant_id, organization_id, client_route_scope_key, synced_at);



CREATE INDEX IF NOT EXISTS idx_im_realtime_subscriptions_retention_until
    ON im_realtime_subscriptions (tenant_id, organization_id, retention_until)
    WHERE retention_until IS NOT NULL;

-- ============================================================
-- 11. 实时订阅范围
-- ============================================================

DROP TABLE IF EXISTS im_realtime_subscription_scopes;
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
    synced_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
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
-- 12. Presence 状�?
-- ============================================================

DROP TABLE IF EXISTS im_presence_states;
CREATE TABLE im_presence_states (
    tenant_id TEXT NOT NULL,
    organization_id TEXT NOT NULL DEFAULT '0',
    principal_kind TEXT NOT NULL,
    principal_id TEXT NOT NULL,
    device_id TEXT NOT NULL,
    session_id TEXT,
    presence_status TEXT NOT NULL,
    last_sync_seq INTEGER NOT NULL DEFAULT 0 CHECK (last_sync_seq >= 0),
    last_resume_at TEXT,
    last_seen_at TEXT,
    resume_required INTEGER NOT NULL DEFAULT 0,
    payload_json TEXT NOT NULL CHECK (json_valid(payload_json)),
    payload_hash TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    retention_until TEXT,
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

DROP TABLE IF EXISTS im_route_bindings;
CREATE TABLE im_route_bindings (
    tenant_id TEXT NOT NULL,
    organization_id TEXT NOT NULL DEFAULT '0',
    principal_kind TEXT NOT NULL,
    principal_id TEXT NOT NULL,
    device_id TEXT NOT NULL,
    owner_node_id TEXT NOT NULL,
    session_id TEXT,
    connection_kind TEXT NOT NULL,
    route_epoch INTEGER NOT NULL CHECK (route_epoch > 0),
    bound_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
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

DROP TABLE IF EXISTS im_realtime_disconnect_fences;
CREATE TABLE im_realtime_disconnect_fences (
    tenant_id TEXT NOT NULL,
    organization_id TEXT NOT NULL DEFAULT '0',
    principal_kind TEXT NOT NULL,
    principal_id TEXT NOT NULL,
    device_id TEXT NOT NULL,
    session_id TEXT,
    owner_node_id TEXT NOT NULL,
    disconnected_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    fence_token TEXT NOT NULL,
    payload_json TEXT NOT NULL CHECK (json_valid(payload_json)),
    payload_hash TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    retention_until TEXT,
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

DROP TABLE IF EXISTS im_rtc_sessions;
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
    latest_signal_seq INTEGER NOT NULL DEFAULT 0 CHECK (latest_signal_seq >= 0),
    signaling_stream_id TEXT,
    artifact_message_id TEXT,
    started_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    ended_at TEXT,
    payload_json TEXT NOT NULL CHECK (json_valid(payload_json)),
    payload_hash TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    retention_until TEXT,
    CONSTRAINT pk_im_rtc_sessions PRIMARY KEY (tenant_id, organization_id, rtc_session_id),
    CONSTRAINT chk_im_rtc_sessions_state CHECK (session_state IN (
        'started', 'accepted', 'rejected', 'ended',
        'initiating', 'ringing', 'connecting', 'connected',
        'on_hold', 'reconnecting', 'canceled', 'failed', 'timeout'
    ))
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

-- ============================================================
-- 16. RTC 信令
-- ============================================================

DROP TABLE IF EXISTS im_rtc_signals;
CREATE TABLE im_rtc_signals (
    tenant_id TEXT NOT NULL,
    organization_id TEXT NOT NULL DEFAULT '0',
    rtc_session_id TEXT NOT NULL,
    signal_seq INTEGER NOT NULL CHECK (signal_seq > 0),
    sender_principal_kind TEXT NOT NULL,
    sender_principal_id TEXT NOT NULL,
    receiver_principal_kind TEXT,
    receiver_principal_id TEXT,
    signal_type TEXT NOT NULL,
    payload_json TEXT NOT NULL CHECK (json_valid(payload_json)),
    payload_hash TEXT NOT NULL,
    occurred_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    retention_until TEXT,
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

-- ============================================================
-- 17. 审计记录
-- ============================================================

DROP TABLE IF EXISTS im_audit_records;
CREATE TABLE im_audit_records (
    tenant_id TEXT NOT NULL,
    organization_id TEXT NOT NULL DEFAULT '0',
    audit_seq INTEGER NOT NULL CHECK (audit_seq > 0),
    record_id TEXT NOT NULL,
    aggregate_type TEXT NOT NULL,
    aggregate_id TEXT NOT NULL,
    action TEXT NOT NULL,
    actor_id TEXT NOT NULL,
    actor_kind TEXT NOT NULL,
    actor_session_id TEXT,
    payload TEXT,
    recorded_at TEXT NOT NULL,
    occurred_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    target_type TEXT,
    target_id TEXT,
    retention_class TEXT NOT NULL DEFAULT 'access',
    integrity_anchor TEXT,
    integrity_anchored_at TEXT,
    chain_prev_hash TEXT,
    chain_hash TEXT NOT NULL,
    created_at TEXT NOT NULL,
    retention_until TEXT,
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

CREATE INDEX IF NOT EXISTS idx_im_audit_records_retention_until
    ON im_audit_records (tenant_id, organization_id, retention_until)
    WHERE retention_until IS NOT NULL;

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

-- ============================================================
-- 18. 通知任务
-- ============================================================

DROP TABLE IF EXISTS im_notification_tasks;
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
    payload_json TEXT NOT NULL CHECK (json_valid(payload_json)),
    payload_hash TEXT NOT NULL,
    requested_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    dispatched_at TEXT,
    failure_reason TEXT,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    retention_until TEXT,
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
-- 19. 自动化执�?
-- ============================================================

DROP TABLE IF EXISTS im_automation_executions;
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
    input_payload_json TEXT,
    input_payload_hash TEXT,
    output_payload_json TEXT,
    output_payload_hash TEXT,
    execution_state TEXT NOT NULL DEFAULT 'requested',
    retry_count INTEGER NOT NULL DEFAULT 0 CHECK (retry_count >= 0),
    requested_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    completed_at TEXT,
    failure_reason TEXT,
    payload_json TEXT NOT NULL CHECK (json_valid(payload_json)),
    payload_hash TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    retention_until TEXT,
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

-- ============================================================
-- 21. 投影：会话摘�?
-- ============================================================

DROP TABLE IF EXISTS im_conversations;
CREATE TABLE im_conversations (
    tenant_id TEXT NOT NULL,
    organization_id TEXT NOT NULL DEFAULT '0',
    conversation_id TEXT NOT NULL,
    conversation_type TEXT,
    message_count INTEGER NOT NULL DEFAULT 0 CHECK (message_count >= 0),
    last_message_id INTEGER,
    last_message_seq INTEGER NOT NULL DEFAULT 0 CHECK (last_message_seq >= 0),
    last_sender_kind TEXT,
    last_sender_id TEXT,
    last_summary TEXT,
    last_message_at TEXT,
    last_activity_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    agent_handoff_json TEXT,
    payload_json TEXT NOT NULL CHECK (json_valid(payload_json)),
    payload_hash TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    retention_until TEXT,
    CONSTRAINT pk_im_conversations PRIMARY KEY (tenant_id, organization_id, conversation_id)
);

CREATE INDEX IF NOT EXISTS idx_im_conversations_activity
    ON im_conversations (tenant_id, organization_id, last_activity_at DESC, conversation_id);

CREATE INDEX IF NOT EXISTS idx_im_conversations_retention_until
    ON im_conversations (tenant_id, organization_id, retention_until)
    WHERE retention_until IS NOT NULL;

-- ============================================================
-- 22. 投影：会话成�?
-- ============================================================

DROP TABLE IF EXISTS im_conversation_members;
CREATE TABLE im_conversation_members (
    tenant_id TEXT NOT NULL,
    organization_id TEXT NOT NULL DEFAULT '0',
    conversation_id TEXT NOT NULL,
    principal_kind TEXT NOT NULL,
    principal_id TEXT NOT NULL,
    member_id INTEGER NOT NULL,             -- Snowflake ID
    membership_role TEXT NOT NULL,
    membership_state TEXT NOT NULL,
    invited_by TEXT,
    joined_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    removed_at TEXT,
    attributes_json TEXT NOT NULL DEFAULT '{}',
    payload_json TEXT NOT NULL CHECK (json_valid(payload_json)),
    payload_hash TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    retention_until TEXT,
    -- member_id is the principal's Snowflake i64 (parsed from principal_id); principal already
    -- identifies a member uniquely within a conversation, so no separate UK on member_id is needed.
    -- See specs/database-table-registry.json writeOwner for ownership of this table.
    CONSTRAINT pk_im_conversation_members PRIMARY KEY (tenant_id, organization_id, conversation_id, principal_kind, principal_id),
    CONSTRAINT chk_im_conversation_members_state CHECK (membership_state IN ('invited', 'joined', 'linked', 'removed', 'left'))
);

CREATE INDEX IF NOT EXISTS idx_im_conversation_members_principal
    ON im_conversation_members (tenant_id, organization_id, principal_kind, principal_id, membership_state, conversation_id);

CREATE INDEX IF NOT EXISTS idx_im_conversation_members_active
    ON im_conversation_members (tenant_id, organization_id, conversation_id, principal_kind, principal_id)
    WHERE membership_state = 'joined';

CREATE INDEX IF NOT EXISTS idx_im_conversation_members_retention_until
    ON im_conversation_members (tenant_id, organization_id, retention_until)
    WHERE retention_until IS NOT NULL;

-- ============================================================
-- 23. 投影：已读游�?
-- ============================================================

DROP TABLE IF EXISTS im_conversation_read_cursors;
CREATE TABLE im_conversation_read_cursors (
    tenant_id TEXT NOT NULL,
    organization_id TEXT NOT NULL DEFAULT '0',
    conversation_id TEXT NOT NULL,
    member_id INTEGER NOT NULL,
    device_id TEXT NOT NULL DEFAULT '',
    principal_kind TEXT NOT NULL,
    principal_id TEXT NOT NULL,
    read_seq INTEGER NOT NULL DEFAULT 0 CHECK (read_seq >= 0),
    last_read_message_id INTEGER,
    payload_json TEXT NOT NULL CHECK (json_valid(payload_json)),
    payload_hash TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    retention_until TEXT,
    CONSTRAINT pk_im_conversation_read_cursors PRIMARY KEY (tenant_id, organization_id, conversation_id, member_id, device_id)
);

CREATE INDEX IF NOT EXISTS idx_im_conversation_read_cursors_principal
    ON im_conversation_read_cursors (tenant_id, organization_id, principal_kind, principal_id, conversation_id);

CREATE INDEX IF NOT EXISTS idx_im_conversation_read_cursors_retention_until
    ON im_conversation_read_cursors (tenant_id, organization_id, retention_until)
    WHERE retention_until IS NOT NULL;

-- ============================================================
-- 24. 投影：注册客户端路由
-- ============================================================

DROP TABLE IF EXISTS im_registered_client_routes;
CREATE TABLE im_registered_client_routes (
    tenant_id TEXT NOT NULL,
    organization_id TEXT NOT NULL DEFAULT '0',
    principal_kind TEXT NOT NULL,
    principal_id TEXT NOT NULL,
    device_id TEXT NOT NULL,
    registered_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    payload_json TEXT NOT NULL CHECK (json_valid(payload_json)),
    payload_hash TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    retention_until TEXT,
    CONSTRAINT pk_im_registered_client_routes PRIMARY KEY (tenant_id, organization_id, principal_kind, principal_id, device_id)
);

CREATE INDEX IF NOT EXISTS idx_im_registered_client_routes_retention_until
    ON im_registered_client_routes (tenant_id, organization_id, retention_until)
    WHERE retention_until IS NOT NULL;

-- ============================================================
-- 25. 投影：客户端路由同步 Feed
-- ============================================================

DROP TABLE IF EXISTS im_client_sync_events;
CREATE TABLE im_client_sync_events (
    tenant_id TEXT NOT NULL,
    organization_id TEXT NOT NULL DEFAULT '0',
    principal_kind TEXT NOT NULL,
    principal_id TEXT NOT NULL,
    device_id TEXT NOT NULL,
    sync_seq INTEGER NOT NULL CHECK (sync_seq > 0),
    origin_event_id TEXT NOT NULL,
    origin_event_type TEXT NOT NULL,
    conversation_id TEXT,
    message_id INTEGER,
    message_seq INTEGER CHECK (message_seq IS NULL OR message_seq > 0),
    member_id INTEGER,
    read_seq INTEGER CHECK (read_seq IS NULL OR read_seq >= 0),
    last_read_message_id INTEGER,
    actor_kind TEXT,
    actor_id TEXT,
    actor_device_id TEXT,
    summary TEXT,
    payload_json TEXT NOT NULL CHECK (json_valid(payload_json)),
    payload_hash TEXT NOT NULL,
    occurred_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    retention_until TEXT,
    CONSTRAINT pk_im_client_sync_events PRIMARY KEY (tenant_id, organization_id, principal_kind, principal_id, device_id, sync_seq)
);

CREATE INDEX IF NOT EXISTS idx_im_client_sync_events_window
    ON im_client_sync_events (tenant_id, organization_id, principal_kind, principal_id, device_id, sync_seq);

CREATE INDEX IF NOT EXISTS idx_im_client_sync_events_conversation
    ON im_client_sync_events (tenant_id, organization_id, conversation_id, sync_seq)
    WHERE conversation_id IS NOT NULL;

CREATE INDEX IF NOT EXISTS idx_im_client_sync_events_retention_until
    ON im_client_sync_events (tenant_id, organization_id, retention_until)
    WHERE retention_until IS NOT NULL;

-- ============================================================
-- 26. 投影：客户端路由同步检查点
-- ============================================================

DROP TABLE IF EXISTS im_client_sync_cursors;
CREATE TABLE im_client_sync_cursors (
    tenant_id TEXT NOT NULL,
    organization_id TEXT NOT NULL DEFAULT '0',
    principal_kind TEXT NOT NULL,
    principal_id TEXT NOT NULL,
    device_id TEXT NOT NULL,
    latest_sync_seq INTEGER NOT NULL DEFAULT 0 CHECK (latest_sync_seq >= 0),
    trimmed_through_seq INTEGER NOT NULL DEFAULT 0 CHECK (trimmed_through_seq >= 0),
    payload_json TEXT NOT NULL CHECK (json_valid(payload_json)),
    payload_hash TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    retention_until TEXT,
    CONSTRAINT pk_im_client_sync_cursors PRIMARY KEY (tenant_id, organization_id, principal_kind, principal_id, device_id),
    CONSTRAINT chk_im_client_sync_cursors_order CHECK (trimmed_through_seq <= latest_sync_seq)
);

CREATE INDEX IF NOT EXISTS idx_im_client_sync_cursors_retention_until
    ON im_client_sync_cursors (tenant_id, organization_id, retention_until)
    WHERE retention_until IS NOT NULL;

-- ============================================================
-- 27. 投影：联系人
-- ============================================================

-- ============================================================
-- 28. 投影：直接聊天绑�?
-- ============================================================

-- ============================================================
-- 29. Stream Sessions
-- ============================================================

DROP TABLE IF EXISTS im_stream_sessions;
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
    last_frame_seq INTEGER NOT NULL DEFAULT 0 CHECK (last_frame_seq >= 0),
    last_checkpoint_seq INTEGER CHECK (last_checkpoint_seq >= 0),
    result_message_id INTEGER,
    complete_frame_seq INTEGER CHECK (complete_frame_seq >= 0),
    abort_frame_seq INTEGER CHECK (abort_frame_seq >= 0),
    abort_reason TEXT,
    opened_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    closed_at TEXT,
    expires_at TEXT,
    payload_json TEXT NOT NULL CHECK (json_valid(payload_json)),
    payload_hash TEXT NOT NULL,
    version INTEGER NOT NULL DEFAULT 1 CHECK (version > 0),
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    retention_until TEXT,
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

CREATE INDEX IF NOT EXISTS idx_im_stream_sessions_active
    ON im_stream_sessions (tenant_id, organization_id)
    WHERE stream_state NOT IN ('completed', 'aborted', 'expired');

CREATE INDEX IF NOT EXISTS idx_im_stream_sessions_retention_until
    ON im_stream_sessions (tenant_id, organization_id, retention_until)
    WHERE retention_until IS NOT NULL;

-- ============================================================
-- 30. Stream Frames
-- ============================================================

DROP TABLE IF EXISTS im_stream_frames;
CREATE TABLE im_stream_frames (
    tenant_id TEXT NOT NULL,
    organization_id TEXT NOT NULL DEFAULT '0',
    stream_id TEXT NOT NULL,
    frame_seq INTEGER NOT NULL CHECK (frame_seq > 0),
    producer_principal_kind TEXT NOT NULL,
    producer_principal_id TEXT NOT NULL,
    schema_ref TEXT,
    payload_json TEXT NOT NULL CHECK (json_valid(payload_json)),
    payload_hash TEXT NOT NULL,
    occurred_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    retention_until TEXT,
    CONSTRAINT pk_im_stream_frames PRIMARY KEY (tenant_id, organization_id, stream_id, frame_seq)
);

CREATE INDEX IF NOT EXISTS idx_im_stream_frames_stream_seq
    ON im_stream_frames (tenant_id, organization_id, stream_id, frame_seq);

CREATE INDEX IF NOT EXISTS idx_im_stream_frames_retention_until
    ON im_stream_frames (tenant_id, organization_id, retention_until)
    WHERE retention_until IS NOT NULL;

-- source: deployments/database/postgres/migrations/012_im_social_org_interactions.sql
-- Migration 012: Social Relations, Organization Model, Message Interactions
-- 对齐行业最专业 IM（微�?Telegram/Discord/Slack）的数据库设�?
-- 所�?ID 统一使用 Snowflake ID (INTEGER)

-- ============================================================
-- 设计原则�?
-- 1. 所有主�?ID 使用 Snowflake INTEGER
-- 2. 租户和用户引�?IAM 系统（iam_tenant, iam_user�?
-- 3. 组织模型（Space/Group/Channel）是 IM 专有
-- 4. 社交关系独立持久化，不依赖内�?事件溯源
-- 5. 消息互动（Reaction/Pin/Thread）独立表
-- ============================================================

-- ============================================================
-- 第一部分：社交关系真值表
-- ============================================================

-- 1. 好友请求�?
CREATE TABLE IF NOT EXISTS im_friend_requests (
    tenant_id           TEXT NOT NULL,
    organization_id     TEXT NOT NULL DEFAULT '0',
    request_id          INTEGER NOT NULL,           -- Snowflake ID
    requester_user_id   TEXT NOT NULL,              -- 引用 iam_user.user_id
    target_user_id      TEXT NOT NULL,              -- 引用 iam_user.user_id
    request_message     TEXT,
    status              TEXT NOT NULL DEFAULT 'pending',
    expired_at          TEXT,
    created_at          TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at          TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
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

-- 2. 好友关系�?
CREATE TABLE IF NOT EXISTS im_friendships (
    tenant_id           TEXT NOT NULL,
    organization_id     TEXT NOT NULL DEFAULT '0',
    friendship_id       INTEGER NOT NULL,           -- Snowflake ID
    user_low_id         TEXT NOT NULL,              -- 规范化：较小�?user_id
    user_high_id        TEXT NOT NULL,              -- 规范化：较大�?user_id
    initiator_user_id   TEXT NOT NULL,              -- 发起好友请求的用�?
    status              TEXT NOT NULL DEFAULT 'active',
    established_at      TEXT,
    updated_at          TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT pk_im_friendships PRIMARY KEY (tenant_id, organization_id, friendship_id),
    CONSTRAINT uk_im_friendships_pair UNIQUE (tenant_id, organization_id, user_low_id, user_high_id),
    CONSTRAINT chk_im_friendships_status CHECK (status IN ('active', 'removed')),
    CONSTRAINT chk_im_friendships_not_self CHECK (user_low_id < user_high_id)
);

CREATE INDEX IF NOT EXISTS idx_im_friendships_user_low
    ON im_friendships (tenant_id, organization_id, user_low_id, status, established_at DESC);

CREATE INDEX IF NOT EXISTS idx_im_friendships_user_high
    ON im_friendships (tenant_id, organization_id, user_high_id, status, established_at DESC);

-- 3. 用户屏蔽�?
CREATE TABLE IF NOT EXISTS im_user_blocks (
    tenant_id           TEXT NOT NULL,
    organization_id     TEXT NOT NULL DEFAULT '0',
    block_id            INTEGER NOT NULL,           -- Snowflake ID
    blocker_user_id     TEXT NOT NULL,              -- 屏蔽�?
    blocked_user_id     TEXT NOT NULL,              -- 被屏蔽�?
    scope               TEXT NOT NULL DEFAULT 'all',
    direct_chat_id      INTEGER,                    -- �?direct_chat 作用�?
    reason              TEXT,
    expires_at          TEXT,
    created_at          TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at          TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
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

-- 4. 单聊会话�?
CREATE TABLE IF NOT EXISTS im_direct_chats (
    tenant_id           TEXT NOT NULL,
    organization_id     TEXT NOT NULL DEFAULT '0',
    direct_chat_id      INTEGER NOT NULL,           -- Snowflake ID
    left_actor_kind     TEXT NOT NULL,
    left_actor_id       TEXT NOT NULL,
    right_actor_kind    TEXT NOT NULL,
    right_actor_id      TEXT NOT NULL,
    pair_hash           TEXT NOT NULL,              -- 规范化后的哈�?
    status              TEXT NOT NULL DEFAULT 'active',
    conversation_id     TEXT,                       -- 关联的会�?ID
    created_at          TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at          TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
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

-- 5. 外部连接�?
CREATE TABLE IF NOT EXISTS im_external_connections (
    tenant_id           TEXT NOT NULL,
    organization_id     TEXT NOT NULL DEFAULT '0',
    connection_id       INTEGER NOT NULL,           -- Snowflake ID
    external_tenant_id  TEXT NOT NULL,
    external_org_name   TEXT,
    connection_kind     TEXT NOT NULL DEFAULT 'shared_channel',
    status              TEXT NOT NULL DEFAULT 'active',
    established_at      TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at          TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT pk_im_external_connections PRIMARY KEY (tenant_id, organization_id, connection_id),
    CONSTRAINT uk_im_external_connections_pair UNIQUE (tenant_id, organization_id, external_tenant_id),
    CONSTRAINT chk_im_external_connections_kind CHECK (connection_kind IN ('shared_channel')),
    CONSTRAINT chk_im_external_connections_status CHECK (status IN ('active', 'suspended', 'revoked')),
    CONSTRAINT chk_im_external_connections_not_self CHECK (tenant_id != external_tenant_id)
);

-- 6. 外部成员链接�?
CREATE TABLE IF NOT EXISTS im_external_member_links (
    tenant_id               TEXT NOT NULL,
    organization_id         TEXT NOT NULL DEFAULT '0',
    link_id                 INTEGER NOT NULL,           -- Snowflake ID
    connection_id           INTEGER NOT NULL,
    local_actor_kind        TEXT NOT NULL,
    local_actor_id          TEXT NOT NULL,
    external_member_id      TEXT NOT NULL,
    external_display_name   TEXT,
    status                  TEXT NOT NULL DEFAULT 'active',
    linked_at               TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at              TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT pk_im_external_member_links PRIMARY KEY (tenant_id, organization_id, link_id),
    CONSTRAINT uk_im_external_member_links_mapping UNIQUE (tenant_id, organization_id, connection_id, local_actor_id, external_member_id),
    CONSTRAINT chk_im_external_member_links_status CHECK (status IN ('active', 'revoked'))
);

CREATE INDEX IF NOT EXISTS idx_im_external_member_links_connection
    ON im_external_member_links (tenant_id, organization_id, connection_id, status);

CREATE INDEX IF NOT EXISTS idx_im_external_member_links_local_actor
    ON im_external_member_links (tenant_id, organization_id, local_actor_kind, local_actor_id, status);

-- 7. 共享频道策略�?
CREATE TABLE IF NOT EXISTS im_shared_channel_policies (
    tenant_id               TEXT NOT NULL,
    organization_id         TEXT NOT NULL DEFAULT '0',
    policy_id               INTEGER NOT NULL,           -- Snowflake ID
    connection_id           INTEGER NOT NULL,
    channel_id              TEXT NOT NULL,
    conversation_id         TEXT,
    policy_version          INTEGER NOT NULL DEFAULT 1,
    history_visibility      TEXT NOT NULL DEFAULT 'shared',
    status                  TEXT NOT NULL DEFAULT 'active',
    applied_at              TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at              TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT pk_im_shared_channel_policies PRIMARY KEY (tenant_id, organization_id, policy_id),
    CONSTRAINT uk_im_shared_channel_policies_target UNIQUE (tenant_id, organization_id, connection_id, channel_id),
    CONSTRAINT chk_im_shared_channel_policies_visibility CHECK (history_visibility IN ('shared', 'isolated')),
    CONSTRAINT chk_im_shared_channel_policies_status CHECK (status IN ('active', 'suspended'))
);

CREATE INDEX IF NOT EXISTS idx_im_shared_channel_policies_connection
    ON im_shared_channel_policies (tenant_id, organization_id, connection_id, status);

-- ============================================================
-- 第二部分：组织模型（IM 专有�?
-- ============================================================

-- 8. 空间/组织�?
CREATE TABLE IF NOT EXISTS im_spaces (
    tenant_id           TEXT NOT NULL,
    organization_id     TEXT NOT NULL DEFAULT '0',
    space_id            INTEGER NOT NULL,           -- Snowflake ID
    space_name          TEXT NOT NULL,
    space_type          TEXT NOT NULL DEFAULT 'organization',
    owner_user_id       TEXT NOT NULL,              -- 引用 iam_user.user_id
    description         TEXT,
    avatar_url          TEXT,
    max_members         INTEGER NOT NULL DEFAULT 10000,
    settings_json       TEXT NOT NULL DEFAULT '{}',
    created_at          TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at          TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT pk_im_spaces PRIMARY KEY (tenant_id, organization_id, space_id),
    CONSTRAINT chk_im_spaces_type CHECK (space_type IN ('organization', 'team', 'project', 'community'))
);

CREATE INDEX IF NOT EXISTS idx_im_spaces_owner
    ON im_spaces (tenant_id, organization_id, owner_user_id, created_at DESC);

CREATE INDEX IF NOT EXISTS idx_im_spaces_type
    ON im_spaces (tenant_id, organization_id, space_type, created_at DESC);

-- 9. 空间成员�?
CREATE TABLE IF NOT EXISTS im_space_members (
    tenant_id           TEXT NOT NULL,
    organization_id     TEXT NOT NULL DEFAULT '0',
    space_id            INTEGER NOT NULL,
    user_id             TEXT NOT NULL,              -- 引用 iam_user.user_id
    role                TEXT NOT NULL DEFAULT 'member',
    nickname            TEXT,
    joined_at           TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at          TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT pk_im_space_members PRIMARY KEY (tenant_id, organization_id, space_id, user_id),
    CONSTRAINT chk_im_space_members_role CHECK (role IN ('owner', 'admin', 'member', 'guest'))
);

CREATE INDEX IF NOT EXISTS idx_im_space_members_user
    ON im_space_members (tenant_id, organization_id, user_id, role);

-- 10. 群组�?
CREATE TABLE IF NOT EXISTS im_chat_groups (
    tenant_id           TEXT NOT NULL,
    organization_id     TEXT NOT NULL DEFAULT '0',
    group_id            INTEGER NOT NULL,           -- Snowflake ID
    space_id            INTEGER,                    -- 所属空间（可选）
    group_name          TEXT NOT NULL,
    group_type          TEXT NOT NULL DEFAULT 'normal',
    owner_user_id       TEXT NOT NULL,              -- 引用 iam_user.user_id
    conversation_id     TEXT,                       -- 关联的会�?ID
    max_members         INTEGER NOT NULL DEFAULT 500,
    description         TEXT,
    avatar_url          TEXT,
    announcement        TEXT,                       -- 群公�?
    settings_json       TEXT NOT NULL DEFAULT '{}',
    created_at          TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at          TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
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

-- 11. 群组成员�?
CREATE TABLE IF NOT EXISTS im_group_members (
    tenant_id           TEXT NOT NULL,
    organization_id     TEXT NOT NULL DEFAULT '0',
    group_id            INTEGER NOT NULL,
    user_id             TEXT NOT NULL,              -- 引用 iam_user.user_id
    role                TEXT NOT NULL DEFAULT 'member',
    nickname            TEXT,                       -- 群内昵称
    mute_until          TEXT,               -- 禁言截止时间
    joined_at           TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at          TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT pk_im_group_members PRIMARY KEY (tenant_id, organization_id, group_id, user_id),
    CONSTRAINT chk_im_group_members_role CHECK (role IN ('owner', 'admin', 'member', 'muted'))
);

CREATE INDEX IF NOT EXISTS idx_im_group_members_user
    ON im_group_members (tenant_id, organization_id, user_id, role);

CREATE INDEX IF NOT EXISTS idx_im_group_members_role
    ON im_group_members (tenant_id, organization_id, group_id, role, joined_at);

-- 12. 频道�?
CREATE TABLE IF NOT EXISTS im_chat_channels (
    tenant_id           TEXT NOT NULL,
    organization_id     TEXT NOT NULL DEFAULT '0',
    channel_id          INTEGER NOT NULL,           -- Snowflake ID
    space_id            INTEGER NOT NULL,
    channel_name        TEXT NOT NULL,
    channel_type        TEXT NOT NULL DEFAULT 'text',
    description         TEXT,
    conversation_id     TEXT,                       -- 关联的会�?ID
    position            INTEGER NOT NULL DEFAULT 0,
    is_nsfw             INTEGER NOT NULL DEFAULT 0,
    is_pinned           INTEGER NOT NULL DEFAULT 0,
    topic               TEXT,                       -- 频道话题
    settings_json       TEXT NOT NULL DEFAULT '{}',
    created_at          TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at          TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT pk_im_chat_channels PRIMARY KEY (tenant_id, organization_id, channel_id),
    CONSTRAINT chk_im_chat_channels_type CHECK (channel_type IN ('text', 'voice', 'announcement', 'forum'))
);

CREATE INDEX IF NOT EXISTS idx_im_chat_channels_space
    ON im_chat_channels (tenant_id, organization_id, space_id, position, channel_name);

CREATE INDEX IF NOT EXISTS idx_im_chat_channels_conversation
    ON im_chat_channels (tenant_id, organization_id, conversation_id)
    WHERE conversation_id IS NOT NULL;

-- 13. 频道访问规则�?
CREATE TABLE IF NOT EXISTS im_channel_access_rules (
    tenant_id           TEXT NOT NULL,
    organization_id     TEXT NOT NULL DEFAULT '0',
    rule_id             INTEGER NOT NULL,           -- Snowflake ID
    channel_id          INTEGER NOT NULL,
    rule_type           TEXT NOT NULL,
    principal_kind      TEXT,                       -- user/role/group
    principal_id        TEXT,
    permission          TEXT NOT NULL,              -- view/send/manage
    created_at          TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT pk_im_channel_access_rules PRIMARY KEY (tenant_id, organization_id, rule_id),
    CONSTRAINT uk_im_channel_access_rules_target UNIQUE (tenant_id, organization_id, channel_id, rule_type, principal_kind, principal_id, permission),
    CONSTRAINT chk_im_channel_access_rules_type CHECK (rule_type IN ('allow', 'deny'))
);

CREATE INDEX IF NOT EXISTS idx_im_channel_access_rules_channel
    ON im_channel_access_rules (tenant_id, organization_id, channel_id, rule_type);

-- ============================================================
-- 第三部分：消息互动表
-- ============================================================

-- 14. 消息 Reaction �?
CREATE TABLE IF NOT EXISTS im_message_reactions (
    tenant_id           TEXT NOT NULL,
    organization_id     TEXT NOT NULL DEFAULT '0',
    conversation_id     TEXT NOT NULL,
    message_id          INTEGER NOT NULL,
    user_id             TEXT NOT NULL,              -- 引用 iam_user.user_id
    reaction_type       TEXT NOT NULL,              -- emoji 类型（如 👍, ❤️, 😂�?
    created_at          TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT pk_im_message_reactions PRIMARY KEY (tenant_id, organization_id, conversation_id, message_id, user_id, reaction_type)
);

CREATE INDEX IF NOT EXISTS idx_im_message_reactions_message
    ON im_message_reactions (tenant_id, organization_id, conversation_id, message_id, reaction_type);

CREATE INDEX IF NOT EXISTS idx_im_message_reactions_user
    ON im_message_reactions (tenant_id, organization_id, user_id, created_at DESC);

-- 15. 消息 Pin �?
CREATE TABLE IF NOT EXISTS im_message_pins (
    tenant_id           TEXT NOT NULL,
    organization_id     TEXT NOT NULL DEFAULT '0',
    conversation_id     TEXT NOT NULL,
    message_id          INTEGER NOT NULL,
    pinned_by_user_id   TEXT NOT NULL,              -- 引用 iam_user.user_id
    pin_reason          TEXT,
    pinned_at           TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT pk_im_message_pins PRIMARY KEY (tenant_id, organization_id, conversation_id, message_id)
);

CREATE INDEX IF NOT EXISTS idx_im_message_pins_conversation
    ON im_message_pins (tenant_id, organization_id, conversation_id, pinned_at DESC);

CREATE INDEX IF NOT EXISTS idx_im_message_pins_user
    ON im_message_pins (tenant_id, organization_id, pinned_by_user_id, pinned_at DESC);

-- 16. Thread �?
CREATE TABLE IF NOT EXISTS im_threads (
    tenant_id           TEXT NOT NULL,
    organization_id     TEXT NOT NULL DEFAULT '0',
    thread_id           INTEGER NOT NULL,           -- Snowflake ID
    conversation_id     TEXT NOT NULL,
    root_message_id     INTEGER NOT NULL,
    thread_title        TEXT,
    reply_count         INTEGER NOT NULL DEFAULT 0 CHECK (reply_count >= 0),
    last_reply_at       TEXT,
    last_reply_user_id  TEXT,
    created_at          TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at          TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT pk_im_threads PRIMARY KEY (tenant_id, organization_id, thread_id),
    CONSTRAINT uk_im_threads_root UNIQUE (tenant_id, organization_id, conversation_id, root_message_id)
);

CREATE INDEX IF NOT EXISTS idx_im_threads_conversation
    ON im_threads (tenant_id, organization_id, conversation_id, last_reply_at DESC);

CREATE INDEX IF NOT EXISTS idx_im_threads_root_message
    ON im_threads (tenant_id, organization_id, root_message_id);

-- 17. Thread 订阅�?
CREATE TABLE IF NOT EXISTS im_thread_subscriptions (
    tenant_id           TEXT NOT NULL,
    organization_id     TEXT NOT NULL DEFAULT '0',
    thread_id           INTEGER NOT NULL,
    user_id             TEXT NOT NULL,              -- 引用 iam_user.user_id
    last_read_seq       INTEGER NOT NULL DEFAULT 0,
    notification_level  TEXT NOT NULL DEFAULT 'all',
    subscribed_at       TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at          TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT pk_im_thread_subscriptions PRIMARY KEY (tenant_id, organization_id, thread_id, user_id),
    CONSTRAINT chk_im_thread_subscriptions_level CHECK (notification_level IN ('all', 'mentions', 'none'))
);

CREATE INDEX IF NOT EXISTS idx_im_thread_subscriptions_user
    ON im_thread_subscriptions (tenant_id, organization_id, user_id, subscribed_at DESC);

-- ============================================================
-- 第四部分：IM 用户扩展�?
-- ============================================================

-- 18. IM 用户资料扩展�?
CREATE TABLE IF NOT EXISTS im_user_profiles (
    tenant_id               TEXT NOT NULL,
    organization_id         TEXT NOT NULL DEFAULT '0',
    user_id                 TEXT NOT NULL,              -- 引用 iam_user.user_id
    im_nickname             TEXT,                       -- IM 专属昵称
    im_avatar_url           TEXT,                       -- IM 专属头像
    im_status_message       TEXT,                       -- 状态消�?
    im_notification_prefs   TEXT NOT NULL DEFAULT '{}', -- 通知偏好
    im_mute_settings        TEXT NOT NULL DEFAULT '{}', -- 免打扰设�?
    im_privacy_settings     TEXT NOT NULL DEFAULT '{}', -- 隐私设置
    im_online_status        TEXT NOT NULL DEFAULT 'online',
    last_active_at          TEXT,
    created_at              TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at              TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT pk_im_user_profiles PRIMARY KEY (tenant_id, organization_id, user_id),
    CONSTRAINT chk_im_user_profiles_online_status CHECK (im_online_status IN ('online', 'away', 'busy', 'invisible', 'offline'))
);

-- 19. 用户设置�?
CREATE TABLE IF NOT EXISTS im_user_settings (
    tenant_id           TEXT NOT NULL,
    organization_id     TEXT NOT NULL DEFAULT '0',
    user_id             TEXT NOT NULL,
    setting_key         TEXT NOT NULL,
    setting_value       TEXT NOT NULL,
    updated_at          TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT pk_im_user_settings PRIMARY KEY (tenant_id, organization_id, user_id, setting_key)
);

-- 20. 会话设置表（用户对特定会话的设置�?
CREATE TABLE IF NOT EXISTS im_conversation_settings (
    tenant_id           TEXT NOT NULL,
    organization_id     TEXT NOT NULL DEFAULT '0',
    conversation_id     TEXT NOT NULL,
    user_id             TEXT NOT NULL,
    is_muted            INTEGER NOT NULL DEFAULT 0,
    mute_until          TEXT,
    is_pinned           INTEGER NOT NULL DEFAULT 0,
    is_archived         INTEGER NOT NULL DEFAULT 0,
    is_blocked          INTEGER NOT NULL DEFAULT 0,
    notification_level  TEXT NOT NULL DEFAULT 'all',
    custom_name         TEXT,                       -- 用户自定义会话名�?
    settings_json       TEXT NOT NULL DEFAULT '{}',
    updated_at          TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT pk_im_conversation_settings PRIMARY KEY (tenant_id, organization_id, conversation_id, user_id),
    CONSTRAINT chk_im_conversation_settings_notification CHECK (notification_level IN ('all', 'mentions', 'none'))
);

CREATE INDEX IF NOT EXISTS idx_im_conversation_settings_user
    ON im_conversation_settings (tenant_id, organization_id, user_id, is_pinned DESC, updated_at DESC);

-- ============================================================
-- 第五部分：消息搜索索�?
-- ============================================================

-- 21. 消息搜索向量�?
-- ============================================================
-- Part 5: Message search (SQLite contract parity)
-- ============================================================
-- IM message search runtime authority is PostgreSQL-only (tsvector + GIN).
-- SQLite baseline keeps search_text for bootstrap parity; production IM
-- services require PostgreSQL (see database/README.md).

ALTER TABLE im_conversation_messages ADD COLUMN search_text TEXT;

CREATE INDEX IF NOT EXISTS idx_im_messages_search_text
    ON im_conversation_messages (tenant_id, organization_id, conversation_id, search_text)
    WHERE deleted_at IS NULL AND search_text IS NOT NULL;

CREATE TRIGGER IF NOT EXISTS im_messages_search_text_insert
AFTER INSERT ON im_conversation_messages
BEGIN
    UPDATE im_conversation_messages
    SET search_text = trim(
        coalesce(json_extract(NEW.payload_json, '$.text'), '') || ' ' ||
        coalesce(json_extract(NEW.payload_json, '$.caption'), '') || ' ' ||
        coalesce(json_extract(NEW.payload_json, '$.description'), '')
    )
    WHERE rowid = NEW.rowid;
END;

CREATE TRIGGER IF NOT EXISTS im_messages_search_text_update
AFTER UPDATE OF payload_json ON im_conversation_messages
BEGIN
    UPDATE im_conversation_messages
    SET search_text = trim(
        coalesce(json_extract(NEW.payload_json, '$.text'), '') || ' ' ||
        coalesce(json_extract(NEW.payload_json, '$.caption'), '') || ' ' ||
        coalesce(json_extract(NEW.payload_json, '$.description'), '')
    )
    WHERE rowid = NEW.rowid;
END;


-- ============================================================
-- Part 6: Invitations and bans
-- ============================================================

CREATE TABLE IF NOT EXISTS im_invitations (
    tenant_id           TEXT NOT NULL,
    organization_id     TEXT NOT NULL DEFAULT '0',
    invitation_id       INTEGER NOT NULL,           -- Snowflake ID
    inviter_user_id     TEXT NOT NULL,
    invitee_user_id     TEXT,
    invitee_email       TEXT,
    invitee_phone       TEXT,
    target_type         TEXT NOT NULL,              -- space/group/channel
    target_id           INTEGER NOT NULL,
    role                TEXT NOT NULL DEFAULT 'member',
    status              TEXT NOT NULL DEFAULT 'pending',
    message             TEXT,
    expires_at          TEXT,
    accepted_at         TEXT,
    created_at          TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at          TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT pk_im_invitations PRIMARY KEY (tenant_id, organization_id, invitation_id),
    CONSTRAINT chk_im_invitations_target_type CHECK (target_type IN ('space', 'group', 'channel')),
    CONSTRAINT chk_im_invitations_status CHECK (status IN ('pending', 'accepted', 'declined', 'expired', 'canceled'))
);

CREATE INDEX IF NOT EXISTS idx_im_invitations_invitee
    ON im_invitations (tenant_id, organization_id, invitee_user_id, status, created_at DESC)
    WHERE invitee_user_id IS NOT NULL;

CREATE INDEX IF NOT EXISTS idx_im_invitations_target
    ON im_invitations (tenant_id, organization_id, target_type, target_id, status);

-- 25. 封禁记录�?
CREATE TABLE IF NOT EXISTS im_ban_records (
    tenant_id           TEXT NOT NULL,
    organization_id     TEXT NOT NULL DEFAULT '0',
    ban_id              INTEGER NOT NULL,           -- Snowflake ID
    target_type         TEXT NOT NULL,              -- space/group/channel
    target_id           INTEGER NOT NULL,
    banned_user_id      TEXT NOT NULL,
    banned_by_user_id   TEXT NOT NULL,
    reason              TEXT,
    expires_at          TEXT,
    unbanned_at         TEXT,
    unbanned_by_user_id TEXT,
    created_at          TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at          TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
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

-- 注册新表�?database-table-registry.json
-- 注册新表�?database-prefix-registry.json

-- source: database/migrations/postgres/0002_im_runtime_state_snapshots.up.sql

CREATE TABLE IF NOT EXISTS im_runtime_state_snapshots (
    snapshot_scope TEXT NOT NULL,
    snapshot_key TEXT NOT NULL,
    payload_json TEXT NOT NULL CHECK (json_valid(payload_json)),
    payload_hash TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT pk_im_runtime_state_snapshots PRIMARY KEY (snapshot_scope, snapshot_key)
);

CREATE INDEX IF NOT EXISTS idx_im_runtime_state_snapshots_key
    ON im_runtime_state_snapshots (snapshot_key);

-- folded migration: migrations/sqlite/0003_im_commit_journal_organization_scope.up.sql
-- Align im_commit_journal with organization-scoped journal writes.
-- SQLite-compatible version: SQLite does not support ADD COLUMN IF NOT EXISTS,
-- so we check pragma_table_info before adding the column.

-- Add organization_id column if it does not exist.
INSERT INTO im_commit_journal (partition_key, commit_offset, event_id, tenant_id, aggregate_type, aggregate_id, aggregate_seq, event_type, payload_json, payload_hash, occurred_at, created_at)
SELECT 'migration_check', 0, 'migration_check', 'migration_check', 'migration_check', 'migration_check', 1, 'migration_check', '{}', 'migration_check', '1970-01-01T00:00:00Z', '1970-01-01T00:00:00Z'
WHERE NOT EXISTS (SELECT 1 FROM pragma_table_info('im_commit_journal') WHERE name = 'organization_id');
DELETE FROM im_commit_journal WHERE partition_key = 'migration_check';

-- Recreate indexes with organization_id scope.
DROP INDEX IF EXISTS idx_im_commit_journal_tenant_aggregate_seq;
CREATE INDEX IF NOT EXISTS idx_im_commit_journal_tenant_aggregate_seq
    ON im_commit_journal (tenant_id, organization_id, aggregate_type, aggregate_id, aggregate_seq);

DROP INDEX IF EXISTS idx_im_commit_journal_tenant_occurred;
CREATE INDEX IF NOT EXISTS idx_im_commit_journal_tenant_occurred
    ON im_commit_journal (tenant_id, organization_id, occurred_at, event_id);

DROP INDEX IF EXISTS idx_im_commit_journal_retention_until;
CREATE INDEX IF NOT EXISTS idx_im_commit_journal_retention_until
    ON im_commit_journal (tenant_id, organization_id, retention_until)
    WHERE retention_until IS NOT NULL;

-- folded migration: migrations/sqlite/0004_im_organization_id_default_zero.up.sql
-- Align IM organization scope sentinel with SUBJECT_ID_SPEC tenant-level default `0`.
-- Historical rows may still carry the legacy TEXT sentinel `default`.

UPDATE im_commit_journal
SET organization_id = '0'
WHERE organization_id = 'default';

-- folded migration: migrations/sqlite/0005_im_organization_id_non_empty_check.up.sql
-- Enforce organization_id cannot be empty string on all organization-scoped IM tables.
-- SQLite-compatible version: SQLite does not support adding CHECK constraints via
-- ALTER TABLE, so we use BEFORE INSERT/UPDATE triggers that reject empty organization_id.
--
-- Each trigger raises an ABORT if organization_id is empty string.

-- Note: SQLite triggers fire per-row, so we create one trigger pair per table.
-- Only tables that exist at migration time will have triggers created.

CREATE TRIGGER IF NOT EXISTS chk_im_commit_journal_org_id_non_empty_ins
    BEFORE INSERT ON im_commit_journal
    FOR EACH ROW WHEN NEW.organization_id = ''
BEGIN
    SELECT RAISE(ABORT, 'organization_id must not be empty');
END;

CREATE TRIGGER IF NOT EXISTS chk_im_commit_journal_org_id_non_empty_upd
    BEFORE UPDATE ON im_commit_journal
    FOR EACH ROW WHEN NEW.organization_id = ''
BEGIN
    SELECT RAISE(ABORT, 'organization_id must not be empty');
END;

CREATE TRIGGER IF NOT EXISTS chk_im_outbox_events_org_id_non_empty_ins
    BEFORE INSERT ON im_outbox_events
    FOR EACH ROW WHEN NEW.organization_id = ''
BEGIN
    SELECT RAISE(ABORT, 'organization_id must not be empty');
END;

CREATE TRIGGER IF NOT EXISTS chk_im_outbox_events_org_id_non_empty_upd
    BEFORE UPDATE ON im_outbox_events
    FOR EACH ROW WHEN NEW.organization_id = ''
BEGIN
    SELECT RAISE(ABORT, 'organization_id must not be empty');
END;

CREATE TRIGGER IF NOT EXISTS chk_im_inbox_events_org_id_non_empty_ins
    BEFORE INSERT ON im_inbox_events
    FOR EACH ROW WHEN NEW.organization_id = ''
BEGIN
    SELECT RAISE(ABORT, 'organization_id must not be empty');
END;

CREATE TRIGGER IF NOT EXISTS chk_im_inbox_events_org_id_non_empty_upd
    BEFORE UPDATE ON im_inbox_events
    FOR EACH ROW WHEN NEW.organization_id = ''
BEGIN
    SELECT RAISE(ABORT, 'organization_id must not be empty');
END;

CREATE TRIGGER IF NOT EXISTS chk_im_conversation_messages_org_id_non_empty_ins
    BEFORE INSERT ON im_conversation_messages
    FOR EACH ROW WHEN NEW.organization_id = ''
BEGIN
    SELECT RAISE(ABORT, 'organization_id must not be empty');
END;

CREATE TRIGGER IF NOT EXISTS chk_im_conversation_messages_org_id_non_empty_upd
    BEFORE UPDATE ON im_conversation_messages
    FOR EACH ROW WHEN NEW.organization_id = ''
BEGIN
    SELECT RAISE(ABORT, 'organization_id must not be empty');
END;

CREATE TRIGGER IF NOT EXISTS chk_im_idempotency_keys_org_id_non_empty_ins
    BEFORE INSERT ON im_idempotency_keys
    FOR EACH ROW WHEN NEW.organization_id = ''
BEGIN
    SELECT RAISE(ABORT, 'organization_id must not be empty');
END;

CREATE TRIGGER IF NOT EXISTS chk_im_idempotency_keys_org_id_non_empty_upd
    BEFORE UPDATE ON im_idempotency_keys
    FOR EACH ROW WHEN NEW.organization_id = ''
BEGIN
    SELECT RAISE(ABORT, 'organization_id must not be empty');
END;

CREATE TRIGGER IF NOT EXISTS chk_im_presence_states_org_id_non_empty_ins
    BEFORE INSERT ON im_presence_states
    FOR EACH ROW WHEN NEW.organization_id = ''
BEGIN
    SELECT RAISE(ABORT, 'organization_id must not be empty');
END;

CREATE TRIGGER IF NOT EXISTS chk_im_presence_states_org_id_non_empty_upd
    BEFORE UPDATE ON im_presence_states
    FOR EACH ROW WHEN NEW.organization_id = ''
BEGIN
    SELECT RAISE(ABORT, 'organization_id must not be empty');
END;

CREATE TRIGGER IF NOT EXISTS chk_im_stream_sessions_org_id_non_empty_ins
    BEFORE INSERT ON im_stream_sessions
    FOR EACH ROW WHEN NEW.organization_id = ''
BEGIN
    SELECT RAISE(ABORT, 'organization_id must not be empty');
END;

CREATE TRIGGER IF NOT EXISTS chk_im_stream_sessions_org_id_non_empty_upd
    BEFORE UPDATE ON im_stream_sessions
    FOR EACH ROW WHEN NEW.organization_id = ''
BEGIN
    SELECT RAISE(ABORT, 'organization_id must not be empty');
END;

CREATE TRIGGER IF NOT EXISTS chk_im_stream_frames_org_id_non_empty_ins
    BEFORE INSERT ON im_stream_frames
    FOR EACH ROW WHEN NEW.organization_id = ''
BEGIN
    SELECT RAISE(ABORT, 'organization_id must not be empty');
END;

CREATE TRIGGER IF NOT EXISTS chk_im_stream_frames_org_id_non_empty_upd
    BEFORE UPDATE ON im_stream_frames
    FOR EACH ROW WHEN NEW.organization_id = ''
BEGIN
    SELECT RAISE(ABORT, 'organization_id must not be empty');
END;

-- folded migration: migrations/sqlite/0006_fix_missing_aggregate_type.up.sql
-- Columns aggregate_type / aggregate_id are already declared in the CREATE TABLE
-- definitions above (im_commit_journal and im_audit_records). The previous
-- ALTER TABLE ADD COLUMN IF NOT EXISTS block was removed because:
--   1. Old SQLite versions (< 3.35) do not support ADD COLUMN IF NOT EXISTS.
--   2. The columns already exist in the baseline CREATE TABLE, making these
--      ALTER statements redundant and causing "duplicate column" errors on
--      fresh databases.

-- folded migration: migrations/sqlite/0008_im_rtc_state_machine_expansion.up.sql
-- ============================================================================
-- RTC State Machine Expansion (SQLite)
-- ============================================================================
-- SQLite adaptation of postgres migration 0008.
-- SQLite does not support ALTER TABLE ADD/DROP CONSTRAINT, so CHECK
-- constraints are NOT modified here; they remain as the original 4-state
-- CHECK from baseline. Application-layer validation (RtcSessionState::from_str)
-- enforces the full 11-state machine. The columns are added so the schema
-- matches postgres for cross-engine compatibility.
-- ============================================================================

-- 1. Add lifecycle timestamp columns
ALTER TABLE im_rtc_sessions ADD COLUMN initiating_at      TEXT;
ALTER TABLE im_rtc_sessions ADD COLUMN ringing_at         TEXT;
ALTER TABLE im_rtc_sessions ADD COLUMN connecting_at      TEXT;
ALTER TABLE im_rtc_sessions ADD COLUMN connected_at       TEXT;
ALTER TABLE im_rtc_sessions ADD COLUMN on_hold_since      TEXT;
ALTER TABLE im_rtc_sessions ADD COLUMN reconnecting_since TEXT;
ALTER TABLE im_rtc_sessions ADD COLUMN canceled_at        TEXT;
ALTER TABLE im_rtc_sessions ADD COLUMN failed_at          TEXT;
ALTER TABLE im_rtc_sessions ADD COLUMN timeout_at         TEXT;
ALTER TABLE im_rtc_sessions ADD COLUMN ended_reason       TEXT;
ALTER TABLE im_rtc_sessions ADD COLUMN failure_reason     TEXT;

-- 2. Add client_signal_id to im_rtc_signals for idempotency
ALTER TABLE im_rtc_signals ADD COLUMN client_signal_id TEXT;

CREATE UNIQUE INDEX IF NOT EXISTS uk_im_rtc_signals_client_signal_id
    ON im_rtc_signals (tenant_id, organization_id, rtc_session_id, sender_principal_kind, sender_principal_id, client_signal_id)
    WHERE client_signal_id IS NOT NULL;

-- 3. Indexes for lifecycle queries
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

-- folded migration: migrations/sqlite/0009_im_rtc_lifecycle_tables.up.sql
-- ============================================================================
-- RTC Lifecycle Tables (SQLite)
-- ============================================================================
-- SQLite adaptation of postgres migration 0009.
-- SQLite does not have DOUBLE PRECISION; use REAL.
-- SQLite does not enforce CHECK constraints on NULL unless explicitly written.
-- ============================================================================

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
    payload_json TEXT NOT NULL CHECK (json_valid(payload_json)),
    payload_hash        TEXT NOT NULL,
    publish_status      TEXT NOT NULL DEFAULT 'pending',
    attempt_count       INTEGER NOT NULL DEFAULT 0 CHECK (attempt_count >= 0),
    available_at        TEXT NOT NULL,
    published_at        TEXT,
    created_at          TEXT NOT NULL,
    updated_at          TEXT NOT NULL,
    retention_until     TEXT,
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

-- RTC Quality Reports
CREATE TABLE IF NOT EXISTS im_rtc_quality_reports (
    tenant_id               TEXT NOT NULL,
    organization_id         TEXT NOT NULL DEFAULT '0',
    report_id               TEXT NOT NULL,
    rtc_session_id          TEXT NOT NULL,
    participant_principal_kind TEXT NOT NULL,
    participant_principal_id   TEXT NOT NULL,
    participant_device_id     TEXT NOT NULL,
    reported_at             TEXT NOT NULL,
    mos_score               REAL,
    rtt_ms                  REAL,
    jitter_ms               REAL,
    packet_loss_rate        REAL,
    packets_sent            INTEGER,
    packets_received        INTEGER,
    packets_lost            INTEGER,
    bytes_sent              INTEGER,
    bytes_received          INTEGER,
    audio_bitrate_kbps      INTEGER,
    video_bitrate_kbps      INTEGER,
    audio_codec             TEXT,
    video_codec             TEXT,
    resolution_width        INTEGER,
    resolution_height       INTEGER,
    frame_rate_fps          REAL,
    quality_grade           TEXT,
    payload_json            TEXT,
    payload_hash            TEXT,
    created_at              TEXT NOT NULL,
    retention_until         TEXT,
    CONSTRAINT pk_im_rtc_quality_reports PRIMARY KEY (tenant_id, organization_id, report_id)
);

CREATE INDEX IF NOT EXISTS idx_im_rtc_quality_reports_session_time
    ON im_rtc_quality_reports (tenant_id, organization_id, rtc_session_id, reported_at);

CREATE INDEX IF NOT EXISTS idx_im_rtc_quality_reports_participant
    ON im_rtc_quality_reports (tenant_id, organization_id, participant_principal_kind, participant_principal_id, reported_at);

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
    issued_at                   TEXT NOT NULL,
    expires_at                  TEXT NOT NULL,
    rotated_from_credential_id  TEXT,
    rotated_at                  TEXT,
    revoked_at                  TEXT,
    revoked_reason              TEXT,
    revoked_by_principal_kind   TEXT,
    revoked_by_principal_id     TEXT,
    credential_payload_json TEXT NOT NULL CHECK (json_valid(credential_payload_json)),
    credential_payload_hash     TEXT NOT NULL,
    created_at                  TEXT NOT NULL,
    updated_at                  TEXT NOT NULL,
    retention_until             TEXT,
    CONSTRAINT pk_im_rtc_participant_credentials PRIMARY KEY (tenant_id, organization_id, credential_id)
);

CREATE INDEX IF NOT EXISTS idx_im_rtc_participant_credentials_session
    ON im_rtc_participant_credentials (tenant_id, organization_id, rtc_session_id, participant_principal_kind, participant_principal_id);

CREATE INDEX IF NOT EXISTS idx_im_rtc_participant_credentials_active
    ON im_rtc_participant_credentials (tenant_id, organization_id, rtc_session_id, credential_state, expires_at)
    WHERE credential_state = 'active';

CREATE INDEX IF NOT EXISTS idx_im_rtc_participant_credentials_expiry
    ON im_rtc_participant_credentials (tenant_id, organization_id, expires_at)
    WHERE credential_state = 'active';

-- ============================================================
-- Contact Tags (SQLite-compatible version of PostgreSQL baseline)
-- Source: postgres baseline im_contact_tags
-- ============================================================

CREATE TABLE IF NOT EXISTS im_contact_tags (
    tenant_id           TEXT NOT NULL,
    organization_id     TEXT NOT NULL DEFAULT '0',
    owner_user_id       TEXT NOT NULL,
    tag_id              INTEGER NOT NULL,
    name                TEXT NOT NULL,
    color               TEXT NOT NULL,
    count               INTEGER NOT NULL DEFAULT 0,
    bg                  TEXT NOT NULL DEFAULT '',
    border              TEXT NOT NULL DEFAULT '',
    created_at          TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at          TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT pk_im_contact_tags PRIMARY KEY (tenant_id, organization_id, owner_user_id, tag_id)
);

CREATE INDEX IF NOT EXISTS idx_im_contact_tags_owner
    ON im_contact_tags (tenant_id, organization_id, owner_user_id, updated_at DESC, tag_id DESC);

-- ============================================================
-- Contact Preferences (SQLite-compatible version of PostgreSQL baseline)
-- Source: postgres baseline im_contact_preferences
-- ============================================================

CREATE TABLE IF NOT EXISTS im_contact_preferences (
    tenant_id           TEXT NOT NULL,
    organization_id     TEXT NOT NULL DEFAULT '0',
    owner_user_id       TEXT NOT NULL,
    target_user_id      TEXT NOT NULL,
    is_starred          INTEGER NOT NULL DEFAULT 0,
    is_blocked          INTEGER NOT NULL DEFAULT 0,
    remark              TEXT,
    updated_at          TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT pk_im_contact_preferences PRIMARY KEY (tenant_id, organization_id, owner_user_id, target_user_id)
);

CREATE INDEX IF NOT EXISTS idx_im_contact_preferences_owner
    ON im_contact_preferences (tenant_id, organization_id, owner_user_id, updated_at DESC);

-- ============================================================
-- Contact Recommendations (SQLite-compatible version of PostgreSQL baseline)
-- Source: postgres baseline im_contact_recommendations
-- ============================================================

CREATE TABLE IF NOT EXISTS im_contact_recommendations (
    tenant_id               TEXT NOT NULL,
    organization_id         TEXT NOT NULL DEFAULT '0',
    owner_user_id           TEXT NOT NULL,
    target_user_id          TEXT NOT NULL,
    recommendation_id       INTEGER NOT NULL,
    target_conversation_id  TEXT,
    created_at              TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT pk_im_contact_recommendations PRIMARY KEY (tenant_id, organization_id, recommendation_id)
);

CREATE INDEX IF NOT EXISTS idx_im_contact_recommendations_owner_target
    ON im_contact_recommendations (tenant_id, organization_id, owner_user_id, target_user_id, created_at DESC);

-- ============================================================
-- FTS5 Full-Text Search (SQLite equivalent of PostgreSQL GIN search_vector index)
-- Mirrors PostgreSQL idx_im_messages_search (USING GIN(search_vector) WHERE deleted_at IS NULL).
-- Indexes message text extracted from im_conversation_messages.payload_json
-- (text, caption, description fields). Triggers keep the FTS table in sync.
-- ============================================================

CREATE VIRTUAL TABLE IF NOT EXISTS im_conversation_messages_fts USING fts5(
    tenant_id UNINDEXED,
    organization_id UNINDEXED,
    conversation_id UNINDEXED,
    message_id UNINDEXED,
    content,
    tokenize = 'unicode61'
);

CREATE TRIGGER IF NOT EXISTS im_conversation_messages_fts_ai AFTER INSERT ON im_conversation_messages
BEGIN
    INSERT INTO im_conversation_messages_fts(tenant_id, organization_id, conversation_id, message_id, content)
    VALUES (NEW.tenant_id, NEW.organization_id, NEW.conversation_id, NEW.message_id,
        COALESCE(json_extract(NEW.payload_json, '$.text'), '') || ' ' ||
        COALESCE(json_extract(NEW.payload_json, '$.caption'), '') || ' ' ||
        COALESCE(json_extract(NEW.payload_json, '$.description'), ''));
END;

CREATE TRIGGER IF NOT EXISTS im_conversation_messages_fts_ad AFTER DELETE ON im_conversation_messages
BEGIN
    DELETE FROM im_conversation_messages_fts
    WHERE tenant_id = OLD.tenant_id AND organization_id = OLD.organization_id AND message_id = OLD.message_id;
END;

CREATE TRIGGER IF NOT EXISTS im_conversation_messages_fts_au AFTER UPDATE ON im_conversation_messages
BEGIN
    DELETE FROM im_conversation_messages_fts
    WHERE tenant_id = OLD.tenant_id AND organization_id = OLD.organization_id AND message_id = OLD.message_id;
    INSERT INTO im_conversation_messages_fts(tenant_id, organization_id, conversation_id, message_id, content)
    SELECT NEW.tenant_id, NEW.organization_id, NEW.conversation_id, NEW.message_id,
        COALESCE(json_extract(NEW.payload_json, '$.text'), '') || ' ' ||
        COALESCE(json_extract(NEW.payload_json, '$.caption'), '') || ' ' ||
        COALESCE(json_extract(NEW.payload_json, '$.description'), '')
    WHERE NEW.deleted_at IS NULL;
END;

-- Group knowledgebase orchestration parity. IM does not use SQLite as its
-- durable event authority, but the schema remains available for lifecycle
-- validation and standalone co-location checks.
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
    last_synchronized_membership_epoch INTEGER NOT NULL DEFAULT 0 CHECK (last_synchronized_membership_epoch >= 0),
    last_error_code TEXT,
    last_error_at TEXT,
    created_by TEXT NOT NULL,
    updated_by TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    archived_at TEXT,
    deleted_at TEXT,
    version INTEGER NOT NULL DEFAULT 1 CHECK (version > 0),
    CONSTRAINT pk_im_conversation_knowledge_space_link PRIMARY KEY (tenant_id, organization_id, conversation_id),
    CONSTRAINT uk_im_conversation_knowledge_space_link_id UNIQUE (id),
    CONSTRAINT uk_im_conversation_knowledge_space_link_uuid UNIQUE (link_uuid),
    CONSTRAINT chk_im_conversation_knowledge_space_link_state CHECK (lifecycle_state IN ('provisioning', 'active', 'failed', 'archived', 'deleted')),
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
    CONSTRAINT chk_im_conversation_knowledge_space_link_archived_at CHECK ((lifecycle_state = 'archived') = (archived_at IS NOT NULL)),
    CONSTRAINT chk_im_conversation_knowledge_space_link_deleted_at CHECK ((lifecycle_state = 'deleted') = (deleted_at IS NOT NULL)),
    CONSTRAINT chk_im_conversation_knowledge_space_link_membership_sync_epoch CHECK (last_synchronized_membership_epoch <= membership_epoch)
);

CREATE UNIQUE INDEX IF NOT EXISTS uk_im_conversation_knowledge_space_link_space
    ON im_conversation_knowledge_space_link (knowledge_space_id)
    WHERE knowledge_space_id IS NOT NULL
      AND lifecycle_state IN ('provisioning', 'active', 'archived');

CREATE UNIQUE INDEX IF NOT EXISTS uk_im_conversation_knowledge_space_link_binding
    ON im_conversation_knowledge_space_link (knowledgebase_binding_id)
    WHERE knowledgebase_binding_id IS NOT NULL
      AND lifecycle_state IN ('provisioning', 'active', 'archived');

CREATE INDEX IF NOT EXISTS idx_im_conversation_knowledge_space_link_state
    ON im_conversation_knowledge_space_link (tenant_id, organization_id, lifecycle_state, updated_at, conversation_id);

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
    knowledgebase_binding_id INTEGER NOT NULL,
    knowledgebase_binding_uuid TEXT NOT NULL,
    upstream_link_generation INTEGER NOT NULL,
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
    CONSTRAINT uk_im_group_knowledge_launch_tickets_idempotency UNIQUE (tenant_id, organization_id, conversation_id, actor_kind, actor_id, principal_kind, principal_id, session_id, idempotency_key_hash),
    CONSTRAINT chk_im_group_knowledge_launch_tickets_delegated_user CHECK (actor_kind = 'user' AND principal_kind = 'user' AND actor_id = principal_id),
    CONSTRAINT chk_im_group_knowledge_launch_tickets_binding_id CHECK (knowledgebase_binding_id > 0),
    CONSTRAINT chk_im_group_knowledge_launch_tickets_upstream_link_generation CHECK (upstream_link_generation > 0),
    CONSTRAINT chk_im_group_knowledge_launch_tickets_target_reference CHECK (
        knowledge_space_id > 0
        AND NULLIF(TRIM(knowledge_space_uuid), '') IS NOT NULL
        AND length(CAST(knowledge_space_uuid AS BLOB)) <= 256
        AND knowledgebase_binding_id > 0
        AND NULLIF(TRIM(knowledgebase_binding_uuid), '') IS NOT NULL
        AND length(CAST(knowledgebase_binding_uuid AS BLOB)) <= 256
    ),
    CONSTRAINT chk_im_group_knowledge_launch_tickets_expiry CHECK (expires_at > created_at),
    CONSTRAINT chk_im_group_knowledge_launch_tickets_consumer CHECK ((consumed_at IS NULL AND consumed_by_service IS NULL AND consumed_trace_id IS NULL) OR (consumed_at IS NOT NULL AND consumed_by_service IS NOT NULL AND consumed_trace_id IS NOT NULL))
);

CREATE INDEX IF NOT EXISTS idx_im_group_knowledge_launch_tickets_expiry
    ON im_group_knowledge_launch_tickets (tenant_id, organization_id, expires_at)
    WHERE consumed_at IS NULL;

CREATE INDEX IF NOT EXISTS idx_im_group_knowledge_launch_tickets_actor
    ON im_group_knowledge_launch_tickets (tenant_id, organization_id, actor_kind, actor_id, principal_kind, principal_id, session_id, created_at DESC);

COMMIT;
