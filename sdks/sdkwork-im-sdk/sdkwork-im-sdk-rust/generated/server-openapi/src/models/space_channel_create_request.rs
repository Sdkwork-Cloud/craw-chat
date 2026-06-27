use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct SpaceChannelCreateRequest {
    #[serde(rename = "channelName")]
    pub channel_name: String,

    #[serde(rename = "channelType")]
    pub channel_type: String,
}
