//! Automation service runtime bootstrap from process environment.

use std::sync::{Arc, OnceLock};

use im_adapters_local_disk::FileAutomationExecutionStore;
use im_adapters_local_memory::MemoryAutomationExecutionStore;
use im_adapters_postgres_journal::{
    PostgresAutomationExecutionStore, PostgresCommitJournal, PostgresJournalConfig,
};
use im_app_context::{
    allows_header_only_app_context_fallback, resolve_web_environment_from_process_env,
};
use sdkwork_database_config::{DatabaseConfig, DatabaseEngine};
use sdkwork_im_contract_agent::AutomationExecutionStore;
use sdkwork_im_contract_core::ContractError;
use sdkwork_im_contract_message::{CommitEnvelope, CommitJournal, CommitPosition};
use sdkwork_web_core::WebEnvironment;
use tracing::info;

use crate::runtime::AutomationRuntime;
use crate::state::AppState;

const IM_DATABASE_URL_ENV: &str = "SDKWORK_IM_DATABASE_URL";
const AUTOMATION_EXECUTION_STORE_FILE_ENV: &str = "SDKWORK_IM_AUTOMATION_EXECUTION_STORE_FILE";

static DEFAULT_AUTOMATION_RUNTIME: OnceLock<Arc<AutomationRuntime>> = OnceLock::new();

pub fn default_automation_runtime() -> Arc<AutomationRuntime> {
    if should_use_ephemeral_automation_runtime() {
        return build_runtime_from_env().unwrap_or_else(|_| Arc::new(AutomationRuntime::default()));
    }

    DEFAULT_AUTOMATION_RUNTIME
        .get_or_init(|| build_automation_runtime_or_fallback())
        .clone()
}

fn should_use_ephemeral_automation_runtime() -> bool {
    matches!(
        resolve_web_environment_from_process_env(),
        WebEnvironment::Dev | WebEnvironment::Test
    )
}

/// Fail-closed durable bootstrap for production process entrypoints.
pub fn ensure_durable_automation_runtime_from_env() -> Result<(), String> {
    build_runtime_from_env().map(|_| ())
}

pub fn build_runtime_from_env() -> Result<Arc<AutomationRuntime>, String> {
    let journal = resolve_automation_commit_journal_from_env()?;
    let store = resolve_automation_execution_store_from_env(&journal)?;
    Ok(Arc::new(AutomationRuntime::with_dyn_execution_store(
        journal, store,
    )))
}

pub fn default_app_state() -> AppState {
    AppState {
        runtime: default_automation_runtime(),
    }
}

fn build_automation_runtime_or_fallback() -> Arc<AutomationRuntime> {
    match build_runtime_from_env() {
        Ok(runtime) => runtime,
        Err(error) if allows_header_only_app_context_fallback() => {
            tracing::warn!(
                error = %error,
                "automation-service bootstrap unavailable; using in-memory runtime fallback (development/test only)"
            );
            Arc::new(AutomationRuntime::default())
        }
        Err(error) => {
            panic!(
                "automation-service durable bootstrap failed in production-like environment: {error}"
            );
        }
    }
}

enum AutomationCommitJournal {
    Memory(NoopJournalForDev),
    Postgres(PostgresCommitJournal),
}

impl CommitJournal for AutomationCommitJournal {
    fn append(&self, envelope: CommitEnvelope) -> Result<CommitPosition, ContractError> {
        match self {
            Self::Memory(journal) => journal.append(envelope),
            Self::Postgres(journal) => journal.append(envelope),
        }
    }
}

fn resolve_automation_execution_store_from_env(
    journal: &Arc<AutomationCommitJournal>,
) -> Result<Arc<dyn AutomationExecutionStore>, String> {
    if let Some(path) = resolve_automation_execution_store_path_from_env() {
        let environment = resolve_web_environment_from_process_env();
        if !matches!(environment, WebEnvironment::Dev | WebEnvironment::Test) {
            return Err(format!(
                "file-backed automation execution store is single-node development storage and is forbidden in production; configure PostgreSQL with {IM_DATABASE_URL_ENV}"
            ));
        }
        info!(
            path = %path,
            "automation-service using file-backed automation execution store"
        );
        return Ok(Arc::new(FileAutomationExecutionStore::new(path)));
    }

    if let AutomationCommitJournal::Postgres(pg_journal) = journal.as_ref() {
        info!("automation-service using postgres automation execution store");
        return Ok(Arc::new(PostgresAutomationExecutionStore::from_pool(
            pg_journal.pool().clone(),
        )));
    }

    let environment = resolve_web_environment_from_process_env();
    if matches!(environment, WebEnvironment::Dev | WebEnvironment::Test) {
        info!("automation-service using in-memory automation execution store (development only)");
        return Ok(Arc::new(MemoryAutomationExecutionStore::default()));
    }

    Err(format!(
        "PostgreSQL automation execution store is required in production: set {IM_DATABASE_URL_ENV}"
    ))
}

