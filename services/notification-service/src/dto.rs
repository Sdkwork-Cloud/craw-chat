use std::collections::BTreeSet;

use im_domain_core::notification::NotificationTask;
use serde::{Deserialize, Serialize};

pub(crate) const NOTIFICATION_REQUEST_DELIVERY_PROOF_VERSION: &str =
    "notification.request.delivery-proof.v1";

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestNotification {
    pub notification_id: String,
    pub source_event_id: String,
    pub source_event_type: String,
    pub category: String,
    pub channel: String,
    pub recipient_id: String,
    pub recipient_kind: String,
    pub title: Option<String>,
    pub body: Option<String>,
    pub payload: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct NotificationRecipient {
    pub recipient_id: String,
    pub recipient_kind: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RequestNotificationFanout {
    pub notification_id_seed: String,
    pub source_event_id: String,
    pub source_event_type: String,
    pub category: String,
    pub channel: String,
    pub recipients: BTreeSet<NotificationRecipient>,
    pub title: Option<String>,
    pub body: Option<String>,
    pub payload: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RequestAutomationResultNotification {
    pub execution_id: String,
    pub target_ref: String,
    pub output_payload: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RequestMessagePostedNotifications {
    pub source_event_id: String,
    pub conversation_id: String,
    pub message_id: String,
    pub message_seq: u64,
    pub message_type: String,
    pub summary: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotificationRequestResult {
    pub task: NotificationTask,
    pub is_new: bool,
    pub request_key: String,
    pub delivery_status: NotificationRequestDeliveryStatus,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NotificationRequestDeliveryStatus {
    Accepted,
    Applied,
    Replayed,
    Failed,
}

impl NotificationRequestDeliveryStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Accepted => "accepted",
            Self::Applied => "applied",
            Self::Replayed => "replayed",
            Self::Failed => "failed",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotificationRequestResponse {
    #[serde(flatten)]
    pub task: NotificationTask,
    pub request_key: String,
    pub delivery_status: NotificationRequestDeliveryStatus,
    pub proof_version: String,
}

impl From<NotificationRequestResult> for NotificationRequestResponse {
    fn from(value: NotificationRequestResult) -> Self {
        Self {
            task: value.task,
            request_key: value.request_key,
            delivery_status: value.delivery_status,
            proof_version: NOTIFICATION_REQUEST_DELIVERY_PROOF_VERSION.into(),
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct NotificationListResponse {
    pub(crate) items: Vec<NotificationTask>,
}
