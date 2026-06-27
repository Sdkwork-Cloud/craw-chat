use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct SpaceChannelView {
    #[serde(rename = "channelId")]
    pub channel_id: String,

    #[serde(rename = "channelName")]
    pub channel_name: String,

    #[serde(rename = "channelType")]
    pub channel_type: String,
}
