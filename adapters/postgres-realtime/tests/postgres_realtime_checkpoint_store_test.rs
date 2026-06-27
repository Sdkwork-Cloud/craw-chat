use im_adapters_postgres_realtime::{
    PostgresRealtimeCheckpointStore, PostgresRealtimeConfig, PostgresRealtimeConnectionManager,
    PostgresRealtimeDisconnectFenceStore, PostgresRealtimeEventWindowStore, PostgresRealtimePool,
    PostgresRealtimePresenceStateStore, PostgresRealtimeSubscriptionStore,
    postgres_realtime_client_route_scope_key, postgres_realtime_payload_hash,
};
use im_platform_contracts::{
    ContractError, PresenceStateStore, RealtimeCheckpointRecord, RealtimeCheckpointStore,
    RealtimeDisconnectFenceStore, RealtimeEventWindowStore, RealtimeSubscriptionStore,
};

#[test]
fn test_postgres_realtime_adapter_does_not_own_sql_contracts() {
    let source = include_str!("../src/lib.rs");

    assert!(
        source.contains("im_postgres_realtime_contracts"),
        "PostgreSQL realtime adapter must depend on the shared SQL contract crate"
    );
    assert!(
        !source.contains("const LOAD_REALTIME_CHECKPOINT_SQL"),
        "PostgreSQL realtime adapter must not duplicate shared SQL constants"
    );
}

#[test]
fn test_postgres_realtime_checkpoint_store_public_api_implements_contract() {
    fn assert_checkpoint_store<T: RealtimeCheckpointStore>() {}
    fn assert_event_window_store<T: RealtimeEventWindowStore>() {}
    fn assert_subscription_store<T: RealtimeSubscriptionStore>() {}
    fn assert_disconnect_fence_store<T: RealtimeDisconnectFenceStore>() {}
    fn assert_presence_store<T: PresenceStateStore>() {}
    fn assert_pool_api_usable(_: Option<PostgresRealtimePool>) {}
    fn assert_manager_api_usable(_: Option<PostgresRealtimeConnectionManager>) {}

    assert_checkpoint_store::<PostgresRealtimeCheckpointStore>();
    assert_event_window_store::<PostgresRealtimeEventWindowStore>();
    assert_subscription_store::<PostgresRealtimeSubscriptionStore>();
    assert_disconnect_fence_store::<PostgresRealtimeDisconnectFenceStore>();
    assert_presence_store::<PostgresRealtimePresenceStateStore>();
    assert_pool_api_usable(None);
    assert_manager_api_usable(None);

    let config = PostgresRealtimeConfig::new("postgres://chat_user:chat_pass@localhost:5432/chat");
    assert_eq!(
        config.database_url(),
        "postgres://chat_user:chat_pass@localhost:5432/chat"
    );
    assert_eq!(config.pool_max_size(), 16);
}

#[test]
fn test_postgres_realtime_config_exposes_min_idle_pool_control() {
    let source = include_str!("../src/lib.rs");
    let config = PostgresRealtimeConfig::new("postgres://chat_user:chat_pass@localhost:5432/chat")
        .with_pool_min_idle(2)
        .with_pool_max_size(30);

    assert_eq!(config.pool_min_idle(), Some(2));
    assert_eq!(config.pool_max_size(), 30);
    assert!(
        source.contains(".min_idle(config.pool_min_idle)")
            || source.contains(".min_idle(self.pool_min_idle)"),
        "PostgreSQL realtime pool construction must not rely on r2d2 default min_idle=max_size"
    );
}

#[test]
fn test_postgres_realtime_config_exposes_single_shared_pool_constructor() {
    let source = include_str!("../src/lib.rs");

    assert!(
        source
            .contains("pub fn connect_pool(&self) -> Result<PostgresRealtimePool, ContractError>"),
        "PostgreSQL realtime config must expose a shared pool constructor for multi-store runtime wiring"
    );
    assert_eq!(
        source.matches("Pool::builder()").count(),
        1,
        "PostgreSQL realtime adapter must not duplicate r2d2 pool creation across individual stores"
    );
}

