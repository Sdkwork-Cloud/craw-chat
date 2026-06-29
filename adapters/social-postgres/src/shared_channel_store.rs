//! PostgreSQL store for shared channel policies.

use std::sync::Arc;

use im_domain_core::social::{SharedChannelPolicy, SharedChannelPolicyStatus};
use im_platform_contracts::ContractError;
use r2d2::Pool;

use crate::{SocialPostgresConnectionManager, postgres_pool_client, postgres_unavailable, run_postgres_io};

/// Shared channel policy record for database storage.
#[derive(Clone, Debug)]
pub struct SharedChannelPolicyRecord {
    pub tenant_id: String,
    pub organization_id: String,
    pub policy_id: i64,
    pub connection_id: i64,
    pub channel_id: String,
    pub conversation_id: Option<String>,
    pub policy_version: i64,
    pub history_visibility: String,
    pub status: String,
    pub applied_at: String,
    pub updated_at: String,
}

impl SharedChannelPolicyRecord {
    pub fn from_domain(scp: &SharedChannelPolicy, organization_id: &str) -> Self {
        Self {
            tenant_id: scp.tenant_id.clone(),
            organization_id: organization_id.to_string(),
            policy_id: scp.policy_id.parse().unwrap_or(0),
            connection_id: scp.connection_id.parse().unwrap_or(0),
            channel_id: scp.channel_id.clone(),
            conversation_id: scp.conversation_id.clone(),
            policy_version: scp.policy_version as i64,
            history_visibility: scp.history_visibility.clone(),
            status: shared_channel_policy_status_to_str(&scp.status).to_string(),
            applied_at: scp.applied_at.clone(),
            updated_at: scp.updated_at.clone(),
        }
    }
}

fn shared_channel_policy_status_to_str(status: &SharedChannelPolicyStatus) -> &'static str {
    match status {
        SharedChannelPolicyStatus::Active => "active",
        SharedChannelPolicyStatus::Suspended => "suspended",
    }
}

/// Trait for shared channel policy persistence.
pub trait SharedChannelPolicyStore: Send + Sync {
    fn insert(&self, record: &SharedChannelPolicyRecord) -> Result<(), ContractError>;
    fn get_by_id(
        &self,
        tenant_id: &str,
        org_id: &str,
        policy_id: i64,
    ) -> Result<Option<SharedChannelPolicyRecord>, ContractError>;
    fn find_by_target(
        &self,
        tenant_id: &str,
        org_id: &str,
        connection_id: i64,
        channel_id: &str,
    ) -> Result<Option<SharedChannelPolicyRecord>, ContractError>;
    fn list_by_connection(
        &self,
        tenant_id: &str,
        org_id: &str,
        connection_id: i64,
        status: &str,
        limit: i64,
    ) -> Result<Vec<SharedChannelPolicyRecord>, ContractError>;
    fn update_status(
        &self,
        tenant_id: &str,
        org_id: &str,
        policy_id: i64,
        status: &str,
        updated_at: &str,
    ) -> Result<(), ContractError>;
    fn update_version(
        &self,
        tenant_id: &str,
        org_id: &str,
        policy_id: i64,
        version: i64,
        updated_at: &str,
    ) -> Result<(), ContractError>;
}

const INSERT_SQL: &str = r#"
INSERT INTO im_shared_channel_policies (
    tenant_id, organization_id, policy_id, connection_id, channel_id,
    conversation_id, policy_version, history_visibility, status,
    applied_at, updated_at
) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
ON CONFLICT (tenant_id, organization_id, policy_id) DO NOTHING
"#;

const GET_BY_ID_SQL: &str = r#"
SELECT tenant_id, organization_id, policy_id, connection_id, channel_id,
       conversation_id, policy_version, history_visibility, status,
       applied_at, updated_at
FROM im_shared_channel_policies
WHERE tenant_id = $1 AND organization_id = $2 AND policy_id = $3
"#;

const FIND_BY_TARGET_SQL: &str = r#"
SELECT tenant_id, organization_id, policy_id, connection_id, channel_id,
       conversation_id, policy_version, history_visibility, status,
       applied_at, updated_at
FROM im_shared_channel_policies
WHERE tenant_id = $1 AND organization_id = $2 AND connection_id = $3 AND channel_id = $4
LIMIT 1
"#;

const LIST_BY_CONNECTION_SQL: &str = r#"
SELECT tenant_id, organization_id, policy_id, connection_id, channel_id,
       conversation_id, policy_version, history_visibility, status,
       applied_at, updated_at
FROM im_shared_channel_policies
WHERE tenant_id = $1 AND organization_id = $2 AND connection_id = $3 AND status = $4
ORDER BY applied_at DESC
LIMIT $5
"#;

const UPDATE_STATUS_SQL: &str = r#"
UPDATE im_shared_channel_policies
SET status = $4, updated_at = $5
WHERE tenant_id = $1 AND organization_id = $2 AND policy_id = $3
"#;

const UPDATE_VERSION_SQL: &str = r#"
UPDATE im_shared_channel_policies
SET policy_version = $4, updated_at = $5
WHERE tenant_id = $1 AND organization_id = $2 AND policy_id = $3
"#;

