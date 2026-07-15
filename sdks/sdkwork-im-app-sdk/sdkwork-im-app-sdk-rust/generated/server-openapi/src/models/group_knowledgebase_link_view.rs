use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct GroupKnowledgebaseLinkView {
    #[serde(rename = "conversationId")]
    pub conversation_id: String,

    #[serde(rename = "spaceId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub space_id: Option<String>,

    #[serde(rename = "spaceUuid")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub space_uuid: Option<String>,

    #[serde(rename = "lifecycleState")]
    pub lifecycle_state: String,

    #[serde(rename = "provisioningOperationId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provisioning_operation_id: Option<String>,

    #[serde(rename = "membershipEpoch")]
    pub membership_epoch: String,

    #[serde(rename = "upstreamLinkGeneration")]
    pub upstream_link_generation: String,

    #[serde(rename = "lastErrorCode")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_error_code: Option<String>,
}
