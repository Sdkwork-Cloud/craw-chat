use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use sdkwork_database_config::{DatabaseConfig, DatabaseEngine};
use sdkwork_database_lifecycle::LifecycleOrchestrator;
use sdkwork_database_spi::{DefaultDatabaseModule, MigrationProvider};
use sdkwork_database_sqlx::{DatabasePool, create_pool_from_config};
use tempfile::TempDir;

const MODULE_MANIFEST: &str = r#"{
  "schemaVersion": 1,
  "kind": "sdkwork.database.module",
  "moduleId": "im",
  "serviceCode": "IM",
  "engines": ["postgres", "sqlite"],
  "defaultEngine": "postgres",
  "tablePrefix": "im_",
  "contractVersion": "1.0.0",
  "baselineStrategy": "baseline-plus-migrations",
  "baselineAnchorTable": "im_commit_journal",
  "paths": {
    "contract": "contract/schema.yaml",
    "migrations": "migrations",
    "seeds": "seeds",
    "driftPolicy": "drift/policy.yaml"
  },
  "lifecycle": { "activeSeedLocales": ["zh-CN"] }
}"#;

const LEGACY_GROUP_KNOWLEDGEBASE_SCHEMA: &str = r#"
CREATE TABLE im_conversation_knowledge_space_link (
    id INTEGER NOT NULL,
    link_uuid TEXT NOT NULL,
    tenant_id TEXT NOT NULL,
    organization_id TEXT NOT NULL,
    conversation_id TEXT NOT NULL,
    knowledge_space_id INTEGER,
    knowledge_space_uuid TEXT,
    knowledgebase_binding_id INTEGER,
    lifecycle_state TEXT NOT NULL,
    provisioning_operation_id TEXT,
    creation_idempotency_key TEXT NOT NULL,
    last_source_event_id TEXT,
    membership_epoch INTEGER NOT NULL,
    last_synchronized_membership_epoch INTEGER NOT NULL,
    last_error_code TEXT,
    last_error_at TEXT,
    created_by TEXT NOT NULL,
    updated_by TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    archived_at TEXT,
    deleted_at TEXT,
    version INTEGER NOT NULL,
    PRIMARY KEY (tenant_id, organization_id, conversation_id),
    UNIQUE (id),
    UNIQUE (link_uuid)
);

CREATE TABLE im_group_knowledge_launch_tickets (
    id INTEGER NOT NULL PRIMARY KEY,
    ticket_hash TEXT NOT NULL UNIQUE,
    tenant_id TEXT NOT NULL,
    organization_id TEXT NOT NULL,
    conversation_id TEXT NOT NULL,
    knowledge_space_id INTEGER NOT NULL,
    knowledge_space_uuid TEXT NOT NULL,
    binding_version INTEGER NOT NULL,
    membership_epoch INTEGER NOT NULL,
    actor_kind TEXT NOT NULL,
    actor_id TEXT NOT NULL,
    principal_kind TEXT NOT NULL,
    principal_id TEXT NOT NULL,
    session_id TEXT NOT NULL,
    issuing_app_id TEXT,
    issued_by TEXT NOT NULL,
    idempotency_key_hash TEXT NOT NULL,
    request_fingerprint_hash TEXT NOT NULL,
    ticket_ciphertext TEXT NOT NULL,
    expires_at TEXT NOT NULL,
    consumed_at TEXT,
    consumed_by_service TEXT,
    consumed_trace_id TEXT,
    created_at TEXT NOT NULL
);
"#;

fn repository_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("database host must remain inside the IM repository")
}

fn write_file(path: &Path, content: &str) {
    fs::create_dir_all(path.parent().expect("file parent")).expect("create test directory");
    fs::write(path, content).expect("write test file");
}

fn copy_repository_file(relative_path: &str, destination: &Path) {
    fs::create_dir_all(destination.parent().expect("file parent")).expect("create test directory");
    fs::copy(repository_root().join(relative_path), destination)
        .expect("copy repository lifecycle asset");
}

