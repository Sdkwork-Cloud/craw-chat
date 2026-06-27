use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct EffectiveProtocolSnapshotResponse {
    #[serde(rename = "allowedBindings")]
    pub allowed_bindings: Vec<String>,

    #[serde(rename = "allowedCodecs")]
    pub allowed_codecs: Vec<String>,

    #[serde(rename = "enabledCapabilities")]
    pub enabled_capabilities: Vec<String>,

    #[serde(rename = "killSwitchActive")]
    pub kill_switch_active: bool,

    pub precedence: Vec<String>,

    #[serde(rename = "protocolVersion")]
    pub protocol_version: String,

    #[serde(rename = "quotaProfileId")]
    pub quota_profile_id: String,

    #[serde(rename = "releaseChannel")]
    pub release_channel: String,
}
