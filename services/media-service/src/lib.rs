use std::sync::Arc;

use axum::extract::{DefaultBodyLimit, Extension, State};
use axum::http::Request;
use axum::middleware::{self, Next};
use axum::response::{Html, IntoResponse, Response};
use axum::{Json, Router, routing::get};
use im_app_context::AppContext;
use im_platform_contracts::ProviderHealthSnapshot;
use im_time::utc_now_rfc3339_millis;
use sdkwork_im_api_registry::HttpMethod;
use sdkwork_im_openapi::{
    OpenApiServiceSpec, build_openapi_document, extract_routes_from_function, render_docs_html,
};
use sdkwork_im_web_bootstrap::{
    im_service_router_config, mount_im_infra_routes,
};
use sdkwork_routes_web_framework_backend_api::response::{ApiProblem, ApiResult, finish_api_json};
use sdkwork_web_core::{
    WebFrameworkError, WebFrameworkErrorKind, WebRequestContext, problem_response,
    ProblemCorrelation,
};
use tokio::sync::Semaphore;

const MEDIA_MAX_IN_FLIGHT_REQUESTS_ENV: &str = "SDKWORK_IM_MEDIA_MAX_IN_FLIGHT_REQUESTS";
const MEDIA_MAX_IN_FLIGHT_REQUESTS_DEFAULT: usize = 1_000;
const MEDIA_MAX_IN_FLIGHT_REQUESTS_MAX: usize = 20_000;
const MEDIA_MAX_REQUEST_BODY_BYTES_ENV: &str = "SDKWORK_IM_MEDIA_MAX_REQUEST_BODY_BYTES";
const MEDIA_MAX_REQUEST_BODY_BYTES_DEFAULT: usize = 256 * 1024;
const MEDIA_MAX_REQUEST_BODY_BYTES_MAX: usize = 1024 * 1024;

#[derive(Clone)]
pub struct AppState {
    runtime: Arc<MediaRuntime>,
}

#[derive(Clone)]
struct PublicAppGuardrails {
    request_gate: Arc<Semaphore>,
}

pub struct MediaRuntime;

impl Default for MediaRuntime {
    fn default() -> Self {
        Self::new()
    }
}

impl MediaRuntime {
    pub fn new() -> Self {
        Self
    }

    pub fn provider_health_snapshot(
        &self,
        _tenant_id: &str,
    ) -> Result<ProviderHealthSnapshot, MediaError> {
        let mut snapshot =
            ProviderHealthSnapshot::healthy("sdkwork-drive", utc_now_rfc3339_millis());
        snapshot
            .details
            .insert("storageAuthority".into(), "sdkwork-drive".into());
        snapshot
            .details
            .insert("mediaResourceRole".into(), "usage-structure-only".into());
        snapshot
            .details
            .insert("driveReference".into(), "drive_uri".into());
        snapshot
            .details
            .insert("uploadLifecycle".into(), "delegated-to-drive".into());
        Ok(snapshot)
    }
}

#[derive(Debug, Clone)]
pub struct MediaError {
    status: axum::http::StatusCode,
    code: &'static str,
    message: String,
}

impl MediaError {
    fn internal(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            status: axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            code,
            message: message.into(),
        }
    }

    fn payload_too_large(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            status: axum::http::StatusCode::PAYLOAD_TOO_LARGE,
            code,
            message: message.into(),
        }
    }

    pub fn status(&self) -> axum::http::StatusCode {
        self.status
    }

    pub fn code(&self) -> &'static str {
        self.code
    }

    pub fn message(&self) -> &str {
        self.message.as_str()
    }
}

