//! Open API (`/im/v3/api/social/*`) handlers backed by the social runtime.

use std::sync::OnceLock;

use axum::extract::{Extension, Path, Query, State};
use axum::response::Response;
use axum::routing::{delete, get, post};
use axum::{Json, Router};
use im_app_context::AppContext;
use im_domain_core::social::FriendRequest;
use im_time::utc_now_rfc3339_millis;
use sdkwork_im_runtime_id::RuntimeSnowflakeIdGenerator;
use sdkwork_utils_rust::{cursor_list_page_data, SdkWorkCursorListQuery};
use sdkwork_web_core::WebRequestContext;
use serde::{Deserialize, Serialize};

use crate::friendship::{
    self, AcceptFriendRequestRequest, AppState, CancelFriendRequestRequest,
    DeclineFriendRequestRequest, FriendRequestInventoryDirectionQuery,
    FriendRequestInventoryStatusQuery, RemoveFriendshipRequest, SocialServiceError,
    SubmitFriendRequestRequest,
};
use crate::api_payload::resource_item;
use crate::block::{BlockUserRequest, OpenApiUserBlockResponse, ReleaseUserBlockRequest};
use im_domain_core::social::BlockScope;
use crate::runtime::deterministic_social_id;

use sdkwork_utils_rust::MAX_LIST_PAGE_SIZE;

const FRIEND_REQUEST_LIST_MAX_LIMIT: usize = MAX_LIST_PAGE_SIZE as usize;

fn openapi_social_principal(auth: &AppContext) -> Result<&str, SocialServiceError> {
    auth.ensure_user_actor_principal().map_err(|error| {
        SocialServiceError::invalid("social_principal_invalid", error.message())
    })
}

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

