pub mod storage;

use sdkwork_im_contract_core::ContractError;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdminCapabilityProfileRecord {
    pub tenant_id: String,
    pub profile_id: String,
    pub release_channel: String,
    pub capability_keys: Vec<String>,
    pub updated_at: String,
}

pub trait AdminCapabilityProfileStore: Send + Sync {
    fn load_profile(
        &self,
        tenant_id: &str,
        profile_id: &str,
    ) -> Result<Option<AdminCapabilityProfileRecord>, ContractError>;

    fn save_profile(&self, record: AdminCapabilityProfileRecord) -> Result<(), ContractError>;
}
