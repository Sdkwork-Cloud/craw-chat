//! Shared SdkWorkApiResponse serialization and async/blocking bridges
//! for social HTTP handlers.
//!
//! The social runtime performs synchronous file I/O (cross-instance file
//! locks, commit journal replay, state snapshots) and may invoke synchronous
//! Postgres adapters. To avoid starving the Tokio multi-threaded runtime,
//! handlers MUST route this work through [`run_blocking_social_call`] so it
//! runs on the dedicated `spawn_blocking` thread pool instead of an async
//! worker thread. This mirrors the pattern established by
//! `im-calls-service::handlers::run_blocking_call` and is mandated by
//! `RUST_CODE_SPEC.md §6` ("Do not hold locks across `.await`").

use axum::response::Response;
use sdkwork_routes_web_framework_backend_api::response::{
    created_json, finish_api_json, finish_api_response, no_content,
};
use sdkwork_web_core::WebRequestContext;
use serde::Serialize;

use crate::friendship::{AppState, SocialServiceError};

pub(crate) fn finish_enveloped_json<T: Serialize>(
    ctx: &WebRequestContext,
    result: Result<T, SocialServiceError>,
) -> Response {
    finish_api_json(ctx, result.map_err(Into::into))
}

pub(crate) fn finish_created_enveloped_json<T: Serialize>(
    ctx: &WebRequestContext,
    result: Result<T, SocialServiceError>,
) -> Response {
    finish_api_response(
        ctx,
        result
            .map_err(Into::into)
            .and_then(|data| created_json(ctx, data)),
    )
}

pub(crate) fn finish_no_content(
    ctx: &WebRequestContext,
    result: Result<(), SocialServiceError>,
) -> Response {
    finish_api_response(
        ctx,
        result.map_err(Into::into).and_then(|_| no_content(ctx)),
    )
}

/// Run a synchronous social-runtime operation off the Tokio async worker
/// pool.
///
/// `state` is moved into the closure so the closure can borrow
/// `state.social_runtime` synchronously on the blocking thread. The closure
/// must not retain any references into the async stack — owned values
/// (`AppContext`, request DTOs) should be captured by move.
///
/// Returns `Result<T, SocialServiceError>` so handlers can pipe the result
/// directly into [`finish_enveloped_json`] or [`crate::openapi::finish_open_api_json`].
pub(crate) async fn run_blocking_social_call<F, T>(
    state: AppState,
    operation: F,
) -> Result<T, SocialServiceError>
where
    F: FnOnce(AppState) -> Result<T, SocialServiceError> + Send + 'static,
    T: Send + 'static,
{
    tokio::task::spawn_blocking(move || operation(state))
        .await
        .map_err(SocialServiceError::blocking_join_failed)?
}
