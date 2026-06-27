use serde::{Deserialize, Serialize};

use crate::message::{MessageAttributes, Sender};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum StreamDurabilityClass {
    Transient,
    DurableSession,
    EventLog,
}

impl StreamDurabilityClass {
    pub fn as_wire_value(&self) -> &'static str {
        match self {
            Self::Transient => "transient",
            Self::DurableSession => "durable_session",
            Self::EventLog => "event_log",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StreamSessionState {
    Created,
    Opened,
    Active,
    Checkpointed,
    Completed,
    Aborted,
    Expired,
}

impl StreamSessionState {
    pub fn as_wire_value(&self) -> &'static str {
        match self {
            Self::Created => "created",
            Self::Opened => "opened",
            Self::Active => "active",
            Self::Checkpointed => "checkpointed",
            Self::Completed => "completed",
            Self::Aborted => "aborted",
            Self::Expired => "expired",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StreamSession {
    pub tenant_id: String,
    pub stream_id: String,
    pub owner_principal_id: String,
    pub owner_principal_kind: String,
    pub stream_type: String,
    pub scope_kind: String,
    pub scope_id: String,
    pub durability_class: StreamDurabilityClass,
    pub ordering_scope: String,
    pub schema_ref: Option<String>,
    pub state: StreamSessionState,
    pub last_frame_seq: u64,
    pub last_checkpoint_seq: Option<u64>,
    pub result_message_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub complete_frame_seq: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub abort_frame_seq: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub abort_reason: Option<String>,
    pub opened_at: String,
    pub closed_at: Option<String>,
    pub expires_at: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StreamFrame {
    pub tenant_id: String,
    pub stream_id: String,
    pub stream_type: String,
    pub scope_kind: String,
    pub scope_id: String,
    pub frame_seq: u64,
    pub frame_type: String,
    pub schema_ref: Option<String>,
    pub encoding: String,
    pub payload: String,
    pub sender: Sender,
    pub attributes: MessageAttributes,
    pub occurred_at: String,
}
