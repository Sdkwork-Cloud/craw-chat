use std::fs::{self, OpenOptions};
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use std::time::Duration;

use chrono::{SecondsFormat, Utc};
use rusqlite::{params, Connection, OptionalExtension};
use sha2::{Digest, Sha256};
use url::Url;

use super::crypto::PayloadCipher;
use super::{validate_identifier, OfflinePrincipalScope};

pub(super) const OFFLINE_SCHEMA_VERSION: i64 = 4;
const DATABASE_FILE_NAME: &str = "offline-im-cache.sqlite";
const LEGACY_DATABASE_FILE_NAME: &str = "offline-im-cache.sqlite";
const INITIALIZATION_LOCK_FILE_NAME: &str = "offline-im-cache.init.lock";
const DATABASE_ROOT: &str = "client-local/im-cache/v1";
const CURRENT_MIGRATION: &str =
    include_str!("../../database/migrations/sqlite/0004_create_im_pc_client_local_store.up.sql");
#[cfg(test)]
const CURRENT_BASELINE: &str =
    include_str!("../../database/ddl/baseline/sqlite/0004_im_pc_client_local_store.sql");

const EXPECTED_SCHEMA_OBJECTS: [&str; 12] = [
    "index:idx_im_local_cache_cursor_scope_cached",
    "index:idx_im_local_conversation_cache_scope_updated",
    "index:idx_im_local_message_cache_scope_cached",
    "index:idx_im_local_message_cache_scope_conversation_seq",
    "index:idx_im_local_pending_send_scope_claim",
    "index:idx_im_local_pending_send_scope_created",
    "index:idx_im_local_pending_send_scope_status_created",
    "table:im_local_cache_cursor",
    "table:im_local_conversation_cache",
    "table:im_local_installation",
    "table:im_local_message_cache",
    "table:im_local_pending_send",
];

pub(super) struct OfflineDatabase {
    pub(super) connection: Connection,
    pub(super) cipher: PayloadCipher,
    pub(super) scope_fingerprint: String,
}

pub(super) fn scope_fingerprint(scope: &OfflinePrincipalScope) -> Result<String, String> {
    validate_scope_identity(scope)?;
    let mut canonical = String::from("sdkwork-im-pc-client-local-scope:v1;");
    for value in [
        scope.environment.as_str(),
        scope.deployment_profile.as_str(),
        scope.deployment_mode.as_str(),
        scope.api_origin.as_str(),
        scope.tenant_id.as_str(),
        scope.organization_id.as_str(),
        scope.account_id.as_str(),
        scope.principal_kind.as_str(),
        scope.principal_id.as_str(),
    ] {
        canonical.push_str(format!("{}:{value};", value.len()).as_str());
    }
    Ok(format!("{:x}", Sha256::digest(canonical.as_bytes())))
}

pub(super) fn scoped_database_path(
    app_data_dir: &Path,
    scope: &OfflinePrincipalScope,
) -> Result<PathBuf, String> {
    Ok(app_data_dir
        .join(DATABASE_ROOT)
        .join(scope_fingerprint(scope)?)
        .join(DATABASE_FILE_NAME))
}

pub(super) fn remove_legacy_unscoped_database(app_data_dir: &Path) -> Result<(), String> {
    remove_sqlite_family(app_data_dir.join(LEGACY_DATABASE_FILE_NAME).as_path())
}

pub(super) fn open_scoped_database(
    app_data_dir: &Path,
    scope: &OfflinePrincipalScope,
) -> Result<OfflineDatabase, String> {
    let fingerprint = scope_fingerprint(scope)?;
    let path = scoped_database_path(app_data_dir, scope)?;
    with_scope_initialization_lock(path.as_path(), || {
        let connection = open_database(path.as_path(), scope, fingerprint.as_str())?;
        let cipher = load_scope_cipher(&connection, fingerprint.as_str())?;
        Ok(OfflineDatabase {
            connection,
            cipher,
            scope_fingerprint: fingerprint,
        })
    })
}

