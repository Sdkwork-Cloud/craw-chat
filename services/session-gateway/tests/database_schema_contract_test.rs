use std::path::PathBuf;

fn postgres_core_schema() -> String {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../database/ddl/baseline/postgres/0001_im_legacy_baseline.sql");
    std::fs::read_to_string(path)
        .expect("core IM PostgreSQL migration should be checked in")
        .replace("\r\n", "\n")
        .to_lowercase()
}

fn assert_contains_all(source: &str, expected: &[&str]) {
    for item in expected {
        assert!(
            source.contains(item),
            "core IM PostgreSQL schema is missing required contract fragment: {item}"
        );
    }
}

fn schema_section<'a>(source: &'a str, start_marker: &str, end_marker: &str) -> &'a str {
    let start = source
        .find(start_marker)
        .unwrap_or_else(|| panic!("schema is missing section start marker: {start_marker}"));
    let tail = &source[start..];
    let end = tail
        .find(end_marker)
        .map(|offset| start + offset)
        .unwrap_or_else(|| panic!("schema is missing section end marker: {end_marker}"));
    &source[start..end]
}

#[test]
fn test_core_im_postgres_schema_defines_append_only_and_idempotency_tables() {
    let schema = postgres_core_schema();

    assert_contains_all(
        &schema,
        &[
            "create table if not exists im_commit_journal",
            "create table if not exists im_outbox_events",
            "create table if not exists im_inbox_events",
            "create table if not exists im_idempotency_keys",
            "constraint pk_im_commit_journal primary key (partition_key, commit_offset)",
            "constraint uk_im_commit_journal_event unique (event_id)",
            "constraint uk_im_inbox_events_source unique (tenant_id, source_system, source_event_id)",
            "constraint chk_im_outbox_events_publish_status check (publish_status in ('pending', 'published', 'failed'))",
            "constraint chk_im_inbox_events_process_status check (process_status in ('pending', 'processed', 'failed'))",
            "idx_im_outbox_events_status_available",
            "idx_im_inbox_events_status_received",
        ],
    );
}

