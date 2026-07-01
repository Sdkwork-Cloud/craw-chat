use axum::Json;
use axum::extract::{Extension, Path, Query, State};
use axum::response::Response;
use im_app_context::AppContext;
use sdkwork_routes_web_framework_backend_api::response::{ApiResult, finish_api_json};
use sdkwork_web_core::WebRequestContext;

use crate::dto::{
    AbortStreamRequest, AppendStreamFrameRequest, CheckpointStreamRequest, CompleteStreamRequest,
    ListStreamFramesQuery, OpenStreamRequest, StreamFrameMutationResponse, StreamFrameWindow,
    StreamSessionMutationResponse,
};
use crate::helpers::{
    ensure_standalone_stream_open_allowed, ensure_standalone_stream_session_allowed,
    stream_abort_request_key, stream_append_request_key, stream_checkpoint_request_key,
    stream_complete_request_key, stream_open_request_key,
};
use crate::state::AppState;

pub(crate) async fn open_stream(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    Json(request): Json<OpenStreamRequest>,
) -> Response {
    let result: ApiResult<StreamSessionMutationResponse> = (|| {
        ensure_standalone_stream_open_allowed(&request)?;
        let request_key = stream_open_request_key(&auth, request.stream_id.as_str());
        Ok(StreamSessionMutationResponse::from_outcome(
            state.runtime.open_stream_with_outcome(&auth, request)?,
            request_key,
        ))
    })();
    finish_api_json(&ctx, result)
}

pub(crate) async fn checkpoint_stream(
    Path(stream_id): Path<String>,
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    Json(request): Json<CheckpointStreamRequest>,
) -> Response {
    let result: ApiResult<StreamSessionMutationResponse> = (|| {
        ensure_standalone_stream_session_allowed(&state.runtime, &auth, stream_id.as_str())?;
        let request_key =
            stream_checkpoint_request_key(&auth, stream_id.as_str(), request.frame_seq);
        Ok(StreamSessionMutationResponse::from_outcome(
            state
                .runtime
                .checkpoint_stream_with_outcome(&auth, stream_id.as_str(), request)?,
            request_key,
        ))
    })();
    finish_api_json(&ctx, result)
}

pub(crate) async fn append_stream_frame(
    Path(stream_id): Path<String>,
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    Json(request): Json<AppendStreamFrameRequest>,
) -> Response {
    let result: ApiResult<StreamFrameMutationResponse> = (|| {
        ensure_standalone_stream_session_allowed(&state.runtime, &auth, stream_id.as_str())?;
        let request_key =
            stream_append_request_key(&auth, stream_id.as_str(), request.frame_seq);
        Ok(StreamFrameMutationResponse::from_outcome(
            state
                .runtime
                .append_frame_with_outcome(&auth, stream_id.as_str(), request)?,
            request_key,
        ))
    })();
    finish_api_json(&ctx, result)
}

pub(crate) async fn list_stream_frames(
    Path(stream_id): Path<String>,
    Query(query): Query<ListStreamFramesQuery>,
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
) -> Response {
    let result: ApiResult<StreamFrameWindow> = (|| {
        ensure_standalone_stream_session_allowed(&state.runtime, &auth, stream_id.as_str())?;
        Ok(state
            .runtime
            .list_frames(&auth, stream_id.as_str(), query)?)
    })();
    finish_api_json(&ctx, result)
}

pub(crate) async fn complete_stream(
    Path(stream_id): Path<String>,
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    Json(request): Json<CompleteStreamRequest>,
) -> Response {
    let result: ApiResult<StreamSessionMutationResponse> = (|| {
        ensure_standalone_stream_session_allowed(&state.runtime, &auth, stream_id.as_str())?;
        let request_key = stream_complete_request_key(&auth, stream_id.as_str());
        Ok(StreamSessionMutationResponse::from_outcome(
            state
                .runtime
                .complete_stream_with_outcome(&auth, stream_id.as_str(), request)?,
            request_key,
        ))
    })();
    finish_api_json(&ctx, result)
}

pub(crate) async fn abort_stream(
    Path(stream_id): Path<String>,
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    Json(request): Json<AbortStreamRequest>,
) -> Response {
    let result: ApiResult<StreamSessionMutationResponse> = (|| {
        ensure_standalone_stream_session_allowed(&state.runtime, &auth, stream_id.as_str())?;
        let request_key = stream_abort_request_key(&auth, stream_id.as_str());
        Ok(StreamSessionMutationResponse::from_outcome(
            state
                .runtime
                .abort_stream_with_outcome(&auth, stream_id.as_str(), request)?,
            request_key,
        ))
    })();
    finish_api_json(&ctx, result)
}
