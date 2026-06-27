use std::sync::Arc;

use im_adapters_local_memory::MemoryPresenceStateStore;
use im_app_context::AppContext;
use im_domain_core::presence::{PresenceClientView, PresenceStatus};
use sdkwork_im_contract_control::{PresenceStateRecord, PresenceStateStore};
use sdkwork_im_contract_core::ContractError;

fn demo_auth(actor_kind: &str, session_id: &str, device_id: &str) -> AppContext {
    AppContext {
        tenant_id: "100001".into(),
        organization_id: "0".into(),
        user_id: "1".into(),
        actor_id: "1".into(),
        actor_kind: actor_kind.into(),
        session_id: Some(session_id.into()),
        app_id: None,
        environment: None,
        deployment_mode: None,
        auth_level: None,
        data_scope: Default::default(),
        permission_scope: Default::default(),
        device_id: Some(device_id.into()),
    }
}

fn presence_record(
    session_id: &str,
    status: PresenceStatus,
    last_seen_at: &str,
) -> PresenceStateRecord {
    PresenceStateRecord {
        tenant_id: "100001".into(),
            organization_id: "0".into(),
        principal_kind: "user".into(),
        principal_id: "1".into(),
        device_id: "d_cold".into(),
        presence: PresenceClientView {
            tenant_id: "100001".into(),
            principal_id: "1".into(),
            device_id: "d_cold".into(),
            platform: None,
            session_id: Some(session_id.into()),
            status,
            last_sync_seq: 17,
            last_resume_at: Some("2026-05-06T00:00:00.000Z".into()),
            last_seen_at: Some(last_seen_at.into()),
        },
        resume_required: false,
        updated_at: last_seen_at.into(),
    }
}

struct HeartbeatAfterStaleListStore {
    inner: MemoryPresenceStateStore,
    refreshed_at: &'static str,
}

impl HeartbeatAfterStaleListStore {
    fn new(refreshed_at: &'static str) -> Self {
        let inner = MemoryPresenceStateStore::default();
        inner
            .save_state(presence_record(
                "s_old",
                PresenceStatus::Online,
                "2026-05-06T00:00:00.000Z",
            ))
            .expect("seed stale online presence should succeed");
        Self {
            inner,
            refreshed_at,
        }
    }
}

impl PresenceStateStore for HeartbeatAfterStaleListStore {
    fn load_state(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_kind: &str,
        principal_id: &str,
        device_id: &str,
    ) -> Result<Option<PresenceStateRecord>, ContractError> {
        self.inner
            .load_state(tenant_id, organization_id, principal_kind, principal_id, device_id)
    }

    fn save_state(&self, record: PresenceStateRecord) -> Result<(), ContractError> {
        self.inner.save_state(record)
    }

    fn list_states_for_principal(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_kind: &str,
        principal_id: &str,
    ) -> Result<Vec<PresenceStateRecord>, ContractError> {
        self.inner
            .list_states_for_principal(tenant_id, organization_id, principal_kind, principal_id)
    }

    fn list_online_states_seen_at_or_before(
        &self,
        cutoff_seen_at: &str,
        limit: usize,
    ) -> Result<Vec<PresenceStateRecord>, ContractError> {
        let stale = self
            .inner
            .list_online_states_seen_at_or_before(cutoff_seen_at, limit)?;
        self.inner.save_state(presence_record(
            "s_fresh",
            PresenceStatus::Online,
            self.refreshed_at,
        ))?;
        Ok(stale)
    }

    fn expire_online_state_if_seen_at_or_before(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_kind: &str,
        principal_id: &str,
        device_id: &str,
        cutoff_seen_at: &str,
        expired_at: &str,
    ) -> Result<Option<PresenceStateRecord>, ContractError> {
        self.inner.expire_online_state_if_seen_at_or_before(
            tenant_id,
            organization_id,
            principal_kind,
            principal_id,
            device_id,
            cutoff_seen_at,
            expired_at,
        )
    }
}

