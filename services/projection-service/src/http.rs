use std::sync::Arc;

use axum::extract::rejection::QueryRejection;
use axum::extract::{DefaultBodyLimit, Extension, Path, Query, State};
use axum::http::{HeaderName, HeaderValue, Request, StatusCode, Uri, header};
use axum::middleware::{self, Next};
use axum::response::{Html, IntoResponse, Response};
use axum::{
    Json, Router,
    routing::{delete, get, post},
};
use im_app_context::AppContext;
use sdkwork_im_api_registry::HttpMethod;
use sdkwork_im_openapi::{
    OpenApiServiceSpec, build_openapi_document, extract_routes_from_function, render_docs_html,
};
use sdkwork_im_web_bootstrap::{im_service_router_config, mount_im_infra_routes};
use sdkwork_routes_web_framework_backend_api::response::{ApiProblem, ApiResult, finish_api_json};
use sdkwork_utils_rust::{
    MAX_LIST_PAGE_SIZE, SDKWORK_TRACE_ID_HEADER, SdkWorkCursorListQuery, SdkWorkProblemDetail,
    SdkWorkResultCode, SdkWorkSeqWindowQuery,
};
use sdkwork_web_core::{
    ProblemCorrelation, WebFrameworkError, WebFrameworkErrorKind, WebRequestContext,
    problem_response,
};
use serde::{Deserialize, Serialize};
use tokio::sync::Semaphore;

