use serde::{Deserialize, Serialize};

use crate::models::{RealtimeSubscriptionItemInput};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct RealtimeSubscriptionSyncRequest {
    #[serde(rename = "deviceId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conversations: Option<Vec<String>>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<RealtimeSubscriptionItemInput>>,
}
