use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct SpaceGroupMemberView {
    #[serde(rename = "userId")]
    pub user_id: String,

    pub role: String,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,

    #[serde(rename = "muteUntil")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mute_until: Option<String>,

    #[serde(rename = "joinedAt")]
    pub joined_at: String,
}
