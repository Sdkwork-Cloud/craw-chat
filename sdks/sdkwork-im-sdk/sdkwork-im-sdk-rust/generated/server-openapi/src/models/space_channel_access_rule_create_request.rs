use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct SpaceChannelAccessRuleCreateRequest {
    #[serde(rename = "ruleType")]
    pub rule_type: String,

    #[serde(rename = "principalKind")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub principal_kind: Option<String>,

    #[serde(rename = "principalId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,

    pub permission: String,
}
