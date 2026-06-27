//! PostgreSQL store for IM user settings key-value rows.

use std::collections::HashMap;
use std::sync::Arc;

use im_platform_contracts::ContractError;
use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;
use serde_json::Value;

use crate::{NoTls, postgres_pool_client, postgres_unavailable, run_postgres_io};

/// Trait for user settings persistence.
pub trait UserSettingsStore: Send + Sync {
    fn list_by_user(
        &self,
        tenant_id: &str,
        org_id: &str,
        user_id: &str,
    ) -> Result<HashMap<String, Value>, ContractError>;
    fn upsert_settings(
        &self,
        tenant_id: &str,
        org_id: &str,
        user_id: &str,
        settings: &HashMap<String, Value>,
        updated_at: &str,
    ) -> Result<(), ContractError>;
}

const LIST_BY_USER_SQL: &str = r#"
SELECT setting_key, setting_value
FROM im_user_settings
WHERE tenant_id = $1 AND organization_id = $2 AND user_id = $3
"#;

const UPSERT_SETTING_SQL: &str = r#"
INSERT INTO im_user_settings (
    tenant_id, organization_id, user_id, setting_key, setting_value, updated_at
) VALUES ($1, $2, $3, $4, $5, $6::timestamptz)
ON CONFLICT (tenant_id, organization_id, user_id, setting_key) DO UPDATE SET
    setting_value = EXCLUDED.setting_value,
    updated_at = EXCLUDED.updated_at
"#;

/// PostgreSQL-backed user settings store.
#[derive(Clone)]
pub struct PostgresUserSettingsStore {
    pool: Arc<Pool<PostgresConnectionManager<NoTls>>>,
}

impl PostgresUserSettingsStore {
    pub fn new(pool: Arc<Pool<PostgresConnectionManager<NoTls>>>) -> Self {
        Self { pool }
    }
}

impl UserSettingsStore for PostgresUserSettingsStore {
    fn list_by_user(
        &self,
        tenant_id: &str,
        org_id: &str,
        user_id: &str,
    ) -> Result<HashMap<String, Value>, ContractError> {
        let pool = self.pool.clone();
        let tid = tenant_id.to_string();
        let oid = org_id.to_string();
        let uid = user_id.to_string();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "list_user_settings")?;
            let rows = client
                .query(LIST_BY_USER_SQL, &[&tid, &oid, &uid])
                .map_err(|e| postgres_unavailable("list_user_settings", e))?;
            let mut settings = HashMap::new();
            for row in rows {
                let key: String = row.get("setting_key");
                let value: Value = row.get("setting_value");
                settings.insert(key, value);
            }
            Ok(settings)
        })
    }

    fn upsert_settings(
        &self,
        tenant_id: &str,
        org_id: &str,
        user_id: &str,
        settings: &HashMap<String, Value>,
        updated_at: &str,
    ) -> Result<(), ContractError> {
        let pool = self.pool.clone();
        let tid = tenant_id.to_string();
        let oid = org_id.to_string();
        let uid = user_id.to_string();
        let entries: Vec<(String, Value)> = settings
            .iter()
            .map(|(key, value)| (key.clone(), value.clone()))
            .collect();
        let updated_at = updated_at.to_string();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "upsert_user_settings")?;
            for (key, value) in entries {
                client
                    .execute(
                        UPSERT_SETTING_SQL,
                        &[&tid, &oid, &uid, &key, &value, &updated_at],
                    )
                    .map_err(|e| postgres_unavailable("upsert_user_settings", e))?;
            }
            Ok(())
        })
    }
}
