use serde::{Deserialize, Serialize};

/// Automatic stale reclaim result for pending shared-channel sync entries.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct SocialSharedChannelSyncPendingStaleReclaimResponse {
    #[serde(flatten)]
    pub additional_properties: std::collections::HashMap<String, serde_json::Value>,
}
