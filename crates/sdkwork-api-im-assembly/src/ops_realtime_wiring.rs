use std::sync::Arc;
use std::time::Duration;

use ops_service::{
    LagItem, OpsRuntime, RealtimeInboxDiagnosticsView, RealtimeInboxHighRiskWindowView,
    RouteOwnershipView,
};
use session_gateway::{
    RealtimeClusterBridge, RealtimeDeliveryRuntime, RealtimeInboxDiagnosticsSnapshot,
    RealtimePlaneBootstrap,
};
use tokio::task::JoinHandle;

const OPS_REALTIME_MIRROR_INTERVAL: Duration = Duration::from_secs(1);
const OPS_ROUTE_MIRROR_LIMIT: usize = 200;

pub(crate) struct OpsRealtimeMirrorHandle {
    task: JoinHandle<()>,
}

impl Drop for OpsRealtimeMirrorHandle {
    fn drop(&mut self) {
        self.task.abort();
    }
}

pub(crate) fn spawn_ops_realtime_mirror(
    ops_runtime: Arc<OpsRuntime>,
    bootstrap: &RealtimePlaneBootstrap,
) -> OpsRealtimeMirrorHandle {
    let realtime_cluster = bootstrap.assembly.realtime_cluster();
    let realtime_runtime = bootstrap.assembly.realtime_runtime();
    let node_id = bootstrap.node_id.clone();
    let task = tokio::spawn(async move {
        let mut failure_reported = false;
        loop {
            let ops_runtime = ops_runtime.clone();
            let realtime_cluster = realtime_cluster.clone();
            let realtime_runtime = realtime_runtime.clone();
            let node_id = node_id.clone();
            let result = tokio::task::spawn_blocking(move || {
                mirror_realtime_state(
                    ops_runtime.as_ref(),
                    realtime_cluster.as_ref(),
                    realtime_runtime.as_ref(),
                    node_id.as_str(),
                )
            })
            .await;

            match result {
                Ok(Ok(())) => failure_reported = false,
                Ok(Err(error)) if !failure_reported => {
                    tracing::warn!(error = %error, "ops realtime diagnostics mirror unavailable");
                    failure_reported = true;
                }
                Err(error) => {
                    tracing::error!(error = %error, "ops realtime diagnostics mirror task failed");
                    return;
                }
                Ok(Err(_)) => {}
            }
            tokio::time::sleep(OPS_REALTIME_MIRROR_INTERVAL).await;
        }
    });
    OpsRealtimeMirrorHandle { task }
}

fn mirror_realtime_state(
    ops_runtime: &OpsRuntime,
    realtime_cluster: &RealtimeClusterBridge,
    realtime_runtime: &RealtimeDeliveryRuntime,
    node_id: &str,
) -> Result<(), String> {
    if let Some(lifecycle) = realtime_cluster.node_lifecycle(node_id) {
        ops_runtime.set_node_lifecycle(
            lifecycle.drain_status.as_str(),
            lifecycle.rebalance_state.as_str(),
        );
    }

    let route_page = realtime_cluster.routes_for_node_page(node_id, None, OPS_ROUTE_MIRROR_LIMIT);
    ops_runtime.update_route_ownership_snapshot(
        route_page
            .items
            .into_iter()
            .map(|route| RouteOwnershipView {
                tenant_id: route.tenant_id,
                principal_id: route.principal_id,
                device_id: route.device_id,
                owner_node_id: route.owner_node_id,
                connection_kind: route.connection_kind,
                bound_at: route.bound_at,
            })
            .collect(),
        route_page.total,
    );

    let inbox = realtime_runtime
        .realtime_inbox_diagnostics()
        .map_err(|error| error.message);
    update_realtime_inbox_from_snapshot(ops_runtime, inbox)
}

fn update_realtime_inbox_from_snapshot(
    ops_runtime: &OpsRuntime,
    snapshot: Result<RealtimeInboxDiagnosticsSnapshot, String>,
) -> Result<(), String> {
    match snapshot {
        Ok(snapshot) => {
            let view = map_realtime_inbox(snapshot);
            ops_runtime.update_realtime_inbox(view.clone());
            ops_runtime.upsert_lag_items(realtime_lag_items(&view));
            Ok(())
        }
        Err(error) => {
            ops_runtime.update_realtime_inbox(RealtimeInboxDiagnosticsView::default());
            Err(error)
        }
    }
}

/// Builds real delivery-lag items from the realtime inbox diagnostics.
///
/// For every high-risk client route window the lag is the number of events
/// produced for that scope that are not yet acknowledged (`pending`), with
/// `current_offset` = newest produced sequence watermark and `committed_offset`
/// = delivered watermark, so `lag = current - committed` holds. A `cluster`
/// aggregate item reports the whole-plane backlog.
fn realtime_lag_items(view: &RealtimeInboxDiagnosticsView) -> Vec<LagItem> {
    let mut items = Vec::with_capacity(view.high_risk_windows.len().saturating_add(1));
    for window in &view.high_risk_windows {
        items.push(LagItem {
            component: "realtime".to_owned(),
            scope_id: format!(
                "{}:{}:{}:{}",
                window.tenant_id, window.principal_kind, window.principal_id, window.device_id
            ),
            current_offset: window.trimmed_through_seq + window.pending_event_count,
            committed_offset: window.trimmed_through_seq,
            lag: window.pending_event_count,
        });
    }
    items.push(LagItem {
        component: "realtime".to_owned(),
        scope_id: "cluster".to_owned(),
        current_offset: view.max_trimmed_through_seq + view.pending_event_count,
        committed_offset: view.max_trimmed_through_seq,
        lag: view.pending_event_count,
    });
    items
}

