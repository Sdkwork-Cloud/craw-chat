use axum::Json;
use axum::extract::{Extension, Path, Query, State};
use axum::response::Response;
use im_app_context::AppContext;
use sdkwork_routes_web_framework_backend_api::response::{ApiResult, finish_api_json};
use sdkwork_utils_rust::{MAX_LIST_PAGE_SIZE, SdkWorkCursorListQuery, SdkWorkPageData};
use sdkwork_web_core::WebRequestContext;

use crate::dto::{
    AbortStreamRequest, AppendStreamFrameRequest, CheckpointStreamRequest, CompleteStreamRequest,
    OpenStreamRequest, StreamFrameMutationResponse, StreamSessionMutationResponse,
};
use crate::error::StreamingError;
use crate::helpers::{
    ensure_standalone_stream_open_allowed, ensure_standalone_stream_session_allowed,
    stream_abort_request_key, stream_append_request_key, stream_checkpoint_request_key,
    stream_complete_request_key, stream_open_request_key,
};
use crate::state::AppState;

async fn run_streaming_sync<F, T>(operation: F) -> ApiResult<T>
where
    F: FnOnce() -> ApiResult<T> + Send + 'static,
    T: Send + 'static,
{
    tokio::task::spawn_blocking(operation)
        .await
        .unwrap_or_else(|join_error| {
            Err(StreamingError {
                status: axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                code: "stream_runtime_worker_failed",
                message: format!("stream runtime worker failed: {join_error}"),
            }
            .into())
        })
}

pub(crate) async fn open_stream(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    Json(request): Json<OpenStreamRequest>,
) -> Response {
    let runtime = state.runtime.clone();
    let result = run_streaming_sync(move || {
        ensure_standalone_stream_open_allowed(&request)?;
        let request_key = stream_open_request_key(&auth, request.stream_id.as_str());
        Ok(StreamSessionMutationResponse::from_outcome(
            runtime.open_stream_with_outcome(&auth, request)?,
            request_key,
        ))
    })
    .await;
    finish_api_json(&ctx, result)
}

pub(crate) async fn checkpoint_stream(
    Path(stream_id): Path<String>,
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    Json(request): Json<CheckpointStreamRequest>,
) -> Response {
    let runtime = state.runtime.clone();
    let result = run_streaming_sync(move || {
        ensure_standalone_stream_session_allowed(&runtime, &auth, stream_id.as_str())?;
        let request_key =
            stream_checkpoint_request_key(&auth, stream_id.as_str(), request.frame_seq);
        Ok(StreamSessionMutationResponse::from_outcome(
            runtime.checkpoint_stream_with_outcome(&auth, stream_id.as_str(), request)?,
            request_key,
        ))
    })
    .await;
    finish_api_json(&ctx, result)
}

pub(crate) async fn append_stream_frame(
    Path(stream_id): Path<String>,
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    Json(request): Json<AppendStreamFrameRequest>,
) -> Response {
    let runtime = state.runtime.clone();
    let result = run_streaming_sync(move || {
        ensure_standalone_stream_session_allowed(&runtime, &auth, stream_id.as_str())?;
        let request_key = stream_append_request_key(&auth, stream_id.as_str(), request.frame_seq);
        Ok(StreamFrameMutationResponse::from_outcome(
            runtime.append_frame_with_outcome(&auth, stream_id.as_str(), request)?,
            request_key,
        ))
    })
    .await;
    finish_api_json(&ctx, result)
}

pub(crate) async fn list_stream_frames(
    Path(stream_id): Path<String>,
    Query(query): Query<SdkWorkCursorListQuery>,
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
) -> Response {
    let runtime = state.runtime.clone();
    let result = run_streaming_sync(move || {
        ensure_standalone_stream_session_allowed(&runtime, &auth, stream_id.as_str())?;
        if let Some(raw_page_size) = query.page_size {
            if !(1..=MAX_LIST_PAGE_SIZE).contains(&raw_page_size) {
                Err(StreamingError {
                    status: axum::http::StatusCode::BAD_REQUEST,
                    code: "page_size_invalid",
                    message: format!("page_size must be between 1 and {MAX_LIST_PAGE_SIZE}"),
                })?;
            }
        }
        let paging = query.resolve().map_err(|_| StreamingError {
            status: axum::http::StatusCode::BAD_REQUEST,
            code: "cursor_invalid",
            message: "cursor must encode a non-negative frame sequence".into(),
        })?;
        let after_frame_seq = u64::try_from(paging.offset).map_err(|_| StreamingError {
            status: axum::http::StatusCode::BAD_REQUEST,
            code: "cursor_invalid",
            message: "cursor must encode a non-negative frame sequence".into(),
        })?;
        let page_size = paging.page_size;
        Ok(runtime.list_frames(&auth, stream_id.as_str(), after_frame_seq, page_size)?)
    })
    .await;
    finish_api_json(&ctx, result)
}

pub(crate) async fn complete_stream(
    Path(stream_id): Path<String>,
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    Json(request): Json<CompleteStreamRequest>,
) -> Response {
    let runtime = state.runtime.clone();
    let result = run_streaming_sync(move || {
        ensure_standalone_stream_session_allowed(&runtime, &auth, stream_id.as_str())?;
        let request_key = stream_complete_request_key(&auth, stream_id.as_str());
        Ok(StreamSessionMutationResponse::from_outcome(
            runtime.complete_stream_with_outcome(&auth, stream_id.as_str(), request)?,
            request_key,
        ))
    })
    .await;
    finish_api_json(&ctx, result)
}

pub(crate) async fn abort_stream(
    Path(stream_id): Path<String>,
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    Json(request): Json<AbortStreamRequest>,
) -> Response {
    let runtime = state.runtime.clone();
    let result = run_streaming_sync(move || {
        ensure_standalone_stream_session_allowed(&runtime, &auth, stream_id.as_str())?;
        let request_key = stream_abort_request_key(&auth, stream_id.as_str());
        Ok(StreamSessionMutationResponse::from_outcome(
            runtime.abort_stream_with_outcome(&auth, stream_id.as_str(), request)?,
            request_key,
        ))
    })
    .await;
    finish_api_json(&ctx, result)
}
