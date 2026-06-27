use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct MessageVisibilityMutationResult {
    #[serde(rename = "tenantId")]
    pub tenant_id: String,

    #[serde(rename = "conversationId")]
    pub conversation_id: String,

    #[serde(rename = "messageId")]
    pub message_id: String,

    #[serde(rename = "messageSeq")]
    pub message_seq: i64,

    #[serde(rename = "principalKind")]
    pub principal_kind: String,

    #[serde(rename = "principalId")]
    pub principal_id: String,

    #[serde(rename = "isDeleted")]
    pub is_deleted: bool,

    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}
