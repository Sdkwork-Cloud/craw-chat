use std::collections::BTreeMap;
use std::ops::Bound::{Excluded, Included, Unbounded};
use std::sync::{Arc, Mutex, MutexGuard};

use im_time::utc_now_rfc3339_millis;
use sdkwork_utils_rust::{
    DEFAULT_LIST_PAGE_SIZE, MAX_LIST_PAGE_SIZE, SdkWorkCursorListQuery, SdkWorkPageData,
    base64url_decode, base64url_encode, cursor_list_page_data,
};
use tokio::sync::Semaphore;

use crate::dto::{
    ClusterNodeView, ClusterView, DiagnosticBundle, LagItem, LagView, OpsHealthResponse,
    ProviderBindingDriftItemView, ProviderBindingDriftView, ProviderBindingItemView,
    ProviderBindingSnapshotView, ProviderBindingsView, RealtimeInboxDiagnosticsView,
    RouteOwnershipView, RuntimeDirInspectionView, ServiceHealthView,
    SideEffectOutboxDiagnosticsView,
};
use crate::error::OpsError;

#[derive(Clone)]
pub struct AppState {
    pub(crate) runtime: Arc<OpsRuntime>,
}

impl AppState {
    pub fn new(runtime: Arc<OpsRuntime>) -> Self {
        Self { runtime }
    }
}

#[derive(Clone)]
pub(crate) struct PublicAppGuardrails {
    pub(crate) request_gate: Arc<Semaphore>,
}

pub struct OpsRuntime {
    node_id: String,
    profile: String,
    bind_addr: String,
    publish_cluster_node: bool,
    services: Vec<ServiceHealthView>,
    owned_scopes: Vec<String>,
    lag_items: Mutex<Vec<LagItem>>,
    drain_status: Mutex<String>,
    rebalance_state: Mutex<String>,
    client_routes: Mutex<Vec<RouteOwnershipView>>,
    client_route_total: Mutex<usize>,
    provider_bindings: Mutex<BTreeMap<String, ProviderBindingSnapshotView>>,
    runtime_dir_inspection: Mutex<RuntimeDirInspectionView>,
    side_effect_outboxes: Mutex<Vec<SideEffectOutboxDiagnosticsView>>,
    realtime_inbox: Mutex<RealtimeInboxDiagnosticsView>,
}

const DIAGNOSTIC_COLLECTION_LIMIT: usize = 200;
const OPS_CURSOR_MAX_BYTES: usize = 4 * 1024;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct OpsCursorPayload {
    version: u8,
    resource: String,
    key: Vec<String>,
}

fn resolve_ops_page_size(query: &SdkWorkCursorListQuery) -> Result<usize, OpsError> {
    let page_size = query.page_size.unwrap_or(DEFAULT_LIST_PAGE_SIZE);
    if !(1..=MAX_LIST_PAGE_SIZE).contains(&page_size) {
        return Err(OpsError::invalid_parameter(format!(
            "page_size must be between 1 and {MAX_LIST_PAGE_SIZE}"
        )));
    }
    Ok(page_size as usize)
}

fn decode_ops_cursor(
    cursor: Option<&str>,
    resource: &str,
    key_parts: usize,
) -> Result<Option<Vec<String>>, OpsError> {
    let Some(cursor) = cursor else {
        return Ok(None);
    };
    let cursor = cursor.trim();
    if cursor.is_empty() || cursor.len() > OPS_CURSOR_MAX_BYTES {
        return Err(OpsError::invalid_parameter(
            "cursor is empty or exceeds 4096 bytes",
        ));
    }
    let bytes = base64url_decode(cursor)
        .ok_or_else(|| OpsError::invalid_parameter("cursor is not valid base64url"))?;
    let payload: OpsCursorPayload = serde_json::from_slice(bytes.as_slice())
        .map_err(|_| OpsError::invalid_parameter("cursor payload is invalid"))?;
    if payload.version != 1 || payload.resource != resource || payload.key.len() != key_parts {
        return Err(OpsError::invalid_parameter(
            "cursor does not match the requested ops resource",
        ));
    }
    Ok(Some(payload.key))
}

