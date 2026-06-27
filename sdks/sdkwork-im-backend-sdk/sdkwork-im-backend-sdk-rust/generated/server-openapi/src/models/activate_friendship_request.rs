use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ActivateFriendshipRequest {
    #[serde(rename = "directChatId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub direct_chat_id: Option<String>,

    #[serde(rename = "establishedAt")]
    pub established_at: String,

    #[serde(rename = "eventId")]
    pub event_id: String,

    #[serde(rename = "friendshipId")]
    pub friendship_id: String,

    #[serde(rename = "initiatorUserId")]
    pub initiator_user_id: String,

    #[serde(rename = "peerUserId")]
    pub peer_user_id: String,
}