/// Map [`MediaError::status`] to the canonical [`WebFrameworkErrorKind`].
fn media_error_kind(status: &axum::http::StatusCode) -> WebFrameworkErrorKind {
    use axum::http::StatusCode;
    match *status {
        StatusCode::BAD_REQUEST => WebFrameworkErrorKind::BadRequest,
        StatusCode::UNAUTHORIZED => WebFrameworkErrorKind::MissingCredentials,
        StatusCode::FORBIDDEN => WebFrameworkErrorKind::Forbidden,
        StatusCode::NOT_FOUND => WebFrameworkErrorKind::NotFound,
        StatusCode::CONFLICT => WebFrameworkErrorKind::Conflict,
        StatusCode::PAYLOAD_TOO_LARGE => WebFrameworkErrorKind::PayloadTooLarge,
        StatusCode::SERVICE_UNAVAILABLE => WebFrameworkErrorKind::DependencyUnavailable,
        StatusCode::NOT_IMPLEMENTED => WebFrameworkErrorKind::NotImplemented,
        _ => WebFrameworkErrorKind::InternalServerError,
    }
}

impl From<MediaError> for ApiProblem {
    fn from(error: MediaError) -> Self {
        let framework_error = WebFrameworkError {
            kind: media_error_kind(&error.status),
            message: error.message,
            retry_after_seconds: None,
        };
        ApiProblem::from_web_framework(framework_error)
    }
}

/// Fallback `IntoResponse` for contexts where [`WebRequestContext`] is not
/// available (e.g. middleware without context injection). Produces a
/// `application/problem+json` response without `traceId`.
impl IntoResponse for MediaError {
    fn into_response(self) -> Response {
        let error = WebFrameworkError {
            kind: media_error_kind(&self.status),
            message: self.message,
            retry_after_seconds: None,
        };
        problem_response(&error, ProblemCorrelation::from(None))
    }
}

pub fn default_app_state() -> AppState {
    AppState {
        runtime: Arc::new(MediaRuntime::new()),
    }
}

pub fn build_default_app() -> Router {
    build_app(Arc::new(MediaRuntime::new()))
}

pub fn apply_public_http_guardrails(router: Router) -> Router {
    let guardrails = PublicAppGuardrails {
        request_gate: Arc::new(Semaphore::new(resolve_usize_env_with_upper_bound(
            MEDIA_MAX_IN_FLIGHT_REQUESTS_ENV,
            MEDIA_MAX_IN_FLIGHT_REQUESTS_DEFAULT,
            MEDIA_MAX_IN_FLIGHT_REQUESTS_MAX,
        ))),
    };
    let body_limit = resolve_usize_env_with_upper_bound(
        MEDIA_MAX_REQUEST_BODY_BYTES_ENV,
        MEDIA_MAX_REQUEST_BODY_BYTES_DEFAULT,
        MEDIA_MAX_REQUEST_BODY_BYTES_MAX,
    );
    router
        .layer(DefaultBodyLimit::max(body_limit))
        .layer(middleware::from_fn_with_state(
            guardrails,
            enforce_in_flight_gate,
        ))
}

pub fn build_domain_api_router(state: AppState) -> Router {
    Router::new()
        .route(
            "/app/v3/api/media/provider_health",
            get(get_media_provider_health),
        )
        .with_state(state)
}

pub fn build_app(runtime: Arc<MediaRuntime>) -> Router {
    mount_im_infra_routes(
        build_business_router(runtime),
        im_service_router_config(),
    )
}

fn build_business_router(runtime: Arc<MediaRuntime>) -> Router {
    let state = AppState { runtime };
    Router::new()
        .route("/openapi.json", get(openapi_json))
        .route("/docs", get(docs_html))
        .merge(build_domain_api_router(state))
}

pub fn build_public_app() -> Router {
    mount_im_infra_routes(
        apply_public_http_guardrails(build_business_router(Arc::new(MediaRuntime::new()))),
        im_service_router_config(),
    )
}

async fn openapi_json() -> Result<Json<serde_json::Value>, MediaError> {
    Ok(Json(build_media_service_openapi_document().map_err(
        |message| MediaError::internal("openapi_export_failed", message),
    )?))
}

async fn docs_html() -> Html<String> {
    Html(render_docs_html(&media_service_openapi_spec()))
}

fn build_media_service_openapi_document() -> Result<serde_json::Value, String> {
    // Extract routes from `build_domain_api_router` (not `build_business_router`)
    // because the business router only `.merge()`s the domain router; the
    // extractor follows direct `.route()` calls and does not recurse into
    // merged sub-routers. The domain router owns the media endpoint.
    let routes = extract_routes_from_function(
        include_str!("lib.rs"),
        "build_domain_api_router",
        &[],
        &["/openapi.json", "/docs"],
    )?;
    Ok(build_openapi_document(
        &media_service_openapi_spec(),
        &routes,
        classify_media_route_tag,
        classify_media_route_security,
        summarize_media_route,
    ))
}

