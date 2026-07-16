use im_adapters_postgres_projection::PostgresProjectionConfig;
use im_platform_contracts::{MetadataStore, TimelineProjectionStore};
use r2d2_postgres::postgres::{Client, NoTls};
use sdkwork_im_contract_message::{TimelineProjectionRecord, TimelineProjectionScope};

const POSTGRES_TEST_DATABASE_URL_ENV: &str = "SDKWORK_IM_POSTGRES_TEST_DATABASE_URL";
const CORE_SCHEMA_SQL: &str =
    include_str!("../../../database/ddl/baseline/postgres/0001_im_baseline.sql");

#[test]
fn test_postgres_projection_pool_connect_bridges_from_tokio_runtime() {
    let source = include_str!("../src/lib.rs");
    assert!(
        source.contains("connect_pool_bridged"),
        "PostgreSQL projection adapter must bridge pool creation off Tokio worker threads"
    );
    assert!(
        source.contains("build_projection_pool"),
        "PostgreSQL projection adapter must isolate pool construction in build_projection_pool"
    );
}

#[test]
fn test_postgres_projection_live_store_roundtrip_when_database_is_configured() {
    let Some(database_url) = std::env::var(POSTGRES_TEST_DATABASE_URL_ENV)
        .ok()
        .filter(|value| !value.trim().is_empty())
    else {
        eprintln!(
            "skipping live PostgreSQL projection integration test because {POSTGRES_TEST_DATABASE_URL_ENV} is not set"
        );
        return;
    };

    apply_schema(database_url.as_str());

    let stores = PostgresProjectionConfig::new(database_url.as_str())
        .connect_stores()
        .expect("live PostgreSQL projection stores should connect");

    let suffix = unique_suffix();
    let tenant_id = format!("t_proj_{suffix}");
    let conversation_id = format!("c_proj_{suffix}");
    let organization_a = format!("org_a_{suffix}");
    let organization_b = format!("org_b_{suffix}");
    let scope_a = TimelineProjectionScope::new(
        tenant_id.as_str(),
        organization_a.as_str(),
        conversation_id.as_str(),
    )
    .expect("organization A timeline scope should be valid");
    let scope_b = TimelineProjectionScope::new(
        tenant_id.as_str(),
        organization_b.as_str(),
        conversation_id.as_str(),
    )
    .expect("organization B timeline scope should be valid");
    let snapshot_scope = format!("{tenant_id}|default|{conversation_id}");
    let snapshot_key = "conversation-summary";
    let snapshot_payload = r#"{"conversationId":"conv-1","title":"live projection"}"#;
    let timeline_payload_a = r#"{"messageId":"99","messageSeq":1,"summary":"organization A"}"#;
    let timeline_payload_b = r#"{"messageId":"100","messageSeq":1,"summary":"organization B"}"#;

    stores
        .metadata
        .put_snapshot(snapshot_scope.as_str(), snapshot_key, snapshot_payload)
        .expect("metadata snapshot should persist");

    let loaded_snapshot = stores
        .metadata
        .load_snapshot(snapshot_scope.as_str(), snapshot_key)
        .expect("metadata snapshot should load")
        .expect("metadata snapshot should exist");
    assert_eq!(loaded_snapshot, snapshot_payload);

    stores
        .timeline
        .upsert_timeline_entry(&scope_a, 1, timeline_payload_a)
        .expect("organization A timeline entry should persist");

    stores
        .timeline
        .upsert_timeline_entry(&scope_b, 1, timeline_payload_b)
        .expect("organization B timeline entry should persist");

    let loaded_timeline_a = stores
        .timeline
        .load_timeline(&scope_a)
        .expect("organization A timeline should load");
    assert_eq!(loaded_timeline_a, vec![(1, timeline_payload_a.to_owned())]);
    let loaded_timeline_b = stores
        .timeline
        .load_timeline(&scope_b)
        .expect("organization B timeline should load");
    assert_eq!(loaded_timeline_b, vec![(1, timeline_payload_b.to_owned())]);

    stores
        .timeline
        .upsert_timeline_entries(
            &scope_a,
            &[TimelineProjectionRecord {
                message_seq: 2,
                payload: r#"{"messageId":"100","messageSeq":2,"summary":"batch"}"#.into(),
            }],
        )
        .expect("timeline batch should persist");

    drop(stores);
    let restarted_stores = PostgresProjectionConfig::new(database_url.as_str())
        .connect_stores()
        .expect("restarted PostgreSQL projection stores should connect");
    let loaded_timeline_a = restarted_stores
        .timeline
        .load_timeline(&scope_a)
        .expect("organization A timeline should load after restart");
    assert_eq!(loaded_timeline_a.len(), 2);
    assert_eq!(loaded_timeline_a[0].1, timeline_payload_a);
    let loaded_timeline_b = restarted_stores
        .timeline
        .load_timeline(&scope_b)
        .expect("organization B timeline should remain isolated after restart");
    assert_eq!(loaded_timeline_b, vec![(1, timeline_payload_b.to_owned())]);
}

#[test]
fn test_postgres_projection_timeline_scope_rejects_missing_organization() {
    assert!(TimelineProjectionScope::new("tenant", "", "conversation").is_err());
}

fn apply_schema(database_url: &str) {
    let mut client = Client::connect(database_url, NoTls).expect("postgres client should connect");
    client
        .batch_execute(CORE_SCHEMA_SQL)
        .expect("core schema should apply");
}

fn unique_suffix() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_nanos().to_string())
        .unwrap_or_else(|_| "0".into())
}
