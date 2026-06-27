use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct SocialSharedChannelSyncPendingTargetedReleaseRequest {
    #[serde(rename = "requestKeys")]
    pub request_keys: Vec<String>,
}
