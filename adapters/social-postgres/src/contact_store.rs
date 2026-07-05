//! PostgreSQL persistence for contact tags and per-contact preferences.

use std::sync::Arc;

use im_platform_contracts::ContractError;
use r2d2::Pool;

use crate::{SocialPostgresConnectionManager, postgres_pool_client, postgres_unavailable, run_postgres_io};
use crate::wire_id::social_entity_id_to_i64;

#[derive(Clone, Debug)]
pub struct ContactTagRecord {
    pub tenant_id: String,
    pub organization_id: String,
    pub owner_user_id: String,
    pub tag_id: i64,
    pub name: String,
    pub color: String,
    pub count: i32,
    pub bg: String,
    pub border: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Clone, Debug, Default)]
pub struct ContactPreferencesRecord {
    pub tenant_id: String,
    pub organization_id: String,
    pub owner_user_id: String,
    pub target_user_id: String,
    pub is_starred: bool,
    pub is_blocked: bool,
    pub remark: Option<String>,
    pub updated_at: String,
}

#[derive(Clone, Debug)]
pub struct ContactRecommendationRecord {
    pub tenant_id: String,
    pub organization_id: String,
    pub owner_user_id: String,
    pub target_user_id: String,
    pub recommendation_id: i64,
    pub target_conversation_id: Option<String>,
    pub created_at: String,
}

