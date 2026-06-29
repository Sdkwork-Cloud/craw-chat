//! PostgreSQL store for user blocks.

use std::sync::Arc;

use im_domain_core::social::{BlockScope, UserBlock};
use im_platform_contracts::ContractError;
use r2d2::Pool;

use crate::{SocialPostgresConnectionManager, postgres_pool_client, postgres_unavailable, run_postgres_io};

/// User block record for database storage.
#[derive(Clone, Debug)]
pub struct UserBlockRecord {
    pub tenant_id: String,
    pub organization_id: String,
    pub block_id: i64,
    pub blocker_user_id: String,
    pub blocked_user_id: String,
    pub scope: String,
    pub direct_chat_id: Option<i64>,
    pub reason: Option<String>,
    pub expires_at: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl UserBlockRecord {
    pub fn from_domain(ub: &UserBlock, organization_id: &str) -> Self {
        Self {
            tenant_id: ub.tenant_id.clone(),
            organization_id: organization_id.to_string(),
            block_id: ub.block_id.parse().unwrap_or(0),
            blocker_user_id: ub.blocker_user_id.clone(),
            blocked_user_id: ub.blocked_user_id.clone(),
            scope: block_scope_to_str(&ub.scope).to_string(),
            direct_chat_id: ub.direct_chat_id.as_ref().and_then(|s| s.parse().ok()),
            reason: None,
            expires_at: ub.expires_at.clone(),
            created_at: ub.created_at.clone(),
            updated_at: ub.updated_at.clone(),
        }
    }
}

fn block_scope_to_str(scope: &BlockScope) -> &'static str {
    match scope {
        BlockScope::All => "all",
        BlockScope::Friendship => "friendship",
        BlockScope::DirectChat => "direct_chat",
    }
}

/// Trait for user block persistence.
pub trait UserBlockStore: Send + Sync {
    fn insert(&self, record: &UserBlockRecord) -> Result<(), ContractError>;
    fn get_by_id(
        &self,
        tenant_id: &str,
        org_id: &str,
        block_id: i64,
    ) -> Result<Option<UserBlockRecord>, ContractError>;
    fn find_active_block(
        &self,
        tenant_id: &str,
        org_id: &str,
        blocker_id: &str,
        blocked_id: &str,
        scope: &str,
    ) -> Result<Option<UserBlockRecord>, ContractError>;
    fn list_by_blocker(
        &self,
        tenant_id: &str,
        org_id: &str,
        blocker_id: &str,
        limit: i64,
    ) -> Result<Vec<UserBlockRecord>, ContractError>;
    fn list_by_blocked(
        &self,
        tenant_id: &str,
        org_id: &str,
        blocked_id: &str,
        limit: i64,
    ) -> Result<Vec<UserBlockRecord>, ContractError>;
    fn delete_by_blocker(
        &self,
        tenant_id: &str,
        org_id: &str,
        block_id: i64,
        blocker_user_id: &str,
    ) -> Result<bool, ContractError>;
}

const INSERT_SQL: &str = r#"
INSERT INTO im_user_blocks (
    tenant_id, organization_id, block_id, blocker_user_id, blocked_user_id,
    scope, direct_chat_id, reason, expires_at, created_at, updated_at
) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
ON CONFLICT (tenant_id, organization_id, block_id) DO NOTHING
"#;

const GET_BY_ID_SQL: &str = r#"
SELECT tenant_id, organization_id, block_id, blocker_user_id, blocked_user_id,
       scope, direct_chat_id, reason, expires_at, created_at, updated_at
FROM im_user_blocks
WHERE tenant_id = $1 AND organization_id = $2 AND block_id = $3
"#;

const FIND_ACTIVE_BLOCK_SQL: &str = r#"
SELECT tenant_id, organization_id, block_id, blocker_user_id, blocked_user_id,
       scope, direct_chat_id, reason, expires_at, created_at, updated_at
FROM im_user_blocks
WHERE tenant_id = $1 AND organization_id = $2
  AND blocker_user_id = $3 AND blocked_user_id = $4 AND scope = $5
  AND (expires_at IS NULL OR expires_at > NOW())
LIMIT 1
"#;

const LIST_BY_BLOCKER_SQL: &str = r#"
SELECT tenant_id, organization_id, block_id, blocker_user_id, blocked_user_id,
       scope, direct_chat_id, reason, expires_at, created_at, updated_at
FROM im_user_blocks
WHERE tenant_id = $1 AND organization_id = $2 AND blocker_user_id = $3
ORDER BY created_at DESC
LIMIT $4
"#;

const LIST_BY_BLOCKED_SQL: &str = r#"
SELECT tenant_id, organization_id, block_id, blocker_user_id, blocked_user_id,
       scope, direct_chat_id, reason, expires_at, created_at, updated_at
FROM im_user_blocks
WHERE tenant_id = $1 AND organization_id = $2 AND blocked_user_id = $3
ORDER BY created_at DESC
LIMIT $4
"#;

const DELETE_BY_BLOCKER_SQL: &str = r#"
DELETE FROM im_user_blocks
WHERE tenant_id = $1 AND organization_id = $2 AND block_id = $3 AND blocker_user_id = $4
"#;

