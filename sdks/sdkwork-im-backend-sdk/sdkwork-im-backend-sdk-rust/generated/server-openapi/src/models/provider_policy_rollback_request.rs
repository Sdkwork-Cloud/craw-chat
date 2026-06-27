use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ProviderPolicyRollbackRequest {
    #[serde(rename = "targetVersion")]
    pub target_version: i64,
}
