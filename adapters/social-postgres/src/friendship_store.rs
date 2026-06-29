//! PostgreSQL store for friendships.

use std::sync::Arc;

use im_domain_core::social::{Friendship, FriendshipStatus};
use im_platform_contracts::ContractError;
use r2d2::Pool;

use crate::{SocialPostgresConnectionManager, postgres_pool_client, postgres_unavailable, run_postgres_io};

/// Friendship record for database storage.
#[derive(Clone, Debug)]
pub struct FriendshipRecord {
    pub tenant_id: String,
    pub organization_id: String,
    pub friendship_id: i64,
    pub user_low_id: String,
    pub user_high_id: String,
    pub initiator_user_id: String,
    pub status: String,
    pub established_at: Option<String>,
    pub updated_at: String,
}

impl FriendshipRecord {
    pub fn from_domain(fs: &Friendship, organization_id: &str) -> Self {
        Self {
            tenant_id: fs.tenant_id.clone(),
            organization_id: organization_id.to_string(),
            friendship_id: fs.friendship_id.parse().unwrap_or(0),
            user_low_id: fs.user_low_id.clone(),
            user_high_id: fs.user_high_id.clone(),
            initiator_user_id: fs.initiator_user_id.clone(),
            status: friendship_status_to_str(&fs.status).to_string(),
            established_at: fs.established_at.clone(),
            updated_at: fs.updated_at.clone(),
        }
    }
}

fn friendship_status_to_str(status: &FriendshipStatus) -> &'static str {
    match status {
        FriendshipStatus::Active => "active",
        FriendshipStatus::Removed => "removed",
    }
}

/// Trait for friendship persistence.
pub trait FriendshipStore: Send + Sync {
    fn insert(&self, record: &FriendshipRecord) -> Result<(), ContractError>;
    fn get_by_id(
        &self,
        tenant_id: &str,
        org_id: &str,
        friendship_id: i64,
    ) -> Result<Option<FriendshipRecord>, ContractError>;
    fn find_by_pair(
        &self,
        tenant_id: &str,
        org_id: &str,
        user_low_id: &str,
        user_high_id: &str,
    ) -> Result<Option<FriendshipRecord>, ContractError>;
    fn list_by_user(
        &self,
        tenant_id: &str,
        org_id: &str,
        user_id: &str,
        status: &str,
        limit: i64,
    ) -> Result<Vec<FriendshipRecord>, ContractError>;
    fn update_status(
        &self,
        tenant_id: &str,
        org_id: &str,
        friendship_id: i64,
        status: &str,
        updated_at: &str,
    ) -> Result<(), ContractError>;
}

const INSERT_SQL: &str = r#"
INSERT INTO im_friendships (
    tenant_id, organization_id, friendship_id, user_low_id, user_high_id,
    initiator_user_id, status, established_at, updated_at
) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
ON CONFLICT (tenant_id, organization_id, friendship_id) DO NOTHING
"#;

const GET_BY_ID_SQL: &str = r#"
SELECT tenant_id, organization_id, friendship_id, user_low_id, user_high_id,
       initiator_user_id, status, established_at, updated_at
FROM im_friendships
WHERE tenant_id = $1 AND organization_id = $2 AND friendship_id = $3
"#;

const FIND_BY_PAIR_SQL: &str = r#"
SELECT tenant_id, organization_id, friendship_id, user_low_id, user_high_id,
       initiator_user_id, status, established_at, updated_at
FROM im_friendships
WHERE tenant_id = $1 AND organization_id = $2 AND user_low_id = $3 AND user_high_id = $4
LIMIT 1
"#;

const LIST_BY_USER_SQL: &str = r#"
SELECT tenant_id, organization_id, friendship_id, user_low_id, user_high_id,
       initiator_user_id, status, established_at, updated_at
FROM im_friendships
WHERE tenant_id = $1 AND organization_id = $2
  AND (user_low_id = $3 OR user_high_id = $3)
  AND status = $4
ORDER BY established_at DESC
LIMIT $5
"#;

const UPDATE_STATUS_SQL: &str = r#"
UPDATE im_friendships
SET status = $4, updated_at = $5
WHERE tenant_id = $1 AND organization_id = $2 AND friendship_id = $3
"#;