fn resolve_automation_commit_journal_from_env() -> Result<Arc<AutomationCommitJournal>, String> {
    if let Ok(config) = DatabaseConfig::from_env("IM") {
        if config.engine == DatabaseEngine::Postgres {
            let journal = PostgresJournalConfig::from_database_config(&config)
                .connect()
                .map_err(|error| {
                    format!("postgres automation journal bootstrap failed: {error:?}")
                })?;
            info!("automation-service using postgres commit journal");
            return Ok(Arc::new(AutomationCommitJournal::Postgres(journal)));
        }
    }

    if let Some(database_url) = resolve_im_database_url_from_env() {
        let journal = PostgresJournalConfig::new(database_url)
            .connect()
            .map_err(|error| format!("postgres automation journal bootstrap failed: {error:?}"))?;
        info!("automation-service using postgres commit journal");
        return Ok(Arc::new(AutomationCommitJournal::Postgres(journal)));
    }

    let environment = resolve_web_environment_from_process_env();
    if matches!(environment, WebEnvironment::Dev | WebEnvironment::Test) {
        info!("automation-service using in-memory commit journal (development only)");
        return Ok(Arc::new(AutomationCommitJournal::Memory(
            NoopJournalForDev::default(),
        )));
    }

    Err(format!(
        "postgres automation commit journal is required in production: set {IM_DATABASE_URL_ENV}"
    ))
}

fn resolve_automation_execution_store_path_from_env() -> Option<String> {
    std::env::var(AUTOMATION_EXECUTION_STORE_FILE_ENV)
        .ok()
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())
}

fn resolve_im_database_url_from_env() -> Option<String> {
    std::env::var(IM_DATABASE_URL_ENV)
        .ok()
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())
}

#[derive(Default)]
struct NoopJournalForDev;

impl CommitJournal for NoopJournalForDev {
    fn append(&self, _envelope: CommitEnvelope) -> Result<CommitPosition, ContractError> {
        Ok(CommitPosition::new("noop", 0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

    #[test]
    fn production_requires_durable_automation_backends() {
        let _guard = ENV_LOCK
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        let database_url = std::env::var(IM_DATABASE_URL_ENV).ok();
        let execution_store_file = std::env::var(AUTOMATION_EXECUTION_STORE_FILE_ENV).ok();
        let im_env = std::env::var("SDKWORK_IM_ENVIRONMENT").ok();
        unsafe {
            std::env::remove_var(IM_DATABASE_URL_ENV);
            std::env::remove_var(AUTOMATION_EXECUTION_STORE_FILE_ENV);
            std::env::set_var("SDKWORK_IM_ENVIRONMENT", "prod");
        }

        assert!(resolve_automation_commit_journal_from_env().is_err());
        assert!(build_runtime_from_env().is_err());

        unsafe {
            if let Some(value) = database_url {
                std::env::set_var(IM_DATABASE_URL_ENV, value);
            } else {
                std::env::remove_var(IM_DATABASE_URL_ENV);
            }
            if let Some(value) = execution_store_file {
                std::env::set_var(AUTOMATION_EXECUTION_STORE_FILE_ENV, value);
            } else {
                std::env::remove_var(AUTOMATION_EXECUTION_STORE_FILE_ENV);
            }
            if let Some(value) = im_env {
                std::env::set_var("SDKWORK_IM_ENVIRONMENT", value);
            } else {
                std::env::remove_var("SDKWORK_IM_ENVIRONMENT");
            }
        }
    }

    #[test]
    fn production_rejects_file_backed_automation_store() {
        let _guard = ENV_LOCK
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        let execution_store_file = std::env::var(AUTOMATION_EXECUTION_STORE_FILE_ENV).ok();
        let im_env = std::env::var("SDKWORK_IM_ENVIRONMENT").ok();
        unsafe {
            std::env::set_var(AUTOMATION_EXECUTION_STORE_FILE_ENV, "automation-test.json");
            std::env::set_var("SDKWORK_IM_ENVIRONMENT", "prod");
        }
        let journal = Arc::new(AutomationCommitJournal::Memory(NoopJournalForDev));
        let result = resolve_automation_execution_store_from_env(&journal);
        let error = result
            .err()
            .expect("production file store must fail closed");
        assert!(error.contains("forbidden in production"));

        unsafe {
            if let Some(value) = execution_store_file {
                std::env::set_var(AUTOMATION_EXECUTION_STORE_FILE_ENV, value);
            } else {
                std::env::remove_var(AUTOMATION_EXECUTION_STORE_FILE_ENV);
            }
            if let Some(value) = im_env {
                std::env::set_var("SDKWORK_IM_ENVIRONMENT", value);
            } else {
                std::env::remove_var("SDKWORK_IM_ENVIRONMENT");
            }
        }
    }
}
