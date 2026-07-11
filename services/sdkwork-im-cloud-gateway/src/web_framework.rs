use std::sync::Arc;

use axum::Router;
use axum::http::{StatusCode, header};
use axum::middleware::from_fn_with_state;
use axum::response::IntoResponse;
use axum::routing::get;
use im_app_context::resolve_web_environment_from_process_env;
use sdkwork_iam_web_adapter::{IamAuthorizationPolicy, IamWebRequestContextResolver};
use sdkwork_im_realtime_api_paths::REALTIME_WS;
use sdkwork_im_web_bootstrap::shared_iam_web_request_context_resolver_from_env;
use sdkwork_web_axum::with_web_request_context;
use sdkwork_web_bootstrap::{
    HttpMethod, HttpRoute, HttpRouteManifest, ReadinessCheck, SecurityPolicy, ServiceRouterConfig,
    WebEnvironment, WebFramework, WebFrameworkBuilder, WebRequestContextProfile, service_router,
};
use sdkwork_web_core::{
    EnforcePrincipalTenantIsolationPolicy, HttpMetricsRegistry, WebFrameworkOptionalFeatures,
};
use sdkwork_web_store_sqlx::{
    connect_and_bootstrap_webstore_database_from_env, shared_audit_emitter,
    shared_idempotency_store, shared_rate_limit_store, shared_security_event_emitter,
};

use crate::constants::{
    GATEWAY_REQUEST_TIMEOUT_SECS_DEFAULT, GATEWAY_REQUEST_TIMEOUT_SECS_ENV,
    GATEWAY_REQUEST_TIMEOUT_SECS_MAX,
};
use crate::gateway_protection::{
    TenantRateLimitConfig, TenantRateLimiter, per_tenant_rate_limit_middleware,
};

const IM_APP_API_PREFIX: &str = "/im/v3/api";

/// Resolve the gateway-level HTTP request timeout from env, clamped to
/// `[1, GATEWAY_REQUEST_TIMEOUT_SECS_MAX]`. Defaults to
/// `GATEWAY_REQUEST_TIMEOUT_SECS_DEFAULT` (90s) which exceeds the upstream
/// proxy timeout (60s) to allow upstream responses to complete and return
/// through the gateway interceptor pipeline.
fn resolve_gateway_request_timeout() -> std::time::Duration {
    let secs = std::env::var(GATEWAY_REQUEST_TIMEOUT_SECS_ENV)
        .ok()
        .and_then(|v| v.parse::<u64>().ok())
        .filter(|&v| v > 0)
        .unwrap_or(GATEWAY_REQUEST_TIMEOUT_SECS_DEFAULT)
        .min(GATEWAY_REQUEST_TIMEOUT_SECS_MAX);
    std::time::Duration::from_secs(secs)
}

const GATEWAY_PUBLIC_ROUTES: &[HttpRoute] = &[HttpRoute::public(
    HttpMethod::Get,
    REALTIME_WS,
    "realtime",
    "realtime.websocket.upgrade",
)];

fn gateway_security_policy(environment: &WebEnvironment) -> SecurityPolicy {
    let mut security_policy = if matches!(environment, WebEnvironment::Dev | WebEnvironment::Test) {
        SecurityPolicy::default()
    } else {
        SecurityPolicy::production()
    };
    if matches!(environment, WebEnvironment::Dev | WebEnvironment::Test) {
        security_policy.cors.allow_all_origins = true;
        security_policy
            .cross_site
            .reject_untrusted_state_changing_origins = false;
        security_policy.cross_site.reject_cookie_auth_without_origin = false;
    }
    security_policy
}

fn gateway_optional_features(
    environment: &WebEnvironment,
    sync_dev_assembly: bool,
) -> WebFrameworkOptionalFeatures {
    if sync_dev_assembly || matches!(environment, WebEnvironment::Dev | WebEnvironment::Test) {
        WebFrameworkOptionalFeatures::default()
    } else {
        WebFrameworkOptionalFeatures::production_sqlx().control_plane_standalone()
    }
}

