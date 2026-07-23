//! Sdkwork IM HTTP service bootstrap through `sdkwork-web-framework`.
//!
//! Upstream IM services that own `/im/v3/api/*` (and optional `/backend/v3/api/*`) routes
//! must mount the standard interceptor chain instead of the legacy `im-app-context`
//! middleware.

use std::sync::{Arc, OnceLock};

use axum::Router;
use sdkwork_web_bootstrap::{ServiceRouterConfig, service_router};

/// Mount canonical infrastructure probes on an IM HTTP service router.
pub fn mount_im_infra_routes(router: Router, config: ServiceRouterConfig) -> Router {
    service_router(router, config)
}

/// Standard infra router config for IM cloud HTTP service processes.
pub fn im_service_router_config() -> ServiceRouterConfig {
    ServiceRouterConfig::default()
        .with_readiness_check(sdkwork_im_service_readiness::im_env_readiness_check())
        .with_metrics(im_service_http_metrics())
}
use im_app_context::{
    AppContext, app_context_from_web_request, resolve_app_context,
    resolve_web_environment_from_process_env,
};
use sdkwork_iam_web_adapter::{
    IamAuthorizationPolicy, IamWebRequestContextResolver, iam_web_request_context_resolver_from_env,
};
use sdkwork_im_realtime_api_paths::REALTIME_WS;
use sdkwork_web_axum::{WebFrameworkLayer, with_web_request_context};
use sdkwork_web_bootstrap::SecurityPolicy;
use sdkwork_web_core::{
    DomainContextInjector, EnforcePrincipalTenantIsolationPolicy, HttpMetricsDimensions,
    HttpMetricsRegistry, HttpRouteManifest, WebApiSurface, WebEnvironment, WebRequestContext,
    WebRequestContextProfile,
};

static SHARED_IAM_WEB_REQUEST_CONTEXT_RESOLVER: OnceLock<IamWebRequestContextResolver> =
    OnceLock::new();

#[derive(Clone, Default)]
struct ImAppContextInjector;

static IM_HTTP_METRICS: OnceLock<Arc<HttpMetricsRegistry>> = OnceLock::new();

/// Shared HTTP metrics registry for IM service processes (`OBSERVABILITY_SPEC.md` §3).
pub fn im_service_http_metrics() -> Arc<HttpMetricsRegistry> {
    IM_HTTP_METRICS
        .get_or_init(|| {
            let environment = resolve_web_environment_from_process_env();
            let service = std::env::var("SDKWORK_IM_SERVICE_NAME")
                .or_else(|_| std::env::var("OTEL_SERVICE_NAME"))
                .ok()
                .filter(|value| !value.trim().is_empty())
                .unwrap_or_else(|| "sdkwork-im-service".to_owned());
            let deployment_profile = std::env::var("SDKWORK_IM_DEPLOYMENT_PROFILE")
                .unwrap_or_else(|_| "standalone".to_owned());
            HttpMetricsRegistry::with_dimensions(
                HttpMetricsDimensions::from_profile_environment(environment)
                    .with_service(service)
                    .with_deployment_profile(deployment_profile)
                    .with_runtime_target("server"),
            )
        })
        .clone()
}

impl DomainContextInjector for ImAppContextInjector {
    fn inject(&self, request: &mut axum::extract::Request, context: &WebRequestContext) {
        request.extensions_mut().insert(context.clone());
        if let Some(app_context) = im_app_context_from_framework_request(request, context) {
            request.extensions_mut().insert(app_context);
        }
    }
}

fn im_app_context_from_framework_request(
    request: &axum::extract::Request,
    context: &WebRequestContext,
) -> Option<AppContext> {
    let mut projected = app_context_from_web_request(context)?;
    if !matches!(context.api_surface, WebApiSurface::OpenApi) {
        return Some(projected);
    }

    // The standard principal does not yet model IM's delegated `agent` actor
    // kind. Preserve that open-api domain claim only after the dual-token
    // parser has verified it and every standard identity dimension agrees with
    // WebRequestContext. Tenant, organization, user, session, app, and scopes
    // always remain framework-owned. App-api/backend-api never enter this
    // compatibility context.
    let Ok(delegated) = resolve_app_context(request.headers()) else {
        return Some(projected);
    };
    let principal = context.principal()?;
    let same_standard_identity = delegated.tenant_id == principal.tenant_id()
        && active_organization_id(Some(delegated.organization_id.as_str()))
            == active_organization_id(principal.organization_id())
        && delegated.user_id == principal.user_id()
        && delegated.session_id.as_deref() == principal.session_id()
        && delegated.app_id.as_deref() == Some(principal.app_id());
    if same_standard_identity {
        projected.actor_id = delegated.actor_id;
        projected.actor_kind = delegated.actor_kind;
        projected.device_id = delegated.device_id;
    }
    Some(projected)
}

