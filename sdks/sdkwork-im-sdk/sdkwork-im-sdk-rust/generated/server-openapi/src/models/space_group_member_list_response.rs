use serde::{Deserialize, Serialize};

use crate::models::{SpaceGroupMemberView};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct SpaceGroupMemberListResponse {
    pub items: Vec<SpaceGroupMemberView>,
}
