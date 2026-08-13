//! PostgreSQL stores for space members, invitations, bans, and channel access rules.

use std::sync::Arc;

use im_platform_contracts::ContractError;
use r2d2::Pool;

use crate::member_capacity::MemberInsertOutcome;
use crate::{
    SocialPostgresConnectionManager, optional_postgres_timestamptz, postgres_pool_client,
    postgres_unavailable, run_postgres_io,
};

// ---------------------------------------------------------------------------
// Space member
// ---------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct SpaceMemberRecord {
    pub tenant_id: String,
    pub organization_id: String,
    pub space_id: i64,
    pub user_id: String,
    pub role: String,
    pub nickname: Option<String>,
    pub joined_at: String,
    pub updated_at: String,
}

pub trait SpaceMemberStore: Send + Sync {
    fn insert(&self, record: &SpaceMemberRecord) -> Result<(), ContractError>;
    fn get_by_id(
        &self,
        tenant_id: &str,
        org_id: &str,
        space_id: i64,
        user_id: &str,
    ) -> Result<Option<SpaceMemberRecord>, ContractError>;
    fn list_by_space(
        &self,
        tenant_id: &str,
        org_id: &str,
        space_id: i64,
        cursor_joined_at: Option<&str>,
        cursor_user_id: Option<&str>,
        limit: i64,
    ) -> Result<Vec<SpaceMemberRecord>, ContractError>;
    fn count_by_space(
        &self,
        tenant_id: &str,
        org_id: &str,
        space_id: i64,
    ) -> Result<i64, ContractError>;
    /// Insert only when current member count is below `max_members`.
    fn insert_within_capacity(
        &self,
        record: &SpaceMemberRecord,
        max_members: i32,
    ) -> Result<MemberInsertOutcome, ContractError>;
    fn update(&self, record: &SpaceMemberRecord) -> Result<(), ContractError>;
    fn delete(
        &self,
        tenant_id: &str,
        org_id: &str,
        space_id: i64,
        user_id: &str,
    ) -> Result<(), ContractError>;
    fn list_space_ids_by_user(
        &self,
        tenant_id: &str,
        org_id: &str,
        user_id: &str,
        limit: i64,
    ) -> Result<Vec<i64>, ContractError>;
}

const SPACE_MEMBER_INSERT_SQL: &str = r#"
INSERT INTO im_space_members (
    tenant_id, organization_id, space_id, user_id, role, nickname, joined_at, updated_at
) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
ON CONFLICT (tenant_id, organization_id, space_id, user_id) DO NOTHING
"#;

const SPACE_MEMBER_RESERVE_CAPACITY_SQL: &str = r#"
WITH locked_space AS (
    SELECT max_members
    FROM im_spaces
    WHERE tenant_id = $1 AND organization_id = $2 AND space_id = $3
    FOR UPDATE
),
member_count AS (
    SELECT COUNT(*)::bigint AS current_count
    FROM im_space_members
    WHERE tenant_id = $1 AND organization_id = $2 AND space_id = $3
)
SELECT ls.max_members, mc.current_count
FROM locked_space ls, member_count mc
"#;

const SPACE_MEMBER_GET_SQL: &str = r#"
SELECT tenant_id, organization_id, space_id, user_id, role, nickname, joined_at::text, updated_at::text
FROM im_space_members
WHERE tenant_id = $1 AND organization_id = $2 AND space_id = $3 AND user_id = $4
"#;

const SPACE_MEMBER_LIST_SQL: &str = r#"
SELECT tenant_id, organization_id, space_id, user_id, role, nickname, joined_at::text, updated_at::text
FROM im_space_members
WHERE tenant_id = $1 AND organization_id = $2 AND space_id = $3
  AND ($4::timestamptz IS NULL OR (joined_at, user_id) > ($4::timestamptz, $5::text))
ORDER BY joined_at ASC, user_id ASC
LIMIT $6
"#;

const SPACE_MEMBER_COUNT_SQL: &str = r#"
SELECT COUNT(*) FROM im_space_members
WHERE tenant_id = $1 AND organization_id = $2 AND space_id = $3
"#;

const SPACE_MEMBER_UPDATE_SQL: &str = r#"
UPDATE im_space_members
SET role = $5, nickname = $6, updated_at = $7
WHERE tenant_id = $1 AND organization_id = $2 AND space_id = $3 AND user_id = $4
"#;

const SPACE_MEMBER_DELETE_SQL: &str = r#"
DELETE FROM im_space_members
WHERE tenant_id = $1 AND organization_id = $2 AND space_id = $3 AND user_id = $4
"#;

const SPACE_MEMBER_LIST_SPACE_IDS_SQL: &str = r#"
SELECT space_id
FROM im_space_members
WHERE tenant_id = $1 AND organization_id = $2 AND user_id = $3
ORDER BY joined_at DESC
LIMIT $4
"#;

