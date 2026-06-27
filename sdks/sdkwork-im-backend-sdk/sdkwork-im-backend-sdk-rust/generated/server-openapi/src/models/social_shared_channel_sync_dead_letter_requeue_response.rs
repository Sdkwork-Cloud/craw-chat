use serde::{Deserialize, Serialize};

/// Bulk requeue result for dead-letter shared-channel sync entries.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct SocialSharedChannelSyncDeadLetterRequeueResponse {
    #[serde(flatten)]
    pub additional_properties: std::collections::HashMap<String, serde_json::Value>,
}