fn encode_ops_cursor(resource: &str, key: Vec<String>) -> Result<String, OpsError> {
    let payload = serde_json::to_vec(&OpsCursorPayload {
        version: 1,
        resource: resource.to_owned(),
        key,
    })
    .map_err(|error| {
        OpsError::internal(
            "ops_cursor_encode_failed",
            format!("failed to encode ops cursor: {error}"),
        )
    })?;
    Ok(base64url_encode(payload.as_slice()))
}

impl Default for OpsRuntime {
    fn default() -> Self {
        Self::new_internal(
            "unconfigured",
            "unconfigured",
            "unconfigured",
            Vec::new(),
            Vec::new(),
            false,
        )
    }
}

impl OpsRuntime {
    pub fn from_env() -> Self {
        Self::from_env_with_optional_node_id(
            read_non_empty_env("SDKWORK_IM_REALTIME_NODE_ID"),
            false,
        )
    }

    pub fn from_env_with_node_id(node_id: impl Into<String>) -> Self {
        let node_id = node_id.into();
        let node_id = (!node_id.trim().is_empty()).then(|| node_id.trim().to_owned());
        Self::from_env_with_optional_node_id(node_id, true)
    }

    fn from_env_with_optional_node_id(
        node_id: Option<String>,
        publish_observed_cluster_node: bool,
    ) -> Self {
        let publish_cluster_node = publish_observed_cluster_node && node_id.is_some();
        let profile = read_non_empty_env("SDKWORK_IM_PROFILE_ID")
            .unwrap_or_else(|| "unconfigured".to_owned());
        let bind_addr = read_non_empty_env("SDKWORK_IM_OPS_SERVICE_BIND_ADDR")
            .or_else(|| read_non_empty_env("SDKWORK_IM_APPLICATION_PUBLIC_INGRESS_BIND"))
            .unwrap_or_else(|| "unconfigured".to_owned());

        Self::new_internal(
            node_id.unwrap_or_else(|| "unconfigured".to_owned()),
            profile,
            bind_addr,
            Vec::new(),
            Vec::new(),
            publish_cluster_node,
        )
    }

    pub fn new(
        node_id: impl Into<String>,
        profile: impl Into<String>,
        bind_addr: impl Into<String>,
        service_names: Vec<String>,
        owned_scopes: Vec<String>,
    ) -> Self {
        Self::new_internal(
            node_id,
            profile,
            bind_addr,
            service_names,
            owned_scopes,
            true,
        )
    }

    fn new_internal(
        node_id: impl Into<String>,
        profile: impl Into<String>,
        bind_addr: impl Into<String>,
        service_names: Vec<String>,
        owned_scopes: Vec<String>,
        publish_cluster_node: bool,
    ) -> Self {
        Self {
            node_id: node_id.into(),
            profile: profile.into(),
            bind_addr: bind_addr.into(),
            publish_cluster_node,
            services: service_names
                .into_iter()
                .map(|service| ServiceHealthView {
                    service,
                    status: "unavailable".into(),
                })
                .collect(),
            owned_scopes,
            lag_items: Mutex::new(default_lag_items()),
            drain_status: Mutex::new("unavailable".into()),
            rebalance_state: Mutex::new("unavailable".into()),
            client_routes: Mutex::new(Vec::new()),
            client_route_total: Mutex::new(0),
            provider_bindings: Mutex::new(BTreeMap::new()),
            runtime_dir_inspection: Mutex::new(RuntimeDirInspectionView::unmanaged()),
            side_effect_outboxes: Mutex::new(Vec::new()),
            realtime_inbox: Mutex::new(RealtimeInboxDiagnosticsView::default()),
        }
    }

    pub fn set_node_lifecycle(&self, drain_status: &str, rebalance_state: &str) {
        *lock_ops_mutex(&self.drain_status, "ops drain status") = drain_status.into();
        *lock_ops_mutex(&self.rebalance_state, "ops rebalance state") = rebalance_state.into();
    }

