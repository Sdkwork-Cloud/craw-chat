//! Canonical PostgreSQL contact inventory assembled from normalized Social tables.

use std::sync::Arc;

use im_platform_contracts::ContractError;
use r2d2::Pool;

use crate::{
    SocialPostgresConnectionManager, optional_postgres_timestamptz, postgres_pool_client,
    postgres_unavailable, run_postgres_io,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ContactInventoryRecord {
    pub tenant_id: String,
    pub organization_id: String,
    pub owner_user_id: String,
    pub target_user_id: String,
    pub friendship_id: i64,
    pub established_at: String,
    pub updated_at: String,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
    pub direct_chat_id: Option<i64>,
    pub conversation_id: Option<String>,
    pub is_starred: bool,
    pub is_blocked: bool,
    pub remark: Option<String>,
}

#[derive(Clone, Copy, Debug)]
pub struct ContactInventoryQuery<'a> {
    pub tenant_id: &'a str,
    pub organization_id: &'a str,
    pub owner_user_id: &'a str,
    pub cursor_updated_at: Option<&'a str>,
    pub cursor_friendship_id: Option<i64>,
    pub limit: i64,
}

pub trait ContactInventoryStore: Send + Sync {
    fn list_contacts(
        &self,
        query: ContactInventoryQuery<'_>,
    ) -> Result<Vec<ContactInventoryRecord>, ContractError>;
}

const LIST_CONTACTS_SQL: &str = r#"
with friendship_page as (
    select
        f.tenant_id,
        f.organization_id,
        f.friendship_id,
        case
            when f.user_low_id = $3 then f.user_high_id
            else f.user_low_id
        end as target_user_id,
        coalesce(f.established_at, f.updated_at) as established_at,
        f.updated_at
    from im_friendships f
    where f.tenant_id = $1
      and f.organization_id = $2
      and f.status = 'active'
      and (f.user_low_id = $3 or f.user_high_id = $3)
      and (
        $4::timestamptz is null
        or f.updated_at < $4::timestamptz
        or (f.updated_at = $4::timestamptz and f.friendship_id > $5)
      )
    order by f.updated_at desc, f.friendship_id asc
    limit $6
)
select
    f.tenant_id,
    f.organization_id,
    $3::text as owner_user_id,
    f.target_user_id,
    f.friendship_id,
    f.established_at::text,
    f.updated_at::text,
    coalesce(preference.remark, profile.im_nickname) as display_name,
    profile.im_avatar_url,
    direct_chat.direct_chat_id,
    direct_chat.conversation_id,
    coalesce(preference.is_starred, false) as is_starred,
    (
        coalesce(preference.is_blocked, false)
        or exists (
            select 1
            from im_user_blocks block_record
            where block_record.tenant_id = f.tenant_id
              and block_record.organization_id = f.organization_id
              and block_record.blocker_user_id = $3
              and block_record.blocked_user_id = f.target_user_id
              and block_record.scope in ('all', 'friendship')
              and (block_record.expires_at is null or block_record.expires_at > now())
        )
    ) as is_blocked,
    preference.remark
from friendship_page f
left join lateral (
    select d.direct_chat_id, d.conversation_id
    from im_direct_chats d
    where d.tenant_id = f.tenant_id
      and d.organization_id = f.organization_id
      and d.status = 'active'
      and (
        (
          d.left_actor_kind = 'user'
          and d.left_actor_id = $3
          and d.right_actor_kind = 'user'
          and d.right_actor_id = f.target_user_id
        )
        or (
          d.right_actor_kind = 'user'
          and d.right_actor_id = $3
          and d.left_actor_kind = 'user'
          and d.left_actor_id = f.target_user_id
        )
      )
    order by d.updated_at desc, d.direct_chat_id asc
    limit 1
) direct_chat on true
left join im_user_profiles profile
  on profile.tenant_id = f.tenant_id
 and profile.organization_id = f.organization_id
 and profile.user_id = f.target_user_id
left join im_contact_preferences preference
  on preference.tenant_id = f.tenant_id
 and preference.organization_id = f.organization_id
 and preference.owner_user_id = $3
 and preference.target_user_id = f.target_user_id
order by f.updated_at desc, f.friendship_id asc
"#;

#[derive(Clone)]
pub struct PostgresContactInventoryStore {
    pool: Arc<Pool<SocialPostgresConnectionManager>>,
}

impl PostgresContactInventoryStore {
    pub fn new(pool: Arc<Pool<SocialPostgresConnectionManager>>) -> Self {
        Self { pool }
    }
}

impl ContactInventoryStore for PostgresContactInventoryStore {
    fn list_contacts(
        &self,
        query: ContactInventoryQuery<'_>,
    ) -> Result<Vec<ContactInventoryRecord>, ContractError> {
        if query.limit <= 0 || query.limit > 201 {
            return Err(ContractError::Invalid(
                "contact inventory query limit must be between 1 and 201".into(),
            ));
        }

        let pool = self.pool.clone();
        let tenant_id = query.tenant_id.to_owned();
        let organization_id = query.organization_id.to_owned();
        let owner_user_id = query.owner_user_id.to_owned();
        let cursor_updated_at = query.cursor_updated_at.map(str::to_owned);
        let cursor_friendship_id = query.cursor_friendship_id;
        let limit = query.limit;
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "list_contact_inventory")?;
            let cursor_updated_at = optional_postgres_timestamptz(
                cursor_updated_at.as_deref(),
                "contact_inventory_cursor_updated_at",
            )?;
            let rows = client
                .query(
                    LIST_CONTACTS_SQL,
                    &[
                        &tenant_id,
                        &organization_id,
                        &owner_user_id,
                        &cursor_updated_at,
                        &cursor_friendship_id,
                        &limit,
                    ],
                )
                .map_err(|error| postgres_unavailable("list_contact_inventory", error))?;

            Ok(rows
                .iter()
                .map(|row| ContactInventoryRecord {
                    tenant_id: row.get(0),
                    organization_id: row.get(1),
                    owner_user_id: row.get(2),
                    target_user_id: row.get(3),
                    friendship_id: row.get(4),
                    established_at: row.get(5),
                    updated_at: row.get(6),
                    display_name: row.get(7),
                    avatar_url: row.get(8),
                    direct_chat_id: row.get(9),
                    conversation_id: row.get(10),
                    is_starred: row.get(11),
                    is_blocked: row.get(12),
                    remark: row.get(13),
                })
                .collect())
        })
    }
}

#[cfg(test)]
mod tests {
    use super::LIST_CONTACTS_SQL;

    #[test]
    fn contact_inventory_sql_is_bounded_and_scope_safe() {
        let normalized = LIST_CONTACTS_SQL.to_ascii_lowercase();
        assert!(normalized.contains("f.tenant_id = $1"));
        assert!(normalized.contains("f.organization_id = $2"));
        assert!(normalized.contains("order by f.updated_at desc, f.friendship_id asc"));
        assert!(normalized.contains("limit $6"));
        assert!(normalized.contains("from im_friendships"));
        assert!(normalized.contains("from im_direct_chats"));
        assert!(!normalized.contains(["im", "projection", ""].join("_").as_str()));
    }
}
