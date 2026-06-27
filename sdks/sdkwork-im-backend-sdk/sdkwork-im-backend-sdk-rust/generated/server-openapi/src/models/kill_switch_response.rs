use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct KillSwitchResponse {
    pub active: bool,

    #[serde(rename = "disabledBindings")]
    pub disabled_bindings: Vec<String>,

    #[serde(rename = "disabledCapabilities")]
    pub disabled_capabilities: Vec<String>,

    #[serde(rename = "disabledCodecs")]
    pub disabled_codecs: Vec<String>,

    pub reason: String,

    #[serde(rename = "ruleId")]
    pub rule_id: String,
}
