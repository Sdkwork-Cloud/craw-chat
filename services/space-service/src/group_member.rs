//! Group member API handlers.

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
pub struct AddMemberRequest {
    pub user_id: String,
    pub role: Option<String>,
    pub nickname: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct MemberResponse {
    pub user_id: String,
    pub role: String,
    pub nickname: Option<String>,
    pub mute_until: Option<String>,
    pub joined_at: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateMemberRequest {
    pub role: Option<String>,
    pub nickname: Option<String>,
    pub mute_until: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ListQuery {
    pub limit: Option<i64>,
}

pub async fn add_group_member(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(_auth): Extension<AppContext>,
    State(_state): State<AppState>,
    Path((_space_id, _group_id)): Path<(String, String)>,
    Json(_request): Json<AddMemberRequest>,
) -> Response {
    let result: ApiResult<serde_json::Value> =
        Ok(serde_json::json!({"status": "added"}));
    finish_api_json(&ctx, result)
}

pub async fn list_group_members(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(_auth): Extension<AppContext>,
    State(_state): State<AppState>,
    Path((_space_id, _group_id)): Path<(String, String)>,
    Query(_query): Query<ListQuery>,
) -> Response {
    let result: ApiResult<Vec<MemberResponse>> = Ok(Vec::new());
    finish_api_json(&ctx, result)
}

pub async fn get_group_member(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(_auth): Extension<AppContext>,
    State(_state): State<AppState>,
    Path((_space_id, _group_id, _user_id)): Path<(String, String, String)>,
) -> Response {
    let result: ApiResult<MemberResponse> = Err(ApiProblem::not_found("group member not found"));
    finish_api_json(&ctx, result)
}

pub async fn update_group_member(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(_auth): Extension<AppContext>,
    State(_state): State<AppState>,
    Path((_space_id, _group_id, _user_id)): Path<(String, String, String)>,
    Json(_request): Json<UpdateMemberRequest>,
) -> Response {
    finish_api_response(&ctx, no_content(&ctx))
}

pub async fn remove_group_member(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(_auth): Extension<AppContext>,
    State(_state): State<AppState>,
    Path((_space_id, _group_id, _user_id)): Path<(String, String, String)>,
) -> Response {
    finish_api_response(&ctx, no_content(&ctx))
}
