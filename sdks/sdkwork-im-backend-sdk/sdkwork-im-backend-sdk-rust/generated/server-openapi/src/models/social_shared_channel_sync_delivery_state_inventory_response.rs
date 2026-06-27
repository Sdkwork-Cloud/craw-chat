use serde::{Deserialize, Serialize};

/// Merged shared-channel sync delivery-state inventory snapshot.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct SocialSharedChannelSyncDeliveryStateInventoryResponse {
    #[serde(flatten)]
    pub additional_properties: std::collections::HashMap<String, serde_json::Value>,
}
