//! Axum handlers for the IM call signaling service.
//!
//! All handlers return the canonical [`SdkWorkApiResponse`] envelope via
//! [`finish_api_json`] and emit [`ApiProblem`] errors (`application/problem+json`
//! with numeric `code`) per `API_SPEC.md` §4.5, §14, and §15.
//!
//! Handlers extract [`WebRequestContext`] (injected by `WebFrameworkLayer` via
//! `ImAppContextInjector`) for trace correlation and [`AppContext`] for the
//! authenticated principal. Routes are `dual_token`, so the framework rejects
//! unauthenticated requests before they reach these handlers.

use axum::Json;
use axum::extract::{Extension, Path, Query, State};
use axum::response::Response;
use sdkwork_utils_rust::cursor_list_page_data;
use serde::Deserialize;

use im_app_context::AppContext;
use sdkwork_routes_web_framework_backend_api::response::{ApiProblem, ApiResult, finish_api_json};
use sdkwork_web_core::WebRequestContext;

use crate::dto::{
    CreateRtcSessionRequest, InviteRtcSessionRequest, IssueRtcParticipantCredentialRequest,
    PostRtcSignalRequest, RtcParticipantCredentialResponse, RtcSignalEventResponse,
    SessionMutationResponse, UpdateRtcSessionRequest,
};
use crate::helpers::{
    rtc_session_accept_request_key, rtc_session_create_request_key, rtc_session_end_request_key,
    rtc_session_invite_request_key, rtc_session_reject_request_key,
};
use crate::state::AppState;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ListRtcSignalsQuery {
    pub after_signal_seq: Option<u64>,
    pub page_size: Option<usize>,
    pub cursor: Option<String>,
}

fn map_blocking_join_error(error: tokio::task::JoinError) -> ApiProblem {
    ApiProblem::internal_server_error(format!("call_runtime_blocking_join_failed: {error}"))
}

/// Run Postgres/Redis-backed RTC runtime work off the Tokio async worker pool.
async fn run_blocking_call<F, T>(state: AppState, operation: F) -> ApiResult<T>
where
    F: FnOnce(AppState) -> ApiResult<T> + Send + 'static,
    T: Send + 'static,
{
    tokio::task::spawn_blocking(move || operation(state))
        .await
        .map_err(map_blocking_join_error)?
}

pub(crate) async fn create_call_session(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    Json(request): Json<CreateRtcSessionRequest>,
) -> Response {
    let request_key =
        rtc_session_create_request_key(auth.tenant_id.as_str(), request.rtc_session_id.as_str());
    let result = run_blocking_call(state, move |state| {
        let outcome = state.runtime.create_session_with_outcome(&auth, request)?;
        Ok(SessionMutationResponse::from_outcome(outcome, request_key))
    })
    .await;
    finish_api_json(&ctx, result)
}

pub(crate) async fn retrieve_call_session(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    Path(rtc_session_id): Path<String>,
    State(state): State<AppState>,
) -> Response {
    let result = run_blocking_call(state, move |state| {
        state
            .runtime
            .session(&auth, rtc_session_id.as_str())
            .map_err(ApiProblem::from)
    })
    .await;
    finish_api_json(&ctx, result)
}

pub(crate) async fn list_call_signals(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    Path(rtc_session_id): Path<String>,
    Query(query): Query<ListRtcSignalsQuery>,
    State(state): State<AppState>,
) -> Response {
    let page_size = query.page_size.unwrap_or(20).clamp(1, 200);
    let after_signal_seq = query
        .after_signal_seq
        .or_else(|| query.cursor.as_deref().and_then(|value| value.parse().ok()));
    let result = run_blocking_call(state, move |state| {
        let (items, has_more) = state.runtime.list_signals(
            &auth,
            rtc_session_id.as_str(),
            after_signal_seq,
            Some(page_size),
        )?;
        let next_cursor = has_more
            .then(|| items.last().map(|event| event.signal_seq.to_string()))
            .flatten();
        Ok(cursor_list_page_data(
            items,
            page_size,
            next_cursor,
            has_more,
        ))
    })
    .await;
    finish_api_json(&ctx, result)
}

pub(crate) async fn invite_call_session(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    Path(rtc_session_id): Path<String>,
    State(state): State<AppState>,
    Json(request): Json<InviteRtcSessionRequest>,
) -> Response {
    let request_key = rtc_session_invite_request_key(
        auth.tenant_id.as_str(),
        rtc_session_id.as_str(),
        request.signaling_stream_id.as_deref(),
    );
    let result = run_blocking_call(state, move |state| {
        let outcome =
            state
                .runtime
                .invite_session_with_outcome(&auth, rtc_session_id.as_str(), request)?;
        Ok(SessionMutationResponse::from_outcome(outcome, request_key))
    })
    .await;
    finish_api_json(&ctx, result)
}