fn row_to_space_member_record(row: &postgres::Row) -> SpaceMemberRecord {
    SpaceMemberRecord {
        tenant_id: row.get("tenant_id"),
        organization_id: row.get("organization_id"),
        space_id: row.get("space_id"),
        user_id: row.get("user_id"),
        role: row.get("role"),
        nickname: row.get("nickname"),
        joined_at: row.get("joined_at"),
        updated_at: row.get("updated_at"),
    }
}

#[derive(Clone)]
pub struct PostgresSpaceMemberStore {
    pool: Arc<Pool<SocialPostgresConnectionManager>>,
}

impl PostgresSpaceMemberStore {
    pub fn new(pool: Arc<Pool<SocialPostgresConnectionManager>>) -> Self {
        Self { pool }
    }
}

impl SpaceMemberStore for PostgresSpaceMemberStore {
    fn insert(&self, record: &SpaceMemberRecord) -> Result<(), ContractError> {
        let pool = self.pool.clone();
        let record = record.clone();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "insert_space_member")?;
            client
                .execute(
                    SPACE_MEMBER_INSERT_SQL,
                    &[
                        &record.tenant_id,
                        &record.organization_id,
                        &record.space_id,
                        &record.user_id,
                        &record.role,
                        &record.nickname,
                        &record.joined_at,
                        &record.updated_at,
                    ],
                )
                .map_err(|error| postgres_unavailable("insert_space_member", error))?;
            Ok(())
        })
    }

    fn get_by_id(
        &self,
        tenant_id: &str,
        org_id: &str,
        space_id: i64,
        user_id: &str,
    ) -> Result<Option<SpaceMemberRecord>, ContractError> {
        let pool = self.pool.clone();
        let tenant_id = tenant_id.to_owned();
        let org_id = org_id.to_owned();
        let user_id = user_id.to_owned();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "get_space_member")?;
            let row = client
                .query_opt(
                    SPACE_MEMBER_GET_SQL,
                    &[&tenant_id, &org_id, &space_id, &user_id],
                )
                .map_err(|error| postgres_unavailable("get_space_member", error))?;
            Ok(row.map(|row| row_to_space_member_record(&row)))
        })
    }

    fn list_by_space(
        &self,
        tenant_id: &str,
        org_id: &str,
        space_id: i64,
        cursor_joined_at: Option<&str>,
        cursor_user_id: Option<&str>,
        limit: i64,
    ) -> Result<Vec<SpaceMemberRecord>, ContractError> {
        let pool = self.pool.clone();
        let tenant_id = tenant_id.to_owned();
        let org_id = org_id.to_owned();
        let cursor_joined_at = cursor_joined_at.map(str::to_owned);
        let cursor_user_id = cursor_user_id.map(str::to_owned);
        let cursor_ts_parsed = match &cursor_joined_at {
            Some(ts) => Some(optional_postgres_timestamptz(
                Some(ts.as_str()),
                "cursor_joined_at",
            )?),
            None => None,
        };
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "list_space_members")?;
            let rows = client
                .query(
                    SPACE_MEMBER_LIST_SQL,
                    &[
                        &tenant_id,
                        &org_id,
                        &space_id,
                        &cursor_ts_parsed,
                        &cursor_user_id,
                        &limit,
                    ],
                )
                .map_err(|error| postgres_unavailable("list_space_members", error))?;
            Ok(rows.iter().map(row_to_space_member_record).collect())
        })
    }

    fn count_by_space(
        &self,
        tenant_id: &str,
        org_id: &str,
        space_id: i64,
    ) -> Result<i64, ContractError> {
        let pool = self.pool.clone();
        let tenant_id = tenant_id.to_owned();
        let org_id = org_id.to_owned();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "count_space_members")?;
            let row = client
                .query_one(SPACE_MEMBER_COUNT_SQL, &[&tenant_id, &org_id, &space_id])
                .map_err(|error| postgres_unavailable("count_space_members", error))?;
            Ok(row.get::<_, i64>(0))
        })
    }

    fn insert_within_capacity(
        &self,
        record: &SpaceMemberRecord,
        max_members: i32,
    ) -> Result<MemberInsertOutcome, ContractError> {
        let pool = self.pool.clone();
        let record = record.clone();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "insert_space_member_within_capacity")?;
            let mut transaction = client.transaction().map_err(|error| {
                postgres_unavailable("insert_space_member_within_capacity", error)
            })?;

            let existing = transaction
                .query_opt(
                    SPACE_MEMBER_GET_SQL,
                    &[
                        &record.tenant_id,
                        &record.organization_id,
                        &record.space_id,
                        &record.user_id,
                    ],
                )
                .map_err(|error| {
                    postgres_unavailable("insert_space_member_within_capacity", error)
                })?;
            if existing.is_some() {
                transaction.rollback().map_err(|error| {
                    postgres_unavailable("insert_space_member_within_capacity", error)
                })?;
                return Ok(MemberInsertOutcome::AlreadyExists);
            }

            let capacity_row = transaction
                .query_opt(
                    SPACE_MEMBER_RESERVE_CAPACITY_SQL,
                    &[&record.tenant_id, &record.organization_id, &record.space_id],
                )
                .map_err(|error| {
                    postgres_unavailable("insert_space_member_within_capacity", error)
                })?;
            let Some(capacity_row) = capacity_row else {
                transaction.rollback().map_err(|error| {
                    postgres_unavailable("insert_space_member_within_capacity", error)
                })?;
                return Err(ContractError::Invalid("space not found".to_owned()));
            };
            let space_max: i32 = capacity_row.get("max_members");
            let current_count: i64 = capacity_row.get("current_count");
            let effective_max = i32::min(space_max, max_members);
            if current_count >= i64::from(effective_max) {
                transaction.rollback().map_err(|error| {
                    postgres_unavailable("insert_space_member_within_capacity", error)
                })?;
                return Ok(MemberInsertOutcome::CapacityFull);
            }

            transaction
                .execute(
                    SPACE_MEMBER_INSERT_SQL,
                    &[
                        &record.tenant_id,
                        &record.organization_id,
                        &record.space_id,
                        &record.user_id,
                        &record.role,
                        &record.nickname,
                        &record.joined_at,
                        &record.updated_at,
                    ],
                )
                .map_err(|error| {
                    postgres_unavailable("insert_space_member_within_capacity", error)
                })?;
            transaction.commit().map_err(|error| {
                postgres_unavailable("insert_space_member_within_capacity", error)
            })?;
            Ok(MemberInsertOutcome::Inserted)
        })
    }

    fn update(&self, record: &SpaceMemberRecord) -> Result<(), ContractError> {
        let pool = self.pool.clone();
        let record = record.clone();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "update_space_member")?;
            client
                .execute(
                    SPACE_MEMBER_UPDATE_SQL,
                    &[
                        &record.tenant_id,
                        &record.organization_id,
                        &record.space_id,
                        &record.user_id,
                        &record.role,
                        &record.nickname,
                        &record.updated_at,
                    ],
                )
                .map_err(|error| postgres_unavailable("update_space_member", error))?;
            Ok(())
        })
    }

    fn delete(
        &self,
        tenant_id: &str,
        org_id: &str,
        space_id: i64,
        user_id: &str,
    ) -> Result<(), ContractError> {
        let pool = self.pool.clone();
        let tenant_id = tenant_id.to_owned();
        let org_id = org_id.to_owned();
        let user_id = user_id.to_owned();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "delete_space_member")?;
            client
                .execute(
                    SPACE_MEMBER_DELETE_SQL,
                    &[&tenant_id, &org_id, &space_id, &user_id],
                )
                .map_err(|error| postgres_unavailable("delete_space_member", error))?;
            Ok(())
        })
    }

    fn list_space_ids_by_user(
        &self,
        tenant_id: &str,
        org_id: &str,
        user_id: &str,
        limit: i64,
    ) -> Result<Vec<i64>, ContractError> {
        let pool = self.pool.clone();
        let tenant_id = tenant_id.to_owned();
        let org_id = org_id.to_owned();
        let user_id = user_id.to_owned();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "list_space_ids_by_user")?;
            let rows = client
                .query(
                    SPACE_MEMBER_LIST_SPACE_IDS_SQL,
                    &[&tenant_id, &org_id, &user_id, &limit],
                )
                .map_err(|error| postgres_unavailable("list_space_ids_by_user", error))?;
            Ok(rows
                .iter()
                .map(|row| row.get::<_, i64>("space_id"))
                .collect())
        })
    }
}

