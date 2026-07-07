//! PostgreSQL store for direct chats.

use std::sync::Arc;

use im_domain_core::social::{DirectChat, DirectChatStatus};
use im_platform_contracts::ContractError;
use r2d2::Pool;

use crate::{
    SocialPostgresConnectionManager, optional_postgres_timestamptz, postgres_pool_client,
    postgres_timestamptz, postgres_unavailable, run_postgres_io,
};

/// Direct chat record for database storage.
#[derive(Clone, Debug)]
pub struct DirectChatRecord {
    pub tenant_id: String,
    pub organization_id: String,
    pub direct_chat_id: i64,
    pub left_actor_kind: String,
    pub left_actor_id: String,
    pub right_actor_kind: String,
    pub right_actor_id: String,
    pub pair_hash: String,
    pub status: String,
    pub conversation_id: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Clone, Copy, Debug)]
pub struct DirectChatActorListQuery<'a> {
    pub tenant_id: &'a str,
    pub organization_id: &'a str,
    pub actor_id: &'a str,
    pub status: &'a str,
    pub cursor_updated_at: Option<&'a str>,
    pub cursor_direct_chat_id: Option<i64>,
    pub limit: i64,
}

impl DirectChatRecord {
    pub fn from_domain(dc: &DirectChat, organization_id: &str) -> Self {
        Self {
            tenant_id: dc.tenant_id.clone(),
            organization_id: organization_id.to_string(),
            direct_chat_id: dc.direct_chat_id.parse().unwrap_or(0),
            left_actor_kind: "user".to_string(),
            left_actor_id: dc.left_actor_id.clone(),
            right_actor_kind: "user".to_string(),
            right_actor_id: dc.right_actor_id.clone(),
            pair_hash: dc.pair_hash.clone(),
            status: direct_chat_status_to_str(&dc.status).to_string(),
            conversation_id: dc.conversation_id.clone(),
            created_at: dc.created_at.clone(),
            updated_at: dc.updated_at.clone(),
        }
    }
}

fn direct_chat_status_to_str(status: &DirectChatStatus) -> &'static str {
    match status {
        DirectChatStatus::Active => "active",
        DirectChatStatus::Archived => "archived",
        DirectChatStatus::Closed => "closed",
    }
}

/// Trait for direct chat persistence.
pub trait DirectChatStore: Send + Sync {
    fn insert(&self, record: &DirectChatRecord) -> Result<(), ContractError>;
    fn get_by_id(
        &self,
        tenant_id: &str,
        org_id: &str,
        direct_chat_id: i64,
    ) -> Result<Option<DirectChatRecord>, ContractError>;
    fn find_by_pair_hash(
        &self,
        tenant_id: &str,
        org_id: &str,
        pair_hash: &str,
    ) -> Result<Option<DirectChatRecord>, ContractError>;
    /// Keyset page ordered by `updated_at DESC, direct_chat_id DESC`.
    fn list_by_actor(
        &self,
        query: DirectChatActorListQuery<'_>,
    ) -> Result<Vec<DirectChatRecord>, ContractError>;
    fn update_status(
        &self,
        tenant_id: &str,
        org_id: &str,
        direct_chat_id: i64,
        status: &str,
        updated_at: &str,
    ) -> Result<(), ContractError>;
    fn update_conversation_id(
        &self,
        tenant_id: &str,
        org_id: &str,
        direct_chat_id: i64,
        conversation_id: &str,
        updated_at: &str,
    ) -> Result<(), ContractError>;
}

const INSERT_SQL: &str = r#"
INSERT INTO im_direct_chats (
    tenant_id, organization_id, direct_chat_id, left_actor_kind, left_actor_id,
    right_actor_kind, right_actor_id, pair_hash, status, conversation_id,
    created_at, updated_at
) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11::timestamptz, $12::timestamptz)
ON CONFLICT (tenant_id, organization_id, direct_chat_id) DO NOTHING
"#;

