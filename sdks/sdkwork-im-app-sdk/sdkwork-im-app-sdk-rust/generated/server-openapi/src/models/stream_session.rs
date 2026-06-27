use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct StreamSession {
    #[serde(rename = "tenantId")]
    pub tenant_id: String,

    #[serde(rename = "streamId")]
    pub stream_id: String,

    #[serde(rename = "streamType")]
    pub stream_type: String,

    #[serde(rename = "scopeKind")]
    pub scope_kind: String,

    #[serde(rename = "scopeId")]
    pub scope_id: String,

    #[serde(rename = "durabilityClass")]
    pub durability_class: String,

    #[serde(rename = "orderingScope")]
    pub ordering_scope: String,

    #[serde(rename = "schemaRef")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schema_ref: Option<String>,

    pub state: String,

    #[serde(rename = "lastFrameSeq")]
    pub last_frame_seq: i64,

    #[serde(rename = "lastCheckpointSeq")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_checkpoint_seq: Option<i64>,

    #[serde(rename = "resultMessageId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result_message_id: Option<String>,

    #[serde(rename = "openedAt")]
    pub opened_at: String,

    #[serde(rename = "closedAt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub closed_at: Option<String>,

    #[serde(rename = "expiresAt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
}