fn with_scope_initialization_lock<R>(
    database_path: &Path,
    operation: impl FnOnce() -> Result<R, String>,
) -> Result<R, String> {
    let parent = database_path
        .parent()
        .ok_or_else(|| "client-local database path has no parent directory".to_owned())?;
    fs::create_dir_all(parent)
        .map_err(|error| format!("create client-local database directory failed: {error}"))?;
    restrict_directory_permissions(parent)?;

    let lock_path = parent.join(INITIALIZATION_LOCK_FILE_NAME);
    let lock_file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(lock_path.as_path())
        .map_err(|error| format!("open client-local initialization lock failed: {error}"))?;
    restrict_file_permissions(lock_path.as_path())?;
    lock_file
        .lock()
        .map_err(|error| format!("acquire client-local initialization lock failed: {error}"))?;

    let result = operation();
    let unlock_result = lock_file
        .unlock()
        .map_err(|error| format!("release client-local initialization lock failed: {error}"));
    match (result, unlock_result) {
        (Ok(value), Ok(())) => Ok(value),
        (Err(error), Ok(())) => Err(error),
        (Ok(_), Err(error)) => Err(error),
        (Err(operation_error), Err(unlock_error)) => {
            Err(format!("{operation_error}; additionally, {unlock_error}"))
        }
    }
}

fn load_scope_cipher(
    connection: &Connection,
    scope_fingerprint: &str,
) -> Result<PayloadCipher, String> {
    let persisted = first_persisted_ciphertext(connection)?;
    let cipher = PayloadCipher::from_keyring(scope_fingerprint, persisted.is_none())?;
    if let Some((purpose, record_key, ciphertext)) = persisted {
        cipher
            .decrypt_json(purpose.as_str(), record_key.as_str(), ciphertext.as_str())
            .map_err(|error| {
                format!("client-local encryption key validation failed closed: {error}")
            })?;
    }
    Ok(cipher)
}

fn first_persisted_ciphertext(
    connection: &Connection,
) -> Result<Option<(String, String, String)>, String> {
    let conversation = connection
        .query_row(
            "SELECT conversation_id, payload_ciphertext FROM im_local_conversation_cache LIMIT 1",
            [],
            |row| Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?)),
        )
        .optional()
        .map_err(|error| format!("inspect encrypted conversation cache failed: {error}"))?;
    if let Some((record_key, ciphertext)) = conversation {
        return Ok(Some((
            "conversation-cache".to_owned(),
            record_key,
            ciphertext,
        )));
    }

    let message = connection
        .query_row(
            "SELECT conversation_id, message_seq, payload_ciphertext FROM im_local_message_cache LIMIT 1",
            [],
            |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, i64>(1)?,
                    row.get::<_, String>(2)?,
                ))
            },
        )
        .optional()
        .map_err(|error| format!("inspect encrypted message cache failed: {error}"))?;
    if let Some((conversation_id, message_seq, ciphertext)) = message {
        return Ok(Some((
            "message-cache".to_owned(),
            format!("{conversation_id}:{message_seq}"),
            ciphertext,
        )));
    }

    let cursor = connection
        .query_row(
            "SELECT cursor_scope, cursor_ciphertext FROM im_local_cache_cursor LIMIT 1",
            [],
            |row| Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?)),
        )
        .optional()
        .map_err(|error| format!("inspect encrypted cache cursor failed: {error}"))?;
    if let Some((record_key, ciphertext)) = cursor {
        return Ok(Some(("cache-cursor".to_owned(), record_key, ciphertext)));
    }

    let pending_send = connection
        .query_row(
            "SELECT client_msg_id, payload_ciphertext FROM im_local_pending_send LIMIT 1",
            [],
            |row| Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?)),
        )
        .optional()
        .map_err(|error| format!("inspect encrypted pending send failed: {error}"))?;
    Ok(pending_send
        .map(|(record_key, ciphertext)| ("pending-send".to_owned(), record_key, ciphertext)))
}

