use axum::Json;
use axum::extract::{Extension, Path, Query, State};
use axum::http::HeaderMap;
use im_app_context::AppContext;

use crate::dto::{
    AbortStreamRequest, AppendStreamFrameRequest, CheckpointStreamRequest, CompleteStreamRequest,
    ListStreamFramesQuery, OpenStreamRequest, StreamFrameMutationResponse, StreamFrameWindow,
    StreamSessionMutationResponse,
};
use crate::error::StreamingError;
use crate::helpers::{
    ensure_standalone_stream_open_allowed, ensure_standalone_stream_session_allowed,
    resolve_request_app_context, stream_abort_request_key, stream_append_request_key,
    stream_checkpoint_request_key, stream_complete_request_key, stream_open_request_key,
};
use crate::state::AppState;

pub(crate) async fn open_stream(
    auth: Option<Extension<AppContext>>,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<OpenStreamRequest>,
) -> Result<Json<StreamSessionMutationResponse>, StreamingError> {
    let auth = resolve_request_app_context(auth, &headers)?;
    ensure_standalone_stream_open_allowed(&request)?;
    let request_key = stream_open_request_key(&auth, request.stream_id.as_str());
    Ok(Json(StreamSessionMutationResponse::from_outcome(
        state.runtime.open_stream_with_outcome(&auth, request)?,
        request_key,
    )))
}

pub(crate) async fn checkpoint_stream(
    Path(stream_id): Path<String>,
    auth: Option<Extension<AppContext>>,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<CheckpointStreamRequest>,
) -> Result<Json<StreamSessionMutationResponse>, StreamingError> {
    let auth = resolve_request_app_context(auth, &headers)?;
    ensure_standalone_stream_session_allowed(&state.runtime, &auth, stream_id.as_str())?;
    let request_key = stream_checkpoint_request_key(&auth, stream_id.as_str(), request.frame_seq);
    Ok(Json(StreamSessionMutationResponse::from_outcome(
        state
            .runtime
            .checkpoint_stream_with_outcome(&auth, stream_id.as_str(), request)?,
        request_key,
    )))
}

pub(crate) async fn append_stream_frame(
    Path(stream_id): Path<String>,
    auth: Option<Extension<AppContext>>,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<AppendStreamFrameRequest>,
) -> Result<Json<StreamFrameMutationResponse>, StreamingError> {
    let auth = resolve_request_app_context(auth, &headers)?;
    ensure_standalone_stream_session_allowed(&state.runtime, &auth, stream_id.as_str())?;
    let request_key = stream_append_request_key(&auth, stream_id.as_str(), request.frame_seq);
    Ok(Json(StreamFrameMutationResponse::from_outcome(
        state
            .runtime
            .append_frame_with_outcome(&auth, stream_id.as_str(), request)?,
        request_key,
    )))
}

pub(crate) async fn list_stream_frames(
    Path(stream_id): Path<String>,
    Query(query): Query<ListStreamFramesQuery>,
    auth: Option<Extension<AppContext>>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<StreamFrameWindow>, StreamingError> {
    let auth = resolve_request_app_context(auth, &headers)?;
    ensure_standalone_stream_session_allowed(&state.runtime, &auth, stream_id.as_str())?;
    Ok(Json(state.runtime.list_frames(
        &auth,
        stream_id.as_str(),
        query,
    )?))
}

pub(crate) async fn complete_stream(
    Path(stream_id): Path<String>,
    auth: Option<Extension<AppContext>>,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<CompleteStreamRequest>,
) -> Result<Json<StreamSessionMutationResponse>, StreamingError> {
    let auth = resolve_request_app_context(auth, &headers)?;
    ensure_standalone_stream_session_allowed(&state.runtime, &auth, stream_id.as_str())?;
    let request_key = stream_complete_request_key(&auth, stream_id.as_str());
    Ok(Json(StreamSessionMutationResponse::from_outcome(
        state
            .runtime
            .complete_stream_with_outcome(&auth, stream_id.as_str(), request)?,
        request_key,
    )))
}

pub(crate) async fn abort_stream(
    Path(stream_id): Path<String>,
    auth: Option<Extension<AppContext>>,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<AbortStreamRequest>,
) -> Result<Json<StreamSessionMutationResponse>, StreamingError> {
    let auth = resolve_request_app_context(auth, &headers)?;
    ensure_standalone_stream_session_allowed(&state.runtime, &auth, stream_id.as_str())?;
    let request_key = stream_abort_request_key(&auth, stream_id.as_str());
    Ok(Json(StreamSessionMutationResponse::from_outcome(
        state
            .runtime
            .abort_stream_with_outcome(&auth, stream_id.as_str(), request)?,
        request_key,
    )))
}
