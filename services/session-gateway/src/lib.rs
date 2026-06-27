use std::sync::Arc;

use axum::extract::Extension;
use axum::http::HeaderMap;
use axum::{
    Router,
    routing::{get, post},
};
use im_app_context::AppContext;
use tokio::sync::Semaphore;

mod api_error;
mod rpc_dispatch;
mod gateway_embed;
mod link_framing;
mod link_quic;
mod link_business_contract;
mod link_realtime;
mod link_transport;
mod assembly;
mod auth_context;
mod route_store_tier;
mod runtime_bootstrap;
mod client_route_registration;
mod client_route_state;
mod cluster;
mod cluster_route_event_auth;
mod presence;
mod presence_routes;
mod principal_scope;
mod realtime;
mod websocket;
mod websocket_route;
mod websocket_upgrade;
mod websocket_auth_init;
mod http_limits;
mod http_guardrails;
mod service_readiness;
mod openapi_export;
mod realtime_http_routes;

pub use rpc_dispatch::{SessionGatewayRpcDispatcher, SESSION_GATEWAY_RPC_SERVICE_KEYS};
pub use gateway_embed::{
    bootstrap_gateway_embedded_realtime_plane, GatewayEmbeddedRealtimePlane,
};
pub use link_transport::spawn_link_transport_listeners;
pub use auth_context::{RealtimeAuthContextResolver, resolve_iam_auth_pool_from_env};
pub use assembly::RealtimePlaneAssembly;
pub use runtime_bootstrap::{
    bootstrap_realtime_plane_from_env, spawn_cluster_route_event_subscriber, RealtimePlaneBootstrap,
};
pub use api_error::ApiError;
pub use http_limits::{
    REALTIME_ACCEPT_LEGACY_WEBSOCKET_JSON_ENV, resolve_max_websocket_connections,
    resolve_realtime_node_id_from_env, realtime_accepts_legacy_websocket_json,
};
use client_route_registration::ClientRouteRegistration;
use client_route_state::ClientRouteState;
pub use cluster::{
    RealtimeClientRoute, RealtimeClusterBridge, RealtimeClusterError, RealtimeNodeLifecycleView,
    RealtimeRouteDeliveryResult, RealtimeRouteMigrationResult,
};
pub use presence::{PresenceRuntime, PresenceRuntimeError};
pub use realtime::{
    RealtimeClientRouteStateSnapshot, RealtimeDeliveryRuntime, RealtimeInboxDiagnosticsSnapshot,
    RealtimePostgresAdapterPlan, RealtimePostgresBindingError, RealtimePostgresBindingValue,
    RealtimePostgresBoundParameter, RealtimePostgresBoundStatement, RealtimePostgresBoundTransaction,
    RealtimePostgresCheckpointMutation, RealtimePostgresClientRouteEventMutation,
    RealtimePostgresMethodAtomicity, RealtimePostgresMethodPlan, RealtimePostgresMethodStep,
    RealtimePostgresParameterBinding, RealtimePostgresRowColumn, RealtimePostgresRowMapping,
    RealtimePostgresSqlContract, RealtimeRuntimeError, RealtimeScopeAccessPolicy,
    RealtimeSubscriptionItemInput, StandaloneRealtimeScopeAccessPolicy,
    SyncRealtimeSubscriptionsRequest, realtime_postgres_adapter_plan,
    realtime_postgres_bind_ack_transaction, realtime_postgres_bind_checkpoint_upsert,
    realtime_postgres_bind_client_route_event_upsert, realtime_postgres_bind_publish_transaction,
    realtime_postgres_bind_save_subscription_transaction,
    realtime_postgres_bind_subscription_scope_clear,
    realtime_postgres_bind_subscription_scope_replacements,
    realtime_postgres_bind_subscription_upsert, realtime_postgres_bind_trim_client_route_events,
    realtime_postgres_sql_contract_specs, realtime_postgres_sql_contracts,
    realtime_postgres_transaction_plans,
};
pub use cluster_route_event_auth::REALTIME_CLUSTER_BUS_SECRET_ENV;
pub use websocket::{
    CCP_WEBSOCKET_SUBPROTOCOL, REALTIME_OVERLOAD_CLOSE_CODE, REALTIME_OVERLOAD_CLOSE_REASON,
    RealtimeRouteOwner, RealtimeRouteOwnerError, RealtimeWebsocketMode,
    SESSION_DISCONNECT_CLOSE_CODE, SESSION_DISCONNECT_CLOSE_REASON, serve_realtime_websocket,
};
pub use http_guardrails::apply_public_http_guardrails;
use service_readiness::{healthz, readyz, ServiceReadiness};

