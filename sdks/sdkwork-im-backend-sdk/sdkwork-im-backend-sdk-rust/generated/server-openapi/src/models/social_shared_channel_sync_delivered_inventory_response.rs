use serde::{Deserialize, Serialize};

/// Delivered shared-channel sync ledger snapshot.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct SocialSharedChannelSyncDeliveredInventoryResponse {
    #[serde(flatten)]
    pub additional_properties: std::collections::HashMap<String, serde_json::Value>,
}
