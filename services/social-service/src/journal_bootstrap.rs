//! PostgreSQL-only Social runtime bootstrap.

use std::sync::Arc;

use im_adapters_postgres_journal::{
    PostgresCommitJournal, PostgresJournalPool, PostgresOutboxStore,
};
use im_platform_contracts::OutboxStore;
use sdkwork_database_config::{DatabaseConfig, DatabaseEngine};
use sdkwork_im_runtime_id::build_runtime_id_generator_blocking;
use tracing::info;

use crate::runtime::{SocialRuntime, SocialStateStore};

const IM_DATABASE_URL_ENV: &str = "SDKWORK_IM_DATABASE_URL";

/// Build a social runtime from process environment.
pub fn build_social_runtime_from_env() -> Result<Arc<SocialRuntime>, String> {
    let id_generator = build_runtime_id_generator_blocking("social-service");
    let postgres_journal = resolve_social_commit_journal_from_env()?;
    let pool = im_adapters_social_postgres::SocialPostgresPool::new(
        postgres_journal.pool().inner().clone(),
    );
    let runtime = SocialRuntime::new(
        SocialStateStore::database(pool.clone()),
        Arc::new(postgres_journal.clone()),
    )
    .with_outbox_store(Arc::new(PostgresOutboxStore::from_pool(
        postgres_journal.pool().clone(),
    )) as Arc<dyn OutboxStore>)
    .with_postgres_write_authority(postgres_journal, pool.clone())
    .with_user_directory(crate::user_directory::resolve_social_user_directory_from_pool(Some(pool)))
    .with_id_generator(id_generator);
    Ok(Arc::new(runtime))
}

pub fn resolve_social_commit_journal_from_env() -> Result<PostgresCommitJournal, String> {
    let config = DatabaseConfig::from_env("IM").map_err(|_| {
        format!("social runtime requires PostgreSQL configuration in {IM_DATABASE_URL_ENV}")
    })?;
    if config.engine != DatabaseEngine::Postgres {
        return Err("social runtime requires PostgreSQL; SQLite is client-local only".to_owned());
    }

    let pool = sdkwork_im_database_pool::clone_shared_im_postgres_r2d2_pool()
        .ok_or_else(|| "social runtime requires the installed process PostgreSQL pool".to_owned())?;
    let journal = PostgresCommitJournal::from_pool(PostgresJournalPool::from_pool(pool));
    info!("social-runtime using postgres commit journal");
    Ok(journal)
}

pub fn resolve_social_postgres_pool_from_env()
-> Option<im_adapters_social_postgres::SocialPostgresPool> {
    sdkwork_im_database_pool::clone_shared_im_postgres_r2d2_pool()
        .map(im_adapters_social_postgres::SocialPostgresPool::new)
}
