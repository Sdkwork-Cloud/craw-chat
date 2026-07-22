//! Social commit journal bootstrap aligned with conversation-service patterns.

use std::path::Path;
use std::sync::Arc;

use im_adapters_local_disk::FileCommitJournal;
use im_adapters_local_memory::MemoryCommitJournal;
use im_adapters_postgres_journal::{
    PostgresCommitJournal, PostgresJournalConfig, PostgresOutboxStore,
};
use im_app_context::resolve_web_environment_from_process_env;
use im_platform_contracts::OutboxStore;
use im_platform_contracts::{
    CommitEnvelope, CommitJournal, CommitJournalAggregateScope, CommitJournalReplayCursor,
    CommitJournalReplayPage, ContractError,
};
use sdkwork_database_config::{DatabaseConfig, DatabaseEngine};
use sdkwork_im_runtime_id::build_runtime_id_generator_blocking;
use sdkwork_web_core::WebEnvironment;
use tracing::info;

use crate::conversation_state_bridge::try_apply_social_commits_to_conversation_state;
use crate::runtime::{SocialRuntime, SocialStateStore};
use im_adapters_social_postgres::SocialPostgresConfig;

const IM_DATABASE_URL_ENV: &str = "SDKWORK_IM_DATABASE_URL";
const SOCIAL_COMMIT_PARTITION: &str = "control-plane-social";

/// Production-capable commit journal backend for social runtime processes.
#[derive(Clone)]
pub enum SocialCommitJournal {
    Memory(MemoryCommitJournal),
    File(FileCommitJournal),
    Postgres(PostgresCommitJournal),
}

impl CommitJournal for SocialCommitJournal {
    fn append(
        &self,
        envelope: CommitEnvelope,
    ) -> Result<im_platform_contracts::CommitPosition, ContractError> {
        let position = match self {
            Self::Memory(journal) => CommitJournal::append(journal, envelope.clone()),
            Self::File(journal) => CommitJournal::append(journal, envelope.clone()),
            Self::Postgres(journal) => CommitJournal::append(journal, envelope.clone()),
        }?;
        try_apply_social_commits_to_conversation_state(std::slice::from_ref(&envelope));
        Ok(position)
    }

    fn append_batch(
        &self,
        envelopes: Vec<CommitEnvelope>,
    ) -> Result<Vec<im_platform_contracts::CommitPosition>, ContractError> {
        let positions = match self {
            Self::Memory(journal) => CommitJournal::append_batch(journal, envelopes.clone()),
            Self::File(journal) => CommitJournal::append_batch(journal, envelopes.clone()),
            Self::Postgres(journal) => CommitJournal::append_batch(journal, envelopes.clone()),
        }?;
        try_apply_social_commits_to_conversation_state(&envelopes);
        Ok(positions)
    }

    fn recorded(&self) -> Result<Vec<CommitEnvelope>, ContractError> {
        match self {
            Self::Memory(journal) => CommitJournal::recorded(journal),
            Self::File(journal) => CommitJournal::recorded(journal),
            Self::Postgres(journal) => CommitJournal::recorded(journal),
        }
    }

    fn recorded_page(
        &self,
        cursor: Option<&CommitJournalReplayCursor>,
        limit: usize,
    ) -> Result<CommitJournalReplayPage, ContractError> {
        match self {
            Self::Memory(journal) => CommitJournal::recorded_page(journal, cursor, limit),
            Self::File(journal) => CommitJournal::recorded_page(journal, cursor, limit),
            Self::Postgres(journal) => CommitJournal::recorded_page(journal, cursor, limit),
        }
    }

    fn recorded_page_for_aggregate(
        &self,
        scope: &CommitJournalAggregateScope,
        cursor: Option<&CommitJournalReplayCursor>,
        limit: usize,
    ) -> Result<CommitJournalReplayPage, ContractError> {
        match self {
            Self::Memory(journal) => {
                CommitJournal::recorded_page_for_aggregate(journal, scope, cursor, limit)
            }
            Self::File(journal) => {
                CommitJournal::recorded_page_for_aggregate(journal, scope, cursor, limit)
            }
            Self::Postgres(journal) => {
                CommitJournal::recorded_page_for_aggregate(journal, scope, cursor, limit)
            }
        }
    }
}

impl SocialCommitJournal {
    pub fn uses_postgres_authority(&self) -> bool {
        matches!(self, Self::Postgres(_))
    }
}

