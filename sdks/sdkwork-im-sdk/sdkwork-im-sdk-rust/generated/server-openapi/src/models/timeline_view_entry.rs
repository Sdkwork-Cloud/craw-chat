use serde::{Deserialize, Serialize};

use crate::models::{MessageBody, Sender};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct TimelineViewEntry {
    #[serde(rename = "tenantId")]
    pub tenant_id: String,

    #[serde(rename = "conversationId")]
    pub conversation_id: String,

    #[serde(rename = "messageId")]
    pub message_id: String,

    #[serde(rename = "messageSeq")]
    pub message_seq: i64,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,

    pub sender: Sender,

    pub body: MessageBody,

    #[serde(rename = "messageType")]
    pub message_type: String,

    #[serde(rename = "deliveryMode")]
    pub delivery_mode: String,

    #[serde(rename = "clientMsgId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_msg_id: Option<String>,

    #[serde(rename = "streamSessionId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stream_session_id: Option<String>,

    #[serde(rename = "rtcSessionId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rtc_session_id: Option<String>,

    #[serde(rename = "occurredAt")]
    pub occurred_at: String,

    #[serde(rename = "committedAt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub committed_at: Option<String>,
}
