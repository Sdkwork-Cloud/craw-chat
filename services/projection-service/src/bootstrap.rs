use std::sync::{Arc, OnceLock};

use im_adapters_postgres_journal::{PostgresJournalConfig, PostgresSearchProvider};
use im_adapters_local_memory::{MemoryMetadataStore, MemoryTimelineProjectionStore};
use im_adapters_postgres_projection::{PostgresProjectionConfig, PostgresProjectionStores};
use im_app_context::resolve_web_environment_from_process_env;
use im_platform_contracts::{MetadataStore, TimelineProjectionStore};
use sdkwork_database_config::{DatabaseConfig, DatabaseEngine};
use sdkwork_web_core::WebEnvironment;
use tracing::info;

use crate::snapshot::CONVERSATION_SUMMARY_SNAPSHOT_KEY;
use crate::{
    ProjectionError, TimelineProjectionService, PROJECTION_TIMELINE_MEMORY_CAP_UNLIMITED,
};

const IM_DATABASE_URL_ENV: &str = "SDKWORK_IM_DATABASE_URL";

pub enum ProjectionPersistenceBackend {
    Memory {
        metadata: MemoryMetadataStore,
        timeline: MemoryTimelineProjectionStore,
    },
    Postgres(PostgresProjectionStores),
}

impl ProjectionPersistenceBackend {
    pub fn metadata(&self) -> &dyn MetadataStore {
        match self {
            Self::Memory { metadata, .. } => metadata,
            Self::Postgres(stores) => &stores.metadata,
        }
    }

    pub fn timeline(&self) -> &dyn TimelineProjectionStore {
        match self {
            Self::Memory { timeline, .. } => timeline,
            Self::Postgres(stores) => &stores.timeline,
        }
    }

    pub fn list_conversation_snapshot_scopes(&self) -> Result<Vec<String>, ProjectionError> {
        let scopes = match self {
            Self::Memory { metadata, .. } => {
                metadata.list_scopes_for_snapshot_key(CONVERSATION_SUMMARY_SNAPSHOT_KEY)
            }
            Self::Postgres(stores) => stores
                .metadata
                .list_scopes_for_snapshot_key(CONVERSATION_SUMMARY_SNAPSHOT_KEY)
                .map_err(ProjectionError::StoreFailure)?,
        };
        Ok(scopes)
    }

    pub fn is_postgres(&self) -> bool {
        matches!(self, Self::Postgres(_))
    }
}

pub struct ProjectionRuntime {
    pub service: Arc<TimelineProjectionService>,
    backend: ProjectionPersistenceBackend,
    search_provider: Option<Arc<PostgresSearchProvider>>,
}

impl ProjectionRuntime {
    pub fn in_memory() -> Self {
        let service = Arc::new(TimelineProjectionService::default());
        let memory_cap = crate::resolve_memory_timeline_cap_from_env(false);
        if memory_cap < PROJECTION_TIMELINE_MEMORY_CAP_UNLIMITED {
            service.set_memory_timeline_cap(memory_cap);
        }
        Self {
            service,
            backend: ProjectionPersistenceBackend::Memory {
                metadata: MemoryMetadataStore::default(),
                timeline: MemoryTimelineProjectionStore::default(),
            },
            search_provider: None,
        }
    }

    pub fn search_provider(&self) -> Option<Arc<PostgresSearchProvider>> {
        self.search_provider.clone()
    }

    pub fn service(&self) -> Arc<TimelineProjectionService> {
        self.service.clone()
    }

    pub fn persist_durable_state(&self) -> Result<(), ProjectionError> {
        if !self.backend.is_postgres() {
            return Ok(());
        }
        self.service
            .persist_all_durable_snapshots(self.backend.metadata(), self.backend.timeline())
    }

    /// Returns true when visibility and other durable snapshots must persist successfully.
    pub fn requires_durable_persist(&self) -> bool {
        self.backend.is_postgres() && !allows_in_memory_projection_fallback()
    }
}

fn is_dev_or_test_environment() -> bool {
    matches!(
        resolve_web_environment_from_process_env(),
        WebEnvironment::Dev | WebEnvironment::Test
    )
}

fn running_under_rust_test_harness() -> bool {
    std::env::var("RUST_TEST_THREADS").is_ok()
}

/// Whether `SDKWORK_IM_ENVIRONMENT` indicates production-like deployment (prod/staging/default).
pub use im_app_context::is_production_like_im_environment;

/// Whether projection may fall back to in-memory stores when Postgres bootstrap fails.
///
/// Production processes MUST bootstrap IM database pools before opening adapters;
/// unit tests and local dev may run without pools when IM DSN env vars are present.
pub fn allows_in_memory_projection_fallback() -> bool {
    running_under_rust_test_harness() || is_dev_or_test_environment()
}