// ---------------------------------------------------------------------------
// Invitation
// ---------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct InvitationRecord {
    pub tenant_id: String,
    pub organization_id: String,
    pub invitation_id: i64,
    pub inviter_user_id: String,
    pub invitee_user_id: Option<String>,
    pub invitee_email: Option<String>,
    pub invitee_phone: Option<String>,
    pub target_type: String,
    pub target_id: i64,
    pub role: String,
    pub status: String,
    pub message: Option<String>,
    pub expires_at: Option<String>,
    pub accepted_at: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub retention_until: Option<String>,
}

#[derive(Clone, Copy, Debug)]
pub struct InvitationTargetListQuery<'a> {
    pub tenant_id: &'a str,
    pub organization_id: &'a str,
    pub target_type: &'a str,
    pub target_id: i64,
    pub status: Option<&'a str>,
    pub limit: i64,
    pub cursor_created_at: Option<&'a str>,
    pub cursor_invitation_id: Option<i64>,
}

pub trait InvitationStore: Send + Sync {
    fn insert(&self, record: &InvitationRecord) -> Result<(), ContractError>;
    fn get_by_id(
        &self,
        tenant_id: &str,
        org_id: &str,
        invitation_id: i64,
    ) -> Result<Option<InvitationRecord>, ContractError>;
    fn list_by_target(
        &self,
        query: InvitationTargetListQuery<'_>,
    ) -> Result<Vec<InvitationRecord>, ContractError>;
    fn update(&self, record: &InvitationRecord) -> Result<(), ContractError>;
}

