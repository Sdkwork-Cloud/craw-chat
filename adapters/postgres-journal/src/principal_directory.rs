//! Postgres-backed dynamic principal directory.
//!
//! Semantics: "authenticated means registered" — the gateway has already
//! validated the IAM token, so any user principal reaching the conversation
//! service is admitted on first sight (and recorded for audit), unless an
//! explicit disable flag exists. This keeps new IAM registrations usable
//! immediately while preserving an admin-controlled disable list, which a
//! static JSON catalog cannot do without redeploys.
//!
//! State lives in `im_user_settings` (organization_id = '0' is the
//! tenant-scoped convention):
//! - `principal.disabled` — `{"disabled": true, "disabledAt": ...}` denies;
//! - `principal.seen` — `{"firstSeenAt": ...}` audit record upserted on the
//!   first admitted request.

use im_platform_contracts::{ContractError, PrincipalDirectory, PrincipalDirectoryError};
use postgres::types::Json;
use serde::{Deserialize, Serialize};

use crate::{
    PostgresJournalPool, now_rfc3339, postgres_pool_client, postgres_row_get, postgres_timestamptz,
    postgres_unavailable, run_postgres_io,
};

const READ_DISABLED_SQL: &str = r#"
select setting_value
from im_user_settings
where tenant_id = $1 and organization_id = $2 and user_id = $3 and setting_key = 'principal.disabled'
"#;

const REGISTER_SEEN_SQL: &str = r#"
insert into im_user_settings (tenant_id, organization_id, user_id, setting_key, setting_value, updated_at)
values ($1, '0', $2, 'principal.seen', $3::jsonb, $4::timestamptz)
on conflict (tenant_id, organization_id, user_id, setting_key) do update
set setting_value = excluded.setting_value, updated_at = excluded.updated_at
"#;

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PrincipalDisabledFlag {
    disabled: bool,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct PrincipalSeenRecord {
    first_seen_at: String,
}

enum PrincipalAdmission {
    Admit,
    Disabled,
}

/// PostgreSQL implementation of [`PrincipalDirectory`].
#[derive(Clone)]
pub struct PostgresPrincipalDirectory {
    pool: PostgresJournalPool,
}

impl PostgresPrincipalDirectory {
    pub fn from_pool(pool: PostgresJournalPool) -> Self {
        Self { pool }
    }
}

impl PrincipalDirectory for PostgresPrincipalDirectory {
    fn ensure_active_principal(
        &self,
        tenant_id: &str,
        principal_id: &str,
        principal_kind: &str,
    ) -> Result<(), PrincipalDirectoryError> {
        // Like the static directory, only user principals are constrained by
        // the directory; system/service/agent actors are handled elsewhere.
        if principal_kind != "user" {
            return Ok(());
        }

        let pool = self.pool.clone();
        let tenant_id = tenant_id.to_owned();
        let principal_id = principal_id.to_owned();
        let admission = run_postgres_io({
            let tenant_id = tenant_id.clone();
            let principal_id = principal_id.clone();
            move || -> Result<PrincipalAdmission, ContractError> {
                let mut client = postgres_pool_client(&pool, "ensure_active_principal")?;
                let rows = client
                    .query(READ_DISABLED_SQL, &[&tenant_id, &"0", &principal_id])
                    .map_err(|error| postgres_unavailable("ensure_active_principal", error))?;
                if let Some(row) = rows.first() {
                    let payload: Json<PrincipalDisabledFlag> =
                        postgres_row_get(row, 0, "ensure_active_principal", "setting_value")?;
                    if payload.0.disabled {
                        return Ok(PrincipalAdmission::Disabled);
                    }
                }

                // Authenticated means registered: record the admission for audit
                // and admit the principal.
                let seen: Json<PrincipalSeenRecord> = Json(PrincipalSeenRecord {
                    first_seen_at: now_rfc3339(),
                });
                let updated_at = postgres_timestamptz(&now_rfc3339(), "updated_at")?;
                client
                    .execute(
                        REGISTER_SEEN_SQL,
                        &[&tenant_id, &principal_id, &seen, &updated_at],
                    )
                    .map_err(|error| postgres_unavailable("ensure_active_principal", error))?;
                Ok(PrincipalAdmission::Admit)
            }
        });

        match admission {
            Ok(PrincipalAdmission::Admit) => Ok(()),
            Ok(PrincipalAdmission::Disabled) => Err(PrincipalDirectoryError::PrincipalDisabled {
                tenant_id: tenant_id.clone(),
                principal_id: principal_id.clone(),
                principal_kind: "user".into(),
            }),
            Err(error) => Err(PrincipalDirectoryError::Unavailable(format!("{error:?}"))),
        }
    }
}
