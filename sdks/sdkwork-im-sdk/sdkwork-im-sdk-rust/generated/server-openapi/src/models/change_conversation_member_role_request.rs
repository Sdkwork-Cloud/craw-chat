use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ChangeConversationMemberRoleRequest {
    #[serde(rename = "memberId")]
    pub member_id: String,

    pub role: String,
}
