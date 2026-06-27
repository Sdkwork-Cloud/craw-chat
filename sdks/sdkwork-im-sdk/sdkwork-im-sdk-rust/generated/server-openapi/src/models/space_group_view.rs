use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct SpaceGroupView {
    #[serde(rename = "groupId")]
    pub group_id: String,

    #[serde(rename = "groupName")]
    pub group_name: String,
}
