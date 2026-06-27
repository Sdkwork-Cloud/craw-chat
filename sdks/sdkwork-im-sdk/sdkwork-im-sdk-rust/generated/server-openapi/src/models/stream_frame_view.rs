use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct StreamFrameView {
    #[serde(rename = "streamId")]
    pub stream_id: String,

    #[serde(rename = "frameSeq")]
    pub frame_seq: i64,

    pub payload: String,

    #[serde(rename = "createdAt")]
    pub created_at: String,
}