#[test]
fn test_core_im_postgres_schema_defines_hot_cursor_tables_and_indexes() {
    let schema = postgres_core_schema();

    assert_contains_all(
        &schema,
        &[
            "create table if not exists im_conversation_messages",
            "create table if not exists im_realtime_device_events",
            "delivery_class text not null",
            "create table if not exists im_realtime_checkpoints",
            "create table if not exists im_realtime_subscriptions",
            "create table if not exists im_realtime_subscription_scopes",
            "create table if not exists im_presence_states",
            "create table if not exists im_route_bindings",
            "create table if not exists im_realtime_disconnect_fences",
            "create table if not exists im_rtc_sessions",
            "initiator_principal_kind text not null",
            "create table if not exists im_rtc_signals",
            "create table if not exists im_audit_records",
            "create table if not exists im_notification_tasks",
            "create table if not exists im_automation_executions",
            "create table if not exists im_stream_sessions",
            "owner_principal_kind text not null",
            "owner_principal_id text not null",
            "create table if not exists im_stream_frames",
            "constraint pk_im_conversation_messages primary key (tenant_id, conversation_id, message_seq)",
            "constraint pk_im_realtime_device_events primary key (tenant_id, client_route_scope_key, realtime_seq)",
            "constraint pk_im_realtime_checkpoints primary key (tenant_id, client_route_scope_key)",
            "constraint pk_im_realtime_subscriptions primary key (tenant_id, client_route_scope_key)",
            "constraint pk_im_realtime_subscription_scopes primary key (",
            "capacity_trimmed_event_count bigint not null default 0 check (capacity_trimmed_event_count >= 0)",
            "capacity_trimmed_through_seq bigint not null default 0 check (capacity_trimmed_through_seq >= 0)",
            "last_capacity_trimmed_at timestamptz",
            "constraint chk_im_realtime_checkpoints_order check (\n        acked_through_seq <= latest_realtime_seq\n        and trimmed_through_seq <= latest_realtime_seq\n        and capacity_trimmed_through_seq <= trimmed_through_seq\n    )",
            "constraint chk_im_realtime_checkpoints_capacity_trim_meta check (\n        (\n            capacity_trimmed_event_count = 0\n            and capacity_trimmed_through_seq = 0\n            and last_capacity_trimmed_at is null\n        )\n        or (\n            capacity_trimmed_event_count > 0\n            and capacity_trimmed_through_seq > 0\n            and last_capacity_trimmed_at is not null\n        )\n    )",
            "where conname = 'fk_im_realtime_device_events_checkpoint'\n          and conrelid = 'im_realtime_device_events'::regclass",
            "constraint fk_im_realtime_device_events_checkpoint\n            foreign key (tenant_id, client_route_scope_key)\n            references im_realtime_checkpoints (tenant_id, client_route_scope_key)\n            on delete cascade\n            deferrable initially deferred\n            not valid",
            "constraint fk_im_realtime_subscription_scopes_device foreign key (tenant_id, client_route_scope_key)",
            "constraint pk_im_rtc_sessions primary key (tenant_id, rtc_session_id)",
            "constraint chk_im_rtc_sessions_state check (session_state in ('started', 'accepted', 'rejected', 'ended'))",
            "constraint pk_im_rtc_signals primary key (tenant_id, rtc_session_id, signal_seq)",
            "constraint pk_im_audit_records primary key (tenant_id, audit_seq)",
            "constraint pk_im_notification_tasks primary key (tenant_id, notification_id)",
            "constraint chk_im_notification_tasks_status check (notification_status in ('requested', 'dispatched', 'failed'))",
            "constraint pk_im_automation_executions primary key (tenant_id, principal_kind, principal_id, execution_id)",
            "constraint chk_im_automation_executions_state check (execution_state in ('requested', 'running', 'succeeded', 'failed'))",
            "constraint pk_im_stream_sessions primary key (tenant_id, stream_id)",
            "constraint chk_im_stream_sessions_state check (stream_state in ('created', 'opened', 'active', 'checkpointed', 'completed', 'aborted', 'expired'))",
            "constraint chk_im_stream_sessions_seq_order check (",
            "constraint pk_im_stream_frames primary key (tenant_id, stream_id, frame_seq)",
            "idx_im_messages_tenant_conversation_seq",
            "idx_im_realtime_device_events_scope_seq",
            "idx_im_realtime_checkpoints_capacity_trimmed",
            "idx_im_realtime_subscriptions_principal",
            "idx_im_realtime_subscriptions_synced_at",
            "idx_im_realtime_subscriptions_items_gin",
            "idx_im_realtime_subscription_scopes_fanout",
            "idx_im_realtime_subscription_scopes_device",
            "idx_im_rtc_sessions_conversation",
            "idx_im_rtc_signals_session_seq",
            "idx_im_audit_records_tenant_seq",
            "idx_im_notification_tasks_recipient_updated",
            "idx_im_notification_tasks_status",
            "idx_im_automation_executions_principal_updated",
            "idx_im_automation_executions_state",
            "idx_im_stream_sessions_scope",
            "idx_im_stream_sessions_updated",
            "idx_im_stream_frames_stream_seq",
            "idx_im_route_bindings_owner_node",
            "idx_im_realtime_disconnect_fences_disconnected_at",
            "idx_im_presence_states_principal",
            "idx_im_presence_states_online_seen_at",
            "constraint chk_im_presence_states_status check (presence_status in ('online', 'offline'))",
            "constraint chk_im_route_bindings_connection_kind check (connection_kind in (",
            "'ccp/ws/1'",
            "'ccp/tcp/1'",
            "'ccp/udp/1'",
            "'ccp/quic/1'",
        ],
    );
}

#[test]
fn test_core_im_postgres_schema_defines_drive_backed_message_media_refs() {
    let schema = postgres_core_schema();

    assert_contains_all(
        &schema,
        &[
            "create table if not exists im_message_media_refs",
            "drive_space_id text not null",
            "drive_node_id text not null",
            "drive_uri text not null",
            "drive_node_version text",
            "media_resource_snapshot jsonb not null",
            "resource_hash text not null",
            "constraint pk_im_message_media_refs primary key (tenant_id, conversation_id, message_seq, part_index)",
            "constraint uk_im_message_media_refs_message_part unique (tenant_id, message_id, part_index)",
            "constraint fk_im_message_media_refs_message foreign key (tenant_id, conversation_id, message_seq)",
            "references im_conversation_messages (tenant_id, conversation_id, message_seq)",
            "constraint chk_im_message_media_refs_drive_uri check (",
            "drive_uri = ('drive://spaces/' || drive_space_id || '/nodes/' || drive_node_id)",
            "constraint chk_im_message_media_refs_media_source check (\n        media_source in ('drive', 'external_url', 'data_url', 'provider_asset', 'generated')\n    )",
            "create index if not exists idx_im_message_media_refs_drive_node",
            "on im_message_media_refs (tenant_id, drive_space_id, drive_node_id, message_seq desc)",
            "create index if not exists idx_im_message_media_refs_role",
            "create index if not exists idx_im_message_media_refs_retention_until",
        ],
    );

    let media_refs = schema_section(
        &schema,
        "create table if not exists im_message_media_refs",
        "create table if not exists im_realtime_device_events",
    );
    for forbidden in [
        "bucket",
        "object_key",
        "objectkey",
        "storage_provider",
        "upload_session",
        "download_url",
        "presign",
        "object_storage",
    ] {
        assert!(
            !media_refs.contains(forbidden),
            "im_message_media_refs must not store Drive-owned storage lifecycle field `{forbidden}`"
        );
    }
}

