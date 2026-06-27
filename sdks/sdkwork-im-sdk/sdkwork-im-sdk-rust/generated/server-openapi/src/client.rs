use std::sync::Arc;

use crate::api::{PresenceApi, RealtimeApi, CallsApi, SocialApi, ChatApi, StreamsApi, SpacesApi};
use crate::http::{SdkworkConfig, SdkworkError, SdkworkHttpClient};

#[derive(Clone)]
pub struct SdkworkImClient {
    http: Arc<SdkworkHttpClient>,
}

impl SdkworkImClient {
    pub fn new(config: SdkworkConfig) -> Result<Self, SdkworkError> {
        Ok(Self {
            http: Arc::new(SdkworkHttpClient::new(config)?),
        })
    }

    pub fn new_with_base_url(base_url: impl Into<String>) -> Result<Self, SdkworkError> {
        Self::new(SdkworkConfig::new(base_url))
    }
    pub fn set_auth_token(&self, token: impl Into<String>) -> &Self {
        self.http.set_auth_token(token);
        self
    }

    pub fn set_access_token(&self, token: impl Into<String>) -> &Self {
        self.http.set_access_token(token);
        self
    }

    pub fn set_header(&self, key: impl Into<String>, value: impl Into<String>) -> &Self {
        self.http.set_header(key, value);
        self
    }

    pub fn http_client(&self) -> Arc<SdkworkHttpClient> {
        Arc::clone(&self.http)
    }

    pub fn presence(&self) -> PresenceApi {
            PresenceApi::new(Arc::clone(&self.http))
        }

    pub fn realtime(&self) -> RealtimeApi {
            RealtimeApi::new(Arc::clone(&self.http))
        }

    pub fn calls(&self) -> CallsApi {
            CallsApi::new(Arc::clone(&self.http))
        }

    pub fn social(&self) -> SocialApi {
            SocialApi::new(Arc::clone(&self.http))
        }

    pub fn chat(&self) -> ChatApi {
            ChatApi::new(Arc::clone(&self.http))
        }

    pub fn streams(&self) -> StreamsApi {
            StreamsApi::new(Arc::clone(&self.http))
        }

    pub fn spaces(&self) -> SpacesApi {
            SpacesApi::new(Arc::clone(&self.http))
        }
}