fn create_test_module(
    include_folded_sqlite_baseline: bool,
    include_conversation_id_migration: bool,
) -> TempDir {
    let temp = TempDir::new().expect("create temporary module root");
    let database_root = temp.path().join("database");
    write_file(
        &database_root.join("database.manifest.json"),
        MODULE_MANIFEST,
    );

    if include_folded_sqlite_baseline {
        copy_repository_file(
            "database/ddl/baseline/sqlite/0001_im_baseline.sql",
            &database_root.join("ddl/baseline/sqlite/0001_im_baseline.sql"),
        );
    }
    if include_conversation_id_migration {
        copy_repository_file(
            "database/migrations/sqlite/0002_rewrite_legacy_conversation_id_prefixes.up.sql",
            &database_root
                .join("migrations/sqlite/0002_rewrite_legacy_conversation_id_prefixes.up.sql"),
        );
    }
    copy_repository_file(
        "database/migrations/sqlite/0003_group_knowledgebase_binding.up.sql",
        &database_root.join("migrations/sqlite/0003_group_knowledgebase_binding.up.sql"),
    );

    temp
}

async fn sqlite_pool() -> DatabasePool {
    create_pool_from_config(DatabaseConfig {
        engine: DatabaseEngine::Sqlite,
        url: "sqlite::memory:".to_string(),
        max_connections: 1,
        ..Default::default()
    })
    .await
    .expect("create isolated SQLite pool")
}

fn sqlite(pool: &DatabasePool) -> &sqlx::SqlitePool {
    pool.as_sqlite().expect("SQLite pool")
}

async fn table_exists(pool: &sqlx::SqlitePool, table_name: &str) -> bool {
    sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM sqlite_master WHERE type = 'table' AND name = ?",
    )
    .bind(table_name)
    .fetch_one(pool)
    .await
    .expect("inspect SQLite table")
        > 0
}

async fn column_exists(pool: &sqlx::SqlitePool, table_name: &str, column_name: &str) -> bool {
    sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM pragma_table_info(?) WHERE name = ?")
        .bind(table_name)
        .bind(column_name)
        .fetch_one(pool)
        .await
        .expect("inspect SQLite table columns")
        > 0
}

async fn migration_history_count(pool: &sqlx::SqlitePool, version: &str) -> i64 {
    sqlx::query_scalar(
        "SELECT COUNT(*) FROM ops_schema_migration_history \
         WHERE module_id = 'im' AND version = ? AND engine = 'sqlite'",
    )
    .bind(version)
    .fetch_one(pool)
    .await
    .expect("read migration history")
}

async fn prepare_anchored_legacy_database(pool: &sqlx::SqlitePool) {
    sqlx::query("CREATE TABLE im_commit_journal (id INTEGER PRIMARY KEY)")
        .execute(pool)
        .await
        .expect("create baseline anchor");
    sqlx::raw_sql(LEGACY_GROUP_KNOWLEDGEBASE_SCHEMA)
        .execute(pool)
        .await
        .expect("create legacy group knowledgebase schema");
}

async fn create_orchestrator(root: &Path, pool: DatabasePool) -> LifecycleOrchestrator {
    let module = Arc::new(
        DefaultDatabaseModule::from_app_root(root).expect("load temporary database module"),
    );
    LifecycleOrchestrator::new(pool, module).with_applied_by("group-knowledgebase-lifecycle-test")
}

