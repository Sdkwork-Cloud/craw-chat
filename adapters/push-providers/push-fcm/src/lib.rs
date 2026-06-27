//! FCM (Firebase Cloud Messaging) adapter.
//!
//! Prefers Firebase HTTP v1 when `SDKWORK_IM_FCM_CREDENTIALS_PATH` is configured.
//! Falls back to the legacy HTTP API when `SDKWORK_IM_FCM_SERVER_KEY` is configured.

use im_platform_contracts::{
    ProviderHealthSnapshot, PushDeliveryResult, PushMessage, PushProvider,
};
use im_time::utc_now_rfc3339_millis;
use jsonwebtoken::{Algorithm, EncodingKey, Header};
use sdkwork_im_contract_core::ContractError;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeMap;
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

const FCM_HTTP_V1_URL: &str = "https://fcm.googleapis.com/v1/projects";
const FCM_LEGACY_URL: &str = "https://fcm.googleapis.com/fcm/send";
const FCM_REQUEST_TIMEOUT_SECONDS: u64 = 30;
const FCM_OAUTH_SCOPE: &str = "https://www.googleapis.com/auth/firebase.messaging";

/// FCM adapter configuration.
#[derive(Clone, Debug)]
pub struct FcmConfig {
    pub project_id: String,
    pub server_key: Option<String>,
    pub credentials_path: Option<String>,
}

impl FcmConfig {
    pub fn from_env() -> Result<Self, String> {
        Ok(Self {
            project_id: std::env::var("SDKWORK_IM_FCM_PROJECT_ID")
                .map_err(|_| "SDKWORK_IM_FCM_PROJECT_ID required".to_owned())?,
            server_key: std::env::var("SDKWORK_IM_FCM_SERVER_KEY").ok(),
            credentials_path: std::env::var("SDKWORK_IM_FCM_CREDENTIALS_PATH").ok(),
        })
    }

    fn endpoint(&self) -> String {
        format!("{}/{}/messages:send", FCM_HTTP_V1_URL, self.project_id)
    }

    fn legacy_transport_configured(&self) -> bool {
        self.server_key
            .as_deref()
            .map(str::trim)
            .is_some_and(|value| !value.is_empty())
    }

    fn oauth_transport_configured(&self) -> bool {
        self.credentials_path
            .as_deref()
            .map(str::trim)
            .is_some_and(|value| !value.is_empty())
    }
}

#[derive(Debug, Deserialize)]
struct ServiceAccountCredentials {
    client_email: String,
    private_key: String,
    token_uri: String,
}

#[derive(Debug, Serialize)]
struct ServiceAccountJwtClaims<'a> {
    iss: &'a str,
    sub: &'a str,
    aud: &'a str,
    scope: &'a str,
    iat: u64,
    exp: u64,
}

#[derive(Debug, Deserialize)]
struct OAuthTokenResponse {
    access_token: String,
}

/// FCM push notification adapter.
pub struct FcmPushProvider {
    config: FcmConfig,
    plugin_id: &'static str,
}

impl FcmPushProvider {
    pub fn new(config: FcmConfig) -> Self {
        Self {
            config,
            plugin_id: "push-fcm",
        }
    }

    fn make_request(
        &self,
        message: &PushMessage,
    ) -> Result<PushDeliveryResult, ContractError> {
        if self.config.oauth_transport_configured() {
            return self.send_oauth_v1(message);
        }
        if self.config.legacy_transport_configured() {
            return self.send_legacy(message);
        }
        Ok(PushDeliveryResult {
            accepted: false,
            provider_message_id: None,
            error: Some(
                "FCM credentials are not configured (SDKWORK_IM_FCM_CREDENTIALS_PATH or SDKWORK_IM_FCM_SERVER_KEY)"
                    .into(),
            ),
            token_invalid: false,
        })
    }

    fn send_oauth_v1(&self, message: &PushMessage) -> Result<PushDeliveryResult, ContractError> {
        let credentials_path = self
            .config
            .credentials_path
            .as_deref()
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .ok_or_else(|| {
                ContractError::Unavailable("FCM credentials path must not be empty".to_owned())
            })?;
        let credentials = load_service_account_credentials(credentials_path)?;
        let access_token = fetch_oauth_access_token(&credentials)?;
        let body = build_v1_request_body(message);
        let client = reqwest::blocking::Client::builder()
            .timeout(std::time::Duration::from_secs(FCM_REQUEST_TIMEOUT_SECONDS))
            .build()
            .map_err(|error| ContractError::Unavailable(format!("fcm client init failed: {error}")))?;
        let response = client
            .post(self.config.endpoint())
            .header("Authorization", format!("Bearer {access_token}"))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .map_err(|error| ContractError::Unavailable(format!("fcm v1 request failed: {error}")))?;
        let status = response.status();
        let payload: Value = response.json().map_err(|error| {
            ContractError::Unavailable(format!("fcm v1 response decode failed: {error}"))
        })?;
        parse_v1_fcm_response(status.as_u16(), payload)
    }

