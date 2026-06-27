use serde::{Deserialize, Serialize};

use crate::models::{CreateConversationResult, DirectChat, FriendRequest, Friendship};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct SocialFriendRequestAcceptanceResponse {
    #[serde(rename = "friendRequest")]
    pub friend_request: FriendRequest,

    pub friendship: Friendship,

    #[serde(rename = "directChat")]
    pub direct_chat: DirectChat,

    pub conversation: CreateConversationResult,
}
