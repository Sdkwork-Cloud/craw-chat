use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct RemoveConversationMemberRequest {
    #[serde(rename = "memberId")]
    pub member_id: String,
}
