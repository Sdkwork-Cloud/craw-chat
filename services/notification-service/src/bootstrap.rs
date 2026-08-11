//! Notification service runtime bootstrap from process environment.

use std::sync::{Arc, OnceLock};

use conversation_runtime::conversation_state::ConversationStateService;
use im_adapters_local_disk::FileNotificationTaskStore;
use im_adapters_local_memory::MemoryNotificationTaskStore;
use im_adapters_postgres_journal::{
    PostgresAggregateStore, PostgresCommitJournal, PostgresJournalConfig,
    PostgresNotificationTaskStore,
};
use im_app_context::{
    allows_header_only_app_context_fallback, resolve_web_environment_from_process_env,
};
use sdkwork_database_config::{DatabaseConfig, DatabaseEngine};
use sdkwork_im_contract_core::ContractError;
use sdkwork_im_contract_message::{CommitEnvelope, CommitJournal, CommitPosition};
use sdkwork_im_contract_notification::NotificationTaskStore;
use sdkwork_web_core::WebEnvironment;
use tracing::info;

use crate::state::{AppState, NotificationRuntime};

const IM_DATABASE_URL_ENV: &str = "SDKWORK_DATABASE_URL";
const NOTIFICATION_TASK_STORE_FILE_ENV: &str = "SDKWORK_IM_NOTIFICATION_TASK_STORE_FILE";

static DEFAULT_NOTIFICATION_RUNTIME: OnceLock<Arc<NotificationRuntime>> = OnceLock::new();

pub fn default_notification_runtime() -> Arc<NotificationRuntime> {
    if should_use_ephemeral_notification_runtime() {
        return build_runtime_from_env()
            .unwrap_or_else(|_| Arc::new(NotificationRuntime::default()));
    }

    DEFAULT_NOTIFICATION_RUNTIME
        .get_or_init(build_notification_runtime_or_fallback)
        .clone()
}

fn should_use_ephemeral_notification_runtime() -> bool {
    matches!(
        resolve_web_environment_from_process_env(),
        WebEnvironment::Dev | WebEnvironment::Test
    )
}

/// Fail-closed durable bootstrap for production process entrypoints.
pub fn ensure_durable_notification_runtime_from_env() -> Result<(), String> {
    build_runtime_from_env().map(|_| ())
}

pub fn build_runtime_from_env() -> Result<Arc<NotificationRuntime>, String> {
    let journal = resolve_notification_commit_journal_from_env()?;
    let store = resolve_notification_task_store_from_env(&journal)?;
    let runtime = match journal.as_ref() {
        NotificationCommitJournal::Postgres(postgres_journal) => {
            NotificationRuntime::with_dyn_task_store_and_aggregate_store(
                journal.clone(),
                store,
                Arc::new(PostgresAggregateStore::from_pool(
                    postgres_journal.pool().clone(),
                )),
            )
        }
        NotificationCommitJournal::Memory(_) => {
            NotificationRuntime::with_dyn_task_store_and_conversation_state(
                journal,
                store,
                Arc::new(ConversationStateService::default()),
            )
        }
    };
    Ok(Arc::new(runtime))
}

/// Spawns the background notification delivery worker for the given runtime.
///
/// The worker claims `requested` tasks (lease-based, `FOR UPDATE SKIP
/// LOCKED`) and writes each task into the recipient's realtime event window
/// (`im_realtime_device_events`). PostgreSQL delivery is used when a
/// PostgreSQL pool is configured; hosts without one (dev in-memory runtimes)
/// fall back to a documented no-op sink so the pipeline stays observable.
pub fn spawn_notification_delivery_worker_from_env(
    runtime: Arc<NotificationRuntime>,
) -> tokio::task::JoinHandle<()> {
    use crate::delivery::{
        NoopNotificationRealtimeDelivery, PostgresNotificationRealtimeDelivery,
        spawn_delivery_worker,
    };

    let task_store = runtime.task_store();
    let realtime: Arc<dyn crate::delivery::NotificationRealtimeDelivery> =
        match resolve_postgres_realtime_pool_from_env() {
            Some(pool) => Arc::new(PostgresNotificationRealtimeDelivery::from_pool(pool)),
            None => Arc::new(NoopNotificationRealtimeDelivery),
        };
    spawn_delivery_worker(task_store, realtime)
}