    fn send_legacy(&self, message: &PushMessage) -> Result<PushDeliveryResult, ContractError> {
        let server_key = self
            .config
            .server_key
            .as_deref()
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .ok_or_else(|| {
                ContractError::Unavailable("FCM server key must not be empty".to_owned())
            })?;
        let mut data = serde_json::Map::new();
        data.insert("category".into(), Value::String(message.category.clone()));
        if let Some(payload) = message.payload.as_deref() {
            data.insert("payload".into(), Value::String(payload.to_owned()));
        }
        let body = serde_json::json!({
            "to": message.device_token,
            "notification": {
                "title": message.title,
                "body": message.body,
            },
            "data": Value::Object(data),
            "priority": "high",
            "content_available": message.content_available,
        });
        let client = reqwest::blocking::Client::builder()
            .timeout(std::time::Duration::from_secs(FCM_REQUEST_TIMEOUT_SECONDS))
            .build()
            .map_err(|error| ContractError::Unavailable(format!("fcm client init failed: {error}")))?;
        let response = client
            .post(FCM_LEGACY_URL)
            .header("Authorization", format!("key={server_key}"))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .map_err(|error| ContractError::Unavailable(format!("fcm request failed: {error}")))?;
        let status = response.status();
        let payload: Value = response.json().map_err(|error| {
            ContractError::Unavailable(format!("fcm response decode failed: {error}"))
        })?;
        parse_legacy_fcm_response(status.as_u16(), payload)
    }
}

fn load_service_account_credentials(
    credentials_path: &str,
) -> Result<ServiceAccountCredentials, ContractError> {
    let raw = fs::read_to_string(credentials_path).map_err(|error| {
        ContractError::Unavailable(format!(
            "failed to read FCM credentials at {credentials_path}: {error}"
        ))
    })?;
    serde_json::from_str(&raw).map_err(|error| {
        ContractError::Unavailable(format!(
            "failed to parse FCM credentials at {credentials_path}: {error}"
        ))
    })
}

fn fetch_oauth_access_token(
    credentials: &ServiceAccountCredentials,
) -> Result<String, ContractError> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|error| ContractError::Unavailable(format!("system clock error: {error}")))?
        .as_secs();
    let claims = ServiceAccountJwtClaims {
        iss: credentials.client_email.as_str(),
        sub: credentials.client_email.as_str(),
        aud: credentials.token_uri.as_str(),
        scope: FCM_OAUTH_SCOPE,
        iat: now,
        exp: now + 3600,
    };
    let header = Header::new(Algorithm::RS256);
    let encoding_key = EncodingKey::from_rsa_pem(credentials.private_key.as_bytes()).map_err(
        |error| ContractError::Unavailable(format!("invalid FCM service account private key: {error}")),
    )?;
    let assertion = jsonwebtoken::encode(&header, &claims, &encoding_key).map_err(|error| {
        ContractError::Unavailable(format!("failed to sign FCM OAuth JWT: {error}"))
    })?;
    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(FCM_REQUEST_TIMEOUT_SECONDS))
        .build()
        .map_err(|error| ContractError::Unavailable(format!("fcm oauth client init failed: {error}")))?;
    let response = client
        .post(credentials.token_uri.as_str())
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(format!(
            "grant_type=urn:ietf:params:oauth:grant-type:jwt-bearer&assertion={assertion}"
        ))
        .send()
        .map_err(|error| ContractError::Unavailable(format!("fcm oauth token request failed: {error}")))?;
    let status = response.status();
    let payload: Value = response.json().map_err(|error| {
        ContractError::Unavailable(format!("fcm oauth token response decode failed: {error}"))
    })?;
    if !(200..300).contains(&status.as_u16()) {
        return Err(ContractError::Unavailable(format!(
            "fcm oauth token request failed with HTTP {}: {payload}",
            status.as_u16()
        )));
    }
    let token = serde_json::from_value::<OAuthTokenResponse>(payload).map_err(|error| {
        ContractError::Unavailable(format!("fcm oauth token response missing access_token: {error}"))
    })?;
    Ok(token.access_token)
}

