//! Channel access rule API handlers.

use axum::Json;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};

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
    State(_state): State<AppState>,
    Path((_space_id, _channel_id)): Path<(String, String)>,
    Json(_request): Json<CreateAccessRuleRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    Ok((
        StatusCode::CREATED,
        Json(serde_json::json!({"status": "created"})),
    ))
}

pub async fn list_access_rules(
    State(_state): State<AppState>,
    Path((_space_id, _channel_id)): Path<(String, String)>,
    Query(_query): Query<ListQuery>,
) -> Result<impl IntoResponse, StatusCode> {
    Ok(Json(Vec::<AccessRuleResponse>::new()))
}

pub async fn delete_access_rule(
    State(_state): State<AppState>,
    Path((_space_id, _channel_id, _rule_id)): Path<(String, String, String)>,
) -> Result<impl IntoResponse, StatusCode> {
    Ok(StatusCode::NO_CONTENT)
}
