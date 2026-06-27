use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ProtocolSchemaResponse {
    #[serde(rename = "bindingProtocols")]
    pub binding_protocols: Vec<String>,

    pub kind: String,

    #[serde(rename = "requiredCapabilities")]
    pub required_capabilities: Vec<String>,

    pub schema: String,

    pub stage: String,

    #[serde(rename = "supportedConsumers")]
    pub supported_consumers: Vec<String>,
}
