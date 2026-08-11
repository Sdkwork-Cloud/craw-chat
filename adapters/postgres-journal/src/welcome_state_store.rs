//! Postgres-backed [`WelcomeStateStore`] using `im_user_settings`.

use im_platform_contracts::{ContractError, WelcomeSentRecord, WelcomeStateStore};
use postgres::types::Json;

use crate::{
    PostgresJournalPool, now_rfc3339, postgres_pool_client, postgres_row_get, postgres_timestamptz,
    postgres_unavailable, run_postgres_io,
};

const READ_WELCOME_SENT_SQL: &str = r#"
select setting_value
from im_user_settings
where tenant_id = $1 and organization_id = $2 and user_id = $3 and setting_key = 'welcome.sent'
"#;

const WRITE_WELCOME_SENT_SQL: &str = r#"
insert into im_user_settings (tenant_id, organization_id, user_id, setting_key, setting_value, updated_at)
values ($1, $2, $3, 'welcome.sent', $4::jsonb, $5::timestamptz)
on conflict (tenant_id, organization_id, user_id, setting_key) do update
set setting_value = excluded.setting_value, updated_at = excluded.updated_at
"#;

const USER_HAS_CONVERSATIONS_WITH_MESSAGES_SQL: &str = r#"
select exists (
    select 1
    from im_conversation_members m
    join im_conversations c
      on c.tenant_id = m.tenant_id
     and c.organization_id = m.organization_id
     and c.conversation_id = m.conversation_id
    where m.tenant_id = $1
      and m.organization_id = $2
      and m.principal_kind = 'user'
      and m.principal_id = $3
      and m.membership_state in ('invited', 'joined', 'linked')
      and c.conversation_id <> $4
      and c.message_count > 0
)
"#;

/// PostgreSQL implementation of [`WelcomeStateStore`].
#[derive(Clone)]
pub struct PostgresWelcomeStateStore {
    pool: PostgresJournalPool,
}

impl PostgresWelcomeStateStore {
    pub fn from_pool(pool: PostgresJournalPool) -> Self {
        Self { pool }
    }
}

impl WelcomeStateStore for PostgresWelcomeStateStore {
    fn read_welcome_sent(
        &self,
        tenant_id: &str,
        organization_id: &str,
        user_id: &str,
    ) -> Result<Option<WelcomeSentRecord>, ContractError> {
        let pool = self.pool.clone();
        let tenant_id = tenant_id.to_owned();
        let organization_id = organization_id.to_owned();
        let user_id = user_id.to_owned();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "read_welcome_sent")?;
            let rows = client
                .query(
                    READ_WELCOME_SENT_SQL,
                    &[&tenant_id, &organization_id, &user_id],
                )
                .map_err(|error| postgres_unavailable("read_welcome_sent", error))?;
            let Some(row) = rows.first() else {
                return Ok(None);
            };
            let payload: Json<WelcomeSentRecord> =
                postgres_row_get(row, 0, "read_welcome_sent", "setting_value")?;
            Ok(Some(payload.0))
        })
    }

    fn write_welcome_sent(&self, record: &WelcomeSentRecord) -> Result<(), ContractError> {
        let pool = self.pool.clone();
        // 必须以 JSON 原生类型绑定 `$4::jsonb`；`String` 的 ToSql 只接受
        // TEXT/VARCHAR 等类型，绑定 jsonb 参数会触发 "error serializing parameter"。
        let payload = Json(record.clone());
        let tenant_id = record.tenant_id.clone();
        let organization_id = record.organization_id.clone();
        let user_id = record.user_id.clone();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "write_welcome_sent")?;
            let updated_at = postgres_timestamptz(&now_rfc3339(), "updated_at")?;
            client
                .execute(
                    WRITE_WELCOME_SENT_SQL,
                    &[
                        &tenant_id,
                        &organization_id,
                        &user_id,
                        &payload,
                        &updated_at,
                    ],
                )
                .map_err(|error| postgres_unavailable("write_welcome_sent", error))?;
            Ok(())
        })
    }

    fn user_has_conversations_with_messages(
        &self,
        tenant_id: &str,
        organization_id: &str,
        user_id: &str,
        exclude_conversation_id: &str,
    ) -> Result<bool, ContractError> {
        let pool = self.pool.clone();
        let tenant_id = tenant_id.to_owned();
        let organization_id = organization_id.to_owned();
        let user_id = user_id.to_owned();
        let exclude_conversation_id = exclude_conversation_id.to_owned();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "user_has_conversations_with_messages")?;
            let row = client
                .query_one(
                    USER_HAS_CONVERSATIONS_WITH_MESSAGES_SQL,
                    &[
                        &tenant_id,
                        &organization_id,
                        &user_id,
                        &exclude_conversation_id,
                    ],
                )
                .map_err(|error| {
                    postgres_unavailable("user_has_conversations_with_messages", error)
                })?;
            Ok(row.get(0))
        })
    }
}
