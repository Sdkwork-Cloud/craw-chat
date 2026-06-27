use serde::{Deserialize, Serialize};

use crate::models::{SpaceInviteView};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct SpaceInviteListResponse {
    pub items: Vec<SpaceInviteView>,
}