const INVITATION_INSERT_SQL: &str = r#"
INSERT INTO im_invitations (
    tenant_id, organization_id, invitation_id, inviter_user_id,
    invitee_user_id, invitee_email, invitee_phone,
    target_type, target_id, role, status, message, expires_at, accepted_at,
    created_at, updated_at, retention_until
) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17)
"#;

const INVITATION_GET_SQL: &str = r#"
SELECT tenant_id, organization_id, invitation_id, inviter_user_id,
       invitee_user_id, invitee_email, invitee_phone,
       target_type, target_id, role, status, message,
       expires_at::text, accepted_at::text, created_at::text, updated_at::text,
       retention_until::text
FROM im_invitations
WHERE tenant_id = $1 AND organization_id = $2 AND invitation_id = $3
"#;

const INVITATION_LIST_BY_TARGET_SQL: &str = r#"
SELECT tenant_id, organization_id, invitation_id, inviter_user_id,
       invitee_user_id, invitee_email, invitee_phone,
       target_type, target_id, role, status, message,
       expires_at::text, accepted_at::text, created_at::text, updated_at::text,
       retention_until::text
FROM im_invitations
WHERE tenant_id = $1 AND organization_id = $2
  AND target_type = $3 AND target_id = $4
  AND ($5::text IS NULL OR status = $5)
  AND ($6::timestamptz IS NULL OR (created_at, invitation_id) < ($6::timestamptz, $7::int8))
ORDER BY created_at DESC, invitation_id DESC
LIMIT $8
"#;

const INVITATION_UPDATE_SQL: &str = r#"
UPDATE im_invitations
SET status = $4, accepted_at = $5, updated_at = $6
WHERE tenant_id = $1 AND organization_id = $2 AND invitation_id = $3
"#;

fn row_to_invitation_record(row: &postgres::Row) -> InvitationRecord {
    InvitationRecord {
        tenant_id: row.get("tenant_id"),
        organization_id: row.get("organization_id"),
        invitation_id: row.get("invitation_id"),
        inviter_user_id: row.get("inviter_user_id"),
        invitee_user_id: row.get("invitee_user_id"),
        invitee_email: row.get("invitee_email"),
        invitee_phone: row.get("invitee_phone"),
        target_type: row.get("target_type"),
        target_id: row.get("target_id"),
        role: row.get("role"),
        status: row.get("status"),
        message: row.get("message"),
        expires_at: row.get("expires_at"),
        accepted_at: row.get("accepted_at"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
        retention_until: row.get("retention_until"),
    }
}

#[derive(Clone)]
pub struct PostgresInvitationStore {
    pool: Arc<Pool<SocialPostgresConnectionManager>>,
}

impl PostgresInvitationStore {
    pub fn new(pool: Arc<Pool<SocialPostgresConnectionManager>>) -> Self {
        Self { pool }
    }
}

impl InvitationStore for PostgresInvitationStore {
    fn insert(&self, record: &InvitationRecord) -> Result<(), ContractError> {
        let pool = self.pool.clone();
        let record = record.clone();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "insert_invitation")?;
            client
                .execute(
                    INVITATION_INSERT_SQL,
                    &[
                        &record.tenant_id,
                        &record.organization_id,
                        &record.invitation_id,
                        &record.inviter_user_id,
                        &record.invitee_user_id,
                        &record.invitee_email,
                        &record.invitee_phone,
                        &record.target_type,
                        &record.target_id,
                        &record.role,
                        &record.status,
                        &record.message,
                        &record.expires_at,
                        &record.accepted_at,
                        &record.created_at,
                        &record.updated_at,
                        &record.retention_until,
                    ],
                )
                .map_err(|error| postgres_unavailable("insert_invitation", error))?;
            Ok(())
        })
    }

    fn get_by_id(
        &self,
        tenant_id: &str,
        org_id: &str,
        invitation_id: i64,
    ) -> Result<Option<InvitationRecord>, ContractError> {
        let pool = self.pool.clone();
        let tenant_id = tenant_id.to_owned();
        let org_id = org_id.to_owned();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "get_invitation")?;
            let row = client
                .query_opt(INVITATION_GET_SQL, &[&tenant_id, &org_id, &invitation_id])
                .map_err(|error| postgres_unavailable("get_invitation", error))?;
            Ok(row.map(|row| row_to_invitation_record(&row)))
        })
    }

    fn list_by_target(
        &self,
        query: InvitationTargetListQuery<'_>,
    ) -> Result<Vec<InvitationRecord>, ContractError> {
        let pool = self.pool.clone();
        let tenant_id = query.tenant_id.to_owned();
        let org_id = query.organization_id.to_owned();
        let target_type = query.target_type.to_owned();
        let target_id = query.target_id;
        let status = query.status.map(str::to_owned);
        let limit = query.limit;
        let cursor_created_at = query.cursor_created_at.map(str::to_owned);
        let cursor_invitation_id = query.cursor_invitation_id;
        let cursor_ts_parsed = match &cursor_created_at {
            Some(ts) => Some(optional_postgres_timestamptz(
                Some(ts.as_str()),
                "cursor_created_at",
            )?),
            None => None,
        };
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "list_invitations")?;
            let rows = client
                .query(
                    INVITATION_LIST_BY_TARGET_SQL,
                    &[
                        &tenant_id,
                        &org_id,
                        &target_type,
                        &target_id,
                        &status,
                        &cursor_ts_parsed,
                        &cursor_invitation_id,
                        &limit,
                    ],
                )
                .map_err(|error| postgres_unavailable("list_invitations", error))?;
            Ok(rows.iter().map(row_to_invitation_record).collect())
        })
    }

    fn update(&self, record: &InvitationRecord) -> Result<(), ContractError> {
        let pool = self.pool.clone();
        let record = record.clone();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "update_invitation")?;
            client
                .execute(
                    INVITATION_UPDATE_SQL,
                    &[
                        &record.tenant_id,
                        &record.organization_id,
                        &record.invitation_id,
                        &record.status,
                        &record.accepted_at,
                        &record.updated_at,
                    ],
                )
                .map_err(|error| postgres_unavailable("update_invitation", error))?;
            Ok(())
        })
    }
}