use super::{
    ConversationPreferencesView, ConversationProfileView, FavoriteMessageRequest,
    MessageFavoriteView, ProjectionAccessError, ProjectionRuntime, TimelineProjectionService,
    UpdateConversationPreferencesRequest, UpdateConversationProfileRequest,
};

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase", default)]
struct SearchMessagesQuery {
    pub q: Option<String>,
    pub conversation_id: Option<String>,
    #[serde(flatten)]
    pub paging: SdkWorkCursorListQuery,
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct FavoriteMessagesQuery {
    #[serde(flatten)]
    paging: SdkWorkCursorListQuery,
    favorite_type: Option<String>,
    q: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ConversationProfileItemResponse {
    item: ConversationProfileView,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ConversationPreferencesItemResponse {
    item: ConversationPreferencesView,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct MessageFavoriteItemResponse {
    item: MessageFavoriteView,
}

const PROJECTION_MAX_IN_FLIGHT_REQUESTS_ENV: &str = "SDKWORK_IM_PROJECTION_MAX_IN_FLIGHT_REQUESTS";
const PROJECTION_MAX_IN_FLIGHT_REQUESTS_DEFAULT: usize = 1_000;
const PROJECTION_MAX_IN_FLIGHT_REQUESTS_MAX: usize = 50_000;
const PROJECTION_MAX_REQUEST_BODY_BYTES_ENV: &str = "SDKWORK_IM_PROJECTION_MAX_REQUEST_BODY_BYTES";
const PROJECTION_MAX_REQUEST_BODY_BYTES_DEFAULT: usize = 5 * 1024 * 1024;
const PROJECTION_MAX_REQUEST_BODY_BYTES_MAX: usize = 20 * 1024 * 1024;
const FORBIDDEN_PAGINATION_QUERY_ALIASES: &[&str] =
    &["pageSize", "limit", "page_no", "pageNo", "per_page", "size"];

#[derive(Clone)]
struct PublicAppGuardrails {
    request_gate: Arc<Semaphore>,
}

#[derive(Debug)]
pub struct ProjectionApiError {
    status: axum::http::StatusCode,
    #[allow(dead_code)]
    code: &'static str,
    message: String,
}

impl ProjectionApiError {
    fn internal(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            status: axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            code,
            message: message.into(),
        }
    }

    fn from_query_rejection(rejection: QueryRejection) -> Self {
        Self {
            status: rejection.status(),
            code: "invalid_query",
            message: rejection.body_text(),
        }
    }
}

impl From<ProjectionAccessError> for ProjectionApiError {
    fn from(value: ProjectionAccessError) -> Self {
        Self {
            status: value.status(),
            code: value.code(),
            message: value.message().to_owned(),
        }
    }
}

/// Map [`ProjectionApiError::status`] to the canonical [`WebFrameworkErrorKind`].
fn projection_api_error_kind(status: &axum::http::StatusCode) -> WebFrameworkErrorKind {
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

impl From<ProjectionApiError> for ApiProblem {
    fn from(error: ProjectionApiError) -> Self {
        let framework_error = WebFrameworkError {
            kind: projection_api_error_kind(&error.status),
            message: error.message,
            retry_after_seconds: None,
        };
        ApiProblem::from_web_framework(framework_error)
    }
}

impl From<ProjectionAccessError> for ApiProblem {
    fn from(value: ProjectionAccessError) -> Self {
        ProjectionApiError::from(value).into()
    }
}

impl IntoResponse for ProjectionApiError {
    fn into_response(self) -> Response {
        let error = WebFrameworkError {
            kind: projection_api_error_kind(&self.status),
            message: self.message,
            retry_after_seconds: None,
        };
        problem_response(&error, ProblemCorrelation::from(None))
    }
}

fn map_blocking_join_error(error: tokio::task::JoinError) -> ApiProblem {
    ApiProblem::internal_server_error(format!("projection_runtime_blocking_join_failed: {error}"))
}

/// Run in-memory projection reads and writes off the Tokio async worker pool.
///
/// Projection handlers acquire process-local mutexes and may perform synchronous
/// journal/Postgres adapter work. Executing that work on async workers starves
/// the standalone gateway runtime and can wedge unrelated routes such as `/healthz`.
async fn run_blocking_projection<F, T>(
    service: Arc<TimelineProjectionService>,
    auth: AppContext,
    operation: F,
) -> ApiResult<T>
where
    F: FnOnce(Arc<TimelineProjectionService>, AppContext) -> ApiResult<T> + Send + 'static,
    T: Send + 'static,
{
    tokio::task::spawn_blocking(move || operation(service, auth))
        .await
        .map_err(map_blocking_join_error)?
}

fn finish_query_rejection(ctx: &WebRequestContext, rejection: QueryRejection) -> Response {
    finish_api_json(
        ctx,
        Err::<(), ApiProblem>(ProjectionApiError::from_query_rejection(rejection).into()),
    )
}

fn invalid_parameter_response(ctx: &WebRequestContext, detail: impl Into<String>) -> Response {
    let trace_id = ctx.resolved_trace_id();
    let problem = SdkWorkProblemDetail::platform(
        SdkWorkResultCode::InvalidParameter,
        detail,
        trace_id.clone(),
    );
    let status = StatusCode::from_u16(problem.status).unwrap_or(StatusCode::BAD_REQUEST);
    let mut response = (status, Json(problem)).into_response();
    response.headers_mut().insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static("application/problem+json"),
    );
    if let Ok(value) = HeaderValue::from_str(trace_id.as_str()) {
        if let Ok(header_name) = HeaderName::from_bytes(SDKWORK_TRACE_ID_HEADER.as_bytes()) {
            response.headers_mut().insert(header_name, value);
        }
    }
    response
}

fn finish_api_no_content(ctx: &WebRequestContext, result: ApiResult<()>) -> Response {
    match result {
        Ok(()) => {
            let trace_id = ctx.resolved_trace_id();
            let mut response = StatusCode::NO_CONTENT.into_response();
            if let Ok(value) = HeaderValue::from_str(trace_id.as_str()) {
                if let Ok(header_name) = HeaderName::from_bytes(SDKWORK_TRACE_ID_HEADER.as_bytes())
                {
                    response.headers_mut().insert(header_name, value);
                }
            }
            response
        }
        Err(problem) => finish_api_json(ctx, Err::<(), ApiProblem>(problem)),
    }
}

fn query_key(raw_pair: &str) -> &str {
    raw_pair
        .split_once('=')
        .map(|(key, _)| key)
        .unwrap_or(raw_pair)
}

fn raw_query_has_key(query: &str, expected_key: &str) -> bool {
    query
        .split('&')
        .map(query_key)
        .any(|key| key == expected_key)
}

fn forbidden_pagination_alias(query: &str) -> Option<&'static str> {
    FORBIDDEN_PAGINATION_QUERY_ALIASES
        .iter()
        .copied()
        .find(|alias| raw_query_has_key(query, alias))
}

fn reject_non_standard_list_query(ctx: &WebRequestContext, uri: &Uri) -> Option<Response> {
    let query = uri.query()?;
    if let Some(alias) = forbidden_pagination_alias(query) {
        return Some(invalid_parameter_response(
            ctx,
            format!(
                "query parameter `{alias}` is not supported; use canonical `page_size` for list pagination"
            ),
        ));
    }
    if raw_query_has_key(query, "page") && raw_query_has_key(query, "cursor") {
        return Some(invalid_parameter_response(
            ctx,
            "query parameters `page` and `cursor` must not be combined",
        ));
    }
    None
}

pub fn default_projection_service() -> Arc<TimelineProjectionService> {
    default_projection_runtime().service()
}

pub fn default_projection_runtime() -> Arc<ProjectionRuntime> {
    crate::bootstrap::shared_projection_runtime()
}

pub fn build_default_app() -> Router {
    let runtime = default_projection_runtime();
    build_app(runtime.service())
}

pub fn build_supplemental_domain_api_router(service: Arc<TimelineProjectionService>) -> Router {
    Router::new()
        .route("/im/v3/api/chat/contacts", get(get_contacts))
        .route("/im/v3/api/chat/inbox", get(get_inbox))
        .route(
            "/im/v3/api/chat/conversations/{conversation_id}",
            get(get_conversation_summary),
        )
        .route(
            "/im/v3/api/chat/conversations/{conversation_id}/member_directory",
            get(get_member_directory),
        )
        .route(
            "/im/v3/api/chat/conversations/{conversation_id}/pins",
            get(get_pinned_messages),
        )
        .route(
            "/im/v3/api/chat/conversations/{conversation_id}/messages",
            get(get_timeline),
        )
        .route(
            "/im/v3/api/chat/conversations/{conversation_id}/messages/{message_id}/interaction_summary",
            get(get_message_interaction_summary),
        )
        .route(
            "/im/v3/api/chat/conversations/{conversation_id}/profile",
            get(get_conversation_profile).patch(patch_conversation_profile),
        )
        .route(
            "/im/v3/api/chat/conversations/{conversation_id}/preferences",
            get(get_conversation_preferences).patch(patch_conversation_preferences),
        )
        .route(
            "/im/v3/api/chat/messages/search",
            get(search_messages),
        )
        .route(
            "/im/v3/api/chat/messages/favorites",
            get(list_message_favorites),
        )
        .route(
            "/im/v3/api/chat/messages/favorites/{favorite_id}",
            delete(delete_message_favorite),
        )
        .route(
            "/im/v3/api/chat/messages/{message_id}/favorites",
            post(create_message_favorite),
        )
        .route(
            "/im/v3/api/chat/messages/{message_id}/visibility",
            delete(delete_message_visibility),
        )
        .with_state(service)
}

pub fn build_domain_api_router(service: Arc<TimelineProjectionService>) -> Router {
    Router::new()
        .route("/im/v3/api/chat/contacts", get(get_contacts))
        .route("/im/v3/api/chat/inbox", get(get_inbox))
        .route(
            "/im/v3/api/chat/conversations/{conversation_id}",
            get(get_conversation_summary),
        )
        .route(
            "/im/v3/api/chat/conversations/{conversation_id}/read_cursor",
            get(get_read_cursor),
        )
        .route(
            "/im/v3/api/chat/conversations/{conversation_id}/member_directory",
            get(get_member_directory),
        )
        .route(
            "/im/v3/api/chat/conversations/{conversation_id}/pins",
            get(get_pinned_messages),
        )
        .route(
            "/im/v3/api/chat/conversations/{conversation_id}/messages/{message_id}/interaction_summary",
            get(get_message_interaction_summary),
        )
        .route(
            "/im/v3/api/chat/conversations/{conversation_id}/messages",
            get(get_timeline),
        )
        .route(
            "/im/v3/api/chat/conversations/{conversation_id}/profile",
            get(get_conversation_profile).patch(patch_conversation_profile),
        )
        .route(
            "/im/v3/api/chat/conversations/{conversation_id}/preferences",
            get(get_conversation_preferences).patch(patch_conversation_preferences),
        )
        .route(
            "/im/v3/api/chat/messages/search",
            get(search_messages),
        )
        .route(
            "/im/v3/api/chat/messages/favorites",
            get(list_message_favorites),
        )
        .route(
            "/im/v3/api/chat/messages/favorites/{favorite_id}",
            delete(delete_message_favorite),
        )
        .route(
            "/im/v3/api/chat/messages/{message_id}/favorites",
            post(create_message_favorite),
        )
        .route(
            "/im/v3/api/chat/messages/{message_id}/visibility",
            delete(delete_message_visibility),
        )
        .with_state(service)
}

pub fn apply_public_http_guardrails(router: Router) -> Router {
    let guardrails = PublicAppGuardrails {
        request_gate: Arc::new(Semaphore::new(resolve_max_in_flight_requests())),
    };
    router
        .layer(DefaultBodyLimit::max(resolve_max_http_request_body_bytes()))
        .layer(middleware::from_fn_with_state(
            guardrails,
            enforce_in_flight_gate,
        ))
}

pub fn build_public_app() -> Router {
    mount_im_infra_routes(
        apply_public_http_guardrails(build_business_router(
            default_projection_runtime().service(),
        )),
        im_service_router_config(),
    )
}

pub fn build_public_app_with_service(service: Arc<TimelineProjectionService>) -> Router {
    mount_im_infra_routes(
        apply_public_http_guardrails(build_business_router(service)),
        im_service_router_config(),
    )
}

pub fn build_app(service: Arc<TimelineProjectionService>) -> Router {
    mount_im_infra_routes(build_business_router(service), im_service_router_config())
}

/// Integration-test router that resolves dual-token headers into handler extensions.
pub fn build_integration_test_app(service: Arc<TimelineProjectionService>) -> Router {
    use axum::extract::Request;
    use axum::middleware::{Next, from_fn};

    async fn inject_test_auth_context(request: Request, next: Next) -> Response {
        let path = request.uri().path().to_owned();
        let method = request.method().as_str().to_owned();
        if let Ok(resolved) = im_app_context::resolve_app_context_for_request(
            request.headers(),
            path.as_str(),
            method.as_str(),
        ) {
            let mut request = request;
            request
                .extensions_mut()
                .insert(resolved.app_request_context);
            request.extensions_mut().insert(resolved.app_context);
            return next.run(request).await;
        }
        next.run(request).await
    }

    build_app(service).layer(from_fn(inject_test_auth_context))
}

fn build_business_router(service: Arc<TimelineProjectionService>) -> Router {
    Router::new()
        .route("/openapi.json", get(openapi_json))
        .route("/docs", get(docs))
        .merge(build_domain_api_router(service))
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
            return ProjectionApiError {
                status: axum::http::StatusCode::SERVICE_UNAVAILABLE,
                code: "http_overloaded",
                message: "server is at maximum in-flight request capacity, please retry later"
                    .to_owned(),
            }
            .into_response();
        }
    };
    let response = next.run(request).await;
    drop(permit);
    response
}

