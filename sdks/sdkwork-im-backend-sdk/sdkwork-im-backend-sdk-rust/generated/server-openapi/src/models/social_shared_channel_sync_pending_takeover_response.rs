use serde::{Deserialize, Serialize};

/// Targeted takeover result for pending shared-channel sync entries.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct SocialSharedChannelSyncPendingTakeoverResponse {
    #[serde(flatten)]
    pub additional_properties: std::collections::HashMap<String, serde_json::Value>,
}
