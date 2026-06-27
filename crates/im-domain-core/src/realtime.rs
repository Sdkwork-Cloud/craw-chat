use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RealtimeSubscription {
    pub scope_type: String,
    pub scope_id: String,
    pub event_types: Vec<String>,
    pub subscribed_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RealtimeSubscriptionSnapshot {
    pub tenant_id: String,
    pub principal_id: String,
    pub device_id: String,
    pub items: Vec<RealtimeSubscription>,
    pub synced_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RealtimeEvent {
    pub tenant_id: String,
    pub principal_id: String,
    pub device_id: String,
    pub realtime_seq: u64,
    pub scope_type: String,
    pub scope_id: String,
    pub event_type: String,
    pub delivery_class: String,
    pub payload: String,
    pub occurred_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RealtimeEventWindow {
    pub device_id: String,
    pub items: Vec<RealtimeEvent>,
    pub next_after_seq: Option<u64>,
    pub has_more: bool,
    pub acked_through_seq: u64,
    pub trimmed_through_seq: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RealtimeAckState {
    pub tenant_id: String,
    pub principal_id: String,
    pub device_id: String,
    pub acked_through_seq: u64,
    pub trimmed_through_seq: u64,
    pub retained_event_count: usize,
    pub acked_at: String,
}
