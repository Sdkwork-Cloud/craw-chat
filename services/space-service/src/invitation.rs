//! Invitation API handlers.

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
pub struct CreateInvitationRequest {
    pub invitee_user_id: Option<String>,
    pub invitee_email: Option<String>,
    pub invitee_phone: Option<String>,
    pub target_type: String,
    pub target_id: String,
    pub role: Option<String>,
    pub message: Option<String>,
    pub expires_at: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct InvitationResponse {
    pub invitation_id: String,
    pub inviter_user_id: String,
    pub invitee_user_id: Option<String>,
    pub target_type: String,
    pub target_id: String,
    pub status: String,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct ListQuery {
    pub status: Option<String>,
    pub limit: Option<i64>,
}

pub async fn create_invitation(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(_auth): Extension<AppContext>,
    State(_state): State<AppState>,
    Path(_space_id): Path<String>,
    Json(_request): Json<CreateInvitationRequest>,
) -> Response {
    let result: ApiResult<serde_json::Value> =
        Ok(serde_json::json!({"status": "created"}));
    finish_api_json(&ctx, result)
}

pub async fn list_invitations(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(_auth): Extension<AppContext>,
    State(_state): State<AppState>,
    Path(_space_id): Path<String>,
    Query(_query): Query<ListQuery>,
) -> Response {
    let result: ApiResult<Vec<InvitationResponse>> = Ok(Vec::new());
    finish_api_json(&ctx, result)
}

pub async fn get_invitation(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(_auth): Extension<AppContext>,
    State(_state): State<AppState>,
    Path((_space_id, _invite_code)): Path<(String, String)>,
) -> Response {
    let result: ApiResult<InvitationResponse> = Err(ApiProblem::not_found("invitation not found"));
    finish_api_json(&ctx, result)
}

pub async fn revoke_invitation(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(_auth): Extension<AppContext>,
    State(_state): State<AppState>,
    Path((_space_id, _invite_code)): Path<(String, String)>,
) -> Response {
    finish_api_response(&ctx, no_content(&ctx))
}

pub async fn accept_invitation(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(_auth): Extension<AppContext>,
    State(_state): State<AppState>,
    Path((_space_id, _invite_code)): Path<(String, String)>,
) -> Response {
    finish_api_response(&ctx, no_content(&ctx))
}
