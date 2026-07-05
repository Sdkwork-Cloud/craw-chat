//! PostgreSQL store for friend requests.

use std::sync::Arc;

use im_domain_core::social::{FriendRequest, FriendRequestStatus};
use im_platform_contracts::ContractError;
use r2d2::Pool;

use crate::{SocialPostgresConnectionManager, postgres_pool_client, postgres_unavailable, run_postgres_io};

/// Friend request record for database storage.
#[derive(Clone, Debug)]
pub struct FriendRequestRecord {
    pub tenant_id: String,
    pub organization_id: String,
    pub request_id: i64,
    pub requester_user_id: String,
    pub target_user_id: String,
    pub request_message: Option<String>,
    pub status: String,
    pub expired_at: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl FriendRequestRecord {
    pub fn from_domain(fr: &FriendRequest, organization_id: &str) -> Self {
        Self {
            tenant_id: fr.tenant_id.clone(),
            organization_id: organization_id.to_string(),
            request_id: fr.request_id.parse().unwrap_or(0),
            requester_user_id: fr.requester_user_id.clone(),
            target_user_id: fr.target_user_id.clone(),
            request_message: fr.request_message.clone(),
            status: friend_request_status_to_str(&fr.status).to_string(),
            expired_at: fr.expired_at.clone(),
            created_at: fr.created_at.clone(),
            updated_at: fr.updated_at.clone(),
        }
    }
}

fn friend_request_status_to_str(status: &FriendRequestStatus) -> &'static str {
    match status {
        FriendRequestStatus::Pending => "pending",
        FriendRequestStatus::Accepted => "accepted",
        FriendRequestStatus::Declined => "declined",
        FriendRequestStatus::Canceled => "canceled",
        FriendRequestStatus::Expired => "expired",
    }
}

/// Trait for friend request persistence.
pub trait FriendRequestStore: Send + Sync {
    fn insert(&self, record: &FriendRequestRecord) -> Result<(), ContractError>;
    fn get_by_id(
        &self,
        tenant_id: &str,
        org_id: &str,
        request_id: i64,
    ) -> Result<Option<FriendRequestRecord>, ContractError>;
    fn list_by_requester(
        &self,
        tenant_id: &str,
        org_id: &str,
        requester_id: &str,
        status: &str,
        limit: i64,
    ) -> Result<Vec<FriendRequestRecord>, ContractError>;
    fn list_by_target(
        &self,
        tenant_id: &str,
        org_id: &str,
        target_id: &str,
        status: &str,
        limit: i64,
    ) -> Result<Vec<FriendRequestRecord>, ContractError>;
    fn update_status(
        &self,
        tenant_id: &str,
        org_id: &str,
        request_id: i64,
        status: &str,
        updated_at: &str,
    ) -> Result<(), ContractError>;
    /// Removes a friend request row (journal-append compensation for submitted events).
    fn delete_by_id(
        &self,
        tenant_id: &str,
        org_id: &str,
        request_id: i64,
    ) -> Result<(), ContractError>;
    /// Unconditional status update used only when rolling back supplemental writes.
    fn revert_status_for_compensation(
        &self,
        tenant_id: &str,
        org_id: &str,
        request_id: i64,
        status: &str,
        updated_at: &str,
    ) -> Result<(), ContractError>;
    fn find_by_pair_and_status(
        &self,
        tenant_id: &str,
        org_id: &str,
        requester_id: &str,
        target_id: &str,
        status: &str,
    ) -> Result<Option<FriendRequestRecord>, ContractError>;
    fn count_by_requester_created_between(
        &self,
        tenant_id: &str,
        org_id: &str,
        requester_id: &str,
        start_inclusive: &str,
        end_exclusive: &str,
    ) -> Result<i64, ContractError>;
    /// Keyset inventory page ordered by `updated_at DESC`, `created_at DESC`, `request_id ASC`.
    fn list_inventory(
        &self,
        tenant_id: &str,
        org_id: &str,
        user_id: &str,
        direction: &str,
        status: Option<&str>,
        cursor_updated_at: Option<&str>,
        cursor_created_at: Option<&str>,
        cursor_request_id: Option<i64>,
        limit: i64,
    ) -> Result<Vec<FriendRequestRecord>, ContractError>;
    fn count_pending_incoming_by_target(
        &self,
        tenant_id: &str,
        org_id: &str,
        target_user_id: &str,
    ) -> Result<i64, ContractError>;
}

const COUNT_PENDING_INCOMING_SQL: &str = r#"
SELECT COUNT(*)::bigint
FROM im_friend_requests
WHERE tenant_id = $1
  AND organization_id = $2
  AND target_user_id = $3
  AND status = 'pending'