#[test]
fn test_core_im_postgres_schema_defines_notification_and_automation_hot_paths() {
    let schema = postgres_core_schema();

    assert_contains_all(
        &schema,
        &[
            "create table if not exists im_notification_tasks",
            "recipient_kind text not null",
            "recipient_id text not null",
            "notification_status text not null",
            "constraint pk_im_notification_tasks primary key (tenant_id, notification_id)",
            "constraint uk_im_notification_tasks_source unique (tenant_id, source_event_id, recipient_kind, recipient_id, category, channel)",
            "constraint chk_im_notification_tasks_status check (notification_status in ('requested', 'dispatched', 'failed'))",
            "create index if not exists idx_im_notification_tasks_recipient_updated",
            "on im_notification_tasks (tenant_id, recipient_kind, recipient_id, updated_at desc, notification_id)",
            "create index if not exists idx_im_notification_tasks_status",
            "on im_notification_tasks (tenant_id, notification_status, updated_at desc)",
            "create table if not exists im_automation_executions",
            "principal_kind text not null",
            "principal_id text not null",
            "execution_state text not null",
            "retry_count integer not null default 0 check (retry_count >= 0)",
            "constraint pk_im_automation_executions primary key (tenant_id, principal_kind, principal_id, execution_id)",
            "constraint uk_im_automation_executions_request unique (tenant_id, principal_kind, principal_id, execution_id, request_hash)",
            "constraint chk_im_automation_executions_state check (execution_state in ('requested', 'running', 'succeeded', 'failed'))",
            "create index if not exists idx_im_automation_executions_principal_updated",
            "on im_automation_executions (tenant_id, principal_kind, principal_id, updated_at desc, execution_id)",
            "create index if not exists idx_im_automation_executions_state",
            "on im_automation_executions (tenant_id, execution_state, updated_at desc)",
        ],
    );
}

#[test]
fn test_core_im_postgres_schema_defines_projection_hot_paths() {
    let schema = postgres_core_schema();

    assert_contains_all(
        &schema,
        &[
            "create table if not exists im_projection_timeline_entries",
            "constraint pk_im_projection_timeline_entries primary key (tenant_id, conversation_id, message_seq)",
            "create index if not exists idx_im_projection_timeline_entries_message",
            "on im_projection_timeline_entries (tenant_id, message_id)",
            "create table if not exists im_projection_conversation_summaries",
            "constraint pk_im_projection_conversation_summaries primary key (tenant_id, conversation_id)",
            "create index if not exists idx_im_projection_conversation_summaries_activity",
            "on im_projection_conversation_summaries (tenant_id, last_activity_at desc, conversation_id)",
            "create table if not exists im_projection_conversation_members",
            "principal_kind text not null",
            "constraint pk_im_projection_conversation_members primary key (tenant_id, conversation_id, principal_kind, principal_id)",
            "constraint uk_im_projection_conversation_members_member unique (tenant_id, conversation_id, member_id)",
            "create index if not exists idx_im_projection_conversation_members_principal",
            "on im_projection_conversation_members (tenant_id, principal_kind, principal_id, membership_state, conversation_id)",
            "create index if not exists idx_im_projection_conversation_members_active",
            "where membership_state = 'joined'",
            "create table if not exists im_projection_read_cursors",
            "principal_kind text not null",
            "constraint pk_im_projection_read_cursors primary key (tenant_id, conversation_id, member_id)",
            "create index if not exists idx_im_projection_read_cursors_principal",
            "on im_projection_read_cursors (tenant_id, principal_kind, principal_id, conversation_id)",
            "create table if not exists im_projection_registered_client_routes",
            "constraint pk_im_projection_registered_client_routes primary key (tenant_id, principal_kind, principal_id, device_id)",
            "create table if not exists im_projection_client_route_sync_feeds",
            "constraint pk_im_projection_client_route_sync_feeds primary key (tenant_id, principal_kind, principal_id, device_id, sync_seq)",
            "create index if not exists idx_im_projection_client_route_sync_feeds_window",
            "on im_projection_client_route_sync_feeds (tenant_id, principal_kind, principal_id, device_id, sync_seq)",
            "create table if not exists im_projection_client_route_sync_checkpoints",
            "constraint pk_im_projection_client_route_sync_checkpoints primary key (tenant_id, principal_kind, principal_id, device_id)",
            "constraint chk_im_projection_client_route_sync_checkpoints_order check (trimmed_through_seq <= latest_sync_seq)",
            "create table if not exists im_projection_contacts",
            "constraint pk_im_projection_contacts primary key (tenant_id, owner_user_id, contact_type, target_user_id)",
            "create index if not exists idx_im_projection_contacts_owner_activity",
            "on im_projection_contacts (tenant_id, owner_user_id, last_interaction_at desc, target_user_id)",
            "create table if not exists im_projection_direct_chat_bindings",
            "constraint pk_im_projection_direct_chat_bindings primary key (tenant_id, direct_chat_id)",
            "constraint uk_im_projection_direct_chat_bindings_conversation unique (tenant_id, conversation_id)",
            "create index if not exists idx_im_projection_direct_chat_bindings_conversation",
            "on im_projection_direct_chat_bindings (tenant_id, conversation_id, direct_chat_status)",
        ],
    );
}

