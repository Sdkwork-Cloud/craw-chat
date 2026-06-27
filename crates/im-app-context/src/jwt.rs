use std::collections::BTreeMap;

use sdkwork_utils_rust::{base64url_decode, base64url_encode, hmac_sha256_base64url};
use sdkwork_web_core::{
    EnvBootstrapTenantSigningKeyLookup, JwtVerifier, TenantBoundJwtVerifier, WebEnvironment,
};
use serde_json::{Value, json};

use crate::env::{
    APP_CONTEXT_JWT_KEY_ID_DEFAULT, APP_CONTEXT_JWT_KEY_ID_ENV, APP_CONTEXT_JWT_SIGNING_SECRET_ENV,
    APP_CONTEXT_JWT_TENANT_ID_ENV, resolve_dev_jwt_signing_secret, resolve_web_environment_from_process_env,
    tenant_signing_lookup_from_env,
};
use crate::error::AppContextError;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct TokenClaims {
    values: BTreeMap<String, String>,
}

impl TokenClaims {
    pub(crate) fn parse(raw: &str) -> Result<Self, AppContextError> {
        let raw = raw.trim();
        if raw.is_empty() {
            return Err(AppContextError::invalid("token must not be empty"));
        }
        let environment = resolve_web_environment_from_process_env();
        let dev_or_test = matches!(environment, WebEnvironment::Dev | WebEnvironment::Test);
        if raw.starts_with('{') {
            if !dev_or_test {
                return Err(AppContextError::invalid(
                    "raw JSON bearer tokens are not allowed outside dev/test environments",
                ));
            }
            return Self::from_json_str(raw);
        }
        if is_jwt_token(raw) {
            validate_jwt_token(raw)?;
            let value = decode_jwt_payload(raw)?
                .ok_or_else(|| AppContextError::invalid("token payload must be present"))?;
            return Self::from_json_value(value);
        }
        if !dev_or_test {
            return Err(AppContextError::invalid(
                "key-value bearer tokens are not allowed outside dev/test environments; use signed JWT",
            ));
        }
        Ok(Self {
            values: parse_key_value_claims(raw),
        })
    }

    fn from_json_str(raw: &str) -> Result<Self, AppContextError> {
        let value = serde_json::from_str::<Value>(raw)
            .map_err(|error| AppContextError::invalid(format!("invalid token json: {error}")))?;
        Self::from_json_value(value)
    }

    fn from_json_value(value: Value) -> Result<Self, AppContextError> {
        let object = value
            .as_object()
            .ok_or_else(|| AppContextError::invalid("token claims must be a JSON object"))?;
        Ok(Self {
            values: object
                .iter()
                .filter_map(|(key, value)| {
                    claim_value_to_string(value).map(|value| (key.clone(), value))
                })
                .collect(),
        })
    }

    pub(crate) fn optional(&self, names: &[&str]) -> Option<String> {
        names.iter().find_map(|name| {
            self.values
                .get(*name)
                .map(String::as_str)
                .map(str::trim)
                .filter(|value| !value.is_empty())
                .map(ToOwned::to_owned)
        })
    }

    pub(crate) fn required(&self, names: &[&str], label: &str) -> Result<String, AppContextError> {
        self.optional(names)
            .ok_or_else(|| AppContextError::missing(format!("{label} claim is required")))
    }
}

fn decode_jwt_payload(raw: &str) -> Result<Option<Value>, AppContextError> {
    let mut parts = raw.split('.');
    let _header = parts.next();
    let Some(payload) = parts.next() else {
        return Ok(None);
    };
    if parts.next().is_none() {
        return Ok(None);
    }
    let decoded = base64url_decode(payload).ok_or_else(|| {
        AppContextError::invalid("invalid token payload: base64url decode failed".to_owned())
    })?;
    let value = serde_json::from_slice::<Value>(&decoded).map_err(|error| {
        AppContextError::invalid(format!("invalid token payload json: {error}"))
    })?;
    Ok(Some(value))
}

fn is_jwt_token(raw: &str) -> bool {
    let mut parts = raw.split('.');
    matches!(
        (parts.next(), parts.next(), parts.next()),
        (Some(_), Some(_), Some(_))
    ) && parts.next().is_none()
}

fn decode_jwt_header(raw: &str) -> Result<Value, AppContextError> {
    let header_segment = raw
        .split('.')
        .next()
        .ok_or_else(|| AppContextError::invalid("token header segment is required"))?;
    let decoded = base64url_decode(header_segment).ok_or_else(|| {
        AppContextError::invalid("invalid token header: base64url decode failed".to_owned())
    })?;
    serde_json::from_slice::<Value>(&decoded)
        .map_err(|error| AppContextError::invalid(format!("invalid token header json: {error}")))
}

fn verify_tenant_bound_jwt(
    raw: &str,
    lookup: EnvBootstrapTenantSigningKeyLookup,
) -> Result<(), AppContextError> {
    let verifier = TenantBoundJwtVerifier::new(lookup);
    verifier
        .verify_and_decode_claims(raw)
        .map_err(|error| AppContextError::invalid(error.message))?;
    Ok(())
}

