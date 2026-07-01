use std::sync::Arc;

use im_adapters_postgres_realtime::{
    PostgresRealtimeCheckpointStore, PostgresRealtimeConfig, PostgresRealtimeEventWindowStore,
    PostgresRealtimePool, PostgresRealtimeSubscriptionStore,
};
use session_gateway::{RealtimeDeliveryRuntime, RealtimeSubscriptionItemInput};

const POSTGRES_TEST_DATABASE_URL_ENV: &str = "SDKWORK_IM_POSTGRES_TEST_DATABASE_URL";
const CORE_SCHEMA_SQL: &str = include_str!(
    "../../../database/ddl/baseline/postgres/0001_im_baseline.sql"
);

#[test]
fn test_session_gateway_realtime_runtime_uses_postgres_stores_for_rebuild_ack_and_migration() {
    let Some(database_url) = std::env::var(POSTGRES_TEST_DATABASE_URL_ENV)
        .ok()
        .filter(|value| !value.trim().is_empty())
    else {
        eprintln!(
            "skipping session-gateway PostgreSQL realtime runtime live test because {POSTGRES_TEST_DATABASE_URL_ENV} is not set"
        );
        return;
    };

    let config = PostgresRealtimeConfig::new(database_url)
        .with_pool_max_size(4)
        .with_pool_min_idle(0);
    let pool = config
        .connect_pool()
        .expect("session-gateway PostgreSQL realtime runtime pool should connect");
    apply_core_schema(&pool);

    let suffix = unique_suffix();
    let tenant_id = format!("t_runtime_{suffix}");
    let principal_id = format!("1135_{suffix}");
    let source_device_id = format!("d_runtime_source_{suffix}");
    let target_device_id = format!("d_runtime_target_{suffix}");
    let conversation_id = format!("c_runtime_{suffix}");

    let seed_runtime = runtime_for_pool(pool.clone());
    sync_conversation_subscription(
        &seed_runtime,
        tenant_id.as_str(),
        principal_id.as_str(),
        source_device_id.as_str(),
        conversation_id.as_str(),
    );

    let first_delivery = seed_runtime
        .publish_scope_event_for_principal_kind(
            tenant_id.as_str(),
            "default",
            principal_id.as_str(),
            "user",
            "conversation",
            conversation_id.as_str(),
            "message.posted",
            format!(r#"{{"messageId":"m_runtime_{suffix}_1"}}"#),
            vec![source_device_id.clone()],
        )
        .expect("PostgreSQL backed runtime should publish the first event");
    assert_eq!(first_delivery, 1);

    let rebuilt_runtime = runtime_for_pool(pool.clone());
    let restored_window = rebuilt_runtime
        .list_events_for_principal_kind(
            tenant_id.as_str(),
            "default",
            principal_id.as_str(),
            "user",
            source_device_id.as_str(),
            0,
            10,
        )
        .expect("rebuilt runtime should restore unacked events from PostgreSQL");
    assert_eq!(restored_window.items.len(), 1);
    assert_eq!(restored_window.items[0].realtime_seq, 1);

    let ack = rebuilt_runtime
        .ack_events_for_principal_kind(
            tenant_id.as_str(),
            "default",
            principal_id.as_str(),
            "user",
            source_device_id.as_str(),
            1,
        )
        .expect("PostgreSQL backed runtime should ack and trim");
    assert_eq!(ack.acked_through_seq, 1);
    assert_eq!(ack.trimmed_through_seq, 1);
    assert_eq!(ack.retained_event_count, 0);

    let after_ack_rebuild = runtime_for_pool(pool.clone());
    let trimmed_window = after_ack_rebuild
        .list_events_for_principal_kind(
            tenant_id.as_str(),
            "default",
            principal_id.as_str(),
            "user",
            source_device_id.as_str(),
            0,
            10,
        )
        .expect("rebuilt runtime should preserve PostgreSQL trim checkpoint");
    assert!(trimmed_window.items.is_empty());
    assert_eq!(trimmed_window.acked_through_seq, 1);
    assert_eq!(trimmed_window.trimmed_through_seq, 1);

    let snapshot = after_ack_rebuild
        .take_client_route_state_for_principal_kind(
            tenant_id.as_str(),
            "default",
            principal_id.as_str(),
            "user",
            source_device_id.as_str(),
        )
        .expect("source runtime should take PostgreSQL-backed client route state for migration");
    assert_eq!(snapshot.subscriptions.len(), 1);
    assert!(snapshot.events.is_empty());
    assert_eq!(snapshot.acked_through_seq, 1);

    let target_runtime = runtime_for_pool(pool.clone());
    target_runtime
        .restore_client_route_state(session_gateway::RealtimeClientRouteStateSnapshot {
            device_id: target_device_id.clone(),
            ..snapshot
        })
        .expect("target runtime should restore migrated PostgreSQL-backed state");

    let source_after_take_delivery = after_ack_rebuild
        .publish_scope_event_for_principal_kind(
            tenant_id.as_str(),
            "default",
            principal_id.as_str(),
            "user",
            "conversation",
            conversation_id.as_str(),
            "message.posted",
            format!(r#"{{"messageId":"m_runtime_{suffix}_source_after_take"}}"#),
            vec![source_device_id.clone()],
        )
        .expect("source runtime should not fail after take");
    assert_eq!(source_after_take_delivery, 0);

    let target_delivery = target_runtime
        .publish_scope_event_for_principal_kind(
            tenant_id.as_str(),
            "default",
            principal_id.as_str(),
            "user",
            "conversation",
            conversation_id.as_str(),
            "message.posted",
            format!(r#"{{"messageId":"m_runtime_{suffix}_target"}}"#),
            vec![target_device_id.clone()],
        )
        .expect("target runtime should publish after restore");
    assert_eq!(target_delivery, 1);

    let target_window = target_runtime
        .list_events_for_principal_kind(
            tenant_id.as_str(),
            "default",
            principal_id.as_str(),
            "user",
            target_device_id.as_str(),
            1,
            10,
        )
        .expect("target runtime should expose the restored client route window");
    assert_eq!(target_window.items.len(), 1);
    assert_eq!(target_window.items[0].device_id, target_device_id);
    assert_eq!(target_window.items[0].realtime_seq, 2);
}

fn runtime_for_pool(pool: PostgresRealtimePool) -> RealtimeDeliveryRuntime {
    RealtimeDeliveryRuntime::with_durable_stores_for_standalone_gateway(
        Arc::new(PostgresRealtimeCheckpointStore::from_pool(pool.clone())),
        Arc::new(PostgresRealtimeSubscriptionStore::from_pool(pool.clone())),
        Arc::new(PostgresRealtimeEventWindowStore::from_pool(pool)),
    )
}

fn sync_conversation_subscription(
    runtime: &RealtimeDeliveryRuntime,
    tenant_id: &str,
    principal_id: &str,
    device_id: &str,
    conversation_id: &str,
) {
    runtime
        .sync_subscriptions_for_principal_kind(
            tenant_id,
            "default",
            principal_id,
            "user",
            device_id,
            vec![RealtimeSubscriptionItemInput {
                scope_type: "conversation".into(),
                scope_id: conversation_id.into(),
                event_types: vec!["message.posted".into()],
            }],
        )
        .expect("PostgreSQL backed runtime should sync subscriptions");
}

fn apply_core_schema(pool: &PostgresRealtimePool) {
    let pool = pool.clone();
    std::thread::spawn(move || {
        let mut client = pool
            .get()
            .expect("session-gateway PostgreSQL schema client should connect");
        client
            .batch_execute(CORE_SCHEMA_SQL)
            .expect("core PostgreSQL schema should apply before session-gateway runtime checks");
    })
    .join()
    .expect("session-gateway PostgreSQL schema worker thread should not panic");
}

fn unique_suffix() -> String {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("system time should be after epoch")
        .as_nanos()
        .to_string()
}
