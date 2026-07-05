use serde::{Deserialize, Serialize};

use crate::models::{UserBlock};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct OpenApiBlockUserResponse {
    #[serde(rename = "userBlock")]
    pub user_block: UserBlock,
}