pub(crate) async fn accept_call_session(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    Path(rtc_session_id): Path<String>,
    State(state): State<AppState>,
    Json(request): Json<UpdateRtcSessionRequest>,
) -> Response {
    let request_key =
        rtc_session_accept_request_key(auth.tenant_id.as_str(), rtc_session_id.as_str());
    let result = run_blocking_call(state, move |state| {
        let outcome =
            state
                .runtime
                .accept_session_with_outcome(&auth, rtc_session_id.as_str(), request)?;
        Ok(SessionMutationResponse::from_outcome(outcome, request_key))
    })
    .await;
    finish_api_json(&ctx, result)
}

pub(crate) async fn reject_call_session(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    Path(rtc_session_id): Path<String>,
    State(state): State<AppState>,
    Json(request): Json<UpdateRtcSessionRequest>,
) -> Response {
    let request_key =
        rtc_session_reject_request_key(auth.tenant_id.as_str(), rtc_session_id.as_str());
    let result = run_blocking_call(state, move |state| {
        let outcome =
            state
                .runtime
                .reject_session_with_outcome(&auth, rtc_session_id.as_str(), request)?;
        Ok(SessionMutationResponse::from_outcome(outcome, request_key))
    })
    .await;
    finish_api_json(&ctx, result)
}

pub(crate) async fn end_call_session(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    Path(rtc_session_id): Path<String>,
    State(state): State<AppState>,
    Json(request): Json<UpdateRtcSessionRequest>,
) -> Response {
    let request_key = rtc_session_end_request_key(auth.tenant_id.as_str(), rtc_session_id.as_str());
    let result = run_blocking_call(state, move |state| {
        let outcome =
            state
                .runtime
                .end_session_with_outcome(&auth, rtc_session_id.as_str(), request)?;
        Ok(SessionMutationResponse::from_outcome(outcome, request_key))
    })
    .await;
    finish_api_json(&ctx, result)
}

pub(crate) async fn post_call_signal(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    Path(rtc_session_id): Path<String>,
    State(state): State<AppState>,
    Json(request): Json<PostRtcSignalRequest>,
) -> Response {
    let request_key = im_domain_core::rtc::encode_im_call_key_segments([
        auth.tenant_id.as_str(),
        "call.signal",
        rtc_session_id.as_str(),
    ]);
    let result = run_blocking_call(state, move |state| {
        let event = state
            .runtime
            .post_signal(&auth, rtc_session_id.as_str(), request)?;
        Ok(RtcSignalEventResponse::from_outcome(
            event,
            true,
            request_key,
        ))
    })
    .await;
    finish_api_json(&ctx, result)
}

pub(crate) async fn issue_participant_credential(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    Path(rtc_session_id): Path<String>,
    State(state): State<AppState>,
    Json(request): Json<IssueRtcParticipantCredentialRequest>,
) -> Response {
    let result = run_blocking_call(state, move |state| {
        let session = state.runtime.session(&auth, rtc_session_id.as_str())?;

        if session.state.is_terminal() {
            return Err(ApiProblem::bad_request(format!(
                "call session is in terminal state {}; credentials cannot be issued: {rtc_session_id}",
                session.state.as_str()
            )));
        }

        let is_initiator =
            session.initiator_id == auth.actor_id && session.initiator_kind == auth.actor_kind;
        let has_admin_permission = auth.has_permission("im.calls.credentials.issue");
        let is_invited_or_accepted_self = request.participant_id == auth.actor_id
            && (session.participants.invited_ids.contains(&auth.actor_id)
                || session.participants.accepted_ids.contains(&auth.actor_id))
            && (auth.actor_kind == session.initiator_kind || auth.actor_kind == "user");
        if !is_initiator && !has_admin_permission && !is_invited_or_accepted_self {
            return Err(ApiProblem::forbidden(
                "principal is not authorized to issue call participant credentials",
            ));
        }

        let credential = state.runtime.issue_participant_credential(
            &auth,
            rtc_session_id.as_str(),
            request.participant_id.as_str(),
        )?;

        Ok(RtcParticipantCredentialResponse {
            tenant_id: credential.tenant_id,
            rtc_session_id: credential.rtc_session_id,
            participant_id: credential.participant_id,
            credential: credential.credential,
            expires_at: credential.expires_at,
        })
    })
    .await;
    finish_api_json(&ctx, result)
}

/// Refresh an expiring participant credential.
pub(crate) async fn refresh_participant_credential(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    Path(rtc_session_id): Path<String>,
    State(state): State<AppState>,
    Json(request): Json<IssueRtcParticipantCredentialRequest>,
) -> Response {
    let result = run_blocking_call(state, move |state| {
        let credential = state.runtime.refresh_participant_credential(
            &auth,
            rtc_session_id.as_str(),
            request.participant_id.as_str(),
        )?;
        Ok(RtcParticipantCredentialResponse {
            tenant_id: credential.tenant_id,
            rtc_session_id: credential.rtc_session_id,
            participant_id: credential.participant_id,
            credential: credential.credential,
            expires_at: credential.expires_at,
        })
    })
    .await;
    finish_api_json(&ctx, result)
}