fn open_database(
    path: &Path,
    scope: &OfflinePrincipalScope,
    fingerprint: &str,
) -> Result<Connection, String> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|error| format!("create client-local database directory failed: {error}"))?;
        restrict_directory_permissions(parent)?;
    }

    let existed = path.is_file();
    if existed {
        let inspection = Connection::open(path)
            .map_err(|error| format!("inspect client-local sqlite database failed: {error}"));
        match inspection {
            Ok(connection) => match schema_version(&connection) {
                Ok(version) if version > OFFLINE_SCHEMA_VERSION => {
                    return Err(format!(
                            "client-local schema version {version} is newer than supported version {OFFLINE_SCHEMA_VERSION}"
                        ));
                }
                Ok(version) => {
                    let is_current = version == OFFLINE_SCHEMA_VERSION
                        && configure_connection(&connection).is_ok()
                        && integrity_is_clean(&connection)
                        && schema_inventory(&connection)
                            .is_ok_and(|inventory| inventory == EXPECTED_SCHEMA_OBJECTS)
                        && installation_matches(&connection, scope, fingerprint).unwrap_or(false);
                    drop(connection);
                    if !is_current {
                        remove_sqlite_family(path)?;
                    }
                }
                Err(_) => {
                    drop(connection);
                    remove_sqlite_family(path)?;
                }
            },
            Err(_) => remove_sqlite_family(path)?,
        }
    }

    let connection = Connection::open(path)
        .map_err(|error| format!("open client-local sqlite database failed: {error}"))?;
    configure_connection(&connection)?;
    if schema_version(&connection)? == 0 {
        connection
            .execute_batch(CURRENT_MIGRATION)
            .map_err(|error| format!("apply client-local sqlite migration failed: {error}"))?;
        bind_installation(&connection, scope, fingerprint)?;
    }
    if schema_version(&connection)? != OFFLINE_SCHEMA_VERSION
        || !integrity_is_clean(&connection)
        || schema_inventory(&connection)? != EXPECTED_SCHEMA_OBJECTS
        || !installation_matches(&connection, scope, fingerprint)?
    {
        return Err("client-local sqlite database failed schema or scope validation".to_owned());
    }
    connection
        .execute_batch("PRAGMA optimize;")
        .map_err(|error| format!("optimize client-local sqlite database failed: {error}"))?;
    restrict_file_permissions(path)?;
    Ok(connection)
}

fn configure_connection(connection: &Connection) -> Result<(), String> {
    connection
        .busy_timeout(Duration::from_secs(5))
        .map_err(|error| format!("configure client-local sqlite busy timeout failed: {error}"))?;
    connection
        .execute_batch(
            r#"
            PRAGMA journal_mode = WAL;
            PRAGMA synchronous = FULL;
            PRAGMA foreign_keys = ON;
            PRAGMA trusted_schema = OFF;
            PRAGMA secure_delete = ON;
            PRAGMA temp_store = MEMORY;
            PRAGMA journal_size_limit = 67108864;
            PRAGMA wal_autocheckpoint = 1000;
            "#,
        )
        .map_err(|error| format!("configure client-local sqlite connection failed: {error}"))?;
    Ok(())
}

fn bind_installation(
    connection: &Connection,
    scope: &OfflinePrincipalScope,
    fingerprint: &str,
) -> Result<(), String> {
    let now = Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true);
    connection
        .execute(
            r#"
            INSERT INTO im_local_installation (
                installation_key, schema_version, scope_fingerprint, environment,
                deployment_profile, deployment_mode, api_origin, tenant_id,
                organization_id, account_id, principal_kind, principal_id,
                created_at, updated_at
            ) VALUES (
                'current', ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?12
            )
            "#,
            params![
                OFFLINE_SCHEMA_VERSION,
                fingerprint,
                &scope.environment,
                &scope.deployment_profile,
                &scope.deployment_mode,
                &scope.api_origin,
                &scope.tenant_id,
                &scope.organization_id,
                &scope.account_id,
                &scope.principal_kind,
                &scope.principal_id,
                now,
            ],
        )
        .map_err(|error| format!("bind client-local database scope failed: {error}"))?;
    Ok(())
}

