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
    SdkWorkResultCode,
};
use sdkwork_web_core::{
    ProblemCorrelation, WebFrameworkError, WebFrameworkErrorKind, WebRequestContext,
    problem_response,
};
use serde::{Deserialize, Serialize};
use tokio::sync::Semaphore;

use super::{
    ConversationPreferencesView, ConversationProfileView, ConversationStateAccessError,
    ConversationStateRuntime, ConversationStateService, FavoriteMessageRequest,
    MessageFavoriteView, UpdateConversationPreferencesRequest, UpdateConversationProfileRequest,
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

#[derive(Debug, Deserialize, Default)]
struct InboxQuery {
    #[serde(flatten)]
    paging: SdkWorkCursorListQuery,
    conversation_type: Option<String>,
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

const CONVERSATION_STATE_MAX_IN_FLIGHT_REQUESTS_ENV: &str =
    "SDKWORK_IM_CONVERSATION_STATE_MAX_IN_FLIGHT_REQUESTS";
const CONVERSATION_STATE_MAX_IN_FLIGHT_REQUESTS_DEFAULT: usize = 1_000;
const CONVERSATION_STATE_MAX_IN_FLIGHT_REQUESTS_MAX: usize = 50_000;
const CONVERSATION_STATE_MAX_REQUEST_BODY_BYTES_ENV: &str =
    "SDKWORK_IM_CONVERSATION_STATE_MAX_REQUEST_BODY_BYTES";
const CONVERSATION_STATE_MAX_REQUEST_BODY_BYTES_DEFAULT: usize = 5 * 1024 * 1024;
const CONVERSATION_STATE_MAX_REQUEST_BODY_BYTES_MAX: usize = 20 * 1024 * 1024;
const FORBIDDEN_PAGINATION_QUERY_ALIASES: &[&str] =
    &["pageSize", "limit", "page_no", "pageNo", "per_page", "size"];

#[derive(Clone)]
struct PublicAppGuardrails {
    request_gate: Arc<Semaphore>,
}

#[derive(Debug)]
pub struct ConversationStateApiError {
    status: axum::http::StatusCode,
    #[allow(dead_code)]
    code: &'static str,
    message: String,
}

impl ConversationStateApiError {
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

impl From<ConversationStateAccessError> for ConversationStateApiError {
    fn from(value: ConversationStateAccessError) -> Self {
        Self {
            status: value.status(),
            code: value.code(),
            message: value.message().to_owned(),
        }
    }
}

/// Map [`ConversationStateApiError::status`] to the canonical [`WebFrameworkErrorKind`].
fn conversation_state_api_error_kind(status: &axum::http::StatusCode) -> WebFrameworkErrorKind {
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

impl From<ConversationStateApiError> for ApiProblem {
    fn from(error: ConversationStateApiError) -> Self {
        let framework_error = WebFrameworkError {
            kind: conversation_state_api_error_kind(&error.status),
            message: error.message,
            retry_after_seconds: None,
            auth_profile: None,
            failed_stage: None,
            reason: None,
        };
        ApiProblem::from_web_framework(framework_error)
    }
}

impl From<ConversationStateAccessError> for ApiProblem {
    fn from(value: ConversationStateAccessError) -> Self {
        ConversationStateApiError::from(value).into()
    }
}

impl IntoResponse for ConversationStateApiError {
    fn into_response(self) -> Response {
        let error = WebFrameworkError {
            kind: conversation_state_api_error_kind(&self.status),
            message: self.message,
            retry_after_seconds: None,
            auth_profile: None,
            failed_stage: None,
            reason: None,
        };
        problem_response(&error, ProblemCorrelation::from(None))
    }
}

fn map_blocking_join_error(error: tokio::task::JoinError) -> ApiProblem {
    ApiProblem::internal_server_error(format!(
        "conversation_state_runtime_blocking_join_failed: {error}"
    ))
}

/// Run in-memory conversation_state reads and writes off the Tokio async worker pool.
///
/// ConversationState handlers acquire process-local mutexes and may perform synchronous
/// journal/Postgres adapter work. Executing that work on async workers starves
/// the standalone gateway runtime and can wedge unrelated routes such as `/healthz`.
async fn run_blocking_conversation_state<F, T>(
    service: Arc<ConversationStateService>,
    auth: AppContext,
    operation: F,
) -> ApiResult<T>
where
    F: FnOnce(Arc<ConversationStateService>, AppContext) -> ApiResult<T> + Send + 'static,
    T: Send + 'static,
{
    tokio::task::spawn_blocking(move || operation(service, auth))
        .await
        .map_err(map_blocking_join_error)?
}

fn finish_query_rejection(ctx: &WebRequestContext, rejection: QueryRejection) -> Response {
    finish_api_json(
        ctx,
        Err::<(), ApiProblem>(ConversationStateApiError::from_query_rejection(rejection).into()),
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
    if let Ok(value) = HeaderValue::from_str(trace_id.as_str())
        && let Ok(header_name) = HeaderName::from_bytes(SDKWORK_TRACE_ID_HEADER.as_bytes())
    {
        response.headers_mut().insert(header_name, value);
    }
    response
}

fn finish_api_no_content(ctx: &WebRequestContext, result: ApiResult<()>) -> Response {
    match result {
        Ok(()) => {
            let trace_id = ctx.resolved_trace_id();
            let mut response = StatusCode::NO_CONTENT.into_response();
            if let Ok(value) = HeaderValue::from_str(trace_id.as_str())
                && let Ok(header_name) = HeaderName::from_bytes(SDKWORK_TRACE_ID_HEADER.as_bytes())
            {
                response.headers_mut().insert(header_name, value);
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

pub fn default_conversation_state_service() -> Arc<ConversationStateService> {
    default_conversation_state_runtime().service()
}

pub fn default_conversation_state_runtime() -> Arc<ConversationStateRuntime> {
    crate::conversation_state::bootstrap::shared_conversation_state_runtime()
}

pub fn build_default_app() -> Router {
    let runtime = default_conversation_state_runtime();
    build_app(runtime.service())
}

/// Conversation-owned query routes that complement the command/history router.
///
/// These routes remain under the Conversation service boundary. Social contact
/// inventory is intentionally excluded and is served by `social-service`.
pub fn build_conversation_query_api_router(service: Arc<ConversationStateService>) -> Router {
    Router::new()
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

pub fn build_domain_api_router(service: Arc<ConversationStateService>) -> Router {
    Router::new()
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
            default_conversation_state_runtime().service(),
        )),
        im_service_router_config(),
    )
}

pub fn build_public_app_with_service(service: Arc<ConversationStateService>) -> Router {
    mount_im_infra_routes(
        apply_public_http_guardrails(build_business_router(service)),
        im_service_router_config(),
    )
}

pub fn build_app(service: Arc<ConversationStateService>) -> Router {
    mount_im_infra_routes(build_business_router(service), im_service_router_config())
}

/// Integration-test router that resolves dual-token headers into handler extensions.
pub fn build_integration_test_app(service: Arc<ConversationStateService>) -> Router {
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
                .insert(resolved.web_request_context);
            request.extensions_mut().insert(resolved.app_context);
            return next.run(request).await;
        }
        next.run(request).await
    }

    build_app(service).layer(from_fn(inject_test_auth_context))
}

fn build_business_router(service: Arc<ConversationStateService>) -> Router {
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
            return ConversationStateApiError {
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

async fn openapi_json() -> Result<Json<serde_json::Value>, ConversationStateApiError> {
    Ok(Json(
        build_conversation_state_service_openapi_document().map_err(|message| {
            ConversationStateApiError::internal("openapi_export_failed", message)
        })?,
    ))
}

async fn docs() -> Html<String> {
    Html(render_docs_html(&conversation_state_service_openapi_spec()))
}

fn build_conversation_state_service_openapi_document() -> Result<serde_json::Value, String> {
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
        &conversation_state_service_openapi_spec(),
        &routes,
        conversation_state_service_tag,
        conversation_state_service_requires_app_context,
        conversation_state_service_summary,
    ))
}

fn conversation_state_service_openapi_spec() -> OpenApiServiceSpec<'static> {
    OpenApiServiceSpec {
        title: "Sdkwork IM ConversationState Service API",
        version: env!("CARGO_PKG_VERSION"),
        description: "Live OpenAPI contract generated from the Conversation state router for inbox, conversation summaries, read cursor, message search, and interaction summary queries.",
        openapi_path: "/openapi.json",
        docs_path: "/docs",
    }
}

fn conversation_state_service_tag(path: &str, _method: HttpMethod) -> String {
    match path {
        "/healthz" | "/readyz" => "system".to_owned(),
        "/im/v3/api/chat/inbox" => "inbox".to_owned(),
        _ => "conversations".to_owned(),
    }
}

fn conversation_state_service_requires_app_context(path: &str, _method: HttpMethod) -> bool {
    !matches!(path, "/healthz" | "/readyz")
}

fn conversation_state_service_summary(path: &str, method: HttpMethod) -> String {
    match (path, method) {
        ("/healthz", HttpMethod::Get) => "Check conversation_state service health".to_owned(),
        ("/readyz", HttpMethod::Get) => "Check conversation_state service readiness".to_owned(),
        _ => format!(
            "{} {}",
            conversation_state_service_method_display(method),
            path.trim_matches('/').replace('/', " ")
        ),
    }
}

fn conversation_state_service_method_display(method: HttpMethod) -> &'static str {
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

fn resolve_list_page_size(page_size: Option<i32>) -> Result<Option<usize>, SdkWorkResultCode> {
    validate_list_page_size(page_size)
}

async fn search_messages(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    uri: Uri,
    query: Result<Query<SearchMessagesQuery>, QueryRejection>,
    State(service): State<Arc<ConversationStateService>>,
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
    let page_size = match resolve_list_page_size(query.paging.page_size) {
        Ok(value) => value,
        Err(_) => {
            return invalid_parameter_response(&ctx, "list pagination parameters are invalid");
        }
    };
    let cursor = query.paging.cursor.clone();
    let result = run_blocking_conversation_state(service, auth, move |service, auth| {
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

async fn get_inbox(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    uri: Uri,
    query: Result<Query<InboxQuery>, QueryRejection>,
    State(service): State<Arc<ConversationStateService>>,
) -> Response {
    if let Some(response) = reject_non_standard_list_query(&ctx, &uri) {
        return response;
    }
    let Query(query) = match query {
        Ok(value) => value,
        Err(rejection) => return finish_query_rejection(&ctx, rejection),
    };
    let page_size = match resolve_list_page_size(query.paging.page_size) {
        Ok(value) => value,
        Err(_) => {
            return invalid_parameter_response(&ctx, "list pagination parameters are invalid");
        }
    };
    let cursor = query.paging.cursor.clone();
    let conversation_type = query.conversation_type.clone();
    let search_query = query.q.clone();
    let result = run_blocking_conversation_state(service, auth, move |service, auth| {
        Ok(service.inbox_window_from_auth_context_filtered(
            &auth,
            page_size,
            cursor.as_deref(),
            conversation_type.as_deref(),
            search_query.as_deref(),
        )?)
    })
    .await;
    finish_api_json(&ctx, result)
}

async fn get_conversation_summary(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    Path(conversation_id): Path<String>,
    State(service): State<Arc<ConversationStateService>>,
) -> Response {
    let result = run_blocking_conversation_state(service, auth, move |service, auth| {
        service
            .conversation_summary_from_auth_context(&auth, conversation_id.as_str())?
            .ok_or_else(|| {
                ConversationStateApiError {
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
    State(service): State<Arc<ConversationStateService>>,
) -> Response {
    let result = run_blocking_conversation_state(service, auth, move |service, auth| {
        service
            .read_cursor_from_auth_context(&auth, conversation_id.as_str())?
            .ok_or_else(|| {
                ConversationStateApiError {
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
    State(service): State<Arc<ConversationStateService>>,
) -> Response {
    if let Some(response) = reject_non_standard_list_query(&ctx, &uri) {
        return response;
    }
    let Query(query) = match query {
        Ok(value) => value,
        Err(rejection) => return finish_query_rejection(&ctx, rejection),
    };
    let page_size = match resolve_list_page_size(query.page_size) {
        Ok(value) => value,
        Err(_) => {
            return invalid_parameter_response(&ctx, "list pagination parameters are invalid");
        }
    };
    let cursor = query.cursor.clone();
    let result = run_blocking_conversation_state(service, auth, move |service, auth| {
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
    State(service): State<Arc<ConversationStateService>>,
) -> Response {
    if let Some(response) = reject_non_standard_list_query(&ctx, &uri) {
        return response;
    }
    let Query(query) = match query {
        Ok(value) => value,
        Err(rejection) => return finish_query_rejection(&ctx, rejection),
    };
    let page_size = match resolve_list_page_size(query.page_size) {
        Ok(value) => value,
        Err(_) => {
            return invalid_parameter_response(&ctx, "list pagination parameters are invalid");
        }
    };
    let cursor = query.cursor.clone();
    let result = run_blocking_conversation_state(service, auth, move |service, auth| {
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
    State(service): State<Arc<ConversationStateService>>,
) -> Response {
    let result = run_blocking_conversation_state(service, auth, move |service, auth| {
        service
            .message_interaction_summary_from_auth_context(
                &auth,
                conversation_id.as_str(),
                message_id.as_str(),
            )?
            .ok_or_else(|| {
                ConversationStateApiError {
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
    State(service): State<Arc<ConversationStateService>>,
) -> Response {
    let result = run_blocking_conversation_state(service, auth, move |service, auth| {
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
    State(service): State<Arc<ConversationStateService>>,
    Json(body): Json<UpdateConversationProfileRequest>,
) -> Response {
    let result = run_blocking_conversation_state(service, auth, move |service, auth| {
        let profile = service.update_conversation_profile_from_auth_context(
            &auth,
            conversation_id.as_str(),
            body,
        )?;
        let organization_id = im_platform_contracts::normalize_realtime_organization_id(
            auth.organization_id.as_str(),
        );
        service
            .enqueue_conversation_profile_updated(
                auth.tenant_id.as_str(),
                organization_id.as_str(),
                &profile,
            )
            .map_err(ConversationStateAccessError::from)?;
        Ok(ConversationProfileItemResponse { item: profile })
    })
    .await;
    finish_api_json(&ctx, result)
}

async fn get_conversation_preferences(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    Path(conversation_id): Path<String>,
    State(service): State<Arc<ConversationStateService>>,
) -> Response {
    let result = run_blocking_conversation_state(service, auth, move |service, auth| {
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
    State(service): State<Arc<ConversationStateService>>,
    Json(body): Json<UpdateConversationPreferencesRequest>,
) -> Response {
    let result = run_blocking_conversation_state(service, auth, move |service, auth| {
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
    State(service): State<Arc<ConversationStateService>>,
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
    let result = run_blocking_conversation_state(service, auth, move |service, auth| {
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
    State(service): State<Arc<ConversationStateService>>,
    Json(body): Json<FavoriteMessageRequest>,
) -> Response {
    let result = run_blocking_conversation_state(service, auth, move |service, auth| {
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
    State(service): State<Arc<ConversationStateService>>,
) -> Response {
    let result = run_blocking_conversation_state(service, auth, move |service, auth| {
        Ok(service.delete_message_favorite_from_auth_context(&auth, favorite_id.as_str())?)
    })
    .await;
    finish_api_json(&ctx, result)
}

async fn delete_message_visibility(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    Path(message_id): Path<String>,
    State(service): State<Arc<ConversationStateService>>,
) -> Response {
    let result = run_blocking_conversation_state(service, auth, move |service, auth| {
        service.delete_message_visibility_from_auth_context(&auth, message_id.as_str())?;
        Ok(())
    })
    .await;
    finish_api_no_content(&ctx, result)
}

fn resolve_max_in_flight_requests() -> usize {
    std::env::var(CONVERSATION_STATE_MAX_IN_FLIGHT_REQUESTS_ENV)
        .ok()
        .and_then(|v| v.parse::<usize>().ok())
        .filter(|&parsed| parsed > 0)
        .unwrap_or(CONVERSATION_STATE_MAX_IN_FLIGHT_REQUESTS_DEFAULT)
        .min(CONVERSATION_STATE_MAX_IN_FLIGHT_REQUESTS_MAX)
}

fn resolve_max_http_request_body_bytes() -> usize {
    std::env::var(CONVERSATION_STATE_MAX_REQUEST_BODY_BYTES_ENV)
        .ok()
        .and_then(|v| v.parse::<usize>().ok())
        .filter(|&parsed| parsed > 0)
        .unwrap_or(CONVERSATION_STATE_MAX_REQUEST_BODY_BYTES_DEFAULT)
        .min(CONVERSATION_STATE_MAX_REQUEST_BODY_BYTES_MAX)
}