async fn openapi_json() -> Result<Json<serde_json::Value>, ProjectionApiError> {
    Ok(Json(build_projection_service_openapi_document().map_err(
        |message| ProjectionApiError::internal("openapi_export_failed", message),
    )?))
}

async fn docs() -> Html<String> {
    Html(render_docs_html(&projection_service_openapi_spec()))
}

fn build_projection_service_openapi_document() -> Result<serde_json::Value, String> {
    let http_source = include_str!("http.rs");
    let mut routes = extract_routes_from_function(
        http_source,
        "build_business_router",
        &[],
        &["/openapi.json", "/docs"],
    )?;
    routes.extend(extract_routes_from_function(
        http_source,
        "build_domain_api_router",
        &[],
        &[],
    )?);

    Ok(build_openapi_document(
        &projection_service_openapi_spec(),
        &routes,
        projection_service_tag,
        projection_service_requires_app_context,
        projection_service_summary,
    ))
}

fn projection_service_openapi_spec() -> OpenApiServiceSpec<'static> {
    OpenApiServiceSpec {
        title: "Sdkwork IM Projection Service API",
        version: env!("CARGO_PKG_VERSION"),
        description: "Live OpenAPI contract generated from the projection-service router for inbox, timeline, message search, contacts, read cursor, and interaction summary queries.",
        openapi_path: "/openapi.json",
        docs_path: "/docs",
    }
}

