use std::collections::HashMap;
use std::fmt;
use std::sync::{Arc, Mutex, MutexGuard};

use im_platform_contracts::ClusterEventBus;
use im_time::utc_now_rfc3339_millis;
use serde::Deserialize;
use sdkwork_im_contract_control::RealtimeDisconnectFenceStore;
use sdkwork_im_runtime_route::{
    RouteBinding, RouteBindingRequest, RouteDirectory, RouteMigrationResult, RouteNodeLifecycle,
    RouteRuntimeError, RouteStore,
};
use tokio::sync::watch;

use crate::{
    RealtimeDeliveryRuntime,
    cluster_route_event_auth::{
        sign_cluster_route_event, verify_and_extract_cluster_route_event,
    },
    principal_scope::typed_client_route_scope_key,
    realtime::{RealtimeClientRouteStateSnapshot, RealtimeRuntimeError},
};

mod disconnect;

use disconnect::{ClusterMemoryDisconnectFenceStore, RealtimeDisconnectFence};

fn lock_cluster_mutex<'a, T>(mutex: &'a Mutex<T>, label: &'static str) -> MutexGuard<'a, T> {
    match mutex.lock() {
        Ok(guard) => guard,
        Err(poisoned) => {
            tracing::warn!("recovering poisoned mutex in session-gateway/cluster: {label}");
            poisoned.into_inner()
        }
    }
}

pub type RealtimeClientRoute = RouteBinding;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RealtimeRouteDeliveryResult {
    pub target_node_id: String,
    pub route_state: String,
    pub delivered: usize,
    pub delivery_error_code: Option<String>,
    pub delivery_error_message: Option<String>,
}

pub type RealtimeNodeLifecycleView = RouteNodeLifecycle;
pub type RealtimeRouteMigrationResult = RouteMigrationResult;

#[derive(Clone, Debug, Deserialize)]
struct ClusterRouteEventPayload {
    tenant_id: String,
    organization_id: String,
    principal_id: String,
    principal_kind: String,
    device_id: String,
    scope_type: String,
    scope_id: String,
    event_type: String,
    payload: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RealtimeClusterError {
    pub code: &'static str,
    pub message: String,
    pub node_id: String,
    pub drain_status: String,
    pub rebalance_state: String,
}

#[derive(Clone)]
pub struct RealtimeClusterBridge {
    node_runtimes: Arc<Mutex<HashMap<String, Arc<RealtimeDeliveryRuntime>>>>,
    route_store: Arc<dyn RouteStore>,
    route_epoch_notifiers: Arc<Mutex<HashMap<String, watch::Sender<u64>>>>,
    disconnect_fences: Arc<Mutex<HashMap<String, RealtimeDisconnectFence>>>,
    disconnect_fence_store: Arc<dyn RealtimeDisconnectFenceStore>,
    cluster_bus: Option<Arc<dyn ClusterEventBus>>,
    cluster_bus_secret: Option<String>,
}

impl Default for RealtimeClusterBridge {
    fn default() -> Self {
        Self::with_disconnect_fence_store(Arc::new(ClusterMemoryDisconnectFenceStore::default()))
    }
}

impl fmt::Debug for RealtimeClusterBridge {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("RealtimeClusterBridge")
            .finish_non_exhaustive()
    }
}

impl RealtimeClusterBridge {
    pub fn with_disconnect_fence_store(
        disconnect_fence_store: Arc<dyn RealtimeDisconnectFenceStore>,
    ) -> Self {
        Self::with_runtime_parts(
            disconnect_fence_store,
            Arc::new(RouteDirectory::default()) as Arc<dyn RouteStore>,
        )
    }

    pub fn with_disconnect_fence_store_and_route_store(
        disconnect_fence_store: Arc<dyn RealtimeDisconnectFenceStore>,
        route_store: Arc<dyn RouteStore>,
    ) -> Self {
        Self::with_runtime_parts(disconnect_fence_store, route_store)
    }

    fn with_runtime_parts(
        disconnect_fence_store: Arc<dyn RealtimeDisconnectFenceStore>,
        route_store: Arc<dyn RouteStore>,
    ) -> Self {
        Self {
            node_runtimes: Arc::new(Mutex::new(HashMap::new())),
            route_store,
            route_epoch_notifiers: Arc::new(Mutex::new(HashMap::new())),
            disconnect_fences: Arc::new(Mutex::new(HashMap::new())),
            disconnect_fence_store,
            cluster_bus: None,
            cluster_bus_secret: None,
        }
    }

    /// Attach a cross-node event bus for delivering route events to
    /// remote nodes. Without a bus, only local-node delivery is available.
    pub fn with_cluster_bus(mut self, bus: Arc<dyn ClusterEventBus>) -> Self {
        self.cluster_bus = Some(bus);
        self
    }

    pub fn with_cluster_bus_auth(mut self, secret: impl Into<String>) -> Self {
        self.cluster_bus_secret = Some(secret.into());
        self
    }

    pub fn bind_node_runtime(&self, node_id: &str, runtime: Arc<RealtimeDeliveryRuntime>) {
        lock_cluster_mutex(&self.node_runtimes, "node_runtimes").insert(node_id.into(), runtime);
        self.route_store.register_node(node_id);
    }

