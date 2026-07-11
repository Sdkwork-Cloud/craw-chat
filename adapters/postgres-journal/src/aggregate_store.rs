//! PostgreSQL implementation of [`ConversationAggregateStore`] trait.
//!
//! Manages conversation members and read cursors with Snowflake IDs.

use im_platform_contracts::{
    CONVERSATION_AGGREGATE_PAGE_SIZE_MAX, ContractError, ConversationAggregateStore,
    ConversationMemberPage, ConversationMemberPageCursor, ConversationMemberRecord, ReadCursorPage,
    ReadCursorPageCursor, ReadCursorRecord,
};

use crate::{
    PostgresJournalPool, postgres_pool_client, postgres_timestamptz, postgres_unavailable,
    run_postgres_io,
};

/// PostgreSQL implementation of [`ConversationAggregateStore`].
#[derive(Clone)]
pub struct PostgresAggregateStore {
    pool: PostgresJournalPool,
}

impl PostgresAggregateStore {
    pub fn from_pool(pool: PostgresJournalPool) -> Self {
        Self { pool }
    }
}

// SQL constants

const LOAD_MEMBERS_SQL: &str = r#"
select tenant_id, organization_id, conversation_id, principal_kind, principal_id,
    member_id, membership_role, membership_state, invited_by, joined_at, removed_at, attributes_json::text
from im_projection_conversation_members
where tenant_id = $1 and organization_id = $2 and conversation_id = $3
    and membership_state in ('joined', 'linked', 'invited')
    and ($4::text is null or (principal_kind, principal_id) > ($4, $5))
order by principal_kind asc, principal_id asc
limit $6
"#;

const LOAD_MEMBER_SQL: &str = r#"
select tenant_id, organization_id, conversation_id, principal_kind, principal_id,
    member_id, membership_role, membership_state, invited_by, joined_at, removed_at, attributes_json::text
from im_projection_conversation_members
where tenant_id = $1 and organization_id = $2 and conversation_id = $3
    and principal_kind = $4 and principal_id = $5
"#;

const LOAD_MEMBER_BY_ID_SQL: &str = r#"
select tenant_id, organization_id, conversation_id, principal_kind, principal_id,
    member_id, membership_role, membership_state, invited_by, joined_at, removed_at, attributes_json::text
from im_projection_conversation_members
where tenant_id = $1 and organization_id = $2 and conversation_id = $3
    and member_id = $4
"#;

const LOAD_EVENT_RECIPIENTS_SQL: &str = r#"
select tenant_id, organization_id, conversation_id, principal_kind, principal_id,
    member_id, membership_role, membership_state, invited_by, joined_at, removed_at, attributes_json::text
from im_projection_conversation_members
where tenant_id = $1 and organization_id = $2 and conversation_id = $3
    and membership_state in ('joined', 'linked')
    and ($4::text is null or (principal_kind, principal_id) > ($4, $5))
    and joined_at <= $6
order by principal_kind asc, principal_id asc
limit $7
"#;

const UPSERT_MEMBER_SQL: &str = r#"
insert into im_projection_conversation_members (
    tenant_id, organization_id, conversation_id, principal_kind, principal_id,
    member_id, membership_role, membership_state, invited_by, joined_at, payload_json, payload_hash, created_at, updated_at
) values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, '{}'::jsonb, '', $11, $11)
on conflict (tenant_id, organization_id, conversation_id, principal_kind, principal_id)
do update set
    member_id = excluded.member_id,
    membership_role = excluded.membership_role,
    membership_state = excluded.membership_state,
    invited_by = excluded.invited_by,
    joined_at = excluded.joined_at,
    updated_at = excluded.updated_at
"#;

const REMOVE_MEMBER_SQL: &str = r#"
update im_projection_conversation_members
set membership_state = 'removed', removed_at = $6, updated_at = $6
where tenant_id = $1 and organization_id = $2 and conversation_id = $3
    and principal_kind = $4 and principal_id = $5
"#;

const LOAD_READ_CURSORS_SQL: &str = r#"
select tenant_id, organization_id, conversation_id, member_id, device_id, principal_kind, principal_id,
    read_seq, last_read_message_id, updated_at
