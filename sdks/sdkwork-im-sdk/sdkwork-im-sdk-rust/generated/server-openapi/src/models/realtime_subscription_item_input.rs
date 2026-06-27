use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct RealtimeSubscriptionItemInput {
    #[serde(rename = "scopeType")]
    pub scope_type: String,

    #[serde(rename = "scopeId")]
    pub scope_id: String,

    #[serde(rename = "eventTypes")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event_types: Option<Vec<String>>,
}
