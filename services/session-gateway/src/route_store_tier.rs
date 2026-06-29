//! Redis hot route store with PostgreSQL durable mirror (bounded retry on write failures).

use std::sync::Arc;

use im_adapters_postgres_realtime::{PostgresRealtimePool, PostgresRoutePersistence};
use im_adapters_redis_cache::RedisBackedRouteStore;
use sdkwork_im_runtime_route::{
    RouteBinding, RouteBindingRequest, RouteMigrationResult, RouteNodeLifecycle, RouteRuntimeError,
    RouteStore,
};
use tracing::warn;

const POSTGRES_MIRROR_MAX_ATTEMPTS: u32 = 3;
// P1-6 fix: Increased base backoff from 1ms to 10ms for better fault tolerance
// New backoff sequence: 10ms, 20ms, 40ms (was 1ms, 2ms, 4ms)
const POSTGRES_MIRROR_BASE_BACKOFF_MS: u64 = 10;

#[derive(Clone)]
pub struct RedisPostgresTieredRouteStore {
    redis: RedisBackedRouteStore,
    postgres: PostgresRoutePersistence,
}

impl RedisPostgresTieredRouteStore {
    pub fn new(
        redis_url: impl AsRef<str>,
        pool: PostgresRealtimePool,
    ) -> Result<Arc<dyn RouteStore>, String> {
        Ok(Arc::new(Self {
            redis: RedisBackedRouteStore::new(redis_url)?,
            postgres: PostgresRoutePersistence::from_pool(pool),
        }))
    }

    fn mirror_persist_with_retry(&self, binding: &RouteBinding) {
        for attempt in 1..=POSTGRES_MIRROR_MAX_ATTEMPTS {
            match self.postgres.persist(binding) {
                Ok(()) => return,
                Err(error) => {
                    if attempt == POSTGRES_MIRROR_MAX_ATTEMPTS {
                        warn!(
                            target: "sdkwork.im",
                            event = "im.realtime.route_store.postgres_mirror_failed",
                            node_id = %binding.owner_node_id,
                            code = error.code,
                            message = %error.message,
                            attempts = attempt,
                        );
                    } else {
                        warn!(
                            target: "sdkwork.im",
                            event = "im.realtime.route_store.postgres_mirror_retry",
                            node_id = %binding.owner_node_id,
                            code = error.code,
                            message = %error.message,
                            attempt,
                        );
                        // P1-6 fix: Exponential backoff with increased base (10ms, 20ms, 40ms)
                        // This provides better tolerance for transient database failures
                        // compared to the previous 1ms/2ms/4ms sequence.
                        // Total worst-case delay: 70ms over 3 attempts (was 7ms).
                        let backoff_ms = POSTGRES_MIRROR_BASE_BACKOFF_MS << (attempt - 1);
                        std::thread::sleep(std::time::Duration::from_millis(backoff_ms));
                    }
                }
            }
        }
    }

    fn mirror_remove_with_retry(&self, binding: &RouteBinding) {
        for attempt in 1..=POSTGRES_MIRROR_MAX_ATTEMPTS {
            match self.postgres.remove(binding) {
                Ok(()) => return,
                Err(error) => {
                    if attempt == POSTGRES_MIRROR_MAX_ATTEMPTS {
                        warn!(
                            target: "sdkwork.im",
                            event = "im.realtime.route_store.postgres_mirror_delete_failed",
                            node_id = %binding.owner_node_id,
                            code = error.code,
                            message = %error.message,
                            attempts = attempt,
                        );
                    } else {
                        warn!(
                            target: "sdkwork.im",
                            event = "im.realtime.route_store.postgres_mirror_delete_retry",
                            node_id = %binding.owner_node_id,
                            code = error.code,
                            message = %error.message,
                            attempt,
                        );
                        // P1-6 fix: Exponential backoff with increased base (10ms, 20ms, 40ms)
                        let backoff_ms = POSTGRES_MIRROR_BASE_BACKOFF_MS << (attempt - 1);
                        std::thread::sleep(std::time::Duration::from_millis(backoff_ms));
                    }
                }
            }
        }
    }
}

impl RouteStore for RedisPostgresTieredRouteStore {
    fn register_node(&self, node_id: &str) {
        self.redis.register_node(node_id);
    }

    fn bind(&self, request: RouteBindingRequest) -> Result<RouteBinding, RouteRuntimeError> {
        let binding = self.redis.bind(request)?;
        match self.postgres.persist(&binding) {
            Ok(()) => Ok(binding),
            Err(error) => {
                let _ = self.redis.release(
                    binding.tenant_id.as_str(),
                    binding.organization_id.as_str(),
                    binding.principal_id.as_str(),
                    binding.principal_kind.as_str(),
                    binding.device_id.as_str(),
                    binding.owner_node_id.as_str(),
                );
                Err(error)
            }
        }
    }

    fn mark_node_draining(&self, node_id: &str) -> Result<RouteNodeLifecycle, RouteRuntimeError> {
        self.redis.mark_node_draining(node_id)
    }

    fn activate_node(&self, node_id: &str) -> Result<RouteNodeLifecycle, RouteRuntimeError> {
        self.redis.activate_node(node_id)
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
            self.redis
                .migrate_routes_at(source_node_id, target_node_id, migrated_at)?;
        for route in self.redis.routes_for_node(target_node_id) {
            self.mirror_persist_with_retry(&route);
        }
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
        if let Some(binding) = self.redis.lookup(
            tenant_id,
            organization_id,
            principal_id,
            principal_kind,
            device_id,
        ) {
            return Some(binding);
        }
        let binding = self.postgres.load(
            tenant_id,
            organization_id,
            principal_id,
            principal_kind,
            device_id,
        )?;
        if let Err(error) = self.redis.warm_binding(&binding) {
            warn!(
                target: "sdkwork.im",
                event = "im.realtime.route_store.redis_warm_failed",
                node_id = %binding.owner_node_id,
                code = error.code,
                message = %error.message,
            );
        }
        Some(binding)
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
        let removed = self.redis.release(
            tenant_id,
            organization_id,
            principal_id,
            principal_kind,
            device_id,
            owner_node_id,
        )?;
        self.mirror_remove_with_retry(&removed);
        Some(removed)
    }

    fn restore_if_current(
        &self,
        expected_current: &RouteBinding,
        restore_to: RouteBinding,
    ) -> Option<RouteBinding> {
        let restored = self.redis.restore_if_current(expected_current, restore_to)?;
        self.mirror_persist_with_retry(&restored);
        Some(restored)
    }

    fn routes_for_node(&self, node_id: &str) -> Vec<RouteBinding> {
        self.redis.routes_for_node(node_id)
    }

    fn node_lifecycle(&self, node_id: &str) -> Option<RouteNodeLifecycle> {
        self.redis.node_lifecycle(node_id)
    }
}