pub(crate) fn finish_open_api_json<T: Serialize>(
    ctx: &WebRequestContext,
    result: Result<T, SocialServiceError>,
) -> Response {
    crate::envelope::finish_enveloped_json(ctx, result)
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct OpenApiFriendRequestListQuery {
    direction: Option<FriendRequestInventoryDirectionQuery>,
    #[serde(default)]
    status: FriendRequestInventoryStatusQuery,
    #[serde(flatten)]
    paging: SdkWorkCursorListQuery,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct OpenApiSubmitFriendRequestRequest {
    target_user_id: String,
    request_message: Option<String>,
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

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct OpenApiFriendRequestPendingCountResponse {
    count: usize,
}

pub fn build_open_api_router(state: AppState) -> Router {
    Router::new()
        .route(
            "/im/v3/api/social/friend_requests",
            get(list_friend_requests).post(create_friend_request),
        )
        .route(
            "/im/v3/api/social/friend_requests/pending/count",
            get(count_pending_friend_requests),
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
            "/im/v3/api/social/friendships",
            get(list_friendships),
        )
        .route(
            "/im/v3/api/social/friendships/{friendship_id}/remove",
            post(remove_friendship),
        )
        .route(
            "/im/v3/api/social/user_blocks",
            post(create_user_block),
        )
        .route(
            "/im/v3/api/social/user_blocks/{block_id}",
            delete(release_user_block),
        )
        .merge(crate::openapi_contacts::routes())
        .with_state(state)
}

async fn count_pending_friend_requests(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
) -> Response {
    let result = (|| {
        let _read_lock = state.social_runtime.acquire_cross_instance_read_lock()?;
        state
            .social_runtime
            .refresh_state_from_authority_for_read()?;
        let count = state.social_runtime.count_pending_incoming_friend_requests(
            auth.tenant_id.as_str(),
            auth.organization_id.as_str(),
            openapi_social_principal(&auth)?,
        );
        Ok(resource_item(OpenApiFriendRequestPendingCountResponse { count }))
    })();
    finish_open_api_json(&ctx, result)
}

async fn list_friend_requests(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    Query(query): Query<OpenApiFriendRequestListQuery>,
    State(state): State<AppState>,
) -> Response {
    let result = (|| {
        let direction = query
            .direction
            .unwrap_or(FriendRequestInventoryDirectionQuery::Incoming);
        let paging = query.paging.resolve().map_err(|_| {
            SocialServiceError::invalid("cursor_invalid", "friend request list cursor is invalid")
        })?;
        let limit = paging.page_size;
        if limit == 0 || limit > FRIEND_REQUEST_LIST_MAX_LIMIT {
            return Err(SocialServiceError::invalid(
                "page_size_invalid",
                format!("pageSize must be between 1 and {FRIEND_REQUEST_LIST_MAX_LIMIT}"),
            ));
        }

        let cursor = if let Some(cursor) = query.paging.cursor.as_deref() {
            Some(friendship::parse_friend_request_inventory_cursor(cursor)?)
        } else {
            None
        };

        let _read_lock = state.social_runtime.acquire_cross_instance_read_lock()?;
        state
            .social_runtime
            .refresh_state_from_authority_for_read()?;
        let page = state.social_runtime.list_friend_requests(
            auth.tenant_id.as_str(),
            auth.organization_id.as_str(),
            openapi_social_principal(&auth)?,
            direction,
            query.status,
            limit,
            cursor.as_ref(),
        )?;

        let has_more = page.next_cursor.is_some();
        Ok(cursor_list_page_data(
            page.items,
            limit,
            page.next_cursor,
            has_more,
        ))
    })();

    finish_open_api_json(&ctx, result)
}

async fn create_friend_request(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    Json(request): Json<OpenApiSubmitFriendRequestRequest>,
) -> Response {
    let result = (|| {
        let requested_at = utc_now_rfc3339_millis();
        let submitted = state.social_runtime.submit_friend_request(
            auth.tenant_id.as_str(),
            &auth,
            SubmitFriendRequestRequest {
                request_id: next_open_api_id()?,
                event_id: next_open_api_event_id()?,
                requester_user_id: openapi_social_principal(&auth)?.to_owned(),
                target_user_id: request.target_user_id,
                request_message: request.request_message,
                requested_at,
            },
        )?;

        Ok(resource_item(OpenApiFriendRequestMutationResponse {
            friend_request: submitted.friend_request,
        }))
    })();

    finish_open_api_json(&ctx, result)
}

async fn accept_friend_request(
    Path(request_id): Path<String>,
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
) -> Response {
    let result = (|| {
        let accepted_at = utc_now_rfc3339_millis();
        let accepted = state.social_runtime.accept_friend_request(
            auth.tenant_id.as_str(),
            &auth,
            request_id.as_str(),
            AcceptFriendRequestRequest {
                event_id: next_open_api_event_id()?,
                accepted_by_user_id: openapi_social_principal(&auth)?.to_owned(),
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

        Ok(resource_item(OpenApiFriendRequestAcceptanceResponse {
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
    })();

    finish_open_api_json(&ctx, result)
}

async fn decline_friend_request(
    Path(request_id): Path<String>,
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
) -> Response {
    let result = (|| {
        let declined_at = utc_now_rfc3339_millis();
        let declined = state.social_runtime.decline_friend_request(
            auth.tenant_id.as_str(),
            &auth,
            request_id.as_str(),
            DeclineFriendRequestRequest {
                event_id: next_open_api_event_id()?,
                declined_by_user_id: openapi_social_principal(&auth)?.to_owned(),
                declined_at,
            },
        )?;

        Ok(resource_item(OpenApiFriendRequestMutationResponse {
            friend_request: declined.friend_request,
        }))
    })();

    finish_open_api_json(&ctx, result)
}

async fn cancel_friend_request(
    Path(request_id): Path<String>,
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
) -> Response {
    let result = (|| {
        let canceled_at = utc_now_rfc3339_millis();
        let canceled = state.social_runtime.cancel_friend_request(
            auth.tenant_id.as_str(),
            &auth,
            request_id.as_str(),
            CancelFriendRequestRequest {
                event_id: next_open_api_event_id()?,
                canceled_by_user_id: openapi_social_principal(&auth)?.to_owned(),
                canceled_at,
            },
        )?;

        Ok(resource_item(OpenApiFriendRequestMutationResponse {
            friend_request: canceled.friend_request,
        }))
    })();

    finish_open_api_json(&ctx, result)
}

async fn list_friendships(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    Query(query): Query<SdkWorkCursorListQuery>,
    State(state): State<AppState>,
) -> Response {
    let result = (|| {
        let paging = query.resolve().map_err(|_| {
            SocialServiceError::invalid("cursor_invalid", "friendship list cursor is invalid")
        })?;
        let limit = paging.page_size;
        if limit == 0 || limit > friendship::FRIENDSHIP_LIST_MAX_LIMIT {
            return Err(SocialServiceError::invalid(
                "page_size_invalid",
                format!(
                    "pageSize must be between 1 and {}",
                    friendship::FRIENDSHIP_LIST_MAX_LIMIT
                ),
            ));
        }
        let cursor = if let Some(cursor) = query.cursor.as_deref() {
            Some(friendship::parse_friendship_inventory_cursor(cursor)?)
        } else {
            None
        };

        let _read_lock = state.social_runtime.acquire_cross_instance_read_lock()?;
        state
            .social_runtime
            .refresh_state_from_authority_for_read()?;
        let page = state.social_runtime.list_friendships(
            auth.tenant_id.as_str(),
            auth.organization_id.as_str(),
            openapi_social_principal(&auth)?,
            limit,
            cursor.as_ref(),
        )?;
        let has_more = page.next_cursor.is_some();
        Ok(cursor_list_page_data(
            page.items,
            limit,
            page.next_cursor,
            has_more,
        ))
    })();

    finish_open_api_json(&ctx, result)
}

async fn remove_friendship(
    Path(friendship_id): Path<String>,
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
) -> Response {
    let result = (|| {
        let removed_at = utc_now_rfc3339_millis();
        let removed = state.social_runtime.remove_friendship(
            auth.tenant_id.as_str(),
            &auth,
            friendship_id.as_str(),
            RemoveFriendshipRequest {
                event_id: next_open_api_event_id()?,
                removed_by_user_id: openapi_social_principal(&auth)?.to_owned(),
                removed_at,
            },
        )?;

        Ok(resource_item(OpenApiFriendshipMutationResponse {
            friendship: removed.friendship,
        }))
    })();

    finish_open_api_json(&ctx, result)
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct OpenApiBlockUserRequest {
    blocked_user_id: String,
    scope: BlockScope,
    direct_chat_id: Option<String>,
    expires_at: Option<String>,
}

async fn create_user_block(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    Json(request): Json<OpenApiBlockUserRequest>,
) -> Response {
    let result = (|| {
        let effective_at = utc_now_rfc3339_millis();
        let block_id = format!("blk_{}", next_open_api_id()?);
        let blocked = state.social_runtime.block_user(
            auth.tenant_id.as_str(),
            &auth,
            BlockUserRequest {
                block_id,
                event_id: next_open_api_event_id()?,
                blocker_user_id: openapi_social_principal(&auth)?.to_owned(),
                blocked_user_id: request.blocked_user_id,
                scope: request.scope,
                direct_chat_id: request.direct_chat_id,
                expires_at: request.expires_at,
                effective_at,
            },
        )?;
        Ok(resource_item(OpenApiUserBlockResponse {
            user_block: blocked.user_block,
            latest_commit: blocked.latest_commit.into(),
            persistence: blocked.persistence,
        }))
    })();
    finish_open_api_json(&ctx, result)
}

async fn release_user_block(
    Path(block_id): Path<String>,
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
) -> Response {
    let result = (|| {
        let released_at = utc_now_rfc3339_millis();
        let released = state.social_runtime.release_user_block(
            auth.tenant_id.as_str(),
            &auth,
            block_id.as_str(),
            ReleaseUserBlockRequest {
                event_id: next_open_api_event_id()?,
                released_by_user_id: openapi_social_principal(&auth)?.to_owned(),
                released_at,
            },
        )?;
        Ok(resource_item(OpenApiUserBlockResponse {
            user_block: released.user_block,
            latest_commit: released.latest_commit.into(),
            persistence: released.persistence,
        }))
    })();
    finish_open_api_json(&ctx, result)
}