async fn configure_production_framework_builder<R>(
    builder: WebFrameworkBuilder<R>,
) -> WebFrameworkBuilder<R>
where
    R: sdkwork_web_core::WebRequestContextResolver + Clone + std::any::Any,
{
    let Ok(host) = connect_and_bootstrap_webstore_database_from_env().await else {
        return builder;
    };
    let Some(sqlite) = host.pool().as_sqlite().cloned() else {
        return builder;
    };

    builder
        .audit_emitter(shared_audit_emitter(sqlite.clone()))
        .security_event_emitter(shared_security_event_emitter(sqlite.clone()))
        .idempotency_store(shared_idempotency_store(sqlite.clone()))
        .rate_limit_store(shared_rate_limit_store(sqlite))
}

fn wrap_gateway_router_with_resolver(
    router: Router,
    resolver: IamWebRequestContextResolver,
    readiness: Option<Arc<dyn ReadinessCheck>>,
    sync_dev_assembly: bool,
) -> Router {
    let environment = if sync_dev_assembly {
        WebEnvironment::Test
    } else {
        resolve_web_environment_from_process_env()
    };
    let route_manifest = HttpRouteManifest::new(GATEWAY_PUBLIC_ROUTES);
    let security_policy = gateway_security_policy(&environment);
    let optional_features = gateway_optional_features(&environment, sync_dev_assembly);
    let profile = WebRequestContextProfile {
        gateway_api_prefixes: vec![IM_APP_API_PREFIX.to_owned()],
        public_path_prefixes: vec![
            "/health".to_owned(),
            "/healthz".to_owned(),
            "/readyz".to_owned(),
            "/metrics".to_owned(),
            "/openapi.json".to_owned(),
            "/openapi/".to_owned(),
            "/docs".to_owned(),
            "/app/v3/api/auth/".to_owned(),
            "/app/v3/api/oauth/".to_owned(),
            // Realtime websocket upgrades authenticate through the first `auth.init` frame
            // (see docs/架构/20-WebSocket实时传输绑定标准.md), not HTTP Authorization.
            REALTIME_WS.to_owned(),
            // Registry-owned websocket proxy routes (RouteProtocol::Websocket).
            "/ws/".to_owned(),
            "/admin/".to_owned(),
            "/api/".to_owned(),
            "/".to_owned(),
        ],
        environment,
        ..WebRequestContextProfile::default()
    };
    let mut framework_builder = WebFramework::builder(resolver)
        .profile(profile)
        .security_policy(security_policy)
        .route_manifest(route_manifest)
        .authorization_policy(Arc::new(IamAuthorizationPolicy::new(route_manifest)))
        .tenant_isolation_policy(Arc::new(EnforcePrincipalTenantIsolationPolicy))
        .optional_features(optional_features)
        .request_timeout(resolve_gateway_request_timeout());
    if let Some(check) = readiness {
        framework_builder = framework_builder.readiness_check(check);
    }
    let framework = framework_builder.build();
    let metrics = framework.metrics().clone();
    service_router_with_runtime_metrics(
        with_web_request_context(router, framework.layer().clone()).layer(from_fn_with_state(
            TenantRateLimiter::new(TenantRateLimitConfig::from_env()),
            per_tenant_rate_limit_middleware,
        )),
        framework.service_router_config(),
        metrics,
    )
}

async fn wrap_gateway_router_with_resolver_from_env(
    router: Router,
    resolver: IamWebRequestContextResolver,
    readiness: Arc<dyn ReadinessCheck>,
) -> Router {
    let environment = resolve_web_environment_from_process_env();
    let production_assembly = matches!(environment, WebEnvironment::Prod);
    let route_manifest = HttpRouteManifest::new(GATEWAY_PUBLIC_ROUTES);
    let security_policy = gateway_security_policy(&environment);
    let optional_features = gateway_optional_features(&environment, false);
    let profile = WebRequestContextProfile {
        gateway_api_prefixes: vec![IM_APP_API_PREFIX.to_owned()],
        public_path_prefixes: vec![
            "/health".to_owned(),
            "/healthz".to_owned(),
            "/readyz".to_owned(),
            "/metrics".to_owned(),
            "/openapi.json".to_owned(),
            "/openapi/".to_owned(),
            "/docs".to_owned(),
            "/app/v3/api/auth/".to_owned(),
            "/app/v3/api/oauth/".to_owned(),
            REALTIME_WS.to_owned(),
            "/ws/".to_owned(),
            "/admin/".to_owned(),
            "/api/".to_owned(),
            "/".to_owned(),
        ],
        environment,
        ..WebRequestContextProfile::default()
    };
    let mut framework_builder = WebFramework::builder(resolver)
        .profile(profile)
        .security_policy(security_policy)
        .route_manifest(route_manifest)
        .authorization_policy(Arc::new(IamAuthorizationPolicy::new(route_manifest)))
        .tenant_isolation_policy(Arc::new(EnforcePrincipalTenantIsolationPolicy))
        .optional_features(optional_features)
        .request_timeout(resolve_gateway_request_timeout())
        .readiness_check(readiness);
    if production_assembly {
        framework_builder = configure_production_framework_builder(framework_builder).await;
    }
    let framework = framework_builder.build();
    let metrics = framework.metrics().clone();
    service_router_with_runtime_metrics(
        with_web_request_context(router, framework.layer().clone()).layer(from_fn_with_state(
            TenantRateLimiter::new(TenantRateLimitConfig::from_env()),
            per_tenant_rate_limit_middleware,
        )),
        framework.service_router_config(),
        metrics,
    )
}