/// Resolves the process PostgreSQL pool for the realtime delivery sink.
fn resolve_postgres_realtime_pool_from_env()
-> Option<im_adapters_postgres_journal::PostgresJournalPool> {
    if let Ok(config) = DatabaseConfig::from_env("IM")
        && config.engine == DatabaseEngine::Postgres
            && let Ok(journal) = PostgresJournalConfig::from_database_config(&config).connect_pool()
            {
                return Some(journal);
            }
    let database_url = resolve_im_database_url_from_env()?;
    PostgresJournalConfig::new(database_url).connect_pool().ok()
}

pub fn default_app_state() -> AppState {
    AppState {
        runtime: default_notification_runtime(),
    }
}

fn build_notification_runtime_or_fallback() -> Arc<NotificationRuntime> {
    match build_runtime_from_env() {
        Ok(runtime) => runtime,
        Err(error) if allows_header_only_app_context_fallback() => {
            tracing::warn!(
                error = %error,
                "notification-service bootstrap unavailable; using in-memory runtime fallback (development/test only)"
            );
            Arc::new(NotificationRuntime::default())
        }
        Err(error) => {
            panic!(
                "notification-service durable bootstrap failed in production-like environment: {error}"
            );
        }
    }
}

enum NotificationCommitJournal {
    Memory(NoopJournalForDev),
    Postgres(PostgresCommitJournal),
}

impl CommitJournal for NotificationCommitJournal {
    fn append(&self, envelope: CommitEnvelope) -> Result<CommitPosition, ContractError> {
        match self {
            Self::Memory(journal) => journal.append(envelope),
            Self::Postgres(journal) => journal.append(envelope),
        }
    }
}

fn resolve_notification_task_store_from_env(
    journal: &Arc<NotificationCommitJournal>,
) -> Result<Arc<dyn NotificationTaskStore>, String> {
    if let Some(path) = resolve_notification_task_store_path_from_env() {
        let environment = resolve_web_environment_from_process_env();
        if !matches!(environment, WebEnvironment::Dev | WebEnvironment::Test) {
            return Err(format!(
                "file-backed notification task store is single-node development storage and is forbidden in production; configure PostgreSQL with {IM_DATABASE_URL_ENV}"
            ));
        }
        info!(
            path = %path,
            "notification-service using file-backed notification task store"
        );
        return Ok(Arc::new(FileNotificationTaskStore::new(path)));
    }

    if let NotificationCommitJournal::Postgres(pg_journal) = journal.as_ref() {
        info!("notification-service using postgres notification task store");
        return Ok(Arc::new(PostgresNotificationTaskStore::from_pool(
            pg_journal.pool().clone(),
        )));
    }

    let environment = resolve_web_environment_from_process_env();
    if matches!(environment, WebEnvironment::Dev | WebEnvironment::Test) {
        info!("notification-service using in-memory notification task store (development only)");
        return Ok(Arc::new(MemoryNotificationTaskStore::default()));
    }

    Err(format!(
        "PostgreSQL notification task store is required in production: set {IM_DATABASE_URL_ENV}"
    ))
}