#[test]
fn test_runtime_restores_presence_as_offline_and_requires_fresh_resume_after_rebuild() {
    let presence_store = Arc::new(MemoryPresenceStateStore::default());
    let runtime_before = session_gateway::PresenceRuntime::with_store(presence_store.clone());
    runtime_before
        .register_client_route(&demo_auth("user", "s_before", "d_phone"), "d_phone")
        .expect("phone registration should persist presence inventory");
    runtime_before
        .register_client_route(&demo_auth("user", "s_before", "d_pad"), "d_pad")
        .expect("pad registration should persist presence inventory");

    let resumed = runtime_before
        .resume(
            &demo_auth("user", "s_before", "d_pad"),
            "d_pad".into(),
            0,
            7,
            vec!["d_pad".into(), "d_phone".into()],
        )
        .expect("initial resume should succeed");
    assert_eq!(resumed.presence.devices[0].status.as_str(), "online");

    let runtime_after = session_gateway::PresenceRuntime::with_store(presence_store);

    let restored = runtime_after
        .presence_snapshot(
            &demo_auth("user", "s_after", "d_pad"),
            Some("d_pad".into()),
            Vec::new(),
        )
        .expect("presence snapshot should restore after rebuild");
    assert_eq!(restored.devices.len(), 2);
    assert_eq!(restored.devices[0].device_id, "d_pad");
    assert_eq!(restored.devices[0].status.as_str(), "offline");
    assert!(restored.devices[0].last_resume_at.is_some());
    assert!(restored.devices[0].last_seen_at.is_some());
    assert_eq!(restored.devices[1].device_id, "d_phone");
    assert_eq!(restored.devices[1].status.as_str(), "offline");

    let stale_heartbeat = runtime_after.heartbeat(
        &demo_auth("user", "s_before", "d_pad"),
        "d_pad".into(),
        7,
        vec!["d_pad".into(), "d_phone".into()],
    );
    let stale_error = stale_heartbeat.expect_err("stale pre-restart heartbeat should be rejected");
    assert_eq!(stale_error.code(), "reconnect_required");

    let resumed_after = runtime_after
        .resume(
            &demo_auth("user", "s_after", "d_pad"),
            "d_pad".into(),
            7,
            7,
            vec!["d_pad".into(), "d_phone".into()],
        )
        .expect("fresh resume after rebuild should succeed");
    assert_eq!(resumed_after.presence.devices[0].status.as_str(), "online");
    assert_eq!(
        resumed_after.presence.devices[0].session_id.as_deref(),
        Some("s_after")
    );
}

#[test]
fn test_presence_runtime_resume_returns_incremental_sync_window_from_runtime_link_owner() {
    let runtime = session_gateway::PresenceRuntime::default();
    runtime
        .register_client_route(&demo_auth("user", "s_demo", "d_pad"), "d_pad")
        .expect("client route registration should seed presence state");

    let resumed = runtime
        .resume(
            &demo_auth("user", "s_demo", "d_pad"),
            "d_pad".into(),
            4,
            9,
            vec!["d_pad".into()],
        )
        .expect("resume should succeed");

    assert!(resumed.resume_required);
    assert_eq!(resumed.resume_from_sync_seq, 5);
    assert_eq!(resumed.latest_sync_seq, 9);
}

#[test]
fn test_presence_runtime_expires_stale_online_devices_and_requires_fresh_resume() {
    let presence_store = Arc::new(MemoryPresenceStateStore::default());
    let runtime = session_gateway::PresenceRuntime::with_store(presence_store.clone());
    runtime
        .register_client_route(&demo_auth("user", "s_old", "d_pad"), "d_pad")
        .expect("client route registration should seed presence state");
    runtime
        .resume(
            &demo_auth("user", "s_old", "d_pad"),
            "d_pad".into(),
            0,
            11,
            vec!["d_pad".into()],
        )
        .expect("resume should mark device online");

    let expired = runtime
        .expire_stale_online_devices("9999-12-31T23:59:59.999Z", "9999-12-31T23:59:59.999Z")
        .expect("presence expiration should succeed");

    assert_eq!(expired, 1);

    let snapshot = runtime
        .presence_snapshot(
            &demo_auth("user", "s_new", "d_pad"),
            Some("d_pad".into()),
            vec!["d_pad".into()],
        )
        .expect("presence snapshot should still be readable after expiration");
    assert_eq!(snapshot.devices[0].status.as_str(), "offline");
    assert!(snapshot.devices[0].session_id.is_none());
    assert_eq!(
        snapshot.devices[0].last_seen_at.as_deref(),
        Some("9999-12-31T23:59:59.999Z")
    );

    let stale_heartbeat = runtime.heartbeat(
        &demo_auth("user", "s_old", "d_pad"),
        "d_pad".into(),
        11,
        vec!["d_pad".into()],
    );
    let stale_error = stale_heartbeat.expect_err("expired device must require fresh resume");
    assert_eq!(stale_error.code(), "reconnect_required");

    let persisted = presence_store
        .load_state("100001", "default", "user", "1", "d_pad")
        .expect("presence state load should succeed")
        .expect("presence state should persist after expiration");
    assert_eq!(persisted.presence.status.as_str(), "offline");
    assert!(persisted.presence.session_id.is_none());
    assert!(persisted.resume_required);
}