// ---------------------------------------------------------------------------
// Ban
// ---------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct BanRecord {
    pub tenant_id: String,
    pub organization_id: String,
    pub ban_id: i64,
    pub target_type: String,
    pub target_id: i64,
    pub banned_user_id: String,
    pub banned_by_user_id: String,
    pub reason: Option<String>,
    pub expires_at: Option<String>,
    pub unbanned_at: Option<String>,
    pub unbanned_by_user_id: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Clone, Copy, Debug)]
pub struct BanTargetListQuery<'a> {
    pub tenant_id: &'a str,
    pub organization_id: &'a str,
    pub target_type: &'a str,
    pub target_id: i64,
    pub cursor_created_at: Option<&'a str>,
    pub cursor_ban_id: Option<i64>,
    pub limit: i64,
}

pub trait BanStore: Send + Sync {
    fn insert(&self, record: &BanRecord) -> Result<(), ContractError>;
    fn get_active_by_user(
        &self,
        tenant_id: &str,
        org_id: &str,
        target_type: &str,
        target_id: i64,
        banned_user_id: &str,
    ) -> Result<Option<BanRecord>, ContractError>;
    fn list_active_by_target(
        &self,
        query: BanTargetListQuery<'_>,
    ) -> Result<Vec<BanRecord>, ContractError>;
    fn update(&self, record: &BanRecord) -> Result<(), ContractError>;
}

const BAN_INSERT_SQL: &str = r#"
INSERT INTO im_ban_records (
    tenant_id, organization_id, ban_id, target_type, target_id,
    banned_user_id, banned_by_user_id, reason, expires_at,
    unbanned_at, unbanned_by_user_id, created_at, updated_at
) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
"#;

const BAN_GET_ACTIVE_SQL: &str = r#"
SELECT tenant_id, organization_id, ban_id, target_type, target_id,
       banned_user_id, banned_by_user_id, reason,
       expires_at::text, unbanned_at::text, unbanned_by_user_id,
       created_at::text, updated_at::text
FROM im_ban_records
WHERE tenant_id = $1 AND organization_id = $2
  AND target_type = $3 AND target_id = $4 AND banned_user_id = $5
  AND unbanned_at IS NULL
  AND (expires_at IS NULL OR expires_at > NOW())
ORDER BY created_at DESC
LIMIT 1
"#;

const BAN_LIST_ACTIVE_SQL: &str = r#"
SELECT tenant_id, organization_id, ban_id, target_type, target_id,
       banned_user_id, banned_by_user_id, reason,
       expires_at::text, unbanned_at::text, unbanned_by_user_id,
       created_at::text, updated_at::text