fn projection_service_tag(path: &str, _method: HttpMethod) -> String {
    match path {
        "/healthz" | "/readyz" => "system".to_owned(),
        "/im/v3/api/chat/contacts" => "contacts".to_owned(),
        "/im/v3/api/chat/inbox" => "inbox".to_owned(),
        _ => "conversations".to_owned(),
    }
}

fn projection_service_requires_app_context(path: &str, _method: HttpMethod) -> bool {
    !matches!(path, "/healthz" | "/readyz")
}

fn projection_service_summary(path: &str, method: HttpMethod) -> String {
    match (path, method) {
        ("/healthz", HttpMethod::Get) => "Check projection service health".to_owned(),
        ("/readyz", HttpMethod::Get) => "Check projection service readiness".to_owned(),
        _ => format!(
            "{} {}",
            projection_service_method_display(method),
            path.trim_matches('/').replace('/', " ")
        ),
    }
}

fn projection_service_method_display(method: HttpMethod) -> &'static str {
    match method {
        HttpMethod::Delete => "Delete",
        HttpMethod::Get => "Get",
        HttpMethod::Head => "Head",
        HttpMethod::Options => "Options",
        HttpMethod::Patch => "Patch",
        HttpMethod::Post => "Post",
        HttpMethod::Put => "Put",
    }
}

