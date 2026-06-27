use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct CapabilityProfileResponse {
    #[serde(rename = "enabledCapabilities")]
    pub enabled_capabilities: Vec<String>,

    #[serde(rename = "experimentalCapabilities")]
    pub experimental_capabilities: Vec<String>,

    #[serde(rename = "profileId")]
    pub profile_id: String,

    #[serde(rename = "releaseChannel")]
    pub release_channel: String,
}
