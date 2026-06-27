use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct CreateRoomRequest {
    #[serde(rename = "conversationId")]
    pub conversation_id: String,

    #[serde(rename = "roomId")]
    pub room_id: String,

    #[serde(rename = "roomKind")]
    pub room_kind: String,
}
