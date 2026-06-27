use serde::{Deserialize, Serialize};

/// Dead-letter shared-channel sync queue snapshot.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct SocialSharedChannelSyncDeadLetterInventoryResponse {
    #[serde(flatten)]
    pub additional_properties: std::collections::HashMap<String, serde_json::Value>,
}
