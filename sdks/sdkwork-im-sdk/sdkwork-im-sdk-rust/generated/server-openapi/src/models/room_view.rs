use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct RoomView {
    #[serde(rename = "roomId")]
    pub room_id: String,

    #[serde(rename = "roomKind")]
    pub room_kind: String,

    #[serde(rename = "conversationId")]
    pub conversation_id: String,

    #[serde(rename = "activeMemberCount")]
    pub active_member_count: i64,

    #[serde(rename = "maxMembers")]
    pub max_members: i64,
}
