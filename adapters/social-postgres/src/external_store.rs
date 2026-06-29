//! PostgreSQL store for external connections and member links.

use std::sync::Arc;

use im_domain_core::social::{
    ExternalConnection, ExternalConnectionStatus, ExternalMemberLink, ExternalMemberLinkStatus,
};
use im_platform_contracts::ContractError;
use r2d2::Pool;

use crate::{SocialPostgresConnectionManager, postgres_pool_client, postgres_unavailable, run_postgres_io};

/// External connection record for database storage.
#[derive(Clone, Debug)]
pub struct ExternalConnectionRecord {
    pub tenant_id: String,
    pub organization_id: String,
    pub connection_id: i64,
    pub external_tenant_id: String,
    pub external_org_name: Option<String>,
    pub connection_kind: String,
    pub status: String,
    pub established_at: String,
    pub updated_at: String,
}

impl ExternalConnectionRecord {
    pub fn from_domain(ec: &ExternalConnection, organization_id: &str) -> Self {
        Self {
            tenant_id: ec.tenant_id.clone(),
            organization_id: organization_id.to_string(),
            connection_id: ec.connection_id.parse().unwrap_or(0),
            external_tenant_id: ec.external_tenant_id.clone(),
            external_org_name: ec.external_org_name.clone(),
            connection_kind: "shared_channel".to_string(),
            status: external_connection_status_to_str(&ec.status).to_string(),
            established_at: ec.established_at.clone(),
            updated_at: ec.updated_at.clone(),
        }
    }
}

fn external_connection_status_to_str(status: &ExternalConnectionStatus) -> &'static str {
    match status {
        ExternalConnectionStatus::Active => "active",
        ExternalConnectionStatus::Suspended => "suspended",
        ExternalConnectionStatus::Revoked => "revoked",
    }
}

/// External member link record for database storage.
#[derive(Clone, Debug)]
pub struct ExternalMemberLinkRecord {
    pub tenant_id: String,
    pub organization_id: String,
    pub link_id: i64,
    pub connection_id: i64,
    pub local_actor_kind: String,
    pub local_actor_id: String,
    pub external_member_id: String,
    pub external_display_name: Option<String>,
    pub status: String,
    pub linked_at: String,
    pub updated_at: String,
}

impl ExternalMemberLinkRecord {
    pub fn from_domain(eml: &ExternalMemberLink, organization_id: &str) -> Self {
        Self {
            tenant_id: eml.tenant_id.clone(),
            organization_id: organization_id.to_string(),
            link_id: eml.link_id.parse().unwrap_or(0),
            connection_id: eml.connection_id.parse().unwrap_or(0),
            local_actor_kind: eml.local_actor_kind.clone(),
            local_actor_id: eml.local_actor_id.clone(),
            external_member_id: eml.external_member_id.clone(),
            external_display_name: eml.external_display_name.clone(),
            status: external_member_link_status_to_str(&eml.status).to_string(),
            linked_at: eml.linked_at.clone(),
            updated_at: eml.updated_at.clone(),
        }
    }
}

fn external_member_link_status_to_str(status: &ExternalMemberLinkStatus) -> &'static str {
    match status {
        ExternalMemberLinkStatus::Active => "active",
        ExternalMemberLinkStatus::Revoked => "revoked",
    }
}

/// Trait for external connection persistence.
pub trait ExternalConnectionStore: Send + Sync {
    fn insert(&self, record: &ExternalConnectionRecord) -> Result<(), ContractError>;
    fn get_by_id(
        &self,
        tenant_id: &str,
        org_id: &str,
        connection_id: i64,
    ) -> Result<Option<ExternalConnectionRecord>, ContractError>;
    fn find_by_external_tenant(
        &self,
        tenant_id: &str,
        org_id: &str,
        external_tenant_id: &str,
    ) -> Result<Option<ExternalConnectionRecord>, ContractError>;
}

/// Trait for external member link persistence.
pub trait ExternalMemberLinkStore: Send + Sync {
    fn insert(&self, record: &ExternalMemberLinkRecord) -> Result<(), ContractError>;
    fn get_by_id(
        &self,
        tenant_id: &str,
        org_id: &str,
        link_id: i64,
    ) -> Result<Option<ExternalMemberLinkRecord>, ContractError>;
    fn list_by_connection(
        &self,
        tenant_id: &str,
        org_id: &str,
        connection_id: i64,
        status: &str,
        limit: i64,
    ) -> Result<Vec<ExternalMemberLinkRecord>, ContractError>;
    fn find_by_mapping(
        &self,
        tenant_id: &str,
        org_id: &str,
        connection_id: i64,
        local_actor_id: &str,
        external_member_id: &str,
    ) -> Result<Option<ExternalMemberLinkRecord>, ContractError>;
}