    pub fn update_route_ownership(&self, client_routes: Vec<RouteOwnershipView>) {
        let total = client_routes.len();
        self.update_route_ownership_snapshot(client_routes, total);
    }

    pub fn update_route_ownership_snapshot(
        &self,
        mut client_routes: Vec<RouteOwnershipView>,
        total: usize,
    ) {
        client_routes.sort_by(|left, right| {
            left.tenant_id
                .cmp(&right.tenant_id)
                .then_with(|| left.principal_id.cmp(&right.principal_id))
                .then_with(|| left.device_id.cmp(&right.device_id))
        });
        *lock_ops_mutex(&self.client_routes, "ops client routes") = client_routes;
        *lock_ops_mutex(&self.client_route_total, "ops client route total") = total;
    }

    pub fn update_runtime_dir_inspection(&self, inspection: RuntimeDirInspectionView) {
        *lock_ops_mutex(&self.runtime_dir_inspection, "ops runtime_dir inspection") = inspection;
    }

    pub fn update_provider_binding_snapshot(&self, mut snapshot: ProviderBindingSnapshotView) {
        snapshot
            .effective_bindings
            .sort_by(|left, right| left.domain.cmp(&right.domain));
        let key = snapshot.tenant_id.clone().unwrap_or_default();
        lock_ops_mutex(&self.provider_bindings, "ops provider bindings").insert(key, snapshot);
    }

    pub fn replace_provider_binding_snapshots<I>(&self, snapshots: I)
    where
        I: IntoIterator<Item = ProviderBindingSnapshotView>,
    {
        let mut provider_bindings =
            lock_ops_mutex(&self.provider_bindings, "ops provider bindings");
        provider_bindings.clear();
        for mut snapshot in snapshots {
            snapshot
                .effective_bindings
                .sort_by(|left, right| left.domain.cmp(&right.domain));
            let key = snapshot.tenant_id.clone().unwrap_or_default();
            provider_bindings.insert(key, snapshot);
        }
    }

    pub fn update_side_effect_outboxes(
        &self,
        mut side_effect_outboxes: Vec<SideEffectOutboxDiagnosticsView>,
    ) {
        side_effect_outboxes.sort_by(|left, right| left.name.cmp(&right.name));
        *lock_ops_mutex(&self.side_effect_outboxes, "ops side-effect outboxes") =
            side_effect_outboxes;
    }

    pub fn side_effect_outboxes_view(&self) -> Vec<SideEffectOutboxDiagnosticsView> {
        lock_ops_mutex(&self.side_effect_outboxes, "ops side-effect outboxes").clone()
    }

    pub fn update_realtime_inbox(&self, realtime_inbox: RealtimeInboxDiagnosticsView) {
        *lock_ops_mutex(&self.realtime_inbox, "ops realtime inbox") = realtime_inbox;
    }

    pub fn replace_lag_items(&self, mut lag_items: Vec<LagItem>) {
        lag_items.sort_by(|left, right| {
            left.component
                .cmp(&right.component)
                .then_with(|| left.scope_id.cmp(&right.scope_id))
        });
        *lock_ops_mutex(&self.lag_items, "ops lag items") = lag_items;
    }

    pub fn node_id(&self) -> &str {
        self.node_id.as_str()
    }

    pub fn health_view(&self) -> OpsHealthResponse {
        let realtime_inbox = lock_ops_mutex(&self.realtime_inbox, "ops realtime inbox").clone();
        let status = rollup_health_status(
            self.services
                .iter()
                .map(|service| service.status.as_str())
                .chain([realtime_inbox.status.as_str()]),
        )
        .into();
        OpsHealthResponse {
            status,
            items: self.services.clone(),
            realtime_inbox,
        }
    }

