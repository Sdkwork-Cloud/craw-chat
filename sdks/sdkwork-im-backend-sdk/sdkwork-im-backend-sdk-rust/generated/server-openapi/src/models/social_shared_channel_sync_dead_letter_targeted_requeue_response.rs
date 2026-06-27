use serde::{Deserialize, Serialize};

/// Targeted requeue result for selected dead-letter shared-channel sync entries.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct SocialSharedChannelSyncDeadLetterTargetedRequeueResponse {
    #[serde(flatten)]
    pub additional_properties: std::collections::HashMap<String, serde_json::Value>,
}
