use std::sync::Arc;

use crate::api::paths::app_path;
use crate::http::{SdkworkError, SdkworkHttpClient};
use crate::models::{MediaHealthRetrieveResponse, PrincipalProfileHealthRetrieveResponse};

#[derive(Clone)]
pub struct ProviderApi {
    client: Arc<SdkworkHttpClient>,
}

impl ProviderApi {
    pub fn new(client: Arc<SdkworkHttpClient>) -> Self {
        Self { client }
    }

    /// Retrieve media provider health
    pub async fn media_health_retrieve(&self) -> Result<MediaHealthRetrieveResponse, SdkworkError> {
        let path = app_path(&"/media/provider_health".to_string());
        self.client.get(&path, None, None).await
    }

    /// Retrieve principal-profile provider health
    pub async fn principal_profile_health_retrieve(&self) -> Result<PrincipalProfileHealthRetrieveResponse, SdkworkError> {
        let path = app_path(&"/principal/profiles/provider_health".to_string());
        self.client.get(&path, None, None).await
    }

}
