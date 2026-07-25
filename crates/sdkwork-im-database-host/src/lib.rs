use std::path::PathBuf;
use std::sync::Arc;

use sdkwork_database_config::DatabaseConfig;
use sdkwork_database_lifecycle::{LifecycleOrchestrator, lifecycle_options_from_env};
use sdkwork_database_spi::{DatabaseAssetProvider, DatabaseManifest, DefaultDatabaseModule};
use sdkwork_database_sqlx::{DatabasePool, create_pool_from_config};

pub struct ImDatabaseHost {
    pool: DatabasePool,
    module: Arc<DefaultDatabaseModule>,
}

impl ImDatabaseHost {
    pub fn pool(&self) -> &DatabasePool {
        &self.pool
    }

    pub fn module(&self) -> Arc<DefaultDatabaseModule> {
        self.module.clone()
    }
}

pub async fn bootstrap_im_database(pool: DatabasePool) -> Result<ImDatabaseHost, String> {
    ensure_im_postgres_authority(&pool)?;
    let app_root = resolve_app_root();
    let module = Arc::new(
        DefaultDatabaseModule::from_app_root(&app_root)
            .map_err(|error| format!("load IM database module failed: {error}"))?,
    );
    let manifest = DatabaseManifest::from_file(module.manifest_path())
        .map_err(|error| format!("read IM database manifest failed: {error}"))?;
    let options = lifecycle_options_from_env("IM", &manifest);
    let orchestrator =
        LifecycleOrchestrator::new(pool.clone(), module.clone()).with_applied_by("sdkwork-im");

    orchestrator
        .init()
        .await
        .map_err(|error| format!("IM database init failed: {error}"))?;

    if options.auto_migrate {
        orchestrator
            .migrate()
            .await
            .map_err(|error| format!("IM database migrate failed: {error}"))?;
    }

    Ok(ImDatabaseHost { pool, module })
}

pub async fn bootstrap_im_database_from_env() -> Result<ImDatabaseHost, String> {
    let _ = dotenvy::dotenv();
    let config = DatabaseConfig::from_env("IM")
        .map_err(|error| format!("read IM database config failed: {error}"))?;
    let pool = create_pool_from_config(config)
        .await
        .map_err(|error| format!("create IM database pool failed: {error}"))?;
    bootstrap_im_database(pool).await
}

fn ensure_im_postgres_authority(pool: &DatabasePool) -> Result<(), String> {
    match pool {
        DatabasePool::Postgres(_, _) => Ok(()),
        DatabasePool::Sqlite(_, _) => Err(
            "IM authoritative server persistence requires PostgreSQL; SQLite is client-local only"
                .to_owned(),
        ),
    }
}

fn resolve_app_root() -> PathBuf {
    std::env::var("SDKWORK_IM_APP_ROOT")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("../..")
                .canonicalize()
                .unwrap_or_else(|_| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../.."))
        })
}

#[cfg(test)]
mod tests {
    use sdkwork_database_config::{DatabaseConfig, DatabaseEngine};
    use sdkwork_database_sqlx::create_pool_from_config;

    use super::*;

    #[tokio::test]
    async fn bootstrap_rejects_sqlite_before_database_lifecycle() {
        let pool = create_pool_from_config(DatabaseConfig {
            engine: DatabaseEngine::Sqlite,
            url: "sqlite::memory:".to_owned(),
            max_connections: 1,
            ..DatabaseConfig::default()
        })
        .await
        .expect("create isolated client-local fixture");

        let error = match bootstrap_im_database(pool.clone()).await {
            Ok(_) => panic!("server bootstrap must reject SQLite"),
            Err(error) => error,
        };
        assert!(error.contains("requires PostgreSQL"));
        assert!(error.contains("client-local only"));
        pool.close().await;
    }
}
