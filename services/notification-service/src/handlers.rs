use axum::Json;
use axum::extract::{Extension, Path, State};
use axum::http::HeaderMap;
use im_app_context::AppContext;
use im_domain_core::notification::NotificationTask;

use crate::dto::{NotificationListResponse, NotificationRequestResponse, RequestNotification};
use crate::error::NotificationError;
use crate::helpers::resolve_request_app_context;
use crate::state::AppState;

pub(crate) async fn request_notification(
    auth: Option<Extension<AppContext>>,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<RequestNotification>,
) -> Result<Json<NotificationRequestResponse>, NotificationError> {
    let auth = resolve_request_app_context(auth, &headers)?;
    Ok(Json(
        state
            .runtime
            .request_notification_from_app_context(&auth, request)?
            .into(),
    ))
}

pub(crate) async fn list_notifications(
    auth: Option<Extension<AppContext>>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<NotificationListResponse>, NotificationError> {
    let auth = resolve_request_app_context(auth, &headers)?;
    Ok(Json(NotificationListResponse {
        items: state.runtime.list_notifications(&auth)?,
    }))
}

pub(crate) async fn get_notification(
    Path(notification_id): Path<String>,
    auth: Option<Extension<AppContext>>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<NotificationTask>, NotificationError> {
    let auth = resolve_request_app_context(auth, &headers)?;
    Ok(Json(
        state
            .runtime
            .get_notification(&auth, notification_id.as_str())?,
    ))
}
