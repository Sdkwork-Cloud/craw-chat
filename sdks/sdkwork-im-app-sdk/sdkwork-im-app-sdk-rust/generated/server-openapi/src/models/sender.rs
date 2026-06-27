use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Sender {
    pub id: String,

    pub kind: String,

    #[serde(rename = "memberId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub member_id: Option<String>,

    #[serde(rename = "deviceId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,

    #[serde(rename = "sessionId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,

    pub metadata: std::collections::HashMap<String, String>,
}
