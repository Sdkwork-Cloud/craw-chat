//! Friendship API handlers.

use axum::extract::{Extension, Path, Query, State};
use axum::response::Response;
use im_app_context::AppContext;
use sdkwork_routes_web_framework_backend_api::response::{
    ApiProblem, ApiResult, finish_api_json, finish_api_response, no_content,
};
use sdkwork_web_core::WebRequestContext;
use serde::{Deserialize, Serialize};

use im_adapters_social_postgres::friendship_store::FriendshipRecord;

use crate::postgres::http::PostgresAppState;

#[derive(Debug, Serialize)]
pub struct FriendshipResponse {
    pub friendship_id: String,
    pub user_low_id: String,
    pub user_high_id: String,
    pub status: String,
    pub established_at: Option<String>,
}

impl From<FriendshipRecord> for FriendshipResponse {
    fn from(record: FriendshipRecord) -> Self {
        Self {
            friendship_id: record.friendship_id.to_string(),
            user_low_id: record.user_low_id,
            user_high_id: record.user_high_id,
            status: record.status,
            established_at: record.established_at,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct ListQuery {
    pub limit: Option<i64>,
}

pub async fn list_friends(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<PostgresAppState>,
    Query(query): Query<ListQuery>,
) -> Response {
    let result: ApiResult<Vec<FriendshipResponse>> = (|| {
        let limit = query.limit.unwrap_or(20);
        let records = state
            .friendship_store
            .list_by_user(
                auth.tenant_id.as_str(),
                auth.organization_id.as_str(),
                auth.actor_id.as_str(),
                "active",
                limit,
            )
            .map_err(|_| ApiProblem::internal_server_error("failed to list friendships"))?;
        Ok(records.into_iter().map(FriendshipResponse::from).collect())
    })();
    finish_api_json(&ctx, result)
}

pub async fn get_friendship(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<PostgresAppState>,
    Path(friendship_id): Path<String>,
) -> Response {
    let result: ApiResult<FriendshipResponse> = (|| {
        let fid: i64 = friendship_id.parse().unwrap_or(0);
        let record = state
            .friendship_store
            .get_by_id(auth.tenant_id.as_str(), auth.organization_id.as_str(), fid)
            .map_err(|_| ApiProblem::internal_server_error("failed to read friendship"))?
            .ok_or_else(|| ApiProblem::not_found("friendship not found"))?;
        Ok(FriendshipResponse::from(record))
    })();
    finish_api_json(&ctx, result)
}

pub async fn remove_friendship(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<PostgresAppState>,
    Path(friendship_id): Path<String>,
) -> Response {
    let result: Result<Response, ApiProblem> = (|| {
        let fid: i64 = friendship_id.parse().unwrap_or(0);
        let now = chrono::Utc::now().to_rfc3339();
        state
            .friendship_store
            .update_status(
                auth.tenant_id.as_str(),
                auth.organization_id.as_str(),
                fid,
                "removed",
                &now,
            )
            .map_err(|_| ApiProblem::internal_server_error("failed to remove friendship"))?;
        no_content(&ctx)
    })();
    finish_api_response(&ctx, result)
}
