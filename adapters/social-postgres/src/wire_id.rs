//! Positive Snowflake parsing for Social and Space PostgreSQL entity keys.

use im_platform_contracts::ContractError;

/// Parse the canonical decimal wire representation of a positive signed Snowflake ID.
///
/// Runtime entity IDs are never hashed or otherwise remapped. Rejecting non-canonical
/// input preserves a one-to-one relationship between the API, journal and normalized
/// PostgreSQL rows.
pub fn parse_social_entity_id(wire_id: &str) -> Result<i64, ContractError> {
    if wire_id.is_empty()
        || wire_id.starts_with('0')
        || !wire_id.bytes().all(|byte| byte.is_ascii_digit())
    {
        return Err(invalid_social_entity_id());
    }
    let value = wire_id
        .parse::<i64>()
        .map_err(|_| invalid_social_entity_id())?;
    if value <= 0 {
        return Err(invalid_social_entity_id());
    }
    Ok(value)
}

fn invalid_social_entity_id() -> ContractError {
    ContractError::Invalid(
        "social entity id must be a canonical positive signed int64 string".into(),
    )
}

#[cfg(test)]
mod tests {
    use super::parse_social_entity_id;

    #[test]
    fn numeric_wire_ids_pass_through() {
        assert_eq!(
            parse_social_entity_id("330339707122622464"),
            Ok(330339707122622464)
        );
    }

    #[test]
    fn rejects_non_canonical_or_non_positive_ids() {
        for value in ["", "0", "01", "-1", "fs_abc123", " 42", "42 "] {
            assert!(
                parse_social_entity_id(value).is_err(),
                "{value:?} must not be remapped into a PostgreSQL key"
            );
        }
    }

    #[test]
    fn rejects_values_outside_signed_bigint_range() {
        assert!(parse_social_entity_id("9223372036854775808").is_err());
    }
}
