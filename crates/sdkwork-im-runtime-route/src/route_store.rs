use std::sync::Arc;

use crate::{
    RouteBinding, RouteBindingRequest, RouteDirectory, RouteMigrationResult, RouteNodeLifecycle,
    RouteRuntimeError,
};

/// Hot route ownership store for the Route Plane.
///
/// Phase 1 keeps the in-memory `RouteDirectory` as the authoritative local cache while
/// optional Redis and PostgreSQL implementations mirror bindings for cross-node recovery.
pub trait RouteStore: Send + Sync {
    fn register_node(&self, node_id: &str);
    fn bind(&self, request: RouteBindingRequest) -> Result<RouteBinding, RouteRuntimeError>;
    fn mark_node_draining(&self, node_id: &str) -> Result<RouteNodeLifecycle, RouteRuntimeError>;
    fn activate_node(&self, node_id: &str) -> Result<RouteNodeLifecycle, RouteRuntimeError>;
    fn migrate_routes(
        &self,
        source_node_id: &str,
        target_node_id: &str,
    ) -> Result<RouteMigrationResult, RouteRuntimeError>;
    fn migrate_routes_at(
        &self,
        source_node_id: &str,
        target_node_id: &str,
        migrated_at: &str,
    ) -> Result<RouteMigrationResult, RouteRuntimeError>;
    fn lookup(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_id: &str,
        principal_kind: &str,
        device_id: &str,
    ) -> Option<RouteBinding>;
    fn release(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_id: &str,
        principal_kind: &str,
        device_id: &str,
        owner_node_id: &str,
    ) -> Option<RouteBinding>;
    fn restore_if_current(
        &self,
        expected_current: &RouteBinding,
        restore_to: RouteBinding,
    ) -> Option<RouteBinding>;
    fn routes_for_node(&self, node_id: &str) -> Vec<RouteBinding>;
    fn node_lifecycle(&self, node_id: &str) -> Option<RouteNodeLifecycle>;
}

impl RouteStore for RouteDirectory {
    fn register_node(&self, node_id: &str) {
        RouteDirectory::register_node(self, node_id);
    }

    fn bind(&self, request: RouteBindingRequest) -> Result<RouteBinding, RouteRuntimeError> {
        RouteDirectory::bind(self, request)
    }

    fn mark_node_draining(&self, node_id: &str) -> Result<RouteNodeLifecycle, RouteRuntimeError> {
        RouteDirectory::mark_node_draining(self, node_id)
    }

    fn activate_node(&self, node_id: &str) -> Result<RouteNodeLifecycle, RouteRuntimeError> {
        RouteDirectory::activate_node(self, node_id)
    }

    fn migrate_routes(
        &self,
        source_node_id: &str,
        target_node_id: &str,
    ) -> Result<RouteMigrationResult, RouteRuntimeError> {
        RouteDirectory::migrate_routes(self, source_node_id, target_node_id)
    }

    fn migrate_routes_at(
        &self,
        source_node_id: &str,
        target_node_id: &str,
        migrated_at: &str,
    ) -> Result<RouteMigrationResult, RouteRuntimeError> {
        RouteDirectory::migrate_routes_at(self, source_node_id, target_node_id, migrated_at)
    }

    fn lookup(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_id: &str,
        principal_kind: &str,
        device_id: &str,
    ) -> Option<RouteBinding> {
        RouteDirectory::lookup(
            self,
            tenant_id,
            organization_id,
            principal_id,
            principal_kind,
            device_id,
        )
    }

    fn release(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_id: &str,
        principal_kind: &str,
        device_id: &str,
        owner_node_id: &str,
    ) -> Option<RouteBinding> {
        RouteDirectory::release(
            self,
            tenant_id,
            organization_id,
            principal_id,
            principal_kind,
            device_id,
            owner_node_id,
        )
    }

    fn restore_if_current(
        &self,
        expected_current: &RouteBinding,
        restore_to: RouteBinding,
    ) -> Option<RouteBinding> {
        RouteDirectory::restore_if_current(self, expected_current, restore_to)
    }

    fn routes_for_node(&self, node_id: &str) -> Vec<RouteBinding> {
        RouteDirectory::routes_for_node(self, node_id)
    }

    fn node_lifecycle(&self, node_id: &str) -> Option<RouteNodeLifecycle> {
        RouteDirectory::node_lifecycle(self, node_id)
    }
}

pub fn memory_route_store() -> Arc<dyn RouteStore> {
    Arc::new(RouteDirectory::default())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_route_store_implements_route_store_trait() {
        let store = memory_route_store();
        store.register_node("node_a");
        let binding = store
            .bind(
                RouteBindingRequest::new("100001", "1", "user", "d_pad", "node_a")
                    .with_session_id(Some("s_demo"))
                    .with_connection_kind("websocket")
                    .with_bound_at("2026-06-22T00:00:00.000Z"),
            )
            .expect("bind should succeed");
        assert_eq!(binding.owner_node_id, "node_a");
        assert_eq!(
            store
                .lookup("100001", "default", "1", "user", "d_pad")
                .expect("lookup should succeed")
                .route_epoch,
            1
        );
    }
}
