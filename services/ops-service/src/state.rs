use std::collections::BTreeMap;
use std::sync::{Arc, Mutex, MutexGuard};

use im_time::utc_now_rfc3339_millis;
use tokio::sync::Semaphore;

use crate::dto::{
    ClusterNodeView, ClusterView, DiagnosticBundle, LagItem, LagView, OpsHealthResponse,
    ProjectionPlaneDiagnosticsView, ProjectionReplayMetricsView, ProjectionReplayStatusView,
    ProviderBindingDriftItemView, ProviderBindingDriftView, ProviderBindingItemView,
    ProviderBindingSnapshotView, ProviderBindingsView, RealtimeInboxDiagnosticsView,
    RouteOwnershipView, RuntimeDirInspectionView, ServiceHealthView,
    SideEffectOutboxDiagnosticsView,
};

#[derive(Clone)]
pub struct AppState {
    pub(crate) runtime: Arc<OpsRuntime>,
}

#[derive(Clone)]
pub(crate) struct PublicAppGuardrails {
    pub(crate) request_gate: Arc<Semaphore>,
}

pub struct OpsRuntime {
    node_id: String,
    profile: String,
    bind_addr: String,
    services: Vec<ServiceHealthView>,
    owned_scopes: Vec<String>,
    lag_items: Mutex<Vec<LagItem>>,
    drain_status: Mutex<String>,
    rebalance_state: Mutex<String>,
    client_routes: Mutex<Vec<RouteOwnershipView>>,
    provider_bindings: Mutex<BTreeMap<String, ProviderBindingSnapshotView>>,
    runtime_dir_inspection: Mutex<RuntimeDirInspectionView>,
    projection_plane: Mutex<ProjectionPlaneDiagnosticsView>,
    side_effect_outboxes: Mutex<Vec<SideEffectOutboxDiagnosticsView>>,
    realtime_inbox: Mutex<RealtimeInboxDiagnosticsView>,
}

impl Default for OpsRuntime {
    fn default() -> Self {
        Self::new(
            "ops_node_1",
            "standalone",
            "127.0.0.1:18091",
            vec!["ops-service".into()],
            vec!["node:ops_node_1".into()],
        )
    }
}

impl OpsRuntime {
    pub fn new(
        node_id: impl Into<String>,
        profile: impl Into<String>,
        bind_addr: impl Into<String>,
        service_names: Vec<String>,
        owned_scopes: Vec<String>,
    ) -> Self {
        Self {
            node_id: node_id.into(),
            profile: profile.into(),
            bind_addr: bind_addr.into(),
            services: service_names
                .into_iter()
                .map(|service| ServiceHealthView {
                    service,
                    status: "ok".into(),
                })
                .collect(),
            owned_scopes,
            lag_items: Mutex::new(default_lag_items()),
            drain_status: Mutex::new("active".into()),
            rebalance_state: Mutex::new("stable".into()),
            client_routes: Mutex::new(Vec::new()),
            provider_bindings: Mutex::new(BTreeMap::new()),
            runtime_dir_inspection: Mutex::new(RuntimeDirInspectionView::unmanaged()),
            projection_plane: Mutex::new(ProjectionPlaneDiagnosticsView::default()),
            side_effect_outboxes: Mutex::new(Vec::new()),
            realtime_inbox: Mutex::new(RealtimeInboxDiagnosticsView::default()),
        }
    }

    pub fn set_node_lifecycle(&self, drain_status: &str, rebalance_state: &str) {
        *lock_ops_mutex(&self.drain_status, "ops drain status") = drain_status.into();
        *lock_ops_mutex(&self.rebalance_state, "ops rebalance state") = rebalance_state.into();
    }

    pub fn update_route_ownership(&self, mut client_routes: Vec<RouteOwnershipView>) {
        client_routes.sort_by(|left, right| {
            left.tenant_id
                .cmp(&right.tenant_id)
                .then_with(|| left.principal_id.cmp(&right.principal_id))
                .then_with(|| left.device_id.cmp(&right.device_id))
        });
        *lock_ops_mutex(&self.client_routes, "ops client routes") = client_routes;
    }

