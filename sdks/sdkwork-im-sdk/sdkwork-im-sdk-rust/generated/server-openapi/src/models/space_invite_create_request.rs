use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct SpaceInviteCreateRequest {
    #[serde(rename = "maxUses")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_uses: Option<i64>,
}
