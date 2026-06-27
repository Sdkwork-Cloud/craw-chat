use im_adapters_postgres_realtime::{
    PostgresRealtimeCheckpointStore, PostgresRealtimeConfig, PostgresRealtimeDisconnectFenceStore,
    PostgresRealtimeEventWindowStore, PostgresRealtimePool, PostgresRealtimePresenceStateStore,
    PostgresRealtimeSubscriptionStore,
};
use im_domain_core::{
    presence::{PresenceClientView, PresenceStatus},
    realtime::{RealtimeEvent, RealtimeSubscription},
};
use im_platform_contracts::{
    PresenceStateRecord, PresenceStateStore, RealtimeCheckpointRecord, RealtimeCheckpointStore,
    RealtimeDisconnectFenceRecord, RealtimeDisconnectFenceStore, RealtimeEventWindowRecord,
    RealtimeEventWindowStore, RealtimeMatchingSubscriptionQuery, RealtimeSubscriptionRecord,
    RealtimeSubscriptionStore,
};
use r2d2_postgres::postgres::{Client, NoTls};

const POSTGRES_TEST_DATABASE_URL_ENV: &str = "SDKWORK_IM_POSTGRES_TEST_DATABASE_URL";
const CORE_SCHEMA_SQL: &str = include_str!(
    "../../../database/ddl/baseline/postgres/0001_im_legacy_baseline.sql"
);

