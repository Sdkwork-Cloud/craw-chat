use axum::Json;
use axum::extract::{Extension, Path, State};
use axum::http::HeaderMap;

use im_app_context::AppContext;

use crate::dto::{
    CreateRtcSessionRequest, InviteRtcSessionRequest, IssueRtcParticipantCredentialRequest,
    PostRtcSignalRequest, RtcParticipantCredentialResponse, RtcSessionMutationResponse,
    RtcSignalEventResponse, UpdateRtcSessionRequest,
};
use crate::error::CallingError;
use crate::helpers::{
    resolve_request_app_context, rtc_session_accept_request_key, rtc_session_create_request_key,
    rtc_session_end_request_key, rtc_session_invite_request_key, rtc_session_reject_request_key,
};
use crate::state::AppState;

pub(crate) async fn create_call_session(
    auth: Option<Extension<AppContext>>,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<CreateRtcSessionRequest>,
) -> Result<Json<RtcSessionMutationResponse>, CallingError> {
    let auth = resolve_request_app_context(auth, &headers)?;
    let request_key =
        rtc_session_create_request_key(auth.tenant_id.as_str(), request.rtc_session_id.as_str());
    Ok(Json(RtcSessionMutationResponse::from_outcome(
        state
            .runtime
            .create_session_with_outcome(&auth, request)?,
        request_key,
    )))
}

