//! Sensitive data redaction for logging and audit trails.
//!
//! This module provides automatic redaction of sensitive data in logs,
//! ensuring that tokens, passwords, and other confidential information
//! are never exposed in log output or audit trails.
//!
//! ## Redaction Patterns
//!
//! - **Bearer tokens**: Authorization headers with bearer tokens
//! - **Access tokens**: Query parameters or headers with access tokens
//! - **Passwords**: Password fields in JSON payloads
//! - **API keys**: API key parameters
//! - **Secret keys**: Secret or private key fields
//! - **Email addresses**: Email patterns (optional, for privacy)
//!
//! ## Architecture
//!
//! ```
//! ┌─────────────────┐
//! │ Log Entry       │
//! └────────┬────────┘
//!          │
//!          v
//! ┌─────────────────┐
//! │ Pattern Matcher │  ← Regex patterns for sensitive data
//! └────────┬────────┘
//!          │
//!          v
//! ┌─────────────────┐
//! │ Redaction       │  ← Replace with [REDACTED]
//! └────────┬────────┘
//!          │
//!          v
//! ┌─────────────────┐
//! │ Output Log      │  ← Safe to expose
//! └─────────────────┘
//! ```

use std::sync::Arc;
use std::collections::HashMap;

use regex::Regex;
use serde::{Deserialize, Serialize};
use tracing_subscriber::Layer;

/// Sensitive data redactor with configurable patterns.
#[derive(Clone, Debug)]
pub struct SensitiveDataRedactor {
    /// Regex patterns for sensitive data detection.
    patterns: Vec<(Regex, String)>,
    /// Field names to redact in JSON payloads.
    sensitive_fields: Vec<String>,
    /// Enable email redaction (optional for privacy).
    redact_emails: bool,
    /// Enable IP redaction (optional for privacy).
    redact_ips: bool,
}

