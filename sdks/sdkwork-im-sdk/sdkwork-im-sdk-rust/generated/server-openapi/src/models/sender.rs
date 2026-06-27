use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Sender {
    pub id: String,

    pub kind: String,

    #[serde(rename = "principalId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,

    #[serde(rename = "principalKind")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub principal_kind: Option<String>,

    #[serde(rename = "displayName")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    #[serde(rename = "avatarUrl")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
}