#[test]
fn test_core_im_postgres_schema_allows_shared_history_linked_members() {
    let schema = postgres_core_schema();
    let conversation_members = schema_section(
        &schema,
        "create table if not exists im_projection_conversation_members",
        "create table if not exists im_projection_read_cursors",
    );

    assert!(
        conversation_members.contains(
            "constraint chk_im_projection_conversation_members_state check (membership_state in ('invited', 'joined', 'linked', 'removed', 'left'))"
        ) || conversation_members.contains(
            "constraint chk_im_projection_conversation_members_state check (membership_state in ('invited', 'joined', 'removed', 'left', 'linked'))"
        ),
        "conversation member projection must allow membership_state='linked' because shared-channel linked members are runtime/domain-valid readers"
    );
}

#[test]
fn test_core_im_postgres_schema_defines_disconnect_fence_cas_contract() {
    let schema = postgres_core_schema();

    assert_contains_all(
        &schema,
        &[
            "create table if not exists im_realtime_disconnect_fences",
            "constraint pk_im_realtime_disconnect_fences primary key (tenant_id, principal_kind, principal_id, device_id)",
            "disconnected_at timestamptz not null",
            "session_id text",
            "owner_node_id text not null",
            "fence_token text not null",
            "constraint uk_im_realtime_disconnect_fences_token unique (tenant_id, fence_token)",
            "create index if not exists idx_im_realtime_disconnect_fences_disconnected_at",
            "on im_realtime_disconnect_fences (tenant_id, disconnected_at, principal_kind, principal_id, device_id)",
        ],
    );
}

#[test]
fn test_core_im_postgres_schema_defines_explicit_nullable_idempotency_indexes() {
    let schema = postgres_core_schema();

    assert_contains_all(
        &schema,
        &[
            "create unique index if not exists uk_im_conversation_messages_client\n    on im_conversation_messages (tenant_id, conversation_id, sender_principal_kind, sender_principal_id, client_msg_id)\n    where client_msg_id is not null",
        ],
    );

    assert!(
        !schema.contains(
            "constraint uk_im_conversation_messages_client unique (tenant_id, conversation_id, sender_principal_kind, sender_principal_id, client_msg_id)"
        ),
        "client_msg_id idempotency must be a partial unique index so nullable non-idempotent messages do not share a hidden unique constraint"
    );
}