#[derive(Clone)]
pub struct AppState {
    presence_runtime: Arc<PresenceRuntime>,
    realtime_runtime: Arc<RealtimeDeliveryRuntime>,
    client_route_state: ClientRouteState,
    client_route_registration: ClientRouteRegistration,
    websocket_connection_semaphore: Arc<Semaphore>,
    readiness: ServiceReadiness,
    auth_resolver: RealtimeAuthContextResolver,
}

#[derive(Debug, serde::Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub(crate) struct PresenceHeartbeatRequest {
    device_id: Option<String>,
}

pub fn build_app() -> Router {
    build_app_with_state(AppState::default())
}

pub fn build_app_with_cluster(realtime_cluster: Arc<RealtimeClusterBridge>) -> Router {
    build_app_with_state(AppState::with_cluster(realtime_cluster))
}

pub fn build_app_with_cluster_and_runtime(
    realtime_cluster: Arc<RealtimeClusterBridge>,
    realtime_runtime: Arc<RealtimeDeliveryRuntime>,
) -> Router {
    build_app_with_state(AppState::with_cluster_and_runtime(
        realtime_cluster,
        realtime_runtime,
    ))
}

pub fn build_app_with_cluster_runtime_and_presence(
    realtime_cluster: Arc<RealtimeClusterBridge>,
    realtime_runtime: Arc<RealtimeDeliveryRuntime>,
    presence_runtime: Arc<PresenceRuntime>,
) -> Router {
    build_app_with_state(AppState::with_cluster_and_runtime_and_presence(
        realtime_cluster,
        realtime_runtime,
        presence_runtime,
    ))
}

pub fn default_app_state() -> AppState {
    AppState::default()
}

/// Realtime websocket upgrade route mounted outside the SDKWork HTTP interceptor pipeline.
///
/// Browser clients authenticate through the first `auth.init` frame after upgrade; wrapping this
/// route with `WebFrameworkLayer` breaks Axum websocket upgrade state and returns HTTP 426.
pub fn build_realtime_websocket_router(state: AppState) -> Router {
    Router::new()
        .route(
            "/im/v3/api/realtime/ws",
            get(websocket_upgrade::realtime_websocket),
        )
        .with_state(state)
}

pub fn build_domain_http_api_router(state: AppState) -> Router {
    Router::new()
        .route("/readyz", get(readyz))
        .route(
            "/im/v3/api/presence/heartbeat",
            post(presence_routes::heartbeat_presence),
        )
        .route(
            "/im/v3/api/presence/me",
            get(presence_routes::get_presence_me),
        )
        .route(
            "/im/v3/api/realtime/subscriptions/sync",
            post(realtime_http_routes::sync_realtime_subscriptions),
        )
        .route(
            "/im/v3/api/realtime/events/ack",
            post(realtime_http_routes::ack_realtime_events),
        )
        .route("/im/v3/api/realtime/events", get(realtime_http_routes::list_realtime_events))
        .with_state(state)
}

pub fn build_domain_api_router(state: AppState) -> Router {
    build_realtime_websocket_router(state.clone()).merge(build_domain_http_api_router(state))
}

fn build_infra_and_http_router(state: AppState) -> Router {
    Router::new()
        .route("/healthz", get(healthz))
        .route("/openapi.json", get(openapi_export::openapi_json))
        .route("/docs", get(openapi_export::docs))
        .merge(build_domain_http_api_router(state))
}

pub fn build_public_http_app(state: AppState) -> Router {
    apply_public_http_guardrails(build_infra_and_http_router(state))
}

pub fn compose_public_app_router(state: AppState) -> Router {
    build_realtime_websocket_router(state.clone()).merge(build_public_http_app(state))
}

pub fn build_public_app() -> Router {
    compose_public_app_router(AppState::default())
}

pub fn build_public_app_with_state(state: AppState) -> Router {
    compose_public_app_router(state)
}

pub fn build_public_app_with_realtime_plane(
    assembly: RealtimePlaneAssembly,
    node_id: &str,
) -> Router {
    build_public_app_with_realtime_bootstrap(&RealtimePlaneBootstrap {
        assembly,
        node_id: node_id.to_owned(),
        cluster_bus: None,
        iam_auth_pool: None,
    })
}

pub fn build_public_app_with_realtime_bootstrap(bootstrap: &RealtimePlaneBootstrap) -> Router {
    build_public_app_with_state(AppState::from_realtime_bootstrap(bootstrap))
}

fn build_app_with_state(state: AppState) -> Router {
    compose_public_app_router(state)
}

impl Default for AppState {
    fn default() -> Self {
        Self::with_cluster(Arc::new(RealtimeClusterBridge::default()))
    }
}