FROM im_ban_records
WHERE tenant_id = $1 AND organization_id = $2
  AND target_type = $3 AND target_id = $4
  AND unbanned_at IS NULL
  AND (expires_at IS NULL OR expires_at > NOW())
  AND ($5::timestamptz IS NULL OR (created_at, ban_id) < ($5::timestamptz, $6::int8))
ORDER BY created_at DESC, ban_id DESC
LIMIT $7
"#;

const BAN_UPDATE_SQL: &str = r#"
UPDATE im_ban_records
SET unbanned_at = $4, unbanned_by_user_id = $5, updated_at = $6
WHERE tenant_id = $1 AND organization_id = $2 AND ban_id = $3
"#;

fn row_to_ban_record(row: &postgres::Row) -> BanRecord {
    BanRecord {
        tenant_id: row.get("tenant_id"),
        organization_id: row.get("organization_id"),
        ban_id: row.get("ban_id"),
        target_type: row.get("target_type"),
        target_id: row.get("target_id"),
        banned_user_id: row.get("banned_user_id"),
        banned_by_user_id: row.get("banned_by_user_id"),
        reason: row.get("reason"),
        expires_at: row.get("expires_at"),
        unbanned_at: row.get("unbanned_at"),
        unbanned_by_user_id: row.get("unbanned_by_user_id"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    }
}

#[derive(Clone)]
pub struct PostgresBanStore {
    pool: Arc<Pool<SocialPostgresConnectionManager>>,
}

impl PostgresBanStore {
    pub fn new(pool: Arc<Pool<SocialPostgresConnectionManager>>) -> Self {
        Self { pool }
    }
}

impl BanStore for PostgresBanStore {
    fn insert(&self, record: &BanRecord) -> Result<(), ContractError> {
        let pool = self.pool.clone();
        let record = record.clone();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "insert_ban")?;
            client
                .execute(
                    BAN_INSERT_SQL,
                    &[
                        &record.tenant_id,
                        &record.organization_id,
                        &record.ban_id,
                        &record.target_type,
                        &record.target_id,
                        &record.banned_user_id,
                        &record.banned_by_user_id,
                        &record.reason,
                        &record.expires_at,
                        &record.unbanned_at,
                        &record.unbanned_by_user_id,
                        &record.created_at,
                        &record.updated_at,
                    ],
                )
                .map_err(|error| postgres_unavailable("insert_ban", error))?;
            Ok(())
        })
    }

    fn get_active_by_user(
        &self,
        tenant_id: &str,
        org_id: &str,
        target_type: &str,
        target_id: i64,
        banned_user_id: &str,
    ) -> Result<Option<BanRecord>, ContractError> {
        let pool = self.pool.clone();
        let tenant_id = tenant_id.to_owned();
        let org_id = org_id.to_owned();
        let target_type = target_type.to_owned();
        let banned_user_id = banned_user_id.to_owned();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "get_active_ban")?;
            let row = client
                .query_opt(
                    BAN_GET_ACTIVE_SQL,
                    &[
                        &tenant_id,
                        &org_id,
                        &target_type,
                        &target_id,
                        &banned_user_id,
                    ],
                )
                .map_err(|error| postgres_unavailable("get_active_ban", error))?;
            Ok(row.map(|row| row_to_ban_record(&row)))
        })
    }

    fn list_active_by_target(
        &self,
        query: BanTargetListQuery<'_>,
    ) -> Result<Vec<BanRecord>, ContractError> {
        let pool = self.pool.clone();
        let tenant_id = query.tenant_id.to_owned();
        let organization_id = query.organization_id.to_owned();
        let target_type = query.target_type.to_owned();
        let target_id = query.target_id;
        let cursor_created_at = query.cursor_created_at.map(str::to_owned);
        let cursor_ban_id = query.cursor_ban_id;
        let limit = query.limit;
        let cursor_ts_parsed = match &cursor_created_at {
            Some(ts) => Some(optional_postgres_timestamptz(
                Some(ts.as_str()),
                "cursor_created_at",
            )?),
            None => None,
        };
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "list_active_bans")?;
            let rows = client
                .query(
                    BAN_LIST_ACTIVE_SQL,
                    &[
                        &tenant_id,
                        &organization_id,
                        &target_type,
                        &target_id,
                        &cursor_ts_parsed,
                        &cursor_ban_id,
                        &limit,
                    ],
                )
                .map_err(|error| postgres_unavailable("list_active_bans", error))?;
            Ok(rows.iter().map(row_to_ban_record).collect())
        })
    }

    fn update(&self, record: &BanRecord) -> Result<(), ContractError> {
        let pool = self.pool.clone();
        let record = record.clone();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "update_ban")?;
            client
                .execute(
                    BAN_UPDATE_SQL,
                    &[
                        &record.tenant_id,
                        &record.organization_id,
                        &record.ban_id,
                        &record.unbanned_at,
                        &record.unbanned_by_user_id,
                        &record.updated_at,
                    ],
                )
                .map_err(|error| postgres_unavailable("update_ban", error))?;
            Ok(())
        })
    }
}