    #[allow(clippy::too_many_arguments)]
    pub fn bind_client_route_for_principal_kind(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_id: &str,
        principal_kind: &str,
        device_id: &str,
        owner_node_id: &str,
        session_id: Option<&str>,
        connection_kind: &str,
    ) -> Result<RealtimeClientRoute, RealtimeClusterError> {
        self.bind_client_route_internal(
            tenant_id,
            organization_id,
            principal_id,
            principal_kind,
            device_id,
            owner_node_id,
            session_id,
            connection_kind,
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn bind_client_route_internal(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_id: &str,
        principal_kind: &str,
        device_id: &str,
        owner_node_id: &str,
        session_id: Option<&str>,
        connection_kind: &str,
    ) -> Result<RealtimeClientRoute, RealtimeClusterError> {
        self.require_known_node(owner_node_id)?;
        let previous_route = self.resolve_client_route_internal(
            tenant_id,
            organization_id,
            principal_id,
            principal_kind,
            device_id,
        );
        let mut moved_state: Option<(
            Arc<RealtimeDeliveryRuntime>,
            Arc<RealtimeDeliveryRuntime>,
            String,
        )> = None;
        if let Some(previous_route) = previous_route.as_ref()
            && previous_route.owner_node_id != owner_node_id
        {
            let target_runtime = self.require_runtime(owner_node_id)?;
            match self.require_runtime(previous_route.owner_node_id.as_str()) {
                Ok(source_runtime) => {
                    self.move_client_route_state_between_runtimes(
                        &source_runtime,
                        &target_runtime,
                        tenant_id,
                        organization_id,
                        principal_id,
                        principal_kind,
                        device_id,
                        previous_route.owner_node_id.as_str(),
                        owner_node_id,
                        "route rebind",
                    )?;
                    moved_state = Some((
                        source_runtime,
                        target_runtime,
                        previous_route.owner_node_id.clone(),
                    ));
                }
                Err(error) if error.code == "node_runtime_missing" => {
                    // The previous owner runtime is already gone, so in-memory state
                    // cannot be handed off. Keep availability by moving ownership to the
                    // new active node and let the target restore any durable checkpoint
                    // state it knows about.
                    target_runtime
                        .ensure_client_route_state_for_principal_kind(
                            tenant_id,
                            organization_id,
                            principal_id,
                            principal_kind,
                            device_id,
                        )
                        .map_err(|error| {
                            self.runtime_store_error(
                                "restore durable checkpoint during route rebind",
                                owner_node_id,
                                error,
                            )
                        })?;
                }
                Err(error) => return Err(error),
            }
        }

        let binding_request = RouteBindingRequest::new(
            tenant_id,
            principal_id,
            principal_kind,
            device_id,
            owner_node_id,
        )
        .with_organization_id(organization_id)
        .with_session_id(session_id)
        .with_connection_kind(connection_kind)
        .with_bound_at(cluster_timestamp());
        let binding = match self.route_store.bind(binding_request) {
            Ok(binding) => binding,
            Err(error) => {
                let route_error = self.route_error(error);
                if let Some((source_runtime, target_runtime, source_node_id)) = moved_state.as_ref()
                {
                    return Err(
                        self.rollback_moved_client_route_states_after_route_commit_error(
                            source_runtime,
                            target_runtime,
                            &[previous_route
                                .as_ref()
                                .expect("moved runtime state should have previous route")
                                .clone()],
                            source_node_id.as_str(),
                            owner_node_id,
                            "route rebind",
                            route_error,
                        ),
                    );
                }
                return Err(route_error);
            }
        };
        self.observe_route_epoch_internal(
            tenant_id,
            organization_id,
            principal_id,
            principal_kind,
            device_id,
            binding.route_epoch,
        );
        Ok(binding)
    }

    pub fn ensure_route_session_current_for_principal_kind(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_id: &str,
        principal_kind: &str,
        device_id: &str,
        session_id: Option<&str>,
    ) -> Result<(), RealtimeClusterError> {
        self.ensure_route_session_current_internal(
            tenant_id,
            organization_id,
            principal_id,
            principal_kind,
            device_id,
            session_id,
        )
    }

    fn ensure_route_session_current_internal(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_id: &str,
        principal_kind: &str,
        device_id: &str,
        session_id: Option<&str>,
    ) -> Result<(), RealtimeClusterError> {
        let Some(route) = self.resolve_client_route_internal(
            tenant_id,
            organization_id,
            principal_id,
            principal_kind,
            device_id,
        ) else {
            return Ok(());
        };
        let Some(current_session_id) = route.session_id.as_deref() else {
            return Ok(());
        };
        let Some(requested_session_id) = session_id else {
            return Err(self.node_error(
                "session_id_required",
                route.owner_node_id.as_str(),
                format!(
                    "client route session id is required because the route is currently owned by node {}",
                    route.owner_node_id
                ),
            ));
        };
        if current_session_id == requested_session_id {
            return Ok(());
        }

        Err(self.node_error(
            "stale_session",
            route.owner_node_id.as_str(),
            format!(
                "client route session is owned by a newer session on node {}",
                route.owner_node_id
            ),
        ))
    }

    pub fn ensure_client_route_local_for_principal_kind(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_id: &str,
        principal_kind: &str,
        device_id: &str,
        local_node_id: &str,
    ) -> Result<(), RealtimeClusterError> {
        self.ensure_client_route_local_internal(
            tenant_id,
            organization_id,
            principal_id,
            principal_kind,
            device_id,
            local_node_id,
        )
    }

    fn ensure_client_route_local_internal(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_id: &str,
        principal_kind: &str,
        device_id: &str,
        local_node_id: &str,
    ) -> Result<(), RealtimeClusterError> {
        let Some(route) = self.resolve_client_route_internal(
            tenant_id,
            organization_id,
            principal_id,
            principal_kind,
            device_id,
        ) else {
            return Ok(());
        };
        if route.owner_node_id == local_node_id {
            return Ok(());
        }

        if self
            .node_lifecycle(local_node_id)
            .is_some_and(|node| node.drain_status != "active")
        {
            return Err(self.node_error(
                "node_draining",
                local_node_id,
                format!(
                    "node {local_node_id} cannot rebind a client route currently owned by node {} while draining",
                    route.owner_node_id
                ),
            ));
        }

        Err(self.node_error(
            "route_owned_by_other_node",
            route.owner_node_id.as_str(),
            format!(
                "client route is currently owned by node {}",
                route.owner_node_id
            ),
        ))
    }

    pub fn resolve_client_route_for_principal_kind(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_id: &str,
        principal_kind: &str,
        device_id: &str,
    ) -> Option<RealtimeClientRoute> {
        self.resolve_client_route_internal(
            tenant_id,
            organization_id,
            principal_id,
            principal_kind,
            device_id,
        )
    }

    fn resolve_client_route_internal(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_id: &str,
        principal_kind: &str,
        device_id: &str,
    ) -> Option<RealtimeClientRoute> {
        self.route_store.lookup(
            tenant_id,
            organization_id,
            principal_id,
            principal_kind,
            device_id,
        )
    }

    pub fn subscribe_client_route_epoch_for_principal_kind(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_id: &str,
        principal_kind: &str,
        device_id: &str,
    ) -> watch::Receiver<u64> {
        self.subscribe_client_route_epoch_internal(
            tenant_id,
            organization_id,
            principal_id,
            principal_kind,
            device_id,
        )
    }

    fn subscribe_client_route_epoch_internal(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_id: &str,
        principal_kind: &str,
        device_id: &str,
    ) -> watch::Receiver<u64> {
        let scope_key = client_route_scope_key(tenant_id, organization_id, principal_id, principal_kind, device_id);
        let current_epoch = self
            .resolve_client_route_internal(
                tenant_id,
                organization_id,
                principal_id,
                principal_kind,
                device_id,
            )
            .map(|route| route.route_epoch)
            .unwrap_or(0);
        let sender = lock_cluster_mutex(&self.route_epoch_notifiers, "route_epoch_notifiers")
            .entry(scope_key)
            .or_insert_with(|| {
                let (sender, _) = watch::channel(current_epoch);
                sender
            })
            .clone();
        if *sender.borrow() != current_epoch {
            let _ = sender.send(current_epoch);
        }
        sender.subscribe()
    }

    pub fn release_client_route_for_principal_kind(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_id: &str,
        principal_kind: &str,
        device_id: &str,
        owner_node_id: &str,
    ) -> Option<RealtimeClientRoute> {
        let released = self.release_client_route_internal(
            tenant_id,
            organization_id,
            principal_id,
            principal_kind,
            device_id,
            owner_node_id,
        );

        // Drop in-memory realtime delivery state to prevent unbounded growth from
        // disconnected clients. This cleans up the 13 per-client HashMaps that
        // would otherwise retain stale subscriptions, event windows, and watchers.
        if let Some(runtime) = lock_cluster_mutex(&self.node_runtimes, "node_runtimes")
            .get(owner_node_id)
            .cloned()
        {
            if let Err(error) = runtime.drop_client_route_state(
                tenant_id,
                organization_id,
                principal_id,
                principal_kind,
                device_id,
            ) {
                tracing::warn!(
                    error = ?error,
                    tenant_id = %tenant_id,
                    principal_id = %principal_id,
                    device_id = %device_id,
                    "failed to drop realtime client route state on disconnect"
                );
            }
        }

        released
    }

    pub fn restore_client_route_if_current(
        &self,
        expected_current: &RealtimeClientRoute,
        restore_to: RealtimeClientRoute,
    ) -> Option<RealtimeClientRoute> {
        let restored = self
            .route_store
            .restore_if_current(expected_current, restore_to);
        if let Some(restored_route) = restored.as_ref() {
            self.observe_route_epoch_internal(
                restored_route.tenant_id.as_str(),
                restored_route.organization_id.as_str(),
                restored_route.principal_id.as_str(),
                restored_route.principal_kind.as_str(),
                restored_route.device_id.as_str(),
                restored_route.route_epoch,
            );
        }
        restored
    }

    fn release_client_route_internal(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_id: &str,
        principal_kind: &str,
        device_id: &str,
        owner_node_id: &str,
    ) -> Option<RealtimeClientRoute> {
        self.route_store.release(
            tenant_id,
            organization_id,
            principal_id,
            principal_kind,
            device_id,
            owner_node_id,
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn move_client_route_state_between_runtimes(
        &self,
        source_runtime: &Arc<RealtimeDeliveryRuntime>,
        target_runtime: &Arc<RealtimeDeliveryRuntime>,
        tenant_id: &str,
        organization_id: &str,
        principal_id: &str,
        principal_kind: &str,
        device_id: &str,
        source_node_id: &str,
        target_node_id: &str,
        operation: &'static str,
    ) -> Result<(), RealtimeClusterError> {
        let take_context = match operation {
            "route rebind" => "take client route state for route rebind",
            "route migration" => "take client route state for route migration",
            _ => "take client route state",
        };
        let restore_context = match operation {
            "route rebind" => "restore client route state for route rebind",
            "route migration" => "restore client route state for route migration",
            _ => "restore client route state",
        };
        let snapshot: RealtimeClientRouteStateSnapshot = source_runtime
            .take_client_route_state_for_principal_kind(
                tenant_id,
                organization_id,
                principal_id,
                principal_kind,
                device_id,
            )
            .map_err(|error| self.runtime_store_error(take_context, source_node_id, error))?;
        if let Err(target_error) = target_runtime.restore_client_route_state(snapshot.clone()) {
            if let Err(source_error) = source_runtime.restore_client_route_state(snapshot) {
                return Err(self.node_error(
                    "runtime_state_compensation_failed",
                    source_node_id,
                    format!(
                        "{operation} failed and rollback restore failed; target restore on node {target_node_id} failed: {}; source compensation restore on node {source_node_id} failed: {}",
                        target_error.message, source_error.message
                    ),
                ));
            }
            return Err(self.runtime_store_error(restore_context, target_node_id, target_error));
        }
        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    fn rollback_moved_client_route_states_after_route_commit_error(
        &self,
        source_runtime: &Arc<RealtimeDeliveryRuntime>,
        target_runtime: &Arc<RealtimeDeliveryRuntime>,
        moved_routes: &[RealtimeClientRoute],
        source_node_id: &str,
        target_node_id: &str,
        operation: &'static str,
        original_error: RealtimeClusterError,
    ) -> RealtimeClusterError {
        let mut rollback_errors = Vec::new();
        for route in moved_routes.iter().rev() {
            if let Err(error) = self.move_client_route_state_between_runtimes(
                target_runtime,
                source_runtime,
                route.tenant_id.as_str(),
                route.organization_id.as_str(),
                route.principal_id.as_str(),
                route.principal_kind.as_str(),
                route.device_id.as_str(),
                target_node_id,
                source_node_id,
                operation,
            ) {
                rollback_errors.push(format!(
                    "{}:{}:{}:{} -> {}",
                    route.tenant_id,
                    route.principal_kind,
                    route.principal_id,
                    route.device_id,
                    error.message
                ));
            }
        }

        if rollback_errors.is_empty() {
            return original_error;
        }

        self.node_error(
            "runtime_state_compensation_failed",
            source_node_id,
            format!(
                "{operation} route commit failed: {}; state rollback from node {target_node_id} to node {source_node_id} failed for {} route(s): {}",
                original_error.message,
                rollback_errors.len(),
                rollback_errors.join("; ")
            ),
        )
    }

    pub fn routes_for_node(&self, owner_node_id: &str) -> Vec<RealtimeClientRoute> {
        self.route_store.routes_for_node(owner_node_id)
    }

    pub fn node_lifecycle(&self, node_id: &str) -> Option<RealtimeNodeLifecycleView> {
        self.route_store.node_lifecycle(node_id)
    }

    pub fn mark_node_draining(
        &self,
        node_id: &str,
    ) -> Result<RealtimeNodeLifecycleView, RealtimeClusterError> {
        self.route_store
            .mark_node_draining(node_id)
            .map_err(|error| self.route_error(error))
    }

    pub fn activate_node(
        &self,
        node_id: &str,
    ) -> Result<RealtimeNodeLifecycleView, RealtimeClusterError> {
        self.route_store
            .activate_node(node_id)
            .map_err(|error| self.route_error(error))
    }

    pub fn migrate_node_routes(
        &self,
        source_node_id: &str,
        target_node_id: &str,
    ) -> Result<RealtimeRouteMigrationResult, RealtimeClusterError> {
        if source_node_id == target_node_id {
            return Err(self.node_error(
                "same_node_migration",
                source_node_id,
                "source and target nodes must be different".into(),
            ));
        }

        let source_state = self.node_lifecycle(source_node_id).ok_or_else(|| {
            self.node_error(
                "node_not_found",
                source_node_id,
                format!("source node not found: {source_node_id}"),
            )
        })?;
        if source_state.drain_status != "draining" {
            return Err(self.node_error(
                "node_not_draining",
                source_node_id,
                format!("source node must be draining before migration: {source_node_id}"),
            ));
        }

        let target_state = self.node_lifecycle(target_node_id).ok_or_else(|| {
            self.node_error(
                "target_node_not_found",
                target_node_id,
                format!("target node not found: {target_node_id}"),
            )
        })?;
        if target_state.drain_status != "active" || target_state.rebalance_state != "stable" {
            return Err(self.node_error(
                "target_node_unavailable",
                target_node_id,
                format!("target node is not available for route migration: {target_node_id}"),
            ));
        }

        let routes = self.routes_for_node(source_node_id);
        let mut moved_routes = Vec::new();
        let mut runtime_pair: Option<(Arc<RealtimeDeliveryRuntime>, Arc<RealtimeDeliveryRuntime>)> =
            None;
        if !routes.is_empty() {
            let source_runtime = self.require_runtime(source_node_id)?;
            let target_runtime = self.require_runtime(target_node_id)?;

            for route in &routes {
                if let Err(error) = self.move_client_route_state_between_runtimes(
                    &source_runtime,
                    &target_runtime,
                    route.tenant_id.as_str(),
                    route.organization_id.as_str(),
                    route.principal_id.as_str(),
                    route.principal_kind.as_str(),
                    route.device_id.as_str(),
                    source_node_id,
                    target_node_id,
                    "route migration",
                ) {
                    if moved_routes.is_empty() {
                        return Err(error);
                    }
                    return Err(
                        self.rollback_moved_client_route_states_after_route_commit_error(
                            &source_runtime,
                            &target_runtime,
                            &moved_routes,
                            source_node_id,
                            target_node_id,
                            "route migration",
                            error,
                        ),
                    );
                }
                moved_routes.push(route.clone());
            }
            runtime_pair = Some((source_runtime, target_runtime));
        }

        let migration = match self.route_store.migrate_routes_at(
            source_node_id,
            target_node_id,
            cluster_timestamp().as_str(),
        ) {
            Ok(migration) => migration,
            Err(error) => {
                let route_error = self.route_error(error);
                if let Some((source_runtime, target_runtime)) = runtime_pair.as_ref() {
                    return Err(
                        self.rollback_moved_client_route_states_after_route_commit_error(
                            source_runtime,
                            target_runtime,
                            &moved_routes,
                            source_node_id,
                            target_node_id,
                            "route migration",
                            route_error,
                        ),
                    );
                }
                return Err(route_error);
            }
        };
        for route in routes {
            if let Some(current_route) = self.resolve_client_route_internal(
                route.tenant_id.as_str(),
                route.organization_id.as_str(),
                route.principal_id.as_str(),
                route.principal_kind.as_str(),
                route.device_id.as_str(),
            ) {
                self.observe_route_epoch_internal(
                    current_route.tenant_id.as_str(),
                    current_route.organization_id.as_str(),
                    current_route.principal_id.as_str(),
                    current_route.principal_kind.as_str(),
                    current_route.device_id.as_str(),
                    current_route.route_epoch,
                );
            }
        }
        Ok(migration)
    }

    pub fn ingest_cluster_route_event_for_node(
        &self,
        own_node_id: &str,
        event_json: &str,
    ) -> Result<RealtimeRouteDeliveryResult, RealtimeClusterError> {
        let secret = self.require_cluster_bus_secret(own_node_id)?;
        let verified_event = verify_and_extract_cluster_route_event(secret, event_json)
            .map_err(|error| {
                self.node_error(
                    "invalid_cluster_route_event_signature",
                    own_node_id,
                    error,
                )
            })?;
        let parsed = serde_json::from_value::<ClusterRouteEventPayload>(verified_event).map_err(
            |error| {
                self.node_error(
                    "invalid_cluster_route_event",
                    own_node_id,
                    format!("invalid cluster route event json: {error}"),
                )
            },
        )?;
        let runtime = self.require_runtime(own_node_id)?;
        let delivered = runtime
            .publish_scope_event_for_principal_kind(
                parsed.tenant_id.as_str(),
                parsed.organization_id.as_str(),
                parsed.principal_id.as_str(),
                parsed.principal_kind.as_str(),
                parsed.scope_type.as_str(),
                parsed.scope_id.as_str(),
                parsed.event_type.as_str(),
                parsed.payload,
                vec![parsed.device_id.clone()],
            )
            .map_err(|error| {
                self.node_error(
                    "cluster_route_delivery_failed",
                    own_node_id,
                    format!("cluster route delivery failed: {}", error.message),
                )
            })?;

        Ok(RealtimeRouteDeliveryResult {
            target_node_id: own_node_id.to_owned(),
            route_state: "cluster_ingress".to_string(),
            delivered,
            delivery_error_code: None,
            delivery_error_message: None,
        })
    }

    fn require_cluster_bus_secret(
        &self,
        own_node_id: &str,
    ) -> Result<&str, RealtimeClusterError> {
        self.cluster_bus_secret.as_deref().ok_or_else(|| {
            self.node_error(
                "cluster_bus_secret_missing",
                own_node_id,
                "cluster bus secret is not configured on this node".to_string(),
            )
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub fn publish_client_route_event_for_principal_kind(
        &self,
        origin_node_id: &str,
        tenant_id: &str,
        organization_id: &str,
        principal_id: &str,
        principal_kind: &str,
        device_id: &str,
        scope_type: &str,
        scope_id: &str,
        event_type: &str,
        payload: String,
    ) -> RealtimeRouteDeliveryResult {
        self.publish_client_route_event_internal(
            origin_node_id,
            tenant_id,
            organization_id,
            principal_id,
            principal_kind,
            device_id,
            scope_type,
            scope_id,
            event_type,
            payload,
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn publish_client_route_event_internal(
        &self,
        origin_node_id: &str,
        tenant_id: &str,
        organization_id: &str,
        principal_id: &str,
        principal_kind: &str,
        device_id: &str,
        scope_type: &str,
        scope_id: &str,
        event_type: &str,
        payload: String,
    ) -> RealtimeRouteDeliveryResult {
        let route = self.resolve_client_route_internal(
            tenant_id,
            organization_id,
            principal_id,
            principal_kind,
            device_id,
        );
        let runtimes = lock_cluster_mutex(&self.node_runtimes, "node_runtimes");
        let (target_node_id, route_state, runtime) = match route {
            Some(ref route) => {
                let target_node_id = route.owner_node_id.clone();
                let runtime = runtimes.get(target_node_id.as_str()).cloned();
                let route_state = if runtime.is_some() {
                    "resolved"
                } else {
                    "target_runtime_missing"
                };
                (target_node_id, route_state, runtime)
            }
            None => {
                let runtime = runtimes.get(origin_node_id).cloned();
                let route_state = if runtime.is_some() {
                    "local_fallback"
                } else {
                    "origin_runtime_missing"
                };
                (origin_node_id.into(), route_state, runtime)
            }
        };
        drop(runtimes);

        // Try the cluster bus for remote nodes when the target runtime
        // is not available locally.
        if runtime.is_none()
            && route_state == "target_runtime_missing"
            && let Some(ref bus) = self.cluster_bus
        {
            let event = serde_json::json!({
                "tenant_id": tenant_id,
                "principal_id": principal_id,
                "principal_kind": principal_kind,
                "device_id": device_id,
                "scope_type": scope_type,
                "scope_id": scope_id,
                "event_type": event_type,
                "payload": payload,
            });
            let secret = match self.cluster_bus_secret.as_deref() {
                Some(secret) => secret,
                None => {
                    return RealtimeRouteDeliveryResult {
                        target_node_id,
                        route_state: "remote_publish_failed".to_string(),
                        delivered: 0,
                        delivery_error_code: Some("cluster_bus_secret_missing".to_string()),
                        delivery_error_message: Some(
                            "cluster bus secret is not configured on this node".to_string(),
                        ),
                    };
                }
            };
            let event_json = match sign_cluster_route_event(secret, &event) {
                Ok(value) => value,
                Err(error) => {
                    return RealtimeRouteDeliveryResult {
                        target_node_id,
                        route_state: "remote_publish_failed".to_string(),
                        delivered: 0,
                        delivery_error_code: Some("cluster_bus_sign_failed".to_string()),
                        delivery_error_message: Some(error),
                    };
                }
            };
            match bus.publish_route_event(target_node_id.as_str(), &event_json) {
                Ok(()) => {
                    return RealtimeRouteDeliveryResult {
                        target_node_id,
                        route_state: "remote_published".to_string(),
                        delivered: 1,
                        delivery_error_code: None,
                        delivery_error_message: None,
                    };
                }
                Err(error) => {
                    return RealtimeRouteDeliveryResult {
                        target_node_id,
                        route_state: "remote_publish_failed".to_string(),
                        delivered: 0,
                        delivery_error_code: Some("cluster_bus_error".to_string()),
                        delivery_error_message: Some(error),
                    };
                }
            }
        }

        let (delivered, delivery_error_code, delivery_error_message) = runtime
            .map(|runtime| {
                runtime.publish_scope_event_for_principal_kind(
                    tenant_id,
                    organization_id,
                    principal_id,
                    principal_kind,
                    scope_type,
                    scope_id,
                    event_type,
                    payload,
                    vec![device_id.into()],
                )
            })
            .map(|result| match result {
                Ok(delivered) => (delivered, None, None),
                Err(error) => (
                    0,
                    Some(error.code.to_string()),
                    Some(error.message.to_string()),
                ),
            })
            .unwrap_or((0, None, None));

        RealtimeRouteDeliveryResult {
            target_node_id,
            route_state: route_state.to_string(),
            delivered,
            delivery_error_code,
            delivery_error_message,
        }
    }

    fn require_known_node(&self, node_id: &str) -> Result<(), RealtimeClusterError> {
        if self.route_store.node_lifecycle(node_id).is_some() {
            return Ok(());
        }

        Err(self.node_error(
            "node_not_found",
            node_id,
            format!("node not found: {node_id}"),
        ))
    }

    fn require_runtime(
        &self,
        node_id: &str,
    ) -> Result<Arc<RealtimeDeliveryRuntime>, RealtimeClusterError> {
        lock_cluster_mutex(&self.node_runtimes, "node_runtimes")
            .get(node_id)
            .cloned()
            .ok_or_else(|| {
                self.node_error(
                    "node_runtime_missing",
                    node_id,
                    format!("node runtime not found: {node_id}"),
                )
            })
    }

    fn node_error(
        &self,
        code: &'static str,
        node_id: &str,
        message: String,
    ) -> RealtimeClusterError {
        let lifecycle = self.node_lifecycle(node_id);
        RealtimeClusterError {
            code,
            message,
            node_id: node_id.into(),
            drain_status: lifecycle
                .as_ref()
                .map(|item| item.drain_status.clone())
                .unwrap_or_else(|| "unknown".into()),
            rebalance_state: lifecycle
                .as_ref()
                .map(|item| item.rebalance_state.clone())
                .unwrap_or_else(|| "unknown".into()),
        }
    }

    fn runtime_store_error(
        &self,
        action: &str,
        node_id: &str,
        error: RealtimeRuntimeError,
    ) -> RealtimeClusterError {
        self.node_error(
            error.code,
            node_id,
            format!("{action} failed: {}", error.message),
        )
    }

    fn route_error(&self, error: RouteRuntimeError) -> RealtimeClusterError {
        self.node_error(error.code, error.node_id.as_str(), error.message)
    }

    fn observe_route_epoch_internal(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_id: &str,
        principal_kind: &str,
        device_id: &str,
        route_epoch: u64,
    ) {
        let scope_key = client_route_scope_key(tenant_id, organization_id, principal_id, principal_kind, device_id);
        let sender = lock_cluster_mutex(&self.route_epoch_notifiers, "route_epoch_notifiers")
            .entry(scope_key)
            .or_insert_with(|| {
                let (sender, _) = watch::channel(route_epoch);
                sender
            })
            .clone();
        if *sender.borrow() != route_epoch {
            let _ = sender.send(route_epoch);
        }
    }
}

fn client_route_scope_key(
    tenant_id: &str,
    organization_id: &str,
    principal_id: &str,
    principal_kind: &str,
    device_id: &str,
) -> String {
    typed_client_route_scope_key(
        tenant_id,
        organization_id,
        principal_id,
        principal_kind,
        device_id,
    )
}

fn cluster_timestamp() -> String {
    utc_now_rfc3339_millis()
}

#[cfg(test)]
mod tests {
    use std::panic::{self, AssertUnwindSafe};
    use std::sync::{Arc, Mutex};

    use im_adapters_local_memory::MemoryRealtimeDisconnectFenceStore;
    use im_platform_contracts::{
        ContractError, RealtimeDisconnectFenceRecord, RealtimeDisconnectFenceStore,
    };

    use super::*;
    use crate::RealtimeSubscriptionItemInput;

    fn expect_ok<T>(result: Result<T, crate::realtime::RealtimeRuntimeError>) -> T {
        result.expect("realtime runtime operation should succeed")
    }

    fn poison_mutex<T>(mutex: &Mutex<T>) {
        let _ = panic::catch_unwind(AssertUnwindSafe(|| {
            let _guard = mutex.lock().expect("test poison lock should succeed");
            panic!("intentional poison for regression coverage");
        }));
    }

    #[test]
    fn test_bind_node_runtime_recovers_from_poisoned_runtime_registry_lock() {
        let cluster = RealtimeClusterBridge::default();
        poison_mutex(&cluster.node_runtimes);

        let result = panic::catch_unwind(AssertUnwindSafe(|| {
            cluster.bind_node_runtime(
                "node_a",
                Arc::new(RealtimeDeliveryRuntime::permissive_for_tests()),
            );
        }));
        assert!(
            result.is_ok(),
            "bind_node_runtime should not panic when runtime registry mutex is poisoned"
        );
        assert!(cluster.node_lifecycle("node_a").is_some());
    }

    #[test]
    fn test_route_rebind_recovers_from_poisoned_runtime_registry_lock() {
        let cluster = RealtimeClusterBridge::default();
        cluster.bind_node_runtime(
            "node_a",
            Arc::new(RealtimeDeliveryRuntime::permissive_for_tests()),
        );
        cluster.bind_node_runtime(
            "node_b",
            Arc::new(RealtimeDeliveryRuntime::permissive_for_tests()),
        );
        cluster
            .bind_client_route_for_principal_kind(
                "100001",
                "default",
                "1",
                "user",
                "d_pad",
                "node_a",
                Some("s_old"),
                "websocket",
            )
            .expect("initial route bind should succeed");

        poison_mutex(&cluster.node_runtimes);

        let result = panic::catch_unwind(AssertUnwindSafe(|| {
            cluster.bind_client_route_for_principal_kind(
                "100001",
                "default",
                "1",
                "user",
                "d_pad",
                "node_b",
                Some("s_new"),
                "http",
            )
        }));
        assert!(
            result.is_ok(),
            "route rebind should not panic when runtime registry mutex is poisoned"
        );
        let bind_result = result.expect("panic status should be captured");
        assert!(
            bind_result.is_ok(),
            "route rebind should recover from poisoned runtime registry lock"
        );
    }

    #[test]
    fn test_publish_recovers_from_poisoned_runtime_registry_lock() {
        let cluster = RealtimeClusterBridge::default();
        let runtime_a = Arc::new(RealtimeDeliveryRuntime::permissive_for_tests());
        cluster.bind_node_runtime("node_a", runtime_a.clone());
        expect_ok(runtime_a.sync_subscriptions_for_principal_kind(
            "100001",
            "default",
            "1",
            "user",
            "d_pad",
            vec![RealtimeSubscriptionItemInput {
                scope_type: "conversation".into(),
                scope_id: "c_demo".into(),
                event_types: vec!["message.posted".into()],
            }],
        ));

        poison_mutex(&cluster.node_runtimes);

        let result = panic::catch_unwind(AssertUnwindSafe(|| {
            cluster.publish_client_route_event_for_principal_kind(
                "node_a",
                "100001",
                "default",
                "1",
                "user",
                "d_pad",
                "conversation",
                "c_demo",
                "message.posted",
                r#"{"messageId":"msg_poison"}"#.into(),
            )
        }));
        assert!(
            result.is_ok(),
            "publish should not panic when runtime registry mutex is poisoned"
        );
        let publish_result = result.expect("panic status should be captured");
        assert_eq!(publish_result.target_node_id, "node_a");
        assert_eq!(publish_result.route_state, "local_fallback");
        assert_eq!(publish_result.delivered, 1);
    }

    #[test]
    fn test_publish_does_not_fallback_to_origin_when_route_points_to_missing_target_runtime() {
        let cluster = RealtimeClusterBridge::default();
        let runtime_a = Arc::new(RealtimeDeliveryRuntime::permissive_for_tests());
        let runtime_b = Arc::new(RealtimeDeliveryRuntime::permissive_for_tests());
        cluster.bind_node_runtime("node_a", runtime_a.clone());
        cluster.bind_node_runtime("node_b", runtime_b);

        expect_ok(runtime_a.sync_subscriptions_for_principal_kind(
            "100001",
            "default",
            "1",
            "user",
            "d_pad",
            vec![RealtimeSubscriptionItemInput {
                scope_type: "conversation".into(),
                scope_id: "c_demo".into(),
                event_types: vec!["message.posted".into()],
            }],
        ));
        cluster
            .bind_client_route_for_principal_kind(
                "100001",
                "default",
                "1",
                "user",
                "d_pad",
                "node_b",
                None,
                "websocket",
            )
            .expect("route bind should succeed");

        cluster
            .node_runtimes
            .lock()
            .expect("realtime cluster runtime registry should lock")
            .remove("node_b");

        let result = cluster.publish_client_route_event_for_principal_kind(
            "node_a",
            "100001",
            "default",
            "1",
            "user",
            "d_pad",
            "conversation",
            "c_demo",
            "message.posted",
            r#"{"messageId":"msg_demo_1"}"#.into(),
        );

        assert_eq!(result.target_node_id, "node_b");
        assert_eq!(result.route_state, "target_runtime_missing");
        assert_eq!(result.delivered, 0);

        let origin_window = expect_ok(
            runtime_a.list_events_for_principal_kind("100001", "default", "1", "user", "d_pad", 0, 10),
        );
        assert_eq!(origin_window.items.len(), 0);
    }

    #[test]
    fn test_direct_rebind_self_heals_stale_route_when_previous_runtime_is_missing() {
        let cluster = RealtimeClusterBridge::default();
        let runtime_a = Arc::new(RealtimeDeliveryRuntime::permissive_for_tests());
        let runtime_b = Arc::new(RealtimeDeliveryRuntime::permissive_for_tests());
        cluster.bind_node_runtime("node_a", runtime_a);
        cluster.bind_node_runtime("node_b", runtime_b.clone());

        cluster
            .bind_client_route_for_principal_kind(
                "100001",
                "default",
                "1",
                "user",
                "d_pad",
                "node_a",
                Some("s_old"),
                "websocket",
            )
            .expect("initial route bind should succeed");
        cluster
            .mark_node_draining("node_a")
            .expect("source drain should succeed");

        cluster
            .node_runtimes
            .lock()
            .expect("realtime cluster runtime registry should lock")
            .remove("node_a");

        let rebound = cluster
            .bind_client_route_for_principal_kind(
                "100001",
                "default",
                "1",
                "user",
                "d_pad",
                "node_b",
                Some("s_new"),
                "http",
            )
            .expect("stale route should not block takeover when previous runtime is missing");
        assert_eq!(rebound.owner_node_id, "node_b");
        assert_eq!(rebound.connection_kind, "http");
        assert_eq!(rebound.session_id.as_deref(), Some("s_new"));

        let source_lifecycle = cluster
            .node_lifecycle("node_a")
            .expect("source lifecycle should remain observable");
        assert_eq!(source_lifecycle.drain_status, "drained");
        assert_eq!(source_lifecycle.rebalance_state, "stable");
        assert_eq!(source_lifecycle.owned_route_count, 0);

        expect_ok(runtime_b.sync_subscriptions_for_principal_kind(
            "100001",
            "default",
            "1",
            "user",
            "d_pad",
            vec![RealtimeSubscriptionItemInput {
                scope_type: "conversation".into(),
                scope_id: "c_demo".into(),
                event_types: vec!["message.posted".into()],
            }],
        ));

        let publish = cluster.publish_client_route_event_for_principal_kind(
            "node_a",
            "100001",
            "default",
            "1",
            "user",
            "d_pad",
            "conversation",
            "c_demo",
            "message.posted",
            r#"{"messageId":"msg_after_stale_takeover"}"#.into(),
        );

        assert_eq!(publish.target_node_id, "node_b");
        assert_eq!(publish.route_state, "resolved");
        assert_eq!(publish.delivered, 1);

        let target_window = expect_ok(
            runtime_b.list_events_for_principal_kind("100001", "default", "1", "user", "d_pad", 0, 10),
        );
        assert_eq!(target_window.items.len(), 1);
        assert_eq!(target_window.items[0].event_type, "message.posted");
    }

    #[test]
    fn test_route_session_fence_rejects_stale_session_after_takeover() {
        let cluster = RealtimeClusterBridge::default();
        let runtime_a = Arc::new(RealtimeDeliveryRuntime::permissive_for_tests());
        let runtime_b = Arc::new(RealtimeDeliveryRuntime::permissive_for_tests());
        cluster.bind_node_runtime("node_a", runtime_a);
        cluster.bind_node_runtime("node_b", runtime_b);

        cluster
            .bind_client_route_for_principal_kind(
                "100001",
                "default",
                "1",
                "user",
                "d_pad",
                "node_a",
                Some("s_old"),
                "websocket",
            )
            .expect("initial route bind should succeed");
        cluster
            .bind_client_route_for_principal_kind(
                "100001",
                "default",
                "1",
                "user",
                "d_pad",
                "node_b",
                Some("s_new"),
                "http",
            )
            .expect("takeover route bind should succeed");

        let stale_error = cluster
            .ensure_route_session_current_for_principal_kind(
                "100001",
                "default",
                "1",
                "user",
                "d_pad",
                Some("s_old"),
            )
            .expect_err("stale session should be rejected after takeover");
        assert_eq!(stale_error.code, "stale_session");
        assert_eq!(stale_error.node_id, "node_b");

        cluster
            .ensure_route_session_current_for_principal_kind(
                "100001",
                "default",
                "1",
                "user",
                "d_pad",
                Some("s_new"),
            )
            .expect("current session should remain valid");
    }

    #[test]
    fn test_route_session_fence_requires_session_id_once_route_is_bound_to_session() {
        let cluster = RealtimeClusterBridge::default();
        let runtime = Arc::new(RealtimeDeliveryRuntime::permissive_for_tests());
        cluster.bind_node_runtime("node_a", runtime);

        cluster
            .bind_client_route_for_principal_kind(
                "100001",
                "default",
                "1",
                "user",
                "d_pad",
                "node_a",
                Some("s_live"),
                "websocket",
            )
            .expect("initial route bind should succeed");

        let error = cluster
            .ensure_route_session_current_for_principal_kind(
                "100001", "default", "1", "user", "d_pad", None,
            )
            .expect_err("missing session id should be rejected once route has current session");
        assert_eq!(error.code, "session_id_required");
        assert_eq!(error.node_id, "node_a");
    }

    #[test]
    fn test_disconnect_fence_requires_resume_until_cleared() {
        let cluster = RealtimeClusterBridge::default();
        let runtime = Arc::new(RealtimeDeliveryRuntime::permissive_for_tests());
        cluster.bind_node_runtime("node_a", runtime);

        cluster
            .mark_client_route_disconnected_for_principal_kind(
                "100001",
                "default",
                "1",
                "user",
                "d_pad",
                Some("s_old"),
                "node_a",
            )
            .expect("disconnect fence should persist");

        let error = cluster
            .ensure_client_route_resume_not_required_for_principal_kind(
                "100001", "default", "1", "user", "d_pad",
            )
            .expect_err("disconnect fence should require an explicit resume");
        assert_eq!(error.code, "reconnect_required");
        assert_eq!(error.node_id, "node_a");
        assert!(
            cluster
                .disconnect_fence_matches_client_route_session_for_principal_kind(
                    "100001",
                    "default",
                    "1",
                    "user",
                    "d_pad",
                    Some("s_old")
                )
                .expect("session match should load")
        );
        assert!(
            !cluster
                .disconnect_fence_matches_client_route_session_for_principal_kind(
                    "100001",
                    "default",
                    "1",
                    "user",
                    "d_pad",
                    Some("s_other")
                )
                .expect("session mismatch should load")
        );

        assert!(
            cluster
                .clear_client_route_disconnect_fence_for_principal_kind(
                    "100001", "default", "1", "user", "d_pad"
                )
                .expect("disconnect fence clear should succeed")
        );
        cluster
            .ensure_client_route_resume_not_required_for_principal_kind(
                "100001", "default", "1", "user", "d_pad",
            )
            .expect("fresh resume should clear the disconnect fence");
    }

    #[test]
    fn test_disconnect_fence_survives_bridge_rebuild_with_shared_store() {
        let store = Arc::new(MemoryRealtimeDisconnectFenceStore::default());
        let cluster_a = RealtimeClusterBridge::with_disconnect_fence_store(store.clone());
        let runtime_a = Arc::new(RealtimeDeliveryRuntime::permissive_for_tests());
        cluster_a.bind_node_runtime("node_a", runtime_a);
        cluster_a
            .mark_client_route_disconnected_for_principal_kind(
                "100001",
                "default",
                "1",
                "user",
                "d_pad",
                Some("s_old"),
                "node_a",
            )
            .expect("disconnect fence should persist");

        let cluster_b = RealtimeClusterBridge::with_disconnect_fence_store(store);
        let runtime_b = Arc::new(RealtimeDeliveryRuntime::permissive_for_tests());
        cluster_b.bind_node_runtime("node_b", runtime_b);

        let error = cluster_b
            .ensure_client_route_resume_not_required_for_principal_kind(
                "100001", "default", "1", "user", "d_pad",
            )
            .expect_err("persisted disconnect fence should still require a fresh resume");
        assert_eq!(error.code, "reconnect_required");
        assert!(
            cluster_b
                .disconnect_fence_matches_client_route_session_for_principal_kind(
                    "100001",
                    "default",
                    "1",
                    "user",
                    "d_pad",
                    Some("s_old")
                )
                .expect("restored session match should load")
        );

        assert!(
            cluster_b
                .clear_client_route_disconnect_fence_for_principal_kind(
                    "100001", "default", "1", "user", "d_pad"
                )
                .expect("restored fence clear should succeed")
        );
        cluster_b
            .ensure_client_route_resume_not_required_for_principal_kind(
                "100001", "default", "1", "user", "d_pad",
            )
            .expect("clearing the restored fence should allow traffic again");
    }

    #[test]
    fn test_disconnect_fence_clear_for_current_session_does_not_delete_new_disconnect_fence() {
        let store = Arc::new(MemoryRealtimeDisconnectFenceStore::default());
        store
            .save_fence(RealtimeDisconnectFenceRecord {
                tenant_id: "100001".into(),
                organization_id: "0".into(),
                principal_kind: "user".into(),
                principal_id: "1".into(),
                device_id: "d_pad".into(),
                session_id: Some("s_new".into()),
                owner_node_id: "node_b".into(),
                disconnected_at: "2026-05-06T00:00:02.000Z".into(),
                fence_token: "fence:100001:user:3:d_pad:s_new:node_b:2026-05-06T00:00:02.000Z"
                    .into(),
            })
            .expect("new disconnect fence should persist");
        let cluster = RealtimeClusterBridge::with_disconnect_fence_store(store.clone());

        let cleared = cluster
            .clear_client_route_disconnect_fence_for_current_session(
                "100001",
                "default",
                "1",
                "user",
                "d_pad",
                Some("s_new"),
            )
            .expect("protected fence clear should succeed");

        assert!(!cleared);
        let error = cluster
            .ensure_client_route_resume_not_required_for_principal_kind(
                "100001", "default", "1", "user", "d_pad",
            )
            .expect_err("current session disconnect fence must still require a fresh resume");
        assert_eq!(error.code, "reconnect_required");
    }

    #[derive(Clone, Default)]
    struct FailingDisconnectFenceStore;

    impl RealtimeDisconnectFenceStore for FailingDisconnectFenceStore {
        fn load_fence(
            &self,
            _tenant_id: &str,
            _organization_id: &str,
            _principal_kind: &str,
            _principal_id: &str,
            _device_id: &str,
        ) -> Result<Option<RealtimeDisconnectFenceRecord>, ContractError> {
            Err(ContractError::Unavailable(
                "disconnect fence store load failed".into(),
            ))
        }

        fn save_fence(&self, _record: RealtimeDisconnectFenceRecord) -> Result<(), ContractError> {
            Err(ContractError::Unavailable(
                "disconnect fence store save failed".into(),
            ))
        }

        fn clear_fence(
            &self,
            _tenant_id: &str,
            _organization_id: &str,
            _principal_kind: &str,
            _principal_id: &str,
            _device_id: &str,
        ) -> Result<bool, ContractError> {
            Err(ContractError::Unavailable(
                "disconnect fence store clear failed".into(),
            ))
        }

        fn clear_fence_disconnected_at_or_before(
            &self,
            _tenant_id: &str,
            _organization_id: &str,
            _principal_kind: &str,
            _principal_id: &str,
            _device_id: &str,
            _cutoff_disconnected_at: &str,
        ) -> Result<bool, ContractError> {
            Err(ContractError::Unavailable(
                "disconnect fence store clear failed".into(),
            ))
        }

        fn clear_fence_if_matches(
            &self,
            _expected: &RealtimeDisconnectFenceRecord,
        ) -> Result<bool, ContractError> {
            Err(ContractError::Unavailable(
                "disconnect fence store clear failed".into(),
            ))
        }
    }

    #[test]
    fn test_disconnect_fence_store_failures_surface_as_controlled_cluster_errors() {
        let cluster = RealtimeClusterBridge::with_disconnect_fence_store(Arc::new(
            FailingDisconnectFenceStore,
        ));
        let runtime = Arc::new(RealtimeDeliveryRuntime::permissive_for_tests());
        cluster.bind_node_runtime("node_a", runtime);

        let save_error = cluster
            .mark_client_route_disconnected_for_principal_kind(
                "100001",
                "default",
                "1",
                "user",
                "d_pad",
                Some("s_old"),
                "node_a",
            )
            .expect_err("save failure should not panic");
        assert_eq!(save_error.code, "disconnect_fence_store_unavailable");

        let load_error = cluster
            .ensure_client_route_resume_not_required_for_principal_kind(
                "100001", "default", "1", "user", "d_pad",
            )
            .expect_err("load failure should surface as a controlled error");
        assert_eq!(load_error.code, "disconnect_fence_store_unavailable");

        let clear_error = cluster
            .clear_client_route_disconnect_fence_for_principal_kind(
                "100001", "default", "1", "user", "d_pad",
            )
            .expect_err("clear failure should not panic");
        assert_eq!(clear_error.code, "disconnect_fence_store_unavailable");
    }
}