fn row_to_record(row: &postgres::Row) -> SharedChannelPolicyRecord {
    SharedChannelPolicyRecord {
        tenant_id: row.get("tenant_id"),
        organization_id: row.get("organization_id"),
        policy_id: row.get("policy_id"),
        connection_id: row.get("connection_id"),
        channel_id: row.get("channel_id"),
        conversation_id: row.get("conversation_id"),
        policy_version: row.get("policy_version"),
        history_visibility: row.get("history_visibility"),
        status: row.get("status"),
        applied_at: row.get("applied_at"),
        updated_at: row.get("updated_at"),
    }
}

/// PostgreSQL-backed shared channel policy store.
#[derive(Clone)]
pub struct PostgresSharedChannelPolicyStore {
    pool: Arc<Pool<SocialPostgresConnectionManager>>,
}

impl PostgresSharedChannelPolicyStore {
    pub fn new(pool: Arc<Pool<SocialPostgresConnectionManager>>) -> Self {
        Self { pool }
    }
}

impl SharedChannelPolicyStore for PostgresSharedChannelPolicyStore {
    fn insert(&self, record: &SharedChannelPolicyRecord) -> Result<(), ContractError> {
        let pool = self.pool.clone();
        let r = record.clone();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "insert_shared_channel_policy")?;
            client
                .execute(
                    INSERT_SQL,
                    &[
                        &r.tenant_id,
                        &r.organization_id,
                        &r.policy_id,
                        &r.connection_id,
                        &r.channel_id,
                        &r.conversation_id,
                        &r.policy_version,
                        &r.history_visibility,
                        &r.status,
                        &r.applied_at,
                        &r.updated_at,
                    ],
                )
                .map_err(|e| postgres_unavailable("insert_shared_channel_policy", e))?;
            Ok(())
        })
    }

    fn get_by_id(
        &self,
        tenant_id: &str,
        org_id: &str,
        policy_id: i64,
    ) -> Result<Option<SharedChannelPolicyRecord>, ContractError> {
        let pool = self.pool.clone();
        let tid = tenant_id.to_string();
        let oid = org_id.to_string();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "get_shared_channel_policy")?;
            let row = client
                .query_opt(GET_BY_ID_SQL, &[&tid, &oid, &policy_id])
                .map_err(|e| postgres_unavailable("get_shared_channel_policy", e))?;
            Ok(row.map(|r| row_to_record(&r)))
        })
    }

    fn find_by_target(
        &self,
        tenant_id: &str,
        org_id: &str,
        connection_id: i64,
        channel_id: &str,
    ) -> Result<Option<SharedChannelPolicyRecord>, ContractError> {
        let pool = self.pool.clone();
        let tid = tenant_id.to_string();
        let oid = org_id.to_string();
        let chid = channel_id.to_string();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "find_shared_channel_policy_by_target")?;
            let row = client
                .query_opt(FIND_BY_TARGET_SQL, &[&tid, &oid, &connection_id, &chid])
                .map_err(|e| postgres_unavailable("find_shared_channel_policy_by_target", e))?;
            Ok(row.map(|r| row_to_record(&r)))
        })
    }

    fn list_by_connection(
        &self,
        tenant_id: &str,
        org_id: &str,
        connection_id: i64,
        status: &str,
        limit: i64,
    ) -> Result<Vec<SharedChannelPolicyRecord>, ContractError> {
        let pool = self.pool.clone();
        let tid = tenant_id.to_string();
        let oid = org_id.to_string();
        let st = status.to_string();
        run_postgres_io(move || {
            let mut client =
                postgres_pool_client(&pool, "list_shared_channel_policies_by_connection")?;
            let rows = client
                .query(
                    LIST_BY_CONNECTION_SQL,
                    &[&tid, &oid, &connection_id, &st, &limit],
                )
                .map_err(|e| {
                    postgres_unavailable("list_shared_channel_policies_by_connection", e)
                })?;
            Ok(rows.iter().map(row_to_record).collect())
        })
    }

    fn update_status(
        &self,
        tenant_id: &str,
        org_id: &str,
        policy_id: i64,
        status: &str,
        updated_at: &str,
    ) -> Result<(), ContractError> {
        let pool = self.pool.clone();
        let tid = tenant_id.to_string();
        let oid = org_id.to_string();
        let st = status.to_string();
        let ua = updated_at.to_string();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "update_shared_channel_policy_status")?;
            client
                .execute(UPDATE_STATUS_SQL, &[&tid, &oid, &policy_id, &st, &ua])
                .map_err(|e| postgres_unavailable("update_shared_channel_policy_status", e))?;
            Ok(())
        })
    }

    fn update_version(
        &self,
        tenant_id: &str,
        org_id: &str,
        policy_id: i64,
        version: i64,
        updated_at: &str,
    ) -> Result<(), ContractError> {
        let pool = self.pool.clone();
        let tid = tenant_id.to_string();
        let oid = org_id.to_string();
        let ua = updated_at.to_string();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "update_shared_channel_policy_version")?;
            client
                .execute(UPDATE_VERSION_SQL, &[&tid, &oid, &policy_id, &version, &ua])
                .map_err(|e| postgres_unavailable("update_shared_channel_policy_version", e))?;
            Ok(())
        })
    }
}
