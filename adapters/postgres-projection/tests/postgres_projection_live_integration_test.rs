use im_adapters_postgres_projection::PostgresProjectionConfig;
use im_platform_contracts::{MetadataStore, TimelineProjectionStore};
use r2d2_postgres::postgres::{Client, NoTls};
use sdkwork_im_contract_message::TimelineProjectionRecord;

const POSTGRES_TEST_DATABASE_URL_ENV: &str = "SDKWORK_IM_POSTGRES_TEST_DATABASE_URL";
const CORE_SCHEMA_SQL: &str = include_str!(
    "../../../database/ddl/baseline/postgres/0001_im_baseline.sql"
);

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

    let stores = PostgresProjectionConfig::new(database_url)
        .connect_stores()
        .expect("live PostgreSQL projection stores should connect");

    let suffix = unique_suffix();
    let tenant_id = format!("t_proj_{suffix}");
    let conversation_id = format!("c_proj_{suffix}");
    let snapshot_scope = format!("{tenant_id}|default|{conversation_id}");
    let snapshot_key = "conversation-summary";
    let snapshot_payload = r#"{"conversationId":"conv-1","title":"live projection"}"#;
    let timeline_payload = r#"{"messageId":"99","messageSeq":1,"summary":"hello from live projection"}"#;

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

    let scopes = stores
        .metadata
        .list_scopes_for_snapshot_key(snapshot_key)
        .expect("metadata scopes should list");
    assert!(scopes.iter().any(|scope| scope == &snapshot_scope));

    stores
        .timeline
        .upsert_timeline_entry(
            tenant_id.as_str(),
            conversation_id.as_str(),
            1,
            timeline_payload,
        )
        .expect("timeline entry should persist");

    let loaded_timeline = stores
        .timeline
        .load_timeline(tenant_id.as_str(), conversation_id.as_str())
        .expect("timeline should load");
    assert_eq!(loaded_timeline.len(), 1);
    assert_eq!(loaded_timeline[0].0, 1);
    assert_eq!(loaded_timeline[0].1, timeline_payload);

    stores
        .timeline
        .upsert_timeline_entries(
            tenant_id.as_str(),
            conversation_id.as_str(),
            &[TimelineProjectionRecord {
                message_seq: 2,
                payload: r#"{"messageId":"100","messageSeq":2,"summary":"batch"}"#.into(),
            }],
        )
        .expect("timeline batch should persist");

    let loaded_timeline = stores
        .timeline
        .load_timeline(tenant_id.as_str(), conversation_id.as_str())
        .expect("timeline should load after batch");
    assert_eq!(loaded_timeline.len(), 2);
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