fn validate_list_page_size(page_size: Option<i32>) -> Result<Option<usize>, SdkWorkResultCode> {
    match page_size {
        Some(value) if !(1..=MAX_LIST_PAGE_SIZE).contains(&value) => {
            Err(SdkWorkResultCode::InvalidParameter)
        }
        Some(value) => Ok(Some(value as usize)),
        None => Ok(None),
    }
}

fn resolve_list_page_size(
    ctx: &WebRequestContext,
    page_size: Option<i32>,
) -> Result<Option<usize>, Response> {
    validate_list_page_size(page_size)
        .map_err(|_| invalid_parameter_response(ctx, "list pagination parameters are invalid"))
}

async fn search_messages(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    uri: Uri,
    query: Result<Query<SearchMessagesQuery>, QueryRejection>,
    State(service): State<Arc<TimelineProjectionService>>,
) -> Response {
    if let Some(response) = reject_non_standard_list_query(&ctx, &uri) {
        return response;
    }
    let Query(query) = match query {
        Ok(value) => value,
        Err(rejection) => return finish_query_rejection(&ctx, rejection),
    };
    let search_query = query.q.clone();
    let conversation_id = query
        .conversation_id
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_owned);
    let page_size = match resolve_list_page_size(&ctx, query.paging.page_size) {
        Ok(value) => value,
        Err(response) => return response,
    };
    let cursor = query.paging.cursor.clone();
    let result = run_blocking_projection(service, auth, move |service, auth| {
        Ok(service.search_messages_from_auth_context(
            &auth,
            search_query.as_deref().unwrap_or_default(),
            conversation_id.as_deref(),
            page_size,
            cursor.as_deref(),
        )?)
    })
    .await;
    finish_api_json(&ctx, result)
}

