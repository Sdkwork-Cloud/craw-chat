use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ConversationInboxPreferencesView {
    #[serde(rename = "isPinned")]
    pub is_pinned: bool,

    #[serde(rename = "isMuted")]
    pub is_muted: bool,

    #[serde(rename = "isMarkedUnread")]
    pub is_marked_unread: bool,

    #[serde(rename = "isHidden")]
    pub is_hidden: bool,
}
