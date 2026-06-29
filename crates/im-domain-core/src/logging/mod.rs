//! Logging utilities and sensitive data redaction.
//!
//! This module provides logging infrastructure including automatic
//! redaction of sensitive data in log output and audit trails.

pub mod redactor;

pub use redactor::{RedactedAuditEntry, SensitiveDataRedactor};