"#;

const INSERT_SQL: &str = r#"
INSERT INTO im_friend_requests (
    tenant_id, organization_id, request_id, requester_user_id, target_user_id,
    request_message, status, expired_at, created_at, updated_at
) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
ON CONFLICT (tenant_id, organization_id, request_id) DO NOTHING
"#;

const GET_BY_ID_SQL: &str = r#"
SELECT tenant_id, organization_id, request_id, requester_user_id, target_user_id,
       request_message, status, expired_at, created_at, updated_at
FROM im_friend_requests
WHERE tenant_id = $1 AND organization_id = $2 AND request_id = $3
"#;

const LIST_BY_REQUESTER_SQL: &str = r#"
SELECT tenant_id, organization_id, request_id, requester_user_id, target_user_id,
       request_message, status, expired_at, created_at, updated_at
FROM im_friend_requests
WHERE tenant_id = $1 AND organization_id = $2 AND requester_user_id = $3 AND status = $4
ORDER BY created_at DESC
LIMIT $5
"#;

const LIST_BY_TARGET_SQL: &str = r#"
SELECT tenant_id, organization_id, request_id, requester_user_id, target_user_id,
       request_message, status, expired_at, created_at, updated_at
FROM im_friend_requests
WHERE tenant_id = $1 AND organization_id = $2 AND target_user_id = $3 AND status = $4
ORDER BY created_at DESC
LIMIT $5
"#;

const UPDATE_STATUS_SQL: &str = r#"
UPDATE im_friend_requests
SET status = $4, updated_at = $5
WHERE tenant_id = $1 AND organization_id = $2 AND request_id = $3 AND status = 'pending'
"#;

const DELETE_BY_ID_SQL: &str = r#"
DELETE FROM im_friend_requests
WHERE tenant_id = $1 AND organization_id = $2 AND request_id = $3
"#;

const REVERT_STATUS_SQL: &str = r#"
UPDATE im_friend_requests
SET status = $4, updated_at = $5
WHERE tenant_id = $1 AND organization_id = $2 AND request_id = $3
"#;

const FIND_BY_PAIR_AND_STATUS_SQL: &str = r#"
SELECT tenant_id, organization_id, request_id, requester_user_id, target_user_id,
       request_message, status, expired_at, created_at, updated_at
FROM im_friend_requests
WHERE tenant_id = $1 AND organization_id = $2 AND requester_user_id = $3 AND target_user_id = $4 AND status = $5
LIMIT 1
"#;

const COUNT_BY_REQUESTER_CREATED_BETWEEN_SQL: &str = r#"
SELECT COUNT(*)::bigint
FROM im_friend_requests
WHERE tenant_id = $1
  AND organization_id = $2
  AND requester_user_id = $3
  AND created_at >= $4
  AND created_at < $5
"#;

const LIST_INVENTORY_INCOMING_SQL: &str = r#"
SELECT tenant_id, organization_id, request_id, requester_user_id, target_user_id,
       request_message, status, expired_at, created_at, updated_at
FROM im_friend_requests
WHERE tenant_id = $1 AND organization_id = $2 AND target_user_id = $3
  AND ($4::text IS NULL OR status = $4)
  AND (
    $5::text IS NULL
    OR updated_at < $5::text
    OR (updated_at = $5::text AND created_at < $6::text)
    OR (updated_at = $5::text AND created_at = $6::text AND request_id > $7)
  )
ORDER BY updated_at DESC, created_at DESC, request_id ASC
LIMIT $8
"#;

const LIST_INVENTORY_OUTGOING_SQL: &str = r#"
SELECT tenant_id, organization_id, request_id, requester_user_id, target_user_id,
       request_message, status, expired_at, created_at, updated_at
FROM im_friend_requests
WHERE tenant_id = $1 AND organization_id = $2 AND requester_user_id = $3
  AND ($4::text IS NULL OR status = $4)
  AND (
    $5::text IS NULL
    OR updated_at < $5::text
    OR (updated_at = $5::text AND created_at < $6::text)
    OR (updated_at = $5::text AND created_at = $6::text AND request_id > $7)
  )
ORDER BY updated_at DESC, created_at DESC, request_id ASC
LIMIT $8
"#;