    pub fn update_runtime_dir_inspection(&self, inspection: RuntimeDirInspectionView) {
        *lock_ops_mutex(&self.runtime_dir_inspection, "ops runtime_dir inspection") = inspection;
    }

    pub fn update_provider_binding_snapshot(&self, snapshot: ProviderBindingSnapshotView) {
        let key = snapshot.tenant_id.clone().unwrap_or_default();
        lock_ops_mutex(&self.provider_bindings, "ops provider bindings").insert(key, snapshot);
    }

    pub fn replace_provider_binding_snapshots(&self, snapshots: Vec<ProviderBindingSnapshotView>) {
        let mut provider_bindings =
            lock_ops_mutex(&self.provider_bindings, "ops provider bindings");
        provider_bindings.clear();
        for snapshot in snapshots {
            let key = snapshot.tenant_id.clone().unwrap_or_default();
            provider_bindings.insert(key, snapshot);
        }
    }

    pub fn update_projection_plane(&self, projection_plane: ProjectionPlaneDiagnosticsView) {
        *lock_ops_mutex(&self.projection_plane, "ops projection-plane") = projection_plane;
    }

    pub fn update_side_effect_outboxes(
        &self,
        mut side_effect_outboxes: Vec<SideEffectOutboxDiagnosticsView>,
    ) {
        side_effect_outboxes.sort_by(|left, right| left.name.cmp(&right.name));
        *lock_ops_mutex(&self.side_effect_outboxes, "ops side-effect outboxes") =
            side_effect_outboxes;
    }

    pub fn update_realtime_inbox(&self, realtime_inbox: RealtimeInboxDiagnosticsView) {
        *lock_ops_mutex(&self.realtime_inbox, "ops realtime inbox") = realtime_inbox;
    }

    pub fn update_projection_replay_lag(&self, mut projection_lag_items: Vec<LagItem>) {
        projection_lag_items.retain(|item| item.component == "projection_replay");
        if projection_lag_items.is_empty() {
            projection_lag_items.push(default_projection_replay_lag_item());
        }
        projection_lag_items.sort_by(|left, right| left.scope_id.cmp(&right.scope_id));

        let mut lag_items = lock_ops_mutex(&self.lag_items, "ops lag items");
        let mut merged = lag_items
            .iter()
            .filter(|item| item.component != "projection_replay")
            .cloned()
            .collect::<Vec<_>>();
        merged.extend(projection_lag_items);
        *lag_items = merged;
    }

    pub fn update_projection_live_lag(&self, mut projection_lag_items: Vec<LagItem>) {
        projection_lag_items.retain(|item| item.component == "projection_live");
        projection_lag_items.sort_by(|left, right| left.scope_id.cmp(&right.scope_id));

        let mut lag_items = lock_ops_mutex(&self.lag_items, "ops lag items");
        let mut merged = lag_items
            .iter()
            .filter(|item| item.component != "projection_live")
            .cloned()
            .collect::<Vec<_>>();
        merged.extend(projection_lag_items);
        *lag_items = merged;
    }

    pub fn node_id(&self) -> &str {
        self.node_id.as_str()
    }

    pub fn health_view(&self) -> OpsHealthResponse {
        let projection_plane =
            lock_ops_mutex(&self.projection_plane, "ops projection-plane").clone();
        let realtime_inbox = lock_ops_mutex(&self.realtime_inbox, "ops realtime inbox").clone();
        let status = rollup_health_status(
            self.services
                .iter()
                .map(|service| service.status.as_str())
                .chain([
                    projection_plane.status.as_str(),
                    realtime_inbox.status.as_str(),
                ]),
        )
        .into();
        OpsHealthResponse {
            status,
            items: self.services.clone(),
            projection_plane: projection_plane.into(),
            realtime_inbox,
        }
    }

