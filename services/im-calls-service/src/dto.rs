use im_domain_core::rtc::{RtcSession, RtcSessionState, RtcSignalEvent};
use serde::{Deserialize, Serialize};

const RTC_SESSION_DELIVERY_PROOF_VERSION: &str = "im.call.session.delivery-proof.v1";
const RTC_SIGNAL_DELIVERY_PROOF_VERSION: &str = "im.call.signal.delivery-proof.v1";

#[derive(Clone, Debug)]
pub struct SessionMutationOutcome {
    pub session: RtcSession,
    pub applied: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SessionDeliveryStatus {
    Applied,
    Replayed,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionMutationResponse {
    #[serde(flatten)]
    pub session: RtcSession,
    pub request_key: String,
    pub delivery_status: SessionDeliveryStatus,
    pub proof_version: String,
}

impl SessionMutationResponse {
    pub fn from_outcome(outcome: SessionMutationOutcome, request_key: String) -> Self {
        Self {
            session: outcome.session,
            request_key,
            delivery_status: if outcome.applied {
                SessionDeliveryStatus::Applied
            } else {
                SessionDeliveryStatus::Replayed
            },
            proof_version: RTC_SESSION_DELIVERY_PROOF_VERSION.into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RtcSignalEventResponse {
    #[serde(flatten)]
    pub event: RtcSignalEvent,
    pub request_key: String,
    pub delivery_status: SessionDeliveryStatus,
    pub proof_version: String,
}

impl RtcSignalEventResponse {
    pub fn from_outcome(
        event: RtcSignalEvent,
        applied: bool,
        request_key: String,
    ) -> Self {
        Self {
            event,
            request_key,
            delivery_status: if applied {
                SessionDeliveryStatus::Applied
            } else {
                SessionDeliveryStatus::Replayed
            },
            proof_version: RTC_SIGNAL_DELIVERY_PROOF_VERSION.into(),
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateSessionRequest {
    pub rtc_session_id: String,
    pub conversation_id: Option<String>,
    pub rtc_mode: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InviteSessionRequest {
    pub signaling_stream_id: Option<String>,
    /// Principal IDs invited to the call session. These are recorded in the
    /// session's `invited_ids` list so subsequent `accept`/`reject`/`end`
    /// authorization checks can admit them. Omitting this field preserves
    /// legacy behavior (invite only establishes the signaling stream).
    #[serde(default)]
    pub participant_ids: Vec<String>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateSessionRequest {
    pub artifact_message_id: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostRtcSignalRequest {
    pub signal_type: String,
    pub schema_ref: Option<String>,
    pub payload: String,
    pub signaling_stream_id: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IssueRtcParticipantCredentialRequest {
    pub participant_id: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RtcParticipantCredentialResponse {
    pub tenant_id: String,
    pub rtc_session_id: String,
    pub participant_id: String,
    pub credential: String,
    pub expires_at: String,
}

pub type CreateRtcSessionRequest = CreateSessionRequest;
pub type InviteRtcSessionRequest = InviteSessionRequest;
pub type UpdateRtcSessionRequest = UpdateSessionRequest;
pub type RtcSessionDeliveryStatus = SessionDeliveryStatus;
pub type RtcSessionMutationOutcome = SessionMutationOutcome;
pub type RtcSessionMutationResponse = SessionMutationResponse;

/// Map the domain `SessionState` to the wire state value expected by the
/// OpenAPI contract (`started` / `accepted` / `rejected` / `ended`).
pub fn rtc_state_wire_value(state: &RtcSessionState) -> &'static str {
    state.as_wire_value()
}
