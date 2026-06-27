use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct UpdateRtcSessionRequest {
    #[serde(rename = "artifactMessageId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub artifact_message_id: Option<String>,
}
