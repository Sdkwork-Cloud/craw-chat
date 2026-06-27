use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum NotificationStatus {
    Requested,
    Dispatched,
    Failed,
}

impl NotificationStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Requested => "requested",
            Self::Dispatched => "dispatched",
            Self::Failed => "failed",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotificationTask {
    pub tenant_id: String,
    pub notification_id: String,
    pub source_event_id: String,
    pub source_event_type: String,
    pub category: String,
    pub channel: String,
    pub recipient_id: String,
    pub recipient_kind: String,
    pub status: NotificationStatus,
    pub title: Option<String>,
    pub body: Option<String>,
    pub payload: Option<String>,
    pub requested_at: String,
    pub dispatched_at: Option<String>,
    pub failure_reason: Option<String>,
}
