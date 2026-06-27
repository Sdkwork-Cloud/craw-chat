use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct BusinessPolicyVocabularyResponse {
    #[serde(rename = "capabilityFlagsField")]
    pub capability_flags_field: String,

    #[serde(rename = "historyVisibilityField")]
    pub history_visibility_field: String,

    #[serde(rename = "historyVisibilityModes")]
    pub history_visibility_modes: Vec<String>,

    #[serde(rename = "policyVersionField")]
    pub policy_version_field: String,

    #[serde(rename = "retentionPolicyRefField")]
    pub retention_policy_ref_field: String,

    #[serde(rename = "retentionPolicyScopes")]
    pub retention_policy_scopes: Vec<String>,
}
