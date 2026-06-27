use serde::{Deserialize, Serialize};

use crate::models::{SpaceGroupView};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct SpaceGroupListResponse {
    pub items: Vec<SpaceGroupView>,
}
