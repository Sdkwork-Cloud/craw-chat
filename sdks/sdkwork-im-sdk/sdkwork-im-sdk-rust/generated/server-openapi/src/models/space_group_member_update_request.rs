use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct SpaceGroupMemberUpdateRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,

    #[serde(rename = "muteUntil")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mute_until: Option<String>,
}
