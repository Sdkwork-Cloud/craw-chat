use std::sync::Arc;

use im_adapters_local_memory::{MemoryMetadataStore, MemoryTimelineProjectionStore};
use im_adapters_postgres_projection::{PostgresProjectionConfig, PostgresProjectionStores};
use im_app_context::resolve_web_environment_from_process_env;
use im_platform_contracts::{MetadataStore, TimelineProjectionStore};
use sdkwork_web_core::WebEnvironment;
use tracing::info;

use crate::snapshot::CONVERSATION_SUMMARY_SNAPSHOT_KEY;
use crate::{ProjectionError, TimelineProjectionService};

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
}

impl ProjectionRuntime {
    pub fn in_memory() -> Self {
        Self {
            service: Arc::new(TimelineProjectionService::default()),
            backend: ProjectionPersistenceBackend::Memory {
                metadata: MemoryMetadataStore::default(),
                timeline: MemoryTimelineProjectionStore::default(),
            },
        }
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
}

pub fn resolve_projection_persistence_from_env() -> Result<ProjectionPersistenceBackend, String> {
    if let Some(database_url) = resolve_im_database_url_from_env() {
        let stores = PostgresProjectionConfig::new(database_url)
            .connect_stores()
            .map_err(|error| format!("postgres projection store bootstrap failed: {error:?}"))?;
        info!("projection-service using postgres durable projection stores");
        return Ok(ProjectionPersistenceBackend::Postgres(stores));
    }

    let environment = resolve_web_environment_from_process_env();
    if matches!(environment, WebEnvironment::Dev | WebEnvironment::Test) {
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

pub fn build_projection_runtime_from_env() -> Result<ProjectionRuntime, String> {
    let backend = resolve_projection_persistence_from_env()?;
    let service = Arc::new(TimelineProjectionService::default());
    let conversation_scopes = backend.list_conversation_snapshot_scopes().map_err(|error| {
        format!("projection durable restore scope discovery failed: {error:?}")
    })?;
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
    Ok(ProjectionRuntime { service, backend })
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
        let web_env = std::env::var("SDKWORK_WEB_ENVIRONMENT").ok();
        unsafe {
            std::env::remove_var(IM_DATABASE_URL_ENV);
            std::env::set_var("SDKWORK_WEB_ENVIRONMENT", "prod");
        }

        let result = resolve_projection_persistence_from_env();
        assert!(result.is_err());

        unsafe {
            if let Some(value) = database_url {
                std::env::set_var(IM_DATABASE_URL_ENV, value);
            } else {
                std::env::remove_var(IM_DATABASE_URL_ENV);
            }
            if let Some(value) = web_env {
                std::env::set_var("SDKWORK_WEB_ENVIRONMENT", value);
            } else {
                std::env::remove_var("SDKWORK_WEB_ENVIRONMENT");
            }
        }
    }
}
