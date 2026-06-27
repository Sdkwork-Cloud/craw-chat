use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct SpaceChannelUpdateRequest {
    #[serde(rename = "channelName")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub channel_name: Option<String>,
}
