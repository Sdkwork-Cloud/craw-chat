use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct CompleteAgentResponseRequest {
    #[serde(rename = "frameSeq")]
    pub frame_seq: i64,

    #[serde(rename = "resultMessageId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result_message_id: Option<String>,
}