    pub fn cluster_view(&self) -> ClusterView {
        if !self.publish_cluster_node {
            return ClusterView { nodes: Vec::new() };
        }
        let drain_status = lock_ops_mutex(&self.drain_status, "ops drain status").clone();
        let rebalance_state = lock_ops_mutex(&self.rebalance_state, "ops rebalance state").clone();
        let client_route_count =
            *lock_ops_mutex(&self.client_route_total, "ops client route total");
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

    pub fn lag_page(
        &self,
        query: SdkWorkCursorListQuery,
    ) -> Result<SdkWorkPageData<LagItem>, OpsError> {
        let page_size = resolve_ops_page_size(&query)?;
        let cursor = decode_ops_cursor(query.cursor.as_deref(), "lag", 2)?;
        let lag_items = lock_ops_mutex(&self.lag_items, "ops lag items");
        let start = cursor.as_ref().map_or(0, |key| {
            lag_items.partition_point(|item| {
                (item.component.as_str(), item.scope_id.as_str())
                    <= (key[0].as_str(), key[1].as_str())
            })
        });
        let mut items = lag_items
            .iter()
            .skip(start)
            .take(page_size.saturating_add(1))
            .cloned()
            .collect::<Vec<_>>();
        let has_more = items.len() > page_size;
        items.truncate(page_size);
        let next_cursor = if has_more {
            items
                .last()
                .map(|item| {
                    encode_ops_cursor("lag", vec![item.component.clone(), item.scope_id.clone()])
                })
                .transpose()?
        } else {
            None
        };
        Ok(cursor_list_page_data(
            items,
            page_size,
            next_cursor,
            has_more,
        ))
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

    pub fn provider_bindings_page(
        &self,
        query: SdkWorkCursorListQuery,
    ) -> Result<SdkWorkPageData<ProviderBindingSnapshotView>, OpsError> {
        let page_size = resolve_ops_page_size(&query)?;
        let cursor = decode_ops_cursor(query.cursor.as_deref(), "provider_bindings", 1)?;
        let provider_bindings = lock_ops_mutex(&self.provider_bindings, "ops provider bindings");
        let iter: Box<dyn Iterator<Item = (&String, &ProviderBindingSnapshotView)> + '_> =
            match cursor.as_ref() {
                Some(key) => {
                    Box::new(provider_bindings.range((Excluded(key[0].clone()), Unbounded)))
                }
                None => Box::new(provider_bindings.iter()),
            };
        let mut window = iter
            .take(page_size.saturating_add(1))
            .map(|(key, snapshot)| (key.clone(), snapshot.clone()))
            .collect::<Vec<_>>();
        let has_more = window.len() > page_size;
        window.truncate(page_size);
        let next_cursor = if has_more {
            window
                .last()
                .map(|(key, _)| encode_ops_cursor("provider_bindings", vec![key.clone()]))
                .transpose()?
        } else {
            None
        };
        Ok(cursor_list_page_data(
            window.into_iter().map(|(_, snapshot)| snapshot).collect(),
            page_size,
            next_cursor,
            has_more,
        ))
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

    pub fn provider_binding_drift_page(
        &self,
        query: SdkWorkCursorListQuery,
    ) -> Result<SdkWorkPageData<ProviderBindingDriftItemView>, OpsError> {
        let page_size = resolve_ops_page_size(&query)?;
        let cursor = decode_ops_cursor(query.cursor.as_deref(), "provider_binding_drift", 2)?;
        let provider_bindings = lock_ops_mutex(&self.provider_bindings, "ops provider bindings");
        let Some(global_snapshot) = provider_bindings.get("") else {
            return Ok(cursor_list_page_data(Vec::new(), page_size, None, false));
        };
        let baseline_bindings = global_snapshot
            .effective_bindings
            .iter()
            .map(|binding| (binding.domain.as_str(), binding))
            .collect::<BTreeMap<_, _>>();
        let tenant_iter: Box<dyn Iterator<Item = (&String, &ProviderBindingSnapshotView)> + '_> =
            match cursor.as_ref() {
                Some(key) => {
                    Box::new(provider_bindings.range((Included(key[0].clone()), Unbounded)))
                }
                None => Box::new(provider_bindings.iter()),
            };
        let mut items = Vec::with_capacity(page_size.saturating_add(1));
        'tenants: for (tenant_key, snapshot) in tenant_iter {
            if tenant_key.is_empty() {
                continue;
            }
            for binding in &snapshot.effective_bindings {
                if cursor.as_ref().is_some_and(|key| {
                    (tenant_key.as_str(), binding.domain.as_str())
                        <= (key[0].as_str(), key[1].as_str())
                }) {
                    continue;
                }
                let Some(baseline) = baseline_bindings.get(binding.domain.as_str()) else {
                    continue;
                };
                if let Some(item) =
                    provider_binding_drift_item(tenant_key.as_str(), baseline, binding)
                {
                    items.push(item);
                    if items.len() > page_size {
                        break 'tenants;
                    }
                }
            }
        }
        let has_more = items.len() > page_size;
        items.truncate(page_size);
        let next_cursor = if has_more {
            items
                .last()
                .map(|item| {
                    encode_ops_cursor(
                        "provider_binding_drift",
                        vec![item.tenant_id.clone(), item.domain.clone()],
                    )
                })
                .transpose()?
        } else {
            None
        };
        Ok(cursor_list_page_data(
            items,
            page_size,
            next_cursor,
            has_more,
        ))
    }

    pub fn diagnostic_bundle(&self) -> DiagnosticBundle {
        let drain_status = lock_ops_mutex(&self.drain_status, "ops drain status").clone();
        let rebalance_state = lock_ops_mutex(&self.rebalance_state, "ops rebalance state").clone();
        let client_routes = {
            let routes = lock_ops_mutex(&self.client_routes, "ops client routes");
            routes
                .iter()
                .take(DIAGNOSTIC_COLLECTION_LIMIT)
                .cloned()
                .collect()
        };
        let client_route_total =
            *lock_ops_mutex(&self.client_route_total, "ops client route total");
        let (provider_bindings, provider_binding_total) = {
            let bindings = lock_ops_mutex(&self.provider_bindings, "ops provider bindings");
            (
                bindings
                    .values()
                    .take(DIAGNOSTIC_COLLECTION_LIMIT)
                    .cloned()
                    .collect(),
                bindings.len(),
            )
        };
        let mut provider_binding_drift = self.provider_binding_drift_view();
        let provider_binding_drift_total = provider_binding_drift.items.len();
        provider_binding_drift
            .items
            .truncate(DIAGNOSTIC_COLLECTION_LIMIT);
        let side_effect_outboxes =
            lock_ops_mutex(&self.side_effect_outboxes, "ops side-effect outboxes").clone();
        let realtime_inbox = lock_ops_mutex(&self.realtime_inbox, "ops realtime inbox").clone();
        let (lag, lag_total) = {
            let lag = lock_ops_mutex(&self.lag_items, "ops lag items");
            (
                lag.iter()
                    .take(DIAGNOSTIC_COLLECTION_LIMIT)
                    .cloned()
                    .collect(),
                lag.len(),
            )
        };
        let collection_totals = BTreeMap::from([
            ("clientRoutes".to_owned(), client_route_total as u64),
            ("providerBindings".to_owned(), provider_binding_total as u64),
            (
                "providerBindingDrift".to_owned(),
                provider_binding_drift_total as u64,
            ),
            ("lag".to_owned(), lag_total as u64),
        ]);
        let truncated_collections = collection_totals
            .iter()
            .filter(|(_, total)| **total > DIAGNOSTIC_COLLECTION_LIMIT as u64)
            .map(|(name, _)| name.clone())
            .collect();
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
            side_effect_outboxes,
            realtime_inbox,
            collection_limit: DIAGNOSTIC_COLLECTION_LIMIT as u32,
            collection_totals,
            truncated_collections,
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
    Vec::new()
}

fn read_non_empty_env(name: &str) -> Option<String> {
    std::env::var(name)
        .ok()
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())
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
    fn test_lag_view_recovers_from_poisoned_lag_lock() {
        let runtime = OpsRuntime::default();
        let _ = std::panic::catch_unwind(|| {
            let _guard = runtime.lag_items.lock().expect("ops lag items should lock");
            panic!("poison ops lag-items lock");
        });

        assert!(runtime.lag_view().items.is_empty());
    }
}