fn validate_jwt_token(raw: &str) -> Result<(), AppContextError> {
    let header = decode_jwt_header(raw)?;
    let algorithm = header
        .get("alg")
        .and_then(Value::as_str)
        .unwrap_or("")
        .trim();

    // Reject alg=none unconditionally — no environment accepts unsigned tokens.
    if algorithm.eq_ignore_ascii_case("none") {
        return Err(AppContextError::invalid(
            "JWT alg=none tokens are not allowed",
        ));
    }

    let environment = resolve_web_environment_from_process_env();
    let dev_or_test = matches!(environment, WebEnvironment::Dev | WebEnvironment::Test);

    // Production path: verify with tenant-bound signing key from env.
    if let Some(lookup) = tenant_signing_lookup_from_env() {
        verify_tenant_bound_jwt(raw, lookup)?;
        return Ok(());
    }

    if !dev_or_test {
        return Err(AppContextError::invalid(format!(
            "signed JWT verification requires {APP_CONTEXT_JWT_TENANT_ID_ENV} and {APP_CONTEXT_JWT_SIGNING_SECRET_ENV} (optional {APP_CONTEXT_JWT_KEY_ID_ENV}, default {APP_CONTEXT_JWT_KEY_ID_DEFAULT}) in production-like environments"
        )));
    }

    // Dev/test fallback: verify HS256 signature with the fixed dev secret.
    // This closes the alg=none bypass (F-11) — unsigned tokens are no longer
    // accepted, even in dev/test.
    verify_dev_hs256_jwt(raw)?;
    let payload = decode_jwt_payload(raw)?
        .ok_or_else(|| AppContextError::invalid("token payload must be present"))?;
    validate_jwt_time_claims(&payload)?;
    Ok(())
}

/// Verifies an HS256 JWT signature using the fixed dev/test secret.
fn verify_dev_hs256_jwt(raw: &str) -> Result<(), AppContextError> {
    let signing_input = raw
        .rsplit_once('.')
        .map(|(head, _)| head)
        .ok_or_else(|| AppContextError::invalid("JWT must have three segments"))?;
    let signature = raw
        .rsplit_once('.')
        .map(|(_, sig)| sig)
        .ok_or_else(|| AppContextError::invalid("JWT signature segment is required"))?;
    if signature.is_empty() {
        return Err(AppContextError::invalid("JWT signature is empty"));
    }
    let dev_secret = resolve_dev_jwt_signing_secret();
    let expected = hmac_sha256_base64url(dev_secret.as_slice(), signing_input.as_bytes());
    // Constant-time comparison to prevent timing attacks.
    if !constant_time_eq(signature.as_bytes(), expected.as_bytes()) {
        return Err(AppContextError::invalid("JWT signature verification failed"));
    }
    Ok(())
}

fn validate_jwt_time_claims(payload: &Value) -> Result<(), AppContextError> {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|duration| duration.as_secs() as i64)
        .unwrap_or(0);
    if let Some(exp) = payload.get("exp") {
        let exp = parse_jwt_numeric_claim(exp, "exp")?;
        if now >= exp {
            return Err(AppContextError::invalid("token has expired"));
        }
    }
    if let Some(nbf) = payload.get("nbf") {
        let nbf = parse_jwt_numeric_claim(nbf, "nbf")?;
        if now < nbf {
            return Err(AppContextError::invalid("token is not yet valid"));
        }
    }
    Ok(())
}

fn parse_jwt_numeric_claim(value: &Value, claim_name: &str) -> Result<i64, AppContextError> {
    match value {
        Value::Number(number) => number.as_i64().ok_or_else(|| {
            AppContextError::invalid(format!("{claim_name} claim must be an integer"))
        }),
        Value::String(raw) => raw.trim().parse::<i64>().map_err(|error| {
            AppContextError::invalid(format!("{claim_name} claim must be an integer: {error}"))
        }),
        _ => Err(AppContextError::invalid(format!(
            "{claim_name} claim must be an integer"
        ))),
    }
}

pub fn encode_local_jwt_claims(claims: Value) -> String {
    let mut claims = claims;
    if let Some(object) = claims.as_object_mut() {
        object
            .entry("token_version")
            .or_insert(json!(sdkwork_web_core::stamp_token_version()));
    }
    // Sign with HS256 using the dev secret so tokens are verifiable in dev/test.
    // This closes the alg=none bypass (F-11).
    let header = base64url_encode(r#"{"alg":"HS256","typ":"JWT"}"#.as_bytes());
    let payload = base64url_encode(claims.to_string().as_bytes());
    let signing_input = format!("{header}.{payload}");
    let dev_secret = resolve_dev_jwt_signing_secret();
    let signature = hmac_sha256_base64url(dev_secret.as_slice(), signing_input.as_bytes());
    format!("{signing_input}.{signature}")
}

fn parse_key_value_claims(raw: &str) -> BTreeMap<String, String> {
    raw.split(';')
        .filter_map(|part| {
            let (key, value) = part.split_once('=')?;
            let key = key.trim();
            let value = value.trim();
            if key.is_empty() || value.is_empty() {
                return None;
            }
            Some((key.to_owned(), value.to_owned()))
        })
        .collect()
}

fn claim_value_to_string(value: &Value) -> Option<String> {
    match value {
        Value::Null => None,
        Value::String(value) => Some(value.trim().to_owned()).filter(|value| !value.is_empty()),
        Value::Bool(value) => Some(value.to_string()),
        Value::Number(value) => Some(value.to_string()),
        Value::Array(items) => {
            let values = items
                .iter()
                .filter_map(claim_value_to_string)
                .collect::<Vec<_>>();
            if values.is_empty() {
                None
            } else {
                Some(values.join(","))
            }
        }
        Value::Object(_) => serde_json::to_string(value).ok(),
    }
}

pub(crate) fn constant_time_eq(left: &[u8], right: &[u8]) -> bool {
    let max_len = left.len().max(right.len());
    let mut diff = left.len() ^ right.len();
    for index in 0..max_len {
        let left_byte = left.get(index).copied().unwrap_or(0);
        let right_byte = right.get(index).copied().unwrap_or(0);
        diff |= usize::from(left_byte ^ right_byte);
    }
    diff == 0
}
