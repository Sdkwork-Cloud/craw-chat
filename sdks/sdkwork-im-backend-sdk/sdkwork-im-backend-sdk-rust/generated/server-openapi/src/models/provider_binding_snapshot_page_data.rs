use serde::{Deserialize, Serialize};

use crate::models::{PageInfo, ProviderBindingSnapshot};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ProviderBindingSnapshotPageData {
    pub items: Vec<ProviderBindingSnapshot>,

    #[serde(rename = "pageInfo")]
    pub page_info: PageInfo,
}