fn resolve_notification_commit_journal_from_env() -> Result<Arc<NotificationCommitJournal>, String>
{
    if let Ok(config) = DatabaseConfig::from_env("IM")
        && config.engine == DatabaseEngine::Postgres {
            let journal = PostgresJournalConfig::from_database_config(&config)
                .connect()
                .map_err(|error| {
                    format!("postgres notification journal bootstrap failed: {error:?}")
                })?;
            info!("notification-service using postgres commit journal");
            return Ok(Arc::new(NotificationCommitJournal::Postgres(journal)));
        }

    if let Some(database_url) = resolve_im_database_url_from_env() {
        let journal = PostgresJournalConfig::new(database_url)
            .connect()
            .map_err(|error| {
                format!("postgres notification journal bootstrap failed: {error:?}")
            })?;
        info!("notification-service using postgres commit journal");
        return Ok(Arc::new(NotificationCommitJournal::Postgres(journal)));
    }

    let environment = resolve_web_environment_from_process_env();
    if matches!(environment, WebEnvironment::Dev | WebEnvironment::Test) {
        info!("notification-service using in-memory commit journal (development only)");
        return Ok(Arc::new(NotificationCommitJournal::Memory(
            NoopJournalForDev,
        )));
    }

    Err(format!(
        "postgres notification commit journal is required in production: set {IM_DATABASE_URL_ENV}"
    ))
}

fn resolve_notification_task_store_path_from_env() -> Option<String> {
    std::env::var(NOTIFICATION_TASK_STORE_FILE_ENV)
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
    fn production_requires_durable_notification_backends() {
        let _guard = ENV_LOCK
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        let database_url = std::env::var(IM_DATABASE_URL_ENV).ok();
        let task_store_file = std::env::var(NOTIFICATION_TASK_STORE_FILE_ENV).ok();
        let im_env = std::env::var("SDKWORK_IM_ENVIRONMENT").ok();
        unsafe {
            std::env::remove_var(IM_DATABASE_URL_ENV);
            std::env::remove_var(NOTIFICATION_TASK_STORE_FILE_ENV);
            std::env::set_var("SDKWORK_IM_ENVIRONMENT", "prod");
        }

        assert!(resolve_notification_commit_journal_from_env().is_err());
        assert!(build_runtime_from_env().is_err());

        unsafe {
            if let Some(value) = database_url {
                std::env::set_var(IM_DATABASE_URL_ENV, value);
            } else {
                std::env::remove_var(IM_DATABASE_URL_ENV);
            }
            if let Some(value) = task_store_file {
                std::env::set_var(NOTIFICATION_TASK_STORE_FILE_ENV, value);
            } else {
                std::env::remove_var(NOTIFICATION_TASK_STORE_FILE_ENV);
            }
            if let Some(value) = im_env {
                std::env::set_var("SDKWORK_IM_ENVIRONMENT", value);
            } else {
                std::env::remove_var("SDKWORK_IM_ENVIRONMENT");
            }
        }
    }

    #[test]
    fn production_rejects_file_backed_notification_store() {
        let _guard = ENV_LOCK
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        let task_store_file = std::env::var(NOTIFICATION_TASK_STORE_FILE_ENV).ok();
        let im_env = std::env::var("SDKWORK_IM_ENVIRONMENT").ok();
        unsafe {
            std::env::set_var(NOTIFICATION_TASK_STORE_FILE_ENV, "notification-test.json");
            std::env::set_var("SDKWORK_IM_ENVIRONMENT", "prod");
        }
        let journal = Arc::new(NotificationCommitJournal::Memory(NoopJournalForDev));
        let result = resolve_notification_task_store_from_env(&journal);
        let error = result
            .err()
            .expect("production file store must fail closed");
        assert!(error.contains("forbidden in production"));

        unsafe {
            if let Some(value) = task_store_file {
                std::env::set_var(NOTIFICATION_TASK_STORE_FILE_ENV, value);
            } else {
                std::env::remove_var(NOTIFICATION_TASK_STORE_FILE_ENV);
            }
            if let Some(value) = im_env {
                std::env::set_var("SDKWORK_IM_ENVIRONMENT", value);
            } else {
                std::env::remove_var("SDKWORK_IM_ENVIRONMENT");
            }
        }
    }
}
