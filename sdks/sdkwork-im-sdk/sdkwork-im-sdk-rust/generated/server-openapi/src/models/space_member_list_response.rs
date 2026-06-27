use serde::{Deserialize, Serialize};

use crate::models::{SpaceMemberView};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct SpaceMemberListResponse {
    pub items: Vec<SpaceMemberView>,
}
