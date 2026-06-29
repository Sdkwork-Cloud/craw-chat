//! Open API (`/im/v3/api/social/*`) handlers backed by the social runtime.

use std::sync::OnceLock;

use axum::Json;
use axum::extract::{Path, Query, State};
use axum::http::HeaderMap;
use axum::routing::{get, post};
use axum::Router;
use im_domain_core::social::FriendRequest;
use im_time::utc_now_rfc3339_millis;
use sdkwork_im_runtime_id::RuntimeSnowflakeIdGenerator;
use serde::{Deserialize, Serialize};

use crate::friendship::{
    self, AcceptFriendRequestRequest, AppState, CancelFriendRequestRequest,
    DeclineFriendRequestRequest, FriendRequestInventoryDirectionQuery,
    FriendRequestInventoryStatusQuery, RemoveFriendshipRequest, SocialServiceError,
    SubmitFriendRequestRequest,
};
use crate::runtime::deterministic_social_id;

const FRIEND_REQUEST_LIST_DEFAULT_LIMIT: usize = 100;
const FRIEND_REQUEST_LIST_MAX_LIMIT: usize = 200;

static OPEN_API_ID_GENERATOR: OnceLock<RuntimeSnowflakeIdGenerator> = OnceLock::new();

/// Initialize the open-api ID generator from the database.
///
/// Must be called during async service startup before any request is served.
/// If not called, the generator falls back to lazy env-based initialization.
pub async fn init_open_api_id_generator() {
    if OPEN_API_ID_GENERATOR.get().is_some() {
        return;
    }
    let generator = RuntimeSnowflakeIdGenerator::from_database_env("social-service")
        .await
        .unwrap_or_else(|error| {
            tracing::warn!(
                ?error,
                "database node_id allocation failed; falling back to env for social open-api"
            );
            RuntimeSnowflakeIdGenerator::from_env().unwrap_or_else(|error| {
                tracing::warn!(
                    ?error,
                    "SDKWORK_IM_ID_NODE_ID missing; using snowflake node 0 for social open-api handlers"
                );
                RuntimeSnowflakeIdGenerator::with_node_id(0)
                    .expect("snowflake node 0 must initialize")
            })
        });
    let _ = OPEN_API_ID_GENERATOR.set(generator);
}

fn id_generator() -> &'static RuntimeSnowflakeIdGenerator {
    OPEN_API_ID_GENERATOR.get_or_init(|| {
        // Fallback for lazy init (e.g., in tests without database)
        RuntimeSnowflakeIdGenerator::from_env().unwrap_or_else(|error| {
            tracing::warn!(
                ?error,
                "SDKWORK_IM_ID_NODE_ID missing; using snowflake node 0 for social open-api handlers"
            );
            RuntimeSnowflakeIdGenerator::with_node_id(0)
                .expect("snowflake node 0 must initialize")
        })
    })
}

pub(crate) fn next_open_api_id() -> Result<String, SocialServiceError> {
    id_generator()
        .next_id()
        .map(|value| value.to_string())
        .map_err(|error| {
            SocialServiceError::invalid(
                "id_generation_failed",
                format!("open-api id generation failed: {error}"),
            )
        })
}