from im_projection_read_cursors
where tenant_id = $1 and organization_id = $2 and conversation_id = $3
    and ($4::bigint is null or (member_id, device_id) > ($4, $5))
order by member_id asc, device_id asc
limit $6
"#;

const LOAD_READ_CURSOR_SQL: &str = r#"
select tenant_id, organization_id, conversation_id, member_id, device_id, principal_kind, principal_id,
    read_seq, last_read_message_id, updated_at
from im_projection_read_cursors
where tenant_id = $1 and organization_id = $2 and conversation_id = $3 and member_id = $4 and device_id = ''
"#;

const UPSERT_READ_CURSOR_SQL: &str = r#"
insert into im_projection_read_cursors (
    tenant_id, organization_id, conversation_id, member_id, device_id, principal_kind, principal_id,
    read_seq, last_read_message_id, payload_json, payload_hash, created_at, updated_at
) values ($1, $2, $3, $4, $5, $6, $7, $8, $9, '{}'::jsonb, '', $10, $10)
on conflict (tenant_id, organization_id, conversation_id, member_id, device_id)
do update set
    read_seq = excluded.read_seq,
    last_read_message_id = excluded.last_read_message_id,
    updated_at = excluded.updated_at
"#;

const CONVERSATION_EXISTS_SQL: &str = r#"
select exists (
    select 1 from im_projection_conversation_members
    where tenant_id = $1 and organization_id = $2 and conversation_id = $3
        and membership_state in ('joined', 'linked')
)
"#;

const READ_HIGH_WATERMARK_SQL: &str = r#"
select coalesce(max(message_seq), 0) as high_watermark
from im_conversation_messages
where tenant_id = $1 and organization_id = $2 and conversation_id = $3
"#;

fn row_to_member(row: &postgres::Row) -> ConversationMemberRecord {
    let joined_at: chrono::DateTime<chrono::Utc> = row.get(9);
    let removed_at: Option<chrono::DateTime<chrono::Utc>> = row.get(10);
    ConversationMemberRecord {
        tenant_id: row.get(0),
        organization_id: row.get(1),
        conversation_id: row.get(2),
        principal_kind: row.get(3),
        principal_id: row.get(4),
        member_id: row.get::<_, i64>(5),
        membership_role: row.get(6),
        membership_state: row.get(7),
        invited_by: row.get(8),
        joined_at: joined_at.to_rfc3339(),
        removed_at: removed_at.map(|dt| dt.to_rfc3339()),
        attributes_json: row.get(11),
    }
}

fn row_to_cursor(row: &postgres::Row) -> ReadCursorRecord {
    let updated_at: chrono::DateTime<chrono::Utc> = row.get(9);
    ReadCursorRecord {
        tenant_id: row.get(0),
        organization_id: row.get(1),
        conversation_id: row.get(2),
        member_id: row.get::<_, i64>(3),
        device_id: row.get(4),
        principal_kind: row.get(5),
        principal_id: row.get(6),
        read_seq: row.get::<_, i64>(7) as u64,
        last_read_message_id: row.get(8),
        updated_at: updated_at.to_rfc3339(),
    }
}

