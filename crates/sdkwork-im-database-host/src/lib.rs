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

    apply_im_initialization_schema_repairs(&pool).await?;

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

const POSTGRES_INITIALIZATION_SCHEMA_REPAIR_SQL: &str = r#"
DO $$
DECLARE
    existing_pk_name text;
    existing_pk_columns text[];
BEGIN
    IF to_regclass('im_conversation_read_cursors') IS NULL THEN
        RETURN;
    END IF;

    ALTER TABLE im_conversation_read_cursors
        ADD COLUMN IF NOT EXISTS device_id TEXT;

    UPDATE im_conversation_read_cursors
       SET device_id = ''
     WHERE device_id IS NULL;

    ALTER TABLE im_conversation_read_cursors
        ALTER COLUMN device_id SET DEFAULT '',
        ALTER COLUMN device_id SET NOT NULL;

    SELECT c.conname,
           array_agg(a.attname ORDER BY keys.ordinality)
      INTO existing_pk_name, existing_pk_columns
      FROM pg_constraint c
      JOIN LATERAL unnest(c.conkey) WITH ORDINALITY AS keys(attnum, ordinality) ON true
      JOIN pg_attribute a
        ON a.attrelid = c.conrelid
       AND a.attnum = keys.attnum
     WHERE c.conrelid = 'im_conversation_read_cursors'::regclass
       AND c.contype = 'p'
     GROUP BY c.conname;

    IF existing_pk_columns IS DISTINCT FROM ARRAY[
        'tenant_id',
        'organization_id',
        'conversation_id',
        'member_id',
        'device_id'
    ] THEN
        IF existing_pk_name IS NOT NULL THEN
            EXECUTE format(
                'ALTER TABLE im_conversation_read_cursors DROP CONSTRAINT %I',
                existing_pk_name
            );
        END IF;

        ALTER TABLE im_conversation_read_cursors
            ADD CONSTRAINT pk_im_conversation_read_cursors
            PRIMARY KEY (tenant_id, organization_id, conversation_id, member_id, device_id);
    END IF;
END $$;
"#;

async fn apply_im_initialization_schema_repairs(pool: &DatabasePool) -> Result<(), String> {
    match pool {
        DatabasePool::Postgres(_, _) => pool
            .execute_raw(POSTGRES_INITIALIZATION_SCHEMA_REPAIR_SQL)
            .await
            .map(|_| ())
            .map_err(|error| format!("IM database initialization schema repair failed: {error}")),
        DatabasePool::Sqlite(_, _) => Ok(()),
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
    #[test]
    fn postgres_initialization_schema_repair_adds_read_cursor_device_scope() {
        let source = include_str!("lib.rs");
        let production_source = source
            .split("#[cfg(test)]")
            .next()
            .expect("source should contain production section");
        let sql = production_source.to_ascii_lowercase();

        for required in [
            "postgres_initialization_schema_repair_sql",
            "to_regclass('im_conversation_read_cursors')",
            "add column if not exists device_id text",
            "alter column device_id set default ''",
            "alter column device_id set not null",
            "drop constraint",
            "primary key (tenant_id, organization_id, conversation_id, member_id, device_id)",
        ] {
            assert!(
                sql.contains(required),
                "Postgres initialization repair SQL must include `{required}`"
            );
        }
    }

    #[test]
    fn bootstrap_runs_initialization_schema_repairs_after_lifecycle_init() {
        let source = include_str!("lib.rs");
        let init = source
            .find(".init()")
            .expect("bootstrap should call lifecycle init");
        let repair = source
            .find("apply_im_initialization_schema_repairs(&pool)")
            .expect("bootstrap should run initialization schema repairs");
        let migrate = source
            .find("if options.auto_migrate")
            .expect("bootstrap should preserve auto_migrate branch");

        assert!(
            init < repair && repair < migrate,
            "schema repairs must run after baseline init and before migrations/traffic dependencies"
        );
    }
}