fn installation_matches(
    connection: &Connection,
    scope: &OfflinePrincipalScope,
    fingerprint: &str,
) -> Result<bool, String> {
    let stored = connection
        .query_row(
            r#"
            SELECT schema_version, scope_fingerprint, environment, deployment_profile,
                   deployment_mode, api_origin, tenant_id, organization_id, account_id,
                   principal_kind, principal_id
            FROM im_local_installation
            WHERE installation_key = 'current'
            "#,
            [],
            |row| {
                Ok((
                    row.get::<_, i64>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                    row.get::<_, String>(3)?,
                    row.get::<_, String>(4)?,
                    row.get::<_, String>(5)?,
                    row.get::<_, String>(6)?,
                    row.get::<_, String>(7)?,
                    row.get::<_, String>(8)?,
                    row.get::<_, String>(9)?,
                    row.get::<_, String>(10)?,
                ))
            },
        )
        .optional()
        .map_err(|error| format!("read client-local database scope failed: {error}"))?;
    Ok(stored.is_some_and(|stored| {
        stored.0 == OFFLINE_SCHEMA_VERSION
            && stored.1 == fingerprint
            && stored.2 == scope.environment
            && stored.3 == scope.deployment_profile
            && stored.4 == scope.deployment_mode
            && stored.5 == scope.api_origin
            && stored.6 == scope.tenant_id
            && stored.7 == scope.organization_id
            && stored.8 == scope.account_id
            && stored.9 == scope.principal_kind
            && stored.10 == scope.principal_id
    }))
}

fn schema_version(connection: &Connection) -> Result<i64, String> {
    connection
        .query_row("PRAGMA user_version", [], |row| row.get(0))
        .map_err(|error| format!("read client-local sqlite schema version failed: {error}"))
}

fn integrity_is_clean(connection: &Connection) -> bool {
    connection
        .query_row("PRAGMA quick_check", [], |row| row.get::<_, String>(0))
        .is_ok_and(|result| result == "ok")
}

fn schema_inventory(connection: &Connection) -> Result<Vec<String>, String> {
    let mut statement = connection
        .prepare(
            "SELECT type || ':' || name FROM sqlite_schema WHERE name NOT LIKE 'sqlite_%' ORDER BY 1",
        )
        .map_err(|error| format!("prepare client-local schema inventory failed: {error}"))?;
    let inventory = statement
        .query_map([], |row| row.get(0))
        .map_err(|error| format!("query client-local schema inventory failed: {error}"))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| format!("collect client-local schema inventory failed: {error}"))?;
    Ok(inventory)
}

fn validate_scope_identity(scope: &OfflinePrincipalScope) -> Result<(), String> {
    if !matches!(
        scope.environment.as_str(),
        "development" | "test" | "staging" | "production"
    ) {
        return Err("environment must be development, test, staging, or production".to_owned());
    }
    if !matches!(scope.deployment_profile.as_str(), "standalone" | "cloud") {
        return Err("deploymentProfile must be standalone or cloud".to_owned());
    }
    if !matches!(scope.deployment_mode.as_str(), "local" | "private" | "saas") {
        return Err("deploymentMode must be local, private, or saas".to_owned());
    }
    let origin = Url::parse(scope.api_origin.as_str())
        .map_err(|_| "apiOrigin must be a normalized HTTP(S) origin".to_owned())?;
    if scope.api_origin.len() > 2_048
        || !matches!(origin.scheme(), "http" | "https")
        || !origin.username().is_empty()
        || origin.password().is_some()
        || origin.query().is_some()
        || origin.fragment().is_some()
        || origin.path() != "/"
        || origin.origin().ascii_serialization() != scope.api_origin
    {
        return Err("apiOrigin must be a normalized credential-free HTTP(S) origin".to_owned());
    }
    for (field, value) in [
        ("tenantId", scope.tenant_id.as_str()),
        ("organizationId", scope.organization_id.as_str()),
        ("accountId", scope.account_id.as_str()),
        ("principalId", scope.principal_id.as_str()),
    ] {
        validate_identifier(field, value)?;
        if value.trim() != value {
            return Err(format!("{field} must already be normalized"));
        }
    }
    if !matches!(
        scope.principal_kind.as_str(),
        "user" | "agent" | "system" | "service"
    ) {
        return Err("principalKind must be user, agent, system, or service".to_owned());
    }
    Ok(())
}

fn remove_sqlite_family(path: &Path) -> Result<(), String> {
    for candidate in [
        path.to_path_buf(),
        sqlite_sidecar_path(path, "-wal"),
        sqlite_sidecar_path(path, "-shm"),
    ] {
        if candidate.is_file() {
            if let Err(error) = fs::remove_file(candidate.as_path()) {
                if error.kind() != ErrorKind::NotFound {
                    return Err(format!(
                        "remove untrusted client-local sqlite file {} failed: {error}",
                        candidate.display()
                    ));
                }
            }
        }
    }
    Ok(())
}

