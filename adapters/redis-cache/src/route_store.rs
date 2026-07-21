//! Redis-backed hot route store for cross-node route recovery.

use std::sync::Arc;

use sdkwork_im_runtime_route::{
    ROUTE_BINDING_PAGE_MAX_SIZE, RouteBinding, RouteBindingPage, RouteBindingRequest,
    RouteDirectory, RouteMigrationResult, RouteNodeLifecycle, RouteRuntimeError, RouteStore,
    encode_route_key_segments, normalize_route_organization_id,
};

use crate::redis_blocking::{RedisBlockingTimeouts, run_bounded_redis_command};

const ROUTE_STORE_KEY_PREFIX: &str = "im:route:v1:binding:";
const ROUTE_STORE_NODE_INDEX_PREFIX: &str = "im:route:v1:node:";

#[derive(Clone)]
pub struct RedisBackedRouteStore {
    memory: RouteDirectory,
    client: redis::Client,
    timeouts: RedisBlockingTimeouts,
}

impl RedisBackedRouteStore {
    pub fn new(redis_url: impl AsRef<str>) -> Result<Self, String> {
        let client = redis::Client::open(redis_url.as_ref())
            .map_err(|error| format!("invalid redis route store url: {error}"))?;
        Ok(Self {
            memory: RouteDirectory::default(),
            client,
            timeouts: RedisBlockingTimeouts::from_env(),
        })
    }

    pub fn into_arc(self) -> Arc<dyn RouteStore> {
        Arc::new(self)
    }

    fn binding_key(route_key: &str) -> String {
        format!("{ROUTE_STORE_KEY_PREFIX}{route_key}")
    }

    fn node_index_key(node_id: &str) -> String {
        format!("{ROUTE_STORE_NODE_INDEX_PREFIX}{node_id}:keys")
    }

    fn route_key_for_binding(binding: &RouteBinding) -> String {
        encode_route_key_segments([
            binding.tenant_id.as_str(),
            normalize_route_organization_id(binding.organization_id.as_str()).as_str(),
            binding.principal_kind.as_str(),
            binding.principal_id.as_str(),
            binding.device_id.as_str(),
        ])
    }

    fn persist_binding(&self, binding: &RouteBinding) -> Result<(), RouteRuntimeError> {
        let route_key = Self::route_key_for_binding(binding);
        let payload = serde_json::to_string(binding).map_err(|error| RouteRuntimeError {
            code: "route_store_encode_failed",
            message: format!("encode route binding failed: {error}"),
            node_id: binding.owner_node_id.clone(),
        })?;
        let binding_key = Self::binding_key(route_key.as_str());
        let node_index_key = Self::node_index_key(binding.owner_node_id.as_str());
        run_bounded_redis_command(
            &self.client,
            self.timeouts,
            "route_store_write",
            move |mut connection| async move {
                redis::cmd("SET")
                    .arg(binding_key)
                    .arg(payload)
                    .query_async::<()>(&mut connection)
                    .await?;
                redis::cmd("SADD")
                    .arg(node_index_key)
                    .arg(route_key.as_str())
                    .query_async::<i32>(&mut connection)
                    .await?;
                Ok(())
            },
        )
        .map_err(|error| RouteRuntimeError {
            code: "route_store_write_failed",
            message: format!("persist route binding failed: {error:?}"),
            node_id: binding.owner_node_id.clone(),
        })
    }

    fn remove_binding(&self, binding: &RouteBinding) -> Result<(), RouteRuntimeError> {
        let route_key = Self::route_key_for_binding(binding);
        let binding_key = Self::binding_key(route_key.as_str());
        let node_index_key = Self::node_index_key(binding.owner_node_id.as_str());
        run_bounded_redis_command(
            &self.client,
            self.timeouts,
            "route_store_delete",
            move |mut connection| async move {
                redis::cmd("DEL")
                    .arg(binding_key)
                    .query_async::<i32>(&mut connection)
                    .await?;
                redis::cmd("SREM")
                    .arg(node_index_key)
                    .arg(route_key.as_str())
                    .query_async::<i32>(&mut connection)
                    .await?;
                Ok(())
            },
        )
        .map_err(|error| RouteRuntimeError {
            code: "route_store_delete_failed",
            message: format!("delete route binding failed: {error:?}"),
            node_id: binding.owner_node_id.clone(),
        })
    }

    fn hydrate_from_redis(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_id: &str,
        principal_kind: &str,
        device_id: &str,
    ) -> Option<RouteBinding> {
        let route_key = encode_route_key_segments([
            tenant_id,
            normalize_route_organization_id(organization_id).as_str(),
            principal_kind,
            principal_id,
            device_id,
        ]);
        let binding_key = Self::binding_key(route_key.as_str());
        let payload: String = run_bounded_redis_command(
            &self.client,
            self.timeouts,
            "route_store_lookup",
            move |mut connection| async move {
                redis::cmd("GET")
                    .arg(binding_key)
                    .query_async(&mut connection)
                    .await
            },
        )
        .ok()?;
        let binding: RouteBinding = serde_json::from_str(payload.as_str()).ok()?;
        self.memory.register_node(binding.owner_node_id.as_str());
        self.memory.observe_external_binding(binding.clone());
        Some(binding)
    }