#[test]
fn test_core_im_postgres_schema_separates_stream_and_rtc_sessions_from_append_logs() {
    let schema = postgres_core_schema();

    assert_contains_all(
        &schema,
        &[
            "create table if not exists im_stream_sessions",
            "owner_principal_kind text not null",
            "owner_principal_id text not null",
            "last_frame_seq bigint not null default 0 check (last_frame_seq >= 0)",
            "last_checkpoint_seq bigint check (last_checkpoint_seq >= 0)",
            "complete_frame_seq bigint check (complete_frame_seq >= 0)",
            "abort_frame_seq bigint check (abort_frame_seq >= 0)",
            "constraint chk_im_stream_sessions_seq_order check (\n        coalesce(last_checkpoint_seq, 0) <= last_frame_seq\n        and coalesce(complete_frame_seq, 0) <= last_frame_seq\n        and coalesce(abort_frame_seq, 0) <= last_frame_seq\n    )",
            "create table if not exists im_stream_frames",
            "constraint pk_im_stream_frames primary key (tenant_id, stream_id, frame_seq)",
            "create table if not exists im_rtc_sessions",
            "initiator_principal_kind text not null",
            "latest_signal_seq bigint not null default 0 check (latest_signal_seq >= 0)",
            "constraint pk_im_rtc_signals primary key (tenant_id, rtc_session_id, signal_seq)",
            "idx_im_rtc_sessions_state",
            "on im_rtc_sessions (tenant_id, session_state, updated_at desc, rtc_session_id)",
            "idx_im_rtc_sessions_provider_session",
            "on im_rtc_sessions (tenant_id, provider_plugin_id, provider_session_id)",
            "where provider_plugin_id is not null and provider_session_id is not null",
        ],
    );
}

#[test]
fn test_core_im_postgres_schema_has_lifecycle_and_payload_guardrails() {
    let schema = postgres_core_schema();

    assert_contains_all(
        &schema,
        &[
            "tenant_id text not null",
            "created_at timestamptz not null",
            "updated_at timestamptz not null",
            "payload_json jsonb not null",
            "payload_hash text not null",
            "retention_until timestamptz",
            "check (message_seq > 0)",
            "check (realtime_seq > 0)",
            "check (signal_seq > 0)",
            "check (audit_seq > 0)",
            "check (frame_seq > 0)",
        ],
    );
}

#[test]
fn test_core_im_postgres_schema_indexes_presence_lease_expiration() {
    let schema = postgres_core_schema();

    assert_contains_all(
        &schema,
        &[
            "create index if not exists idx_im_presence_states_online_seen_at",
            "on im_presence_states (\n        last_seen_at,\n        tenant_id,\n        principal_kind,\n        principal_id,\n        device_id\n    )",
            "where presence_status = 'online' and last_seen_at is not null",
            "presence_status text not null",
            "last_seen_at timestamptz",
            "resume_required boolean not null default false",
        ],
    );
}

#[test]
fn test_core_im_postgres_schema_indexes_retention_cleanup_paths() {
    let schema = postgres_core_schema();

    assert_contains_all(
        &schema,
        &[
            "create index if not exists idx_im_commit_journal_retention_until",
            "create index if not exists idx_im_outbox_events_retention_until",
            "create index if not exists idx_im_inbox_events_retention_until",
            "create index if not exists idx_im_conversation_messages_retention_until",
            "create index if not exists idx_im_realtime_device_events_retention_until",
            "create index if not exists idx_im_realtime_subscriptions_retention_until",
            "create index if not exists idx_im_presence_states_retention_until",
            "create index if not exists idx_im_realtime_disconnect_fences_retention_until",
            "create index if not exists idx_im_rtc_sessions_retention_until",
            "create index if not exists idx_im_rtc_signals_retention_until",
            "create index if not exists idx_im_audit_records_retention_until",
            "create index if not exists idx_im_notification_tasks_retention_until",
            "create index if not exists idx_im_automation_executions_retention_until",
            "create index if not exists idx_im_projection_timeline_entries_retention_until",
            "create index if not exists idx_im_projection_conversation_summaries_retention_until",
            "create index if not exists idx_im_projection_conversation_members_retention_until",
            "create index if not exists idx_im_projection_read_cursors_retention_until",
            "create index if not exists idx_im_projection_registered_client_routes_retention_until",
            "create index if not exists idx_im_projection_client_route_sync_feeds_retention_until",
            "create index if not exists idx_im_projection_client_route_sync_checkpoints_retention_until",
            "create index if not exists idx_im_projection_contacts_retention_until",
            "create index if not exists idx_im_projection_direct_chat_bindings_retention_until",
            "create index if not exists idx_im_stream_sessions_retention_until",
            "create index if not exists idx_im_stream_frames_retention_until",
            "where retention_until is not null",
        ],
    );
}

#[test]
fn test_core_im_postgres_schema_drops_redundant_idempotency_unique_constraint() {
    let schema = postgres_core_schema();

    assert!(
        schema.contains("drop constraint uk_im_idempotency_keys_scope"),
        "schema should drop redundant uk_im_idempotency_keys_scope to avoid duplicated uniqueness with primary key"
    );
}
