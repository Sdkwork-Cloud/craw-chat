use axum::http::{HeaderMap, HeaderValue, header};
use im_app_context::AppContext;
use serde::Deserialize;
use serde_json::json;

pub const AUTH_INIT_FRAME_TYPE: &str = "auth.init";
pub const AUTH_INIT_TIMEOUT_SECONDS: u64 = 10;
pub const AUTH_INIT_MAX_FRAME_BYTES: usize = 8 * 1024;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuthInitValidationError {
    WrongFrameType,
    MissingAuthToken,
    MissingAccessToken,
}

impl AuthInitValidationError {
    pub fn error_code(&self) -> &'static str {
        match self {
            Self::WrongFrameType | Self::MissingAuthToken | Self::MissingAccessToken => {
                "websocket_auth_required"
            }
        }
    }

    pub fn message(&self) -> &'static str {
        match self {
            Self::WrongFrameType => "auth.init frame is required before websocket frames",
            Self::MissingAuthToken => "auth.init authToken is required",
            Self::MissingAccessToken => "auth.init accessToken is required",
        }
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct WebsocketAuthInitFrame {
    #[serde(rename = "type")]
    pub frame_type: String,
    pub request_id: Option<String>,
    pub auth_token: Option<String>,
    pub access_token: Option<String>,
    pub device_id: Option<String>,
}

pub fn validate_auth_init_frame(frame: &WebsocketAuthInitFrame) -> Result<(), AuthInitValidationError> {
    if frame.frame_type != AUTH_INIT_FRAME_TYPE {
        return Err(AuthInitValidationError::WrongFrameType);
    }
    if frame
        .auth_token
        .as_deref()
        .map(str::trim)
        .is_none_or(str::is_empty)
    {
        return Err(AuthInitValidationError::MissingAuthToken);
    }
    if frame
        .access_token
        .as_deref()
        .map(str::trim)
        .is_none_or(str::is_empty)
    {
        return Err(AuthInitValidationError::MissingAccessToken);
    }
    Ok(())
}

pub fn dual_token_headers_from_auth_init_frame(
    frame: &WebsocketAuthInitFrame,
) -> Result<HeaderMap, AuthInitValidationError> {
    validate_auth_init_frame(frame)?;
    let mut headers = HeaderMap::new();
    headers.insert(
        header::AUTHORIZATION,
        HeaderValue::from_str(
            normalize_websocket_auth_token(frame.auth_token.as_deref().unwrap_or_default()).as_str(),
        )
        .map_err(|_| AuthInitValidationError::MissingAuthToken)?,
    );
    headers.insert(
        "access-token",
        HeaderValue::from_str(frame.access_token.as_deref().unwrap_or_default().trim())
            .map_err(|_| AuthInitValidationError::MissingAccessToken)?,
    );
    Ok(headers)
}

pub fn normalize_websocket_auth_token(token: &str) -> String {
    if token
        .get(..7)
        .is_some_and(|prefix| prefix.eq_ignore_ascii_case("Bearer "))
    {
        token.to_owned()
    } else {
        format!("Bearer {token}")
    }
}

pub fn auth_ok_payload(
    request_id: Option<&str>,
    tenant_id: &str,
    principal_id: &str,
    session_id: Option<&str>,
    device_id: &str,
) -> String {
    json!({
        "type": "auth.ok",
        "requestId": request_id,
        "tenantId": tenant_id,
        "principalId": principal_id,
        "sessionId": session_id,
        "deviceId": device_id,
    })
    .to_string()
}

pub fn auth_ok_payload_from_context(
    request_id: Option<&str>,
    auth: &AppContext,
    device_id: &str,
) -> String {
    auth_ok_payload(
        request_id,
        auth.tenant_id.as_str(),
        auth.user_id.as_str(),
        auth.session_id.as_deref(),
        device_id,
    )
}

pub fn auth_error_payload(request_id: Option<&str>, code: &str, message: &str) -> String {
    json!({
        "type": "error",
        "requestId": request_id,
        "code": code,
        "message": message,
    })
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_auth_init_frame_requires_dual_tokens() {
        let frame = WebsocketAuthInitFrame {
            frame_type: AUTH_INIT_FRAME_TYPE.to_owned(),
            request_id: Some("req-1".to_owned()),
            auth_token: Some("auth-1".to_owned()),
            access_token: Some("access-1".to_owned()),
            device_id: Some("device-1".to_owned()),
        };
        assert!(validate_auth_init_frame(&frame).is_ok());
        assert_eq!(
            dual_token_headers_from_auth_init_frame(&frame)
                .expect("headers")
                .get("access-token")
                .and_then(|value| value.to_str().ok()),
            Some("access-1")
        );
    }

    #[test]
    fn validate_auth_init_frame_rejects_missing_access_token() {
        let frame = WebsocketAuthInitFrame {
            frame_type: AUTH_INIT_FRAME_TYPE.to_owned(),
            request_id: None,
            auth_token: Some("auth-1".to_owned()),
            access_token: None,
            device_id: None,
        };
        assert_eq!(
            validate_auth_init_frame(&frame),
            Err(AuthInitValidationError::MissingAccessToken)
        );
    }
}