fn row_to_record(row: &postgres::Row) -> UserBlockRecord {
    UserBlockRecord {
        tenant_id: row.get("tenant_id"),
        organization_id: row.get("organization_id"),
        block_id: row.get("block_id"),
        blocker_user_id: row.get("blocker_user_id"),
        blocked_user_id: row.get("blocked_user_id"),
        scope: row.get("scope"),
        direct_chat_id: row.get("direct_chat_id"),
        reason: row.get("reason"),
        expires_at: row.get("expires_at"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    }
}

/// PostgreSQL-backed user block store.
#[derive(Clone)]
pub struct PostgresUserBlockStore {
    pool: Arc<Pool<SocialPostgresConnectionManager>>,
}

impl PostgresUserBlockStore {
    pub fn new(pool: Arc<Pool<SocialPostgresConnectionManager>>) -> Self {
        Self { pool }
    }
}

impl UserBlockStore for PostgresUserBlockStore {
    fn insert(&self, record: &UserBlockRecord) -> Result<(), ContractError> {
        let pool = self.pool.clone();
        let r = record.clone();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "insert_user_block")?;
            client
                .execute(
                    INSERT_SQL,
                    &[
                        &r.tenant_id,
                        &r.organization_id,
                        &r.block_id,
                        &r.blocker_user_id,
                        &r.blocked_user_id,
                        &r.scope,
                        &r.direct_chat_id,
                        &r.reason,
                        &r.expires_at,
                        &r.created_at,
                        &r.updated_at,
                    ],
                )
                .map_err(|e| postgres_unavailable("insert_user_block", e))?;
            Ok(())
        })
    }

    fn get_by_id(
        &self,
        tenant_id: &str,
        org_id: &str,
        block_id: i64,
    ) -> Result<Option<UserBlockRecord>, ContractError> {
        let pool = self.pool.clone();
        let tid = tenant_id.to_string();
        let oid = org_id.to_string();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "get_user_block")?;
            let row = client
                .query_opt(GET_BY_ID_SQL, &[&tid, &oid, &block_id])
                .map_err(|e| postgres_unavailable("get_user_block", e))?;
            Ok(row.map(|r| row_to_record(&r)))
        })
    }

    fn find_active_block(
        &self,
        tenant_id: &str,
        org_id: &str,
        blocker_id: &str,
        blocked_id: &str,
        scope: &str,
    ) -> Result<Option<UserBlockRecord>, ContractError> {
        let pool = self.pool.clone();
        let tid = tenant_id.to_string();
        let oid = org_id.to_string();
        let bid = blocker_id.to_string();
        let tid2 = blocked_id.to_string();
        let sc = scope.to_string();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "find_active_user_block")?;
            let row = client
                .query_opt(FIND_ACTIVE_BLOCK_SQL, &[&tid, &oid, &bid, &tid2, &sc])
                .map_err(|e| postgres_unavailable("find_active_user_block", e))?;
            Ok(row.map(|r| row_to_record(&r)))
        })
    }

    fn list_by_blocker(
        &self,
        tenant_id: &str,
        org_id: &str,
        blocker_id: &str,
        limit: i64,
    ) -> Result<Vec<UserBlockRecord>, ContractError> {
        let pool = self.pool.clone();
        let tid = tenant_id.to_string();
        let oid = org_id.to_string();
        let bid = blocker_id.to_string();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "list_user_blocks_by_blocker")?;
            let rows = client
                .query(LIST_BY_BLOCKER_SQL, &[&tid, &oid, &bid, &limit])
                .map_err(|e| postgres_unavailable("list_user_blocks_by_blocker", e))?;
            Ok(rows.iter().map(row_to_record).collect())
        })
    }

    fn list_by_blocked(
        &self,
        tenant_id: &str,
        org_id: &str,
        blocked_id: &str,
        limit: i64,
    ) -> Result<Vec<UserBlockRecord>, ContractError> {
        let pool = self.pool.clone();
        let tid = tenant_id.to_string();
        let oid = org_id.to_string();
        let bid = blocked_id.to_string();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "list_user_blocks_by_blocked")?;
            let rows = client
                .query(LIST_BY_BLOCKED_SQL, &[&tid, &oid, &bid, &limit])
                .map_err(|e| postgres_unavailable("list_user_blocks_by_blocked", e))?;
            Ok(rows.iter().map(row_to_record).collect())
        })
    }

    fn delete_by_blocker(
        &self,
        tenant_id: &str,
        org_id: &str,
        block_id: i64,
        blocker_user_id: &str,
    ) -> Result<bool, ContractError> {
        let pool = self.pool.clone();
        let tid = tenant_id.to_string();
        let oid = org_id.to_string();
        let bid = blocker_user_id.to_string();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "delete_user_block_by_blocker")?;
            let deleted = client
                .execute(DELETE_BY_BLOCKER_SQL, &[&tid, &oid, &block_id, &bid])
                .map_err(|e| postgres_unavailable("delete_user_block_by_blocker", e))?;
            Ok(deleted > 0)
        })
    }
}