const GET_BY_ID_SQL: &str = r#"
SELECT tenant_id, organization_id, direct_chat_id, left_actor_kind, left_actor_id,
       right_actor_kind, right_actor_id, pair_hash, status, conversation_id,
       created_at::text, updated_at::text
FROM im_direct_chats
WHERE tenant_id = $1 AND organization_id = $2 AND direct_chat_id = $3
"#;

const FIND_BY_PAIR_HASH_SQL: &str = r#"
SELECT tenant_id, organization_id, direct_chat_id, left_actor_kind, left_actor_id,
       right_actor_kind, right_actor_id, pair_hash, status, conversation_id,
       created_at::text, updated_at::text
FROM im_direct_chats
WHERE tenant_id = $1 AND organization_id = $2 AND pair_hash = $3
LIMIT 1
"#;

const LIST_BY_ACTOR_SQL: &str = r#"
SELECT tenant_id, organization_id, direct_chat_id, left_actor_kind, left_actor_id,
       right_actor_kind, right_actor_id, pair_hash, status, conversation_id,
       created_at::text, updated_at::text
FROM im_direct_chats
WHERE tenant_id = $1 AND organization_id = $2
  AND (left_actor_id = $3 OR right_actor_id = $3)
  AND status = $4
  AND ($5::timestamptz IS NULL
       OR (updated_at, direct_chat_id) < ($5::timestamptz, $6))
ORDER BY updated_at DESC, direct_chat_id DESC
LIMIT $7
"#;

const UPDATE_STATUS_SQL: &str = r#"
UPDATE im_direct_chats
SET status = $4, updated_at = $5::timestamptz
WHERE tenant_id = $1 AND organization_id = $2 AND direct_chat_id = $3
"#;

const UPDATE_CONVERSATION_ID_SQL: &str = r#"
UPDATE im_direct_chats
SET conversation_id = $4, updated_at = $5::timestamptz
WHERE tenant_id = $1 AND organization_id = $2 AND direct_chat_id = $3
"#;

