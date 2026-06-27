use serde::{Deserialize, Serialize};

use crate::models::{SpaceBanView};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct SpaceBanListResponse {
    pub items: Vec<SpaceBanView>,
}
