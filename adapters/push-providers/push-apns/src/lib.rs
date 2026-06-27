//! APNs (Apple Push Notification service) adapter.
//!
//! Uses HTTP/2 with JWT (JSON Web Token) authentication.
//! Requires: Team ID, Key ID, private key (P8 file), and bundle ID.
//!
//! ## Configuration
//! - `SDKWORK_IM_APNS_TEAM_ID`: Apple Developer Team ID
//! - `SDKWORK_IM_APNS_KEY_ID`: APNs Key ID
//! - `SDKWORK_IM_APNS_KEY_PATH`: Path to P8 private key file
//! - `SDKWORK_IM_APNS_BUNDLE_ID`: App bundle identifier
//! - `SDKWORK_IM_APNS_SANDBOX`: Set to "true" for development environment

use im_platform_contracts::{
    ProviderHealthSnapshot, PushDeliveryResult, PushMessage, PushProvider,
};
use im_time::utc_now_rfc3339_millis;
use sdkwork_im_contract_core::ContractError;
use std::path::PathBuf;

const APNS_DEVELOPMENT_HOST: &str = "api.sandbox.push.apple.com";
const APNS_PRODUCTION_HOST: &str = "api.push.apple.com";

/// APNs adapter configuration.
#[derive(Clone, Debug)]
pub struct ApnsConfig {
    pub team_id: String,
    pub key_id: String,
    pub key_path: PathBuf,
    pub bundle_id: String,
    pub sandbox: bool,
}

impl ApnsConfig {
    pub fn from_env() -> Result<Self, String> {
        Ok(Self {
            team_id: std::env::var("SDKWORK_IM_APNS_TEAM_ID")
                .map_err(|_| "SDKWORK_IM_APNS_TEAM_ID required".to_owned())?,
            key_id: std::env::var("SDKWORK_IM_APNS_KEY_ID")
                .map_err(|_| "SDKWORK_IM_APNS_KEY_ID required".to_owned())?,
            key_path: PathBuf::from(
                std::env::var("SDKWORK_IM_APNS_KEY_PATH")
                    .map_err(|_| "SDKWORK_IM_APNS_KEY_PATH required".to_owned())?,
            ),
            bundle_id: std::env::var("SDKWORK_IM_APNS_BUNDLE_ID")
                .map_err(|_| "SDKWORK_IM_APNS_BUNDLE_ID required".to_owned())?,
            sandbox: std::env::var("SDKWORK_IM_APNS_SANDBOX")
                .map(|v| v.eq_ignore_ascii_case("true"))
                .unwrap_or(false),
        })
    }

    fn host(&self) -> &str {
        if self.sandbox {
            APNS_DEVELOPMENT_HOST
        } else {
            APNS_PRODUCTION_HOST
        }
    }
}

/// APNs push notification adapter.
pub struct ApnsPushProvider {
    config: ApnsConfig,
    plugin_id: &'static str,
}

impl ApnsPushProvider {
    pub fn new(config: ApnsConfig) -> Self {
        Self {
            config,
            plugin_id: "push-apns",
        }
    }

    fn make_request(
        &self,
        device_token: &str,
        _payload: &serde_json::Value,
        _message_id: &str,
    ) -> Result<PushDeliveryResult, ContractError> {
        // Validate device token is hexadecimal
        if !device_token.chars().all(|c| c.is_ascii_hexdigit()) {
            return Ok(PushDeliveryResult {
                accepted: false,
                provider_message_id: None,
                error: Some("invalid device token".into()),
                token_invalid: true,
            });
        }

        let _topic = format!("{}.voip", self.config.bundle_id);
        let _path = format!("/3/device/{}", device_token);
        let _host = self.config.host();

        // APNs requires HTTP/2 with JWT bearer token in the authorization header.
        // JWT is signed with ES256 using the P8 private key.
        //
        // Full implementation requires:
        // 1. JWT signing (ES256) with {team_id}.{key_id} claims
        // 2. HTTP/2 connection to api.push.apple.com:443
        // 3. POST with :path, :method, authorization, apns-topic, apns-push-type headers
        // 4. Parse apns-id from response headers
        //
        // When APNs credentials are configured, replace this stub with:
        //   let jwt = sign_jwt(&self.config)?;
        //   let response = send_apns_request(&self.config, device_token, &jwt, payload).await?;
        //   parse_apns_response(response)

        // APNs HTTP/2 + JWT transport is not wired in this adapter yet.
        Ok(PushDeliveryResult {
            accepted: false,
            provider_message_id: None,
            error: Some(
                "APNs HTTP/2 JWT transport is not configured; push delivery is unavailable"
                    .into(),
            ),
            token_invalid: false,
        })
    }
}

