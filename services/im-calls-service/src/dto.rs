use im_domain_core::rtc::{RtcSession, RtcSessionState};
use serde::{Deserialize, Serialize};

const RTC_SESSION_DELIVERY_PROOF_VERSION: &str = "im.call.session.delivery-proof.v1";
const RTC_SIGNAL_DELIVERY_PROOF_VERSION: &str = "im.call.signal.delivery-proof.v1";

#[derive(Clone, Debug)]
pub struct RtcSessionMutationOutcome {
    pub session: RtcSession,
    pub applied: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RtcSessionDeliveryStatus {
    Applied,
    Replayed,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RtcSessionMutationResponse {
    #[serde(flatten)]
    pub session: RtcSession,
    pub request_key: String,
    pub delivery_status: RtcSessionDeliveryStatus,
    pub proof_version: String,
}

impl RtcSessionMutationResponse {
    pub fn from_outcome(outcome: RtcSessionMutationOutcome, request_key: String) -> Self {
        Self {
            session: outcome.session,
            request_key,
            delivery_status: if outcome.applied {
                RtcSessionDeliveryStatus::Applied
            } else {
                RtcSessionDeliveryStatus::Replayed
            },
            proof_version: RTC_SESSION_DELIVERY_PROOF_VERSION.into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RtcSignalEventResponse {
    #[serde(flatten)]
    pub event: im_domain_core::rtc::RtcSignalEvent,
    pub request_key: String,
    pub delivery_status: RtcSessionDeliveryStatus,
    pub proof_version: String,
}

impl RtcSignalEventResponse {
    pub fn from_outcome(
        event: im_domain_core::rtc::RtcSignalEvent,
        applied: bool,
        request_key: String,
    ) -> Self {
        Self {
            event,
            request_key,
            delivery_status: if applied {
                RtcSessionDeliveryStatus::Applied
            } else {
                RtcSessionDeliveryStatus::Replayed
            },
            proof_version: RTC_SIGNAL_DELIVERY_PROOF_VERSION.into(),
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateRtcSessionRequest {
    pub rtc_session_id: String,
    pub conversation_id: Option<String>,
    pub rtc_mode: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InviteRtcSessionRequest {
    pub signaling_stream_id: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateRtcSessionRequest {
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

/// Map the domain `RtcSessionState` to the wire state value expected by the
/// OpenAPI contract (`started` / `accepted` / `rejected` / `ended`).
pub fn rtc_state_wire_value(state: &RtcSessionState) -> &'static str {
    state.as_wire_value()
}
