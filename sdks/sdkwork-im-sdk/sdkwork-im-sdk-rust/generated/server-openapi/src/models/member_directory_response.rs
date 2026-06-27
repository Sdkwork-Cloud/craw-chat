use serde::{Deserialize, Serialize};

use crate::models::{ConversationMember};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct MemberDirectoryResponse {
    pub items: Vec<ConversationMember>,
}
