use std::sync::Arc;

use crate::api::paths::backend_path;
use crate::http::{SdkworkError, SdkworkHttpClient};

#[derive(Clone)]
pub struct AuditApi {
    client: Arc<SdkworkHttpClient>,
}

impl AuditApi {
    pub fn new(client: Arc<SdkworkHttpClient>) -> Self {
        Self { client }
    }

    /// List audit records
    pub async fn records_list(&self) -> Result<std::collections::HashMap<String, serde_json::Value>, SdkworkError> {
        let path = backend_path(&"/audit/records".to_string());
        self.client.get(&path, None, None).await
    }

    /// Record audit anchor
    pub async fn records_create(&self) -> Result<std::collections::HashMap<String, serde_json::Value>, SdkworkError> {
        let path = backend_path(&"/audit/records".to_string());
        self.client.post(&path, Option::<&serde_json::Value>::None, None, None, None).await
    }

    /// Export audit bundle
    pub async fn export_retrieve(&self) -> Result<std::collections::HashMap<String, serde_json::Value>, SdkworkError> {
        let path = backend_path(&"/audit/export".to_string());
        self.client.get(&path, None, None).await
    }

}
