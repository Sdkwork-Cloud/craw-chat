//! Ban API handlers.

use axum::Json;
use axum::extract::{Extension, Path, Query, State};
use axum::response::Response;
use im_app_context::AppContext;
use serde::{Deserialize, Serialize};
use sdkwork_routes_web_framework_backend_api::response::{
    ApiProblem, ApiResult, finish_api_json, finish_api_response, no_content,
};
use sdkwork_web_core::WebRequestContext;

use crate::http::AppState;

#[derive(Debug, Deserialize)]
pub struct BanUserRequest {
    pub user_id: String,
    pub reason: Option<String>,
    pub expires_at: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct BanResponse {
    pub ban_id: String,
    pub banned_user_id: String,
    pub banned_by_user_id: String,
    pub reason: Option<String>,
    pub expires_at: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct ListQuery {
    pub limit: Option<i64>,
}

pub async fn ban_user(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(_auth): Extension<AppContext>,
    State(_state): State<AppState>,
    Path(_space_id): Path<String>,
    Json(_request): Json<BanUserRequest>,
) -> Response {
    let result: ApiResult<serde_json::Value> =
        Ok(serde_json::json!({"status": "banned"}));
    finish_api_json(&ctx, result)
}

pub async fn list_bans(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(_auth): Extension<AppContext>,
    State(_state): State<AppState>,
    Path(_space_id): Path<String>,
    Query(_query): Query<ListQuery>,
) -> Response {
    let result: ApiResult<Vec<BanResponse>> = Ok(Vec::new());
    finish_api_json(&ctx, result)
}

pub async fn get_ban(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(_auth): Extension<AppContext>,
    State(_state): State<AppState>,
    Path((_space_id, _user_id)): Path<(String, String)>,
) -> Response {
    let result: ApiResult<BanResponse> = Err(ApiProblem::not_found("ban not found"));
    finish_api_json(&ctx, result)
}

pub async fn unban_user(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(_auth): Extension<AppContext>,
    State(_state): State<AppState>,
    Path((_space_id, _user_id)): Path<(String, String)>,
) -> Response {
    finish_api_response(&ctx, no_content(&ctx))
}