#[tokio::test]
async fn group_knowledgebase_migrations_use_lifecycle_discovery_names_for_both_engines() {
    let module =
        DefaultDatabaseModule::from_app_root(repository_root()).expect("load IM database module");

    for engine in [DatabaseEngine::Postgres, DatabaseEngine::Sqlite] {
        let migrations = module
            .list_migrations(engine)
            .await
            .expect("discover group knowledgebase migrations");
        let versions = migrations
            .iter()
            .map(|migration| migration.version.clone())
            .collect::<Vec<_>>();

        assert_eq!(versions, vec!["0002".to_string(), "0003".to_string()]);
        assert!(migrations.iter().all(|migration| {
            migration
                .up_path
                .file_name()
                .and_then(|name| name.to_str())
                .is_some_and(|name| name.ends_with(".up.sql"))
        }));
    }
}

#[tokio::test]
async fn empty_database_applies_folded_baseline_then_group_knowledgebase_migrations() {
    let temp = create_test_module(true, true);
    let pool = sqlite_pool().await;
    let sqlite_pool = sqlite(&pool).clone();
    let orchestrator = create_orchestrator(temp.path(), pool.clone()).await;

    let applied = orchestrator
        .migrate()
        .await
        .expect("empty database must migrate from folded baseline");
    assert_eq!(applied, 3, "one baseline and two migrations must apply");
    assert_eq!(migration_history_count(&sqlite_pool, "0003").await, 1);
    assert!(
        column_exists(
            &sqlite_pool,
            "im_conversation_knowledge_space_link",
            "knowledgebase_binding_uuid",
        )
        .await
    );
    assert!(
        !column_exists(
            &sqlite_pool,
            "im_group_knowledge_launch_tickets",
            "binding_version",
        )
        .await
    );

    let repeated = orchestrator
        .migrate()
        .await
        .expect("repeat migration must be idempotent");
    assert_eq!(repeated, 0);
    pool.close().await;
}

#[tokio::test]
async fn anchored_legacy_database_skips_baseline_and_records_group_knowledgebase_upgrade() {
    let temp = create_test_module(false, false);
    write_file(
        &temp
            .path()
            .join("database/ddl/baseline/sqlite/0001_must_not_run.sql"),
        "CREATE TABLE baseline_must_not_run (id INTEGER PRIMARY KEY);",
    );
    let pool = sqlite_pool().await;
    let sqlite_pool = sqlite(&pool).clone();
    prepare_anchored_legacy_database(&sqlite_pool).await;
    sqlx::query(
        "INSERT INTO im_conversation_knowledge_space_link (\
            id, link_uuid, tenant_id, organization_id, conversation_id, \
            lifecycle_state, creation_idempotency_key, membership_epoch, \
            last_synchronized_membership_epoch, created_by, updated_by, \
            created_at, updated_at, version\
         ) VALUES (1, 'link-1', '100001', '1', 'conversation-1', \
            'provisioning', 'create-1', 0, 0, 'owner-1', 'owner-1', \
            '2026-01-01T00:00:00Z', '2026-01-01T00:00:00Z', 1)",
    )
    .execute(&sqlite_pool)
    .await
    .expect("seed upgradeable legacy provisioning link");

    let orchestrator = create_orchestrator(temp.path(), pool.clone()).await;
    let applied = orchestrator
        .migrate()
        .await
        .expect("anchored legacy database must run the upgrade");
    assert_eq!(applied, 1);
    assert!(!table_exists(&sqlite_pool, "baseline_must_not_run").await);
    assert_eq!(migration_history_count(&sqlite_pool, "0003").await, 1);
    assert!(
        column_exists(
            &sqlite_pool,
            "im_conversation_knowledge_space_link",
            "knowledgebase_binding_uuid",
        )
        .await
    );
    assert!(
        !column_exists(
            &sqlite_pool,
            "im_group_knowledge_launch_tickets",
            "binding_version",
        )
        .await
    );

    let repeated = orchestrator
        .migrate()
        .await
        .expect("anchored upgrade must be idempotent");
    assert_eq!(repeated, 0);
    pool.close().await;
}

