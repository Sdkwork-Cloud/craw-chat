use std::sync::{Arc, OnceLock};

use axum::extract::{DefaultBodyLimit, Extension, Path, Query, State};
use axum::http::Request;
use axum::middleware::{self, Next};
use axum::response::{Html, IntoResponse, Response};
use axum::{Json, Router, routing::{delete, get, post}};
use im_app_context::AppContext;
use im_domain_core::conversation::ConversationReadCursorView;
use sdkwork_im_api_registry::HttpMethod;
use sdkwork_im_openapi::{
    OpenApiServiceSpec, build_openapi_document, extract_routes_from_function, render_docs_html,
};
use sdkwork_im_web_bootstrap::{im_service_router_config, mount_im_infra_routes};
use sdkwork_routes_web_framework_backend_api::response::{ApiProblem, ApiResult, finish_api_json};
use sdkwork_web_core::{
    WebFrameworkError, WebFrameworkErrorKind, WebRequestContext, problem_response, ProblemCorrelation,
};
use serde::{Deserialize, Serialize};
use tokio::sync::Semaphore;

use super::{
    ContactWindowView, ConversationMemberDirectoryEntry, ConversationPreferencesView,
    ConversationProfileView, ConversationSummaryView, DeleteMessageFavoriteResponse,
    FavoriteMessageRequest, FavoriteMessagesWindowView, InboxWindowView,
    MessageFavoriteView, MessageInteractionSummaryView, ProjectionAccessError, ProjectionRuntime, TimelineProjectionService,
    TimelineWindowView, UpdateConversationPreferencesRequest, UpdateConversationProfileRequest,
};

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct TimelineQuery {
    after_seq: Option<u64>,
    limit: Option<usize>,
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct ListQuery {
    limit: Option<usize>,
    cursor: Option<String>,
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct FavoriteMessagesQuery {
    limit: Option<usize>,
    cursor: Option<String>,
    favorite_type: Option<String>,
    q: Option<String>,
}

type TimelineResponse = TimelineWindowView;
type InboxResponse = InboxWindowView;
type ContactsResponse = ContactWindowView;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct MemberDirectoryResponse {
    items: Vec<ConversationMemberDirectoryEntry>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct PinnedMessagesResponse {
    items: Vec<MessageInteractionSummaryView>,
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

#[derive(Clone)]
struct PublicAppGuardrails {
    request_gate: Arc<Semaphore>,
}

#[derive(Debug)]
pub struct ProjectionApiError {
    status: axum::http::StatusCode,
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

static DEFAULT_PROJECTION_RUNTIME: OnceLock<Arc<ProjectionRuntime>> = OnceLock::new();

pub fn default_projection_service() -> Arc<TimelineProjectionService> {
    default_projection_runtime().service()
}

pub fn default_projection_runtime() -> Arc<ProjectionRuntime> {
    DEFAULT_PROJECTION_RUNTIME
        .get_or_init(|| {
            Arc::new(
                crate::build_projection_runtime_from_env().unwrap_or_else(|error| {
                    tracing::warn!(
                        "projection-service bootstrap failed ({error}); \
                         falling back to in-memory projection runtime for local development"
                    );
                    ProjectionRuntime::in_memory()
                }),
            )
        })
        .clone()
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
                message: "server is at maximum in-flight request capacity, please retry later".to_owned(),
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
        description: "Live OpenAPI contract generated from the projection-service router for inbox, timeline, contacts, read cursor, and interaction summary queries.",
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

async fn get_timeline(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    Path(conversation_id): Path<String>,
    Query(query): Query<TimelineQuery>,
    State(service): State<Arc<TimelineProjectionService>>,
) -> Response {
    let result: ApiResult<TimelineResponse> = (|| {
        Ok(service.timeline_window_from_auth_context(
            &auth,
            conversation_id.as_str(),
            query.after_seq,
            query.limit,
        )?)
    })();
    finish_api_json(&ctx, result)
}

async fn get_inbox(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    Query(query): Query<ListQuery>,
    State(service): State<Arc<TimelineProjectionService>>,
) -> Response {
    let result: ApiResult<InboxResponse> = (|| {
        Ok(service.inbox_window_from_auth_context(
            &auth,
            query.limit,
            query.cursor.as_deref(),
        )?)
    })();
    finish_api_json(&ctx, result)
}

async fn get_contacts(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    Query(query): Query<ListQuery>,
    State(service): State<Arc<TimelineProjectionService>>,
) -> Response {
    let result: ApiResult<ContactsResponse> = (|| {
        Ok(service.contact_window_from_auth_context(
            &auth,
            query.limit,
            query.cursor.as_deref(),
        )?)
    })();
    finish_api_json(&ctx, result)
}

async fn get_conversation_summary(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    Path(conversation_id): Path<String>,
    State(service): State<Arc<TimelineProjectionService>>,
) -> Response {
    let result: ApiResult<ConversationSummaryView> = (|| {
        let summary = service
            .conversation_summary_from_auth_context(&auth, conversation_id.as_str())?
            .ok_or_else(|| ProjectionApiError {
                status: axum::http::StatusCode::NOT_FOUND,
                code: "conversation_summary_not_found",
                message: format!("conversation summary not found: {conversation_id}"),
            })?;
        Ok(summary)
    })();
    finish_api_json(&ctx, result)
}

async fn get_read_cursor(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    Path(conversation_id): Path<String>,
    State(service): State<Arc<TimelineProjectionService>>,
) -> Response {
    let result: ApiResult<ConversationReadCursorView> = (|| {
        let cursor = service
            .read_cursor_from_auth_context(&auth, conversation_id.as_str())?
            .ok_or_else(|| ProjectionApiError {
                status: axum::http::StatusCode::NOT_FOUND,
                code: "conversation_read_cursor_not_found",
                message: format!("conversation read cursor not found: {conversation_id}"),
            })?;
        Ok(cursor)
    })();
    finish_api_json(&ctx, result)
}

async fn get_member_directory(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    Path(conversation_id): Path<String>,
    State(service): State<Arc<TimelineProjectionService>>,
) -> Response {
    let result: ApiResult<MemberDirectoryResponse> = (|| {
        Ok(MemberDirectoryResponse {
            items: service.member_directory_from_auth_context(&auth, conversation_id.as_str())?,
        })
    })();
    finish_api_json(&ctx, result)
}

async fn get_pinned_messages(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    Path(conversation_id): Path<String>,
    State(service): State<Arc<TimelineProjectionService>>,
) -> Response {
    let result: ApiResult<PinnedMessagesResponse> = (|| {
        Ok(PinnedMessagesResponse {
            items: service.pinned_messages_from_auth_context(&auth, conversation_id.as_str())?,
        })
    })();
    finish_api_json(&ctx, result)
}

async fn get_message_interaction_summary(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    Path((conversation_id, message_id)): Path<(String, String)>,
    State(service): State<Arc<TimelineProjectionService>>,
) -> Response {
    let result: ApiResult<MessageInteractionSummaryView> = (|| {
        let summary = service
            .message_interaction_summary_from_auth_context(
                &auth,
                conversation_id.as_str(),
                message_id.as_str(),
            )?
            .ok_or_else(|| ProjectionApiError {
                status: axum::http::StatusCode::NOT_FOUND,
                code: "message_interaction_summary_not_found",
                message: format!(
                    "message interaction summary not found: {conversation_id}/{message_id}"
                ),
            })?;
        Ok(summary)
    })();
    finish_api_json(&ctx, result)
}

async fn get_conversation_profile(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    Path(conversation_id): Path<String>,
    State(service): State<Arc<TimelineProjectionService>>,
) -> Response {
    let result: ApiResult<ConversationProfileItemResponse> = (|| {
        Ok(ConversationProfileItemResponse {
            item: service.conversation_profile_from_auth_context(&auth, conversation_id.as_str())?,
        })
    })();
    finish_api_json(&ctx, result)
}

async fn patch_conversation_profile(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    Path(conversation_id): Path<String>,
    State(service): State<Arc<TimelineProjectionService>>,
    Json(body): Json<UpdateConversationProfileRequest>,
) -> Response {
    let result: ApiResult<ConversationProfileItemResponse> = (|| {
        Ok(ConversationProfileItemResponse {
            item: service.update_conversation_profile_from_auth_context(
                &auth,
                conversation_id.as_str(),
                body,
            )?,
        })
    })();
    finish_api_json(&ctx, result)
}

async fn get_conversation_preferences(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    Path(conversation_id): Path<String>,
    State(service): State<Arc<TimelineProjectionService>>,
) -> Response {
    let result: ApiResult<ConversationPreferencesItemResponse> = (|| {
        Ok(ConversationPreferencesItemResponse {
            item: service
                .conversation_preferences_from_auth_context(&auth, conversation_id.as_str())?,
        })
    })();
    finish_api_json(&ctx, result)
}

async fn patch_conversation_preferences(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    Path(conversation_id): Path<String>,
    State(service): State<Arc<TimelineProjectionService>>,
    Json(body): Json<UpdateConversationPreferencesRequest>,
) -> Response {
    let result: ApiResult<ConversationPreferencesItemResponse> = (|| {
        Ok(ConversationPreferencesItemResponse {
            item: service.update_conversation_preferences_from_auth_context(
                &auth,
                conversation_id.as_str(),
                body,
            )?,
        })
    })();
    finish_api_json(&ctx, result)
}

async fn list_message_favorites(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    Query(query): Query<FavoriteMessagesQuery>,
    State(service): State<Arc<TimelineProjectionService>>,
) -> Response {
    let result: ApiResult<FavoriteMessagesWindowView> = (|| {
        Ok(service.message_favorites_window_from_auth_context(
            &auth,
            query.limit,
            query.cursor.as_deref(),
            query.favorite_type.as_deref(),
            query.q.as_deref(),
        )?)
    })();
    finish_api_json(&ctx, result)
}

async fn create_message_favorite(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    Path(message_id): Path<String>,
    State(service): State<Arc<TimelineProjectionService>>,
    Json(body): Json<FavoriteMessageRequest>,
) -> Response {
    let result: ApiResult<MessageFavoriteItemResponse> = (|| {
        Ok(MessageFavoriteItemResponse {
            item: service.create_message_favorite_from_auth_context(
                &auth,
                message_id.as_str(),
                body,
            )?,
        })
    })();
    finish_api_json(&ctx, result)
}

async fn delete_message_favorite(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    Path(favorite_id): Path<String>,
    State(service): State<Arc<TimelineProjectionService>>,
) -> Response {
    let result: ApiResult<DeleteMessageFavoriteResponse> = (|| {
        Ok(service.delete_message_favorite_from_auth_context(
            &auth,
            favorite_id.as_str(),
        )?)
    })();
    finish_api_json(&ctx, result)
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