fn row_to_record(row: &postgres::Row) -> DirectChatRecord {
    DirectChatRecord {
        tenant_id: row.get("tenant_id"),
        organization_id: row.get("organization_id"),
        direct_chat_id: row.get("direct_chat_id"),
        left_actor_kind: row.get("left_actor_kind"),
        left_actor_id: row.get("left_actor_id"),
        right_actor_kind: row.get("right_actor_kind"),
        right_actor_id: row.get("right_actor_id"),
        pair_hash: row.get("pair_hash"),
        status: row.get("status"),
        conversation_id: row.get("conversation_id"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    }
}

/// PostgreSQL-backed direct chat store.
#[derive(Clone)]
pub struct PostgresDirectChatStore {
    pool: Arc<Pool<SocialPostgresConnectionManager>>,
}

impl PostgresDirectChatStore {
    pub fn new(pool: Arc<Pool<SocialPostgresConnectionManager>>) -> Self {
        Self { pool }
    }
}

impl DirectChatStore for PostgresDirectChatStore {
    fn insert(&self, record: &DirectChatRecord) -> Result<(), ContractError> {
        let pool = self.pool.clone();
        let r = record.clone();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "insert_direct_chat")?;
            let created_at = postgres_timestamptz(r.created_at.as_str(), "created_at")?;
            let updated_at = postgres_timestamptz(r.updated_at.as_str(), "updated_at")?;
            client
                .execute(
                    INSERT_SQL,
                    &[
                        &r.tenant_id,
                        &r.organization_id,
                        &r.direct_chat_id,
                        &r.left_actor_kind,
                        &r.left_actor_id,
                        &r.right_actor_kind,
                        &r.right_actor_id,
                        &r.pair_hash,
                        &r.status,
                        &r.conversation_id,
                        &created_at,
                        &updated_at,
                    ],
                )
                .map_err(|e| postgres_unavailable("insert_direct_chat", e))?;
            Ok(())
        })
    }

    fn get_by_id(
        &self,
        tenant_id: &str,
        org_id: &str,
        direct_chat_id: i64,
    ) -> Result<Option<DirectChatRecord>, ContractError> {
        let pool = self.pool.clone();
        let tid = tenant_id.to_string();
        let oid = org_id.to_string();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "get_direct_chat")?;
            let row = client
                .query_opt(GET_BY_ID_SQL, &[&tid, &oid, &direct_chat_id])
                .map_err(|e| postgres_unavailable("get_direct_chat", e))?;
            Ok(row.map(|r| row_to_record(&r)))
        })
    }

    fn find_by_pair_hash(
        &self,
        tenant_id: &str,
        org_id: &str,
        pair_hash: &str,
    ) -> Result<Option<DirectChatRecord>, ContractError> {
        let pool = self.pool.clone();
        let tid = tenant_id.to_string();
        let oid = org_id.to_string();
        let ph = pair_hash.to_string();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "find_direct_chat_by_pair_hash")?;
            let row = client
                .query_opt(FIND_BY_PAIR_HASH_SQL, &[&tid, &oid, &ph])
                .map_err(|e| postgres_unavailable("find_direct_chat_by_pair_hash", e))?;
            Ok(row.map(|r| row_to_record(&r)))
        })
    }

    fn list_by_actor(
        &self,
        query: DirectChatActorListQuery<'_>,
    ) -> Result<Vec<DirectChatRecord>, ContractError> {
        let pool = self.pool.clone();
        let tid = query.tenant_id.to_string();
        let oid = query.organization_id.to_string();
        let aid = query.actor_id.to_string();
        let st = query.status.to_string();
        let cursor_ts = query.cursor_updated_at.map(str::to_owned);
        let cursor_direct_chat_id = query.cursor_direct_chat_id;
        let limit = query.limit;
        let cursor_ts_parsed = match &cursor_ts {
            Some(ts) => Some(optional_postgres_timestamptz(
                Some(ts.as_str()),
                "cursor_updated_at",
            )?),
            None => None,
        };
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "list_direct_chats_by_actor")?;
            let rows = client
                .query(
                    LIST_BY_ACTOR_SQL,
                    &[
                        &tid,
                        &oid,
                        &aid,
                        &st,
                        &cursor_ts_parsed,
                        &cursor_direct_chat_id,
                        &limit,
                    ],
                )
                .map_err(|e| postgres_unavailable("list_direct_chats_by_actor", e))?;
            Ok(rows.iter().map(row_to_record).collect())
        })
    }

    fn update_status(
        &self,
        tenant_id: &str,
        org_id: &str,
        direct_chat_id: i64,
        status: &str,
        updated_at: &str,
    ) -> Result<(), ContractError> {
        let pool = self.pool.clone();
        let tid = tenant_id.to_string();
        let oid = org_id.to_string();
        let st = status.to_string();
        let ua = updated_at.to_string();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "update_direct_chat_status")?;
            let updated_at_ts = postgres_timestamptz(ua.as_str(), "updated_at")?;
            client
                .execute(
                    UPDATE_STATUS_SQL,
                    &[&tid, &oid, &direct_chat_id, &st, &updated_at_ts],
                )
                .map_err(|e| postgres_unavailable("update_direct_chat_status", e))?;
            Ok(())
        })
    }

    fn update_conversation_id(
        &self,
        tenant_id: &str,
        org_id: &str,
        direct_chat_id: i64,
        conversation_id: &str,
        updated_at: &str,
    ) -> Result<(), ContractError> {
        let pool = self.pool.clone();
        let tid = tenant_id.to_string();
        let oid = org_id.to_string();
        let cid = conversation_id.to_string();
        let ua = updated_at.to_string();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "update_direct_chat_conversation_id")?;
            let updated_at_ts = postgres_timestamptz(ua.as_str(), "updated_at")?;
            client
                .execute(
                    UPDATE_CONVERSATION_ID_SQL,
                    &[&tid, &oid, &direct_chat_id, &cid, &updated_at_ts],
                )
                .map_err(|e| postgres_unavailable("update_direct_chat_conversation_id", e))?;
            Ok(())
        })
    }
}
