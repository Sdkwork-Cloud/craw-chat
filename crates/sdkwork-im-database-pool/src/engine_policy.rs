//! IM core database engine policy.
//!
//! Durable IM authority (normalized state, journal evidence, social state, search) is
//! PostgreSQL-only. Client-local SQLite belongs to the owning native PC adapter and
//! is never selected by an IM server process.

use sdkwork_database_config::{DatabaseConfig, DatabaseEngine};

/// Durable IM runtime requires PostgreSQL.
pub fn im_core_requires_postgres_authority(config: &DatabaseConfig) -> bool {
    config.engine == DatabaseEngine::Postgres
}

/// Reject any server configuration that does not select PostgreSQL authority.
pub fn ensure_im_core_postgres_authority(config: &DatabaseConfig) -> Result<(), String> {
    if im_core_requires_postgres_authority(config) {
        return Ok(());
    }
    Err(
        "IM authoritative server persistence requires PostgreSQL; SQLite is client-local only"
            .to_owned(),
    )
}
