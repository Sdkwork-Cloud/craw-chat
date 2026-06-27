use std::sync::Arc;

use crate::api::paths::app_path;
use crate::http::{SdkworkError, SdkworkHttpClient};
use crate::models::{PortalWorkspaceView};

#[derive(Clone)]
pub struct PortalApi {
    client: Arc<SdkworkHttpClient>,
}

impl PortalApi {
    pub fn new(client: Arc<SdkworkHttpClient>) -> Self {
        Self { client }
    }

    /// Read the tenant portal sign-in snapshot
    pub async fn access_retrieve(&self) -> Result<std::collections::HashMap<String, serde_json::Value>, SdkworkError> {
        let path = app_path(&"/portal/access".to_string());
        self.client.get(&path, None, None).await
    }

    /// Read the tenant automation snapshot
    pub async fn automation_retrieve(&self) -> Result<std::collections::HashMap<String, serde_json::Value>, SdkworkError> {
        let path = app_path(&"/portal/automation".to_string());
        self.client.get(&path, None, None).await
    }

    /// Read the tenant conversations snapshot
    pub async fn conversation_snapshot_retrieve(&self) -> Result<std::collections::HashMap<String, serde_json::Value>, SdkworkError> {
        let path = app_path(&"/portal/conversations".to_string());
        self.client.get(&path, None, None).await
    }

    /// Read the tenant dashboard snapshot
    pub async fn dashboard_retrieve(&self) -> Result<std::collections::HashMap<String, serde_json::Value>, SdkworkError> {
        let path = app_path(&"/portal/dashboard".to_string());
        self.client.get(&path, None, None).await
    }

    /// Read the tenant governance snapshot
    pub async fn governance_retrieve(&self) -> Result<std::collections::HashMap<String, serde_json::Value>, SdkworkError> {
        let path = app_path(&"/portal/governance".to_string());
        self.client.get(&path, None, None).await
    }

    /// Read the tenant portal home snapshot
    pub async fn home_retrieve(&self) -> Result<std::collections::HashMap<String, serde_json::Value>, SdkworkError> {
        let path = app_path(&"/portal/home".to_string());
        self.client.get(&path, None, None).await
    }

    /// Read the tenant media snapshot
    pub async fn media_retrieve(&self) -> Result<std::collections::HashMap<String, serde_json::Value>, SdkworkError> {
        let path = app_path(&"/portal/media".to_string());
        self.client.get(&path, None, None).await
    }

    /// Read the tenant realtime snapshot
    pub async fn realtime_retrieve(&self) -> Result<std::collections::HashMap<String, serde_json::Value>, SdkworkError> {
        let path = app_path(&"/portal/realtime".to_string());
        self.client.get(&path, None, None).await
    }

    /// Read the current tenant workspace snapshot
    pub async fn workspace_retrieve(&self) -> Result<PortalWorkspaceView, SdkworkError> {
        let path = app_path(&"/portal/workspace".to_string());
        self.client.get(&path, None, None).await
    }

}
