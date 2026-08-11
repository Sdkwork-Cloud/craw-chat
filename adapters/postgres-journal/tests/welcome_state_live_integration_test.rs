//! Live PostgreSQL coverage for the `welcome.sent` marker write/read round
//! trip and idempotent upsert.
//!
//! Regression guard: `write_welcome_sent` must bind the payload as a JSON
//! native type (`postgres::types::Json`) for the `$4::jsonb` parameter;
//! binding a plain `String` makes `ToSql` reject JSONB and fails with
//! `error serializing parameter 3`.

use std::time::{SystemTime, UNIX_EPOCH};

use im_adapters_postgres_journal::{PostgresJournalConfig, PostgresWelcomeStateStore};
use im_platform_contracts::{WelcomeSentRecord, WelcomeStateStore};

fn suffix() -> String {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock must be after epoch")
        .as_nanos()
        .to_string()
}

fn record(
    tenant_id: &str,
    organization_id: &str,
    user_id: &str,
    welcome_version: &str,
) -> WelcomeSentRecord {
    WelcomeSentRecord {
        tenant_id: tenant_id.to_owned(),
        organization_id: organization_id.to_owned(),
        user_id: user_id.to_owned(),
        conversation_id: format!("conv-{user_id}"),
        message_id: format!("msg-{user_id}"),
        message_seq: 1,
        welcome_version: welcome_version.to_owned(),
        sent_at: chrono::Utc::now().to_rfc3339(),
    }
}

#[tokio::test(flavor = "multi_thread")]
#[ignore = "requires live PostgreSQL via SDKWORK_DATABASE_URL"]
async fn welcome_sent_marker_writes_reads_and_upserts() {
    let database_url =
        std::env::var("SDKWORK_DATABASE_URL").expect("SDKWORK_DATABASE_URL must be set");
    sdkwork_im_database_pool::bootstrap_im_process_database_pools_from_env()
        .await
        .expect("shared IM database pools should bootstrap");
    let pool = PostgresJournalConfig::new(database_url)
        .connect_pool()
        .expect("PostgreSQL pool should connect");
    let store = PostgresWelcomeStateStore::from_pool(pool);

    let suffix = suffix();
    let tenant_id = format!("9{}", &suffix[suffix.len().saturating_sub(15)..]);
    let organization_id = "0";
    let user_id = format!("u_live_{}", &suffix[suffix.len().saturating_sub(12)..]);

    // 未写过的用户 → None。
    assert!(
        store
            .read_welcome_sent(&tenant_id, organization_id, &user_id)
            .expect("read should succeed")
            .is_none(),
        "never-written marker must read as None"
    );

    // 写入 → 原样读回（JSONB 往返不能丢字段）。
    let first = record(&tenant_id, organization_id, &user_id, "v1");
    store
        .write_welcome_sent(&first)
        .expect("welcome.sent write should serialize the jsonb payload");
    let read_back = store
        .read_welcome_sent(&tenant_id, organization_id, &user_id)
        .expect("read after write should succeed")
        .expect("written marker must be present");
    assert_eq!(read_back, first);

    // 幂等 upsert：同 key 再次写入新值 → 读到新值。
    let second = record(&tenant_id, organization_id, &user_id, "v2");
    store
        .write_welcome_sent(&second)
        .expect("idempotent upsert should succeed");
    let read_back = store
        .read_welcome_sent(&tenant_id, organization_id, &user_id)
        .expect("read after upsert should succeed")
        .expect("upserted marker must be present");
    assert_eq!(read_back, second);

    // 组织隔离：其他 organization 下同 user 不受影响。
    let other_org = "1";
    assert!(
        store
            .read_welcome_sent(&tenant_id, other_org, &user_id)
            .expect("isolated read should succeed")
            .is_none(),
        "marker must be scoped per organization"
    );
}