pub(crate) async fn retrieve_call_session(
    Path(rtc_session_id): Path<String>,
    auth: Option<Extension<AppContext>>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<im_domain_core::rtc::RtcSession>, CallingError> {
    let auth = resolve_request_app_context(auth, &headers)?;
    Ok(Json(state.runtime.session(&auth, rtc_session_id.as_str())?))
}

pub(crate) async fn invite_call_session(
    Path(rtc_session_id): Path<String>,
    auth: Option<Extension<AppContext>>,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<InviteRtcSessionRequest>,
) -> Result<Json<RtcSessionMutationResponse>, CallingError> {
    let auth = resolve_request_app_context(auth, &headers)?;
    let request_key = rtc_session_invite_request_key(
        auth.tenant_id.as_str(),
        rtc_session_id.as_str(),
        request.signaling_stream_id.as_deref(),
    );
    Ok(Json(RtcSessionMutationResponse::from_outcome(
        state
            .runtime
            .invite_session_with_outcome(&auth, rtc_session_id.as_str(), request)?,
        request_key,
    )))
}

pub(crate) async fn accept_call_session(
    Path(rtc_session_id): Path<String>,
    auth: Option<Extension<AppContext>>,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<UpdateRtcSessionRequest>,
) -> Result<Json<RtcSessionMutationResponse>, CallingError> {
    let auth = resolve_request_app_context(auth, &headers)?;
    let request_key =
        rtc_session_accept_request_key(auth.tenant_id.as_str(), rtc_session_id.as_str());
    Ok(Json(RtcSessionMutationResponse::from_outcome(
        state
            .runtime
            .accept_session_with_outcome(&auth, rtc_session_id.as_str(), request)?,
        request_key,
    )))
}

pub(crate) async fn reject_call_session(
    Path(rtc_session_id): Path<String>,
    auth: Option<Extension<AppContext>>,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<UpdateRtcSessionRequest>,
) -> Result<Json<RtcSessionMutationResponse>, CallingError> {
    let auth = resolve_request_app_context(auth, &headers)?;
    let request_key =
        rtc_session_reject_request_key(auth.tenant_id.as_str(), rtc_session_id.as_str());
    Ok(Json(RtcSessionMutationResponse::from_outcome(
        state
            .runtime
            .reject_session_with_outcome(&auth, rtc_session_id.as_str(), request)?,
        request_key,
    )))
}

pub(crate) async fn end_call_session(
    Path(rtc_session_id): Path<String>,
    auth: Option<Extension<AppContext>>,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<UpdateRtcSessionRequest>,
) -> Result<Json<RtcSessionMutationResponse>, CallingError> {
    let auth = resolve_request_app_context(auth, &headers)?;
    let request_key = rtc_session_end_request_key(auth.tenant_id.as_str(), rtc_session_id.as_str());
    Ok(Json(RtcSessionMutationResponse::from_outcome(
        state
            .runtime
            .end_session_with_outcome(&auth, rtc_session_id.as_str(), request)?,
        request_key,
    )))
}

pub(crate) async fn post_call_signal(
    Path(rtc_session_id): Path<String>,
    auth: Option<Extension<AppContext>>,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<PostRtcSignalRequest>,
) -> Result<Json<RtcSignalEventResponse>, CallingError> {
    let auth = resolve_request_app_context(auth, &headers)?;
    let request_key = im_domain_core::rtc::encode_im_call_key_segments([
        auth.tenant_id.as_str(),
        "call.signal",
        rtc_session_id.as_str(),
    ]);
    let event = state
        .runtime
        .post_signal(&auth, rtc_session_id.as_str(), request)?;
    Ok(Json(RtcSignalEventResponse::from_outcome(
        event,
        true,
        request_key,
    )))
}

pub(crate) async fn issue_participant_credential(
    Path(rtc_session_id): Path<String>,
    auth: Option<Extension<AppContext>>,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<IssueRtcParticipantCredentialRequest>,
) -> Result<Json<RtcParticipantCredentialResponse>, CallingError> {
    let auth = resolve_request_app_context(auth, &headers)?;
    let session = state.runtime.session(&auth, rtc_session_id.as_str())?;

    // Credential issuance is gated by an authenticated principal who is either the
    // call initiator or the named participant. Without provider integration we emit
    // a deterministic, tenant-scoped credential derived from the call session state
    // so the RTC media runtime (../sdkwork-rtc) can validate it later.
    if !auth.has_permission("im.calls.credentials.issue")
        && session.initiator_id != auth.actor_id
        && request.participant_id != auth.actor_id
    {
        return Err(CallingError {
            status: axum::http::StatusCode::FORBIDDEN,
            code: "call_credential_forbidden",
            message: "principal is not authorized to issue call participant credentials".into(),
        });
    }

    let expires_at = credential_expiry_from_now();
    let credential = issue_local_call_credential(
        auth.tenant_id.as_str(),
        session.rtc_session_id.as_str(),
        request.participant_id.as_str(),
        expires_at.as_str(),
    );

    Ok(Json(RtcParticipantCredentialResponse {
        tenant_id: auth.tenant_id.clone(),
        rtc_session_id: session.rtc_session_id,
        participant_id: request.participant_id,
        credential,
        expires_at,
    }))
}

fn credential_expiry_from_now() -> String {
    const CREDENTIAL_TTL_SECS: u64 = 6 * 60 * 60;
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .unwrap_or(0);
    let expires = now + CREDENTIAL_TTL_SECS;
    format_expiry_rfc3339(expires)
}

fn format_expiry_rfc3339(unix_secs: u64) -> String {
    let (year, month, day, hour, minute, second) = civil_from_unix(unix_secs);
    format!("{year:04}-{month:02}-{day:02}T{hour:02}:{minute:02}:{second:02}Z")
}

fn civil_from_unix(secs: u64) -> (i32, u32, u32, u32, u32, u32) {
    let days = (secs / 86_400) as i64;
    let remainder = (secs % 86_400) as u32;
    let hour = remainder / 3600;
    let minute = (remainder % 3600) / 60;
    let second = remainder % 60;

    let mut days = days - 719_468;
    let era = if days >= 0 { days } else { days - 146_096 } / 146_097;
    let doe = (days - era * 146_097) as i64;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let m = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
    let year = if m <= 2 { y + 1 } else { y } as i32;
    days = 719_468 + era * 146_097 + yoe * 365 + yoe / 4 - yoe / 100 + doy;
    let _ = days;
    (year, m, d, hour, minute, second)
}

fn issue_local_call_credential(
    tenant_id: &str,
    rtc_session_id: &str,
    participant_id: &str,
    expires_at: &str,
) -> String {
    use sdkwork_utils_rust::hmac_sha256_base64url;
    let secret = std::env::var("SDKWORK_IM_APP_CONTEXT_SIGNATURE_SECRET")
        .ok()
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| "sdkwork-im-local-call-credential-secret".to_owned());
    let payload = format!("{tenant_id}|{rtc_session_id}|{participant_id}|{expires_at}");
    let signature = hmac_sha256_base64url(payload.as_bytes(), secret.as_bytes());
    format!("im.call.credential.v1.{signature}")
}