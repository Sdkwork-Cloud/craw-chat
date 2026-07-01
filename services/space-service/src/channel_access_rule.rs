//! Channel access rule API handlers.

use axum::Json;
use axum::extract::{Extension, Path, Query, State};
use axum::response::Response;
use im_app_context::AppContext;
use serde::{Deserialize, Serialize};
use sdkwork_routes_web_framework_backend_api::response::{
    ApiResult, finish_api_json, finish_api_response, no_content,
};
use sdkwork_web_core::WebRequestContext;

use crate::http::AppState;

#[derive(Debug, Deserialize)]
pub struct CreateAccessRuleRequest {
    pub rule_type: String,
    pub principal_kind: Option<String>,
    pub principal_id: Option<String>,
    pub permission: String,
}

#[derive(Debug, Serialize)]
pub struct AccessRuleResponse {
    pub rule_id: String,
    pub channel_id: String,
    pub rule_type: String,
    pub principal_kind: Option<String>,
    pub principal_id: Option<String>,
    pub permission: String,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct ListQuery {
    pub limit: Option<i64>,
}

pub async fn create_access_rule(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(_auth): Extension<AppContext>,
    State(_state): State<AppState>,
    Path((_space_id, _channel_id)): Path<(String, String)>,
    Json(_request): Json<CreateAccessRuleRequest>,
) -> Response {
    let result: ApiResult<serde_json::Value> =
        Ok(serde_json::json!({"status": "created"}));
    finish_api_json(&ctx, result)
}

pub async fn list_access_rules(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(_auth): Extension<AppContext>,
    State(_state): State<AppState>,
    Path((_space_id, _channel_id)): Path<(String, String)>,
    Query(_query): Query<ListQuery>,
) -> Response {
    let result: ApiResult<Vec<AccessRuleResponse>> = Ok(Vec::new());
    finish_api_json(&ctx, result)
}

pub async fn delete_access_rule(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(_auth): Extension<AppContext>,
    State(_state): State<AppState>,
    Path((_space_id, _channel_id, _rule_id)): Path<(String, String, String)>,
) -> Response {
    finish_api_response(&ctx, no_content(&ctx))
}
