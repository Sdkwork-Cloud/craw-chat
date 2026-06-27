use serde::{Deserialize, Serialize};

use crate::models::{SpaceView};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct SpaceListResponse {
    pub items: Vec<SpaceView>,
}
