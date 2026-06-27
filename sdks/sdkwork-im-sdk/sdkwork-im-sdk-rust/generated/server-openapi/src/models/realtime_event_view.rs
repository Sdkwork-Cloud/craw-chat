use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct RealtimeEventView {
    #[serde(rename = "eventId")]
    pub event_id: String,

    pub scope: String,

    #[serde(rename = "scopeId")]
    pub scope_id: String,

    #[serde(rename = "eventType")]
    pub event_type: String,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payload: Option<String>,

    #[serde(rename = "occurredAt")]
    pub occurred_at: String,
}
