use std::sync::Arc;
use std::time::Duration;

use futures_util::StreamExt;
use im_adapters_local_memory::{
    MemoryPresenceStateStore, MemoryRealtimeCheckpointStore, MemoryRealtimeDisconnectFenceStore,
    MemoryRealtimeEventWindowStore, MemoryRealtimeSubscriptionStore,
};
use im_adapters_postgres_realtime::{
    PostgresBackedRouteStore, PostgresRealtimeCheckpointStore, PostgresRealtimeConfig,
    PostgresRealtimeDisconnectFenceStore, PostgresRealtimeEventWindowStore, PostgresRealtimePool,
    PostgresRealtimePresenceStateStore, PostgresRealtimeSubscriptionStore,
};
use im_adapters_redis_cache::{RedisBackedRouteStore, RedisClusterBus};
use im_app_context::resolve_web_environment_from_process_env;
use im_platform_contracts::ClusterEventBus;
use redis::Client as RedisClient;
use sdkwork_im_contract_control::{
    PresenceStateStore, RealtimeCheckpointStore, RealtimeDisconnectFenceStore,
    RealtimeSubscriptionStore,
};
use sdkwork_im_runtime_route::{RouteStore, memory_route_store};
use sdkwork_web_core::WebEnvironment;

use crate::{
    ConversationMemberRealtimeScopeAccessPolicy, PresenceRuntime, RealtimeClusterBridge,
    RealtimeDeliveryRuntime, RealtimePlaneAssembly, StandaloneRealtimeScopeAccessPolicy,
    cluster_route_event_auth::{
        resolve_cluster_bus_secret_from_env, validate_realtime_node_id_for_cluster,
    },
    resolve_realtime_node_id_from_env,
    route_store_tier::RedisPostgresTieredRouteStore,
};
use im_adapters_postgres_journal::{
    PostgresJournalPool, conversation_member_access_gate_from_pool,
};
use sdkwork_im_database_pool::clone_shared_im_postgres_r2d2_pool;
use tokio::sync::watch;
use tokio::task::JoinHandle;
use tracing::warn;

const REALTIME_CLUSTER_BUS_URL_ENV: &str = "SDKWORK_IM_REALTIME_CLUSTER_BUS_URL";
const REALTIME_ROUTE_STORE_URL_ENV: &str = "SDKWORK_IM_REALTIME_ROUTE_STORE_URL";
const REALTIME_DATABASE_URL_ENV: &str = "SDKWORK_IM_DATABASE_URL";
const REALTIME_PERMISSIVE_SCOPE_ACCESS_ENV: &str = "SDKWORK_IM_REALTIME_PERMISSIVE_SCOPE_ACCESS";

fn resolve_realtime_scope_access_policy() -> std::sync::Arc<dyn crate::RealtimeScopeAccessPolicy> {
    let permissive = std::env::var(REALTIME_PERMISSIVE_SCOPE_ACCESS_ENV)
        .ok()
        .is_some_and(|value| value == "1" || value.eq_ignore_ascii_case("true"));
    let environment = resolve_web_environment_from_process_env();
    if permissive {
        if matches!(environment, WebEnvironment::Prod) {
            panic!(
                "{REALTIME_PERMISSIVE_SCOPE_ACCESS_ENV}=true is forbidden in production-like environments; \
                 realtime conversation scopes must be membership-gated via PostgreSQL"
            );
        }
        warn!(
            "{REALTIME_PERMISSIVE_SCOPE_ACCESS_ENV}=true; realtime conversation scopes are not membership-gated (development only)"
        );
        return std::sync::Arc::new(StandaloneRealtimeScopeAccessPolicy);
    }

    if let Some(pool) = clone_shared_im_postgres_r2d2_pool() {
        let gate = conversation_member_access_gate_from_pool(PostgresJournalPool::from_pool(pool));
        return std::sync::Arc::new(ConversationMemberRealtimeScopeAccessPolicy::new(gate));
    }

    if resolve_realtime_database_url_from_env().is_some() {
        let environment = resolve_web_environment_from_process_env();
        if matches!(environment, WebEnvironment::Dev | WebEnvironment::Test) {
            warn!(
                "PostgreSQL is configured but shared IM process pools are not installed; \
                 falling back to permissive realtime scope access until pools are bootstrapped (development only)"
            );
        } else if matches!(environment, WebEnvironment::Prod) {
            panic!(
                "PostgreSQL is configured ({REALTIME_DATABASE_URL_ENV}) but shared IM process pools are not installed; \
                 realtime conversation scopes cannot be membership-gated. Bootstrap shared Postgres pools before starting session-gateway in production."
            );
        } else {
            panic!(
                "PostgreSQL is configured ({REALTIME_DATABASE_URL_ENV}) but shared IM process pools are not installed; \
                 realtime conversation scopes cannot be membership-gated. Bootstrap shared Postgres pools before starting session-gateway."
            );
        }
    }

    if matches!(environment, WebEnvironment::Prod) {
        panic!(
            "session-gateway fail-closed: {REALTIME_DATABASE_URL_ENV} and shared IM Postgres pools are required \
             for membership-gated realtime scopes in production"
        );
    }

    std::sync::Arc::new(StandaloneRealtimeScopeAccessPolicy)
}

