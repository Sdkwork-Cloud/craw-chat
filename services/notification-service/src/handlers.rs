use axum::Json;
use axum::extract::{Extension, Path, State};
use axum::response::Response;
use im_app_context::AppContext;
use im_domain_core::notification::NotificationTask;
use sdkwork_routes_web_framework_backend_api::response::{ApiResult, finish_api_json};
use sdkwork_web_core::WebRequestContext;

use crate::dto::{NotificationListResponse, NotificationRequestResponse, RequestNotification};
use crate::state::AppState;

pub(crate) async fn request_notification(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    Json(request): Json<RequestNotification>,
) -> Response {
    let result: ApiResult<NotificationRequestResponse> = (|| {
        Ok(state
            .runtime
            .request_notification_from_app_context(&auth, request)?
            .into())
    })();
    finish_api_json(&ctx, result)
}

pub(crate) async fn list_notifications(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
) -> Response {
    let result: ApiResult<NotificationListResponse> = (|| {
        Ok(NotificationListResponse {
            items: state.runtime.list_notifications(&auth)?,
        })
    })();
    finish_api_json(&ctx, result)
}

pub(crate) async fn get_notification(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    Path(notification_id): Path<String>,
) -> Response {
    let result: ApiResult<NotificationTask> = (|| {
        Ok(state
            .runtime
            .get_notification(&auth, notification_id.as_str())?)
    })();
    finish_api_json(&ctx, result)
}