fn row_to_record(row: &postgres::Row) -> FriendRequestRecord {
    FriendRequestRecord {
        tenant_id: row.get("tenant_id"),
        organization_id: row.get("organization_id"),
        request_id: row.get("request_id"),
        requester_user_id: row.get("requester_user_id"),
        target_user_id: row.get("target_user_id"),
        request_message: row.get("request_message"),
        status: row.get("status"),
        expired_at: row.get("expired_at"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    }
}

/// PostgreSQL-backed friend request store.
#[derive(Clone)]
pub struct PostgresFriendRequestStore {
    pool: Arc<Pool<SocialPostgresConnectionManager>>,
}

impl PostgresFriendRequestStore {
    pub fn new(pool: Arc<Pool<SocialPostgresConnectionManager>>) -> Self {
        Self { pool }
    }
}

impl FriendRequestStore for PostgresFriendRequestStore {
    fn insert(&self, record: &FriendRequestRecord) -> Result<(), ContractError> {
        let pool = self.pool.clone();
        let r = record.clone();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "insert_friend_request")?;
            client
                .execute(
                    INSERT_SQL,
                    &[
                        &r.tenant_id,
                        &r.organization_id,
                        &r.request_id,
                        &r.requester_user_id,
                        &r.target_user_id,
                        &r.request_message,
                        &r.status,
                        &r.expired_at,
                        &r.created_at,
                        &r.updated_at,
                    ],
                )
                .map_err(|e| postgres_unavailable("insert_friend_request", e))?;
            Ok(())
        })
    }

    fn get_by_id(
        &self,
        tenant_id: &str,
        org_id: &str,
        request_id: i64,
    ) -> Result<Option<FriendRequestRecord>, ContractError> {
        let pool = self.pool.clone();
        let tid = tenant_id.to_string();
        let oid = org_id.to_string();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "get_friend_request")?;
            let row = client
                .query_opt(GET_BY_ID_SQL, &[&tid, &oid, &request_id])
                .map_err(|e| postgres_unavailable("get_friend_request", e))?;
            Ok(row.map(|r| row_to_record(&r)))
        })
    }

    fn list_by_requester(
        &self,
        tenant_id: &str,
        org_id: &str,
        requester_id: &str,
        status: &str,
        limit: i64,
    ) -> Result<Vec<FriendRequestRecord>, ContractError> {
        let pool = self.pool.clone();
        let tid = tenant_id.to_string();
        let oid = org_id.to_string();
        let rid = requester_id.to_string();
        let st = status.to_string();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "list_friend_requests_by_requester")?;
            let rows = client
                .query(LIST_BY_REQUESTER_SQL, &[&tid, &oid, &rid, &st, &limit])
                .map_err(|e| postgres_unavailable("list_friend_requests_by_requester", e))?;
            Ok(rows.iter().map(row_to_record).collect())
        })
    }

    fn list_by_target(
        &self,
        tenant_id: &str,
        org_id: &str,
        target_id: &str,
        status: &str,
        limit: i64,
    ) -> Result<Vec<FriendRequestRecord>, ContractError> {
        let pool = self.pool.clone();
        let tid = tenant_id.to_string();
        let oid = org_id.to_string();
        let tid2 = target_id.to_string();
        let st = status.to_string();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "list_friend_requests_by_target")?;
            let rows = client
                .query(LIST_BY_TARGET_SQL, &[&tid, &oid, &tid2, &st, &limit])
                .map_err(|e| postgres_unavailable("list_friend_requests_by_target", e))?;
            Ok(rows.iter().map(row_to_record).collect())
        })
    }

    fn update_status(
        &self,
        tenant_id: &str,
        org_id: &str,
        request_id: i64,
        status: &str,
        updated_at: &str,
    ) -> Result<(), ContractError> {
        let pool = self.pool.clone();
        let tid = tenant_id.to_string();
        let oid = org_id.to_string();
        let st = status.to_string();
        let ua = updated_at.to_string();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "update_friend_request_status")?;
            let updated = client
                .execute(UPDATE_STATUS_SQL, &[&tid, &oid, &request_id, &st, &ua])
                .map_err(|e| postgres_unavailable("update_friend_request_status", e))?;
            if updated == 0 {
                return Err(ContractError::Conflict(
                    "friend request is not pending or does not exist".to_owned(),
                ));
            }
            Ok(())
        })
    }

    fn delete_by_id(
        &self,
        tenant_id: &str,
        org_id: &str,
        request_id: i64,
    ) -> Result<(), ContractError> {
        let pool = self.pool.clone();
        let tid = tenant_id.to_string();
        let oid = org_id.to_string();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "delete_friend_request")?;
            client
                .execute(DELETE_BY_ID_SQL, &[&tid, &oid, &request_id])
                .map_err(|e| postgres_unavailable("delete_friend_request", e))?;
            Ok(())
        })
    }

    fn revert_status_for_compensation(
        &self,
        tenant_id: &str,
        org_id: &str,
        request_id: i64,
        status: &str,
        updated_at: &str,
    ) -> Result<(), ContractError> {
        let pool = self.pool.clone();
        let tid = tenant_id.to_string();
        let oid = org_id.to_string();
        let st = status.to_string();
        let ua = updated_at.to_string();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "revert_friend_request_status")?;
            client
                .execute(
                    REVERT_STATUS_SQL,
                    &[&tid, &oid, &request_id, &st, &ua],
                )
                .map_err(|e| postgres_unavailable("revert_friend_request_status", e))?;
            Ok(())
        })
    }

    fn find_by_pair_and_status(
        &self,
        tenant_id: &str,
        org_id: &str,
        requester_id: &str,
        target_id: &str,
        status: &str,
    ) -> Result<Option<FriendRequestRecord>, ContractError> {
        let pool = self.pool.clone();
        let tid = tenant_id.to_string();
        let oid = org_id.to_string();
        let rid = requester_id.to_string();
        let tid2 = target_id.to_string();
        let st = status.to_string();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "find_friend_request_by_pair")?;
            let row = client
                .query_opt(FIND_BY_PAIR_AND_STATUS_SQL, &[&tid, &oid, &rid, &tid2, &st])
                .map_err(|e| postgres_unavailable("find_friend_request_by_pair", e))?;
            Ok(row.map(|r| row_to_record(&r)))
        })
    }

    fn count_by_requester_created_between(
        &self,
        tenant_id: &str,
        org_id: &str,
        requester_id: &str,
        start_inclusive: &str,
        end_exclusive: &str,
    ) -> Result<i64, ContractError> {
        let pool = self.pool.clone();
        let tid = tenant_id.to_string();
        let oid = org_id.to_string();
        let rid = requester_id.to_string();
        let start = start_inclusive.to_string();
        let end = end_exclusive.to_string();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "count_friend_requests_by_requester")?;
            let row = client
                .query_one(
                    COUNT_BY_REQUESTER_CREATED_BETWEEN_SQL,
                    &[&tid, &oid, &rid, &start, &end],
                )
                .map_err(|e| postgres_unavailable("count_friend_requests_by_requester", e))?;
            Ok(row.get(0))
        })
    }

    fn list_inventory(
        &self,
        tenant_id: &str,
        org_id: &str,
        user_id: &str,
        direction: &str,
        status: Option<&str>,
        cursor_updated_at: Option<&str>,
        cursor_created_at: Option<&str>,
        cursor_request_id: Option<i64>,
        limit: i64,
    ) -> Result<Vec<FriendRequestRecord>, ContractError> {
        let pool = self.pool.clone();
        let tid = tenant_id.to_string();
        let oid = org_id.to_string();
        let uid = user_id.to_string();
        let direction = direction.to_string();
        let status = status.map(str::to_owned);
        let cursor_updated_at = cursor_updated_at.map(str::to_owned);
        let cursor_created_at = cursor_created_at.map(str::to_owned);
        run_postgres_io(move || {
            let sql = match direction.as_str() {
                "incoming" => LIST_INVENTORY_INCOMING_SQL,
                "outgoing" => LIST_INVENTORY_OUTGOING_SQL,
                other => {
                    return Err(ContractError::Invalid(format!(
                        "unsupported friend request inventory direction: {other}"
                    )));
                }
            };
            let mut client = postgres_pool_client(&pool, "list_friend_request_inventory")?;
            let rows = client
                .query(
                    sql,
                    &[
                        &tid,
                        &oid,
                        &uid,
                        &status,
                        &cursor_updated_at,
                        &cursor_created_at,
                        &cursor_request_id,
                        &limit,
                    ],
                )
                .map_err(|error| postgres_unavailable("list_friend_request_inventory", error))?;
            Ok(rows.iter().map(row_to_record).collect())
        })
    }

    fn count_pending_incoming_by_target(
        &self,
        tenant_id: &str,
        org_id: &str,
        target_user_id: &str,
    ) -> Result<i64, ContractError> {
        let pool = self.pool.clone();
        let tid = tenant_id.to_string();
        let oid = org_id.to_string();
        let target = target_user_id.to_string();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "count_pending_friend_requests")?;
            let row = client
                .query_one(
                    COUNT_PENDING_INCOMING_SQL,
                    &[&tid, &oid, &target],
                )
                .map_err(|error| postgres_unavailable("count_pending_friend_requests", error))?;
            Ok(row.get(0))
        })
    }
}
