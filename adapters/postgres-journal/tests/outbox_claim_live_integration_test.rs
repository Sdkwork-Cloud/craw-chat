//! Live PostgreSQL coverage for domain-scoped, leased outbox claims.
//!
//! Run with `SDKWORK_IM_DATABASE_URL=postgresql://... cargo test -p
//! im-adapters-postgres-journal --test outbox_claim_live_integration_test --
//! --ignored --nocapture`.

use std::sync::{Arc, Barrier};
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use im_adapters_postgres_journal::{PostgresJournalConfig, PostgresOutboxStore};
use im_platform_contracts::{OutboxEventRecord, OutboxPublishStatus, OutboxStore};

fn test_suffix() -> String {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time must be after the Unix epoch")
        .as_nanos()
        .to_string()
}

fn event(
    tenant_id: &str,
    organization_id: &str,
    suffix: &str,
    aggregate_type: &str,
) -> OutboxEventRecord {
    let now = chrono::Utc::now().to_rfc3339();
    OutboxEventRecord {
        tenant_id: tenant_id.to_owned(),
        organization_id: organization_id.to_owned(),
        outbox_id: format!("outbox_{suffix}_{aggregate_type}"),
        aggregate_type: aggregate_type.to_owned(),
        aggregate_id: format!("aggregate_{suffix}_{aggregate_type}"),
        event_id: format!("event_{suffix}_{aggregate_type}"),
        event_type: format!("{aggregate_type}.changed"),
        payload_json: "{}".to_owned(),
        payload_hash: format!("hash_{suffix}_{aggregate_type}"),
        publish_status: OutboxPublishStatus::Pending,
        attempt_count: 0,
        available_at: now.clone(),
        published_at: None,
        created_at: now.clone(),
        updated_at: now,
    }
}

#[tokio::test(flavor = "multi_thread")]
#[ignore = "requires live PostgreSQL via SDKWORK_IM_DATABASE_URL"]
async fn outbox_claims_are_domain_scoped_exclusive_and_fenced() {
    let database_url = std::env::var("SDKWORK_IM_DATABASE_URL")
        .expect("SDKWORK_IM_DATABASE_URL must be set for live integration test");
    sdkwork_im_database_pool::bootstrap_im_process_database_pools_from_env()
        .await
        .expect("shared IM database pools should bootstrap");
    let pool = PostgresJournalConfig::new(database_url)
        .connect_pool()
        .expect("postgres journal pool should connect");
    let store = PostgresOutboxStore::from_pool(pool.clone());
    let suffix = test_suffix();
    let tenant_id = format!("outbox-test-{suffix}");
    let organization_id = "0";

    store
        .enqueue(event(
            tenant_id.as_str(),
            organization_id,
            suffix.as_str(),
            "conversation",
        ))
        .expect("conversation event should enqueue");
    store
        .enqueue(event(
            tenant_id.as_str(),
            organization_id,
            suffix.as_str(),
            "rtc_session",
        ))
        .expect("rtc event should enqueue");

    let barrier = Arc::new(Barrier::new(3));
    let mut workers = Vec::new();
    for _ in 0..2 {
        let worker_store = store.clone();
        let worker_barrier = barrier.clone();
        let worker_tenant_id = tenant_id.clone();
        workers.push(thread::spawn(move || {
            worker_barrier.wait();
            worker_store.claim_pending(
                worker_tenant_id.as_str(),
                organization_id,
                "conversation",
                1,
                Duration::from_secs(30),
            )
        }));
    }
    barrier.wait();
    let conversation_claims = workers
        .into_iter()
        .flat_map(|worker| {
            worker
                .join()
                .expect("claim worker should not panic")
                .expect("claim query should succeed")
        })
        .collect::<Vec<_>>();

    let rtc_claims = store
        .claim_pending(
            tenant_id.as_str(),
            organization_id,
            "rtc_session",
            1,
            Duration::from_secs(30),
        )
        .expect("rtc claim should succeed");

    store
        .enqueue(event(
            tenant_id.as_str(),
            organization_id,
            format!("{suffix}_fence").as_str(),
            "social",
        ))
        .expect("fencing event should enqueue");
    let stale_claim = store
        .claim_pending(
            tenant_id.as_str(),
            organization_id,
            "social",
            1,
            Duration::from_millis(5),
        )
        .expect("initial social claim should succeed")
        .pop()
        .expect("initial social claim should return the event");
    thread::sleep(Duration::from_millis(50));
    let current_claim = store
        .claim_pending(
            tenant_id.as_str(),
            organization_id,
            "social",
            1,
            Duration::from_secs(30),
        )
        .expect("replacement social claim should succeed")
        .pop()
        .expect("replacement social claim should return the event");
    let stale_transition = store.mark_published(&stale_claim);
    let current_transition = store.mark_published(&current_claim);

    let cleanup_pool = pool.clone();
    let cleanup_tenant_id = tenant_id.clone();
    tokio::task::spawn_blocking(move || {
        cleanup_pool
            .get()
            .expect("cleanup connection should be available")
            .execute(
                "delete from im_outbox_events where tenant_id = $1 and organization_id = $2",
                &[&cleanup_tenant_id, &organization_id],
            )
            .expect("test outbox rows should be cleaned up");
    })
    .await
    .expect("cleanup task should not panic");

    assert_eq!(conversation_claims.len(), 1);
    assert_eq!(conversation_claims[0].event.aggregate_type, "conversation");
    assert_eq!(rtc_claims.len(), 1);
    assert_eq!(rtc_claims[0].event.aggregate_type, "rtc_session");
    assert!(stale_transition.is_err());
    assert!(current_transition.is_ok());
}
