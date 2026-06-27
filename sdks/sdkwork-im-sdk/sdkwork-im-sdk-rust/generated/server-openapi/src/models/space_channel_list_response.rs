use serde::{Deserialize, Serialize};

use crate::models::{SpaceChannelView};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct SpaceChannelListResponse {
    pub items: Vec<SpaceChannelView>,
}