fn active_organization_id(value: Option<&str>) -> Option<&str> {
    match value.map(str::trim) {
        None | Some("") | Some("0") => None,
        Some(value) => Some(value),
    }
}

fn im_service_security_policy(environment: &WebEnvironment) -> SecurityPolicy {
    let mut security_policy = if matches!(environment, WebEnvironment::Dev | WebEnvironment::Test) {
        SecurityPolicy::default()
    } else {
        SecurityPolicy::production()
    };
    if matches!(environment, WebEnvironment::Dev | WebEnvironment::Test) {
        security_policy.cors = sdkwork_web_core::CorsPolicy::development_private_network();
        security_policy
            .cross_site
            .reject_untrusted_state_changing_origins = false;
        security_policy.cross_site.reject_cookie_auth_without_origin = false;
    }
    security_policy
}

/// Infra paths that stay anonymous across IM HTTP service processes.
pub fn im_service_public_path_prefixes() -> Vec<String> {
    let mut prefixes = sdkwork_web_bootstrap::infra_public_path_prefixes();
    prefixes.extend([
        "/openapi.json".to_owned(),
        "/openapi/".to_owned(),
        "/docs".to_owned(),
        REALTIME_WS.to_owned(),
    ]);
    prefixes
}

/// Cached IAM resolver for IM HTTP service processes (shared across route crates in one process).
pub async fn shared_iam_web_request_context_resolver_from_env() -> IamWebRequestContextResolver {
    if let Some(resolver) = SHARED_IAM_WEB_REQUEST_CONTEXT_RESOLVER.get() {
        return resolver.clone();
    }
    let resolver = iam_web_request_context_resolver_from_env().await;
    let _ = SHARED_IAM_WEB_REQUEST_CONTEXT_RESOLVER.set(resolver.clone());
    resolver
}

/// Returns the cached IAM resolver when [`shared_iam_web_request_context_resolver_from_env`] has run.
pub fn cached_iam_web_request_context_resolver() -> Option<IamWebRequestContextResolver> {
    SHARED_IAM_WEB_REQUEST_CONTEXT_RESOLVER.get().cloned()
}
/// Profile for IM-owned open-api ingress (`/im/v3/api/*`) with default backend-api prefix.
pub fn im_service_context_profile() -> WebRequestContextProfile {
    WebRequestContextProfile {
        open_api_prefixes: vec!["/im/v3/api".to_owned()],
        public_path_prefixes: im_service_public_path_prefixes(),
        gateway_api_prefixes: Vec::new(),
        environment: resolve_web_environment_from_process_env(),
        ..WebRequestContextProfile::default()
    }
}

fn wrap_im_open_api_service_router_inner(
    resolver: IamWebRequestContextResolver,
    route_manifest: HttpRouteManifest,
    router: Router,
) -> Router {
    let environment = resolve_web_environment_from_process_env();
    let security_policy = im_service_security_policy(&environment);
    let authorization_policy = Arc::new(IamAuthorizationPolicy::new(route_manifest));
    let tenant_isolation_policy = Arc::new(EnforcePrincipalTenantIsolationPolicy);
    let layer = WebFrameworkLayer::new(resolver)
        .with_profile(im_service_context_profile())
        .with_security_policy(security_policy)
        .with_route_manifest(route_manifest)
        .with_authorization_policy(authorization_policy)
        .with_tenant_isolation_policy(tenant_isolation_policy)
        .with_domain_injector(Arc::new(ImAppContextInjector))
        .with_metrics(im_service_http_metrics());
    with_web_request_context(router, layer)
}

/// Initialize structured logging and optional OTel export for IM HTTP service processes.
pub fn init_im_service_tracing_from_env() {
    sdkwork_web_bootstrap::init_tracing_from_env();
}

/// Wrap an IM HTTP service router with the canonical SDKWork interceptor pipeline.
///
/// Uses an empty [`HttpRouteManifest`], so route-level authorization and metrics
/// dimensions are not enforced. Prefer [`wrap_im_service_router_with_manifest`]
/// (or [`wrap_im_open_api_service_router_with_resolver`]) when the route crate
/// owns a manifest.
pub fn wrap_im_open_api_service_router(router: Router) -> Router {
    let resolver = cached_iam_web_request_context_resolver()
        .unwrap_or_else(|| IamWebRequestContextResolver::new(None));
    wrap_im_open_api_service_router_with_resolver(resolver, HttpRouteManifest::new(&[]), router)
}

/// Alias for IM HTTP service processes (open-api and backend-api prefixes).
///
/// Equivalent to [`wrap_im_open_api_service_router`] and retained for
/// backwards compatibility with route crates that have not yet migrated to
/// passing their route manifest. New code should call
/// [`wrap_im_service_router_with_manifest`] instead so that
/// [`IamAuthorizationPolicy`] and route-level metrics dimensions are derived
/// from the actual route table.
pub fn wrap_im_service_router(router: Router) -> Router {
    wrap_im_open_api_service_router(router)
}

