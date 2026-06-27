use serde::{Deserialize, Serialize};

/// Targeted republish result for selected shared-channel sync entries.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct SocialSharedChannelSyncTargetedRepublishResponse {
    #[serde(flatten)]
    pub additional_properties: std::collections::HashMap<String, serde_json::Value>,
}
