use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct StreamRefContentPart {
    pub kind: String,

    #[serde(rename = "streamId")]
    pub stream_id: String,

    #[serde(rename = "streamType")]
    pub stream_type: String,

    pub state: String,
}
