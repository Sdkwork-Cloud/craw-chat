use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct SpaceChannelAccessRuleView {
    #[serde(rename = "ruleId")]
    pub rule_id: String,

    #[serde(rename = "channelId")]
    pub channel_id: String,

    #[serde(rename = "ruleType")]
    pub rule_type: String,

    #[serde(rename = "principalKind")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub principal_kind: Option<String>,

    #[serde(rename = "principalId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,

    pub permission: String,

    #[serde(rename = "createdAt")]
    pub created_at: String,
}
