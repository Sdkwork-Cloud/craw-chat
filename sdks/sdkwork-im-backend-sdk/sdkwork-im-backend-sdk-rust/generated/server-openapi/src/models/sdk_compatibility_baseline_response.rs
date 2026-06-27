use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct SdkCompatibilityBaselineResponse {
    #[serde(rename = "appSdkFamily")]
    pub app_sdk_family: String,

    #[serde(rename = "backendSdkFamily")]
    pub backend_sdk_family: String,

    #[serde(rename = "imSdkFamily")]
    pub im_sdk_family: String,

    #[serde(rename = "rtcSdkFamily")]
    pub rtc_sdk_family: String,

    #[serde(rename = "matrixClientTypes")]
    pub matrix_client_types: Vec<String>,

    #[serde(rename = "protocolGovernancePath")]
    pub protocol_governance_path: String,

    #[serde(rename = "protocolRegistryPath")]
    pub protocol_registry_path: String,
}
