use serde::{Deserialize, Serialize};

use crate::models::{SpaceChannelAccessRuleView};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct SpaceChannelAccessRuleListResponse {
    pub items: Vec<SpaceChannelAccessRuleView>,
}
