use std::sync::Arc;

use crate::api::paths::backend_path;
use crate::http::{SdkworkError, SdkworkHttpClient};

#[derive(Clone)]
pub struct AutomationApi {
    client: Arc<SdkworkHttpClient>,
}

impl AutomationApi {
    pub fn new(client: Arc<SdkworkHttpClient>) -> Self {
        Self { client }
    }

    /// Retrieve automation governance
    pub async fn governance_retrieve(&self) -> Result<std::collections::HashMap<String, serde_json::Value>, SdkworkError> {
        let path = backend_path(&"/automation/governance".to_string());
        self.client.get(&path, None, None).await
    }

}
