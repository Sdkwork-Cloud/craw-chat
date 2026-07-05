//! IM core database engine policy.
//!
//! Durable IM authority (journal, projection, social materializer, search) is
//! PostgreSQL-only. Desktop `SDKWORK_IM_DATABASE_ENGINE=sqlite` keeps sibling
//! module sqlite files under the user data directory but IM services use
//! in-memory ephemeral state in dev/test only.

use sdkwork_database_config::{DatabaseConfig, DatabaseEngine};

/// Durable IM runtime requires PostgreSQL.
pub fn im_core_requires_postgres_authority(config: &DatabaseConfig) -> bool {
    config.engine == DatabaseEngine::Postgres
}

/// Log once when IM services fall back to ephemeral authority for non-Postgres engine.
pub fn log_im_core_ephemeral_non_postgres_authority(service: &str, engine: DatabaseEngine) {
    tracing::info!(
        service,
        engine = %engine,
        "IM core durable authority requires PostgreSQL; using in-memory ephemeral state (desktop sqlite dev mode does not persist IM journal/projection)"
    );
}
