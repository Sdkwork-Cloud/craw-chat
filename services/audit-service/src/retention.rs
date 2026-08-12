//! Audit retention vocabulary and expiry computation.
//!
//! `im_audit_records.retention_class` drives differentiated retention
//! (security = 2y, access = 180d, admin = 1y, data_lifecycle = 3y). The class
//! is derived from the action namespace at insert time so callers never
//! choose their own retention. Only the privileged retention purge may delete
//! expired rows (`im.retention_purge.operation_completed`).

use im_time::rfc3339_add_secs;

pub const AUDIT_RETENTION_CLASSES: &[&str] = &["security", "access", "admin", "data_lifecycle"];

pub const AUDIT_RETENTION_CLASS_DEFAULT: &str = "access";

/// Maps an audit action to its retention class by dotted namespace.
///
/// Actions outside the reserved namespaces default to `access` so unknown
/// future namespaces never inherit an indefinite or overly long window.
pub fn audit_retention_class_for_action(action: &str) -> &'static str {
    let namespace = action.split('.').next().unwrap_or(action).trim();
    match namespace {
        "security" => "security",
        "admin" => "admin",
        "data_lifecycle" => "data_lifecycle",
        "access" => "access",
        _ => AUDIT_RETENTION_CLASS_DEFAULT,
    }
}

/// Returns the retention window in whole days for an audit retention class.
pub fn audit_retention_duration_days(retention_class: &str) -> u64 {
    match retention_class.trim() {
        "security" => 730,
        "admin" => 365,
        "data_lifecycle" => 1_095,
        // "access" and any unknown class: 180 days.
        _ => 180,
    }
}

/// Computes the RFC3339 retention expiry timestamp for an audit record.
pub fn audit_retention_until(retention_class: &str, occurred_at: &str) -> Option<String> {
    let days = audit_retention_duration_days(retention_class);
    rfc3339_add_secs(occurred_at.trim(), days.saturating_mul(86_400) as i64)
}

pub fn is_canonical_audit_retention_class(retention_class: &str) -> bool {
    AUDIT_RETENTION_CLASSES.contains(&retention_class.trim())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audit_retention_class_from_action_namespace() {
        assert_eq!(
            audit_retention_class_for_action("security.login_failed"),
            "security"
        );
        assert_eq!(
            audit_retention_class_for_action("access.export_requested"),
            "access"
        );
        assert_eq!(
            audit_retention_class_for_action("admin.policy_committed"),
            "admin"
        );
        assert_eq!(
            audit_retention_class_for_action("data_lifecycle.purge_completed"),
            "data_lifecycle"
        );
        assert_eq!(
            audit_retention_class_for_action("control.provider_policy_committed"),
            AUDIT_RETENTION_CLASS_DEFAULT
        );
        assert_eq!(audit_retention_class_for_action(""), AUDIT_RETENTION_CLASS_DEFAULT);
    }

    #[test]
    fn test_audit_retention_durations_match_ddl_comment() {
        assert_eq!(audit_retention_duration_days("security"), 730);
        assert_eq!(audit_retention_duration_days("access"), 180);
        assert_eq!(audit_retention_duration_days("admin"), 365);
        assert_eq!(audit_retention_duration_days("data_lifecycle"), 1_095);
    }

    #[test]
    fn test_audit_retention_until_adds_duration_days() {
        let until = audit_retention_until("access", "2026-06-01T00:00:00.000Z")
            .expect("access retention should expire");
        assert_eq!(until, "2026-11-28T00:00:00.000Z");
    }

    #[test]
    fn test_canonical_audit_retention_classes() {
        for class in AUDIT_RETENTION_CLASSES {
            assert!(is_canonical_audit_retention_class(class));
        }
        assert!(!is_canonical_audit_retention_class("ephemeral"));
    }
}
