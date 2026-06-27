use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct SocialSharedChannelSyncPendingTargetedTakeoverRequest {
    #[serde(rename = "allowLegacyUntracked")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_legacy_untracked: Option<bool>,

    #[serde(rename = "requestKeys")]
    pub request_keys: Vec<String>,
}