#[test]
fn test_postgres_realtime_store_io_is_isolated_from_tokio_runtime_context() {
    let source = include_str!("../src/lib.rs");

    assert!(
        source.contains("fn run_postgres_io"),
        "PostgreSQL realtime adapter must centralize blocking driver calls behind a runtime-safe IO boundary"
    );
    assert!(
        source.contains("Handle::try_current()"),
        "PostgreSQL realtime adapter must detect Tokio runtime context before using the synchronous postgres driver"
    );
    assert!(
        source.contains("connect_pool_bridged"),
        "PostgreSQL realtime adapter must bridge pool creation off Tokio worker threads"
    );
    assert!(
        source.contains("std::thread::scope"),
        "PostgreSQL realtime adapter must run synchronous postgres calls outside the async runtime when called from WebSocket handlers"
    );
    assert_eq!(
        source.matches(".pool\n            .get()").count() + source.matches(".pool.get()").count(),
        0,
        "store methods must not fetch r2d2 connections directly; connection acquisition and drop must stay inside run_postgres_io"
    );
    assert_eq!(
        source.matches("fn postgres_pool_client(").count(),
        1,
        "r2d2 connection acquisition should have one helper boundary"
    );
    assert!(
        source.contains("pool.get()"),
        "the r2d2 pool get call should exist only inside postgres_pool_client"
    );
}

#[test]
fn test_postgres_realtime_live_integration_test_is_guarded_by_database_url_env() {
    let source = include_str!("postgres_realtime_live_integration_test.rs");

    assert!(
        source.contains("SDKWORK_IM_POSTGRES_TEST_DATABASE_URL"),
        "live PostgreSQL integration test must be guarded by an explicit database URL env var"
    );
    assert!(
        source.contains("skipping live PostgreSQL realtime integration test"),
        "live PostgreSQL integration test must skip cleanly when no database URL is configured"
    );
    assert!(
        source.contains("batch_execute(CORE_SCHEMA_SQL)"),
        "live PostgreSQL integration test must apply the checked-in core schema before exercising stores"
    );
    for required_evidence in [
        ".trim_window(",
        "event_types: Vec::<String>::new()",
        "stale subscription retry should not delete newer fanout",
        "stores_for_pool(pool.clone())",
        "stale presence write should not overwrite",
        "stale disconnect fence should not replace",
    ] {
        assert!(
            source.contains(required_evidence),
            "live PostgreSQL integration test must cover runtime evidence marker `{required_evidence}`"
        );
    }
}

#[test]
fn test_session_gateway_postgres_realtime_live_runtime_test_covers_core_runtime_path() {
    let source = include_str!(
        "../../../services/session-gateway/tests/postgres_realtime_live_runtime_test.rs"
    );

    for required_evidence in [
        "SDKWORK_IM_POSTGRES_TEST_DATABASE_URL",
        "PostgresRealtimeCheckpointStore::from_pool",
        "PostgresRealtimeSubscriptionStore::from_pool",
        "PostgresRealtimeEventWindowStore::from_pool",
        "publish_scope_event_for_principal_kind",
        "ack_events_for_principal_kind",
        "take_client_route_state_for_principal_kind",
        "restore_client_route_state",
        "source_after_take_delivery, 0",
        "target_window.items[0].realtime_seq, 2",
    ] {
        assert!(
            source.contains(required_evidence),
            "session-gateway PostgreSQL realtime live runtime test must cover runtime evidence marker `{required_evidence}`"
        );
    }
}

#[test]
fn test_session_gateway_postgres_realtime_websocket_live_drill_covers_reconnect_and_trim_path() {
    let source = include_str!(
        "../../../services/session-gateway/tests/postgres_realtime_websocket_live_drill_test.rs"
    );

    for required_evidence in [
        "SDKWORK_IM_POSTGRES_TEST_DATABASE_URL",
        "PostgresRealtimeCheckpointStore::from_pool",
        "PostgresRealtimeSubscriptionStore::from_pool",
        "PostgresRealtimeEventWindowStore::from_pool",
        "connect_legacy_json_socket",
        "subscriptions.sync",
        "publish_scope_event_for_principal_kind",
        "\"catchup\"",
        "events.ack",
        "reconnect after ack/trim must not replay already trimmed events",
        "ackedThroughSeq",
        "trimmedThroughSeq",
    ] {
        assert!(
            source.contains(required_evidence),
            "session-gateway PostgreSQL realtime websocket live drill must cover runtime evidence marker `{required_evidence}`"
        );
    }
}

