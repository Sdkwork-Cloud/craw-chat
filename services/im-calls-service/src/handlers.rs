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
use axum::extract::{Extension, Path, State};
use axum::response::Response;

use im_app_context::AppContext;
use sdkwork_routes_web_framework_backend_api::response::{
    finish_api_json, ApiProblem, ApiResult,
};
use sdkwork_web_core::WebRequestContext;

use crate::dto::{
    CreateRtcSessionRequest, InviteRtcSessionRequest, IssueRtcParticipantCredentialRequest,
    PostRtcSignalRequest, RtcParticipantCredentialResponse, SessionMutationResponse,
    RtcSignalEventResponse, UpdateRtcSessionRequest,
};
use crate::helpers::{
    rtc_session_accept_request_key, rtc_session_create_request_key, rtc_session_end_request_key,
    rtc_session_invite_request_key, rtc_session_reject_request_key,
};
use crate::state::AppState;

pub(crate) async fn create_call_session(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    Json(request): Json<CreateRtcSessionRequest>,
) -> Response {
    let request_key =
        rtc_session_create_request_key(auth.tenant_id.as_str(), request.rtc_session_id.as_str());
    let result: ApiResult<SessionMutationResponse> = (|| {
        let outcome = state
            .runtime
            .create_session_with_outcome(&auth, request)?;
        Ok(SessionMutationResponse::from_outcome(outcome, request_key))
    })();
    finish_api_json(&ctx, result)
}

pub(crate) async fn retrieve_call_session(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    Path(rtc_session_id): Path<String>,
    State(state): State<AppState>,
) -> Response {
    let result: ApiResult<im_domain_core::rtc::RtcSession> = state
        .runtime
        .session(&auth, rtc_session_id.as_str())
        .map_err(ApiProblem::from);
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
    let result: ApiResult<SessionMutationResponse> = (|| {
        let outcome = state
            .runtime
            .invite_session_with_outcome(&auth, rtc_session_id.as_str(), request)?;
        Ok(SessionMutationResponse::from_outcome(outcome, request_key))
    })();
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
    let result: ApiResult<SessionMutationResponse> = (|| {
        let outcome = state
            .runtime
            .accept_session_with_outcome(&auth, rtc_session_id.as_str(), request)?;
        Ok(SessionMutationResponse::from_outcome(outcome, request_key))
    })();
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
    let result: ApiResult<SessionMutationResponse> = (|| {
        let outcome = state
            .runtime
            .reject_session_with_outcome(&auth, rtc_session_id.as_str(), request)?;
        Ok(SessionMutationResponse::from_outcome(outcome, request_key))
    })();
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
    let result: ApiResult<SessionMutationResponse> = (|| {
        let outcome = state
            .runtime
            .end_session_with_outcome(&auth, rtc_session_id.as_str(), request)?;
        Ok(SessionMutationResponse::from_outcome(outcome, request_key))
    })();
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
    let result: ApiResult<RtcSignalEventResponse> = (|| {
        let event = state
            .runtime
            .post_signal(&auth, rtc_session_id.as_str(), request)?;
        Ok(RtcSignalEventResponse::from_outcome(event, true, request_key))
    })();
    finish_api_json(&ctx, result)
}

pub(crate) async fn issue_participant_credential(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    Path(rtc_session_id): Path<String>,
    State(state): State<AppState>,
    Json(request): Json<IssueRtcParticipantCredentialRequest>,
) -> Response {
    let result: ApiResult<RtcParticipantCredentialResponse> = (|| {
        let session = state.runtime.session(&auth, rtc_session_id.as_str())?;

        // Reject credential issuance for terminal sessions. A media session that
        // has ended, been rejected, canceled, timed out, or failed must not admit
        // new participants; providers would otherwise hand out valid join tokens
        // for rooms that no longer exist or are tearing down.
        if session.state.is_terminal() {
            return Err(ApiProblem::bad_request(format!(
                "call session is in terminal state {}; credentials cannot be issued: {rtc_session_id}",
                session.state.as_str()
            )));
        }

        // Credential issuance is gated by an authenticated principal who is:
        //   - the call initiator, OR
        //   - a principal holding the `im.calls.credentials.issue` permission
        //     (operator/admin), OR
        //   - a participant who has been explicitly invited AND is requesting
        //     their own credential (request.participant_id == auth.actor_id).
        // The previous check only verified `participant_id == auth.actor_id`,
        // which allowed any authenticated user who knew the `rtc_session_id` to
        // mint a join credential for themselves, bypassing the invite flow.
        let is_initiator = session.initiator_id == auth.actor_id
            && session.initiator_kind == auth.actor_kind;
        let has_admin_permission = auth.has_permission("im.calls.credentials.issue");
        let is_invited_self = request.participant_id == auth.actor_id
            && session.participants.invited_ids.contains(&auth.actor_id);
        if !is_initiator && !has_admin_permission && !is_invited_self {
            return Err(ApiProblem::forbidden(
                "principal is not authorized to issue call participant credentials",
            ));
        }

        // Delegate to the RTC provider to obtain a real provider-issued
        // credential. The provider owns token generation and expiry semantics.
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
    })();
    finish_api_json(&ctx, result)
}

/// Refresh an expiring participant credential.
///
/// The runtime enforces the same authorization, participant-membership, and
/// non-terminal-state guards as `issue_participant_credential`. The provider
/// issues a fresh credential with a new `expires_at`, extending media access
/// without requiring the participant to re-join the session.
pub(crate) async fn refresh_participant_credential(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    Path(rtc_session_id): Path<String>,
    State(state): State<AppState>,
    Json(request): Json<IssueRtcParticipantCredentialRequest>,
) -> Response {
    let result: ApiResult<RtcParticipantCredentialResponse> = (|| {
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
    })();
    finish_api_json(&ctx, result)
}