impl AppState {
    pub fn from_realtime_plane(assembly: RealtimePlaneAssembly, node_id: &str) -> Self {
        Self::from_realtime_bootstrap(&RealtimePlaneBootstrap {
            assembly,
            node_id: node_id.to_owned(),
            cluster_bus: None,
            iam_auth_pool: None,
        })
    }

    pub fn from_realtime_bootstrap(bootstrap: &RealtimePlaneBootstrap) -> Self {
        Self::with_cluster_runtime_presence_node_and_auth(
            bootstrap.assembly.realtime_cluster(),
            bootstrap.assembly.realtime_runtime(),
            bootstrap.assembly.presence_runtime(),
            bootstrap.node_id.clone(),
            RealtimeAuthContextResolver::new(bootstrap.iam_auth_pool.clone()),
        )
    }

    fn with_cluster(realtime_cluster: Arc<RealtimeClusterBridge>) -> Self {
        Self::with_cluster_and_runtime(
            realtime_cluster,
            Arc::new(RealtimeDeliveryRuntime::standalone_gateway()),
        )
    }

    fn with_cluster_and_runtime(
        realtime_cluster: Arc<RealtimeClusterBridge>,
        realtime_runtime: Arc<RealtimeDeliveryRuntime>,
    ) -> Self {
        Self::with_cluster_and_runtime_and_presence(
            realtime_cluster,
            realtime_runtime,
            Arc::new(PresenceRuntime::default()),
        )
    }

    fn with_cluster_and_runtime_and_presence(
        realtime_cluster: Arc<RealtimeClusterBridge>,
        realtime_runtime: Arc<RealtimeDeliveryRuntime>,
        presence_runtime: Arc<PresenceRuntime>,
    ) -> Self {
        Self::with_cluster_runtime_presence_and_node_id(
            realtime_cluster,
            realtime_runtime,
            presence_runtime,
            resolve_realtime_node_id_from_env(),
        )
    }

    fn with_cluster_runtime_presence_and_node_id(
        realtime_cluster: Arc<RealtimeClusterBridge>,
        realtime_runtime: Arc<RealtimeDeliveryRuntime>,
        presence_runtime: Arc<PresenceRuntime>,
        node_id: String,
    ) -> Self {
        Self::with_cluster_runtime_presence_node_and_auth(
            realtime_cluster,
            realtime_runtime,
            presence_runtime,
            node_id,
            RealtimeAuthContextResolver::default(),
        )
    }

    fn with_cluster_runtime_presence_node_and_auth(
        realtime_cluster: Arc<RealtimeClusterBridge>,
        realtime_runtime: Arc<RealtimeDeliveryRuntime>,
        presence_runtime: Arc<PresenceRuntime>,
        node_id: String,
        auth_resolver: RealtimeAuthContextResolver,
    ) -> Self {
        realtime_cluster.bind_node_runtime(node_id.as_str(), realtime_runtime.clone());
        let client_route_state = ClientRouteState::default();
        let max_connections = resolve_max_websocket_connections();
        Self {
            client_route_registration: ClientRouteRegistration::new(
                node_id.clone(),
                realtime_cluster.clone(),
                presence_runtime.clone(),
                realtime_runtime.clone(),
                client_route_state.clone(),
            ),
            presence_runtime,
            realtime_runtime,
            client_route_state,
            websocket_connection_semaphore: Arc::new(Semaphore::new(max_connections)),
            readiness: ServiceReadiness::from_env(),
            auth_resolver,
        }
    }

    fn prepare_active_client_route(
        &self,
        auth: &AppContext,
        device_id: &str,
        connection_kind: &str,
        allow_session_takeover: bool,
    ) -> Result<(), ApiError> {
        self.client_route_registration.prepare_active_client_route(
            auth,
            device_id,
            connection_kind,
            allow_session_takeover,
        )
    }

    fn current_active_client_route(
        &self,
        auth: &AppContext,
        device_id: &str,
    ) -> Option<RealtimeClientRoute> {
        self.client_route_registration
            .current_active_client_route(auth, device_id)
    }

    fn restore_active_client_route_if_current(
        &self,
        expected_current: &RealtimeClientRoute,
        restore_to: RealtimeClientRoute,
    ) -> Option<RealtimeClientRoute> {
        self.client_route_registration
            .restore_active_client_route_if_current(expected_current, restore_to)
    }

    fn release_active_client_route_if_current_session(&self, auth: &AppContext, device_id: &str) {
        self.client_route_registration
            .release_active_client_route_if_current_session(auth, device_id);
    }

    fn client_route_state_snapshot(
        &self,
        auth: &AppContext,
        requested_device_id: Option<&str>,
    ) -> Result<client_route_state::ClientRouteStateSnapshot, ApiError> {
        self.client_route_state
            .client_route_state_snapshot(auth, requested_device_id)
    }