    pub fn cluster_view(&self) -> ClusterView {
        let drain_status = lock_ops_mutex(&self.drain_status, "ops drain status").clone();
        let rebalance_state = lock_ops_mutex(&self.rebalance_state, "ops rebalance state").clone();
        let client_route_count = lock_ops_mutex(&self.client_routes, "ops client routes").len();
        ClusterView {
            nodes: vec![ClusterNodeView {
                node_id: self.node_id.clone(),
                profile: self.profile.clone(),
                bind_addr: self.bind_addr.clone(),
                drain_status,
                rebalance_state,
                client_route_count,
                owned_scopes: self.owned_scopes.clone(),
                services: self.services.clone(),
            }],
        }
    }

    pub fn lag_view(&self) -> LagView {
        LagView {
            items: lock_ops_mutex(&self.lag_items, "ops lag items").clone(),
        }
    }

    pub fn runtime_dir_view(&self) -> RuntimeDirInspectionView {
        lock_ops_mutex(&self.runtime_dir_inspection, "ops runtime_dir inspection").clone()
    }

    pub fn provider_bindings_view(&self) -> ProviderBindingsView {
        ProviderBindingsView {
            items: lock_ops_mutex(&self.provider_bindings, "ops provider bindings")
                .values()
                .cloned()
                .collect(),
        }
    }

    pub fn provider_binding_drift_view(&self) -> ProviderBindingDriftView {
        let provider_bindings = lock_ops_mutex(&self.provider_bindings, "ops provider bindings");
        let Some(global_snapshot) = provider_bindings.get("") else {
            return ProviderBindingDriftView::default();
        };

        let baseline_bindings = global_snapshot
            .effective_bindings
            .iter()
            .map(|binding| (binding.domain.clone(), binding))
            .collect::<BTreeMap<_, _>>();

        let items = provider_bindings
            .iter()
            .filter_map(|(tenant_key, snapshot)| {
                if tenant_key.is_empty() {
                    return None;
                }

                Some(
                    snapshot
                        .effective_bindings
                        .iter()
                        .filter_map(|binding| {
                            let baseline = baseline_bindings.get(binding.domain.as_str())?;
                            provider_binding_drift_item(tenant_key.as_str(), baseline, binding)
                        })
                        .collect::<Vec<_>>(),
                )
            })
            .flatten()
            .collect();

        ProviderBindingDriftView {
            baseline_tenant_id: None,
            items,
        }
    }

    pub fn replay_status_view(&self) -> ProjectionReplayStatusView {
        let projection_plane =
            lock_ops_mutex(&self.projection_plane, "ops projection-plane").clone();
        let lag = lock_ops_mutex(&self.lag_items, "ops lag items")
            .iter()
            .filter(|item| item.component == "projection_replay")
            .cloned()
            .collect::<Vec<_>>();
        ProjectionReplayStatusView {
            generated_at: utc_now_rfc3339_millis(),
            status: projection_replay_status(&projection_plane.replay).into(),
            replay_throughput_per_second: projection_replay_throughput_per_second(
                &projection_plane.replay,
            ),
            replay: projection_plane.replay,
            lag,
        }
    }

    pub fn diagnostic_bundle(&self) -> DiagnosticBundle {
        let drain_status = lock_ops_mutex(&self.drain_status, "ops drain status").clone();
        let rebalance_state = lock_ops_mutex(&self.rebalance_state, "ops rebalance state").clone();
        let client_routes = lock_ops_mutex(&self.client_routes, "ops client routes").clone();
        let provider_bindings = lock_ops_mutex(&self.provider_bindings, "ops provider bindings")
            .values()
            .cloned()
            .collect();
        let provider_binding_drift = self.provider_binding_drift_view();
        let projection_plane =
            lock_ops_mutex(&self.projection_plane, "ops projection-plane").clone();
        let side_effect_outboxes =
            lock_ops_mutex(&self.side_effect_outboxes, "ops side-effect outboxes").clone();
        let realtime_inbox = lock_ops_mutex(&self.realtime_inbox, "ops realtime inbox").clone();
        let lag = lock_ops_mutex(&self.lag_items, "ops lag items").clone();
        DiagnosticBundle {
            generated_at: utc_now_rfc3339_millis(),
            profile: self.profile.clone(),
            node_id: self.node_id.clone(),
            bind_addr: self.bind_addr.clone(),
            drain_status,
            rebalance_state,
            owned_scopes: self.owned_scopes.clone(),
            services: self.services.clone(),
            lag,
            client_routes,
            provider_bindings,
            provider_binding_drift,
            projection_plane,
            side_effect_outboxes,
            realtime_inbox,
        }
    }
}

