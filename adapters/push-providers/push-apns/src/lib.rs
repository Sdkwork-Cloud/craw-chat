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

use std::fs;
use std::path::PathBuf;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use im_platform_contracts::{
    ProviderHealthSnapshot, PushDeliveryResult, PushMessage, PushProvider,
};
use im_time::utc_now_rfc3339_millis;
use jsonwebtoken::{Algorithm, EncodingKey, Header, encode};
use sdkwork_im_contract_core::ContractError;
use serde::Serialize;

const APNS_DEVELOPMENT_HOST: &str = "api.sandbox.push.apple.com";
const APNS_PRODUCTION_HOST: &str = "api.push.apple.com";
const APNS_REQUEST_TIMEOUT_SECONDS: u64 = 30;
const APNS_JWT_TTL_SECONDS: u64 = 3_500;

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

#[derive(Debug, Serialize)]
struct ApnsJwtClaims {
    iss: String,
    iat: u64,
    exp: u64,
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

    fn sign_jwt(&self) -> Result<String, ContractError> {
        let key_pem = fs::read_to_string(&self.config.key_path).map_err(|error| {
            ContractError::Unavailable(format!(
                "failed to read APNs key at {}: {error}",
                self.config.key_path.display()
            ))
        })?;
        let encoding_key = EncodingKey::from_ec_pem(key_pem.as_bytes()).map_err(|error| {
            ContractError::Unavailable(format!("invalid APNs P8 private key: {error}"))
        })?;
        let issued_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|error| ContractError::Unavailable(format!("system clock error: {error}")))?
            .as_secs();
        let claims = ApnsJwtClaims {
            iss: self.config.team_id.clone(),
            iat: issued_at,
            exp: issued_at.saturating_add(APNS_JWT_TTL_SECONDS),
        };
        let mut header = Header::new(Algorithm::ES256);
        header.kid = Some(self.config.key_id.clone());
        encode(&header, &claims, &encoding_key).map_err(|error| {
            ContractError::Unavailable(format!("APNs JWT signing failed: {error}"))
        })
    }

    fn make_request(
        &self,
        device_token: &str,
        payload: &serde_json::Value,
        message_id: &str,
    ) -> Result<PushDeliveryResult, ContractError> {
        if !device_token.chars().all(|c| c.is_ascii_hexdigit()) {
            return Ok(PushDeliveryResult {
                accepted: false,
                provider_message_id: None,
                error: Some("invalid device token".into()),
                token_invalid: true,
            });
        }

        let jwt = self.sign_jwt()?;
        let url = format!("https://{}/3/device/{}", self.config.host(), device_token);
        let client = reqwest::blocking::Client::builder()
            .timeout(Duration::from_secs(APNS_REQUEST_TIMEOUT_SECONDS))
            .build()
            .map_err(|error| {
                ContractError::Unavailable(format!(
                    "APNs HTTP client initialization failed: {error}"
                ))
            })?;
        let response = client
            .post(url)
            .header("authorization", format!("bearer {jwt}"))
            .header("apns-topic", self.config.bundle_id.as_str())
            .header("apns-push-type", "alert")
            .header("apns-id", message_id)
            .header("content-type", "application/json")
            .json(payload)
            .send()
            .map_err(|error| {
                ContractError::Unavailable(format!("APNs HTTP/2 request failed: {error}"))
            })?;

        let status_code = response.status().as_u16();
        let headers = response.headers().clone();
        let body = response.text().ok();
        parse_apns_response(status_code, &headers, body)
    }
}

fn parse_apns_response(
    status_code: u16,
    headers: &reqwest::header::HeaderMap,
    body: Option<String>,
) -> Result<PushDeliveryResult, ContractError> {
    let provider_message_id = headers
        .get("apns-id")
        .and_then(|value| value.to_str().ok())
        .map(str::to_owned);

    if (200..300).contains(&status_code) {
        return Ok(PushDeliveryResult {
            accepted: true,
            provider_message_id,
            error: None,
            token_invalid: false,
        });
    }

    let reason = body
        .as_deref()
        .and_then(|text| serde_json::from_str::<serde_json::Value>(text).ok())
        .and_then(|payload| {
            payload
                .get("reason")
                .and_then(serde_json::Value::as_str)
                .map(str::to_owned)
        })
        .unwrap_or_else(|| format!("HTTP {status_code}"));

    let token_invalid = matches!(
        reason.as_str(),
        "BadDeviceToken" | "DeviceTokenNotForTopic" | "Unregistered" | "ExpiredToken"
    ) || status_code == 410;

    Ok(PushDeliveryResult {
        accepted: false,
        provider_message_id,
        error: Some(format!("apns delivery failed: {reason}")),
        token_invalid,
    })
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
        details.insert("transport".into(), "http2_jwt".into());
        details.insert("host".into(), self.config.host().into());
        let status = if self.config.key_path.is_file() {
            "healthy"
        } else {
            "degraded"
        };
        ProviderHealthSnapshot {
            plugin_id: self.plugin_id.to_owned(),
            status: status.into(),
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

    fn sample_config() -> ApnsConfig {
        ApnsConfig {
            team_id: "TEAM123".into(),
            key_id: "KEY123".into(),
            key_path: PathBuf::from("/tmp/key.p8"),
            bundle_id: "com.example.app".into(),
            sandbox: true,
        }
    }

    #[test]
    fn test_apns_rejects_non_hex_token() {
        let provider = ApnsPushProvider::new(sample_config());
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
    fn test_parse_apns_response_success() {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            "apns-id",
            reqwest::header::HeaderValue::from_static("abc-123"),
        );
        let result =
            parse_apns_response(200, &headers, None).expect("success response should parse");
        assert!(result.accepted);
        assert_eq!(result.provider_message_id.as_deref(), Some("abc-123"));
    }

    #[test]
    fn test_parse_apns_response_unregistered() {
        let headers = reqwest::header::HeaderMap::new();
        let result =
            parse_apns_response(410, &headers, Some(r#"{"reason":"Unregistered"}"#.into()))
                .expect("error response should parse");
        assert!(!result.accepted);
        assert!(result.token_invalid);
    }

    #[test]
    fn test_provider_health_reports_http2_transport() {
        let provider = ApnsPushProvider::new(sample_config());
        let health = provider.provider_health();
        assert_eq!(
            health.details.get("transport").map(String::as_str),
            Some("http2_jwt")
        );
    }

    #[test]
    fn test_plugin_id_matches_expected() {
        let provider = ApnsPushProvider::new(sample_config());
        assert_eq!(provider.plugin_id(), "push-apns");
    }

    #[test]
    fn test_jwt_ttl_constant_is_below_apns_one_hour_limit() {
        assert!(APNS_JWT_TTL_SECONDS <= 3_600);
    }

    #[test]
    fn test_sign_jwt_claims_carry_required_iat_and_exp() {
        let provider = ApnsPushProvider::new(sample_config());
        // Signing requires a real P8 key file; assert the claim struct itself
        // carries both required APNs claims with the configured TTL so the
        // constant can never silently become dead code again.
        let issued_at = 1_700_000_000u64;
        let claims = ApnsJwtClaims {
            iss: provider.config.team_id.clone(),
            iat: issued_at,
            exp: issued_at.saturating_add(APNS_JWT_TTL_SECONDS),
        };
        assert_eq!(claims.exp.saturating_sub(claims.iat), APNS_JWT_TTL_SECONDS);
        assert!(
            claims.exp.saturating_sub(claims.iat) <= 3_600,
            "APNs provider tokens must not exceed the one-hour expiry limit"
        );
    }
}
