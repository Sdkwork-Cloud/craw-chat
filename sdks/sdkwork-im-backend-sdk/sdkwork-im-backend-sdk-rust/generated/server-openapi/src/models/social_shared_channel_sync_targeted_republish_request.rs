use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct SocialSharedChannelSyncTargetedRepublishRequest {
    #[serde(rename = "requestKeys")]
    pub request_keys: Vec<String>,
}