fn sqlite_sidecar_path(path: &Path, suffix: &str) -> PathBuf {
    let mut value = path.as_os_str().to_os_string();
    value.push(suffix);
    PathBuf::from(value)
}

#[cfg(unix)]
fn restrict_directory_permissions(path: &Path) -> Result<(), String> {
    use std::os::unix::fs::PermissionsExt;

    fs::set_permissions(path, fs::Permissions::from_mode(0o700)).map_err(|error| {
        format!("restrict client-local database directory permissions failed: {error}")
    })
}

#[cfg(not(unix))]
fn restrict_directory_permissions(_path: &Path) -> Result<(), String> {
    Ok(())
}

#[cfg(unix)]
fn restrict_file_permissions(path: &Path) -> Result<(), String> {
    use std::os::unix::fs::PermissionsExt;

    fs::set_permissions(path, fs::Permissions::from_mode(0o600))
        .map_err(|error| format!("restrict client-local database file permissions failed: {error}"))
}

#[cfg(not(unix))]
fn restrict_file_permissions(_path: &Path) -> Result<(), String> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Barrier};
    use std::thread;
    use std::time::{SystemTime, UNIX_EPOCH};

    use super::*;

    fn scope() -> OfflinePrincipalScope {
        OfflinePrincipalScope {
            environment: "development".into(),
            deployment_profile: "standalone".into(),
            deployment_mode: "local".into(),
            api_origin: "http://127.0.0.1:18079".into(),
            tenant_id: "100001".into(),
            organization_id: "org-a".into(),
            account_id: "account-a".into(),
            principal_kind: "user".into(),
            principal_id: "user-a".into(),
        }
    }

    fn temp_path(name: &str) -> PathBuf {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock")
            .as_nanos();
        std::env::temp_dir()
            .join(format!("sdkwork-im-pc-client-local-{name}-{nonce}"))
            .join(DATABASE_FILE_NAME)
    }

    #[test]
    fn baseline_and_runtime_migration_materialize_the_same_inventory() {
        let migration = Connection::open_in_memory().expect("migration database");
        migration
            .execute_batch(CURRENT_MIGRATION)
            .expect("apply migration");
        let baseline = Connection::open_in_memory().expect("baseline database");
        baseline
            .execute_batch(CURRENT_BASELINE)
            .expect("apply baseline");
        assert_eq!(
            schema_inventory(&migration).expect("migration inventory"),
            EXPECTED_SCHEMA_OBJECTS
        );
        assert_eq!(
            schema_inventory(&baseline).expect("baseline inventory"),
            EXPECTED_SCHEMA_OBJECTS
        );
        assert_eq!(
            schema_inventory(&migration).expect("migration inventory"),
            schema_inventory(&baseline).expect("baseline inventory")
        );
    }

    #[test]
    fn every_scope_dimension_changes_the_database_identity() {
        let baseline = scope();
        let baseline_fingerprint = scope_fingerprint(&baseline).expect("fingerprint");
        let mutations = [
            ("environment", "test"),
            ("deployment_profile", "cloud"),
            ("deployment_mode", "private"),
            ("api_origin", "https://im.example.com"),
            ("tenant_id", "100002"),
            ("organization_id", "org-b"),
            ("account_id", "account-b"),
            ("principal_kind", "agent"),
            ("principal_id", "agent-b"),
        ];
        for (field, value) in mutations {
            let mut changed = baseline.clone();
            match field {
                "environment" => changed.environment = value.into(),
                "deployment_profile" => changed.deployment_profile = value.into(),
                "deployment_mode" => changed.deployment_mode = value.into(),
                "api_origin" => changed.api_origin = value.into(),
                "tenant_id" => changed.tenant_id = value.into(),
                "organization_id" => changed.organization_id = value.into(),
                "account_id" => changed.account_id = value.into(),
                "principal_kind" => changed.principal_kind = value.into(),
                "principal_id" => changed.principal_id = value.into(),
                _ => unreachable!(),
            }
            assert_ne!(
                scope_fingerprint(&changed).expect("changed fingerprint"),
                baseline_fingerprint,
                "{field} must participate in the file identity"
            );
        }
    }

    #[test]
    fn account_scopes_use_distinct_database_and_wal_families() {
        let root = temp_path("scope-root")
            .parent()
            .expect("temporary root")
            .to_path_buf();
        let first = scope();
        let mut second = first.clone();
        second.account_id = "account-b".into();
        second.principal_id = "user-b".into();
        let first_path = scoped_database_path(root.as_path(), &first).expect("first path");
        let second_path = scoped_database_path(root.as_path(), &second).expect("second path");
        assert_ne!(first_path, second_path);
        assert_ne!(
            format!("{}-wal", first_path.display()),
            format!("{}-wal", second_path.display())
        );
        assert_eq!(
            first_path.file_name().and_then(|value| value.to_str()),
            Some(DATABASE_FILE_NAME)
        );
        assert_eq!(
            first_path
                .parent()
                .and_then(Path::file_name)
                .and_then(|value| value.to_str())
                .map(str::len),
            Some(64)
        );
    }

    #[test]
    fn concurrent_initialization_is_serialized_per_scope() {
        let root = Arc::new(
            temp_path("concurrent-initialization")
                .parent()
                .expect("temp parent")
                .to_path_buf(),
        );
        let scope = Arc::new(scope());
        let database_path =
            Arc::new(scoped_database_path(root.as_path(), scope.as_ref()).expect("database path"));
        let barrier = Arc::new(Barrier::new(8));
        let mut workers = Vec::new();
        for _ in 0..8 {
            let scope = Arc::clone(&scope);
            let database_path = Arc::clone(&database_path);
            let barrier = Arc::clone(&barrier);
            workers.push(thread::spawn(move || {
                let fingerprint = scope_fingerprint(scope.as_ref()).expect("fingerprint");
                barrier.wait();
                with_scope_initialization_lock(database_path.as_path(), || {
                    let connection = open_database(
                        database_path.as_path(),
                        scope.as_ref(),
                        fingerprint.as_str(),
                    )?;
                    schema_version(&connection)
                })
            }));
        }
        for worker in workers {
            assert_eq!(
                worker.join().expect("initialization worker").expect("open"),
                OFFLINE_SCHEMA_VERSION
            );
        }
        fs::remove_dir_all(root.as_path()).expect("remove concurrent initialization directory");
    }

    #[test]
    fn persisted_ciphertext_probe_rejects_a_different_key() {
        let connection = Connection::open_in_memory().expect("open test database");
        connection
            .execute_batch(CURRENT_MIGRATION)
            .expect("create schema");
        let correct = PayloadCipher::for_test(&"a".repeat(64));
        let ciphertext = correct
            .encrypt_json("conversation-cache", "conversation", r#"{"value":"saved"}"#)
            .expect("encrypt fixture");
        connection
            .execute(
                "INSERT INTO im_local_conversation_cache (tenant_id, organization_id, principal_kind, principal_id, conversation_id, payload_ciphertext, updated_at, cached_at_ms) VALUES ('tenant', 'organization', 'user', 'principal', 'conversation', ?1, '2026-07-24T00:00:00Z', 1)",
                [ciphertext],
            )
            .expect("insert encrypted fixture");
        let persisted = first_persisted_ciphertext(&connection)
            .expect("inspect ciphertext")
            .expect("ciphertext probe");
        assert!(correct
            .decrypt_json(
                persisted.0.as_str(),
                persisted.1.as_str(),
                persisted.2.as_str()
            )
            .is_ok());
        assert!(PayloadCipher::for_test(&"b".repeat(64))
            .decrypt_json(
                persisted.0.as_str(),
                persisted.1.as_str(),
                persisted.2.as_str()
            )
            .is_err());
    }

    #[test]
    fn prelaunch_schema_without_full_scope_is_rebuilt_fail_closed() {
        let path = temp_path("legacy");
        fs::create_dir_all(path.parent().expect("parent")).expect("create parent");
        let legacy = Connection::open(path.as_path()).expect("open legacy");
        legacy
            .execute_batch(
                "CREATE TABLE offline_messages (payload_json TEXT); PRAGMA user_version = 3;",
            )
            .expect("create legacy schema");
        drop(legacy);

        let scope = scope();
        let fingerprint = scope_fingerprint(&scope).expect("fingerprint");
        let connection = open_database(path.as_path(), &scope, fingerprint.as_str())
            .expect("rebuild client-local database");
        assert_eq!(
            schema_inventory(&connection).expect("inventory"),
            EXPECTED_SCHEMA_OBJECTS
        );
        let retired_count: i64 = connection
            .query_row(
                "SELECT COUNT(*) FROM sqlite_schema WHERE name = 'offline_messages'",
                [],
                |row| row.get(0),
            )
            .expect("retired table count");
        assert_eq!(retired_count, 0);
        drop(connection);
        remove_sqlite_family(path.as_path()).expect("cleanup");
        let _ = fs::remove_dir_all(path.parent().expect("parent"));
    }

    #[test]
    fn runtime_connection_applies_the_client_local_sqlite_profile() {
        let path = temp_path("pragma-profile");
        let scope = scope();
        let fingerprint = scope_fingerprint(&scope).expect("fingerprint");
        let connection = open_database(path.as_path(), &scope, fingerprint.as_str())
            .expect("open client-local database");
        let journal_mode: String = connection
            .query_row("PRAGMA journal_mode", [], |row| row.get(0))
            .expect("journal mode");
        let synchronous: i64 = connection
            .query_row("PRAGMA synchronous", [], |row| row.get(0))
            .expect("synchronous");
        let foreign_keys: i64 = connection
            .query_row("PRAGMA foreign_keys", [], |row| row.get(0))
            .expect("foreign keys");
        let trusted_schema: i64 = connection
            .query_row("PRAGMA trusted_schema", [], |row| row.get(0))
            .expect("trusted schema");
        let secure_delete: i64 = connection
            .query_row("PRAGMA secure_delete", [], |row| row.get(0))
            .expect("secure delete");
        let busy_timeout: i64 = connection
            .query_row("PRAGMA busy_timeout", [], |row| row.get(0))
            .expect("busy timeout");
        assert_eq!(journal_mode, "wal");
        assert_eq!(synchronous, 2);
        assert_eq!(foreign_keys, 1);
        assert_eq!(trusted_schema, 0);
        assert_eq!(secure_delete, 1);
        assert_eq!(busy_timeout, 5_000);
        drop(connection);
        remove_sqlite_family(path.as_path()).expect("cleanup");
        let _ = fs::remove_dir_all(path.parent().expect("parent"));
    }

    #[test]
    fn newer_schema_fails_without_deleting_the_file() {
        let path = temp_path("future");
        fs::create_dir_all(path.parent().expect("parent")).expect("create parent");
        let future = Connection::open(path.as_path()).expect("open future");
        future
            .execute_batch("CREATE TABLE future_table (value TEXT); PRAGMA user_version = 5;")
            .expect("create future schema");
        drop(future);
        let scope = scope();
        let fingerprint = scope_fingerprint(&scope).expect("fingerprint");
        assert!(open_database(path.as_path(), &scope, fingerprint.as_str()).is_err());
        assert!(path.is_file());
        remove_sqlite_family(path.as_path()).expect("cleanup");
        let _ = fs::remove_dir_all(path.parent().expect("parent"));
    }

    #[test]
    fn corrupt_database_is_discarded_and_rebuilt() {
        let path = temp_path("corrupt");
        fs::create_dir_all(path.parent().expect("parent")).expect("create parent");
        fs::write(path.as_path(), b"not-a-sqlite-database").expect("write corrupt database");
        let scope = scope();
        let fingerprint = scope_fingerprint(&scope).expect("fingerprint");
        let connection = open_database(path.as_path(), &scope, fingerprint.as_str())
            .expect("rebuild corrupt cache");
        assert_eq!(
            schema_inventory(&connection).expect("inventory"),
            EXPECTED_SCHEMA_OBJECTS
        );
        drop(connection);
        remove_sqlite_family(path.as_path()).expect("cleanup");
        let _ = fs::remove_dir_all(path.parent().expect("parent"));
    }
}
