use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct UpdateReadCursorRequest {
    #[serde(rename = "readSeq")]
    pub read_seq: i64,
}