pub struct RealtimePlaneBootstrap {
    pub assembly: RealtimePlaneAssembly,
    pub node_id: String,
    pub cluster_bus: Option<Arc<RedisClusterBus>>,
    pub iam_auth_pool: Option<Arc<sqlx::PgPool>>,
}

pub async fn bootstrap_realtime_plane_from_env() -> Result<RealtimePlaneBootstrap, String> {
    let node_id = resolve_realtime_node_id_from_env();
    let cluster_enabled = resolve_route_store_redis_url().is_some();
    validate_realtime_node_id_for_cluster(node_id.as_str(), cluster_enabled)?;
    let cluster_bus_secret = if cluster_enabled {
        Some(resolve_cluster_bus_secret_from_env()?)
    } else {
        None
    };
    let cluster_bus = resolve_cluster_bus_from_env(node_id.as_str())?;
    let postgres_pool = connect_realtime_postgres_pool_from_env()?;
    let route_store = resolve_route_store_from_env(postgres_pool.clone())?;
    let shared_cluster_bus = cluster_bus
        .clone()
        .map(|bus| bus as Arc<dyn ClusterEventBus>);

    // HA fail-closed check: when cluster bus is enabled (multi-node HA topology),
    // the disconnect fence MUST use a shared storage backend (Postgres or Redis).
    // Falling back to in-memory storage in HA mode would allow stale session
    // takeover across nodes, defeating the purpose of the disconnect fence.
    if cluster_enabled && postgres_pool.is_none() {
        return Err(
            "HA topology detected (cluster bus enabled) but no PostgreSQL pool available \
             for disconnect fence storage. In-memory fallback is unsafe for multi-node \
             deployments. Set SDKWORK_IM_DATABASE_URL to a shared Postgres instance \
             or disable cluster mode by unsetting SDKWORK_IM_REALTIME_CLUSTER_BUS_URL."
                .to_owned(),
        );
    }

    if postgres_pool.is_none()
        && matches!(
            resolve_web_environment_from_process_env(),
            WebEnvironment::Prod
        )
    {
        return Err(format!(
            "session-gateway fail-closed: {REALTIME_DATABASE_URL_ENV} is required for durable realtime stores in production"
        ));
    }

    let assembly = if let Some(pool) = postgres_pool {
        let disconnect_fence_store = Arc::new(PostgresRealtimeDisconnectFenceStore::from_pool(
            pool.clone(),
        ));
        let checkpoint_store = Arc::new(PostgresRealtimeCheckpointStore::from_pool(pool.clone()));
        let subscription_store =
            Arc::new(PostgresRealtimeSubscriptionStore::from_pool(pool.clone()));
        let event_window_store =
            Arc::new(PostgresRealtimeEventWindowStore::from_pool(pool.clone()));
        let presence_state_store = Arc::new(PostgresRealtimePresenceStateStore::from_pool(pool));
        build_assembly_with_stores(RealtimeAssemblyStoreBundle {
            disconnect_fence_store,
            checkpoint_store,
            subscription_store,
            event_window_store,
            presence_state_store,
            shared_cluster_bus,
            cluster_bus_secret,
            route_store,
        })
    } else {
        build_assembly_with_stores(RealtimeAssemblyStoreBundle {
            disconnect_fence_store: Arc::new(MemoryRealtimeDisconnectFenceStore::default()),
            checkpoint_store: Arc::new(MemoryRealtimeCheckpointStore::default()),
            subscription_store: Arc::new(MemoryRealtimeSubscriptionStore::default()),
            event_window_store: Arc::new(MemoryRealtimeEventWindowStore::default()),
            presence_state_store: Arc::new(MemoryPresenceStateStore::default()),
            shared_cluster_bus,
            cluster_bus_secret,
            route_store,
        })
    };

    assembly.bind_node_runtime(node_id.as_str());

    let iam_auth_pool = crate::resolve_iam_auth_pool_from_env().await;

    Ok(RealtimePlaneBootstrap {
        assembly,
        node_id,
        cluster_bus,
        iam_auth_pool,
    })
}

