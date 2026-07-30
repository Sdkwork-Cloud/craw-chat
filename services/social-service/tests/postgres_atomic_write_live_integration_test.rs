use std::time::{SystemTime, UNIX_EPOCH};

use im_adapters_postgres_journal::PostgresJournalConfig;
use im_adapters_social_postgres::{
    materialize_commits_on_transaction, wire_id::parse_social_entity_id,
};
use im_platform_contracts::{CommitEnvelope, ContractError};
use postgres::{Client, NoTls};

const POSTGRES_TEST_DATABASE_URL_ENV: &str = "SDKWORK_DATABASE_URL";
const CORE_SCHEMA_SQL: &str =
    include_str!("../../../database/ddl/baseline/postgres/0001_im_baseline.sql");

#[test]
fn social_journal_and_normalized_row_roll_back_together_when_write_fails() {
    let Some(database_url) = std::env::var(POSTGRES_TEST_DATABASE_URL_ENV)
        .ok()
        .filter(|value| !value.trim().is_empty())
    else {
        eprintln!(
            "skipping live social PostgreSQL atomicity test because {POSTGRES_TEST_DATABASE_URL_ENV} is not set"
        );
        return;
    };

    let mut schema_client = Client::connect(database_url.as_str(), NoTls)
        .expect("live PostgreSQL schema connection should succeed");
    schema_client
        .batch_execute(CORE_SCHEMA_SQL)
        .expect("IM PostgreSQL baseline should apply idempotently");

    let journal = PostgresJournalConfig::new(database_url)
        .connect()
        .expect("social journal should connect to live PostgreSQL");
    let suffix = unique_suffix();
    let tenant_id = format!("tenant_social_atomic_{suffix}");
    let request_id = i64::try_from(suffix)
        .expect("current epoch nanoseconds should fit a signed int64")
        .to_string();
    let event_id = format!("evt_friend_request_atomic_{suffix}");
    let request_db_id = parse_social_entity_id(request_id.as_str())
        .expect("generated request id should be canonical");
    let commit =
        friend_request_submitted_commit(event_id.as_str(), tenant_id.as_str(), request_id.as_str());

    let rollback_result = journal.append_batch_with_allocated_sequences_in_transaction(
        vec![commit.clone()],
        |txn, inserted| {
            materialize_commits_on_transaction(txn, inserted)?;
            let materialized: i64 = txn
                .query_one(
                    "select count(*) from im_friend_requests where tenant_id = $1 and organization_id = '0' and request_id = $2",
                    &[&tenant_id, &request_db_id],
                )
                .expect("materialized row should be visible inside the transaction")
                .get(0);
            assert_eq!(materialized, 1);
            Err(ContractError::Unavailable(
                "injected failure after social materialization".into(),
            ))
        },
    );
    assert!(matches!(
        rollback_result,
        Err(ContractError::Unavailable(_))
    ));

    let mut verification = journal
        .pool()
        .get()
        .expect("verification connection should be available");
    let rolled_back_row = verification
        .query_one(
            "select \
                (select count(*) from im_commit_journal where event_id = $1), \
                (select count(*) from im_friend_requests where tenant_id = $2 and organization_id = '0' and request_id = $3)",
            &[&event_id, &tenant_id, &request_db_id],
        )
        .expect("rolled-back rows should be countable");
    let rolled_back = (rolled_back_row.get(0), rolled_back_row.get(1));
    assert_eq!(rolled_back, (0, 0));

    journal
        .append_batch_with_allocated_sequences_in_transaction(vec![commit], |txn, inserted| {
            materialize_commits_on_transaction(txn, inserted)
        })
        .expect("social journal and normalized row should commit together");
    let committed_row = verification
        .query_one(
            "select \
                (select count(*) from im_commit_journal where event_id = $1), \
                (select count(*) from im_friend_requests where tenant_id = $2 and organization_id = '0' and request_id = $3)",
            &[&event_id, &tenant_id, &request_db_id],
        )
        .expect("committed rows should be countable");
    let committed = (committed_row.get(0), committed_row.get(1));
    assert_eq!(committed, (1, 1));

    verification
        .execute(
            "delete from im_friend_requests where tenant_id = $1 and organization_id = '0' and request_id = $2",
            &[&tenant_id, &request_db_id],
        )
        .expect("social atomicity normalized row should be cleaned up");
    verification
        .execute(
            "delete from im_commit_journal where event_id = $1",
            &[&event_id],
        )
        .expect("social atomicity journal row should be cleaned up");
}

fn friend_request_submitted_commit(
    event_id: &str,
    tenant_id: &str,
    request_id: &str,
) -> CommitEnvelope {
    CommitEnvelope::minimal(
        event_id,
        tenant_id,
        "friend_request.submitted",
        "friend_request",
        request_id,
        17,
    )
    .with_payload(
        "social.friend_request.submitted.v1",
        &serde_json::json!({
            "requestId": request_id,
            "requesterUserId": format!("requester_{request_id}"),
            "targetUserId": format!("target_{request_id}"),
            "requestMessage": "atomicity test",
            "requestedAt": "2026-07-16T00:00:00.000Z"
        })
        .to_string(),
    )
}

fn unique_suffix() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock should be after unix epoch")
        .as_nanos()
}
