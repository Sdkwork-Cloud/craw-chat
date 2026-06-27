//! User settings API handlers.

use axum::Json;
use axum::extract::{Extension, Path, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::IntoResponse;
use im_app_context::AppContext;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::postgres::http::PostgresAppState;
use crate::postgres::service_http::require_request_scope;

#[derive(Debug, Serialize)]
pub struct UserSettingsResponse {
    pub settings: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserSettingsRequest {
    pub settings: HashMap<String, serde_json::Value>,
}

pub async fn get_user_settings(
    State(state): State<PostgresAppState>,
    headers: HeaderMap,
    auth: Option<Extension<AppContext>>,
    Path(user_id): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    let scope = require_request_scope(auth, &headers)?;
    if scope.user_id != user_id {
        return Err(StatusCode::FORBIDDEN);
    }

    match state.user_settings_store.list_by_user(
        scope.tenant_id.as_str(),
        scope.organization_id.as_str(),
        user_id.as_str(),
    ) {
        Ok(settings) => Ok(Json(UserSettingsResponse { settings })),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn update_user_settings(
    State(state): State<PostgresAppState>,
    headers: HeaderMap,
    auth: Option<Extension<AppContext>>,
    Path(user_id): Path<String>,
    Json(request): Json<UpdateUserSettingsRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let scope = require_request_scope(auth, &headers)?;
    if scope.user_id != user_id {
        return Err(StatusCode::FORBIDDEN);
    }

    let now = chrono::Utc::now().to_rfc3339();
    match state.user_settings_store.upsert_settings(
        scope.tenant_id.as_str(),
        scope.organization_id.as_str(),
        user_id.as_str(),
        &request.settings,
        now.as_str(),
    ) {
        Ok(()) => Ok(StatusCode::NO_CONTENT),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
