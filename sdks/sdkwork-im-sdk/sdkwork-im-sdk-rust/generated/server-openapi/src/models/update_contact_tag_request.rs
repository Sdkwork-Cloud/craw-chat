use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct UpdateContactTagRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bg: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub border: Option<String>,
}