fn lock_ops_mutex<'a, T>(mutex: &'a Mutex<T>, lock_name: &'static str) -> MutexGuard<'a, T> {
    match mutex.lock() {
        Ok(guard) => guard,
        Err(poisoned) => {
            tracing::warn!("recovered poisoned ops mutex lock={lock_name}");
            poisoned.into_inner()
        }
    }
}

fn default_lag_items() -> Vec<LagItem> {
    vec![
        LagItem {
            component: "commit_journal".into(),
            scope_id: "self-hosted.split-services.development".into(),
            current_offset: 0,
            committed_offset: 0,
            lag: 0,
        },
        default_projection_replay_lag_item(),
    ]
}

fn rollup_health_status<'a>(statuses: impl IntoIterator<Item = &'a str>) -> &'static str {
    let mut severity = 0_u8;
    for status in statuses {
        severity = severity.max(health_status_severity(status));
    }
    match severity {
        4 => "critical",
        3 => "unavailable",
        2 => "degraded",
        _ => "ok",
    }
}

fn health_status_severity(status: &str) -> u8 {
    match status {
        "critical" => 4,
        "unavailable" => 3,
        "degraded" => 2,
        "ok" | "idle" => 0,
        _ => 2,
    }
}

fn default_projection_replay_lag_item() -> LagItem {
    LagItem {
        component: "projection_replay".into(),
        scope_id: "projection:*".into(),
        current_offset: 0,
        committed_offset: 0,
        lag: 0,
    }
}

fn projection_replay_status(replay: &ProjectionReplayMetricsView) -> &'static str {
    if replay.backlog_size == 0 && replay.replayed_event_count == 0 {
        "idle"
    } else {
        "replayed"
    }
}

fn projection_replay_throughput_per_second(replay: &ProjectionReplayMetricsView) -> u64 {
    if replay.duration_ms == 0 {
        0
    } else {
        replay.replayed_event_count.saturating_mul(1000) / replay.duration_ms
    }
}

fn provider_binding_drift_item(
    tenant_id: &str,
    baseline: &ProviderBindingItemView,
    binding: &ProviderBindingItemView,
) -> Option<ProviderBindingDriftItemView> {
    let plugin_changed = baseline.selected_plugin_id != binding.selected_plugin_id;
    let source_changed = baseline.selection_source != binding.selection_source;
    if !plugin_changed && !source_changed {
        return None;
    }

    let drift_kind = match (plugin_changed, source_changed) {
        (true, true) => "plugin_and_selection_source_changed",
        (true, false) => "plugin_changed",
        (false, true) => "selection_source_changed",
        (false, false) => unreachable!("drift item should only be built when drift exists"),
    };

    Some(ProviderBindingDriftItemView {
        tenant_id: tenant_id.into(),
        domain: binding.domain.clone(),
        baseline_selected_plugin_id: baseline.selected_plugin_id.clone(),
        selected_plugin_id: binding.selected_plugin_id.clone(),
        baseline_selection_source: baseline.selection_source.clone(),
        selection_source: binding.selection_source.clone(),
        drift_kind: drift_kind.into(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_health_view_recovers_from_poisoned_projection_plane_lock() {
        let runtime = OpsRuntime::default();
        let _ = std::panic::catch_unwind(|| {
            let _guard = runtime
                .projection_plane
                .lock()
                .expect("ops projection-plane should lock");
            panic!("poison ops projection-plane lock");
        });

        let health = runtime.health_view();
        assert_eq!(health.projection_plane.status, "idle");
    }
}