pub trait ContactStore: Send + Sync {
    fn upsert_tag(&self, record: &ContactTagRecord) -> Result<(), ContractError>;
    fn delete_tag(
        &self,
        tenant_id: &str,
        org_id: &str,
        owner_user_id: &str,
        tag_id: i64,
    ) -> Result<(), ContractError>;
    fn list_tags_by_owner(
        &self,
        tenant_id: &str,
        org_id: &str,
        owner_user_id: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<ContactTagRecord>, ContractError>;
    fn get_tag(
        &self,
        tenant_id: &str,
        org_id: &str,
        owner_user_id: &str,
        tag_id: i64,
    ) -> Result<Option<ContactTagRecord>, ContractError>;

    fn upsert_preferences(&self, record: &ContactPreferencesRecord) -> Result<(), ContractError>;
    fn get_preferences(
        &self,
        tenant_id: &str,
        org_id: &str,
        owner_user_id: &str,
        target_user_id: &str,
    ) -> Result<Option<ContactPreferencesRecord>, ContractError>;

    fn insert_recommendation(&self, record: &ContactRecommendationRecord) -> Result<(), ContractError>;
}

const GET_TAG_SQL: &str = r#"
SELECT tenant_id, organization_id, owner_user_id, tag_id, name, color, count, bg, border,
       created_at, updated_at
FROM im_contact_tags
WHERE tenant_id = $1 AND organization_id = $2 AND owner_user_id = $3 AND tag_id = $4
"#;

const UPSERT_TAG_SQL: &str = r#"
INSERT INTO im_contact_tags (
    tenant_id, organization_id, owner_user_id, tag_id, name, color, count, bg, border,
    created_at, updated_at
) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
ON CONFLICT (tenant_id, organization_id, owner_user_id, tag_id) DO UPDATE SET
    name = EXCLUDED.name,
    color = EXCLUDED.color,
    count = EXCLUDED.count,
    bg = EXCLUDED.bg,
    border = EXCLUDED.border,
    updated_at = EXCLUDED.updated_at
"#;

const DELETE_TAG_SQL: &str = r#"
DELETE FROM im_contact_tags
WHERE tenant_id = $1 AND organization_id = $2 AND owner_user_id = $3 AND tag_id = $4
"#;

const LIST_TAGS_SQL: &str = r#"
SELECT tenant_id, organization_id, owner_user_id, tag_id, name, color, count, bg, border,
       created_at, updated_at
FROM im_contact_tags
WHERE tenant_id = $1 AND organization_id = $2 AND owner_user_id = $3
ORDER BY updated_at DESC, tag_id DESC
LIMIT $4 OFFSET $5
"#;

const UPSERT_PREFERENCES_SQL: &str = r#"
INSERT INTO im_contact_preferences (
    tenant_id, organization_id, owner_user_id, target_user_id,
    is_starred, is_blocked, remark, updated_at
) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
ON CONFLICT (tenant_id, organization_id, owner_user_id, target_user_id) DO UPDATE SET
    is_starred = EXCLUDED.is_starred,
    is_blocked = EXCLUDED.is_blocked,
    remark = EXCLUDED.remark,
    updated_at = EXCLUDED.updated_at
"#;

const GET_PREFERENCES_SQL: &str = r#"
SELECT tenant_id, organization_id, owner_user_id, target_user_id,
       is_starred, is_blocked, remark, updated_at
FROM im_contact_preferences
WHERE tenant_id = $1 AND organization_id = $2 AND owner_user_id = $3 AND target_user_id = $4
"#;

const INSERT_RECOMMENDATION_SQL: &str = r#"
INSERT INTO im_contact_recommendations (
    tenant_id, organization_id, owner_user_id, target_user_id,
    recommendation_id, target_conversation_id, created_at
) VALUES ($1, $2, $3, $4, $5, $6, $7)
"#;

#[derive(Clone)]
pub struct PostgresContactStore {
    pool: Arc<Pool<SocialPostgresConnectionManager>>,
}

impl PostgresContactStore {
    pub fn new(pool: Arc<Pool<SocialPostgresConnectionManager>>) -> Self {
        Self { pool }
    }
}

pub fn contact_tag_id_to_i64(tag_id: &str) -> i64 {
    social_entity_id_to_i64(tag_id)
}

pub fn contact_recommendation_id_to_i64(recommendation_id: &str) -> i64 {
    social_entity_id_to_i64(recommendation_id)
}

impl ContactStore for PostgresContactStore {
    fn upsert_tag(&self, record: &ContactTagRecord) -> Result<(), ContractError> {
        let pool = self.pool.clone();
        let record = record.clone();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "upsert_contact_tag")?;
            client
                .execute(
                    UPSERT_TAG_SQL,
                    &[
                        &record.tenant_id,
                        &record.organization_id,
                        &record.owner_user_id,
                        &record.tag_id,
                        &record.name,
                        &record.color,
                        &record.count,
                        &record.bg,
                        &record.border,
                        &record.created_at,
                        &record.updated_at,
                    ],
                )
                .map_err(|error| postgres_unavailable("upsert_contact_tag", error))?;
            Ok(())
        })
    }

    fn delete_tag(
        &self,
        tenant_id: &str,
        org_id: &str,
        owner_user_id: &str,
        tag_id: i64,
    ) -> Result<(), ContractError> {
        let pool = self.pool.clone();
        let tenant_id = tenant_id.to_owned();
        let org_id = org_id.to_owned();
        let owner_user_id = owner_user_id.to_owned();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "delete_contact_tag")?;
            client
                .execute(
                    DELETE_TAG_SQL,
                    &[&tenant_id, &org_id, &owner_user_id, &tag_id],
                )
                .map_err(|error| postgres_unavailable("delete_contact_tag", error))?;
            Ok(())
        })
    }

    fn list_tags_by_owner(
        &self,
        tenant_id: &str,
        org_id: &str,
        owner_user_id: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<ContactTagRecord>, ContractError> {
        let pool = self.pool.clone();
        let tenant_id = tenant_id.to_owned();
        let org_id = org_id.to_owned();
        let owner_user_id = owner_user_id.to_owned();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "list_contact_tags")?;
            let rows = client
                .query(
                    LIST_TAGS_SQL,
                    &[&tenant_id, &org_id, &owner_user_id, &limit, &offset],
                )
                .map_err(|error| postgres_unavailable("list_contact_tags", error))?;
            Ok(rows
                .iter()
                .map(|row| ContactTagRecord {
                    tenant_id: row.get("tenant_id"),
                    organization_id: row.get("organization_id"),
                    owner_user_id: row.get("owner_user_id"),
                    tag_id: row.get("tag_id"),
                    name: row.get("name"),
                    color: row.get("color"),
                    count: row.get("count"),
                    bg: row.get("bg"),
                    border: row.get("border"),
                    created_at: row.get("created_at"),
                    updated_at: row.get("updated_at"),
                })
                .collect())
        })
    }

    fn get_tag(
        &self,
        tenant_id: &str,
        org_id: &str,
        owner_user_id: &str,
        tag_id: i64,
    ) -> Result<Option<ContactTagRecord>, ContractError> {
        let pool = self.pool.clone();
        let tenant_id = tenant_id.to_owned();
        let org_id = org_id.to_owned();
        let owner_user_id = owner_user_id.to_owned();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "get_contact_tag")?;
            let row = client
                .query_opt(
                    GET_TAG_SQL,
                    &[&tenant_id, &org_id, &owner_user_id, &tag_id],
                )
                .map_err(|error| postgres_unavailable("get_contact_tag", error))?;
            Ok(row.map(|row| ContactTagRecord {
                tenant_id: row.get("tenant_id"),
                organization_id: row.get("organization_id"),
                owner_user_id: row.get("owner_user_id"),
                tag_id: row.get("tag_id"),
                name: row.get("name"),
                color: row.get("color"),
                count: row.get("count"),
                bg: row.get("bg"),
                border: row.get("border"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            }))
        })
    }

    fn upsert_preferences(&self, record: &ContactPreferencesRecord) -> Result<(), ContractError> {
        let pool = self.pool.clone();
        let record = record.clone();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "upsert_contact_preferences")?;
            client
                .execute(
                    UPSERT_PREFERENCES_SQL,
                    &[
                        &record.tenant_id,
                        &record.organization_id,
                        &record.owner_user_id,
                        &record.target_user_id,
                        &record.is_starred,
                        &record.is_blocked,
                        &record.remark,
                        &record.updated_at,
                    ],
                )
                .map_err(|error| postgres_unavailable("upsert_contact_preferences", error))?;
            Ok(())
        })
    }

    fn get_preferences(
        &self,
        tenant_id: &str,
        org_id: &str,
        owner_user_id: &str,
        target_user_id: &str,
    ) -> Result<Option<ContactPreferencesRecord>, ContractError> {
        let pool = self.pool.clone();
        let tenant_id = tenant_id.to_owned();
        let org_id = org_id.to_owned();
        let owner_user_id = owner_user_id.to_owned();
        let target_user_id = target_user_id.to_owned();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "get_contact_preferences")?;
            let row = client
                .query_opt(
                    GET_PREFERENCES_SQL,
                    &[&tenant_id, &org_id, &owner_user_id, &target_user_id],
                )
                .map_err(|error| postgres_unavailable("get_contact_preferences", error))?;
            Ok(row.map(|row| ContactPreferencesRecord {
                tenant_id: row.get("tenant_id"),
                organization_id: row.get("organization_id"),
                owner_user_id: row.get("owner_user_id"),
                target_user_id: row.get("target_user_id"),
                is_starred: row.get("is_starred"),
                is_blocked: row.get("is_blocked"),
                remark: row.get("remark"),
                updated_at: row.get("updated_at"),
            }))
        })
    }

    fn insert_recommendation(&self, record: &ContactRecommendationRecord) -> Result<(), ContractError> {
        let pool = self.pool.clone();
        let record = record.clone();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "insert_contact_recommendation")?;
            client
                .execute(
                    INSERT_RECOMMENDATION_SQL,
                    &[
                        &record.tenant_id,
                        &record.organization_id,
                        &record.owner_user_id,
                        &record.target_user_id,
                        &record.recommendation_id,
                        &record.target_conversation_id,
                        &record.created_at,
                    ],
                )
                .map_err(|error| postgres_unavailable("insert_contact_recommendation", error))?;
            Ok(())
        })
    }
}
