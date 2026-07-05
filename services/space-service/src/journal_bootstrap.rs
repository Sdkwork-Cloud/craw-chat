//! Space commit journal bootstrap aligned with social-service patterns.

use std::path::Path;

use im_adapters_local_disk::FileCommitJournal;
use im_adapters_local_memory::MemoryCommitJournal;
use im_adapters_postgres_journal::PostgresCommitJournal;
use im_adapters_social_postgres::{SocialPostgresConfig, SpacePostgresMaterializer};
use im_app_context::resolve_web_environment_from_process_env;
use im_platform_contracts::{CommitEnvelope, CommitJournal, ContractError};
use sdkwork_database_config::{DatabaseConfig, DatabaseEngine};
use sdkwork_web_core::WebEnvironment;
use tracing::info;

const IM_DATABASE_URL_ENV: &str = "SDKWORK_IM_DATABASE_URL";
const SPACE_COMMIT_PARTITION: &str = "control-plane-space";

/// Production-capable commit journal backend for space-service processes.
#[derive(Clone)]
pub enum SpaceCommitJournal {
    Memory(MemoryCommitJournal),
    File(FileCommitJournal),
    Postgres(PostgresCommitJournal),
}

impl CommitJournal for SpaceCommitJournal {
    fn append(&self, envelope: CommitEnvelope) -> Result<im_platform_contracts::CommitPosition, ContractError> {
        match self {
            Self::Memory(journal) => CommitJournal::append(journal, envelope),
            Self::File(journal) => CommitJournal::append(journal, envelope),
            Self::Postgres(journal) => CommitJournal::append(journal, envelope),
        }
    }

    fn append_batch(
        &self,
        envelopes: Vec<CommitEnvelope>,
    ) -> Result<Vec<im_platform_contracts::CommitPosition>, ContractError> {
        match self {
            Self::Memory(journal) => CommitJournal::append_batch(journal, envelopes),
            Self::File(journal) => CommitJournal::append_batch(journal, envelopes),
            Self::Postgres(journal) => CommitJournal::append_batch(journal, envelopes),
        }
    }

    fn recorded(&self) -> Result<Vec<CommitEnvelope>, ContractError> {
        match self {
            Self::Memory(journal) => CommitJournal::recorded(journal),
            Self::File(journal) => CommitJournal::recorded(journal),
            Self::Postgres(journal) => CommitJournal::recorded(journal),
        }
    }
}

impl SpaceCommitJournal {
    pub fn uses_postgres_authority(&self) -> bool {
        matches!(self, Self::Postgres(_))
    }
}

pub fn resolve_space_commit_journal_from_env() -> Result<SpaceCommitJournal, String> {
    if let Ok(config) = DatabaseConfig::from_env("IM") {
        if config.engine == DatabaseEngine::Postgres {
            let journal = im_adapters_postgres_journal::PostgresJournalConfig::from_database_config(
                &config,
            )
            .connect()
            .map_err(|error| format!("postgres commit journal bootstrap failed: {error:?}"))?;
            info!("space-runtime using postgres commit journal");
            return Ok(SpaceCommitJournal::Postgres(journal));
        }

        let environment = resolve_web_environment_from_process_env();
        if matches!(environment, WebEnvironment::Dev | WebEnvironment::Test) {
            sdkwork_im_database_pool::log_im_core_ephemeral_non_postgres_authority(
                "space-runtime",
                config.engine,
            );
            return Ok(SpaceCommitJournal::Memory(MemoryCommitJournal::default()));
        }

        return Err(
            "postgres commit journal is required in production when IM database engine is not postgres"
                .into(),
        );
    }

    if let Some(database_url) = resolve_im_database_url_from_env() {
        let journal = im_adapters_postgres_journal::PostgresJournalConfig::new(database_url)
            .connect()
            .map_err(|error| format!("postgres commit journal bootstrap failed: {error:?}"))?;
        info!("space-runtime using postgres commit journal");
        return Ok(SpaceCommitJournal::Postgres(journal));
    }

    if let Ok(runtime_dir) = std::env::var("SDKWORK_IM_RUNTIME_DIR") {
        if !runtime_dir.trim().is_empty() {
            let journal_path = Path::new(runtime_dir.trim())
                .join("state")
                .join("space-commit-journal.json");
            info!(
                journal_path = %journal_path.display(),
                "space-runtime using file commit journal"
            );
            return Ok(SpaceCommitJournal::File(FileCommitJournal::new(
                SPACE_COMMIT_PARTITION,
                journal_path,
            )));
        }
    }

    let environment = resolve_web_environment_from_process_env();
    if matches!(environment, WebEnvironment::Dev | WebEnvironment::Test) {
        info!("space-runtime using in-memory commit journal (development only)");
        return Ok(SpaceCommitJournal::Memory(MemoryCommitJournal::default()));
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

#[allow(dead_code)]
pub fn resolve_space_postgres_pool_from_env(
) -> Option<im_adapters_social_postgres::SocialPostgresPool> {
    if let Ok(config) = DatabaseConfig::from_env("IM") {
        if config.engine == DatabaseEngine::Postgres {
            return SocialPostgresConfig::from_database_config(&config)
                .connect_pool()
                .ok();
        }
    }

    resolve_im_database_url_from_env().and_then(|database_url| {
        SocialPostgresConfig::new(database_url).connect_pool().ok()
    })
}

pub fn replay_space_journal_to_read_model(
    journal: &SpaceCommitJournal,
    materializer: &SpacePostgresMaterializer,
) {
    let commits = match journal.recorded() {
        Ok(commits) => commits,
        Err(error) => {
            tracing::warn!(
                error = ?error,
                "space read-model replay skipped because commit journal readback is unavailable"
            );
            return;
        }
    };
    if commits.is_empty() {
        return;
    }
    let failures = materializer.try_materialize_commits(commits.as_slice());
    if failures > 0 {
        crate::space_materializer_metrics::record_postgres_materialization_failures(
            failures as u64,
        );
    }
    info!(
        commit_count = commits.len(),
        materialization_failures = failures,
        "replayed space commit journal into supplemental postgres read model"
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use im_domain_events::CommitEnvelope;

    #[test]
    fn memory_journal_variant_delegates_append() {
        let journal = SpaceCommitJournal::Memory(MemoryCommitJournal::default());
        let envelope = CommitEnvelope::minimal(
            "evt-space-1",
            "100001",
            "space.created",
            "space",
            "space-1",
            1,
        );
        let position = journal.append(envelope).expect("append should succeed");
        assert_eq!(position.offset, 1);
        assert_eq!(position.partition, "local-memory");
    }
}