    /// Warm the in-memory and Redis hot tier after loading from a durable fallback store.
    pub fn warm_binding(&self, binding: &RouteBinding) -> Result<(), RouteRuntimeError> {
        self.memory.register_node(binding.owner_node_id.as_str());
        self.memory.observe_external_binding(binding.clone());
        self.persist_binding(binding)
    }

    fn persist_node_bindings(&self, node_id: &str) -> Result<(), RouteRuntimeError> {
        let mut cursor = None;
        loop {
            let page = self.memory.routes_for_node_page(
                node_id,
                cursor.as_deref(),
                ROUTE_BINDING_PAGE_MAX_SIZE,
            );
            for route in page.items {
                self.persist_binding(&route)?;
            }
            let Some(next_cursor) = page.next_cursor else {
                return Ok(());
            };
            cursor = Some(next_cursor);
        }
    }
}

impl RouteStore for RedisBackedRouteStore {
    fn register_node(&self, node_id: &str) {
        self.memory.register_node(node_id);
    }

    fn bind(&self, request: RouteBindingRequest) -> Result<RouteBinding, RouteRuntimeError> {
        let binding = self.memory.bind(request)?;
        if let Err(error) = self.persist_binding(&binding) {
            let _ = self.memory.release(
                binding.tenant_id.as_str(),
                binding.organization_id.as_str(),
                binding.principal_id.as_str(),
                binding.principal_kind.as_str(),
                binding.device_id.as_str(),
                binding.owner_node_id.as_str(),
            );
            return Err(error);
        }
        Ok(binding)
    }

    fn mark_node_draining(&self, node_id: &str) -> Result<RouteNodeLifecycle, RouteRuntimeError> {
        self.memory.mark_node_draining(node_id)
    }

    fn activate_node(&self, node_id: &str) -> Result<RouteNodeLifecycle, RouteRuntimeError> {
        self.memory.activate_node(node_id)
    }

    fn migrate_routes(
        &self,
        source_node_id: &str,
        target_node_id: &str,
    ) -> Result<RouteMigrationResult, RouteRuntimeError> {
        self.migrate_routes_at(source_node_id, target_node_id, "")
    }

    fn migrate_routes_at(
        &self,
        source_node_id: &str,
        target_node_id: &str,
        migrated_at: &str,
    ) -> Result<RouteMigrationResult, RouteRuntimeError> {
        let migration =
            self.memory
                .migrate_routes_at(source_node_id, target_node_id, migrated_at)?;
        self.persist_node_bindings(target_node_id)?;
        Ok(migration)
    }

    fn lookup(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_id: &str,
        principal_kind: &str,
        device_id: &str,
    ) -> Option<RouteBinding> {
        self.memory
            .lookup(
                tenant_id,
                organization_id,
                principal_id,
                principal_kind,
                device_id,
            )
            .or_else(|| {
                self.hydrate_from_redis(
                    tenant_id,
                    organization_id,
                    principal_id,
                    principal_kind,
                    device_id,
                )
            })
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
        let current = self.memory.lookup(
            tenant_id,
            organization_id,
            principal_id,
            principal_kind,
            device_id,
        )?;
        if current.owner_node_id != owner_node_id {
            return None;
        }
        if let Err(error) = self.remove_binding(&current) {
            tracing::warn!(
                target: "sdkwork.im",
                event = "im.route_store.release_redis_failed",
                node_id = %owner_node_id,
                error = ?error,
            );
            return None;
        }
        self.memory.release(
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
        let restored = self
            .memory
            .restore_if_current(expected_current, restore_to)?;
        if let Err(error) = self.persist_binding(&restored) {
            tracing::warn!(
                target: "sdkwork.im",
                event = "im.route_store.restore_redis_failed",
                node_id = %restored.owner_node_id,
                error = ?error,
            );
        }
        Some(restored)
    }

    fn routes_for_node(&self, node_id: &str) -> Vec<RouteBinding> {
        self.memory.routes_for_node(node_id)
    }

    fn routes_for_node_page(
        &self,
        node_id: &str,
        cursor: Option<&str>,
        page_size: usize,
    ) -> RouteBindingPage {
        self.memory.routes_for_node_page(node_id, cursor, page_size)
    }

    fn node_lifecycle(&self, node_id: &str) -> Option<RouteNodeLifecycle> {
        self.memory.node_lifecycle(node_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_route_store_binding_key_is_segment_safe() {
        let binding = RouteBinding {
            tenant_id: "tenant".into(),
            organization_id: "0".into(),
            principal_id: "principal:segment".into(),
            principal_kind: "user".into(),
            device_id: "device".into(),
            owner_node_id: "node_a".into(),
            session_id: Some("s_demo".into()),
            connection_kind: "ccp/tcp/1".into(),
            bound_at: "2026-06-22T00:00:00.000Z".into(),
            route_epoch: 1,
        };
        let key = RedisBackedRouteStore::route_key_for_binding(&binding);
        assert!(key.contains("principal:segment"));
        assert!(
            RedisBackedRouteStore::binding_key(key.as_str()).starts_with(ROUTE_STORE_KEY_PREFIX)
        );
    }
}