/// Build a social runtime from process environment.
///
/// Priority:
/// 1. Postgres `im_commit_journal` when IM database is configured.
/// 2. File journal under `SDKWORK_IM_RUNTIME_DIR/state` when runtime dir is set.
/// 3. In-memory journal for development and tests.
pub fn build_social_runtime_from_env() -> Result<Arc<SocialRuntime>, String> {
    let id_generator = build_runtime_id_generator_blocking("social-service");
    let journal = resolve_social_commit_journal_with_id_generator(id_generator.clone())?;
    let uses_postgres = journal.uses_postgres_authority();

    if uses_postgres {
        let mut runtime = SocialRuntime::new_with_journal_authority(
            SocialStateStore::memory(),
            Arc::new(journal.clone()),
            true,
        );
        if let SocialCommitJournal::Postgres(postgres_journal) = &journal {
            let pool = im_adapters_social_postgres::SocialPostgresPool::new(
                postgres_journal.pool().inner().clone(),
            );
            runtime = runtime
                .with_outbox_store(Arc::new(PostgresOutboxStore::from_pool(
                    postgres_journal.pool().clone(),
                )) as Arc<dyn OutboxStore>)
                .with_postgres_write_authority(postgres_journal.clone(), pool.clone())
                .with_user_directory(
                    crate::user_directory::resolve_social_user_directory_from_pool(Some(pool)),
                )
                .with_id_generator(id_generator);
        }
        return Ok(Arc::new(runtime));
    }

    if let Ok(runtime_dir) = std::env::var("SDKWORK_IM_RUNTIME_DIR")
        && !runtime_dir.trim().is_empty()
    {
        return Ok(Arc::new(SocialRuntime::from_runtime_dir(
            runtime_dir.as_str(),
        )));
    }

    Ok(Arc::new(SocialRuntime::new_with_journal_authority(
        SocialStateStore::memory(),
        Arc::new(journal),
        false,
    )))
}

pub fn resolve_social_commit_journal_from_env() -> Result<SocialCommitJournal, String> {
    let id_generator = build_runtime_id_generator_blocking("social-service");
    resolve_social_commit_journal_with_id_generator(id_generator)
}

pub fn resolve_social_commit_journal_with_id_generator(
    _id_generator: Arc<dyn im_platform_contracts::IdGenerator>,
) -> Result<SocialCommitJournal, String> {
    if let Ok(config) = DatabaseConfig::from_env("IM") {
        if config.engine == DatabaseEngine::Postgres {
            let journal = PostgresJournalConfig::from_database_config(&config)
                .connect()
                .map_err(|error| format!("postgres commit journal bootstrap failed: {error:?}"))?;
            info!("social-runtime using postgres commit journal");
            return Ok(SocialCommitJournal::Postgres(journal));
        }

        let environment = resolve_web_environment_from_process_env();
        if matches!(environment, WebEnvironment::Dev | WebEnvironment::Test) {
            sdkwork_im_database_pool::log_im_core_ephemeral_non_postgres_authority(
                "social-runtime",
                config.engine,
            );
            return Ok(SocialCommitJournal::Memory(MemoryCommitJournal::default()));
        }

        return Err(
            "postgres commit journal is required in production when IM database engine is not postgres"
                .into(),
        );
    }

    if let Some(database_url) = resolve_im_database_url_from_env() {
        let journal = PostgresJournalConfig::new(database_url)
            .connect()
            .map_err(|error| format!("postgres commit journal bootstrap failed: {error:?}"))?;
        info!("social-runtime using postgres commit journal");
        return Ok(SocialCommitJournal::Postgres(journal));
    }

    if let Ok(runtime_dir) = std::env::var("SDKWORK_IM_RUNTIME_DIR")
        && !runtime_dir.trim().is_empty()
    {
        let journal_path = Path::new(runtime_dir.trim())
            .join("state")
            .join("social-commit-journal.json");
        info!(
            journal_path = %journal_path.display(),
            "social-runtime using file commit journal"
        );
        return Ok(SocialCommitJournal::File(FileCommitJournal::new(
            SOCIAL_COMMIT_PARTITION,
            journal_path,
        )));
    }

    let environment = resolve_web_environment_from_process_env();
    if matches!(environment, WebEnvironment::Dev | WebEnvironment::Test) {
        info!("social-runtime using in-memory commit journal (development only)");
        return Ok(SocialCommitJournal::Memory(MemoryCommitJournal::default()));
    }

    Err(format!(
        "postgres commit journal is required in production: set {IM_DATABASE_URL_ENV}"
    ))
}

fn resolve_im_database_url_from_env() -> Option<String> {
    std::env::var(IM_DATABASE_URL_ENV)
        .ok()
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())
}

pub fn resolve_social_postgres_pool_from_env()
-> Option<im_adapters_social_postgres::SocialPostgresPool> {
    if let Ok(config) = DatabaseConfig::from_env("IM")
        && config.engine == DatabaseEngine::Postgres
    {
        return SocialPostgresConfig::from_database_config(&config)
            .connect_pool()
            .ok();
    }

    resolve_im_database_url_from_env()
        .and_then(|database_url| SocialPostgresConfig::new(database_url).connect_pool().ok())
}

#[cfg(test)]
mod tests {
    use super::*;
    use im_domain_events::CommitEnvelope;

    #[test]
    fn memory_journal_variant_delegates_append() {
        let journal = SocialCommitJournal::Memory(MemoryCommitJournal::default());
        let envelope = CommitEnvelope::minimal(
            "evt-social-1",
            "100001",
            "friend_request.submitted",
            "friend_request",
            "fr-1",
            1,
        );
        let position = journal.append(envelope).expect("append should succeed");
        assert_eq!(position.offset, 1);
        assert_eq!(position.partition, "local-memory");
    }
}