fn try_postgres_projection_stores_from_config(
    config: &DatabaseConfig,
) -> Result<PostgresProjectionStores, String> {
    PostgresProjectionConfig::from_database_config(config)
        .connect_stores()
        .map_err(|error| format!("postgres projection store bootstrap failed: {error:?}"))
}

fn try_postgres_projection_stores_from_url(
    database_url: &str,
) -> Result<PostgresProjectionStores, String> {
    PostgresProjectionConfig::new(database_url.to_owned())
        .connect_stores()
        .map_err(|error| format!("postgres projection store bootstrap failed: {error:?}"))
}

pub fn resolve_projection_persistence_from_env() -> Result<ProjectionPersistenceBackend, String> {
    if let Ok(config) = DatabaseConfig::from_env("IM") {
        if config.engine == DatabaseEngine::Postgres {
            match try_postgres_projection_stores_from_config(&config) {
                Ok(stores) => {
                    info!("projection-service using postgres durable projection stores");
                    return Ok(ProjectionPersistenceBackend::Postgres(stores));
                }
                Err(error) if allows_in_memory_projection_fallback() => {
                    tracing::warn!(
                        "{error}; falling back to in-memory projection stores for local/test"
                    );
                    return Ok(ProjectionPersistenceBackend::Memory {
                        metadata: MemoryMetadataStore::default(),
                        timeline: MemoryTimelineProjectionStore::default(),
                    });
                }
                Err(error) => return Err(error),
            }
        } else if is_dev_or_test_environment() {
            sdkwork_im_database_pool::log_im_core_ephemeral_non_postgres_authority(
                "projection-service",
                config.engine,
            );
            return Ok(ProjectionPersistenceBackend::Memory {
                metadata: MemoryMetadataStore::default(),
                timeline: MemoryTimelineProjectionStore::default(),
            });
        } else {
            return Err(
                "postgres projection stores are required in production when IM database engine is not postgres"
                    .into(),
            );
        }
    }

    if let Some(database_url) = resolve_im_database_url_from_env() {
        match try_postgres_projection_stores_from_url(database_url.as_str()) {
            Ok(stores) => {
                info!("projection-service using postgres durable projection stores");
                return Ok(ProjectionPersistenceBackend::Postgres(stores));
            }
            Err(error) if allows_in_memory_projection_fallback() => {
                tracing::warn!(
                    "{error}; falling back to in-memory projection stores for local/test"
                );
                return Ok(ProjectionPersistenceBackend::Memory {
                    metadata: MemoryMetadataStore::default(),
                    timeline: MemoryTimelineProjectionStore::default(),
                });
            }
            Err(error) => return Err(error),
        }
    }

    if allows_in_memory_projection_fallback() {
        info!("projection-service using in-memory projection stores (development only)");
        return Ok(ProjectionPersistenceBackend::Memory {
            metadata: MemoryMetadataStore::default(),
            timeline: MemoryTimelineProjectionStore::default(),
        });
    }

    Err(format!(
        "postgres projection stores are required in production: set {IM_DATABASE_URL_ENV}"
    ))
}

static SHARED_PROJECTION_RUNTIME: OnceLock<Arc<ProjectionRuntime>> = OnceLock::new();

/// Single projection runtime shared by HTTP handlers and embedded journal apply paths.
pub fn shared_projection_runtime() -> Arc<ProjectionRuntime> {
    SHARED_PROJECTION_RUNTIME
        .get_or_init(init_shared_projection_runtime)
        .clone()
}

fn init_shared_projection_runtime() -> Arc<ProjectionRuntime> {
    Arc::new(
        build_projection_runtime_from_env().unwrap_or_else(|error| {
            if allows_in_memory_projection_fallback() {
                tracing::warn!(
                    error = %error,
                    "projection-service bootstrap failed; falling back to in-memory projection runtime for local development"
                );
                ProjectionRuntime::in_memory()
            } else {
                tracing::error!(
                    error = %error,
                    "projection-service fail-closed: durable projection runtime is required in production"
                );
                panic!(
                    "projection-service fail-closed: durable projection runtime bootstrap failed in production: {error}"
                );
            }
        }),
    )
}

