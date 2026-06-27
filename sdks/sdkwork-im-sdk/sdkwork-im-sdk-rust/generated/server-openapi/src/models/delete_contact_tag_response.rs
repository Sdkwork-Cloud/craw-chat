use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct DeleteContactTagResponse {
    #[serde(rename = "tagId")]
    pub tag_id: String,

    pub deleted: bool,
}
