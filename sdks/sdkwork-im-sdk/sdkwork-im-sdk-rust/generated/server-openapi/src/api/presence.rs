use std::sync::Arc;

use crate::api::paths::im_path;
use crate::http::{SdkworkError, SdkworkHttpClient};
use crate::models::{PresenceHeartbeatRequest, PresenceView};

#[derive(Clone)]
pub struct PresenceApi {
    client: Arc<SdkworkHttpClient>,
}

impl PresenceApi {
    pub fn new(client: Arc<SdkworkHttpClient>) -> Self {
        Self { client }
    }

    /// Publish current client route presence heartbeat
    pub async fn heartbeat_create(&self, body: &PresenceHeartbeatRequest) -> Result<PresenceView, SdkworkError> {
        let path = im_path(&"/presence/heartbeat".to_string());
        self.client.post(&path, Some(body), None, None, Some("application/json")).await
    }

    /// Retrieve current principal presence
    pub async fn me_retrieve(&self) -> Result<PresenceView, SdkworkError> {
        let path = im_path(&"/presence/me".to_string());
        self.client.get(&path, None, None).await
    }

}