/// Best-effort embedded projection runtime for unified-process journal append paths.
///
/// Returns `None` when durable bootstrap is unavailable and in-memory fallback is not allowed.
/// HTTP handlers continue to use [`shared_projection_runtime`], which fail-closes in production.
pub fn try_init_embedded_projection_runtime() -> Option<Arc<ProjectionRuntime>> {
    if let Some(runtime) = SHARED_PROJECTION_RUNTIME.get() {
        return Some(runtime.clone());
    }
    let runtime = match build_projection_runtime_from_env() {
        Ok(runtime) => Arc::new(runtime),
        Err(error) if allows_in_memory_projection_fallback() => {
            tracing::warn!(
                error = %error,
                "embedded projection bootstrap failed; using in-memory projection runtime"
            );
            Arc::new(ProjectionRuntime::in_memory())
        }
        Err(error) => {
            tracing::debug!(
                error = %error,
                "embedded projection bootstrap skipped because durable stores are unavailable"
            );
            return None;
        }
    };
    match SHARED_PROJECTION_RUNTIME.set(runtime.clone()) {
        Ok(()) => Some(runtime),
        Err(_) => SHARED_PROJECTION_RUNTIME.get().cloned(),
    }
}

pub fn build_projection_runtime_from_env() -> Result<ProjectionRuntime, String> {
    let backend = resolve_projection_persistence_from_env()?;
    let service = Arc::new(TimelineProjectionService::default());
    if let ProjectionPersistenceBackend::Postgres(stores) = &backend {
        let memory_cap = crate::resolve_memory_timeline_cap_from_env(true);
        let durable_store: Arc<dyn TimelineProjectionStore + Send + Sync> =
            Arc::new(stores.timeline.clone());
        service.configure_durable_timeline(durable_store, memory_cap);
        info!(
            memory_timeline_cap = memory_cap,
            "projection-service configured tiered timeline (postgres durable + in-memory hot cache)"
        );
    }
    let conversation_scopes = backend
        .list_conversation_snapshot_scopes()
        .map_err(|error| format!("projection durable restore scope discovery failed: {error:?}"))?;
    service
        .restore_all_durable_snapshots(
            backend.metadata(),
            backend.timeline(),
            conversation_scopes.as_slice(),
        )
        .map_err(|error| format!("projection durable restore failed: {error:?}"))?;
    if backend.is_postgres() && !conversation_scopes.is_empty() {
        info!(
            restored_conversation_snapshots = conversation_scopes.len(),
            "projection-service restored durable conversation snapshots from postgres"
        );
    }
    Ok(ProjectionRuntime {
        service,
        backend,
        search_provider: resolve_postgres_search_provider_from_env(),
    })
}

fn resolve_postgres_search_provider_from_env() -> Option<Arc<PostgresSearchProvider>> {
    let database_url = resolve_im_database_url_from_env()?;
    let pool = PostgresJournalConfig::new(database_url)
        .connect_pool()
        .ok()?;
    Some(Arc::new(PostgresSearchProvider::from_pool(pool)))
}

fn resolve_im_database_url_from_env() -> Option<String> {
    std::env::var(IM_DATABASE_URL_ENV)
        .ok()
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn production_requires_database_url_for_projection_stores() {
        let database_url = std::env::var(IM_DATABASE_URL_ENV).ok();
        let im_env = std::env::var("SDKWORK_IM_ENVIRONMENT").ok();
        unsafe {
            std::env::remove_var(IM_DATABASE_URL_ENV);
            std::env::set_var("SDKWORK_IM_ENVIRONMENT", "prod");
        }

        let result = resolve_projection_persistence_from_env();
        if running_under_rust_test_harness() {
            assert!(
                result.is_ok(),
                "rust test harness may fall back to in-memory projection when pools are not bootstrapped"
            );
        } else {
            assert!(result.is_err());
        }

        unsafe {
            if let Some(value) = database_url {
                std::env::set_var(IM_DATABASE_URL_ENV, value);
            } else {
                std::env::remove_var(IM_DATABASE_URL_ENV);
            }
            if let Some(value) = im_env {
                std::env::set_var("SDKWORK_IM_ENVIRONMENT", value);
            } else {
                std::env::remove_var("SDKWORK_IM_ENVIRONMENT");
            }
        }
    }

    #[test]
    fn production_env_disallows_in_memory_projection_fallback_outside_tests() {
        struct ScopedImEnvironment {
            previous: Option<String>,
        }

        impl ScopedImEnvironment {
            fn set(value: &str) -> Self {
                let previous = std::env::var("SDKWORK_IM_ENVIRONMENT").ok();
                unsafe {
                    std::env::set_var("SDKWORK_IM_ENVIRONMENT", value);
                }
                Self { previous }
            }
        }

        impl Drop for ScopedImEnvironment {
            fn drop(&mut self) {
                unsafe {
                    if let Some(value) = self.previous.as_ref() {
                        std::env::set_var("SDKWORK_IM_ENVIRONMENT", value);
                    } else {
                        std::env::remove_var("SDKWORK_IM_ENVIRONMENT");
                    }
                }
            }
        }

        let _guard = ScopedImEnvironment::set("prod");
        assert!(!is_dev_or_test_environment());
    }
}
