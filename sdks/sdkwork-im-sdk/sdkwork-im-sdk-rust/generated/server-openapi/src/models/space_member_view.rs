use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct SpaceMemberView {
    #[serde(rename = "userId")]
    pub user_id: String,

    pub role: String,
}