/// Wrap the product gateway router with the canonical SDKWork HTTP interceptor pipeline.
///
/// Health and readiness are owned by `sdkwork-web-bootstrap::service_router`; `/metrics`
/// preserves its HTTP registry and appends bounded IM runtime gauges and counters.
pub fn wrap_gateway_router(router: Router) -> Router {
    wrap_gateway_router_with_resolver(
        router,
        IamWebRequestContextResolver::new(None),
        Some(Arc::new(sdkwork_web_bootstrap::AlwaysReady)),
        true,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::Request;
    use conversation_runtime::{
        ConversationCommitJournal, ConversationRuntime, InMemoryJournal,
        register_embedded_conversation_runtime,
    };
    use http_body_util::BodyExt;
    use tower::ServiceExt;

    #[tokio::test]
    async fn metrics_endpoint_includes_http_and_conversation_runtime_metrics() {
        register_embedded_conversation_runtime(Arc::new(ConversationRuntime::new(
            ConversationCommitJournal::Memory(InMemoryJournal::default()),
        )));

        let response = wrap_gateway_router(Router::new())
            .oneshot(
                Request::builder()
                    .uri("/metrics")
                    .body(Body::empty())
                    .expect("metrics request should build"),
            )
            .await
            .expect("metrics request should complete");
        assert_eq!(response.status(), axum::http::StatusCode::OK);
        let body = response
            .into_body()
            .collect()
            .await
            .expect("metrics body should collect")
            .to_bytes();
        let body = String::from_utf8(body.to_vec()).expect("metrics body should be UTF-8");

        assert!(body.contains("sdkwork_http_requests_total"));
        assert!(body.contains("im_conversation_runtime_entries"));
        assert!(body.contains("deployment_profile=\"standalone\""));
        assert!(body.contains("runtime_target=\"server\""));
    }
}

async fn im_gateway_metrics_handler(metrics: Arc<HttpMetricsRegistry>) -> impl IntoResponse {
    let mut output = metrics.render_prometheus();
    if let Some(runtime) = conversation_runtime::resolve_embedded_conversation_runtime() {
        let dimensions = metrics.dimensions();
        output.push('\n');
        output.push_str(&runtime.render_runtime_metrics_prometheus(
            dimensions.service.as_str(),
            dimensions.environment.as_str(),
            dimensions.deployment_profile.as_str(),
            dimensions.runtime_target.as_str(),
        ));
    }
    (
        StatusCode::OK,
        [(
            header::CONTENT_TYPE,
            "text/plain; version=0.0.4; charset=utf-8",
        )],
        output,
    )
}

fn service_router_with_runtime_metrics(
    router: Router,
    config: ServiceRouterConfig,
    metrics: Arc<HttpMetricsRegistry>,
) -> Router {
    service_router(router, config.skip_metrics()).route(
        "/metrics",
        get({
            move || {
                let metrics = metrics.clone();
                async move { im_gateway_metrics_handler(metrics).await }
            }
        }),
    )
}

/// Wrap the gateway router using IAM database dual-token verification when configured.
pub async fn wrap_gateway_router_from_env(router: Router) -> Router {
    let resolver = shared_iam_web_request_context_resolver_from_env().await;
    let readiness = sdkwork_im_service_readiness::resolve_gateway_readiness_check().await;
    wrap_gateway_router_with_resolver_from_env(router, resolver, readiness).await
}