#[test]
fn test_postgres_realtime_live_core_store_roundtrip_when_database_is_configured() {
    let Some(database_url) = std::env::var(POSTGRES_TEST_DATABASE_URL_ENV)
        .ok()
        .filter(|value| !value.trim().is_empty())
    else {
        eprintln!(
            "skipping live PostgreSQL realtime integration test because {POSTGRES_TEST_DATABASE_URL_ENV} is not set"
        );
        return;
    };

    apply_core_schema(database_url.as_str());

    let config = PostgresRealtimeConfig::new(database_url)
        .with_pool_max_size(4)
        .with_pool_min_idle(0);
    let pool = config
        .connect_pool()
        .expect("live PostgreSQL realtime pool should connect");
    let stores = stores_for_pool(pool.clone());

    let suffix = unique_suffix();
    let tenant_id = format!("t_live_{suffix}");
    let organization_id = "0";
    let principal_kind = "user";
    let principal_id = format!("u_live_{suffix}");
    let checkpoint_device_id = format!("d_checkpoint_live_{suffix}");
    let realtime_device_id = format!("d_realtime_live_{suffix}");
    let session_id = format!("s_live_{suffix}");
    let conversation_id = format!("c_live_{suffix}");
    let realtime_context = LiveRealtimeEventContext {
        tenant_id: tenant_id.as_str(),
        principal_id: principal_id.as_str(),
        device_id: realtime_device_id.as_str(),
        conversation_id: conversation_id.as_str(),
    };

    stores
        .checkpoint
        .save_checkpoint(RealtimeCheckpointRecord {
            tenant_id: tenant_id.clone(),
            organization_id: "0".into(),
            principal_kind: principal_kind.into(),
            principal_id: principal_id.clone(),
            device_id: checkpoint_device_id.clone(),
            latest_realtime_seq: 1,
            acked_through_seq: 0,
            trimmed_through_seq: 0,
            capacity_trimmed_event_count: 0,
            capacity_trimmed_through_seq: 0,
            last_capacity_trimmed_at: None,
            updated_at: "2026-05-09T10:00:00.000Z".into(),
        })
        .expect("checkpoint should save to live PostgreSQL");

    let loaded_checkpoint = stores
        .checkpoint
        .load_checkpoint(
            tenant_id.as_str(),
            organization_id,
            principal_kind,
            principal_id.as_str(),
            checkpoint_device_id.as_str(),
        )
        .expect("checkpoint should load from live PostgreSQL")
        .expect("checkpoint row should exist");
    assert_eq!(loaded_checkpoint.latest_realtime_seq, 1);

    stores
        .event_windows
        .save_windows(vec![RealtimeEventWindowRecord {
            tenant_id: tenant_id.clone(),
            organization_id: "0".into(),
            principal_kind: principal_kind.into(),
            principal_id: principal_id.clone(),
            device_id: realtime_device_id.clone(),
            events: vec![
                realtime_context.event(
                    2,
                    "message.created",
                    format!(r#"{{"messageId":"m_live_{suffix}_2"}}"#),
                    "2026-05-09T10:00:01.000Z",
                ),
                realtime_context.event(
                    3,
                    "message.updated",
                    format!(r#"{{"messageId":"m_live_{suffix}_3"}}"#),
                    "2026-05-09T10:00:01.500Z",
                ),
            ],
            trimmed_through_seq: 0,
            capacity_trimmed_event_count: 0,
            capacity_trimmed_through_seq: 0,
            last_capacity_trimmed_at: None,
            updated_at: "2026-05-09T10:00:01.000Z".into(),
        }])
        .expect("event window should save atomically to live PostgreSQL");

    stores
        .event_windows
        .trim_window(
            tenant_id.as_str(),
            organization_id,
            principal_kind,
            principal_id.as_str(),
            realtime_device_id.as_str(),
            2,
        )
        .expect("event window trim should advance checkpoint and delete acknowledged events");

    let loaded_window = stores
        .event_windows
        .load_window(
            tenant_id.as_str(),
            organization_id,
            principal_kind,
            principal_id.as_str(),
            realtime_device_id.as_str(),
        )
        .expect("event window should load from live PostgreSQL")
        .expect("event window should exist");
    assert_eq!(loaded_window.events.len(), 1);
    assert_eq!(loaded_window.events[0].realtime_seq, 3);
    assert_eq!(loaded_window.trimmed_through_seq, 2);

    stores
        .subscriptions
        .save_subscriptions(RealtimeSubscriptionRecord {
            tenant_id: tenant_id.clone(),
            organization_id: "0".into(),
            principal_kind: principal_kind.into(),
            principal_id: principal_id.clone(),
            device_id: realtime_device_id.clone(),
            items: vec![RealtimeSubscription {
                scope_type: "conversation".into(),
                scope_id: conversation_id.clone(),
                event_types: vec!["message.created".into()],
                subscribed_at: "2026-05-09T10:00:02.000Z".into(),
            }],
            synced_at: "2026-05-09T10:00:02.000Z".into(),
        })
        .expect("subscription should save with fanout rows");

    stores
        .subscriptions
        .save_subscriptions(RealtimeSubscriptionRecord {
            tenant_id: tenant_id.clone(),
            organization_id: "0".into(),
            principal_kind: principal_kind.into(),
            principal_id: principal_id.clone(),
            device_id: checkpoint_device_id.clone(),
            items: vec![RealtimeSubscription {
                scope_type: "conversation".into(),
                scope_id: conversation_id.clone(),
                event_types: Vec::<String>::new(),
                subscribed_at: "2026-05-09T10:00:02.500Z".into(),
            }],
            synced_at: "2026-05-09T10:00:02.500Z".into(),
        })
        .expect("wildcard subscription should save as indexed fanout rows");

    let candidate_device_ids = vec![realtime_device_id.clone(), checkpoint_device_id.clone()];
    let matching = stores
        .subscriptions
        .load_matching_subscriptions(RealtimeMatchingSubscriptionQuery {
            tenant_id: tenant_id.as_str(),
            organization_id,
            principal_kind,
            principal_id: principal_id.as_str(),
            scope_type: "conversation",
            scope_id: conversation_id.as_str(),
            event_type: "message.created",
            candidate_device_ids: candidate_device_ids.as_slice(),
        })
        .expect("matching subscriptions should query indexed fanout rows");
    let matching_device_ids = matching
        .iter()
        .map(|record| record.device_id.as_str())
        .collect::<Vec<_>>();
    assert_eq!(
        matching_device_ids,
        vec![checkpoint_device_id.as_str(), realtime_device_id.as_str()]
    );

    let stale_scope_id = format!("c_stale_{suffix}");
    stores
        .subscriptions
        .save_subscriptions(RealtimeSubscriptionRecord {
            tenant_id: tenant_id.clone(),
            organization_id: "0".into(),
            principal_kind: principal_kind.into(),
            principal_id: principal_id.clone(),
            device_id: realtime_device_id.clone(),
            items: vec![RealtimeSubscription {
                scope_type: "conversation".into(),
                scope_id: stale_scope_id.clone(),
                event_types: vec!["message.created".into()],
                subscribed_at: "2026-05-09T10:00:01.000Z".into(),
            }],
            synced_at: "2026-05-09T10:00:01.000Z".into(),
        })
        .expect("stale subscription retry should not delete newer fanout");
    let stale_matching = stores
        .subscriptions
        .load_matching_subscriptions(RealtimeMatchingSubscriptionQuery {
            tenant_id: tenant_id.as_str(),
            organization_id,
            principal_kind,
            principal_id: principal_id.as_str(),
            scope_type: "conversation",
            scope_id: stale_scope_id.as_str(),
            event_type: "message.created",
            candidate_device_ids: std::slice::from_ref(&realtime_device_id),
        })
        .expect("stale subscription retry should not insert old fanout rows");
    assert!(stale_matching.is_empty());
    let current_matching_after_stale_retry = stores
        .subscriptions
        .load_matching_subscriptions(RealtimeMatchingSubscriptionQuery {
            tenant_id: tenant_id.as_str(),
            organization_id,
            principal_kind,
            principal_id: principal_id.as_str(),
            scope_type: "conversation",
            scope_id: conversation_id.as_str(),
            event_type: "message.created",
            candidate_device_ids: std::slice::from_ref(&realtime_device_id),
        })
        .expect("newer subscription fanout should survive stale retry");
    assert_eq!(current_matching_after_stale_retry.len(), 1);
    assert_eq!(
        current_matching_after_stale_retry[0].device_id,
        realtime_device_id
    );

    let restarted_stores = stores_for_pool(pool.clone());
    let restarted_window = restarted_stores
        .event_windows
        .load_window(
            tenant_id.as_str(),
            organization_id,
            principal_kind,
            principal_id.as_str(),
            realtime_device_id.as_str(),
        )
        .expect("restarted event window store should query persisted rows")
        .expect("persisted event window should survive store reconstruction");
    assert_eq!(restarted_window.events.len(), 1);
    assert_eq!(restarted_window.events[0].realtime_seq, 3);

    let fence = RealtimeDisconnectFenceRecord {
        tenant_id: tenant_id.clone(),
            organization_id: "0".into(),
        principal_kind: principal_kind.into(),
        principal_id: principal_id.clone(),
        device_id: realtime_device_id.clone(),
        session_id: Some(session_id.clone()),
        owner_node_id: "node_live_a".into(),
        disconnected_at: "2026-05-09T10:00:03.000Z".into(),
        fence_token: format!("fence_live_{suffix}"),
    };
    stores
        .disconnect_fences
        .save_fence(fence.clone())
        .expect("disconnect fence should save");
    stores
        .disconnect_fences
        .save_fence(RealtimeDisconnectFenceRecord {
            disconnected_at: "2026-05-09T10:00:02.000Z".into(),
            fence_token: format!("stale_fence_live_{suffix}"),
            ..fence.clone()
        })
        .expect("stale disconnect fence should not replace a newer fence");
    let loaded_fence = stores
        .disconnect_fences
        .load_fence(
            tenant_id.as_str(),
            organization_id,
            principal_kind,
            principal_id.as_str(),
            realtime_device_id.as_str(),
        )
        .expect("disconnect fence should load")
        .expect("disconnect fence row should exist");
    assert_eq!(loaded_fence.fence_token, fence.fence_token);
    assert!(
        stores
            .disconnect_fences
            .clear_fence_if_matches(&fence)
            .expect("disconnect fence CAS clear should run")
    );

    let fresh_presence = presence_record(
        tenant_id.as_str(),
        principal_kind,
        principal_id.as_str(),
        realtime_device_id.as_str(),
        Some(session_id.as_str()),
        PresenceStatus::Online,
        7,
        Some("2026-05-09T10:00:04.000Z"),
        Some("2026-05-09T10:00:04.000Z"),
        "2026-05-09T10:00:04.000Z",
        false,
    );
    stores
        .presence
        .save_state(fresh_presence.clone())
        .expect("presence should save to live PostgreSQL");

    stores
        .presence
        .save_state(presence_record(
            tenant_id.as_str(),
            principal_kind,
            principal_id.as_str(),
            realtime_device_id.as_str(),
            Some("stale_session"),
            PresenceStatus::Online,
            6,
            Some("2026-05-09T10:00:04.000Z"),
            Some("2026-05-09T10:00:04.000Z"),
            "2026-05-09T10:00:04.000Z",
            false,
        ))
        .expect("stale presence write should not overwrite newer state");
    let after_stale_presence = stores
        .presence
        .load_state(
            tenant_id.as_str(),
            organization_id,
            principal_kind,
            principal_id.as_str(),
            realtime_device_id.as_str(),
        )
        .expect("presence should load after stale retry")
        .expect("presence row should exist");
    assert_eq!(after_stale_presence.presence.last_sync_seq, 7);
    assert_eq!(
        after_stale_presence.presence.session_id.as_deref(),
        Some(session_id.as_str())
    );

    let expired = stores
        .presence
        .expire_online_state_if_seen_at_or_before(
            tenant_id.as_str(),
            organization_id,
            principal_kind,
            principal_id.as_str(),
            realtime_device_id.as_str(),
            "2026-05-09T10:00:04.000Z",
            "2026-05-09T10:00:05.000Z",
        )
        .expect("presence stale expiration should use CAS update")
        .expect("presence state should expire");
    assert_eq!(expired.presence.status, PresenceStatus::Offline);
    assert!(expired.resume_required);
}

struct LiveStores {
    checkpoint: PostgresRealtimeCheckpointStore,
    event_windows: PostgresRealtimeEventWindowStore,
    subscriptions: PostgresRealtimeSubscriptionStore,
    disconnect_fences: PostgresRealtimeDisconnectFenceStore,
    presence: PostgresRealtimePresenceStateStore,
}

fn stores_for_pool(pool: PostgresRealtimePool) -> LiveStores {
    LiveStores {
        checkpoint: PostgresRealtimeCheckpointStore::from_pool(pool.clone()),
        event_windows: PostgresRealtimeEventWindowStore::from_pool(pool.clone()),
        subscriptions: PostgresRealtimeSubscriptionStore::from_pool(pool.clone()),
        disconnect_fences: PostgresRealtimeDisconnectFenceStore::from_pool(pool.clone()),
        presence: PostgresRealtimePresenceStateStore::from_pool(pool),
    }
}

struct LiveRealtimeEventContext<'a> {
    tenant_id: &'a str,
    principal_id: &'a str,
    device_id: &'a str,
    conversation_id: &'a str,
}

impl LiveRealtimeEventContext<'_> {
    fn event(
        &self,
        realtime_seq: u64,
        event_type: &str,
        payload: String,
        occurred_at: &str,
    ) -> RealtimeEvent {
        RealtimeEvent {
            tenant_id: self.tenant_id.into(),
            principal_id: self.principal_id.into(),
            device_id: self.device_id.into(),
            realtime_seq,
            scope_type: "conversation".into(),
            scope_id: self.conversation_id.into(),
            event_type: event_type.into(),
            delivery_class: "realtime".into(),
            payload,
            occurred_at: occurred_at.into(),
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn presence_record(
    tenant_id: &str,
    principal_kind: &str,
    principal_id: &str,
    device_id: &str,
    session_id: Option<&str>,
    status: PresenceStatus,
    last_sync_seq: u64,
    last_resume_at: Option<&str>,
    last_seen_at: Option<&str>,
    updated_at: &str,
    resume_required: bool,
) -> PresenceStateRecord {
    PresenceStateRecord {
        tenant_id: tenant_id.into(),
            organization_id: "0".into(),
        principal_kind: principal_kind.into(),
        principal_id: principal_id.into(),
        device_id: device_id.into(),
        presence: PresenceClientView {
            tenant_id: tenant_id.into(),
            principal_id: principal_id.into(),
            device_id: device_id.into(),
            platform: Some("windows".into()),
            session_id: session_id.map(str::to_owned),
            status,
            last_sync_seq,
            last_resume_at: last_resume_at.map(str::to_owned),
            last_seen_at: last_seen_at.map(str::to_owned),
        },
        resume_required,
        updated_at: updated_at.into(),
    }
}

fn apply_core_schema(database_url: &str) {
    let mut client =
        Client::connect(database_url, NoTls).expect("live PostgreSQL schema client should connect");
    client
        .batch_execute(CORE_SCHEMA_SQL)
        .expect("core PostgreSQL schema should apply before live adapter checks");
}

fn unique_suffix() -> String {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("system time should be after epoch")
        .as_nanos()
        .to_string()
}