// ---------------------------------------------------------------------------
// Channel access rule
// ---------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct ChannelAccessRuleRecord {
    pub tenant_id: String,
    pub organization_id: String,
    pub rule_id: i64,
    pub channel_id: i64,
    pub rule_type: String,
    pub principal_kind: Option<String>,
    pub principal_id: Option<String>,
    pub permission: String,
    pub created_at: String,
}

/// Decision produced by evaluating channel access rules for one permission.
///
/// - [`ChannelRuleDecision::Deny`] wins over any allow (fail-closed).
/// - [`ChannelRuleDecision::Allow`] means an explicit allow rule matched.
/// - [`ChannelRuleDecision::NoRule`] means no rule targets the principal and
///   permission; the caller keeps its membership-based default behavior.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChannelRuleDecision {
    Allow,
    Deny,
    NoRule,
}

pub trait ChannelAccessRuleStore: Send + Sync {
    fn insert(&self, record: &ChannelAccessRuleRecord) -> Result<(), ContractError>;
    fn list_by_channel(
        &self,
        tenant_id: &str,
        org_id: &str,
        channel_id: i64,
        cursor_created_at: Option<&str>,
        cursor_rule_id: Option<i64>,
        limit: i64,
    ) -> Result<Vec<ChannelAccessRuleRecord>, ContractError>;
    fn delete(&self, tenant_id: &str, org_id: &str, rule_id: i64) -> Result<(), ContractError>;
    /// Evaluates the rules that target `(channel_id, permission)` for the
    /// principal. Matching rules are exact principal, principal-kind-wide,
    /// or space-wide; any deny wins, otherwise the most specific allow
    /// decides, otherwise [`ChannelRuleDecision::NoRule`].
    fn effective_permission(
        &self,
        tenant_id: &str,
        org_id: &str,
        channel_id: i64,
        principal_kind: &str,
        principal_id: &str,
        permission: &str,
    ) -> Result<ChannelRuleDecision, ContractError>;
}

const ACCESS_RULE_INSERT_SQL: &str = r#"
INSERT INTO im_channel_access_rules (
    tenant_id, organization_id, rule_id, channel_id,
    rule_type, principal_kind, principal_id, permission, created_at
) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
"#;

const ACCESS_RULE_LIST_SQL: &str = r#"
SELECT tenant_id, organization_id, rule_id, channel_id,
       rule_type, principal_kind, principal_id, permission, created_at
FROM im_channel_access_rules
WHERE tenant_id = $1 AND organization_id = $2 AND channel_id = $3
  AND ($4::timestamptz IS NULL OR (created_at, rule_id) > ($4::timestamptz, $5::int8))
ORDER BY created_at ASC, rule_id ASC
LIMIT $6
"#;

const ACCESS_RULE_DELETE_SQL: &str = r#"
DELETE FROM im_channel_access_rules
WHERE tenant_id = $1 AND organization_id = $2 AND rule_id = $3
"#;

/// Loads the rules that could target the principal for one permission.
/// A rule matches when it is space-wide (both principal fields NULL),
/// principal-kind-wide (`principal_kind` equal, `principal_id` NULL), or
/// principal-exact (`principal_kind` and `principal_id` equal).
const ACCESS_RULE_EVALUATE_SQL: &str = r#"
SELECT rule_type, principal_kind, principal_id
FROM im_channel_access_rules
WHERE tenant_id = $1 AND organization_id = $2 AND channel_id = $3
  AND permission = $4
  AND (
        (principal_kind IS NULL AND principal_id IS NULL)
        OR (principal_kind = $5 AND principal_id IS NULL)
        OR (principal_kind = $5 AND principal_id = $6)
  )
"#;

fn row_to_access_rule_record(row: &postgres::Row) -> ChannelAccessRuleRecord {
    ChannelAccessRuleRecord {
        tenant_id: row.get("tenant_id"),
        organization_id: row.get("organization_id"),
        rule_id: row.get("rule_id"),
        channel_id: row.get("channel_id"),
        rule_type: row.get("rule_type"),
        principal_kind: row.get("principal_kind"),
        principal_id: row.get("principal_id"),
        permission: row.get("permission"),
        created_at: row.get("created_at"),
    }
}

#[derive(Clone)]
pub struct PostgresChannelAccessRuleStore {
    pool: Arc<Pool<SocialPostgresConnectionManager>>,
}

impl PostgresChannelAccessRuleStore {
    pub fn new(pool: Arc<Pool<SocialPostgresConnectionManager>>) -> Self {
        Self { pool }
    }
}

