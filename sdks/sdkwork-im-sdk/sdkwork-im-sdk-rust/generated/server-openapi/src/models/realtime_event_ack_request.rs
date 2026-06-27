use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct RealtimeEventAckRequest {
    #[serde(rename = "eventIds")]
    pub event_ids: Vec<String>,
}