impl ConversationAggregateStore for PostgresAggregateStore {
    fn load_members_page(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
        cursor: Option<&ConversationMemberPageCursor>,
        page_size: usize,
    ) -> Result<ConversationMemberPage, ContractError> {
        let query_limit = validated_page_query_limit(page_size)?;
        let pool = self.pool.clone();
        let tenant_id = tenant_id.to_owned();
        let organization_id = organization_id.to_owned();
        let conversation_id = conversation_id.to_owned();
        let cursor_principal_kind = cursor.map(|cursor| cursor.principal_kind.clone());
        let cursor_principal_id = cursor.map(|cursor| cursor.principal_id.clone());
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "load_members_page")?;
            let rows = client
                .query(
                    LOAD_MEMBERS_SQL,
                    &[
                        &tenant_id,
                        &organization_id,
                        &conversation_id,
                        &cursor_principal_kind,
                        &cursor_principal_id,
                        &query_limit,
                    ],
                )
                .map_err(|error| postgres_unavailable("load_members_page", error))?;
            let mut items = rows.iter().map(row_to_member).collect::<Vec<_>>();
            let has_more = items.len() > page_size;
            if has_more {
                items.truncate(page_size);
            }
            let next_cursor = has_more.then(|| items.last()).flatten().map(|member| {
                ConversationMemberPageCursor {
                    principal_kind: member.principal_kind.clone(),
                    principal_id: member.principal_id.clone(),
                }
            });
            Ok(ConversationMemberPage {
                items,
                next_cursor,
                has_more,
            })
        })
    }

    fn load_member(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
        principal_kind: &str,
        principal_id: &str,
    ) -> Result<Option<ConversationMemberRecord>, ContractError> {
        let pool = self.pool.clone();
        let tenant_id = tenant_id.to_owned();
        let organization_id = organization_id.to_owned();
        let conversation_id = conversation_id.to_owned();
        let principal_kind = principal_kind.to_owned();
        let principal_id = principal_id.to_owned();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "load_member")?;
            let row = client
                .query_opt(
                    LOAD_MEMBER_SQL,
                    &[
                        &tenant_id,
                        &organization_id,
                        &conversation_id,
                        &principal_kind,
                        &principal_id,
                    ],
                )
                .map_err(|error| postgres_unavailable("load_member", error))?;
            Ok(row.map(|r| row_to_member(&r)))
        })
    }

    fn load_member_by_id(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
        member_id: i64,
    ) -> Result<Option<ConversationMemberRecord>, ContractError> {
        let pool = self.pool.clone();
        let tenant_id = tenant_id.to_owned();
        let organization_id = organization_id.to_owned();
        let conversation_id = conversation_id.to_owned();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "load_member_by_id")?;
            let row = client
                .query_opt(
                    LOAD_MEMBER_BY_ID_SQL,
                    &[&tenant_id, &organization_id, &conversation_id, &member_id],
                )
                .map_err(|error| postgres_unavailable("load_member_by_id", error))?;
            Ok(row.map(|row| row_to_member(&row)))
        })
    }

    fn load_event_recipients_page(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
        joined_before_or_at: &str,
        cursor: Option<&ConversationMemberPageCursor>,
        page_size: usize,
    ) -> Result<ConversationMemberPage, ContractError> {
        let query_limit = validated_page_query_limit(page_size)?;
        let pool = self.pool.clone();
        let tenant_id = tenant_id.to_owned();
        let organization_id = organization_id.to_owned();
        let conversation_id = conversation_id.to_owned();
        let joined_before_or_at = postgres_timestamptz(joined_before_or_at, "joined_before_or_at")?;
        let cursor_principal_kind = cursor.map(|cursor| cursor.principal_kind.clone());
        let cursor_principal_id = cursor.map(|cursor| cursor.principal_id.clone());
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "load_event_recipients_page")?;
            let rows = client
                .query(
                    LOAD_EVENT_RECIPIENTS_SQL,
                    &[
                        &tenant_id,
                        &organization_id,
                        &conversation_id,
                        &cursor_principal_kind,
                        &cursor_principal_id,
                        &joined_before_or_at,
                        &query_limit,
                    ],
                )
                .map_err(|error| postgres_unavailable("load_event_recipients_page", error))?;
            let mut items = rows.iter().map(row_to_member).collect::<Vec<_>>();
            let has_more = items.len() > page_size;
            if has_more {
                items.truncate(page_size);
            }
            let next_cursor = has_more.then(|| items.last()).flatten().map(|member| {
                ConversationMemberPageCursor {
                    principal_kind: member.principal_kind.clone(),
                    principal_id: member.principal_id.clone(),
                }
            });
            Ok(ConversationMemberPage {
                items,
                next_cursor,
                has_more,
            })
        })
    }

    fn upsert_member(&self, member: ConversationMemberRecord) -> Result<(), ContractError> {
        let pool = self.pool.clone();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "upsert_member")?;
            let joined_at = postgres_timestamptz(member.joined_at.as_str(), "joined_at")?;
            let params: &[&(dyn postgres::types::ToSql + Sync)] = &[
                &member.tenant_id,
                &member.organization_id,
                &member.conversation_id,
                &member.principal_kind,
                &member.principal_id,
                &member.member_id,
                &member.membership_role,
                &member.membership_state,
                &member.invited_by,
                &joined_at,
                &joined_at,
            ];
            client
                .execute(UPSERT_MEMBER_SQL, params)
                .map_err(|error| postgres_unavailable("upsert_member", error))?;
            Ok(())
        })
    }

    fn remove_member(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
        principal_kind: &str,
        principal_id: &str,
        removed_at: &str,
    ) -> Result<(), ContractError> {
        let pool = self.pool.clone();
        let tenant_id = tenant_id.to_owned();
        let organization_id = organization_id.to_owned();
        let conversation_id = conversation_id.to_owned();
        let principal_kind = principal_kind.to_owned();
        let principal_id = principal_id.to_owned();
        let removed_at = removed_at.to_owned();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "remove_member")?;
            let removed_at_ts = postgres_timestamptz(removed_at.as_str(), "removed_at")?;
            client
                .execute(
                    REMOVE_MEMBER_SQL,
                    &[
                        &tenant_id,
                        &organization_id,
                        &conversation_id,
                        &principal_kind,
                        &principal_id,
                        &removed_at_ts,
                    ],
                )
                .map_err(|error| postgres_unavailable("remove_member", error))?;
            Ok(())
        })
    }

    fn load_read_cursors_page(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
        cursor: Option<&ReadCursorPageCursor>,
        page_size: usize,
    ) -> Result<ReadCursorPage, ContractError> {
        let query_limit = validated_page_query_limit(page_size)?;
        let pool = self.pool.clone();
        let tenant_id = tenant_id.to_owned();
        let organization_id = organization_id.to_owned();
        let conversation_id = conversation_id.to_owned();
        let cursor_member_id = cursor.map(|cursor| cursor.member_id);
        let cursor_device_id = cursor.map(|cursor| cursor.device_id.clone());
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "load_read_cursors_page")?;
            let rows = client
                .query(
                    LOAD_READ_CURSORS_SQL,
                    &[
                        &tenant_id,
                        &organization_id,
                        &conversation_id,
                        &cursor_member_id,
                        &cursor_device_id,
                        &query_limit,
                    ],
                )
                .map_err(|error| postgres_unavailable("load_read_cursors_page", error))?;
            let mut items = rows.iter().map(row_to_cursor).collect::<Vec<_>>();
            let has_more = items.len() > page_size;
            if has_more {
                items.truncate(page_size);
            }
            let next_cursor =
                has_more
                    .then(|| items.last())
                    .flatten()
                    .map(|cursor| ReadCursorPageCursor {
                        member_id: cursor.member_id,
                        device_id: cursor.device_id.clone(),
                    });
            Ok(ReadCursorPage {
                items,
                next_cursor,
                has_more,
            })
        })
    }

    fn load_read_cursor(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
        member_id: i64,
    ) -> Result<Option<ReadCursorRecord>, ContractError> {
        let pool = self.pool.clone();
        let tenant_id = tenant_id.to_owned();
        let organization_id = organization_id.to_owned();
        let conversation_id = conversation_id.to_owned();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "load_read_cursor")?;
            let row = client
                .query_opt(
                    LOAD_READ_CURSOR_SQL,
                    &[&tenant_id, &organization_id, &conversation_id, &member_id],
                )
                .map_err(|error| postgres_unavailable("load_read_cursor", error))?;
            Ok(row.map(|r| row_to_cursor(&r)))
        })
    }

    fn upsert_read_cursor(&self, cursor: ReadCursorRecord) -> Result<(), ContractError> {
        let pool = self.pool.clone();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "upsert_read_cursor")?;
            let read_seq_i64 = cursor.read_seq as i64;
            let updated_at = postgres_timestamptz(cursor.updated_at.as_str(), "updated_at")?;
            let params: &[&(dyn postgres::types::ToSql + Sync)] = &[
                &cursor.tenant_id,
                &cursor.organization_id,
                &cursor.conversation_id,
                &cursor.member_id,
                &cursor.device_id,
                &cursor.principal_kind,
                &cursor.principal_id,
                &read_seq_i64,
                &cursor.last_read_message_id,
                &updated_at,
            ];
            client
                .execute(UPSERT_READ_CURSOR_SQL, params)
                .map_err(|error| postgres_unavailable("upsert_read_cursor", error))?;
            Ok(())
        })
    }

    fn load_high_watermark(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
    ) -> Result<u64, ContractError> {
        let pool = self.pool.clone();
        let tenant_id = tenant_id.to_owned();
        let organization_id = organization_id.to_owned();
        let conversation_id = conversation_id.to_owned();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "aggregate_load_high_watermark")?;
            let row = client
                .query_one(
                    READ_HIGH_WATERMARK_SQL,
                    &[&tenant_id, &organization_id, &conversation_id],
                )
                .map_err(|error| postgres_unavailable("aggregate_load_high_watermark", error))?;
            let high_watermark: i64 = row.get(0);
            Ok(high_watermark as u64)
        })
    }

    fn allocate_member_id(&self) -> Result<i64, ContractError> {
        Err(ContractError::UnsupportedCapability(
            "member_id allocation must use IdGenerator; PostgresAggregateStore does not allocate member ids".into(),
        ))
    }

    fn conversation_exists(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
    ) -> Result<bool, ContractError> {
        let pool = self.pool.clone();
        let tenant_id = tenant_id.to_owned();
        let organization_id = organization_id.to_owned();
        let conversation_id = conversation_id.to_owned();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "conversation_exists")?;
            let row = client
                .query_one(
                    CONVERSATION_EXISTS_SQL,
                    &[&tenant_id, &organization_id, &conversation_id],
                )
                .map_err(|error| postgres_unavailable("conversation_exists", error))?;
            let exists: bool = row.get(0);
            Ok(exists)
        })
    }
}