#[tokio::test]
async fn legacy_active_link_upgrade_rolls_back_without_recording_history() {
    let temp = create_test_module(false, false);
    let pool = sqlite_pool().await;
    let sqlite_pool = sqlite(&pool).clone();
    prepare_anchored_legacy_database(&sqlite_pool).await;
    sqlx::query(
        "INSERT INTO im_conversation_knowledge_space_link (\
            id, link_uuid, tenant_id, organization_id, conversation_id, \
            knowledge_space_id, knowledge_space_uuid, knowledgebase_binding_id, \
            lifecycle_state, creation_idempotency_key, membership_epoch, \
            last_synchronized_membership_epoch, created_by, updated_by, \
            created_at, updated_at, version\
         ) VALUES (2, 'link-2', '100001', '1', 'conversation-2', \
            3, 'space-uuid-2', 4, 'active', 'create-2', 0, 0, \
            'owner-1', 'owner-1', '2026-01-01T00:00:00Z', \
            '2026-01-01T00:00:00Z', 1)",
    )
    .execute(&sqlite_pool)
    .await
    .expect("seed non-upgradeable active link");

    let orchestrator = create_orchestrator(temp.path(), pool.clone()).await;
    assert!(orchestrator.migrate().await.is_err());
    assert!(
        column_exists(
            &sqlite_pool,
            "im_conversation_knowledge_space_link",
            "knowledgebase_binding_id",
        )
        .await
    );
    assert!(
        !column_exists(
            &sqlite_pool,
            "im_conversation_knowledge_space_link",
            "knowledgebase_binding_uuid",
        )
        .await
    );
    assert_eq!(
        sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM im_conversation_knowledge_space_link WHERE id = 2",
        )
        .fetch_one(&sqlite_pool)
        .await
        .expect("read rolled back link"),
        1
    );
    assert_eq!(migration_history_count(&sqlite_pool, "0003").await, 0);
    pool.close().await;
}

#[tokio::test]
async fn legacy_launch_ticket_upgrade_rolls_back_without_recording_history() {
    let temp = create_test_module(false, false);
    let pool = sqlite_pool().await;
    let sqlite_pool = sqlite(&pool).clone();
    prepare_anchored_legacy_database(&sqlite_pool).await;
    sqlx::query(
        "INSERT INTO im_group_knowledge_launch_tickets (\
            id, ticket_hash, tenant_id, organization_id, conversation_id, \
            knowledge_space_id, knowledge_space_uuid, binding_version, \
            membership_epoch, actor_kind, actor_id, principal_kind, principal_id, \
            session_id, issued_by, idempotency_key_hash, request_fingerprint_hash, \
            ticket_ciphertext, expires_at, created_at\
         ) VALUES (3, 'ticket-hash-3', '100001', '1', 'conversation-3', \
            5, 'space-uuid-3', 1, 0, 'user', 'owner-1', 'user', 'owner-1', \
            'session-1', 'owner-1', 'idempotency-hash-3', 'fingerprint-hash-3', \
            'ciphertext-3', '2026-01-02T00:00:00Z', '2026-01-01T00:00:00Z')",
    )
    .execute(&sqlite_pool)
    .await
    .expect("seed non-upgradeable legacy ticket");

    let orchestrator = create_orchestrator(temp.path(), pool.clone()).await;
    assert!(orchestrator.migrate().await.is_err());
    assert!(
        column_exists(
            &sqlite_pool,
            "im_group_knowledge_launch_tickets",
            "binding_version",
        )
        .await
    );
    assert!(
        !column_exists(
            &sqlite_pool,
            "im_group_knowledge_launch_tickets",
            "knowledgebase_binding_uuid",
        )
        .await
    );
    assert_eq!(
        sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM im_group_knowledge_launch_tickets WHERE id = 3",
        )
        .fetch_one(&sqlite_pool)
        .await
        .expect("read rolled back ticket"),
        1
    );
    assert_eq!(migration_history_count(&sqlite_pool, "0003").await, 0);
    pool.close().await;
}
