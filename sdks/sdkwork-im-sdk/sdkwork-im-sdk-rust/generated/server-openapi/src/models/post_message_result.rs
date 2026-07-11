use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct PostMessageResult {
    #[serde(rename = "messageId")]
    pub message_id: String,

    #[serde(rename = "messageSeq")]
    pub message_seq: i64,

    #[serde(rename = "eventId")]
    pub event_id: String,

    #[serde(rename = "requestKey")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_key: Option<String>,

    #[serde(rename = "deliveryStatus")]
    pub delivery_status: String,

    #[serde(rename = "proofVersion")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proof_version: Option<String>,
}
