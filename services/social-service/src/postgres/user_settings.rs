//! User settings API handlers.

use axum::Json;
use axum::extract::{Extension, Path, State};
use axum::response::Response;
use im_app_context::AppContext;
use sdkwork_routes_web_framework_backend_api::response::{
    ApiProblem, ApiResult, finish_api_json, finish_api_response, no_content,
};
use sdkwork_web_core::WebRequestContext;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::postgres::http::PostgresAppState;

#[derive(Debug, Serialize)]
pub struct UserSettingsResponse {
    pub settings: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserSettingsRequest {
    pub settings: HashMap<String, serde_json::Value>,
}

pub async fn get_user_settings(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<PostgresAppState>,
    Path(user_id): Path<String>,
) -> Response {
    let result: ApiResult<UserSettingsResponse> = (|| {
        if auth.actor_id != user_id {
            return Err(ApiProblem::forbidden("user can only read own settings"));
        }

        let settings = state
            .user_settings_store
            .list_by_user(
                auth.tenant_id.as_str(),
                auth.organization_id.as_str(),
                user_id.as_str(),
            )
            .map_err(|_| ApiProblem::internal_server_error("failed to read user settings"))?;
        Ok(UserSettingsResponse { settings })
    })();
    finish_api_json(&ctx, result)
}

pub async fn update_user_settings(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<PostgresAppState>,
    Path(user_id): Path<String>,
    Json(request): Json<UpdateUserSettingsRequest>,
) -> Response {
    let result: Result<Response, ApiProblem> = (|| {
        if auth.actor_id != user_id {
            return Err(ApiProblem::forbidden("user can only update own settings"));
        }

        let now = chrono::Utc::now().to_rfc3339();
        state
            .user_settings_store
            .upsert_settings(
                auth.tenant_id.as_str(),
                auth.organization_id.as_str(),
                user_id.as_str(),
                &request.settings,
                now.as_str(),
            )
            .map_err(|_| ApiProblem::internal_server_error("failed to update user settings"))?;
        no_content(&ctx)
    })();
    finish_api_response(&ctx, result)
}
