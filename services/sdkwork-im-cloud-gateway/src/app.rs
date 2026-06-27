//! Gateway application assembly: the `build_app*` family constructs the axum
//! [`Router`] with proxy routes, OpenAPI endpoints, CORS, rate limiting, and
//! the embedded realtime websocket router.

use axum::{Router, middleware::from_fn_with_state, routing::get};
use crate::gateway_protection::{
    CircuitBreakerConfig, CircuitBreakerRegistry, RateLimitConfig, RateLimiter,
    rate_limit_middleware,
};
use sdkwork_im_api_registry::RouteRegistry;
use session_gateway::{AppState, RealtimeAuthContextResolver, resolve_iam_auth_pool_from_env};
use sdkwork_im_cloud_gateway_config::WebGatewayConfig;

use crate::client::build_gateway_upstream_client;
use crate::cors::build_browser_cors_layer;
use crate::openapi::{
    docs, openapi_index_json, openapi_json, openapi_runtime_summary_json, service_docs,
    service_openapi_json,
};
use crate::registry::build_gateway_registry;
use crate::response::gateway_proxy_routes;
use crate::state::GatewayState;

pub fn build_app(config: WebGatewayConfig) -> Router {
    build_app_with_registry(
        config,
        build_gateway_registry().expect("sdkwork-im-cloud-gateway route registry should build"),
    )
}

pub fn build_app_with_registry(config: WebGatewayConfig, registry: RouteRegistry) -> Router {
    build_app_with_registry_and_product_runtime(config, registry, None)
}

pub fn build_app_with_registry_and_product_runtime(
    config: WebGatewayConfig,
    registry: RouteRegistry,
    product_runtime_router: Option<Router>,
) -> Router {
    build_app_with_registry_product_runtime_and_embedded_services(
        config,
        registry,
        product_runtime_router,
        None,
        None,
    )
}

pub fn build_app_with_registry_product_runtime_and_embedded_services(
    config: WebGatewayConfig,
    registry: RouteRegistry,
    product_runtime_router: Option<Router>,
    embedded_session_gateway: Option<Router>,
    embedded_realtime_app_state: Option<AppState>,
) -> Router {
    finish_gateway_app_sync(
        config,
        registry,
        product_runtime_router,
        embedded_session_gateway,
        embedded_realtime_app_state,
        RealtimeAuthContextResolver::default(),
    )
}

/// Build the gateway application with IAM database auth wiring from environment.
pub async fn build_app_with_registry_product_runtime_and_embedded_services_from_env(
    config: WebGatewayConfig,
    registry: RouteRegistry,
    product_runtime_router: Option<Router>,
    embedded_session_gateway: Option<Router>,
    embedded_realtime_app_state: Option<AppState>,
) -> Router {
    let iam_pool = resolve_iam_auth_pool_from_env().await;
    finish_gateway_app_from_env(
        config,
        registry,
        product_runtime_router,
        embedded_session_gateway,
        embedded_realtime_app_state,
        RealtimeAuthContextResolver::new(iam_pool),
    )
    .await
}

async fn finish_gateway_app_from_env(
    config: WebGatewayConfig,
    registry: RouteRegistry,
    product_runtime_router: Option<Router>,
    embedded_session_gateway: Option<Router>,
    embedded_realtime_app_state: Option<AppState>,
    realtime_auth: RealtimeAuthContextResolver,
) -> Router {
    let business_router = Router::new()
        .route("/openapi.json", get(openapi_json))
        .route("/openapi/index.json", get(openapi_index_json))
        .route(
            "/openapi/runtime-summary.json",
            get(openapi_runtime_summary_json),
        )
        .route(
            "/openapi/services/{service_schema}",
            get(service_openapi_json),
        )
        .route("/docs", get(docs))
        .route("/docs/services/{service_id}", get(service_docs))
        .route("/", gateway_proxy_routes())
        .route("/{*path}", gateway_proxy_routes())
        .with_state(GatewayState {
            client: build_gateway_upstream_client(),
            config,
            registry,
            product_runtime_router,
            embedded_session_gateway,
            realtime_auth,
            circuit_breakers: CircuitBreakerRegistry::new(
                CircuitBreakerConfig::from_env(),
            ),
        });

    let wrapped_business = crate::web_framework::wrap_gateway_router_from_env(business_router)
        .await
        .layer(build_browser_cors_layer())
        .layer(from_fn_with_state(
            RateLimiter::new(
                RateLimitConfig::from_env(),
            ),
            rate_limit_middleware,
        ));
    mount_embedded_realtime_websocket_router(embedded_realtime_app_state, wrapped_business)
}

fn finish_gateway_app_sync(
    config: WebGatewayConfig,
    registry: RouteRegistry,
    product_runtime_router: Option<Router>,
    embedded_session_gateway: Option<Router>,
    embedded_realtime_app_state: Option<AppState>,
    realtime_auth: RealtimeAuthContextResolver,
) -> Router {
    let business_router = Router::new()
        .route("/openapi.json", get(openapi_json))
        .route("/openapi/index.json", get(openapi_index_json))
        .route(
            "/openapi/runtime-summary.json",
            get(openapi_runtime_summary_json),
        )
        .route(
            "/openapi/services/{service_schema}",
            get(service_openapi_json),
        )
        .route("/docs", get(docs))
        .route("/docs/services/{service_id}", get(service_docs))
        .route("/", gateway_proxy_routes())
        .route("/{*path}", gateway_proxy_routes())
        .with_state(GatewayState {
            client: build_gateway_upstream_client(),
            config,
            registry,
            product_runtime_router,
            embedded_session_gateway,
            realtime_auth,
            circuit_breakers: CircuitBreakerRegistry::new(
                CircuitBreakerConfig::from_env(),
            ),
        });

    let wrapped_business =
        crate::web_framework::wrap_gateway_router(business_router).layer(build_browser_cors_layer())
        .layer(from_fn_with_state(
            RateLimiter::new(
                RateLimitConfig::from_env(),
            ),
            rate_limit_middleware,
        ));
    mount_embedded_realtime_websocket_router(embedded_realtime_app_state, wrapped_business)
}

fn mount_embedded_realtime_websocket_router(
    embedded_realtime_app_state: Option<AppState>,
    business_router: Router,
) -> Router {
    let Some(app_state) = embedded_realtime_app_state else {
        return business_router;
    };
    session_gateway::build_realtime_websocket_router(app_state).merge(business_router)
}