/// Wrap an IM HTTP service router with the canonical SDKWork interceptor
/// pipeline and an explicit [`HttpRouteManifest`].
///
/// Route crates that own a manifest (see `manifest::route_manifest()`) should
/// call this function instead of [`wrap_im_service_router`] so that the
/// [`WebFrameworkLayer`] receives the real route table. This enables
/// route-level authorization (`IamAuthorizationPolicy`), HTTP metrics
/// dimensions keyed by `route_id`, and OpenAPI metadata consistency per
/// `API_SPEC.md` §4.5, §14, and §15.
///
/// Uses the cached IAM resolver when [`shared_iam_web_request_context_resolver_from_env`]
/// has been invoked (typical for co-located gateways and bin services
/// bootstrapped through `sdkwork_im_web_bootstrap`). Falls back to a default
/// resolver with no IAM database when the cache has not been populated, which
/// is the same behaviour as [`wrap_im_service_router`].
pub fn wrap_im_service_router_with_manifest(
    router: Router,
    route_manifest: HttpRouteManifest,
) -> Router {
    let resolver = cached_iam_web_request_context_resolver()
        .unwrap_or_else(|| IamWebRequestContextResolver::new(None));
    wrap_im_open_api_service_router_with_resolver(resolver, route_manifest, router)
}

/// Wrap with an explicit resolver and route manifest (public routes from manifest + infra prefixes).
pub fn wrap_im_open_api_service_router_with_resolver(
    resolver: IamWebRequestContextResolver,
    route_manifest: HttpRouteManifest,
    router: Router,
) -> Router {
    wrap_im_open_api_service_router_inner(resolver, route_manifest, router)
}

/// Bootstrap from environment (cloud service processes with IAM database lookup).
pub async fn wrap_im_open_api_service_router_from_env(
    route_manifest: HttpRouteManifest,
    router: Router,
) -> Router {
    let resolver = shared_iam_web_request_context_resolver_from_env().await;
    wrap_im_open_api_service_router_inner(resolver, route_manifest, router)
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use sdkwork_web_core::{
        ServerRequestId, WebApiSurface, WebAuthLevel, WebAuthMode, WebDeploymentMode,
        WebEnvironment, WebLoginScope, WebRequestPrincipal, WebSubjectType, WebTransportFacts,
    };

    fn organization_web_request_context() -> WebRequestContext {
        WebRequestContext {
            request_id: ServerRequestId("request-web-context-authority".into()),
            api_surface: WebApiSurface::AppApi,
            auth_mode: WebAuthMode::DualToken,
            principal: Some(
                WebRequestPrincipal::builder()
                    .tenant_id("100001")
                    .organization_id(Some("200001".to_owned()))
                    .login_scope(WebLoginScope::Organization)
                    .user_id("42")
                    .session_id(Some("session-42".to_owned()))
                    .app_id("sdkwork-im-pc")
                    .environment(WebEnvironment::Test)
                    .deployment_mode(WebDeploymentMode::Saas)
                    .auth_level(WebAuthLevel::Password)
                    .data_scope(vec!["organization".to_owned()])
                    .permission_scope(vec!["conversation.read".to_owned()])
                    .subject_type(WebSubjectType::User)
                    .build(),
            ),
            transport: WebTransportFacts {
                path: "/app/v3/api/chat/conversations/1/knowledgebase".into(),
                method: "GET".into(),
                auth_token_present: true,
                access_token_present: true,
                api_key_present: false,
                oauth_bearer_present: false,
                agent_token_present: false,
            },
            locale: None,
            client_kind: None,
            operation: None,
            trace_id: Some("trace-web-context-authority".into()),
            idempotency_key: None,
        }
    }

    #[test]
    fn domain_injection_uses_only_the_framework_resolved_principal() {
        let context = organization_web_request_context();
        let tenant_wide_headers = im_app_context::build_dual_token_headers_for_context(
            &im_app_context::local_service_app_context(
                "100001",
                "42",
                "user",
                None,
                ["conversation.read"],
            ),
            ["conversation.read"],
        );
        let mut request = axum::extract::Request::builder()
            .uri("/app/v3/api/chat/conversations/1/knowledgebase")
            .body(Body::empty())
            .expect("test request");
        *request.headers_mut() = tenant_wide_headers;

        ImAppContextInjector.inject(&mut request, &context);

        let projected = request
            .extensions()
            .get::<im_app_context::AppContext>()
            .expect("framework principal should project into IM AppContext");
        assert_eq!(projected.tenant_id, "100001");
        assert_eq!(projected.organization_id, "200001");
        assert_eq!(projected.user_id, "42");
        assert_eq!(projected.session_id.as_deref(), Some("session-42"));
    }
}