fn build_v1_request_body(message: &PushMessage) -> Value {
    let mut data = serde_json::Map::new();
    data.insert("category".into(), Value::String(message.category.clone()));
    if let Some(payload) = message.payload.as_deref() {
        data.insert("payload".into(), Value::String(payload.to_owned()));
    }
    let mut body = serde_json::json!({
        "message": {
            "token": message.device_token,
            "data": Value::Object(data),
            "android": {
                "priority": "HIGH"
            }
        }
    });
    if message.title.is_some() || message.body.is_some() {
        body["message"]["notification"] = serde_json::json!({
            "title": message.title,
            "body": message.body,
        });
    }
    if message.content_available {
        body["message"]["apns"] = serde_json::json!({
            "payload": {
                "aps": {
                    "content-available": 1
                }
            }
        });
    }
    body
}

fn parse_v1_fcm_response(
    status_code: u16,
    payload: Value,
) -> Result<PushDeliveryResult, ContractError> {
    if status_code == 401 || status_code == 403 {
        return Ok(PushDeliveryResult {
            accepted: false,
            provider_message_id: None,
            error: Some(format!("fcm v1 authorization failed with HTTP {status_code}")),
            token_invalid: false,
        });
    }
    if status_code == 404 {
        return Ok(PushDeliveryResult {
            accepted: false,
            provider_message_id: None,
            error: payload
                .get("error")
                .and_then(Value::as_object)
                .and_then(|error| error.get("message"))
                .and_then(Value::as_str)
                .map(str::to_owned)
                .or_else(|| Some("fcm v1 device token not found".into())),
            token_invalid: true,
        });
    }
    if !(200..300).contains(&status_code) {
        let message = payload
            .get("error")
            .and_then(Value::as_object)
            .and_then(|error| error.get("message"))
            .and_then(Value::as_str)
            .unwrap_or("fcm v1 delivery failed");
        return Ok(PushDeliveryResult {
            accepted: false,
            provider_message_id: None,
            error: Some(format!("fcm v1 request failed with HTTP {status_code}: {message}")),
            token_invalid: false,
        });
    }
    let message_id = payload
        .get("name")
        .and_then(Value::as_str)
        .map(str::to_owned);
    Ok(PushDeliveryResult {
        accepted: true,
        provider_message_id: message_id,
        error: None,
        token_invalid: false,
    })
}

fn parse_legacy_fcm_response(
    status_code: u16,
    payload: Value,
) -> Result<PushDeliveryResult, ContractError> {
    if status_code == 401 || status_code == 403 {
        return Ok(PushDeliveryResult {
            accepted: false,
            provider_message_id: None,
            error: Some(format!("fcm authorization failed with HTTP {status_code}")),
            token_invalid: false,
        });
    }
    if !(200..300).contains(&status_code) {
        return Ok(PushDeliveryResult {
            accepted: false,
            provider_message_id: None,
            error: Some(format!(
                "fcm request failed with HTTP {status_code}: {payload}"
            )),
            token_invalid: false,
        });
    }
    let success = payload
        .get("success")
        .and_then(Value::as_i64)
        .unwrap_or(0);
    if success == 1 {
        let message_id = payload
            .get("message_id")
            .and_then(|value| {
                value
                    .as_u64()
                    .map(|id| id.to_string())
                    .or_else(|| value.as_str().map(str::to_owned))
            })
            .or_else(|| {
                payload
                    .get("multicast_id")
                    .and_then(Value::as_u64)
                    .map(|value| value.to_string())
            });
        return Ok(PushDeliveryResult {
            accepted: true,
            provider_message_id: message_id,
            error: None,
            token_invalid: false,
        });
    }
    let failure = payload
        .get("results")
        .and_then(Value::as_array)
        .and_then(|results| results.first())
        .cloned()
        .unwrap_or(payload.clone());
    let error = failure
        .get("error")
        .and_then(Value::as_str)
        .unwrap_or("fcm delivery failed")
        .to_owned();
    let token_invalid = matches!(
        error.as_str(),
        "InvalidRegistration" | "NotRegistered" | "MismatchSenderId"
    );
    Ok(PushDeliveryResult {
        accepted: false,
        provider_message_id: None,
        error: Some(error),
        token_invalid,
    })
}

impl PushProvider for FcmPushProvider {
    fn send(&self, message: &PushMessage) -> Result<PushDeliveryResult, ContractError> {
        self.make_request(message)
    }

