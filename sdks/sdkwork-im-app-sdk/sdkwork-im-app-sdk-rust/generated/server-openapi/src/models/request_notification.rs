use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct RequestNotification {
    #[serde(rename = "notificationId")]
    pub notification_id: String,

    #[serde(rename = "sourceEventId")]
    pub source_event_id: String,

    #[serde(rename = "sourceEventType")]
    pub source_event_type: String,

    pub category: String,

    pub channel: String,

    #[serde(rename = "recipientId")]
    pub recipient_id: String,

    #[serde(rename = "recipientKind")]
    pub recipient_kind: String,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payload: Option<String>,
}