async fn get_timeline(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    Path(conversation_id): Path<String>,
    uri: Uri,
    query: Result<Query<SdkWorkSeqWindowQuery>, QueryRejection>,
    State(service): State<Arc<TimelineProjectionService>>,
) -> Response {
    if let Some(response) = reject_non_standard_list_query(&ctx, &uri) {
        return response;
    }
    let Query(query) = match query {
        Ok(value) => value,
        Err(rejection) => return finish_query_rejection(&ctx, rejection),
    };
    let after_seq = query.after_seq;
    let page_size = match resolve_list_page_size(&ctx, query.page_size) {
        Ok(value) => value,
        Err(response) => return response,
    };
    let result = run_blocking_projection(service, auth, move |service, auth| {
        Ok(service.timeline_window_from_auth_context(
            &auth,
            conversation_id.as_str(),
            after_seq,
            page_size,
        )?)
    })
    .await;
    finish_api_json(&ctx, result)
}

async fn get_inbox(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    uri: Uri,
    query: Result<Query<SdkWorkCursorListQuery>, QueryRejection>,
    State(service): State<Arc<TimelineProjectionService>>,
) -> Response {
    if let Some(response) = reject_non_standard_list_query(&ctx, &uri) {
        return response;
    }
    let Query(query) = match query {
        Ok(value) => value,
        Err(rejection) => return finish_query_rejection(&ctx, rejection),
    };
    let page_size = match resolve_list_page_size(&ctx, query.page_size) {
        Ok(value) => value,
        Err(response) => return response,
    };
    let cursor = query.cursor.clone();
    let result = run_blocking_projection(service, auth, move |service, auth| {
        Ok(service.inbox_window_from_auth_context(&auth, page_size, cursor.as_deref())?)
    })
    .await;
    finish_api_json(&ctx, result)
}

async fn get_contacts(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    uri: Uri,
    query: Result<Query<SdkWorkCursorListQuery>, QueryRejection>,
    State(service): State<Arc<TimelineProjectionService>>,
) -> Response {
    if let Some(response) = reject_non_standard_list_query(&ctx, &uri) {
        return response;
    }
    let Query(query) = match query {
        Ok(value) => value,
        Err(rejection) => return finish_query_rejection(&ctx, rejection),
    };
    let page_size = match resolve_list_page_size(&ctx, query.page_size) {
        Ok(value) => value,
        Err(response) => return response,
    };
    let cursor = query.cursor.clone();
    let result = run_blocking_projection(service, auth, move |service, auth| {
        Ok(service.contact_window_from_auth_context(&auth, page_size, cursor.as_deref())?)
    })
    .await;
    finish_api_json(&ctx, result)
}

async fn get_conversation_summary(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    Path(conversation_id): Path<String>,
    State(service): State<Arc<TimelineProjectionService>>,
) -> Response {
    let result = run_blocking_projection(service, auth, move |service, auth| {
        service
            .conversation_summary_from_auth_context(&auth, conversation_id.as_str())?
            .ok_or_else(|| {
                ProjectionApiError {
                    status: axum::http::StatusCode::NOT_FOUND,
                    code: "conversation_summary_not_found",
                    message: format!("conversation summary not found: {conversation_id}"),
                }
                .into()
            })
    })
    .await;
    finish_api_json(&ctx, result)
}

async fn get_read_cursor(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    Path(conversation_id): Path<String>,
    State(service): State<Arc<TimelineProjectionService>>,
) -> Response {
    let result = run_blocking_projection(service, auth, move |service, auth| {
        service
            .read_cursor_from_auth_context(&auth, conversation_id.as_str())?
            .ok_or_else(|| {
                ProjectionApiError {
                    status: axum::http::StatusCode::NOT_FOUND,
                    code: "conversation_read_cursor_not_found",
                    message: format!("conversation read cursor not found: {conversation_id}"),
                }
                .into()
            })
    })
    .await;
    finish_api_json(&ctx, result)
}

async fn get_member_directory(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    Path(conversation_id): Path<String>,
    uri: Uri,
    query: Result<Query<SdkWorkCursorListQuery>, QueryRejection>,
    State(service): State<Arc<TimelineProjectionService>>,
) -> Response {
    if let Some(response) = reject_non_standard_list_query(&ctx, &uri) {
        return response;
    }
    let Query(query) = match query {
        Ok(value) => value,
        Err(rejection) => return finish_query_rejection(&ctx, rejection),
    };
    let page_size = match resolve_list_page_size(&ctx, query.page_size) {
        Ok(value) => value,
        Err(response) => return response,
    };
    let cursor = query.cursor.clone();
    let result = run_blocking_projection(service, auth, move |service, auth| {
        Ok(service.member_directory_window_from_auth_context(
            &auth,
            conversation_id.as_str(),
            page_size,
            cursor.as_deref(),
        )?)
    })
    .await;
    finish_api_json(&ctx, result)
}