// External Connection SQL
const EC_INSERT_SQL: &str = r#"
INSERT INTO im_external_connections (
    tenant_id, organization_id, connection_id, external_tenant_id,
    external_org_name, connection_kind, status, established_at, updated_at
) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
ON CONFLICT (tenant_id, organization_id, connection_id) DO NOTHING
"#;

const EC_GET_BY_ID_SQL: &str = r#"
SELECT tenant_id, organization_id, connection_id, external_tenant_id,
       external_org_name, connection_kind, status, established_at, updated_at
FROM im_external_connections
WHERE tenant_id = $1 AND organization_id = $2 AND connection_id = $3
"#;

const EC_FIND_BY_EXTERNAL_TENANT_SQL: &str = r#"
SELECT tenant_id, organization_id, connection_id, external_tenant_id,
       external_org_name, connection_kind, status, established_at, updated_at
FROM im_external_connections
WHERE tenant_id = $1 AND organization_id = $2 AND external_tenant_id = $3
LIMIT 1
"#;

// External Member Link SQL
const EML_INSERT_SQL: &str = r#"
INSERT INTO im_external_member_links (
    tenant_id, organization_id, link_id, connection_id,
    local_actor_kind, local_actor_id, external_member_id,
    external_display_name, status, linked_at, updated_at
) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
ON CONFLICT (tenant_id, organization_id, link_id) DO NOTHING
"#;

const EML_GET_BY_ID_SQL: &str = r#"
SELECT tenant_id, organization_id, link_id, connection_id,
       local_actor_kind, local_actor_id, external_member_id,
       external_display_name, status, linked_at, updated_at
FROM im_external_member_links
WHERE tenant_id = $1 AND organization_id = $2 AND link_id = $3
"#;

const EML_LIST_BY_CONNECTION_SQL: &str = r#"
SELECT tenant_id, organization_id, link_id, connection_id,
       local_actor_kind, local_actor_id, external_member_id,
       external_display_name, status, linked_at, updated_at
FROM im_external_member_links
WHERE tenant_id = $1 AND organization_id = $2 AND connection_id = $3 AND status = $4
ORDER BY linked_at DESC
LIMIT $5
"#;

const EML_FIND_BY_MAPPING_SQL: &str = r#"
SELECT tenant_id, organization_id, link_id, connection_id,
       local_actor_kind, local_actor_id, external_member_id,
       external_display_name, status, linked_at, updated_at
FROM im_external_member_links
WHERE tenant_id = $1 AND organization_id = $2 AND connection_id = $3
  AND local_actor_id = $4 AND external_member_id = $5
LIMIT 1
"#;

fn row_to_ec_record(row: &postgres::Row) -> ExternalConnectionRecord {
    ExternalConnectionRecord {
        tenant_id: row.get("tenant_id"),
        organization_id: row.get("organization_id"),
        connection_id: row.get("connection_id"),
        external_tenant_id: row.get("external_tenant_id"),
        external_org_name: row.get("external_org_name"),
        connection_kind: row.get("connection_kind"),
        status: row.get("status"),
        established_at: row.get("established_at"),
        updated_at: row.get("updated_at"),
    }
}

fn row_to_eml_record(row: &postgres::Row) -> ExternalMemberLinkRecord {
    ExternalMemberLinkRecord {
        tenant_id: row.get("tenant_id"),
        organization_id: row.get("organization_id"),
        link_id: row.get("link_id"),
        connection_id: row.get("connection_id"),
        local_actor_kind: row.get("local_actor_kind"),
        local_actor_id: row.get("local_actor_id"),
        external_member_id: row.get("external_member_id"),
        external_display_name: row.get("external_display_name"),
        status: row.get("status"),
        linked_at: row.get("linked_at"),
        updated_at: row.get("updated_at"),
    }
}

/// PostgreSQL-backed external connection store.
#[derive(Clone)]
pub struct PostgresExternalConnectionStore {
    pool: Arc<Pool<SocialPostgresConnectionManager>>,
}

impl PostgresExternalConnectionStore {
    pub fn new(pool: Arc<Pool<SocialPostgresConnectionManager>>) -> Self {
        Self { pool }
    }
}