fn next_open_api_event_id() -> Result<String, SocialServiceError> {
    Ok(format!("evt_{}", next_open_api_id()?))
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct OpenApiFriendRequestListQuery {
    direction: Option<FriendRequestInventoryDirectionQuery>,
    #[serde(default)]
    status: FriendRequestInventoryStatusQuery,
    limit: Option<usize>,
    cursor: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct OpenApiSubmitFriendRequestRequest {
    target_user_id: String,
    request_message: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct OpenApiFriendRequestListResponse {
    items: Vec<FriendRequest>,
    #[serde(skip_serializing_if = "Option::is_none")]
    next_cursor: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct OpenApiFriendRequestMutationResponse {
    friend_request: FriendRequest,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct OpenApiFriendshipMutationResponse {
    friendship: im_domain_core::social::Friendship,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct OpenApiCreateConversationResult {
    tenant_id: String,
    conversation_id: String,
    kind: String,
    created_at: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct OpenApiFriendRequestAcceptanceResponse {
    friend_request: FriendRequest,
    friendship: im_domain_core::social::Friendship,
    direct_chat: im_domain_core::social::DirectChat,
    conversation: OpenApiCreateConversationResult,
}

pub fn build_open_api_router(state: AppState) -> Router {
    Router::new()
        .route(
            "/im/v3/api/social/friend_requests",
            get(list_friend_requests).post(create_friend_request),
        )
        .route(
            "/im/v3/api/social/friend_requests/{request_id}/accept",
            post(accept_friend_request),
        )
        .route(
            "/im/v3/api/social/friend_requests/{request_id}/decline",
            post(decline_friend_request),
        )
        .route(
            "/im/v3/api/social/friend_requests/{request_id}/cancel",
            post(cancel_friend_request),
        )
        .route(
            "/im/v3/api/social/friendships/{friendship_id}/remove",
            post(remove_friendship),
        )
        .merge(crate::openapi_contacts::routes())
        .with_state(state)
}

async fn list_friend_requests(
    headers: HeaderMap,
    Query(query): Query<OpenApiFriendRequestListQuery>,
    State(state): State<AppState>,
) -> Result<Json<OpenApiFriendRequestListResponse>, SocialServiceError> {
    let auth = friendship::resolve_auth_from_headers(&headers)?;
    let direction = query.direction.unwrap_or(FriendRequestInventoryDirectionQuery::Incoming);
    let limit = query.limit.unwrap_or(FRIEND_REQUEST_LIST_DEFAULT_LIMIT);
    if limit == 0 || limit > FRIEND_REQUEST_LIST_MAX_LIMIT {
        return Err(SocialServiceError::invalid(
            "limit_invalid",
            format!("limit must be between 1 and {FRIEND_REQUEST_LIST_MAX_LIMIT}"),
        ));
    }

    let cursor = if let Some(cursor) = query.cursor.as_deref() {
        Some(friendship::parse_friend_request_inventory_cursor(cursor)?)
    } else {
        None
    };

    let _read_lock = state.social_runtime.acquire_cross_instance_read_lock()?;
    state
        .social_runtime
        .refresh_state_from_authority_for_write()?;
    let page = state.social_runtime.list_friend_requests(
        auth.tenant_id.as_str(),
        auth.organization_id.as_str(),
        auth.user_id.as_str(),
        direction,
        query.status,
        limit,
        cursor.as_ref(),
    )?;

    Ok(Json(OpenApiFriendRequestListResponse {
        items: page.items,
        next_cursor: page.next_cursor,
    }))
}

async fn create_friend_request(
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<OpenApiSubmitFriendRequestRequest>,
) -> Result<Json<OpenApiFriendRequestMutationResponse>, SocialServiceError> {
    let auth = friendship::resolve_auth_from_headers(&headers)?;
    let requested_at = utc_now_rfc3339_millis();
    let submitted = state.social_runtime.submit_friend_request(
        auth.tenant_id.as_str(),
        &auth,
        SubmitFriendRequestRequest {
            request_id: next_open_api_id()?,
            event_id: next_open_api_event_id()?,
            requester_user_id: auth.user_id.clone(),
            target_user_id: request.target_user_id,
            request_message: request.request_message,
            requested_at,
        },
    )?;

    Ok(Json(OpenApiFriendRequestMutationResponse {
        friend_request: submitted.friend_request,
    }))
}

async fn accept_friend_request(
    Path(request_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<OpenApiFriendRequestAcceptanceResponse>, SocialServiceError> {
    let auth = friendship::resolve_auth_from_headers(&headers)?;
    let accepted_at = utc_now_rfc3339_millis();
    let accepted = state.social_runtime.accept_friend_request(
        auth.tenant_id.as_str(),
        &auth,
        request_id.as_str(),
        AcceptFriendRequestRequest {
            event_id: next_open_api_event_id()?,
            accepted_by_user_id: auth.user_id.clone(),
            accepted_at: accepted_at.clone(),
        },
    )?;

    let friendship = accepted.friendship.ok_or_else(|| {
        SocialServiceError::invalid(
            "friendship_materialization_failed",
            format!("friend request {request_id} was accepted without friendship materialization"),
        )
    })?;
    let direct_chat = accepted.direct_chat.ok_or_else(|| {
        SocialServiceError::invalid(
            "direct_chat_materialization_failed",
            format!("friend request {request_id} was accepted without direct chat materialization"),
        )
    })?;
    let conversation_id = direct_chat
        .conversation_id
        .clone()
        .filter(|value| !value.trim().is_empty())
        .unwrap_or_else(|| deterministic_social_id("c_direct_", request_id.as_str()));
    let tenant_id = accepted.friend_request.tenant_id.clone();

    Ok(Json(OpenApiFriendRequestAcceptanceResponse {
        friend_request: accepted.friend_request,
        friendship,
        direct_chat: direct_chat.clone(),
        conversation: OpenApiCreateConversationResult {
            tenant_id,
            conversation_id,
            kind: "direct".into(),
            created_at: accepted_at,
        },
    }))
}

async fn decline_friend_request(
    Path(request_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<OpenApiFriendRequestMutationResponse>, SocialServiceError> {
    let auth = friendship::resolve_auth_from_headers(&headers)?;
    let declined_at = utc_now_rfc3339_millis();
    let declined = state.social_runtime.decline_friend_request(
        auth.tenant_id.as_str(),
        &auth,
        request_id.as_str(),
        DeclineFriendRequestRequest {
            event_id: next_open_api_event_id()?,
            declined_by_user_id: auth.user_id.clone(),
            declined_at,
        },
    )?;

    Ok(Json(OpenApiFriendRequestMutationResponse {
        friend_request: declined.friend_request,
    }))
}

async fn cancel_friend_request(
    Path(request_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<OpenApiFriendRequestMutationResponse>, SocialServiceError> {
    let auth = friendship::resolve_auth_from_headers(&headers)?;
    let canceled_at = utc_now_rfc3339_millis();
    let canceled = state.social_runtime.cancel_friend_request(
        auth.tenant_id.as_str(),
        &auth,
        request_id.as_str(),
        CancelFriendRequestRequest {
            event_id: next_open_api_event_id()?,
            canceled_by_user_id: auth.user_id.clone(),
            canceled_at,
        },
    )?;

    Ok(Json(OpenApiFriendRequestMutationResponse {
        friend_request: canceled.friend_request,
    }))
}

async fn remove_friendship(
    Path(friendship_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<OpenApiFriendshipMutationResponse>, SocialServiceError> {
    let auth = friendship::resolve_auth_from_headers(&headers)?;
    let removed_at = utc_now_rfc3339_millis();
    let removed = state.social_runtime.remove_friendship(
        auth.tenant_id.as_str(),
        &auth,
        friendship_id.as_str(),
        RemoveFriendshipRequest {
            event_id: next_open_api_event_id()?,
            removed_by_user_id: auth.user_id.clone(),
            removed_at,
        },
    )?;

    Ok(Json(OpenApiFriendshipMutationResponse {
        friendship: removed.friendship,
    }))
}
