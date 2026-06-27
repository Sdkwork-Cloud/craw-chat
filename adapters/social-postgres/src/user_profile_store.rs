//! PostgreSQL store for IM user profile extensions.

use std::sync::Arc;

use im_platform_contracts::ContractError;
use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;

use crate::{NoTls, postgres_pool_client, postgres_unavailable, run_postgres_io};

/// User profile record for database storage.
#[derive(Clone, Debug)]
pub struct UserProfileRecord {
    pub tenant_id: String,
    pub organization_id: String,
    pub user_id: String,
    pub im_nickname: Option<String>,
    pub im_avatar_url: Option<String>,
    pub im_status_message: Option<String>,
    pub im_online_status: String,
    pub last_active_at: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

/// Trait for user profile persistence.
pub trait UserProfileStore: Send + Sync {
    fn get_by_user_id(
        &self,
        tenant_id: &str,
        org_id: &str,
        user_id: &str,
    ) -> Result<Option<UserProfileRecord>, ContractError>;
    fn upsert_profile(&self, record: &UserProfileRecord) -> Result<(), ContractError>;
}

const GET_BY_USER_ID_SQL: &str = r#"
SELECT tenant_id, organization_id, user_id, im_nickname, im_avatar_url, im_status_message,
       im_online_status, last_active_at::text, created_at::text, updated_at::text
FROM im_user_profiles
WHERE tenant_id = $1 AND organization_id = $2 AND user_id = $3
"#;

const UPSERT_PROFILE_SQL: &str = r#"
INSERT INTO im_user_profiles (
    tenant_id, organization_id, user_id, im_nickname, im_avatar_url, im_status_message,
    im_online_status, last_active_at, created_at, updated_at
) VALUES ($1, $2, $3, $4, $5, $6, $7, $8::timestamptz, $9::timestamptz, $10::timestamptz)
ON CONFLICT (tenant_id, organization_id, user_id) DO UPDATE SET
    im_nickname = COALESCE(EXCLUDED.im_nickname, im_user_profiles.im_nickname),
    im_avatar_url = COALESCE(EXCLUDED.im_avatar_url, im_user_profiles.im_avatar_url),
    im_status_message = COALESCE(EXCLUDED.im_status_message, im_user_profiles.im_status_message),
    updated_at = EXCLUDED.updated_at
"#;

fn row_to_record(row: &postgres::Row) -> UserProfileRecord {
    UserProfileRecord {
        tenant_id: row.get("tenant_id"),
        organization_id: row.get("organization_id"),
        user_id: row.get("user_id"),
        im_nickname: row.get("im_nickname"),
        im_avatar_url: row.get("im_avatar_url"),
        im_status_message: row.get("im_status_message"),
        im_online_status: row.get("im_online_status"),
        last_active_at: row.get("last_active_at"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    }
}

/// PostgreSQL-backed user profile store.
#[derive(Clone)]
pub struct PostgresUserProfileStore {
    pool: Arc<Pool<PostgresConnectionManager<NoTls>>>,
}

impl PostgresUserProfileStore {
    pub fn new(pool: Arc<Pool<PostgresConnectionManager<NoTls>>>) -> Self {
        Self { pool }
    }
}

impl UserProfileStore for PostgresUserProfileStore {
    fn get_by_user_id(
        &self,
        tenant_id: &str,
        org_id: &str,
        user_id: &str,
    ) -> Result<Option<UserProfileRecord>, ContractError> {
        let pool = self.pool.clone();
        let tid = tenant_id.to_string();
        let oid = org_id.to_string();
        let uid = user_id.to_string();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "get_user_profile")?;
            let row = client
                .query_opt(GET_BY_USER_ID_SQL, &[&tid, &oid, &uid])
                .map_err(|e| postgres_unavailable("get_user_profile", e))?;
            Ok(row.map(|r| row_to_record(&r)))
        })
    }

    fn upsert_profile(&self, record: &UserProfileRecord) -> Result<(), ContractError> {
        let pool = self.pool.clone();
        let r = record.clone();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "upsert_user_profile")?;
            client
                .execute(
                    UPSERT_PROFILE_SQL,
                    &[
                        &r.tenant_id,
                        &r.organization_id,
                        &r.user_id,
                        &r.im_nickname,
                        &r.im_avatar_url,
                        &r.im_status_message,
                        &r.im_online_status,
                        &r.last_active_at,
                        &r.created_at,
                        &r.updated_at,
                    ],
                )
                .map_err(|e| postgres_unavailable("upsert_user_profile", e))?;
            Ok(())
        })
    }
}