#[test]
fn test_presence_runtime_expires_stale_online_devices_loaded_only_from_store() {
    let presence_store = Arc::new(MemoryPresenceStateStore::default());
    presence_store
        .save_state(PresenceStateRecord {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            principal_kind: "user".into(),
            principal_id: "1".into(),
            device_id: "d_cold".into(),
            presence: PresenceClientView {
                tenant_id: "100001".into(),
                principal_id: "1".into(),
                device_id: "d_cold".into(),
                platform: None,
                session_id: Some("s_old".into()),
                status: PresenceStatus::Online,
                last_sync_seq: 17,
                last_resume_at: Some("2026-05-06T00:00:00.000Z".into()),
                last_seen_at: Some("2026-05-06T00:00:00.000Z".into()),
            },
            resume_required: false,
            updated_at: "2026-05-06T00:00:00.000Z".into(),
        })
        .expect("seeded online presence should be persisted");

    let runtime_after_restart =
        session_gateway::PresenceRuntime::with_store(presence_store.clone());

    let expired = runtime_after_restart
        .expire_stale_online_devices("2026-05-06T00:00:01.000Z", "2026-05-06T00:00:02.000Z")
        .expect("presence expiration should scan persisted stale online devices");

    assert_eq!(expired, 1);

    let snapshot = runtime_after_restart
        .presence_snapshot(
            &demo_auth("user", "s_new", "d_cold"),
            Some("d_cold".into()),
            vec!["d_cold".into()],
        )
        .expect("snapshot should restore the expired persisted device");
    assert_eq!(snapshot.devices[0].device_id, "d_cold");
    assert_eq!(snapshot.devices[0].status.as_str(), "offline");
    assert!(snapshot.devices[0].session_id.is_none());
    assert_eq!(snapshot.devices[0].last_sync_seq, 17);
    assert_eq!(
        snapshot.devices[0].last_seen_at.as_deref(),
        Some("2026-05-06T00:00:02.000Z")
    );

    let persisted = presence_store
        .load_state("100001", "default", "user", "1", "d_cold")
        .expect("presence state load should succeed")
        .expect("presence state should still exist");
    assert_eq!(persisted.presence.status.as_str(), "offline");
    assert!(persisted.presence.session_id.is_none());
    assert!(persisted.resume_required);
}

#[test]
fn test_presence_runtime_does_not_expire_device_refreshed_after_stale_scan() {
    let presence_store = Arc::new(HeartbeatAfterStaleListStore::new(
        "2026-05-06T00:00:03.000Z",
    ));
    let runtime = session_gateway::PresenceRuntime::with_store(presence_store.clone());

    let expired = runtime
        .expire_stale_online_devices("2026-05-06T00:00:01.000Z", "2026-05-06T00:00:02.000Z")
        .expect("presence expiration should succeed");

    assert_eq!(expired, 0);

    let persisted = presence_store
        .load_state("100001", "default", "user", "1", "d_cold")
        .expect("presence state load should succeed")
        .expect("presence state should still exist");
    assert_eq!(persisted.presence.status.as_str(), "online");
    assert_eq!(persisted.presence.session_id.as_deref(), Some("s_fresh"));
    assert_eq!(
        persisted.presence.last_seen_at.as_deref(),
        Some("2026-05-06T00:00:03.000Z")
    );
    assert!(!persisted.resume_required);
}