fn media_service_openapi_spec() -> OpenApiServiceSpec<'static> {
    OpenApiServiceSpec {
        title: "Sdkwork IM Media Reference API",
        version: env!("CARGO_PKG_VERSION"),
        description: "Drive-backed media reference and health endpoints. Upload, download grant, object metadata, provider, and storage lifecycle operations are owned by SDKWork Drive.",
        openapi_path: "/openapi.json",
        docs_path: "/docs",
    }
}

fn classify_media_route_tag(path: &str, _method: HttpMethod) -> String {
    if path.starts_with("/app/v3/api/media") {
        "media".into()
    } else {
        "ops".into()
    }
}

fn classify_media_route_security(path: &str, _method: HttpMethod) -> bool {
    path.starts_with("/app/v3/api/")
}

fn summarize_media_route(path: &str, _method: HttpMethod) -> String {
    match path {
        "/app/v3/api/media/provider_health" => {
            "Inspect Drive-backed media reference provider health".into()
        }
        "/healthz" => "Media reference service liveness".into(),
        "/readyz" => "Media reference service readiness".into(),
        "/openapi.json" => "Export media reference OpenAPI schema".into(),
        "/docs" => "Render media reference API documentation".into(),
        _ => format!("Media reference route {path}"),
    }
}

pub async fn get_media_provider_health(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
) -> Response {
    let result: ApiResult<ProviderHealthSnapshot> = (|| {
        Ok(state
            .runtime
            .provider_health_snapshot(auth.tenant_id.as_str())?)
    })();
    finish_api_json(&ctx, result)
}

async fn enforce_in_flight_gate(
    State(guardrails): State<PublicAppGuardrails>,
    request: Request<axum::body::Body>,
    next: Next,
) -> Response {
    if matches!(
        request.uri().path(),
        "/healthz" | "/readyz" | "/livez" | "/metrics" | "/openapi.json" | "/docs"
    ) {
        return next.run(request).await;
    }
    let permit = match guardrails.request_gate.clone().try_acquire_owned() {
        Ok(permit) => permit,
        Err(_) => {
            let problem = ApiProblem::dependency_unavailable(
                "server is at maximum in-flight request capacity, please retry later",
            );
            if let Some(ctx) = request.extensions().get::<WebRequestContext>() {
                return problem.into_response_for(ctx);
            }
            return MediaError {
                status: axum::http::StatusCode::SERVICE_UNAVAILABLE,
                code: "http_overloaded",
                message:
                    "server is at maximum in-flight request capacity, please retry later".to_owned(),
            }
            .into_response();
        }
    };
    let response = next.run(request).await;
    drop(permit);
    response
}

fn resolve_usize_env_with_upper_bound(name: &str, default: usize, max: usize) -> usize {
    std::env::var(name)
        .ok()
        .and_then(|value| value.trim().parse::<usize>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(default)
        .clamp(1, max)
}

#[allow(dead_code)]
fn reject_oversized_payload(bytes: usize, limit: usize) -> Result<(), MediaError> {
    if bytes > limit {
        return Err(MediaError::payload_too_large(
            "media_payload_too_large",
            "media reference payload exceeds configured limit",
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_openapi_document_excludes_drive_owned_lifecycle_paths() {
        let document =
            build_media_service_openapi_document().expect("openapi document should build");
        let paths = document["paths"].as_object().expect("paths object");
        for forbidden in [
            "/im/v3/api/media/uploads",
            "/im/v3/api/media/uploads/{mediaReferenceId}/complete",
            "/im/v3/api/media/{mediaReferenceId}",
            "/im/v3/api/media/{mediaReferenceId}/download_url",
        ] {
            assert!(!paths.contains_key(forbidden));
        }
        assert!(paths.contains_key("/app/v3/api/media/provider_health"));
    }
}
