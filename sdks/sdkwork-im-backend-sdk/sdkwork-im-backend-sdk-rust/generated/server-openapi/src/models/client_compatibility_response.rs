use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ClientCompatibilityResponse {
    #[serde(rename = "blockedExperimentalCapabilities")]
    pub blocked_experimental_capabilities: Vec<String>,

    #[serde(rename = "clientType")]
    pub client_type: String,

    #[serde(rename = "minimumProtocolVersion")]
    pub minimum_protocol_version: String,

    #[serde(rename = "supportedBindings")]
    pub supported_bindings: Vec<String>,

    #[serde(rename = "supportedCapabilities")]
    pub supported_capabilities: Vec<String>,

    #[serde(rename = "supportedCodecs")]
    pub supported_codecs: Vec<String>,
}
