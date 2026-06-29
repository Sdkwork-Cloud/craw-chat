//! PostgreSQL implementation of [`ConversationAggregateStore`] trait.
//!
//! Manages conversation members and read cursors with Snowflake IDs.

use im_platform_contracts::{
    ContractError, ConversationAggregateState, ConversationAggregateStore,
    ConversationMemberRecord, ReadCursorRecord,
};
use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;

use crate::{postgres_pool_client, postgres_unavailable, run_postgres_io, PostgresJournalTlsConnector};

pub type PostgresJournalPool = Pool<PostgresConnectionManager<PostgresJournalTlsConnector>>;

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
"#;

const LOAD_MEMBER_SQL: &str = r#"
select tenant_id, organization_id, conversation_id, principal_kind, principal_id,
    member_id, membership_role, membership_state, invited_by, joined_at, removed_at, attributes_json::text
from im_projection_conversation_members
where tenant_id = $1 and organization_id = $2 and conversation_id = $3
    and principal_kind = $4 and principal_id = $5
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
select tenant_id, organization_id, conversation_id, member_id, principal_kind, principal_id,
    read_seq, last_read_message_id, updated_at
from im_projection_read_cursors
where tenant_id = $1 and organization_id = $2 and conversation_id = $3
"#;

const LOAD_READ_CURSOR_SQL: &str = r#"
select tenant_id, organization_id, conversation_id, member_id, principal_kind, principal_id,
    read_seq, last_read_message_id, updated_at
from im_projection_read_cursors
where tenant_id = $1 and organization_id = $2 and conversation_id = $3 and member_id = $4
"#;

const UPSERT_READ_CURSOR_SQL: &str = r#"
insert into im_projection_read_cursors (
    tenant_id, organization_id, conversation_id, member_id, principal_kind, principal_id,
    read_seq, last_read_message_id, payload_json, payload_hash, created_at, updated_at
) values ($1, $2, $3, $4, $5, $6, $7, $8, '{}'::jsonb, '', $9, $9)
on conflict (tenant_id, organization_id, conversation_id, member_id)
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

fn row_to_member(row: &postgres::Row) -> ConversationMemberRecord {
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
        joined_at: row.get(9),
        removed_at: row.get(10),
        attributes_json: row.get(11),
    }
}

fn row_to_cursor(row: &postgres::Row) -> ReadCursorRecord {
    ReadCursorRecord {
        tenant_id: row.get(0),
        organization_id: row.get(1),
        conversation_id: row.get(2),
        member_id: row.get::<_, i64>(3),
        principal_kind: row.get(4),
        principal_id: row.get(5),
        read_seq: row.get::<_, i64>(6) as u64,
        last_read_message_id: row.get(7),
        updated_at: row.get(8),
    }
}

impl ConversationAggregateStore for PostgresAggregateStore {
    fn load_members(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
    ) -> Result<Vec<ConversationMemberRecord>, ContractError> {
        let pool = self.pool.clone();
        let tenant_id = tenant_id.to_owned();
        let organization_id = organization_id.to_owned();
        let conversation_id = conversation_id.to_owned();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "load_members")?;
            let rows = client
                .query(
                    LOAD_MEMBERS_SQL,
                    &[&tenant_id, &organization_id, &conversation_id],
                )
                .map_err(|error| postgres_unavailable("load_members", error))?;
            Ok(rows.iter().map(row_to_member).collect())
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

    fn upsert_member(&self, member: ConversationMemberRecord) -> Result<(), ContractError> {
        let pool = self.pool.clone();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "upsert_member")?;
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
                &member.joined_at,
                &member.joined_at,
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
            client
                .execute(
                    REMOVE_MEMBER_SQL,
                    &[
                        &tenant_id,
                        &organization_id,
                        &conversation_id,
                        &principal_kind,
                        &principal_id,
                        &removed_at,
                    ],
                )
                .map_err(|error| postgres_unavailable("remove_member", error))?;
            Ok(())
        })
    }

    fn load_read_cursors(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
    ) -> Result<Vec<ReadCursorRecord>, ContractError> {
        let pool = self.pool.clone();
        let tenant_id = tenant_id.to_owned();
        let organization_id = organization_id.to_owned();
        let conversation_id = conversation_id.to_owned();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "load_read_cursors")?;
            let rows = client
                .query(
                    LOAD_READ_CURSORS_SQL,
                    &[&tenant_id, &organization_id, &conversation_id],
                )
                .map_err(|error| postgres_unavailable("load_read_cursors", error))?;
            Ok(rows.iter().map(row_to_cursor).collect())
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
            let params: &[&(dyn postgres::types::ToSql + Sync)] = &[
                &cursor.tenant_id,
                &cursor.organization_id,
                &cursor.conversation_id,
                &cursor.member_id,
                &cursor.principal_kind,
                &cursor.principal_id,
                &read_seq_i64,
                &cursor.last_read_message_id,
                &cursor.updated_at,
            ];
            client
                .execute(UPSERT_READ_CURSOR_SQL, params)
                .map_err(|error| postgres_unavailable("upsert_read_cursor", error))?;
            Ok(())
        })
    }

    fn load_aggregate_state(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
    ) -> Result<ConversationAggregateState, ContractError> {
        let members = self.load_members(tenant_id, organization_id, conversation_id)?;
        let read_cursors = self.load_read_cursors(tenant_id, organization_id, conversation_id)?;
        // Get high watermark from message store - for now use 0
        let high_watermark = 0u64;
        Ok(ConversationAggregateState {
            tenant_id: tenant_id.to_owned(),
            organization_id: organization_id.to_owned(),
            conversation_id: conversation_id.to_owned(),
            members,
            read_cursors,
            high_watermark,
        })
    }

    fn allocate_member_id(&self) -> Result<i64, ContractError> {
        // Member ID should be generated by IdGenerator, not here
        // For now return a placeholder - this will be replaced
        Err(ContractError::UnsupportedCapability(
            "member_id allocation should use IdGenerator".into(),
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