fn validated_page_query_limit(page_size: usize) -> Result<i64, ContractError> {
    if page_size == 0 || page_size > CONVERSATION_AGGREGATE_PAGE_SIZE_MAX {
        return Err(ContractError::Invalid(format!(
            "conversation aggregate page_size must be between 1 and {CONVERSATION_AGGREGATE_PAGE_SIZE_MAX}: {page_size}"
        )));
    }
    i64::try_from(page_size.saturating_add(1)).map_err(|error| {
        ContractError::Invalid(format!(
            "conversation aggregate page_size cannot convert to PostgreSQL limit: {error}"
        ))
    })
}

#[cfg(test)]
mod pagination_contract_tests {
    use super::*;

    #[test]
    fn member_pages_use_stable_keyset_and_sql_limit() {
        let normalized = LOAD_MEMBERS_SQL.to_ascii_lowercase();
        assert!(normalized.contains("(principal_kind, principal_id) >"));
        assert!(normalized.contains("order by principal_kind asc, principal_id asc"));
        assert!(normalized.contains("limit $6"));
        assert!(!normalized.contains(" offset "));
    }

    #[test]
    fn read_cursor_pages_use_stable_keyset_and_sql_limit() {
        let normalized = LOAD_READ_CURSORS_SQL.to_ascii_lowercase();
        assert!(normalized.contains("(member_id, device_id) >"));
        assert!(normalized.contains("order by member_id asc, device_id asc"));
        assert!(normalized.contains("limit $6"));
        assert!(!normalized.contains(" offset "));
    }

    #[test]
    fn event_recipient_pages_include_only_active_members_at_event_time() {
        let normalized = LOAD_EVENT_RECIPIENTS_SQL.to_ascii_lowercase();
        assert!(normalized.contains("membership_state in ('joined', 'linked')"));
        assert!(!normalized.contains("'invited'"));
        assert!(normalized.contains("joined_at <= $6"));
        assert!(normalized.contains("order by principal_kind asc, principal_id asc"));
        assert!(normalized.contains("limit $7"));
        assert!(!normalized.contains(" offset "));
    }
}