    pub(crate) fn rpc_prepare_active_client_route(
        &self,
        auth: &AppContext,
        device_id: &str,
        connection_kind: &str,
    ) -> Result<(), ApiError> {
        self.prepare_active_client_route(auth, device_id, connection_kind, false)
    }

    pub(crate) fn rpc_client_route_state_snapshot(
        &self,
        auth: &AppContext,
        requested_device_id: Option<&str>,
    ) -> Result<client_route_state::ClientRouteStateSnapshot, ApiError> {
        self.client_route_state_snapshot(auth, requested_device_id)
    }

    pub(crate) fn rpc_current_active_client_route(
        &self,
        auth: &AppContext,
        device_id: &str,
    ) -> Option<RealtimeClientRoute> {
        self.current_active_client_route(auth, device_id)
    }

    pub(crate) fn rpc_restore_active_client_route_if_current(
        &self,
        expected_current: &RealtimeClientRoute,
        restore_to: RealtimeClientRoute,
    ) -> Option<RealtimeClientRoute> {
        self.restore_active_client_route_if_current(expected_current, restore_to)
    }

    pub(crate) fn rpc_release_active_client_route_if_current_session(
        &self,
        auth: &AppContext,
        device_id: &str,
    ) {
        self.release_active_client_route_if_current_session(auth, device_id);
    }

    pub(crate) fn rpc_presence_runtime(&self) -> &Arc<PresenceRuntime> {
        &self.presence_runtime
    }

    pub(crate) fn rpc_realtime_runtime(&self) -> &Arc<RealtimeDeliveryRuntime> {
        &self.realtime_runtime
    }

    pub(crate) fn rpc_auth_resolver(&self) -> &RealtimeAuthContextResolver {
        &self.auth_resolver
    }
}

pub(crate) fn resolve_requested_device_id(
    auth: &AppContext,
    requested_device_id: Option<String>,
) -> Result<String, ApiError> {
    sdkwork_im_websocket_auth_gate::resolve_websocket_device_binding(auth, requested_device_id)
        .map_err(|error| ApiError::bad_request(error.code, error.message))
}

pub(crate) async fn resolve_request_app_context(
    auth: Option<Extension<AppContext>>,
    headers: &HeaderMap,
    auth_resolver: &RealtimeAuthContextResolver,
) -> Result<AppContext, ApiError> {
    match auth {
        Some(Extension(auth)) => Ok(auth),
        None => auth_resolver
            .resolve_from_headers(headers)
            .await
            .map_err(ApiError::from),
    }
}

#[cfg(test)]
mod canonical_path_contract_tests {
    use super::{
        REALTIME_ACCEPT_LEGACY_WEBSOCKET_JSON_ENV, REALTIME_CLUSTER_BUS_SECRET_ENV,
        realtime_accepts_legacy_websocket_json,
    };
    use sdkwork_im_realtime_api_paths::{
        PRESENCE_HEARTBEAT, PRESENCE_ME, REALTIME_EVENTS, REALTIME_EVENTS_ACK,
        REALTIME_SUBSCRIPTIONS_SYNC, REALTIME_WS,
    };

    #[test]
    fn build_domain_api_router_literals_match_canonical_paths() {
        let source = include_str!("lib.rs").replace('\r', "");
        for path in [
            PRESENCE_HEARTBEAT,
            PRESENCE_ME,
            REALTIME_SUBSCRIPTIONS_SYNC,
            REALTIME_WS,
            REALTIME_EVENTS_ACK,
            REALTIME_EVENTS,
        ] {
            assert!(
                source.contains(path),
                "router source must declare literal path `{path}` for OpenAPI extraction"
            );
        }
    }

    #[test]
    fn ha_cluster_bus_secret_env_is_canonical() {
        assert_eq!(
            REALTIME_CLUSTER_BUS_SECRET_ENV,
            "SDKWORK_IM_REALTIME_CLUSTER_BUS_SECRET"
        );
    }

    #[test]
    fn legacy_websocket_json_compat_defaults_to_reject_when_unset() {
        let previous = std::env::var(REALTIME_ACCEPT_LEGACY_WEBSOCKET_JSON_ENV).ok();
        unsafe {
            std::env::remove_var(REALTIME_ACCEPT_LEGACY_WEBSOCKET_JSON_ENV);
        }
        assert!(!realtime_accepts_legacy_websocket_json());
        if let Some(value) = previous {
            unsafe {
                std::env::set_var(REALTIME_ACCEPT_LEGACY_WEBSOCKET_JSON_ENV, value);
            }
        }
    }
}