#[test]
fn test_postgres_realtime_presence_sql_uses_indexed_schema_and_cas_expiration() {
    let load_sql = PostgresRealtimePresenceStateStore::presence_load_sql().to_lowercase();
    let upsert_sql = PostgresRealtimePresenceStateStore::presence_upsert_sql().to_lowercase();
    let list_principal_sql =
        PostgresRealtimePresenceStateStore::presence_list_principal_sql().to_lowercase();
    let list_stale_sql =
        PostgresRealtimePresenceStateStore::presence_list_stale_online_sql().to_lowercase();
    let expire_sql =
        PostgresRealtimePresenceStateStore::presence_expire_stale_online_sql().to_lowercase();

    assert!(load_sql.contains("from im_presence_states"));
    assert!(load_sql.contains(
        "where tenant_id = $1\n  and organization_id = $2\n  and principal_kind = $3\n  and principal_id = $4\n  and device_id = $5"
    ));
    assert!(
        upsert_sql
            .contains("on conflict (tenant_id, organization_id, principal_kind, principal_id, device_id) do update")
    );
    assert!(
        upsert_sql.contains("excluded.updated_at > im_presence_states.updated_at"),
        "presence upsert must reject stale whole-record writes by updated_at"
    );
    assert!(
        upsert_sql.contains("excluded.updated_at = im_presence_states.updated_at")
            && upsert_sql.contains("excluded.last_sync_seq > im_presence_states.last_sync_seq"),
        "presence upsert must use last_sync_seq as a deterministic tie-breaker when updated_at collides"
    );
    assert!(
        upsert_sql.contains("excluded.last_sync_seq = im_presence_states.last_sync_seq")
            && upsert_sql.contains("excluded.payload_hash = im_presence_states.payload_hash"),
        "presence upsert must only admit exact idempotent retries when updated_at and last_sync_seq both collide"
    );
    assert!(
        upsert_sql.contains("last_sync_seq = excluded.last_sync_seq"),
        "presence upsert should keep the row internally consistent once the updated_at freshness gate admits the write"
    );
    assert!(upsert_sql.contains("$11::jsonb"));
    assert!(list_principal_sql.contains("from im_presence_states"));
    assert!(list_principal_sql.contains("order by device_id asc"));
    assert!(list_stale_sql.contains("presence_status = 'online'"));
    assert!(list_stale_sql.contains("last_seen_at <= $1"));
    assert!(list_stale_sql.contains("order by last_seen_at asc"));
    assert!(expire_sql.contains("update im_presence_states"));
    assert!(expire_sql.contains("presence_status = 'online'"));
    assert!(expire_sql.contains("last_seen_at <= $6"));
    assert!(expire_sql.contains("returning"));
}

#[test]
fn test_postgres_realtime_disconnect_fence_sql_uses_monotonic_upsert_and_cas_clear() {
    let load_sql = PostgresRealtimeDisconnectFenceStore::fence_load_sql().to_lowercase();
    let upsert_sql = PostgresRealtimeDisconnectFenceStore::fence_upsert_sql().to_lowercase();
    let clear_sql = PostgresRealtimeDisconnectFenceStore::fence_clear_sql().to_lowercase();
    let clear_matches_sql =
        PostgresRealtimeDisconnectFenceStore::fence_clear_if_matches_sql().to_lowercase();
    let clear_at_or_before_sql =
        PostgresRealtimeDisconnectFenceStore::fence_clear_disconnected_at_or_before_sql()
            .to_lowercase();

    assert!(load_sql.contains("from im_realtime_disconnect_fences"));
    assert!(
        upsert_sql
            .contains("on conflict (tenant_id, organization_id, principal_kind, principal_id, device_id) do update")
    );
    assert!(upsert_sql.contains(
        "where excluded.disconnected_at > im_realtime_disconnect_fences.disconnected_at"
    ));
    assert!(clear_sql.contains("delete from im_realtime_disconnect_fences"));
    assert!(clear_matches_sql.contains("fence_token = $6"));
    assert!(clear_at_or_before_sql.contains("disconnected_at <= $6"));
}