async fn get_pinned_messages(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    Path(conversation_id): Path<String>,
    uri: Uri,
    query: Result<Query<SdkWorkCursorListQuery>, QueryRejection>,
    State(service): State<Arc<TimelineProjectionService>>,
) -> Response {
    if let Some(response) = reject_non_standard_list_query(&ctx, &uri) {
        return response;
    }
    let Query(query) = match query {
        Ok(value) => value,
        Err(rejection) => return finish_query_rejection(&ctx, rejection),
    };
    let page_size = match resolve_list_page_size(&ctx, query.page_size) {
        Ok(value) => value,
        Err(response) => return response,
    };
    let cursor = query.cursor.clone();
    let result = run_blocking_projection(service, auth, move |service, auth| {
        Ok(service.pinned_messages_window_from_auth_context(
            &auth,
            conversation_id.as_str(),
            page_size,
            cursor.as_deref(),
        )?)
    })
    .await;
    finish_api_json(&ctx, result)
}

async fn get_message_interaction_summary(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    Path((conversation_id, message_id)): Path<(String, String)>,
    State(service): State<Arc<TimelineProjectionService>>,
) -> Response {
    let result = run_blocking_projection(service, auth, move |service, auth| {
        service
            .message_interaction_summary_from_auth_context(
                &auth,
                conversation_id.as_str(),
                message_id.as_str(),
            )?
            .ok_or_else(|| {
                ProjectionApiError {
                    status: axum::http::StatusCode::NOT_FOUND,
                    code: "message_interaction_summary_not_found",
                    message: format!(
                        "message interaction summary not found: {conversation_id}/{message_id}"
                    ),
                }
                .into()
            })
    })
    .await;
    finish_api_json(&ctx, result)
}

async fn get_conversation_profile(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    Path(conversation_id): Path<String>,
    State(service): State<Arc<TimelineProjectionService>>,
) -> Response {
    let result = run_blocking_projection(service, auth, move |service, auth| {
        Ok(ConversationProfileItemResponse {
            item: service
                .conversation_profile_from_auth_context(&auth, conversation_id.as_str())?,
        })
    })
    .await;
    finish_api_json(&ctx, result)
}

async fn patch_conversation_profile(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    Path(conversation_id): Path<String>,
    State(service): State<Arc<TimelineProjectionService>>,
    Json(body): Json<UpdateConversationProfileRequest>,
) -> Response {
    let result = run_blocking_projection(service, auth, move |service, auth| {
        Ok(ConversationProfileItemResponse {
            item: service.update_conversation_profile_from_auth_context(
                &auth,
                conversation_id.as_str(),
                body,
            )?,
        })
    })
    .await;
    finish_api_json(&ctx, result)
}

async fn get_conversation_preferences(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    Path(conversation_id): Path<String>,
    State(service): State<Arc<TimelineProjectionService>>,
) -> Response {
    let result = run_blocking_projection(service, auth, move |service, auth| {
        Ok(ConversationPreferencesItemResponse {
            item: service
                .conversation_preferences_from_auth_context(&auth, conversation_id.as_str())?,
        })
    })
    .await;
    finish_api_json(&ctx, result)
}

async fn patch_conversation_preferences(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    Path(conversation_id): Path<String>,
    State(service): State<Arc<TimelineProjectionService>>,
    Json(body): Json<UpdateConversationPreferencesRequest>,
) -> Response {
    let result = run_blocking_projection(service, auth, move |service, auth| {
        Ok(ConversationPreferencesItemResponse {
            item: service.update_conversation_preferences_from_auth_context(
                &auth,
                conversation_id.as_str(),
                body,
            )?,
        })
    })
    .await;
    finish_api_json(&ctx, result)
}

