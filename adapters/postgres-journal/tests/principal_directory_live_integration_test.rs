//! Live PostgreSQL coverage for the dynamic principal directory:
//! authenticated-means-registered admission, audit record, and the explicit
//! disable flag.

use std::time::{SystemTime, UNIX_EPOCH};

use im_adapters_postgres_journal::{PostgresJournalConfig, PostgresPrincipalDirectory};
use im_platform_contracts::{PrincipalDirectory, PrincipalDirectoryError};
use postgres::types::Json;
use serde_json::json;

fn suffix() -> String {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock must be after epoch")
        .as_nanos()
        .to_string()
}

fn unique_tenant_id() -> String {
    let suffix = suffix();
    format!("9{}", &suffix[suffix.len().saturating_sub(15)..])
}

fn write_setting(
    pool: &im_adapters_postgres_journal::PostgresJournalPool,
    tenant_id: &str,
    user_id: &str,
    setting_key: &str,
    value: serde_json::Value,
) {
    let mut client = pool
        .inner()
        .get()
        .expect("postgres client should be available");
    client
        .execute(
            r#"
insert into im_user_settings (tenant_id, organization_id, user_id, setting_key, setting_value, updated_at)
values ($1, '0', $2, $3, $4::jsonb, now())
on conflict (tenant_id, organization_id, user_id, setting_key) do update
set setting_value = excluded.setting_value, updated_at = excluded.updated_at
"#,
            &[&tenant_id, &user_id, &setting_key, &Json(value)],
        )
        .expect("setting upsert should succeed");
}

#[tokio::test(flavor = "multi_thread")]
#[ignore = "requires live PostgreSQL via SDKWORK_DATABASE_URL"]
async fn first_sight_user_is_admitted_and_recorded() {
    let database_url =
        std::env::var("SDKWORK_DATABASE_URL").expect("SDKWORK_DATABASE_URL must be set");
    sdkwork_im_database_pool::bootstrap_im_process_database_pools_from_env()
        .await
        .expect("shared IM database pools should bootstrap");
    let pool = PostgresJournalConfig::new(database_url)
        .connect_pool()
        .expect("PostgreSQL pool should connect");
    let directory = PostgresPrincipalDirectory::from_pool(pool.clone());

    let tenant_id = unique_tenant_id();
    let user_id = "u_first_sight";

    // Never seen before: admitted on first sight and recorded for audit.
    directory
        .ensure_active_principal(&tenant_id, user_id, "user")
        .expect("first-sight user must be admitted");

    let mut client = pool.inner().get().expect("client");
    let rows = client
        .query(
            r#"
select setting_value
from im_user_settings
where tenant_id = $1 and organization_id = $2 and user_id = $3 and setting_key = 'principal.seen'
"#,
            &[&tenant_id, &"0", &user_id],
        )
        .expect("seen record should be readable");
    assert_eq!(
        rows.len(),
        1,
        "first-sight admission must be recorded for audit"
    );

    // Repeat admission stays admitted (idempotent upsert).
    directory
        .ensure_active_principal(&tenant_id, user_id, "user")
        .expect("repeat admission must stay admitted");

    // Non-user principals bypass the directory.
    directory
        .ensure_active_principal(&tenant_id, "system", "system")
        .expect("non-user principals must bypass the directory");
}

#[tokio::test(flavor = "multi_thread")]
#[ignore = "requires live PostgreSQL via SDKWORK_DATABASE_URL"]
async fn disabled_user_is_rejected() {
    let database_url =
        std::env::var("SDKWORK_DATABASE_URL").expect("SDKWORK_DATABASE_URL must be set");
    sdkwork_im_database_pool::bootstrap_im_process_database_pools_from_env()
        .await
        .expect("shared IM database pools should bootstrap");
    let pool = PostgresJournalConfig::new(database_url)
        .connect_pool()
        .expect("PostgreSQL pool should connect");
    let directory = PostgresPrincipalDirectory::from_pool(pool.clone());

    let tenant_id = unique_tenant_id();
    let user_id = "u_disabled";
    write_setting(
        &pool,
        &tenant_id,
        user_id,
        "principal.disabled",
        json!({ "disabled": true, "disabledAt": "2026-08-10T00:00:00Z" }),
    );

    let error = directory
        .ensure_active_principal(&tenant_id, user_id, "user")
        .expect_err("disabled user must be rejected");
    assert!(
        matches!(error, PrincipalDirectoryError::PrincipalDisabled { .. }),
        "expected PrincipalDisabled, got {error:?}"
    );
}

#[tokio::test(flavor = "multi_thread")]
#[ignore = "requires live PostgreSQL via SDKWORK_DATABASE_URL"]
async fn disabled_flag_false_admits() {
    let database_url =
        std::env::var("SDKWORK_DATABASE_URL").expect("SDKWORK_DATABASE_URL must be set");
    sdkwork_im_database_pool::bootstrap_im_process_database_pools_from_env()
        .await
        .expect("shared IM database pools should bootstrap");
    let pool = PostgresJournalConfig::new(database_url)
        .connect_pool()
        .expect("PostgreSQL pool should connect");
    let directory = PostgresPrincipalDirectory::from_pool(pool.clone());

    let tenant_id = unique_tenant_id();
    let user_id = "u_not_disabled";
    write_setting(
        &pool,
        &tenant_id,
        user_id,
        "principal.disabled",
        json!({ "disabled": false }),
    );

    directory
        .ensure_active_principal(&tenant_id, user_id, "user")
        .expect("disabled=false must admit the user");
}
