use std::sync::Arc;

use crate::api::paths::backend_path;
use crate::http::{SdkworkError, SdkworkHttpClient};

#[derive(Clone)]
pub struct OpsApi {
    client: Arc<SdkworkHttpClient>,
}

impl OpsApi {
    pub fn new(client: Arc<SdkworkHttpClient>) -> Self {
        Self { client }
    }

    /// Retrieve ops health
    pub async fn health_retrieve(&self) -> Result<std::collections::HashMap<String, serde_json::Value>, SdkworkError> {
        let path = backend_path(&"/ops/health".to_string());
        self.client.get(&path, None, None).await
    }

    /// Retrieve cluster state
    pub async fn cluster_retrieve(&self) -> Result<std::collections::HashMap<String, serde_json::Value>, SdkworkError> {
        let path = backend_path(&"/ops/cluster".to_string());
        self.client.get(&path, None, None).await
    }

    /// Retrieve projection lag
    pub async fn lag_retrieve(&self) -> Result<std::collections::HashMap<String, serde_json::Value>, SdkworkError> {
        let path = backend_path(&"/ops/lag".to_string());
        self.client.get(&path, None, None).await
    }

    /// Retrieve replay status
    pub async fn replay_status_retrieve(&self) -> Result<std::collections::HashMap<String, serde_json::Value>, SdkworkError> {
        let path = backend_path(&"/ops/replay_status".to_string());
        self.client.get(&path, None, None).await
    }

    /// Retrieve commercial readiness
    pub async fn commercial_readiness_retrieve(&self) -> Result<std::collections::HashMap<String, serde_json::Value>, SdkworkError> {
        let path = backend_path(&"/ops/commercial_readiness".to_string());
        self.client.get(&path, None, None).await
    }

    /// Inspect runtime directory
    pub async fn runtime_dir_retrieve(&self) -> Result<std::collections::HashMap<String, serde_json::Value>, SdkworkError> {
        let path = backend_path(&"/ops/runtime_dir".to_string());
        self.client.get(&path, None, None).await
    }

    /// List provider bindings
    pub async fn provider_bindings_list(&self) -> Result<std::collections::HashMap<String, serde_json::Value>, SdkworkError> {
        let path = backend_path(&"/ops/provider_bindings".to_string());
        self.client.get(&path, None, None).await
    }

    /// Retrieve provider binding drift
    pub async fn provider_bindings_drift_retrieve(&self) -> Result<std::collections::HashMap<String, serde_json::Value>, SdkworkError> {
        let path = backend_path(&"/ops/provider_bindings/drift".to_string());
        self.client.get(&path, None, None).await
    }

    /// Retrieve diagnostics
    pub async fn diagnostics_retrieve(&self) -> Result<std::collections::HashMap<String, serde_json::Value>, SdkworkError> {
        let path = backend_path(&"/ops/diagnostics".to_string());
        self.client.get(&path, None, None).await
    }

}