#[test]
fn test_postgres_realtime_subscription_sql_uses_indexed_fanout_table() {
    let load_sql = PostgresRealtimeSubscriptionStore::subscription_load_sql().to_lowercase();
    let upsert_sql = PostgresRealtimeSubscriptionStore::subscription_upsert_sql().to_lowercase();
    let clear_sql = PostgresRealtimeSubscriptionStore::subscription_clear_sql().to_lowercase();
    let clear_scopes_sql =
        PostgresRealtimeSubscriptionStore::subscription_scope_clear_sql().to_lowercase();
    let replace_scope_sql =
        PostgresRealtimeSubscriptionStore::subscription_scope_replace_sql().to_lowercase();
    let matching_sql =
        PostgresRealtimeSubscriptionStore::matching_subscriptions_sql().to_lowercase();

    assert!(load_sql.contains("from im_realtime_subscriptions"));
    assert!(upsert_sql.contains("on conflict (tenant_id, organization_id, client_route_scope_key) do update"));
    assert!(clear_sql.contains("delete from im_realtime_subscriptions"));
    assert!(clear_scopes_sql.contains("delete from im_realtime_subscription_scopes"));
    assert!(replace_scope_sql.contains("insert into im_realtime_subscription_scopes"));
    assert!(replace_scope_sql.contains("current_subscription.synced_at = $10"));
    assert!(matching_sql.contains("from im_realtime_subscription_scopes fs"));
    assert!(matching_sql.contains("fs.event_type in ($7, '*')"));
    assert!(matching_sql.contains("fs.device_id = any($8)"));
}

#[test]
fn test_postgres_realtime_event_window_sql_uses_checkpoint_and_event_tables_atomically() {
    let load_checkpoint_sql =
        PostgresRealtimeEventWindowStore::checkpoint_load_sql().to_lowercase();
    let list_events_sql =
        PostgresRealtimeEventWindowStore::client_route_events_list_sql().to_lowercase();
    let upsert_event_sql =
        PostgresRealtimeEventWindowStore::client_route_event_upsert_sql().to_lowercase();
    let trim_events_sql =
        PostgresRealtimeEventWindowStore::client_route_events_trim_sql().to_lowercase();
    let clear_events_sql =
        PostgresRealtimeEventWindowStore::client_route_events_clear_sql().to_lowercase();
    let diagnostics_sql = PostgresRealtimeEventWindowStore::diagnostics_sql().to_lowercase();
    let high_risk_sql = PostgresRealtimeEventWindowStore::high_risk_windows_sql().to_lowercase();

    assert!(load_checkpoint_sql.contains("from im_realtime_checkpoints"));
    assert!(list_events_sql.contains("from im_realtime_device_events"));
    assert!(list_events_sql.contains("realtime_seq > $4"));
    assert!(list_events_sql.contains("order by realtime_seq asc"));
    assert!(upsert_event_sql.contains("insert into im_realtime_device_events"));
    assert!(
        upsert_event_sql
            .contains("on conflict (tenant_id, organization_id, client_route_scope_key, realtime_seq) do nothing")
    );
    assert!(trim_events_sql.contains("realtime_seq <= $4"));
    assert!(clear_events_sql.contains("where tenant_id = $1 and organization_id = $2 and client_route_scope_key = $3"));
    assert!(diagnostics_sql.contains("from im_realtime_checkpoints c"));
    assert!(
        !diagnostics_sql.contains("payload_json"),
        "diagnostics must not read realtime payloads"
    );
    assert!(high_risk_sql.contains("limit 5"));
}

#[test]
fn test_postgres_realtime_client_route_scope_key_uses_length_prefixed_segments() {
    assert_eq!(
        postgres_realtime_client_route_scope_key("t:demo", "default", "user", "u|demo", "d#pad"),
        "6:t:demo|7:default|4:user|6:u|demo|5:d#pad"
    );
    assert_eq!(
        postgres_realtime_client_route_scope_key("100001", "default", "user", "1", "d_pad"),
        "6:100001|7:default|4:user|1:1|5:d_pad"
    );
}