fn map_realtime_inbox(snapshot: RealtimeInboxDiagnosticsSnapshot) -> RealtimeInboxDiagnosticsView {
    RealtimeInboxDiagnosticsView {
        status: snapshot.status,
        client_route_window_count: snapshot.client_route_window_count,
        pending_event_count: snapshot.pending_event_count,
        max_client_route_window_event_count: snapshot.max_client_route_window_event_count,
        client_route_window_capacity: snapshot.client_route_window_capacity,
        max_client_route_window_usage_permille: snapshot.max_client_route_window_usage_permille,
        max_trimmed_through_seq: snapshot.max_trimmed_through_seq,
        capacity_trimmed_event_count: snapshot.capacity_trimmed_event_count,
        max_capacity_trimmed_through_seq: snapshot.max_capacity_trimmed_through_seq,
        last_capacity_trimmed_at: snapshot.last_capacity_trimmed_at,
        oldest_pending_occurred_at: snapshot.oldest_pending_occurred_at,
        high_risk_windows: snapshot
            .high_risk_windows
            .into_iter()
            .map(|window| RealtimeInboxHighRiskWindowView {
                tenant_id: window.tenant_id,
                principal_kind: window.principal_kind,
                principal_id: window.principal_id,
                device_id: window.device_id,
                pending_event_count: window.pending_event_count,
                trimmed_through_seq: window.trimmed_through_seq,
                capacity_trimmed_event_count: window.capacity_trimmed_event_count,
                capacity_trimmed_through_seq: window.capacity_trimmed_through_seq,
                last_capacity_trimmed_at: window.last_capacity_trimmed_at,
                usage_permille: window.usage_permille,
                oldest_pending_occurred_at: window.oldest_pending_occurred_at,
            })
            .collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use session_gateway::RealtimeInboxHighRiskWindow;

    #[test]
    fn realtime_diagnostics_mapping_preserves_capacity_evidence() {
        let mapped = map_realtime_inbox(RealtimeInboxDiagnosticsSnapshot {
            status: "critical".into(),
            client_route_window_count: 2,
            pending_event_count: 9,
            max_client_route_window_event_count: 8,
            client_route_window_capacity: 10,
            max_client_route_window_usage_permille: 800,
            max_trimmed_through_seq: 7,
            capacity_trimmed_event_count: 3,
            max_capacity_trimmed_through_seq: 6,
            last_capacity_trimmed_at: Some("2026-07-29T00:00:00.000Z".into()),
            oldest_pending_occurred_at: Some("2026-07-28T23:59:00.000Z".into()),
            high_risk_windows: vec![RealtimeInboxHighRiskWindow {
                tenant_id: "100001".into(),
                principal_kind: "user".into(),
                principal_id: "1".into(),
                device_id: "device-1".into(),
                pending_event_count: 8,
                trimmed_through_seq: 7,
                capacity_trimmed_event_count: 3,
                capacity_trimmed_through_seq: 6,
                last_capacity_trimmed_at: Some("2026-07-29T00:00:00.000Z".into()),
                usage_permille: 800,
                oldest_pending_occurred_at: Some("2026-07-28T23:59:00.000Z".into()),
            }],
        });

        assert_eq!(mapped.status, "critical");
        assert_eq!(mapped.capacity_trimmed_event_count, 3);
        assert_eq!(mapped.high_risk_windows.len(), 1);
        assert_eq!(mapped.high_risk_windows[0].usage_permille, 800);
    }

    #[test]
    fn realtime_diagnostics_failure_replaces_stale_health_with_unavailable() {
        let runtime = OpsRuntime::new("node-1", "test", "127.0.0.1:0", Vec::new(), Vec::new());
        runtime.update_realtime_inbox(RealtimeInboxDiagnosticsView {
            status: "ok".into(),
            client_route_window_count: 1,
            pending_event_count: 2,
            max_client_route_window_event_count: 2,
            client_route_window_capacity: 100,
            max_client_route_window_usage_permille: 20,
            max_trimmed_through_seq: 0,
            capacity_trimmed_event_count: 0,
            max_capacity_trimmed_through_seq: 0,
            last_capacity_trimmed_at: None,
            oldest_pending_occurred_at: None,
            high_risk_windows: Vec::new(),
        });

        let result = update_realtime_inbox_from_snapshot(
            &runtime,
            Err("diagnostics backend unavailable".into()),
        );

        assert_eq!(result, Err("diagnostics backend unavailable".into()));
        let health = runtime.health_view();
        assert_eq!(health.status, "unavailable");
        assert_eq!(
            health.realtime_inbox,
            RealtimeInboxDiagnosticsView::default()
        );
    }
}
