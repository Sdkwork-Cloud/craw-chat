use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct UpdateConversationPreferencesRequest {
    #[serde(rename = "isPinned")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_pinned: Option<bool>,

    #[serde(rename = "isMuted")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_muted: Option<bool>,

    #[serde(rename = "isMarkedUnread")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_marked_unread: Option<bool>,

    #[serde(rename = "isHidden")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_hidden: Option<bool>,
}