#[test]
fn test_postgres_realtime_payload_hash_is_stable_sha256() {
    assert_eq!(
        postgres_realtime_payload_hash(r#"{"messageId":"m_demo"}"#),
        "sha256:073e930d02a0b3c8a1a4d50386b6065e1fabd68b68f15a2caaf85e2a655c8f1d"
    );
}

#[test]
fn test_postgres_realtime_checkpoint_store_merges_checkpoint_records_monotonically() {
    let current = checkpoint(42, 40, 39, 7, 38, Some("2026-05-01T10:00:00.000Z"));
    let stale = checkpoint(41, 37, 36, 5, 35, Some("2026-05-01T09:00:00.000Z"));

    let merged = PostgresRealtimeCheckpointStore::merge_checkpoint_for_write(Some(current), stale);

    assert_eq!(merged.latest_realtime_seq, 42);
    assert_eq!(merged.acked_through_seq, 40);
    assert_eq!(merged.trimmed_through_seq, 39);
    assert_eq!(merged.capacity_trimmed_event_count, 7);
    assert_eq!(merged.capacity_trimmed_through_seq, 38);
    assert_eq!(
        merged.last_capacity_trimmed_at.as_deref(),
        Some("2026-05-01T10:00:00.000Z")
    );
}

#[test]
fn test_postgres_realtime_checkpoint_upsert_sql_is_single_statement_monotonic_merge() {
    let sql = PostgresRealtimeCheckpointStore::checkpoint_upsert_sql().to_lowercase();

    assert!(sql.contains("on conflict (tenant_id, organization_id, client_route_scope_key) do update"));
    assert!(sql.contains("latest_realtime_seq = greatest("));
    assert!(sql.contains("acked_through_seq = greatest("));
    assert!(sql.contains("trimmed_through_seq = greatest("));
    assert!(sql.contains("capacity_trimmed_event_count = greatest("));
    assert!(sql.contains("capacity_trimmed_through_seq = least("));
    assert!(
        !sql.contains("select"),
        "checkpoint writes must not depend on pre-reading current state; the SQL upsert is the atomic merge boundary"
    );
}

#[test]
fn test_postgres_realtime_checkpoint_load_sql_targets_schema_primary_key() {
    let sql = PostgresRealtimeCheckpointStore::checkpoint_load_sql().to_lowercase();

    assert!(sql.contains("from im_realtime_checkpoints"));
    assert!(sql.contains("where tenant_id = $1 and organization_id = $2 and client_route_scope_key = $3"));
    assert!(sql.contains("latest_realtime_seq"));
    assert!(sql.contains("acked_through_seq"));
    assert!(sql.contains("trimmed_through_seq"));
    assert!(sql.contains("capacity_trimmed_event_count"));
    assert!(sql.contains("capacity_trimmed_through_seq"));
    assert!(sql.contains("last_capacity_trimmed_at"));
    assert!(sql.contains("updated_at"));
}

#[test]
fn test_postgres_realtime_checkpoint_validation_rejects_invalid_capacity_trim_metadata() {
    let invalid = checkpoint(42, 40, 39, 7, 38, None);

    let error = PostgresRealtimeCheckpointStore::validate_checkpoint_for_write(&invalid)
        .expect_err("invalid capacity trim metadata must be rejected before database IO");

    assert!(matches!(error, ContractError::Conflict(_)));
    let ContractError::Conflict(message) = error else {
        unreachable!("validated by matches")
    };
    assert!(message.contains("last_capacity_trimmed_at"));
}

#[test]
fn test_postgres_realtime_trim_checkpoint_advances_latest_when_ack_exceeds_current_latest() {
    let current = checkpoint(42, 40, 39, 0, 0, None);

    let trimmed = PostgresRealtimeEventWindowStore::checkpoint_after_trim(
        Some(current),
        45,
        "2026-05-01T10:00:03.000Z",
    );

    assert_eq!(trimmed.latest_realtime_seq, 45);
    assert_eq!(trimmed.acked_through_seq, 45);
    assert_eq!(trimmed.trimmed_through_seq, 45);
    PostgresRealtimeCheckpointStore::validate_checkpoint_for_write(&trimmed)
        .expect("trimmed checkpoint must satisfy database constraints");
}

fn checkpoint(
    latest_realtime_seq: u64,
    acked_through_seq: u64,
    trimmed_through_seq: u64,
    capacity_trimmed_event_count: u64,
    capacity_trimmed_through_seq: u64,
    last_capacity_trimmed_at: Option<&str>,
) -> RealtimeCheckpointRecord {
    RealtimeCheckpointRecord {
        tenant_id: "100001".into(),
        organization_id: "0".into(),
        principal_kind: "user".into(),
        principal_id: "1".into(),
        device_id: "d_pad".into(),
        latest_realtime_seq,
        acked_through_seq,
        trimmed_through_seq,
        capacity_trimmed_event_count,
        capacity_trimmed_through_seq,
        last_capacity_trimmed_at: last_capacity_trimmed_at.map(str::to_owned),
        updated_at: "2026-05-01T10:00:02.000Z".into(),
    }
}
