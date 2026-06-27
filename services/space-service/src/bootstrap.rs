//! Bootstrap helpers for space-service process startup.

use std::sync::Arc;

use axum::Router;
use im_adapters_social_postgres::organization_store::{
    PostgresChannelStore, PostgresGroupStore, PostgresSpaceStore,
};
use im_adapters_social_postgres::{SocialPostgresConfig, SocialPostgresPool};

use crate::http::{AppState, build_embedded_app, build_public_app};
use crate::id::build_runtime_id_generator;

/// Environment variable name for database connection URL.
/// Referenced in doc comments but loaded via configuration system.
#[allow(dead_code)]
pub const DATABASE_URL_ENV: &str = "SDKWORK_IM_DATABASE_URL";

pub async fn app_state_from_postgres_pool(pool: SocialPostgresPool) -> AppState {
    let pool_arc = Arc::new(pool.inner().clone());
    AppState {
        space_store: Arc::new(PostgresSpaceStore::new(pool_arc.clone())),
        group_store: Arc::new(PostgresGroupStore::new(pool_arc.clone())),
        channel_store: Arc::new(PostgresChannelStore::new(pool_arc)),
        id_generator: build_runtime_id_generator().await,
    }
}

/// Resolves space [`AppState`] when the IM PostgreSQL database is configured.
pub async fn try_app_state_from_database_url_env() -> Option<AppState> {
    let config = sdkwork_database_config::DatabaseConfig::from_env("IM").ok()?;
    if config.engine != sdkwork_database_config::DatabaseEngine::Postgres {
        return None;
    }
    let pool = SocialPostgresConfig::from_database_config(&config)
        .connect_pool()
        .ok()?;
    Some(app_state_from_postgres_pool(pool).await)
}

/// Builds the embedded space router when `SDKWORK_IM_DATABASE_URL` is configured.
pub async fn try_build_embedded_app_from_database_url_env() -> Option<Router> {
    try_app_state_from_database_url_env()
        .await
        .map(build_embedded_app)
}

/// Builds the public space router when `SDKWORK_IM_DATABASE_URL` is configured.
pub async fn try_build_public_app_from_database_url_env() -> Option<Router> {
    try_app_state_from_database_url_env()
        .await
        .map(build_public_app)
}
