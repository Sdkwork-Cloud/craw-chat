use serde::{Deserialize, Serialize};

use crate::models::{ClientCompatibilityResponse, ProtocolSchemaResponse};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ProtocolRegistryResponse {
    pub bindings: Vec<String>,

    pub codecs: Vec<String>,

    #[serde(rename = "compatibilityMatrix")]
    pub compatibility_matrix: Vec<ClientCompatibilityResponse>,

    #[serde(rename = "protocolVersion")]
    pub protocol_version: String,

    pub schemas: Vec<ProtocolSchemaResponse>,
}