async fn list_message_favorites(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    uri: Uri,
    query: Result<Query<FavoriteMessagesQuery>, QueryRejection>,
    State(service): State<Arc<TimelineProjectionService>>,
) -> Response {
    if let Some(response) = reject_non_standard_list_query(&ctx, &uri) {
        return response;
    }
    let Query(query) = match query {
        Ok(value) => value,
        Err(rejection) => return finish_query_rejection(&ctx, rejection),
    };
    let paging = match query.paging.resolve() {
        Ok(value) => value,
        Err(error) => {
            let detail = match error {
                SdkWorkResultCode::InvalidParameter => {
                    "list pagination parameters are invalid".to_owned()
                }
                other => format!("list pagination failed: {other:?}"),
            };
            return invalid_parameter_response(&ctx, detail);
        }
    };
    let cursor = query.paging.cursor.clone();
    let favorite_type = query.favorite_type.clone();
    let search_query = query.q.clone();
    let result = run_blocking_projection(service, auth, move |service, auth| {
        Ok(service.message_favorites_window_from_auth_context(
            &auth,
            Some(paging.page_size),
            cursor.as_deref(),
            favorite_type.as_deref(),
            search_query.as_deref(),
        )?)
    })
    .await;
    finish_api_json(&ctx, result)
}

async fn create_message_favorite(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    Path(message_id): Path<String>,
    State(service): State<Arc<TimelineProjectionService>>,
    Json(body): Json<FavoriteMessageRequest>,
) -> Response {
    let result = run_blocking_projection(service, auth, move |service, auth| {
        Ok(MessageFavoriteItemResponse {
            item: service.create_message_favorite_from_auth_context(
                &auth,
                message_id.as_str(),
                body,
            )?,
        })
    })
    .await;
    finish_api_json(&ctx, result)
}

async fn delete_message_favorite(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    Path(favorite_id): Path<String>,
    State(service): State<Arc<TimelineProjectionService>>,
) -> Response {
    let result = run_blocking_projection(service, auth, move |service, auth| {
        Ok(service.delete_message_favorite_from_auth_context(&auth, favorite_id.as_str())?)
    })
    .await;
    finish_api_json(&ctx, result)
}

async fn delete_message_visibility(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    Path(message_id): Path<String>,
    State(service): State<Arc<TimelineProjectionService>>,
) -> Response {
    let runtime = default_projection_runtime();
    let result = run_blocking_projection(service, auth, move |service, auth| {
        service.delete_message_visibility_from_auth_context(&auth, message_id.as_str())?;
        if let Err(error) = runtime.persist_durable_state() {
            if runtime.requires_durable_persist() {
                return Err(ProjectionApiError::internal(
                    "message_visibility_persist_failed",
                    format!("failed to persist message visibility snapshot: {error}"),
                )
                .into());
            }
            tracing::warn!(
                error = %error,
                "failed to persist message visibility snapshot after delete; in-memory state remains authoritative until next durable flush"
            );
        }
        Ok(())
    })
    .await;
    finish_api_no_content(&ctx, result)
}

fn resolve_max_in_flight_requests() -> usize {
    std::env::var(PROJECTION_MAX_IN_FLIGHT_REQUESTS_ENV)
        .ok()
        .and_then(|v| v.parse::<usize>().ok())
        .filter(|&parsed| parsed > 0)
        .unwrap_or(PROJECTION_MAX_IN_FLIGHT_REQUESTS_DEFAULT)
        .min(PROJECTION_MAX_IN_FLIGHT_REQUESTS_MAX)
}

fn resolve_max_http_request_body_bytes() -> usize {
    std::env::var(PROJECTION_MAX_REQUEST_BODY_BYTES_ENV)
        .ok()
        .and_then(|v| v.parse::<usize>().ok())
        .filter(|&parsed| parsed > 0)
        .unwrap_or(PROJECTION_MAX_REQUEST_BODY_BYTES_DEFAULT)
        .min(PROJECTION_MAX_REQUEST_BODY_BYTES_MAX)
}
