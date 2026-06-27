use std::sync::Arc;

use im_adapters_local_memory::{
    MemoryPresenceStateStore, MemoryRealtimeCheckpointStore, MemoryRealtimeDisconnectFenceStore,
    MemoryRealtimeEventWindowStore, MemoryRealtimeSubscriptionStore,
};
use im_adapters_postgres_realtime::{
    PostgresBackedRouteStore, PostgresRealtimeCheckpointStore, PostgresRealtimeConfig,
    PostgresRealtimeDisconnectFenceStore, PostgresRealtimeEventWindowStore,
    PostgresRealtimePool, PostgresRealtimePresenceStateStore, PostgresRealtimeSubscriptionStore,
};
use im_adapters_redis_cache::{RedisBackedRouteStore, RedisClusterBus};
use im_platform_contracts::ClusterEventBus;
use redis::Client as RedisClient;
use sdkwork_im_contract_control::{
    PresenceStateStore, RealtimeCheckpointStore, RealtimeDisconnectFenceStore,
    RealtimeSubscriptionStore,
};
use sdkwork_im_runtime_route::{memory_route_store, RouteStore};
use tracing::warn;

use crate::{
    PresenceRuntime, RealtimeClusterBridge, RealtimeDeliveryRuntime, RealtimePlaneAssembly,
    cluster_route_event_auth::{
        resolve_cluster_bus_secret_from_env, validate_realtime_node_id_for_cluster,
    },
    route_store_tier::RedisPostgresTieredRouteStore, resolve_realtime_node_id_from_env,
};

const REALTIME_CLUSTER_BUS_URL_ENV: &str = "SDKWORK_IM_REALTIME_CLUSTER_BUS_URL";
const REALTIME_ROUTE_STORE_URL_ENV: &str = "SDKWORK_IM_REALTIME_ROUTE_STORE_URL";
const REALTIME_DATABASE_URL_ENV: &str = "SDKWORK_IM_DATABASE_URL";

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
    let shared_cluster_bus = cluster_bus.clone().map(|bus| bus as Arc<dyn ClusterEventBus>);

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

    let assembly = if let Some(pool) = postgres_pool {
        let disconnect_fence_store =
            Arc::new(PostgresRealtimeDisconnectFenceStore::from_pool(pool.clone()));
        let checkpoint_store = Arc::new(PostgresRealtimeCheckpointStore::from_pool(pool.clone()));
        let subscription_store =
            Arc::new(PostgresRealtimeSubscriptionStore::from_pool(pool.clone()));
        let event_window_store = Arc::new(PostgresRealtimeEventWindowStore::from_pool(pool.clone()));
        let presence_state_store = Arc::new(PostgresRealtimePresenceStateStore::from_pool(pool));
        build_assembly_with_stores(
            disconnect_fence_store,
            checkpoint_store,
            subscription_store,
            event_window_store,
            presence_state_store,
            shared_cluster_bus,
            cluster_bus_secret,
            route_store,
        )
    } else {
        build_assembly_with_stores(
            Arc::new(MemoryRealtimeDisconnectFenceStore::default()),
            Arc::new(MemoryRealtimeCheckpointStore::default()),
            Arc::new(MemoryRealtimeSubscriptionStore::default()),
            Arc::new(MemoryRealtimeEventWindowStore::default()),
            Arc::new(MemoryPresenceStateStore::default()),
            shared_cluster_bus,
            cluster_bus_secret,
            route_store,
        )
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

fn build_assembly_with_stores<D, C, S, E, P>(
    disconnect_fence_store: Arc<D>,
    checkpoint_store: Arc<C>,
    subscription_store: Arc<S>,
    event_window_store: Arc<E>,
    presence_state_store: Arc<P>,
    cluster_bus: Option<Arc<dyn ClusterEventBus>>,
    cluster_bus_secret: Option<String>,
    route_store: Arc<dyn RouteStore>,
) -> RealtimePlaneAssembly
where
    D: RealtimeDisconnectFenceStore + 'static,
    C: RealtimeCheckpointStore + 'static,
    S: RealtimeSubscriptionStore + 'static,
    E: im_platform_contracts::RealtimeEventWindowStore + 'static,
    P: PresenceStateStore + 'static,
{
    let mut realtime_cluster = RealtimeClusterBridge::with_disconnect_fence_store_and_route_store(
        disconnect_fence_store,
        route_store,
    );
    if let Some(bus) = cluster_bus {
        realtime_cluster = realtime_cluster.with_cluster_bus(bus);
    }
    if let Some(secret) = cluster_bus_secret {
        realtime_cluster = realtime_cluster.with_cluster_bus_auth(secret);
    }

    RealtimePlaneAssembly::new(
        Arc::new(realtime_cluster),
        Arc::new(
            RealtimeDeliveryRuntime::with_durable_stores_for_standalone_gateway(
                checkpoint_store,
                subscription_store,
                event_window_store,
            ),
        ),
        Arc::new(PresenceRuntime::with_store(presence_state_store)),
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
            return RedisPostgresTieredRouteStore::new(redis_url, pool);
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

pub fn spawn_cluster_route_event_subscriber(
    bootstrap: &RealtimePlaneBootstrap,
) -> Option<std::thread::JoinHandle<()>> {
    let cluster_bus = bootstrap.cluster_bus.as_ref()?.clone();
    let cluster = bootstrap.assembly.realtime_cluster();
    let node_id = bootstrap.node_id.clone();

    Some(std::thread::spawn(move || loop {
        match cluster_bus.subscribe_connection(|message| -> redis::ControlFlow<()> {
            let payload = message.get_payload::<String>().unwrap_or_default();
            if payload.is_empty() {
                return redis::ControlFlow::Continue;
            }
            if let Err(delivery_error) =
                cluster.ingest_cluster_route_event_for_node(node_id.as_str(), payload.as_str())
            {
                warn!(
                    target: "sdkwork.im",
                    event = "im.realtime.cluster.ingress_failed",
                    node_id = %node_id,
                    code = delivery_error.code,
                    message = %delivery_error.message,
                );
            }
            redis::ControlFlow::Continue
        }) {
            Ok(_) => break,
            Err(error) => {
                warn!(
                    target: "sdkwork.im",
                    event = "im.realtime.cluster.subscribe_failed",
                    node_id = %node_id,
                    error = ?error,
                );
                std::thread::sleep(std::time::Duration::from_secs(1));
            }
        }
    }))
}
