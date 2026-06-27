use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct SocialSharedChannelSyncPendingTargetedClaimRequest {
    #[serde(rename = "requestKeys")]
    pub request_keys: Vec<String>,
}