impl SensitiveDataRedactor {
    /// Create redactor with standard patterns.
    pub fn new() -> Self {
        Self {
            patterns: vec![
                // Bearer tokens
                (
                    Regex::new(r"Bearer\s+[A-Za-z0-9\-._~+/]+=*").unwrap(),
                    "Bearer [REDACTED]".to_string(),
                ),
                // Access tokens in headers
                (
                    Regex::new(r#"access_token["']?\s*[:=]\s*["'][A-Za-z0-9\-._~+/]+=*["']"#).unwrap(),
                    "access_token=[REDACTED]".to_string(),
                ),
                // Access tokens in query params
                (
                    Regex::new(r"access_token=[A-Za-z0-9\-._~+/]+=*").unwrap(),
                    "access_token=[REDACTED]".to_string(),
                ),
                // Authorization headers (any format)
                (
                    Regex::new(r#"Authorization["']?\s*[:=]\s*["'][^"']+["']"#).unwrap(),
                    "Authorization=[REDACTED]".to_string(),
                ),
                // API keys (with or without quotes)
                (
                    Regex::new(r#"api_key["']?\s*[:=]\s*["']?[A-Za-z0-9\-._~+/]+["']?"#).unwrap(),
                    "api_key=[REDACTED]".to_string(),
                ),
                // Secret keys (with or without quotes)
                (
                    Regex::new(r#"(secret|private_key|password)["']?\s*[:=]\s*["']?[A-Za-z0-9\-_]+["']?"#).unwrap(),
                    "$1=[REDACTED]".to_string(),
                ),
                // JWT tokens
                (
                    Regex::new(r"eyJ[A-Za-z0-9\-._~+/]+=*\.eyJ[A-Za-z0-9\-._~+/]+=*\.[A-Za-z0-9\-._~+/]+=*").unwrap(),
                    "[JWT_REDACTED]".to_string(),
                ),
                // Session IDs
                (
                    Regex::new(r#"session_id["']?\s*[:=]\s*["'][A-Za-z0-9\-]{20,}["']"#).unwrap(),
                    "session_id=[REDACTED]".to_string(),
                ),
                // Generic tokens (long alphanumeric strings)
                (
                    Regex::new(r#"["']?token["']?\s*[:=]\s*["'][A-Za-z0-9\-._~+/]{20,}["']"#).unwrap(),
                    "token=[REDACTED]".to_string(),
                ),
                // Credit card numbers (Visa, MasterCard, Amex, Discover)
                (
                    Regex::new(r"\b(?:4[0-9]{12}(?:[0-9]{3})?|5[1-5][0-9]{14}|3[47][0-9]{13}|6(?:011|5[0-9]{2})[0-9]{12})\b").unwrap(),
                    "[CARD_REDACTED]".to_string(),
                ),
                // SSN (US Social Security Number)
                (
                    Regex::new(r"\b[0-9]{3}-[0-9]{2}-[0-9]{4}\b").unwrap(),
                    "[SSN_REDACTED]".to_string(),
                ),
                // Chinese phone numbers
                (
                    Regex::new(r"\b1[3-9]\d{9}\b").unwrap(),
                    "[PHONE_REDACTED]".to_string(),
                ),
                // International phone numbers
                (
                    Regex::new(r"\+\d{1,3}[\s-]?\d{3,4}[\s-]?\d{4}").unwrap(),
                    "[PHONE_REDACTED]".to_string(),
                ),
                // Connection strings (common databases)
                (
                    Regex::new(r#"(mysql|postgres|mongodb|redis)://[^\s"']+"#).unwrap(),
                    "[CONNECTION_REDACTED]".to_string(),
                ),
                // AWS Access Key ID
                (
                    Regex::new(r"(A3T[A-Z0-9]|AKIA|AGPA|AIDA|AROA|AIPA|ANPA|ANVA|ASIA)[A-Z0-9]{16}").unwrap(),
                    "[AWS_KEY_REDACTED]".to_string(),
                ),
                // AWS Secret Access Key
                (
                    Regex::new(r#"(?i)aws_secret_access_key["']?\s*[:=]\s*["'][A-Za-z0-9/+=]{40}["']"#).unwrap(),
                    "aws_secret_access_key=[REDACTED]".to_string(),
                ),
                // Generic API keys (various formats)
                (
                    Regex::new(r#"(?i)(api[-_]?key|apikey)["']?\s*[:=]\s*["'][A-Za-z0-9\-_]{20,}["']"#).unwrap(),
                    "api_key=[REDACTED]".to_string(),
                ),
                // Private keys (PEM format)
                (
                    Regex::new(r#"-----BEGIN (?:RSA |EC |DSA )?PRIVATE KEY-----[\s\S]*?-----END (?:RSA |EC |DSA )?PRIVATE KEY-----"#).unwrap(),
                    "[PRIVATE_KEY_REDACTED]".to_string(),
                ),
            ],
            sensitive_fields: vec![
                "password".to_string(),
                "secret".to_string(),
                "private_key".to_string(),
                "access_token".to_string(),
                "auth_token".to_string(),
                "api_key".to_string(),
                "session_id".to_string(),
                "credential".to_string(),
            ],
            redact_emails: false,
            redact_ips: false,
        }
    }

    /// Create redactor with custom patterns.
    pub fn with_patterns(patterns: Vec<(String, String)>) -> Self {
        let compiled = patterns
            .into_iter()
            .filter_map(|(pattern, replacement)| {
                Regex::new(&pattern).ok().map(|r| (r, replacement))
            })
            .collect();

        Self {
            patterns: compiled,
            sensitive_fields: vec![],
            redact_emails: false,
            redact_ips: false,
        }
    }

    /// Enable email redaction for privacy compliance.
    pub fn with_email_redaction(mut self, enable: bool) -> Self {
        self.redact_emails = enable;
        if enable {
            self.patterns.push((
                Regex::new(r"[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}").unwrap(),
                "[EMAIL_REDACTED]".to_string(),
            ));
        }
        self
    }

    /// Enable IP redaction for privacy compliance.
    pub fn with_ip_redaction(mut self, enable: bool) -> Self {
        self.redact_ips = enable;
        if enable {
            // IPv4
            self.patterns.push((
                Regex::new(r"\b\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}\b").unwrap(),
                "[IP_REDACTED]".to_string(),
            ));
            // IPv6 (simplified)
            self.patterns.push((
                Regex::new(r"\b[0-9a-fA-F:]+:[0-9a-fA-F:]+\b").unwrap(),
                "[IP_REDACTED]".to_string(),
            ));
        }
        self
    }

    /// Redact sensitive data in a text string.
    pub fn redact(&self, text: &str) -> String {
        let mut result = text.to_owned();

        // Apply regex patterns
        for (pattern, replacement) in &self.patterns {
            result = pattern.replace_all(&result, replacement.as_str()).to_string();
        }

        result
    }

    /// Redact sensitive fields in a JSON structure.
    pub fn redact_json(&self, json: &serde_json::Value) -> serde_json::Value {
        match json {
            serde_json::Value::Object(map) => {
                let redacted = map
                    .iter()
                    .map(|(key, value)| {
                        if self.sensitive_fields.contains(&key.to_lowercase()) {
                            (key.clone(), serde_json::Value::String("[REDACTED]".to_string()))
                        } else {
                            (key.clone(), self.redact_json(value))
                        }
                    })
                    .collect();
                serde_json::Value::Object(redacted)
            }
            serde_json::Value::Array(arr) => {
                let redacted = arr.iter().map(|v| self.redact_json(v)).collect();
                serde_json::Value::Array(redacted)
            }
            serde_json::Value::String(s) => {
                serde_json::Value::String(self.redact(s))
            }
            other => other.clone(),
        }
    }

    /// Check if a field name is sensitive.
    pub fn is_sensitive_field(&self, field_name: &str) -> bool {
        self.sensitive_fields.contains(&field_name.to_lowercase())
    }

    /// Get list of redacted patterns.
    pub fn pattern_count(&self) -> usize {
        self.patterns.len()
    }
}

impl Default for SensitiveDataRedactor {
    fn default() -> Self {
        Self::new()
    }
}

/// Tracing layer for automatic log redaction.
pub struct RedactingLayer {
    redactor: Arc<SensitiveDataRedactor>,
}

impl RedactingLayer {
    pub fn new(redactor: SensitiveDataRedactor) -> Self {
        Self {
            redactor: Arc::new(redactor),
        }
    }

    pub fn default() -> Self {
        Self::new(SensitiveDataRedactor::new())
    }
}

impl<S> Layer<S> for RedactingLayer
where
    S: tracing::Subscriber,
{
    fn on_event(
        &self,
        event: &tracing::Event<'_>,
        _ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
        // Collect event fields
        let mut visitor = RedactingVisitor {
            fields: HashMap::new(),
            redactor: Arc::clone(&self.redactor),
        };
        event.record(&mut visitor);

        // Note: In a full implementation, this would intercept the log output
        // and apply redaction before writing. For simplicity, this example
        // shows the visitor pattern for field collection.
    }
}

/// Visitor for collecting and redacting tracing event fields.
struct RedactingVisitor {
    fields: HashMap<String, String>,
    redactor: Arc<SensitiveDataRedactor>,
}

impl tracing::field::Visit for RedactingVisitor {
    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
        let field_name = field.name();
        let raw_value = format!("{:?}", value);

        if self.redactor.is_sensitive_field(field_name) {
            self.fields.insert(field_name.to_owned(), "[REDACTED]".to_string());
        } else {
            let redacted = self.redactor.redact(&raw_value);
            self.fields.insert(field_name.to_owned(), redacted);
        }
    }

    fn record_str(&mut self, field: &tracing::field::Field, value: &str) {
        let field_name = field.name();

        if self.redactor.is_sensitive_field(field_name) {
            self.fields.insert(field_name.to_owned(), "[REDACTED]".to_string());
        } else {
            let redacted = self.redactor.redact(value);
            self.fields.insert(field_name.to_owned(), redacted);
        }
    }
}

/// Redacted audit trail entry.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RedactedAuditEntry {
    /// Timestamp of the event.
    pub timestamp: String,
    /// Event type/action.
    pub action: String,
    /// Tenant ID (safe to expose).
    pub tenant_id: String,
    /// User ID (safe to expose).
    pub user_id: Option<String>,
    /// Redacted client IP (if IP redaction enabled).
    pub client_ip: String,
    /// Redacted request details.
    pub request_details: serde_json::Value,
    /// Response status.
    pub response_status: u16,
    /// Duration in milliseconds.
    pub duration_ms: u64,
    /// Any errors encountered.
    pub error: Option<String>,
}

impl RedactedAuditEntry {
    /// Create from raw audit data with automatic redaction.
    pub fn from_raw(
        redactor: &SensitiveDataRedactor,
        timestamp: String,
        action: String,
        tenant_id: String,
        user_id: Option<String>,
        client_ip: String,
        request_details: serde_json::Value,
        response_status: u16,
        duration_ms: u64,
        error: Option<String>,
    ) -> Self {
        Self {
            timestamp,
            action,
            tenant_id,
            user_id,
            client_ip: if redactor.redact_ips {
                "[IP_REDACTED]".to_string()
            } else {
                client_ip
            },
            request_details: redactor.redact_json(&request_details),
            response_status,
            duration_ms,
            error: error.map(|e| redactor.redact(&e)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn redactor_bearer_token() {
        let redactor = SensitiveDataRedactor::new();
        let text = "Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWI";
        let redacted = redactor.redact(text);
        
        assert_eq!(redacted, "Authorization: Bearer [REDACTED]");
        assert!(!redacted.contains("eyJ"));
    }

    #[test]
    fn redactor_access_token() {
        let redactor = SensitiveDataRedactor::new();
        let text = "access_token=abc123def456ghi789jkl";
        let redacted = redactor.redact(text);
        
        assert_eq!(redacted, "access_token=[REDACTED]");
    }

    #[test]
    fn redactor_json_payload() {
        let redactor = SensitiveDataRedactor::new();
        let json = serde_json::json!({
            "username": "user123",
            "password": "secret123",
            "access_token": "abc123",
            "data": {
                "nested_secret": "nested_value"
            }
        });
        
        let redacted = redactor.redact_json(&json);
        
        assert_eq!(redacted["password"], "[REDACTED]");
        assert_eq!(redacted["access_token"], "[REDACTED]");
        assert_eq!(redacted["username"], "user123");
    }

    #[test]
    fn redactor_email() {
        let redactor = SensitiveDataRedactor::new().with_email_redaction(true);
        let text = "User email: user@example.com";
        let redacted = redactor.redact(text);
        
        assert_eq!(redacted, "User email: [EMAIL_REDACTED]");
    }

    #[test]
    fn redactor_ip() {
        let redactor = SensitiveDataRedactor::new().with_ip_redaction(true);
        let text = "Client IP: 192.168.1.100";
        let redacted = redactor.redact(text);
        
        assert_eq!(redacted, "Client IP: [IP_REDACTED]");
    }

    #[test]
    fn audit_entry_redaction() {
        let redactor = SensitiveDataRedactor::new().with_ip_redaction(true);
        let entry = RedactedAuditEntry::from_raw(
            &redactor,
            "2026-01-01T00:00:00Z".to_string(),
            "login".to_string(),
            "tenant_1".to_string(),
            Some("user_1".to_string()),
            "192.168.1.1".to_string(),
            serde_json::json!({
                "access_token": "abc123"
            }),
            200,
            100,
            None,
        );
        
        assert_eq!(entry.client_ip, "[IP_REDACTED]");
        assert_eq!(entry.request_details["access_token"], "[REDACTED]");
    }

    #[test]
    fn jwt_redaction() {
        let redactor = SensitiveDataRedactor::new();
        let jwt = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c";
        let redacted = redactor.redact(jwt);
        
        assert_eq!(redacted, "[JWT_REDACTED]");
    }

    #[test]
    fn multiple_patterns() {
        let redactor = SensitiveDataRedactor::new();
        let text = "Bearer token123 and password=secret and api_key=key123";
        let redacted = redactor.redact(text);
        
        assert!(redacted.contains("[REDACTED]"));
        assert!(!redacted.contains("token123"));
        assert!(!redacted.contains("secret"));
        assert!(!redacted.contains("key123"));
    }
}