struct RealtimeAssemblyStoreBundle<D, C, S, E, P> {
    disconnect_fence_store: Arc<D>,
    checkpoint_store: Arc<C>,
    subscription_store: Arc<S>,
    event_window_store: Arc<E>,
    presence_state_store: Arc<P>,
    shared_cluster_bus: Option<Arc<dyn ClusterEventBus>>,
    cluster_bus_secret: Option<String>,
    route_store: Arc<dyn RouteStore>,
}

fn build_assembly_with_stores<D, C, S, E, P>(
    bundle: RealtimeAssemblyStoreBundle<D, C, S, E, P>,
) -> RealtimePlaneAssembly
where
    D: RealtimeDisconnectFenceStore + 'static,
    C: RealtimeCheckpointStore + 'static,
    S: RealtimeSubscriptionStore + 'static,
    E: im_platform_contracts::RealtimeEventWindowStore + 'static,
    P: PresenceStateStore + 'static,
{
    let mut realtime_cluster = RealtimeClusterBridge::with_disconnect_fence_store_and_route_store(
        bundle.disconnect_fence_store,
        bundle.route_store,
    );
    if let Some(bus) = bundle.shared_cluster_bus {
        realtime_cluster = realtime_cluster.with_cluster_bus(bus);
    }
    if let Some(secret) = bundle.cluster_bus_secret {
        realtime_cluster = realtime_cluster.with_cluster_bus_auth(secret);
    }

    RealtimePlaneAssembly::new(
        Arc::new(realtime_cluster),
        Arc::new(
            RealtimeDeliveryRuntime::with_durable_stores_and_scope_access_policy(
                bundle.checkpoint_store,
                bundle.subscription_store,
                bundle.event_window_store,
                resolve_realtime_scope_access_policy(),
            ),
        ),
        Arc::new(PresenceRuntime::with_store(bundle.presence_state_store)),
    )
}

fn resolve_realtime_database_url_from_env() -> Option<String> {
    std::env::var(REALTIME_DATABASE_URL_ENV)
        .ok()
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())
}

fn connect_realtime_postgres_pool_from_env() -> Result<Option<PostgresRealtimePool>, String> {
    let Some(database_url) = resolve_realtime_database_url_from_env() else {
        return Ok(None);
    };
    let pool_max_size = std::env::var("SDKWORK_IM_REALTIME_POSTGRES_POOL_MAX_SIZE")
        .ok()
        .and_then(|value| value.trim().parse::<u32>().ok())
        .unwrap_or(32);
    let pool_min_idle = std::env::var("SDKWORK_IM_REALTIME_POSTGRES_POOL_MIN_IDLE")
        .ok()
        .and_then(|value| value.trim().parse::<u32>().ok())
        .unwrap_or(0);
    let config = PostgresRealtimeConfig::new(database_url)
        .with_pool_max_size(pool_max_size)
        .with_pool_min_idle(pool_min_idle);
    config
        .connect_pool()
        .map(Some)
        .map_err(|error| format!("connect postgres realtime pool failed: {error:?}"))
}

fn resolve_cluster_bus_from_env(node_id: &str) -> Result<Option<Arc<RedisClusterBus>>, String> {
    let redis_url = resolve_route_store_redis_url();
    if redis_url.is_none() {
        return Ok(None);
    }

    let client = RedisClient::open(redis_url.unwrap())
        .map_err(|error| format!("invalid redis cluster bus url: {error}"))?;
    Ok(Some(Arc::new(RedisClusterBus::new(client, node_id))))
}

fn resolve_route_store_from_env(
    postgres_pool: Option<PostgresRealtimePool>,
) -> Result<Arc<dyn RouteStore>, String> {
    if let Some(redis_url) = resolve_route_store_redis_url() {
        if let Some(pool) = postgres_pool {
            return RedisPostgresTieredRouteStore::create(redis_url, pool);
        }
        return RedisBackedRouteStore::new(redis_url).map(|store| store.into_arc());
    }
    if let Some(pool) = postgres_pool {
        return Ok(PostgresBackedRouteStore::from_pool(pool).into_arc());
    }
    Ok(memory_route_store())
}

