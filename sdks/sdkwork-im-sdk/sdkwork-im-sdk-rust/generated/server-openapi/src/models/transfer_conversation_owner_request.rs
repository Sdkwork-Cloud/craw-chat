use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct TransferConversationOwnerRequest {
    #[serde(rename = "memberId")]
    pub member_id: String,
}