    fn provider_health(&self) -> ProviderHealthSnapshot {
        let checked_at = utc_now_rfc3339_millis();
        if self.config.oauth_transport_configured() || self.config.legacy_transport_configured() {
            return ProviderHealthSnapshot::healthy(self.plugin_id, checked_at);
        }
        let mut details = BTreeMap::new();
        details.insert("transport".into(), "unconfigured".into());
        ProviderHealthSnapshot {
            plugin_id: self.plugin_id.to_owned(),
            status: "degraded".into(),
            checked_at,
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
    fn test_fcm_endpoint_includes_project_id() {
        let config = FcmConfig {
            project_id: "my-app-12345".into(),
            server_key: Some("key123".into()),
            credentials_path: None,
        };
        assert!(config.endpoint().contains("my-app-12345"));
        assert!(config.endpoint().contains("messages:send"));
    }

    #[test]
    fn test_provider_health_is_degraded_without_transport() {
        let config = FcmConfig {
            project_id: "my-app-12345".into(),
            server_key: None,
            credentials_path: None,
        };
        let provider = FcmPushProvider::new(config);
        let health = provider.provider_health();
        assert_eq!(health.status, "degraded");
    }

    #[test]
    fn test_provider_health_is_healthy_with_oauth_transport() {
        let config = FcmConfig {
            project_id: "my-app-12345".into(),
            server_key: None,
            credentials_path: Some("/tmp/fcm-service-account.json".into()),
        };
        let provider = FcmPushProvider::new(config);
        let health = provider.provider_health();
        assert_eq!(health.status, "healthy");
    }

    #[test]
    fn test_send_fails_closed_without_credentials() {
        let config = FcmConfig {
            project_id: "my-app-12345".into(),
            server_key: None,
            credentials_path: None,
        };
        let provider = FcmPushProvider::new(config);
        let message = PushMessage {
            device_token: "token".into(),
            title: Some("Hello".into()),
            body: Some("World".into()),
            payload: None,
            category: "message.new".into(),
            content_available: false,
        };
        let result = provider.send(&message).expect("send should return result");
        assert!(!result.accepted);
        assert!(result.error.is_some());
    }

    #[test]
    fn test_parse_legacy_fcm_success_response() {
        let result = parse_legacy_fcm_response(
            200,
            serde_json::json!({ "success": 1, "message_id": "msg-1" }),
        )
        .expect("parse should succeed");
        assert!(result.accepted);
        assert_eq!(result.provider_message_id.as_deref(), Some("msg-1"));
    }

    #[test]
    fn test_parse_v1_fcm_success_response() {
        let result = parse_v1_fcm_response(
            200,
            serde_json::json!({ "name": "projects/demo/messages/msg-1" }),
        )
        .expect("parse should succeed");
        assert!(result.accepted);
        assert_eq!(
            result.provider_message_id.as_deref(),
            Some("projects/demo/messages/msg-1")
        );
    }

    #[test]
    fn test_parse_v1_fcm_invalid_registration() {
        let result = parse_v1_fcm_response(
            404,
            serde_json::json!({
                "error": {
                    "message": "Requested entity was not found."
                }
            }),
        )
        .expect("parse should succeed");
        assert!(!result.accepted);
        assert!(result.token_invalid);
    }

    #[test]
    fn test_parse_legacy_fcm_invalid_registration() {
        let result = parse_legacy_fcm_response(
            200,
            serde_json::json!({
                "success": 0,
                "results": [{ "error": "InvalidRegistration" }]
            }),
        )
        .expect("parse should succeed");
        assert!(!result.accepted);
        assert!(result.token_invalid);
    }

    #[test]
    fn test_build_v1_request_body_includes_notification_and_data() {
        let message = PushMessage {
            device_token: "token".into(),
            title: Some("Hello".into()),
            body: Some("World".into()),
            payload: Some("{\"id\":1}".into()),
            category: "message.new".into(),
            content_available: true,
        };
        let body = build_v1_request_body(&message);
        assert_eq!(body["message"]["token"], "token");
        assert_eq!(body["message"]["notification"]["title"], "Hello");
        assert_eq!(body["message"]["data"]["category"], "message.new");
        assert_eq!(body["message"]["apns"]["payload"]["aps"]["content-available"], 1);
    }

    #[test]
    fn test_plugin_id_matches_expected() {
        let config = FcmConfig {
            project_id: "my-app-12345".into(),
            server_key: None,
            credentials_path: None,
        };
        let provider = FcmPushProvider::new(config);
        assert_eq!(provider.plugin_id(), "push-fcm");
    }
}
