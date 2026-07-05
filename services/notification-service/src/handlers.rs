use axum::Json;
use axum::extract::{Extension, Path, Query, State};
use axum::response::Response;
use im_app_context::AppContext;
use sdkwork_routes_web_framework_backend_api::response::{ApiResult, finish_api_json};
use sdkwork_utils_rust::SdkWorkCursorListQuery;
use sdkwork_web_core::WebRequestContext;

use crate::dto::{NotificationListResponse, NotificationRequestResponse, RequestNotification};
use crate::error::NotificationError;
use crate::state::AppState;

async fn run_notification_sync<F, T>(operation: F) -> ApiResult<T>
where
    F: FnOnce() -> ApiResult<T> + Send + 'static,
    T: Send + 'static,
{
    tokio::task::spawn_blocking(operation)
        .await
        .unwrap_or_else(|join_error| {
            Err(NotificationError {
                status: axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                code: "notification_runtime_worker_failed",
                message: format!("notification runtime worker failed: {join_error}"),
            }
            .into())
        })
}

pub(crate) async fn request_notification(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    Json(request): Json<RequestNotification>,
) -> Response {
    let runtime = state.runtime.clone();
    let result: ApiResult<NotificationRequestResponse> = run_notification_sync(move || {
        Ok(runtime
            .request_notification_from_app_context(&auth, request)?
            .into())
    })
    .await;
    finish_api_json(&ctx, result)
}

pub(crate) async fn list_notifications(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    Query(query): Query<SdkWorkCursorListQuery>,
    State(state): State<AppState>,
) -> Response {
    let runtime = state.runtime.clone();
    let result: ApiResult<NotificationListResponse> = run_notification_sync(move || {
        Ok(NotificationListResponse::from(
            runtime.list_notifications_page(&auth, query)?,
        ))
    })
    .await;
    finish_api_json(&ctx, result)
}

pub(crate) async fn get_notification(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    Path(notification_id): Path<String>,
) -> Response {
    let runtime = state.runtime.clone();
    let result: ApiResult<im_domain_core::notification::NotificationTask> = run_notification_sync(move || {
        Ok(runtime.get_notification(&auth, notification_id.as_str())?)
    })
    .await;
    finish_api_json(&ctx, result)
}