fn row_to_record(row: &postgres::Row) -> FriendshipRecord {
    FriendshipRecord {
        tenant_id: row.get("tenant_id"),
        organization_id: row.get("organization_id"),
        friendship_id: row.get("friendship_id"),
        user_low_id: row.get("user_low_id"),
        user_high_id: row.get("user_high_id"),
        initiator_user_id: row.get("initiator_user_id"),
        status: row.get("status"),
        established_at: row.get("established_at"),
        updated_at: row.get("updated_at"),
    }
}

/// PostgreSQL-backed friendship store.
#[derive(Clone)]
pub struct PostgresFriendshipStore {
    pool: Arc<Pool<SocialPostgresConnectionManager>>,
}

impl PostgresFriendshipStore {
    pub fn new(pool: Arc<Pool<SocialPostgresConnectionManager>>) -> Self {
        Self { pool }
    }
}

impl FriendshipStore for PostgresFriendshipStore {
    fn insert(&self, record: &FriendshipRecord) -> Result<(), ContractError> {
        let pool = self.pool.clone();
        let r = record.clone();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "insert_friendship")?;
            client
                .execute(
                    INSERT_SQL,
                    &[
                        &r.tenant_id,
                        &r.organization_id,
                        &r.friendship_id,
                        &r.user_low_id,
                        &r.user_high_id,
                        &r.initiator_user_id,
                        &r.status,
                        &r.established_at,
                        &r.updated_at,
                    ],
                )
                .map_err(|e| postgres_unavailable("insert_friendship", e))?;
            Ok(())
        })
    }

    fn get_by_id(
        &self,
        tenant_id: &str,
        org_id: &str,
        friendship_id: i64,
    ) -> Result<Option<FriendshipRecord>, ContractError> {
        let pool = self.pool.clone();
        let tid = tenant_id.to_string();
        let oid = org_id.to_string();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "get_friendship")?;
            let row = client
                .query_opt(GET_BY_ID_SQL, &[&tid, &oid, &friendship_id])
                .map_err(|e| postgres_unavailable("get_friendship", e))?;
            Ok(row.map(|r| row_to_record(&r)))
        })
    }

    fn find_by_pair(
        &self,
        tenant_id: &str,
        org_id: &str,
        user_low_id: &str,
        user_high_id: &str,
    ) -> Result<Option<FriendshipRecord>, ContractError> {
        let pool = self.pool.clone();
        let tid = tenant_id.to_string();
        let oid = org_id.to_string();
        let low = user_low_id.to_string();
        let high = user_high_id.to_string();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "find_friendship_by_pair")?;
            let row = client
                .query_opt(FIND_BY_PAIR_SQL, &[&tid, &oid, &low, &high])
                .map_err(|e| postgres_unavailable("find_friendship_by_pair", e))?;
            Ok(row.map(|r| row_to_record(&r)))
        })
    }

    fn list_by_user(
        &self,
        tenant_id: &str,
        org_id: &str,
        user_id: &str,
        status: &str,
        limit: i64,
    ) -> Result<Vec<FriendshipRecord>, ContractError> {
        let pool = self.pool.clone();
        let tid = tenant_id.to_string();
        let oid = org_id.to_string();
        let uid = user_id.to_string();
        let st = status.to_string();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "list_friendships_by_user")?;
            let rows = client
                .query(LIST_BY_USER_SQL, &[&tid, &oid, &uid, &st, &limit])
                .map_err(|e| postgres_unavailable("list_friendships_by_user", e))?;
            Ok(rows.iter().map(row_to_record).collect())
        })
    }

    fn update_status(
        &self,
        tenant_id: &str,
        org_id: &str,
        friendship_id: i64,
        status: &str,
        updated_at: &str,
    ) -> Result<(), ContractError> {
        let pool = self.pool.clone();
        let tid = tenant_id.to_string();
        let oid = org_id.to_string();
        let st = status.to_string();
        let ua = updated_at.to_string();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "update_friendship_status")?;
            client
                .execute(UPDATE_STATUS_SQL, &[&tid, &oid, &friendship_id, &st, &ua])
                .map_err(|e| postgres_unavailable("update_friendship_status", e))?;
            Ok(())
        })
    }
}
