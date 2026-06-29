use std::collections::{BTreeSet, HashMap};
use std::sync::{Arc, Mutex, MutexGuard};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RouteBinding {
    pub tenant_id: String,
    pub organization_id: String,
    pub principal_id: String,
    pub principal_kind: String,
    pub device_id: String,
    pub owner_node_id: String,
    pub session_id: Option<String>,
    pub connection_kind: String,
    pub bound_at: String,
    pub route_epoch: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RouteBindingRequest {
    pub tenant_id: String,
    pub organization_id: String,
    pub principal_id: String,
    pub principal_kind: String,
    pub device_id: String,
    pub owner_node_id: String,
    pub session_id: Option<String>,
    pub connection_kind: String,
    pub bound_at: String,
}

impl RouteBindingRequest {
    pub fn new(
        tenant_id: &str,
        principal_id: &str,
        principal_kind: &str,
        device_id: &str,
        owner_node_id: &str,
    ) -> Self {
        Self {
            tenant_id: tenant_id.into(),
            organization_id: "0".into(),
            principal_id: principal_id.into(),
            principal_kind: principal_kind.into(),
            device_id: device_id.into(),
            owner_node_id: owner_node_id.into(),
            session_id: None,
            connection_kind: "unknown".into(),
            bound_at: String::new(),
        }
    }

    pub fn with_organization_id(mut self, organization_id: &str) -> Self {
        self.organization_id = normalize_route_organization_id(organization_id);
        self
    }

    pub fn with_session_id(mut self, session_id: Option<&str>) -> Self {
        self.session_id = session_id.map(str::to_owned);
        self
    }

    pub fn with_connection_kind(mut self, connection_kind: &str) -> Self {
        self.connection_kind = connection_kind.into();
        self
    }

    pub fn with_bound_at(mut self, bound_at: impl Into<String>) -> Self {
        self.bound_at = bound_at.into();
        self
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RouteNodeLifecycle {
    pub node_id: String,
    pub drain_status: String,
    pub rebalance_state: String,
    pub owned_route_count: usize,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RouteMigrationResult {
    pub source_node_id: String,
    pub target_node_id: String,
    pub migrated_route_count: usize,
    pub source_drain_status: String,
    pub source_rebalance_state: String,
    pub target_drain_status: String,
    pub target_rebalance_state: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RouteRuntimeError {
    pub code: &'static str,
    pub message: String,
    pub node_id: String,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct NodeLifecycleState {
    drain_status: String,
    rebalance_state: String,
}

#[derive(Clone, Default)]
pub struct RouteDirectory {
    routes: Arc<Mutex<RouteDirectoryState>>,
    nodes: Arc<Mutex<HashMap<String, NodeLifecycleState>>>,
}

#[derive(Default)]
struct RouteDirectoryState {
    routes_by_key: HashMap<String, RouteBinding>,
    routes_by_node: HashMap<String, BTreeSet<String>>,
}

impl RouteDirectoryState {
    fn upsert_route(&mut self, route_key: String, route: RouteBinding) {
        if let Some(previous) = self.routes_by_key.insert(route_key.clone(), route.clone()) {
            self.remove_route_key_from_node(previous.owner_node_id.as_str(), route_key.as_str());
        }
        self.routes_by_node
            .entry(route.owner_node_id)
            .or_default()
            .insert(route_key);
    }

    fn remove_route(&mut self, route_key: &str) -> Option<RouteBinding> {
        let removed = self.routes_by_key.remove(route_key)?;
        self.remove_route_key_from_node(removed.owner_node_id.as_str(), route_key);
        Some(removed)
    }

    fn route_keys_for_node(&self, node_id: &str) -> Vec<String> {
        self.routes_by_node
            .get(node_id)
            .map(|route_keys| route_keys.iter().cloned().collect())
            .unwrap_or_default()
    }

    fn remove_route_key_from_node(&mut self, node_id: &str, route_key: &str) {
        let should_remove_node = if let Some(route_keys) = self.routes_by_node.get_mut(node_id) {
            route_keys.remove(route_key);
            route_keys.is_empty()
        } else {
            false
        };
        if should_remove_node {
            self.routes_by_node.remove(node_id);
        }
    }

    fn move_route_keys_between_nodes(
        &mut self,
        source_node_id: &str,
        target_node_id: &str,
        route_keys: impl IntoIterator<Item = String>,
    ) {
        let mut moved_route_keys = Vec::new();
        for route_key in route_keys {
            self.remove_route_key_from_node(source_node_id, route_key.as_str());
            moved_route_keys.push(route_key);
        }
        if moved_route_keys.is_empty() {
            return;
        }

        self.routes_by_node
            .entry(target_node_id.into())
            .or_default()
            .extend(moved_route_keys);
    }
}

fn lock_route_mutex<'a, T>(mutex: &'a Mutex<T>, label: &'static str) -> MutexGuard<'a, T> {
    match mutex.lock() {
        Ok(guard) => guard,
        Err(poisoned) => {
            eprintln!("warning: recovering poisoned mutex in sdkwork-im-runtime-route: {label}");
            poisoned.into_inner()
        }
    }
}

impl RouteDirectory {
    pub fn register_node(&self, node_id: &str) {
        lock_route_mutex(&self.nodes, "nodes")
            .entry(node_id.into())
            .or_insert_with(|| NodeLifecycleState {
                drain_status: "active".into(),
                rebalance_state: "stable".into(),
            });
    }

    pub fn observe_external_binding(&self, binding: RouteBinding) {
        let key = route_key(
            binding.tenant_id.as_str(),
            binding.organization_id.as_str(),
            binding.principal_id.as_str(),
            binding.principal_kind.as_str(),
            binding.device_id.as_str(),
        );
        let mut routes = lock_route_mutex(&self.routes, "routes");
        if let Some(current) = routes.routes_by_key.get(&key)
            && current.route_epoch > binding.route_epoch
        {
            return;
        }
        routes.upsert_route(key, binding);
    }

    pub fn bind(&self, request: RouteBindingRequest) -> Result<RouteBinding, RouteRuntimeError> {
        let lifecycle = self.node_state(request.owner_node_id.as_str())?;
        if lifecycle.drain_status != "active" {
            return Err(RouteRuntimeError {
                code: "node_draining",
                message: format!(
                    "node {} is not accepting new route binds while {}",
                    request.owner_node_id, lifecycle.drain_status
                ),
                node_id: request.owner_node_id,
            });
        }

        let key = route_key(
            request.tenant_id.as_str(),
            request.organization_id.as_str(),
            request.principal_id.as_str(),
            request.principal_kind.as_str(),
            request.device_id.as_str(),
        );
        let mut routes = lock_route_mutex(&self.routes, "routes");
        let previous_owner = routes
            .routes_by_key
            .get(&key)
            .map(|route| route.owner_node_id.clone());
        let next_epoch = routes
            .routes_by_key
            .get(&key)
            .map(|route| {
                if route.owner_node_id == request.owner_node_id
                    && route.session_id == request.session_id
                    && route.connection_kind == request.connection_kind
                {
                    route.route_epoch
                } else {
                    route.route_epoch + 1
                }
            })
            .unwrap_or(1);
        let binding = RouteBinding {
            tenant_id: request.tenant_id,
            organization_id: normalize_route_organization_id(&request.organization_id),
            principal_id: request.principal_id,
            principal_kind: request.principal_kind,
            device_id: request.device_id,
            owner_node_id: request.owner_node_id,
            session_id: request.session_id,
            connection_kind: request.connection_kind,
            bound_at: request.bound_at,
            route_epoch: next_epoch,
        };
        routes.upsert_route(key, binding.clone());
        drop(routes);

        if let Some(previous_owner) = previous_owner
            && previous_owner != binding.owner_node_id
        {
            self.reconcile_departure(previous_owner.as_str())?;
        }
        Ok(binding)
    }

    pub fn mark_node_draining(
        &self,
        node_id: &str,
    ) -> Result<RouteNodeLifecycle, RouteRuntimeError> {
        let owned_route_count = self.owned_route_count(node_id);
        let (drain_status, rebalance_state) = if owned_route_count == 0 {
            ("drained", "stable")
        } else {
            ("draining", "moving_routes")
        };
        self.set_node_state(node_id, drain_status, rebalance_state)?;
        self.node_lifecycle_view(node_id)
    }

    pub fn activate_node(&self, node_id: &str) -> Result<RouteNodeLifecycle, RouteRuntimeError> {
        self.set_node_state(node_id, "active", "stable")?;
        self.node_lifecycle_view(node_id)
    }

    pub fn migrate_routes(
        &self,
        source_node_id: &str,
        target_node_id: &str,
    ) -> Result<RouteMigrationResult, RouteRuntimeError> {
        self.migrate_routes_at(source_node_id, target_node_id, "")
    }

    pub fn migrate_routes_at(
        &self,
        source_node_id: &str,
        target_node_id: &str,
        migrated_at: &str,
    ) -> Result<RouteMigrationResult, RouteRuntimeError> {
        // Phase 1: Validation
        if source_node_id == target_node_id {
            return Err(RouteRuntimeError {
                code: "same_node_migration",
                message: "source and target nodes must be different".into(),
                node_id: source_node_id.into(),
            });
        }

        let source = self.node_state(source_node_id)?;
        if source.drain_status != "draining" {
            return Err(RouteRuntimeError {
                code: "node_not_draining",
                message: format!("source node must be draining before migration: {source_node_id}"),
                node_id: source_node_id.into(),
            });
        }
        let target = self.node_state(target_node_id)?;
        if target.drain_status != "active" || target.rebalance_state != "stable" {
            return Err(RouteRuntimeError {
                code: "target_node_unavailable",
                message: format!("target node is not ready to accept routes: {target_node_id}"),
                node_id: target_node_id.into(),
            });
        }

        // Phase 2: Snapshot for rollback
        let routes = lock_route_mutex(&self.routes, "routes");
        let route_keys = routes.route_keys_for_node(source_node_id);
        let original_routes: Vec<(String, RouteBinding)> = route_keys
            .iter()
            .filter_map(|key| {
                routes.routes_by_key.get(key).map(|route| {
                    (key.clone(), route.clone())
                })
            })
            .collect();
        drop(routes);

        // Phase 3: Perform migration with atomic batch updates
        let mut migrated = 0;
        let mut routes = lock_route_mutex(&self.routes, "routes");
        let mut migrated_route_keys = Vec::new();
        let mut failed_routes = Vec::new();

        for route_key in &route_keys {
            if let Some(route) = routes.routes_by_key.get_mut(route_key.as_str()) {
                if route.owner_node_id == source_node_id {
                    // Validate route is still owned by source (concurrent modification check)
                    route.owner_node_id = target_node_id.into();
                    route.route_epoch += 1;
                    if !migrated_at.is_empty() {
                        route.bound_at = migrated_at.into();
                    }
                    migrated += 1;
                    migrated_route_keys.push(route_key.clone());
                } else {
                    // Route was concurrently modified, skip it
                    failed_routes.push(route_key.clone());
                }
            }
        }

        // Only update node indices if all migrations succeeded
        if failed_routes.is_empty() {
            routes.move_route_keys_between_nodes(source_node_id, target_node_id, migrated_route_keys);
        } else {
            // Rollback partial migration
            tracing::warn!(
                target: "sdkwork.im.route",
                source_node = %source_node_id,
                target_node = %target_node_id,
                failed_count = failed_routes.len(),
                "rolling back migration due to concurrent modifications"
            );

            for (key, original_route) in original_routes {
                routes.upsert_route(key, original_route);
            }

            return Err(RouteRuntimeError {
                code: "migration_concurrent_modification",
                message: format!(
                    "migration rolled back due to {} concurrent route modifications",
                    failed_routes.len()
                ),
                node_id: source_node_id.into(),
            });
        }

        drop(routes);

        // Phase 4: Update node states atomically
        self.set_node_state(target_node_id, "active", "stable")?;
        let source_status = if self.owned_route_count(source_node_id) == 0 {
            self.set_node_state(source_node_id, "drained", "stable")?;
            self.node_state(source_node_id)?
        } else {
            self.set_node_state(source_node_id, "draining", "moving_routes")?;
            self.node_state(source_node_id)?
        };
        let target_status = self.node_state(target_node_id)?;

        tracing::info!(
            target: "sdkwork.im.route",
            source_node = %source_node_id,
            target_node = %target_node_id,
            migrated_count = migrated,
            "route migration completed successfully"
        );

        Ok(RouteMigrationResult {
            source_node_id: source_node_id.into(),
            target_node_id: target_node_id.into(),
            migrated_route_count: migrated,
            source_drain_status: source_status.drain_status,
            source_rebalance_state: source_status.rebalance_state,
            target_drain_status: target_status.drain_status,
            target_rebalance_state: target_status.rebalance_state,
        })
    }

    pub fn lookup(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_id: &str,
        principal_kind: &str,
        device_id: &str,
    ) -> Option<RouteBinding> {
        lock_route_mutex(&self.routes, "routes")
            .routes_by_key
            .get(
                route_key(
                    tenant_id,
                    organization_id,
                    principal_id,
                    principal_kind,
                    device_id,
                )
                .as_str(),
            )
            .cloned()
    }

    pub fn release(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_id: &str,
        principal_kind: &str,
        device_id: &str,
        owner_node_id: &str,
    ) -> Option<RouteBinding> {
        let key = route_key(
            tenant_id,
            organization_id,
            principal_id,
            principal_kind,
            device_id,
        );
        let removed = {
            let mut routes = lock_route_mutex(&self.routes, "routes");
            match routes.routes_by_key.get(&key) {
                Some(route) if route.owner_node_id == owner_node_id => routes.remove_route(&key),
                _ => None,
            }
        };
        if let Some(route) = removed.as_ref() {
            let _ = self.reconcile_departure(route.owner_node_id.as_str());
        }
        removed
    }

    pub fn restore_if_current(
        &self,
        expected_current: &RouteBinding,
        mut restore_to: RouteBinding,
    ) -> Option<RouteBinding> {
        if !same_route_identity(expected_current, &restore_to) {
            return None;
        }
        let key = route_key(
            expected_current.tenant_id.as_str(),
            expected_current.organization_id.as_str(),
            expected_current.principal_id.as_str(),
            expected_current.principal_kind.as_str(),
            expected_current.device_id.as_str(),
        );
        let mut routes = lock_route_mutex(&self.routes, "routes");
        match routes.routes_by_key.get(&key) {
            Some(current) if current == expected_current => {
                restore_to.route_epoch = expected_current.route_epoch + 1;
                routes.upsert_route(key, restore_to.clone());
                let expected_owner = expected_current.owner_node_id.clone();
                let restored_owner = restore_to.owner_node_id.clone();
                drop(routes);
                let _ = self.reconcile_ownership(restored_owner.as_str());
                if expected_owner != restored_owner {
                    let _ = self.reconcile_departure(expected_owner.as_str());
                }
                Some(restore_to)
            }
            _ => None,
        }
    }

    pub fn routes_for_node(&self, node_id: &str) -> Vec<RouteBinding> {
        let routes = lock_route_mutex(&self.routes, "routes");
        let mut items = routes
            .routes_by_node
            .get(node_id)
            .into_iter()
            .flat_map(|route_keys| route_keys.iter())
            .filter_map(|route_key| routes.routes_by_key.get(route_key.as_str()).cloned())
            .collect::<Vec<_>>();
        items.sort_by(|left, right| {
            left.tenant_id
                .cmp(&right.tenant_id)
                .then_with(|| left.principal_kind.cmp(&right.principal_kind))
                .then_with(|| left.principal_id.cmp(&right.principal_id))
                .then_with(|| left.device_id.cmp(&right.device_id))
        });
        items
    }

    pub fn node_lifecycle(&self, node_id: &str) -> Option<RouteNodeLifecycle> {
        self.node_lifecycle_view(node_id).ok()
    }

    fn owned_route_count(&self, node_id: &str) -> usize {
        lock_route_mutex(&self.routes, "routes")
            .routes_by_node
            .get(node_id)
            .map(BTreeSet::len)
            .unwrap_or_default()
    }

    fn node_lifecycle_view(&self, node_id: &str) -> Result<RouteNodeLifecycle, RouteRuntimeError> {
        let state = self.node_state(node_id)?;
        Ok(RouteNodeLifecycle {
            node_id: node_id.into(),
            drain_status: state.drain_status,
            rebalance_state: state.rebalance_state,
            owned_route_count: self.owned_route_count(node_id),
        })
    }

    fn reconcile_departure(&self, node_id: &str) -> Result<(), RouteRuntimeError> {
        let state = match self.node_state(node_id) {
            Ok(state) => state,
            Err(_) => return Ok(()),
        };
        if state.drain_status != "draining" {
            return Ok(());
        }

        if self.owned_route_count(node_id) == 0 {
            self.set_node_state(node_id, "drained", "stable")?;
        } else {
            self.set_node_state(node_id, "draining", "moving_routes")?;
        }
        Ok(())
    }

    fn reconcile_ownership(&self, node_id: &str) -> Result<(), RouteRuntimeError> {
        let state = match self.node_state(node_id) {
            Ok(state) => state,
            Err(_) => return Ok(()),
        };
        if state.drain_status != "drained" || self.owned_route_count(node_id) == 0 {
            return Ok(());
        }

        self.set_node_state(node_id, "draining", "moving_routes")?;
        Ok(())
    }

    fn node_state(&self, node_id: &str) -> Result<NodeLifecycleState, RouteRuntimeError> {
        lock_route_mutex(&self.nodes, "nodes")
            .get(node_id)
            .cloned()
            .ok_or_else(|| RouteRuntimeError {
                code: "node_not_found",
                message: format!("node not found: {node_id}"),
                node_id: node_id.into(),
            })
    }

    fn set_node_state(
        &self,
        node_id: &str,
        drain_status: &str,
        rebalance_state: &str,
    ) -> Result<(), RouteRuntimeError> {
        let mut nodes = lock_route_mutex(&self.nodes, "nodes");
        let state = nodes.get_mut(node_id).ok_or_else(|| RouteRuntimeError {
            code: "node_not_found",
            message: format!("node not found: {node_id}"),
            node_id: node_id.into(),
        })?;
        state.drain_status = drain_status.into();
        state.rebalance_state = rebalance_state.into();
        Ok(())
    }
}

fn same_route_identity(left: &RouteBinding, right: &RouteBinding) -> bool {
    left.tenant_id == right.tenant_id
        && left.organization_id == right.organization_id
        && left.principal_kind == right.principal_kind
        && left.principal_id == right.principal_id
        && left.device_id == right.device_id
}

fn route_key(
    tenant_id: &str,
    organization_id: &str,
    principal_id: &str,
    principal_kind: &str,
    device_id: &str,
) -> String {
    encode_route_key_segments([
        tenant_id,
        normalize_route_organization_id(organization_id).as_str(),
        principal_kind,
        principal_id,
        device_id,
    ])
}

pub fn normalize_route_organization_id(organization_id: &str) -> String {
    let trimmed = organization_id.trim();
    if trimmed.is_empty() || trimmed == "0" {
        "default".to_owned()
    } else {
        trimmed.to_owned()
    }
}

pub fn encode_route_key_segments<'a>(segments: impl IntoIterator<Item = &'a str>) -> String {
    let mut encoded = String::new();
    for segment in segments {
        encoded.push_str(segment.len().to_string().as_str());
        encoded.push('#');
        encoded.push_str(segment);
    }
    encoded
}

mod route_store;
pub use route_store::{memory_route_store, RouteStore};

#[cfg(test)]
mod tests {
    use super::*;
    use std::panic::{self, AssertUnwindSafe};

    fn poison_mutex<T>(mutex: &Mutex<T>) {
        let _ = panic::catch_unwind(AssertUnwindSafe(|| {
            let _guard = mutex.lock().expect("test poison lock should succeed");
            panic!("intentional poison for regression coverage");
        }));
    }

    #[test]
    fn test_register_node_recovers_from_poisoned_nodes_lock() {
        let directory = RouteDirectory::default();
        poison_mutex(&directory.nodes);

        let result = panic::catch_unwind(AssertUnwindSafe(|| {
            directory.register_node("node_a");
        }));
        assert!(
            result.is_ok(),
            "register_node should not panic when node lifecycle store lock is poisoned"
        );
        assert!(directory.node_lifecycle("node_a").is_some());
    }

    #[test]
    fn test_bind_recovers_from_poisoned_routes_lock() {
        let directory = RouteDirectory::default();
        directory.register_node("node_a");
        poison_mutex(&directory.routes);

        let result = panic::catch_unwind(AssertUnwindSafe(|| {
            directory.bind(
                RouteBindingRequest::new("100001", "1", "user", "d_pad", "node_a")
                    .with_session_id(Some("s_demo"))
                    .with_connection_kind("websocket")
                    .with_bound_at("2026-04-12T00:00:00.000Z"),
            )
        }));
        assert!(
            result.is_ok(),
            "bind should not panic when route binding store lock is poisoned"
        );
        let bind_result = result.expect("panic status should be captured");
        assert!(
            bind_result.is_ok(),
            "bind should recover from poisoned route binding store lock"
        );
    }

    #[test]
    fn test_bind_increments_route_epoch_when_same_node_rebinds_new_session() {
        let directory = RouteDirectory::default();
        directory.register_node("node_a");

        let first_bind = directory
            .bind(
                RouteBindingRequest::new("100001", "1", "user", "d_pad", "node_a")
                    .with_session_id(Some("s_old"))
                    .with_connection_kind("websocket")
                    .with_bound_at("2026-04-15T00:00:00.000Z"),
            )
            .expect("initial route bind should succeed");
        assert_eq!(first_bind.route_epoch, 1);

        let rebound = directory
            .bind(
                RouteBindingRequest::new("100001", "1", "user", "d_pad", "node_a")
                    .with_session_id(Some("s_new"))
                    .with_connection_kind("http")
                    .with_bound_at("2026-04-15T00:01:00.000Z"),
            )
            .expect("same-node takeover bind should succeed");

        assert_eq!(rebound.owner_node_id, "node_a");
        assert_eq!(rebound.session_id.as_deref(), Some("s_new"));
        assert_eq!(rebound.connection_kind, "http");
        assert_eq!(
            rebound.route_epoch, 2,
            "route epoch must advance when same-node ownership metadata changes to a newer session"
        );
    }

    #[test]
    fn test_restore_if_current_restores_route_only_when_expected_binding_matches() {
        let directory = RouteDirectory::default();
        directory.register_node("node_a");

        let original = directory
            .bind(
                RouteBindingRequest::new("100001", "1", "user", "d_pad", "node_a")
                    .with_session_id(Some("s_demo"))
                    .with_connection_kind("websocket")
                    .with_bound_at("2026-04-15T00:00:00.000Z"),
            )
            .expect("original route bind should succeed");
        let temporary = directory
            .bind(
                RouteBindingRequest::new("100001", "1", "user", "d_pad", "node_a")
                    .with_session_id(Some("s_demo"))
                    .with_connection_kind("http")
                    .with_bound_at("2026-04-15T00:01:00.000Z"),
            )
            .expect("temporary route bind should succeed");

        let restored_binding = directory
            .restore_if_current(&temporary, original.clone())
            .expect("restore should succeed while current route is still the temporary binding");
        let restored = directory
            .lookup("100001", "default", "1", "user", "d_pad")
            .expect("restored route should remain addressable");
        assert_eq!(restored, restored_binding);
        assert_eq!(restored.owner_node_id, original.owner_node_id);
        assert_eq!(restored.session_id, original.session_id);
        assert_eq!(restored.connection_kind, original.connection_kind);
        assert_eq!(
            restored.route_epoch,
            temporary.route_epoch + 1,
            "route epoch must remain monotonic when a temporary binding is restored"
        );

        let newer = directory
            .bind(
                RouteBindingRequest::new("100001", "1", "user", "d_pad", "node_a")
                    .with_session_id(Some("s_new"))
                    .with_connection_kind("http")
                    .with_bound_at("2026-04-15T00:02:00.000Z"),
            )
            .expect("newer route bind should succeed");

        assert!(
            directory.restore_if_current(&temporary, original).is_none(),
            "stale restore must not overwrite a newer route binding"
        );
        let current = directory
            .lookup("100001", "default", "1", "user", "d_pad")
            .expect("newer route should remain addressable");
        assert_eq!(current, newer);
    }

    #[test]
    fn test_restore_if_current_rejects_identity_mismatched_restore_target() {
        let directory = RouteDirectory::default();
        directory.register_node("node_a");

        let original = directory
            .bind(
                RouteBindingRequest::new("100001", "1", "user", "d_pad", "node_a")
                    .with_session_id(Some("s_demo"))
                    .with_connection_kind("websocket")
                    .with_bound_at("2026-04-15T00:00:00.000Z"),
            )
            .expect("original route bind should succeed");
        let temporary = directory
            .bind(
                RouteBindingRequest::new("100001", "1", "user", "d_pad", "node_a")
                    .with_session_id(Some("s_demo"))
                    .with_connection_kind("http")
                    .with_bound_at("2026-04-15T00:01:00.000Z"),
            )
            .expect("temporary route bind should succeed");
        let mut wrong_identity_restore = original.clone();
        wrong_identity_restore.device_id = "d_other".into();

        assert!(
            directory
                .restore_if_current(&temporary, wrong_identity_restore)
                .is_none(),
            "restore must reject a target binding whose route identity differs from the expected current route"
        );
        assert_eq!(
            directory
                .lookup("100001", "default", "1", "user", "d_pad")
                .expect("temporary route should remain addressable"),
            temporary,
            "identity-mismatched restore must not corrupt the current route slot"
        );
    }

    #[test]
    fn test_restore_if_current_reconciles_drained_restore_owner_back_to_draining() {
        let directory = RouteDirectory::default();
        directory.register_node("node_a");
        directory.register_node("node_b");

        let original = directory
            .bind(
                RouteBindingRequest::new("100001", "1", "user", "d_pad", "node_a")
                    .with_session_id(Some("s_demo"))
                    .with_connection_kind("websocket"),
            )
            .expect("original route bind should succeed");
        directory
            .mark_node_draining("node_a")
            .expect("source owner should enter draining");
        let temporary = directory
            .bind(
                RouteBindingRequest::new("100001", "1", "user", "d_pad", "node_b")
                    .with_session_id(Some("s_demo"))
                    .with_connection_kind("http"),
            )
            .expect("temporary bind should move the route to an active node");

        let source_after_temporary_bind = directory
            .node_lifecycle("node_a")
            .expect("source lifecycle should remain visible");
        assert_eq!(source_after_temporary_bind.drain_status, "drained");
        assert_eq!(source_after_temporary_bind.owned_route_count, 0);

        directory
            .restore_if_current(&temporary, original)
            .expect("restore should succeed while temporary binding is current");

        let restored_source = directory
            .node_lifecycle("node_a")
            .expect("restored source lifecycle should remain visible");
        assert_eq!(restored_source.drain_status, "draining");
        assert_eq!(restored_source.rebalance_state, "moving_routes");
        assert_eq!(restored_source.owned_route_count, 1);
    }

    #[test]
    fn test_route_directory_uses_owner_index_for_node_route_queries() {
        let source = include_str!("lib.rs").replace("\r\n", "\n");

        assert!(
            source.contains("routes_by_node: HashMap<String, BTreeSet<String>>"),
            "RouteDirectory should keep an owner-node -> route-key index"
        );
        assert!(
            !source
                .contains(".values()\n            .filter(|route| route.owner_node_id == node_id)"),
            "RouteDirectory must not full-scan all routes for per-node route queries"
        );
        assert!(
            !source.contains(".routes_by_key\n            .iter()\n            .filter(|(_, route)| route.owner_node_id == node_id)"),
            "RouteDirectory owner index maintenance should stay incremental instead of rebuilding by scanning all routes"
        );
    }

    #[test]
    fn test_route_directory_principal_identity_is_strictly_typed() {
        let source = include_str!("lib.rs").replace("\r\n", "\n");
        let production_source = source
            .split("#[cfg(test)]")
            .next()
            .expect("production source should precede cfg(test) module");

        for forbidden_symbol in [
            "pub principal_kind: Option<String>",
            "principal_kind: Option<String>",
            "principal_kind: Option<&str>",
            "pub fn with_principal_kind(",
            "None => format!(\"{tenant_id}:{principal_id}:{device_id}\")",
            "format!(\"{tenant_id}:{principal_kind}:{principal_id}:{device_id}\")",
        ] {
            assert!(
                !production_source.contains(forbidden_symbol),
                "RouteDirectory route identity must require principal_kind in every cache key and API: {forbidden_symbol}"
            );
        }

        assert!(
            production_source.contains("normalize_route_organization_id(organization_id)")
                && production_source.contains("encode_route_key_segments"),
            "RouteDirectory route key must use segment-safe length-prefixed encoding with organization scope"
        );
        assert!(
            production_source.contains(
                "pub fn lookup(\n        &self,\n        tenant_id: &str,\n        organization_id: &str,\n        principal_id: &str,\n        principal_kind: &str,"
            ),
            "RouteDirectory lookup API must require principal_kind"
        );
        assert!(
            production_source.contains(
                "pub fn release(\n        &self,\n        tenant_id: &str,\n        organization_id: &str,\n        principal_id: &str,\n        principal_kind: &str,"
            ),
            "RouteDirectory release API must require principal_kind"
        );
    }

    #[test]
    fn test_route_directory_key_encoding_is_segment_safe() {
        let directory = RouteDirectory::default();
        directory.register_node("node_a");
        directory.register_node("node_b");

        directory
            .bind(RouteBindingRequest::new(
                "tenant",
                "principal:segment",
                "user",
                "device",
                "node_a",
            ))
            .expect("first route should bind");
        directory
            .bind(RouteBindingRequest::new(
                "tenant",
                "principal",
                "user",
                "segment:device",
                "node_b",
            ))
            .expect("second route should bind without colliding with delimiter-like values");

        assert_eq!(
            directory
                .lookup("tenant", "default", "principal:segment", "user", "device")
                .expect("first route should remain addressable")
                .owner_node_id,
            "node_a"
        );
        assert_eq!(
            directory
                .lookup("tenant", "default", "principal", "user", "segment:device")
                .expect("second route should remain addressable")
                .owner_node_id,
            "node_b"
        );
        assert_eq!(directory.routes_for_node("node_a").len(), 1);
        assert_eq!(directory.routes_for_node("node_b").len(), 1);
    }

    #[test]
    fn test_node_lifecycle_recovers_from_poisoned_nodes_lock() {
        let directory = RouteDirectory::default();
        directory.register_node("node_a");
        poison_mutex(&directory.nodes);

        let result = panic::catch_unwind(AssertUnwindSafe(|| directory.node_lifecycle("node_a")));
        assert!(
            result.is_ok(),
            "node_lifecycle should not panic when node lifecycle store lock is poisoned"
        );
        let lifecycle = result.expect("panic status should be captured");
        assert!(lifecycle.is_some());
    }
}