impl PushProvider for ApnsPushProvider {
    fn send(&self, message: &PushMessage) -> Result<PushDeliveryResult, ContractError> {
        let aps = serde_json::json!({
            "aps": {
                "alert": {
                    "title": message.title,
                    "body": message.body,
                },
                "badge": 1,
                "sound": "default",
                "content-available": if message.content_available { 1 } else { 0 },
            },
            "category": message.category,
            "payload": message.payload,
        });
        let message_id = format!("apns_{}", utc_now_rfc3339_millis());
        self.make_request(&message.device_token, &aps, &message_id)
    }

    fn provider_health(&self) -> ProviderHealthSnapshot {
        let mut details = std::collections::BTreeMap::new();
        details.insert("transport".into(), "http2_jwt_pending".into());
        ProviderHealthSnapshot {
            plugin_id: self.plugin_id.to_owned(),
            status: "degraded".into(),
            checked_at: utc_now_rfc3339_millis(),
            details,
        }
    }

    fn plugin_id(&self) -> &'static str {
        self.plugin_id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apns_rejects_non_hex_token() {
        let config = ApnsConfig {
            team_id: "TEAM123".into(),
            key_id: "KEY123".into(),
            key_path: PathBuf::from("/tmp/key.p8"),
            bundle_id: "com.example.app".into(),
            sandbox: true,
        };
        let provider = ApnsPushProvider::new(config);
        let msg = PushMessage {
            device_token: "not-hex-token!".into(),
            title: Some("Hello".into()),
            body: Some("World".into()),
            payload: None,
            category: "message.new".into(),
            content_available: false,
        };
        let result = provider.send(&msg).expect("send should not error");
        assert!(result.token_invalid, "non-hex token should be rejected");
    }

    #[test]
    fn test_apns_fails_closed_for_valid_hex_token_without_transport() {
        let config = ApnsConfig {
            team_id: "TEAM123".into(),
            key_id: "KEY123".into(),
            key_path: PathBuf::from("/tmp/key.p8"),
            bundle_id: "com.example.app".into(),
            sandbox: true,
        };
        let provider = ApnsPushProvider::new(config);
        let msg = PushMessage {
            device_token: "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef".into(),
            title: Some("Hello".into()),
            body: Some("World".into()),
            payload: None,
            category: "message.new".into(),
            content_available: false,
        };
        let result = provider.send(&msg).expect("send should not error");
        assert!(!result.accepted);
        assert!(result.error.is_some());
    }

    #[test]
    fn test_provider_health_is_degraded_without_transport() {
        let config = ApnsConfig {
            team_id: "TEAM123".into(),
            key_id: "KEY123".into(),
            key_path: PathBuf::from("/tmp/key.p8"),
            bundle_id: "com.example.app".into(),
            sandbox: true,
        };
        let provider = ApnsPushProvider::new(config);
        let health = provider.provider_health();
        assert_eq!(health.status, "degraded");
    }

    #[test]
    fn test_plugin_id_matches_expected() {
        let config = ApnsConfig {
            team_id: "TEAM123".into(),
            key_id: "KEY123".into(),
            key_path: PathBuf::from("/tmp/key.p8"),
            bundle_id: "com.example.app".into(),
            sandbox: true,
        };
        let provider = ApnsPushProvider::new(config);
        assert_eq!(provider.plugin_id(), "push-apns");
    }
}
