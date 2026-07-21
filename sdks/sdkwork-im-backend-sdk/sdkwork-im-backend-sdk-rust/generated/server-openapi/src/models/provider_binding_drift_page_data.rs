use serde::{Deserialize, Serialize};

use crate::models::{PageInfo, ProviderBindingDriftItem};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ProviderBindingDriftPageData {
    pub items: Vec<ProviderBindingDriftItem>,

    #[serde(rename = "pageInfo")]
    pub page_info: PageInfo,
}
