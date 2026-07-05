//! Stable mapping from social wire identifiers to PostgreSQL BIGINT keys.

use sdkwork_utils_rust::sha256_hash;

/// Map a social wire identifier to a stable positive `i64` for PostgreSQL storage.
///
/// Numeric snowflake strings pass through unchanged. Deterministic social ids such as
/// `fs_*` / `dc_*` are hashed so supplemental Postgres stores stay aligned with the
/// event-sourced runtime without changing the wire format exposed to clients.
pub fn social_entity_id_to_i64(wire_id: &str) -> i64 {
    let trimmed = wire_id.trim();
    if let Ok(value) = trimmed.parse::<i64>() {
        if value > 0 {
            return value;
        }
    }

    let digest = sha256_hash(trimmed.as_bytes());
    let hex_prefix = digest.get(..16).unwrap_or(digest.as_str());
    let value = u64::from_str_radix(hex_prefix, 16).unwrap_or(1);
    (value as i64) & i64::MAX
}

#[cfg(test)]
mod tests {
    use super::social_entity_id_to_i64;

    #[test]
    fn numeric_wire_ids_pass_through() {
        assert_eq!(social_entity_id_to_i64("330339707122622464"), 330339707122622464);
    }

    #[test]
    fn deterministic_ids_hash_stably() {
        let first = social_entity_id_to_i64("fs_abc123def456789012345678");
        let second = social_entity_id_to_i64("fs_abc123def456789012345678");
        assert_eq!(first, second);
        assert!(first > 0);
    }
}