fn resolve_route_store_redis_url() -> Option<String> {
    std::env::var(REALTIME_ROUTE_STORE_URL_ENV)
        .ok()
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())
        .or_else(|| {
            std::env::var(REALTIME_CLUSTER_BUS_URL_ENV)
                .ok()
                .map(|value| value.trim().to_owned())
                .filter(|value| !value.is_empty())
        })
}

pub struct ClusterRouteEventSubscriber {
    shutdown_tx: watch::Sender<bool>,
    handle: JoinHandle<()>,
}

impl ClusterRouteEventSubscriber {
    pub async fn shutdown(self, timeout: Duration) {
        let _ = self.shutdown_tx.send(true);
        let mut handle = self.handle;
        if tokio::time::timeout(timeout, &mut handle).await.is_err() {
            handle.abort();
            let _ = handle.await;
        }
    }
}

pub fn spawn_cluster_route_event_subscriber(
    bootstrap: &RealtimePlaneBootstrap,
) -> Option<ClusterRouteEventSubscriber> {
    let cluster_bus = bootstrap.cluster_bus.as_ref()?.clone();
    let cluster = bootstrap.assembly.realtime_cluster();
    let node_id = bootstrap.node_id.clone();
    let (shutdown_tx, mut shutdown_rx) = watch::channel(false);
    let handle = tokio::spawn(async move {
        while !*shutdown_rx.borrow() {
            let subscription = tokio::select! {
                changed = shutdown_rx.changed() => {
                    if changed.is_err() || *shutdown_rx.borrow() {
                        return;
                    }
                    continue;
                }
                result = cluster_bus.subscribe_async() => result,
            };
            let mut pubsub = match subscription {
                Ok(pubsub) => pubsub,
                Err(error) => {
                    warn!(
                        target: "sdkwork.im",
                        event = "im.realtime.cluster.subscribe_failed",
                        node_id = %node_id,
                        error = ?error,
                    );
                    tokio::select! {
                        _ = tokio::time::sleep(Duration::from_secs(1)) => {}
                        changed = shutdown_rx.changed() => {
                            if changed.is_err() || *shutdown_rx.borrow() {
                                return;
                            }
                        }
                    }
                    continue;
                }
            };
            let mut messages = pubsub.on_message();
            loop {
                let message = tokio::select! {
                    changed = shutdown_rx.changed() => {
                        if changed.is_err() || *shutdown_rx.borrow() {
                            return;
                        }
                        continue;
                    }
                    message = messages.next() => message,
                };
                let Some(message) = message else {
                    break;
                };
                let payload = message.get_payload::<String>().unwrap_or_default();
                if payload.is_empty() {
                    continue;
                }
                let cluster_handle = cluster.clone();
                let node_id_owned = node_id.clone();
                let payload_owned = payload.clone();
                let ingress = tokio::task::spawn_blocking(move || {
                    cluster_handle.ingest_cluster_route_event_for_node(
                        node_id_owned.as_str(),
                        payload_owned.as_str(),
                    )
                })
                .await;
                match ingress {
                    Ok(Ok(_)) => {}
                    Ok(Err(delivery_error)) => {
                        warn!(
                            target: "sdkwork.im",
                            event = "im.realtime.cluster.ingress_failed",
                            node_id = %node_id,
                            code = delivery_error.code,
                            message = %delivery_error.message,
                        );
                    }
                    Err(join_error) => {
                        warn!(
                            target: "sdkwork.im",
                            event = "im.realtime.cluster.ingress_task_failed",
                            node_id = %node_id,
                            error = %join_error,
                        );
                    }
                }
            }
        }
    });
    Some(ClusterRouteEventSubscriber {
        shutdown_tx,
        handle,
    })
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use tokio::sync::watch;

    use super::ClusterRouteEventSubscriber;

    #[tokio::test]
    async fn cluster_subscriber_shutdown_aborts_an_unresponsive_task() {
        let (shutdown_tx, _shutdown_rx) = watch::channel(false);
        let handle = tokio::spawn(std::future::pending());
        let subscriber = ClusterRouteEventSubscriber {
            shutdown_tx,
            handle,
        };

        tokio::time::timeout(
            Duration::from_millis(100),
            subscriber.shutdown(Duration::ZERO),
        )
        .await
        .expect("subscriber shutdown must not wait forever after its deadline");
    }
}
