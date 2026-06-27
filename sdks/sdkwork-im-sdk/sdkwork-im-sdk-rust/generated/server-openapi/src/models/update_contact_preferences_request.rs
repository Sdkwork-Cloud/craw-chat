use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct UpdateContactPreferencesRequest {
    #[serde(rename = "isStarred")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_starred: Option<bool>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,

    #[serde(rename = "isBlocked")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_blocked: Option<bool>,
}
