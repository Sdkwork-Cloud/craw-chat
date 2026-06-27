use serde::{Deserialize, Serialize};

/// Pending shared-channel sync queue snapshot.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct SocialSharedChannelSyncPendingInventoryResponse {
    #[serde(flatten)]
    pub additional_properties: std::collections::HashMap<String, serde_json::Value>,
}
