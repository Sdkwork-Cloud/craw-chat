use sdkwork_im_ccp_core::{CapabilitySet, ProtocolVersion, TransportBinding};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct HelloFrame {
    pub protocol: ProtocolVersion,
    pub binding: TransportBinding,
    pub capabilities: CapabilitySet,
    pub trace_id: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct HelloAckFrame {
    pub protocol: ProtocolVersion,
    pub binding: TransportBinding,
    pub capabilities: CapabilitySet,
    pub accepted: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthBindFrame {
    pub principal_id: String,
    pub device_id: Option<String>,
    pub session_id: Option<String>,
    pub actor_kind: String,
    /// Bearer credential for transports without an HTTP upgrade layer (TCP/UDP).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auth_token: Option<String>,
    /// Access token header value for dual-token app context resolution on link transports.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthOkFrame {
    pub tenant_id: String,
    pub principal_id: String,
    pub actor_kind: String,
    pub device_id: Option<String>,
    pub session_id: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SessionResumeFrame {
    pub session_id: String,
    pub last_acked_seq: Option<u64>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SessionResumedFrame {
    pub session_id: String,
    pub resumed: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct HeartbeatFrame {
    pub sequence: Option<u64>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GoAwayFrame {
    pub code: String,
    pub message: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ErrorFrame {
    pub code: String,
    pub message: String,
    pub retryable: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum ControlFrame {
    Hello(HelloFrame),
    HelloAck(HelloAckFrame),
    AuthBind(AuthBindFrame),
    AuthOk(AuthOkFrame),
    SessionResume(SessionResumeFrame),
    SessionResumed(SessionResumedFrame),
    Heartbeat(HeartbeatFrame),
    GoAway(GoAwayFrame),
    Error(ErrorFrame),
}

impl ControlFrame {
    pub fn frame_type(&self) -> &'static str {
        match self {
            Self::Hello(_) => "hello",
            Self::HelloAck(_) => "hello_ack",
            Self::AuthBind(_) => "auth_bind",
            Self::AuthOk(_) => "auth_ok",
            Self::SessionResume(_) => "session_resume",
            Self::SessionResumed(_) => "session_resumed",
            Self::Heartbeat(_) => "heartbeat",
            Self::GoAway(_) => "goaway",
            Self::Error(_) => "error",
        }
    }
}
