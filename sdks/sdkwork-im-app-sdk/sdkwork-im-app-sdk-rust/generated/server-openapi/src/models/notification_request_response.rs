use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct NotificationRequestResponse {
    #[serde(rename = "tenantId")]
    pub tenant_id: String,

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

    pub status: String,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payload: Option<String>,

    #[serde(rename = "requestedAt")]
    pub requested_at: String,

    #[serde(rename = "dispatchedAt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dispatched_at: Option<String>,

    #[serde(rename = "failureReason")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,

    #[serde(rename = "requestKey")]
    pub request_key: String,

    #[serde(rename = "deliveryStatus")]
    pub delivery_status: String,

    #[serde(rename = "proofVersion")]
    pub proof_version: String,
}