impl ExternalConnectionStore for PostgresExternalConnectionStore {
    fn insert(&self, record: &ExternalConnectionRecord) -> Result<(), ContractError> {
        let pool = self.pool.clone();
        let r = record.clone();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "insert_external_connection")?;
            client
                .execute(
                    EC_INSERT_SQL,
                    &[
                        &r.tenant_id,
                        &r.organization_id,
                        &r.connection_id,
                        &r.external_tenant_id,
                        &r.external_org_name,
                        &r.connection_kind,
                        &r.status,
                        &r.established_at,
                        &r.updated_at,
                    ],
                )
                .map_err(|e| postgres_unavailable("insert_external_connection", e))?;
            Ok(())
        })
    }

    fn get_by_id(
        &self,
        tenant_id: &str,
        org_id: &str,
        connection_id: i64,
    ) -> Result<Option<ExternalConnectionRecord>, ContractError> {
        let pool = self.pool.clone();
        let tid = tenant_id.to_string();
        let oid = org_id.to_string();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "get_external_connection")?;
            let row = client
                .query_opt(EC_GET_BY_ID_SQL, &[&tid, &oid, &connection_id])
                .map_err(|e| postgres_unavailable("get_external_connection", e))?;
            Ok(row.map(|r| row_to_ec_record(&r)))
        })
    }

    fn find_by_external_tenant(
        &self,
        tenant_id: &str,
        org_id: &str,
        external_tenant_id: &str,
    ) -> Result<Option<ExternalConnectionRecord>, ContractError> {
        let pool = self.pool.clone();
        let tid = tenant_id.to_string();
        let oid = org_id.to_string();
        let etid = external_tenant_id.to_string();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "find_external_connection_by_tenant")?;
            let row = client
                .query_opt(EC_FIND_BY_EXTERNAL_TENANT_SQL, &[&tid, &oid, &etid])
                .map_err(|e| postgres_unavailable("find_external_connection_by_tenant", e))?;
            Ok(row.map(|r| row_to_ec_record(&r)))
        })
    }
}

/// PostgreSQL-backed external member link store.
#[derive(Clone)]
pub struct PostgresExternalMemberLinkStore {
    pool: Arc<Pool<SocialPostgresConnectionManager>>,
}

impl PostgresExternalMemberLinkStore {
    pub fn new(pool: Arc<Pool<SocialPostgresConnectionManager>>) -> Self {
        Self { pool }
    }
}

impl ExternalMemberLinkStore for PostgresExternalMemberLinkStore {
    fn insert(&self, record: &ExternalMemberLinkRecord) -> Result<(), ContractError> {
        let pool = self.pool.clone();
        let r = record.clone();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "insert_external_member_link")?;
            client
                .execute(
                    EML_INSERT_SQL,
                    &[
                        &r.tenant_id,
                        &r.organization_id,
                        &r.link_id,
                        &r.connection_id,
                        &r.local_actor_kind,
                        &r.local_actor_id,
                        &r.external_member_id,
                        &r.external_display_name,
                        &r.status,
                        &r.linked_at,
                        &r.updated_at,
                    ],
                )
                .map_err(|e| postgres_unavailable("insert_external_member_link", e))?;
            Ok(())
        })
    }

    fn get_by_id(
        &self,
        tenant_id: &str,
        org_id: &str,
        link_id: i64,
    ) -> Result<Option<ExternalMemberLinkRecord>, ContractError> {
        let pool = self.pool.clone();
        let tid = tenant_id.to_string();
        let oid = org_id.to_string();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "get_external_member_link")?;
            let row = client
                .query_opt(EML_GET_BY_ID_SQL, &[&tid, &oid, &link_id])
                .map_err(|e| postgres_unavailable("get_external_member_link", e))?;
            Ok(row.map(|r| row_to_eml_record(&r)))
        })
    }

    fn list_by_connection(
        &self,
        tenant_id: &str,
        org_id: &str,
        connection_id: i64,
        status: &str,
        limit: i64,
    ) -> Result<Vec<ExternalMemberLinkRecord>, ContractError> {
        let pool = self.pool.clone();
        let tid = tenant_id.to_string();
        let oid = org_id.to_string();
        let st = status.to_string();
        run_postgres_io(move || {
            let mut client =
                postgres_pool_client(&pool, "list_external_member_links_by_connection")?;
            let rows = client
                .query(
                    EML_LIST_BY_CONNECTION_SQL,
                    &[&tid, &oid, &connection_id, &st, &limit],
                )
                .map_err(|e| postgres_unavailable("list_external_member_links_by_connection", e))?;
            Ok(rows.iter().map(row_to_eml_record).collect())
        })
    }

    fn find_by_mapping(
        &self,
        tenant_id: &str,
        org_id: &str,
        connection_id: i64,
        local_actor_id: &str,
        external_member_id: &str,
    ) -> Result<Option<ExternalMemberLinkRecord>, ContractError> {
        let pool = self.pool.clone();
        let tid = tenant_id.to_string();
        let oid = org_id.to_string();
        let laid = local_actor_id.to_string();
        let emid = external_member_id.to_string();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "find_external_member_link_by_mapping")?;
            let row = client
                .query_opt(
                    EML_FIND_BY_MAPPING_SQL,
                    &[&tid, &oid, &connection_id, &laid, &emid],
                )
                .map_err(|e| postgres_unavailable("find_external_member_link_by_mapping", e))?;
            Ok(row.map(|r| row_to_eml_record(&r)))
        })
    }
}
