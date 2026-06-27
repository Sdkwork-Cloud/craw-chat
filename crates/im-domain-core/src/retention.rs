use chrono::{DateTime, Duration, FixedOffset, Utc};

/// Derives the retention class token from a policy reference such as `tenant.standard`.
pub fn retention_class_from_policy_ref(retention_policy_ref: &str) -> String {
    let retention_class = retention_policy_ref
        .rsplit('.')
        .next()
        .unwrap_or(retention_policy_ref)
        .trim();
    if retention_class.is_empty() {
        "standard".into()
    } else {
        retention_class.into()
    }
}

/// Returns the retention window in whole days for a class, or `None` when data must be kept
/// indefinitely (for example `legal_hold`).
pub fn retention_duration_days(retention_class: &str) -> Option<u64> {
    match retention_class.trim() {
        "ephemeral" => Some(7),
        "standard" => Some(365),
        "extended" => Some(2_555),
        "legal_hold" => None,
        "" => Some(365),
        _ => Some(365),
    }
}

/// Computes the RFC3339 expiry timestamp for a committed event.
pub fn retention_until_from_class(retention_class: &str, occurred_at: &str) -> Option<String> {
    let days = retention_duration_days(retention_class)?;
    let anchor = parse_rfc3339(occurred_at)?;
    Some(format_rfc3339(anchor + Duration::days(days as i64)))
}

pub fn retention_until_from_policy_ref(
    retention_policy_ref: &str,
    occurred_at: &str,
) -> Option<String> {
    retention_until_from_class(
        retention_class_from_policy_ref(retention_policy_ref).as_str(),
        occurred_at,
    )
}

pub fn retention_until_from_envelope(retention_class: &str, occurred_at: &str) -> Option<String> {
    retention_until_from_class(retention_class, occurred_at)
}

pub fn retention_is_indefinite(retention_class: &str) -> bool {
    retention_duration_days(retention_class).is_none()
}

/// Canonical retention class tokens published through governance vocabulary.
pub const CANONICAL_RETENTION_CLASSES: &[&str] =
    &["ephemeral", "standard", "extended", "legal_hold"];

pub fn canonical_retention_classes() -> &'static [&'static str] {
    CANONICAL_RETENTION_CLASSES
}

pub fn is_retention_expired(retention_until: Option<&str>, now: &str) -> bool {
    let Some(until) = retention_until.map(str::trim).filter(|value| !value.is_empty()) else {
        return false;
    };
    match (parse_rfc3339(now), parse_rfc3339(until)) {
        (Some(now), Some(until)) => now >= until,
        _ => false,
    }
}

fn parse_rfc3339(value: &str) -> Option<DateTime<FixedOffset>> {
    DateTime::parse_from_rfc3339(value.trim())
        .ok()
        .or_else(|| {
            value
                .trim()
                .parse::<DateTime<Utc>>()
                .ok()
                .map(|instant| instant.fixed_offset())
        })
}

fn format_rfc3339(value: DateTime<FixedOffset>) -> String {
    value.with_timezone(&Utc).format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string()
}

#[cfg(test)]
mod tests {
    use super::{
        canonical_retention_classes, is_retention_expired, retention_class_from_policy_ref,
        retention_duration_days, retention_is_indefinite, retention_until_from_class,
        retention_until_from_policy_ref,
    };

    #[test]
    fn test_retention_class_from_policy_ref_uses_suffix_token() {
        assert_eq!(
            retention_class_from_policy_ref("tenant.standard"),
            "standard"
        );
        assert_eq!(
            retention_class_from_policy_ref("space.extended"),
            "extended"
        );
    }

    #[test]
    fn test_retention_until_from_class_adds_duration_days() {
        let until = retention_until_from_class("standard", "2026-01-01T00:00:00.000Z")
            .expect("standard retention should expire");
        assert_eq!(until, "2027-01-01T00:00:00.000Z");
    }

    #[test]
    fn test_retention_until_from_policy_ref_matches_class_derivation() {
        let until = retention_until_from_policy_ref("tenant.ephemeral", "2026-06-01T00:00:00.000Z")
            .expect("ephemeral retention should expire");
        assert_eq!(until, "2026-06-08T00:00:00.000Z");
    }

    #[test]
    fn test_legal_hold_retention_has_no_expiry() {
        assert!(retention_duration_days("legal_hold").is_none());
        assert!(retention_until_from_class("legal_hold", "2026-01-01T00:00:00.000Z").is_none());
    }

    #[test]
    fn test_legal_hold_policy_ref_is_indefinite() {
        assert!(retention_is_indefinite(
            retention_class_from_policy_ref("tenant.legal_hold").as_str()
        ));
        assert!(!retention_is_indefinite("standard"));
    }

    #[test]
    fn test_canonical_retention_classes_match_duration_table() {
        for class in canonical_retention_classes() {
            assert!(
                retention_duration_days(class).is_some() || *class == "legal_hold",
                "canonical class {class} must be recognized"
            );
        }
    }

    #[test]
    fn test_is_retention_expired_compares_instant_order() {
        assert!(is_retention_expired(
            Some("2026-01-01T00:00:00.000Z"),
            "2026-06-01T00:00:00.000Z"
        ));
        assert!(!is_retention_expired(
            Some("2026-12-01T00:00:00.000Z"),
            "2026-06-01T00:00:00.000Z"
        ));
        assert!(!is_retention_expired(None, "2026-06-01T00:00:00.000Z"));
    }
}