impl ChannelAccessRuleStore for PostgresChannelAccessRuleStore {
    fn insert(&self, record: &ChannelAccessRuleRecord) -> Result<(), ContractError> {
        let pool = self.pool.clone();
        let record = record.clone();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "insert_channel_access_rule")?;
            client
                .execute(
                    ACCESS_RULE_INSERT_SQL,
                    &[
                        &record.tenant_id,
                        &record.organization_id,
                        &record.rule_id,
                        &record.channel_id,
                        &record.rule_type,
                        &record.principal_kind,
                        &record.principal_id,
                        &record.permission,
                        &record.created_at,
                    ],
                )
                .map_err(|error| postgres_unavailable("insert_channel_access_rule", error))?;
            Ok(())
        })
    }

    fn list_by_channel(
        &self,
        tenant_id: &str,
        org_id: &str,
        channel_id: i64,
        cursor_created_at: Option<&str>,
        cursor_rule_id: Option<i64>,
        limit: i64,
    ) -> Result<Vec<ChannelAccessRuleRecord>, ContractError> {
        let pool = self.pool.clone();
        let tenant_id = tenant_id.to_owned();
        let org_id = org_id.to_owned();
        let cursor_created_at = cursor_created_at.map(str::to_owned);
        let cursor_ts_parsed = match &cursor_created_at {
            Some(ts) => Some(optional_postgres_timestamptz(
                Some(ts.as_str()),
                "cursor_created_at",
            )?),
            None => None,
        };
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "list_channel_access_rules")?;
            let rows = client
                .query(
                    ACCESS_RULE_LIST_SQL,
                    &[
                        &tenant_id,
                        &org_id,
                        &channel_id,
                        &cursor_ts_parsed,
                        &cursor_rule_id,
                        &limit,
                    ],
                )
                .map_err(|error| postgres_unavailable("list_channel_access_rules", error))?;
            Ok(rows.iter().map(row_to_access_rule_record).collect())
        })
    }

    fn delete(&self, tenant_id: &str, org_id: &str, rule_id: i64) -> Result<(), ContractError> {
        let pool = self.pool.clone();
        let tenant_id = tenant_id.to_owned();
        let org_id = org_id.to_owned();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "delete_channel_access_rule")?;
            client
                .execute(ACCESS_RULE_DELETE_SQL, &[&tenant_id, &org_id, &rule_id])
                .map_err(|error| postgres_unavailable("delete_channel_access_rule", error))?;
            Ok(())
        })
    }

    fn effective_permission(
        &self,
        tenant_id: &str,
        org_id: &str,
        channel_id: i64,
        principal_kind: &str,
        principal_id: &str,
        permission: &str,
    ) -> Result<ChannelRuleDecision, ContractError> {
        let pool = self.pool.clone();
        let tenant_id = tenant_id.to_owned();
        let org_id = org_id.to_owned();
        let principal_kind = principal_kind.to_owned();
        let principal_id = principal_id.to_owned();
        let permission = permission.to_owned();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "evaluate_channel_access_rules")?;
            let rows = client
                .query(
                    ACCESS_RULE_EVALUATE_SQL,
                    &[
                        &tenant_id,
                        &org_id,
                        &channel_id,
                        &permission,
                        &principal_kind,
                        &principal_id,
                    ],
                )
                .map_err(|error| postgres_unavailable("evaluate_channel_access_rules", error))?;
            let mut deny: Option<u8> = None;
            let mut allow: Option<u8> = None;
            for row in rows {
                let rule_type: String = row.get("rule_type");
                let rule_kind: Option<String> = row.get("principal_kind");
                let rule_principal: Option<String> = row.get("principal_id");
                // Exact principal (both set) > kind-wide > space-wide.
                let specificity = match (&rule_kind, &rule_principal) {
                    (Some(kind), Some(principal)) => {
                        if kind == &principal_kind && principal == &principal_id {
                            2
                        } else {
                            continue;
                        }
                    }
                    (Some(kind), None) if kind == &principal_kind => 1,
                    (None, None) => 0,
                    _ => continue,
                };
                if rule_type == "deny" {
                    deny = Some(deny.map_or(specificity, |current| current.max(specificity)));
                } else if rule_type == "allow" {
                    allow = Some(allow.map_or(specificity, |current| current.max(specificity)));
                }
            }
            // Deny wins regardless of specificity (fail-closed). Otherwise the
            // most specific allow decides; a tie keeps the existing allow.
            match deny {
                Some(_) => Ok(ChannelRuleDecision::Deny),
                None => match allow {
                    Some(_) => Ok(ChannelRuleDecision::Allow),
                    None => Ok(ChannelRuleDecision::NoRule),
                },
            }
        })
    }
}
