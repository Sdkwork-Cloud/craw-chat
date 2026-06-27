use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ReadCursorView {
    #[serde(rename = "tenantId")]
    pub tenant_id: String,

    #[serde(rename = "conversationId")]
    pub conversation_id: String,

    #[serde(rename = "principalId")]
    pub principal_id: String,

    #[serde(rename = "readSeq")]
    pub read_seq: i64,

    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}
