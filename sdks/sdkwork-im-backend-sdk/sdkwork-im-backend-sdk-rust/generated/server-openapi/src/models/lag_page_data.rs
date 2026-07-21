use serde::{Deserialize, Serialize};

use crate::models::{LagItem, PageInfo};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct LagPageData {
    pub items: Vec<LagItem>,

    #[serde(rename = "pageInfo")]
    pub page_info: PageInfo,
}
