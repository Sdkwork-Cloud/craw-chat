//! PostgreSQL store for spaces, groups, channels, invitations, and bans.

use std::sync::Arc;

use im_domain_core::space::*;
use im_platform_contracts::ContractError;
use r2d2::Pool;

use crate::member_capacity::MemberInsertOutcome;
use crate::{
    SocialPostgresConnectionManager, optional_postgres_timestamptz, postgres_pool_client,
    postgres_unavailable, run_postgres_io,
};

// ---------------------------------------------------------------------------
// Space Record
// ---------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct SpaceRecord {
    pub tenant_id: String,
    pub organization_id: String,
    pub space_id: i64,
    pub space_name: String,
    pub space_type: String,
    pub owner_user_id: String,
    pub description: Option<String>,
    pub avatar_url: Option<String>,
    pub max_members: i32,
    pub settings_json: String,
    pub created_at: String,
    pub updated_at: String,
}

impl SpaceRecord {
    pub fn to_domain(&self) -> Space {
        Space {
            tenant_id: self.tenant_id.clone(),
            organization_id: self.organization_id.clone(),
            space_id: self.space_id.to_string(),
            space_name: self.space_name.clone(),
            space_type: SpaceType::from_str(&self.space_type).unwrap_or(SpaceType::Organization),
            owner_user_id: self.owner_user_id.clone(),
            description: self.description.clone(),
            avatar_url: self.avatar_url.clone(),
            max_members: self.max_members,
            settings_json: self.settings_json.clone(),
            created_at: self.created_at.clone(),
            updated_at: self.updated_at.clone(),
        }
    }
}

// ---------------------------------------------------------------------------
// Group Record
// ---------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct GroupRecord {
    pub tenant_id: String,
    pub organization_id: String,
    pub group_id: i64,
    pub space_id: Option<i64>,
    pub group_name: String,
    pub group_type: String,
    pub owner_user_id: String,
    pub conversation_id: Option<String>,
    pub max_members: i32,
    pub description: Option<String>,
    pub avatar_url: Option<String>,
    pub announcement: Option<String>,
    pub settings_json: String,
    pub created_at: String,
    pub updated_at: String,
}

impl GroupRecord {
    pub fn to_domain(&self) -> ChatGroup {
        ChatGroup {
            tenant_id: self.tenant_id.clone(),
            organization_id: self.organization_id.clone(),
            group_id: self.group_id.to_string(),
            space_id: self.space_id.map(|s| s.to_string()),
            group_name: self.group_name.clone(),
            group_type: GroupType::from_str(&self.group_type).unwrap_or(GroupType::Normal),
            owner_user_id: self.owner_user_id.clone(),
            conversation_id: self.conversation_id.clone(),
            max_members: self.max_members,
            description: self.description.clone(),
            avatar_url: self.avatar_url.clone(),
            announcement: self.announcement.clone(),
            settings_json: self.settings_json.clone(),
            created_at: self.created_at.clone(),
            updated_at: self.updated_at.clone(),
        }
    }
}

// ---------------------------------------------------------------------------
// Channel Record
// ---------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct ChannelRecord {
    pub tenant_id: String,
    pub organization_id: String,
    pub channel_id: i64,
    pub space_id: i64,
    pub channel_name: String,
    pub channel_type: String,
    pub description: Option<String>,
    pub conversation_id: Option<String>,
    pub position: i32,
    pub is_nsfw: bool,
    pub is_pinned: bool,
    pub topic: Option<String>,
    pub settings_json: String,
    pub created_at: String,
    pub updated_at: String,
}

impl ChannelRecord {
    pub fn to_domain(&self) -> ChatChannel {
        ChatChannel {
            tenant_id: self.tenant_id.clone(),
            organization_id: self.organization_id.clone(),
            channel_id: self.channel_id.to_string(),
            space_id: self.space_id.to_string(),
            channel_name: self.channel_name.clone(),
            channel_type: ChannelType::from_str(&self.channel_type).unwrap_or(ChannelType::Text),
            description: self.description.clone(),
            conversation_id: self.conversation_id.clone(),
            position: self.position,
            is_nsfw: self.is_nsfw,
            is_pinned: self.is_pinned,
            topic: self.topic.clone(),
            settings_json: self.settings_json.clone(),
            created_at: self.created_at.clone(),
            updated_at: self.updated_at.clone(),
        }
    }
}

// ---------------------------------------------------------------------------
// Space Store Trait
// ---------------------------------------------------------------------------

pub trait SpaceStore: Send + Sync {
    fn insert(&self, record: &SpaceRecord) -> Result<(), ContractError>;
    fn get_by_id(
        &self,
        tenant_id: &str,
        org_id: &str,
        space_id: i64,
    ) -> Result<Option<SpaceRecord>, ContractError>;
    fn list_by_owner(
        &self,
        tenant_id: &str,
        org_id: &str,
        owner_user_id: &str,
        limit: i64,
    ) -> Result<Vec<SpaceRecord>, ContractError>;
    fn list_accessible_by_user(
        &self,
        tenant_id: &str,
        org_id: &str,
        user_id: &str,
        cursor_created_at: Option<&str>,
        cursor_space_id: Option<i64>,
        limit: i64,
    ) -> Result<Vec<SpaceRecord>, ContractError>;
    fn update(&self, record: &SpaceRecord) -> Result<(), ContractError>;
    fn delete(&self, tenant_id: &str, org_id: &str, space_id: i64) -> Result<(), ContractError>;
}

const SPACE_INSERT_SQL: &str = r#"
INSERT INTO im_spaces (tenant_id, organization_id, space_id, space_name, space_type, owner_user_id, description, avatar_url, max_members, settings_json, created_at, updated_at)
VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
ON CONFLICT (tenant_id, organization_id, space_id) DO NOTHING
"#;

const SPACE_GET_BY_ID_SQL: &str = r#"
SELECT tenant_id, organization_id, space_id, space_name, space_type, owner_user_id, description, avatar_url, max_members, settings_json, created_at, updated_at
FROM im_spaces WHERE tenant_id = $1 AND organization_id = $2 AND space_id = $3
"#;

const SPACE_LIST_BY_OWNER_SQL: &str = r#"
SELECT tenant_id, organization_id, space_id, space_name, space_type, owner_user_id, description, avatar_url, max_members, settings_json, created_at, updated_at
FROM im_spaces WHERE tenant_id = $1 AND organization_id = $2 AND owner_user_id = $3 ORDER BY created_at DESC LIMIT $4
"#;

const SPACE_LIST_ACCESSIBLE_BY_USER_SQL: &str = r#"
SELECT s.tenant_id, s.organization_id, s.space_id, s.space_name, s.space_type, s.owner_user_id, s.description, s.avatar_url, s.max_members, s.settings_json, s.created_at, s.updated_at
FROM im_spaces s
WHERE s.tenant_id = $1
  AND s.organization_id = $2
  AND (
    s.owner_user_id = $3
    OR s.space_id IN (
      SELECT m.space_id
      FROM im_space_members m
      WHERE m.tenant_id = $1 AND m.organization_id = $2 AND m.user_id = $3
    )
  )
  AND ($4::timestamptz IS NULL OR (s.created_at, s.space_id) < ($4::timestamptz, $5::int8))
ORDER BY s.created_at DESC, s.space_id DESC
LIMIT $6
"#;

const SPACE_UPDATE_SQL: &str = r#"
UPDATE im_spaces SET space_name = $4, description = $5, avatar_url = $6, max_members = $7, settings_json = $8, updated_at = $9
WHERE tenant_id = $1 AND organization_id = $2 AND space_id = $3
"#;

const SPACE_DELETE_SQL: &str = r#"
DELETE FROM im_spaces WHERE tenant_id = $1 AND organization_id = $2 AND space_id = $3
"#;

fn row_to_space_record(row: &postgres::Row) -> SpaceRecord {
    SpaceRecord {
        tenant_id: row.get("tenant_id"),
        organization_id: row.get("organization_id"),
        space_id: row.get("space_id"),
        space_name: row.get("space_name"),
        space_type: row.get("space_type"),
        owner_user_id: row.get("owner_user_id"),
        description: row.get("description"),
        avatar_url: row.get("avatar_url"),
        max_members: row.get("max_members"),
        settings_json: row.get("settings_json"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    }
}

/// PostgreSQL-backed space store.
#[derive(Clone)]
pub struct PostgresSpaceStore {
    pool: Arc<Pool<SocialPostgresConnectionManager>>,
}

impl PostgresSpaceStore {
    pub fn new(pool: Arc<Pool<SocialPostgresConnectionManager>>) -> Self {
        Self { pool }
    }
}

impl SpaceStore for PostgresSpaceStore {
    fn insert(&self, record: &SpaceRecord) -> Result<(), ContractError> {
        let pool = self.pool.clone();
        let r = record.clone();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "insert_space")?;
            client
                .execute(
                    SPACE_INSERT_SQL,
                    &[
                        &r.tenant_id,
                        &r.organization_id,
                        &r.space_id,
                        &r.space_name,
                        &r.space_type,
                        &r.owner_user_id,
                        &r.description,
                        &r.avatar_url,
                        &r.max_members,
                        &r.settings_json,
                        &r.created_at,
                        &r.updated_at,
                    ],
                )
                .map_err(|e| postgres_unavailable("insert_space", e))?;
            Ok(())
        })
    }

    fn get_by_id(
        &self,
        tenant_id: &str,
        org_id: &str,
        space_id: i64,
    ) -> Result<Option<SpaceRecord>, ContractError> {
        let pool = self.pool.clone();
        let tid = tenant_id.to_string();
        let oid = org_id.to_string();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "get_space")?;
            let row = client
                .query_opt(SPACE_GET_BY_ID_SQL, &[&tid, &oid, &space_id])
                .map_err(|e| postgres_unavailable("get_space", e))?;
            Ok(row.map(|r| row_to_space_record(&r)))
        })
    }

    fn list_by_owner(
        &self,
        tenant_id: &str,
        org_id: &str,
        owner_user_id: &str,
        limit: i64,
    ) -> Result<Vec<SpaceRecord>, ContractError> {
        let pool = self.pool.clone();
        let tid = tenant_id.to_string();
        let oid = org_id.to_string();
        let uid = owner_user_id.to_string();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "list_spaces_by_owner")?;
            let rows = client
                .query(SPACE_LIST_BY_OWNER_SQL, &[&tid, &oid, &uid, &limit])
                .map_err(|e| postgres_unavailable("list_spaces_by_owner", e))?;
            Ok(rows.iter().map(row_to_space_record).collect())
        })
    }

    fn list_accessible_by_user(
        &self,
        tenant_id: &str,
        org_id: &str,
        user_id: &str,
        cursor_created_at: Option<&str>,
        cursor_space_id: Option<i64>,
        limit: i64,
    ) -> Result<Vec<SpaceRecord>, ContractError> {
        let pool = self.pool.clone();
        let tid = tenant_id.to_string();
        let oid = org_id.to_string();
        let uid = user_id.to_string();
        let cursor_created_at = cursor_created_at.map(str::to_owned);
        let cursor_ts_parsed = match &cursor_created_at {
            Some(ts) => Some(optional_postgres_timestamptz(
                Some(ts.as_str()),
                "cursor_created_at",
            )?),
            None => None,
        };
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "list_spaces_accessible_by_user")?;
            let rows = client
                .query(
                    SPACE_LIST_ACCESSIBLE_BY_USER_SQL,
                    &[
                        &tid,
                        &oid,
                        &uid,
                        &cursor_ts_parsed,
                        &cursor_space_id,
                        &limit,
                    ],
                )
                .map_err(|e| postgres_unavailable("list_spaces_accessible_by_user", e))?;
            Ok(rows.iter().map(row_to_space_record).collect())
        })
    }

    fn update(&self, record: &SpaceRecord) -> Result<(), ContractError> {
        let pool = self.pool.clone();
        let r = record.clone();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "update_space")?;
            client
                .execute(
                    SPACE_UPDATE_SQL,
                    &[
                        &r.tenant_id,
                        &r.organization_id,
                        &r.space_id,
                        &r.space_name,
                        &r.description,
                        &r.avatar_url,
                        &r.max_members,
                        &r.settings_json,
                        &r.updated_at,
                    ],
                )
                .map_err(|e| postgres_unavailable("update_space", e))?;
            Ok(())
        })
    }

    fn delete(&self, tenant_id: &str, org_id: &str, space_id: i64) -> Result<(), ContractError> {
        let pool = self.pool.clone();
        let tid = tenant_id.to_string();
        let oid = org_id.to_string();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "delete_space")?;
            client
                .execute(SPACE_DELETE_SQL, &[&tid, &oid, &space_id])
                .map_err(|e| postgres_unavailable("delete_space", e))?;
            Ok(())
        })
    }
}

// ---------------------------------------------------------------------------
// Group Store Trait
// ---------------------------------------------------------------------------

pub trait GroupStore: Send + Sync {
    fn insert(&self, record: &GroupRecord) -> Result<(), ContractError>;
    fn insert_with_owner_member(
        &self,
        group: &GroupRecord,
        owner_member: &GroupMemberRecord,
    ) -> Result<(), ContractError>;
    fn get_by_id(
        &self,
        tenant_id: &str,
        org_id: &str,
        group_id: i64,
    ) -> Result<Option<GroupRecord>, ContractError>;
    fn list_by_space(
        &self,
        tenant_id: &str,
        org_id: &str,
        space_id: i64,
        cursor_created_at: Option<&str>,
        cursor_group_id: Option<i64>,
        limit: i64,
    ) -> Result<Vec<GroupRecord>, ContractError>;
    fn list_by_owner(
        &self,
        tenant_id: &str,
        org_id: &str,
        owner_user_id: &str,
        limit: i64,
    ) -> Result<Vec<GroupRecord>, ContractError>;
    fn update(&self, record: &GroupRecord) -> Result<(), ContractError>;
    fn transfer_owner(
        &self,
        tenant_id: &str,
        org_id: &str,
        group_id: i64,
        current_owner_user_id: &str,
        new_owner_user_id: &str,
        updated_at: &str,
    ) -> Result<GroupRecord, ContractError>;
    fn delete(&self, tenant_id: &str, org_id: &str, group_id: i64) -> Result<(), ContractError>;
}

const GROUP_INSERT_SQL: &str = r#"
INSERT INTO im_chat_groups (tenant_id, organization_id, group_id, space_id, group_name, group_type, owner_user_id, conversation_id, max_members, description, avatar_url, announcement, settings_json, created_at, updated_at)
VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15)
ON CONFLICT (tenant_id, organization_id, group_id) DO NOTHING
"#;

const GROUP_GET_BY_ID_SQL: &str = r#"
SELECT tenant_id, organization_id, group_id, space_id, group_name, group_type, owner_user_id, conversation_id, max_members, description, avatar_url, announcement, settings_json, created_at, updated_at
FROM im_chat_groups WHERE tenant_id = $1 AND organization_id = $2 AND group_id = $3
"#;

const GROUP_LIST_BY_SPACE_SQL: &str = r#"
SELECT tenant_id, organization_id, group_id, space_id, group_name, group_type, owner_user_id, conversation_id, max_members, description, avatar_url, announcement, settings_json, created_at, updated_at
FROM im_chat_groups WHERE tenant_id = $1 AND organization_id = $2 AND space_id = $3
  AND ($4::timestamptz IS NULL OR (created_at, group_id) < ($4::timestamptz, $5::int8))
ORDER BY created_at DESC, group_id DESC LIMIT $6
"#;

const GROUP_LIST_BY_OWNER_SQL: &str = r#"
SELECT tenant_id, organization_id, group_id, space_id, group_name, group_type, owner_user_id, conversation_id, max_members, description, avatar_url, announcement, settings_json, created_at, updated_at
FROM im_chat_groups WHERE tenant_id = $1 AND organization_id = $2 AND owner_user_id = $3 ORDER BY created_at DESC LIMIT $4
"#;

const GROUP_UPDATE_SQL: &str = r#"
UPDATE im_chat_groups SET group_name = $4, description = $5, avatar_url = $6, announcement = $7, max_members = $8, settings_json = $9, updated_at = $10
WHERE tenant_id = $1 AND organization_id = $2 AND group_id = $3
"#;

const GROUP_TRANSFER_OWNER_SQL: &str = r#"
UPDATE im_chat_groups
SET owner_user_id = $4, updated_at = $5
WHERE tenant_id = $1 AND organization_id = $2 AND group_id = $3 AND owner_user_id = $6
"#;

const GROUP_MEMBER_DEMOTE_OWNER_SQL: &str = r#"
UPDATE im_group_members
SET role = 'admin', updated_at = $5
WHERE tenant_id = $1 AND organization_id = $2 AND group_id = $3 AND user_id = $4 AND role = 'owner'
"#;

const GROUP_MEMBER_PROMOTE_OWNER_SQL: &str = r#"
UPDATE im_group_members
SET role = 'owner', updated_at = $5
WHERE tenant_id = $1 AND organization_id = $2 AND group_id = $3 AND user_id = $4
"#;

const GROUP_DELETE_SQL: &str = r#"
DELETE FROM im_chat_groups WHERE tenant_id = $1 AND organization_id = $2 AND group_id = $3
"#;

const GROUP_DELETE_MEMBERS_BY_GROUP_SQL: &str = r#"
DELETE FROM im_group_members
WHERE tenant_id = $1 AND organization_id = $2 AND group_id = $3
"#;

fn row_to_group_record(row: &postgres::Row) -> GroupRecord {
    GroupRecord {
        tenant_id: row.get("tenant_id"),
        organization_id: row.get("organization_id"),
        group_id: row.get("group_id"),
        space_id: row.get("space_id"),
        group_name: row.get("group_name"),
        group_type: row.get("group_type"),
        owner_user_id: row.get("owner_user_id"),
        conversation_id: row.get("conversation_id"),
        max_members: row.get("max_members"),
        description: row.get("description"),
        avatar_url: row.get("avatar_url"),
        announcement: row.get("announcement"),
        settings_json: row.get("settings_json"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    }
}

/// PostgreSQL-backed group store.
#[derive(Clone)]
pub struct PostgresGroupStore {
    pool: Arc<Pool<SocialPostgresConnectionManager>>,
}

impl PostgresGroupStore {
    pub fn new(pool: Arc<Pool<SocialPostgresConnectionManager>>) -> Self {
        Self { pool }
    }
}

impl GroupStore for PostgresGroupStore {
    fn insert(&self, record: &GroupRecord) -> Result<(), ContractError> {
        let pool = self.pool.clone();
        let r = record.clone();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "insert_group")?;
            client
                .execute(
                    GROUP_INSERT_SQL,
                    &[
                        &r.tenant_id,
                        &r.organization_id,
                        &r.group_id,
                        &r.space_id,
                        &r.group_name,
                        &r.group_type,
                        &r.owner_user_id,
                        &r.conversation_id,
                        &r.max_members,
                        &r.description,
                        &r.avatar_url,
                        &r.announcement,
                        &r.settings_json,
                        &r.created_at,
                        &r.updated_at,
                    ],
                )
                .map_err(|e| postgres_unavailable("insert_group", e))?;
            Ok(())
        })
    }

    fn insert_with_owner_member(
        &self,
        group: &GroupRecord,
        owner_member: &GroupMemberRecord,
    ) -> Result<(), ContractError> {
        let pool = self.pool.clone();
        let group_record = group.clone();
        let member_record = owner_member.clone();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "insert_group_with_owner")?;
            let mut transaction = client
                .transaction()
                .map_err(|e| postgres_unavailable("insert_group_with_owner_begin", e))?;
            transaction
                .execute(
                    GROUP_INSERT_SQL,
                    &[
                        &group_record.tenant_id,
                        &group_record.organization_id,
                        &group_record.group_id,
                        &group_record.space_id,
                        &group_record.group_name,
                        &group_record.group_type,
                        &group_record.owner_user_id,
                        &group_record.conversation_id,
                        &group_record.max_members,
                        &group_record.description,
                        &group_record.avatar_url,
                        &group_record.announcement,
                        &group_record.settings_json,
                        &group_record.created_at,
                        &group_record.updated_at,
                    ],
                )
                .map_err(|e| postgres_unavailable("insert_group_with_owner_group", e))?;
            transaction
                .execute(
                    GROUP_MEMBER_INSERT_SQL,
                    &[
                        &member_record.tenant_id,
                        &member_record.organization_id,
                        &member_record.group_id,
                        &member_record.user_id,
                        &member_record.role,
                        &member_record.nickname,
                        &member_record.mute_until,
                        &member_record.joined_at,
                        &member_record.updated_at,
                    ],
                )
                .map_err(|e| postgres_unavailable("insert_group_with_owner_member", e))?;
            transaction
                .commit()
                .map_err(|e| postgres_unavailable("insert_group_with_owner_commit", e))?;
            Ok(())
        })
    }

    fn get_by_id(
        &self,
        tenant_id: &str,
        org_id: &str,
        group_id: i64,
    ) -> Result<Option<GroupRecord>, ContractError> {
        let pool = self.pool.clone();
        let tid = tenant_id.to_string();
        let oid = org_id.to_string();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "get_group")?;
            let row = client
                .query_opt(GROUP_GET_BY_ID_SQL, &[&tid, &oid, &group_id])
                .map_err(|e| postgres_unavailable("get_group", e))?;
            Ok(row.map(|r| row_to_group_record(&r)))
        })
    }

    fn list_by_space(
        &self,
        tenant_id: &str,
        org_id: &str,
        space_id: i64,
        cursor_created_at: Option<&str>,
        cursor_group_id: Option<i64>,
        limit: i64,
    ) -> Result<Vec<GroupRecord>, ContractError> {
        let pool = self.pool.clone();
        let tid = tenant_id.to_string();
        let oid = org_id.to_string();
        let cursor_created_at = cursor_created_at.map(str::to_owned);
        let cursor_ts_parsed = match &cursor_created_at {
            Some(ts) => Some(optional_postgres_timestamptz(
                Some(ts.as_str()),
                "cursor_created_at",
            )?),
            None => None,
        };
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "list_groups_by_space")?;
            let rows = client
                .query(
                    GROUP_LIST_BY_SPACE_SQL,
                    &[
                        &tid,
                        &oid,
                        &space_id,
                        &cursor_ts_parsed,
                        &cursor_group_id,
                        &limit,
                    ],
                )
                .map_err(|e| postgres_unavailable("list_groups_by_space", e))?;
            Ok(rows.iter().map(row_to_group_record).collect())
        })
    }

    fn list_by_owner(
        &self,
        tenant_id: &str,
        org_id: &str,
        owner_user_id: &str,
        limit: i64,
    ) -> Result<Vec<GroupRecord>, ContractError> {
        let pool = self.pool.clone();
        let tid = tenant_id.to_string();
        let oid = org_id.to_string();
        let uid = owner_user_id.to_string();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "list_groups_by_owner")?;
            let rows = client
                .query(GROUP_LIST_BY_OWNER_SQL, &[&tid, &oid, &uid, &limit])
                .map_err(|e| postgres_unavailable("list_groups_by_owner", e))?;
            Ok(rows.iter().map(row_to_group_record).collect())
        })
    }

    fn update(&self, record: &GroupRecord) -> Result<(), ContractError> {
        let pool = self.pool.clone();
        let r = record.clone();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "update_group")?;
            client
                .execute(
                    GROUP_UPDATE_SQL,
                    &[
                        &r.tenant_id,
                        &r.organization_id,
                        &r.group_id,
                        &r.group_name,
                        &r.description,
                        &r.avatar_url,
                        &r.announcement,
                        &r.max_members,
                        &r.settings_json,
                        &r.updated_at,
                    ],
                )
                .map_err(|e| postgres_unavailable("update_group", e))?;
            Ok(())
        })
    }

    fn transfer_owner(
        &self,
        tenant_id: &str,
        org_id: &str,
        group_id: i64,
        current_owner_user_id: &str,
        new_owner_user_id: &str,
        updated_at: &str,
    ) -> Result<GroupRecord, ContractError> {
        let pool = self.pool.clone();
        let tid = tenant_id.to_string();
        let oid = org_id.to_string();
        let current_owner = current_owner_user_id.to_string();
        let new_owner = new_owner_user_id.to_string();
        let updated = updated_at.to_string();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "transfer_group_owner")?;
            let mut transaction = client
                .transaction()
                .map_err(|e| postgres_unavailable("transfer_group_owner_begin", e))?;
            let updated_rows = transaction
                .execute(
                    GROUP_TRANSFER_OWNER_SQL,
                    &[&tid, &oid, &group_id, &new_owner, &updated, &current_owner],
                )
                .map_err(|e| postgres_unavailable("transfer_group_owner_update", e))?;
            if updated_rows == 0 {
                return Err(ContractError::Conflict(
                    "group owner mismatch or group not found".to_owned(),
                ));
            }
            transaction
                .execute(
                    GROUP_MEMBER_DEMOTE_OWNER_SQL,
                    &[&tid, &oid, &group_id, &current_owner, &updated],
                )
                .map_err(|e| postgres_unavailable("transfer_group_owner_demote", e))?;
            let promoted_rows = transaction
                .execute(
                    GROUP_MEMBER_PROMOTE_OWNER_SQL,
                    &[&tid, &oid, &group_id, &new_owner, &updated],
                )
                .map_err(|e| postgres_unavailable("transfer_group_owner_promote", e))?;
            if promoted_rows == 0 {
                return Err(ContractError::Invalid(
                    "new owner must be an existing group member".to_owned(),
                ));
            }
            transaction
                .commit()
                .map_err(|e| postgres_unavailable("transfer_group_owner_commit", e))?;
            let row = client
                .query_opt(GROUP_GET_BY_ID_SQL, &[&tid, &oid, &group_id])
                .map_err(|e| postgres_unavailable("transfer_group_owner_reload", e))?
                .ok_or_else(|| {
                    ContractError::Unavailable("group missing after owner transfer".to_owned())
                })?;
            Ok(row_to_group_record(&row))
        })
    }

    fn delete(&self, tenant_id: &str, org_id: &str, group_id: i64) -> Result<(), ContractError> {
        let pool = self.pool.clone();
        let tid = tenant_id.to_string();
        let oid = org_id.to_string();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "delete_group")?;
            let mut transaction = client
                .transaction()
                .map_err(|e| postgres_unavailable("delete_group_begin", e))?;
            transaction
                .execute(GROUP_DELETE_MEMBERS_BY_GROUP_SQL, &[&tid, &oid, &group_id])
                .map_err(|e| postgres_unavailable("delete_group_members", e))?;
            transaction
                .execute(GROUP_DELETE_SQL, &[&tid, &oid, &group_id])
                .map_err(|e| postgres_unavailable("delete_group", e))?;
            transaction
                .commit()
                .map_err(|e| postgres_unavailable("delete_group_commit", e))?;
            Ok(())
        })
    }
}

// ---------------------------------------------------------------------------
// Channel Store Trait
// ---------------------------------------------------------------------------

pub trait ChannelStore: Send + Sync {
    fn insert(&self, record: &ChannelRecord) -> Result<(), ContractError>;
    fn get_by_id(
        &self,
        tenant_id: &str,
        org_id: &str,
        channel_id: i64,
    ) -> Result<Option<ChannelRecord>, ContractError>;
    fn list_by_space(
        &self,
        tenant_id: &str,
        org_id: &str,
        space_id: i64,
        cursor_created_at: Option<&str>,
        cursor_channel_id: Option<i64>,
        limit: i64,
    ) -> Result<Vec<ChannelRecord>, ContractError>;
    fn update(&self, record: &ChannelRecord) -> Result<(), ContractError>;
    fn delete(&self, tenant_id: &str, org_id: &str, channel_id: i64) -> Result<(), ContractError>;
}

const CHANNEL_INSERT_SQL: &str = r#"
INSERT INTO im_chat_channels (tenant_id, organization_id, channel_id, space_id, channel_name, channel_type, description, conversation_id, position, is_nsfw, is_pinned, topic, settings_json, created_at, updated_at)
VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15)
ON CONFLICT (tenant_id, organization_id, channel_id) DO NOTHING
"#;

const CHANNEL_GET_BY_ID_SQL: &str = r#"
SELECT tenant_id, organization_id, channel_id, space_id, channel_name, channel_type, description, conversation_id, position, is_nsfw, is_pinned, topic, settings_json, created_at, updated_at
FROM im_chat_channels WHERE tenant_id = $1 AND organization_id = $2 AND channel_id = $3
"#;

const CHANNEL_LIST_BY_SPACE_SQL: &str = r#"
SELECT tenant_id, organization_id, channel_id, space_id, channel_name, channel_type, description, conversation_id, position, is_nsfw, is_pinned, topic, settings_json, created_at, updated_at
FROM im_chat_channels WHERE tenant_id = $1 AND organization_id = $2 AND space_id = $3
  AND ($4::timestamptz IS NULL OR (created_at, channel_id) < ($4::timestamptz, $5::int8))
ORDER BY created_at DESC, channel_id DESC LIMIT $6
"#;

const CHANNEL_UPDATE_SQL: &str = r#"
UPDATE im_chat_channels SET channel_name = $4, description = $5, position = $6, is_nsfw = $7, is_pinned = $8, topic = $9, settings_json = $10, updated_at = $11
WHERE tenant_id = $1 AND organization_id = $2 AND channel_id = $3
"#;

const CHANNEL_DELETE_SQL: &str = r#"
DELETE FROM im_chat_channels WHERE tenant_id = $1 AND organization_id = $2 AND channel_id = $3
"#;

fn row_to_channel_record(row: &postgres::Row) -> ChannelRecord {
    ChannelRecord {
        tenant_id: row.get("tenant_id"),
        organization_id: row.get("organization_id"),
        channel_id: row.get("channel_id"),
        space_id: row.get("space_id"),
        channel_name: row.get("channel_name"),
        channel_type: row.get("channel_type"),
        description: row.get("description"),
        conversation_id: row.get("conversation_id"),
        position: row.get("position"),
        is_nsfw: row.get("is_nsfw"),
        is_pinned: row.get("is_pinned"),
        topic: row.get("topic"),
        settings_json: row.get("settings_json"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    }
}

/// PostgreSQL-backed channel store.
#[derive(Clone)]
pub struct PostgresChannelStore {
    pool: Arc<Pool<SocialPostgresConnectionManager>>,
}

impl PostgresChannelStore {
    pub fn new(pool: Arc<Pool<SocialPostgresConnectionManager>>) -> Self {
        Self { pool }
    }
}

impl ChannelStore for PostgresChannelStore {
    fn insert(&self, record: &ChannelRecord) -> Result<(), ContractError> {
        let pool = self.pool.clone();
        let r = record.clone();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "insert_channel")?;
            client
                .execute(
                    CHANNEL_INSERT_SQL,
                    &[
                        &r.tenant_id,
                        &r.organization_id,
                        &r.channel_id,
                        &r.space_id,
                        &r.channel_name,
                        &r.channel_type,
                        &r.description,
                        &r.conversation_id,
                        &r.position,
                        &r.is_nsfw,
                        &r.is_pinned,
                        &r.topic,
                        &r.settings_json,
                        &r.created_at,
                        &r.updated_at,
                    ],
                )
                .map_err(|e| postgres_unavailable("insert_channel", e))?;
            Ok(())
        })
    }

    fn get_by_id(
        &self,
        tenant_id: &str,
        org_id: &str,
        channel_id: i64,
    ) -> Result<Option<ChannelRecord>, ContractError> {
        let pool = self.pool.clone();
        let tid = tenant_id.to_string();
        let oid = org_id.to_string();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "get_channel")?;
            let row = client
                .query_opt(CHANNEL_GET_BY_ID_SQL, &[&tid, &oid, &channel_id])
                .map_err(|e| postgres_unavailable("get_channel", e))?;
            Ok(row.map(|r| row_to_channel_record(&r)))
        })
    }

    fn list_by_space(
        &self,
        tenant_id: &str,
        org_id: &str,
        space_id: i64,
        cursor_created_at: Option<&str>,
        cursor_channel_id: Option<i64>,
        limit: i64,
    ) -> Result<Vec<ChannelRecord>, ContractError> {
        let pool = self.pool.clone();
        let tid = tenant_id.to_string();
        let oid = org_id.to_string();
        let cursor_created_at = cursor_created_at.map(str::to_owned);
        let cursor_ts_parsed = match &cursor_created_at {
            Some(ts) => Some(optional_postgres_timestamptz(
                Some(ts.as_str()),
                "cursor_created_at",
            )?),
            None => None,
        };
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "list_channels_by_space")?;
            let rows = client
                .query(
                    CHANNEL_LIST_BY_SPACE_SQL,
                    &[
                        &tid,
                        &oid,
                        &space_id,
                        &cursor_ts_parsed,
                        &cursor_channel_id,
                        &limit,
                    ],
                )
                .map_err(|e| postgres_unavailable("list_channels_by_space", e))?;
            Ok(rows.iter().map(row_to_channel_record).collect())
        })
    }

    fn update(&self, record: &ChannelRecord) -> Result<(), ContractError> {
        let pool = self.pool.clone();
        let r = record.clone();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "update_channel")?;
            client
                .execute(
                    CHANNEL_UPDATE_SQL,
                    &[
                        &r.tenant_id,
                        &r.organization_id,
                        &r.channel_id,
                        &r.channel_name,
                        &r.description,
                        &r.position,
                        &r.is_nsfw,
                        &r.is_pinned,
                        &r.topic,
                        &r.settings_json,
                        &r.updated_at,
                    ],
                )
                .map_err(|e| postgres_unavailable("update_channel", e))?;
            Ok(())
        })
    }

    fn delete(&self, tenant_id: &str, org_id: &str, channel_id: i64) -> Result<(), ContractError> {
        let pool = self.pool.clone();
        let tid = tenant_id.to_string();
        let oid = org_id.to_string();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "delete_channel")?;
            client
                .execute(CHANNEL_DELETE_SQL, &[&tid, &oid, &channel_id])
                .map_err(|e| postgres_unavailable("delete_channel", e))?;
            Ok(())
        })
    }
}

// ---------------------------------------------------------------------------
// Group Member Record / Store
// ---------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct GroupMemberRecord {
    pub tenant_id: String,
    pub organization_id: String,
    pub group_id: i64,
    pub user_id: String,
    pub role: String,
    pub nickname: Option<String>,
    pub mute_until: Option<String>,
    pub joined_at: String,
    pub updated_at: String,
}

pub trait GroupMemberStore: Send + Sync {
    fn insert(&self, record: &GroupMemberRecord) -> Result<(), ContractError>;
    fn get_by_id(
        &self,
        tenant_id: &str,
        org_id: &str,
        group_id: i64,
        user_id: &str,
    ) -> Result<Option<GroupMemberRecord>, ContractError>;
    fn list_by_group(
        &self,
        tenant_id: &str,
        org_id: &str,
        group_id: i64,
        cursor_joined_at: Option<&str>,
        cursor_user_id: Option<&str>,
        limit: i64,
    ) -> Result<Vec<GroupMemberRecord>, ContractError>;
    fn count_by_group(
        &self,
        tenant_id: &str,
        org_id: &str,
        group_id: i64,
    ) -> Result<i64, ContractError>;
    fn insert_within_capacity(
        &self,
        record: &GroupMemberRecord,
        max_members: i32,
    ) -> Result<MemberInsertOutcome, ContractError>;
    fn update(&self, record: &GroupMemberRecord) -> Result<(), ContractError>;
    fn delete(
        &self,
        tenant_id: &str,
        org_id: &str,
        group_id: i64,
        user_id: &str,
    ) -> Result<(), ContractError>;
}

const GROUP_MEMBER_INSERT_SQL: &str = r#"
INSERT INTO im_group_members (
    tenant_id, organization_id, group_id, user_id, role, nickname, mute_until, joined_at, updated_at
) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
ON CONFLICT (tenant_id, organization_id, group_id, user_id) DO NOTHING
"#;

const GROUP_MEMBER_RESERVE_CAPACITY_SQL: &str = r#"
WITH locked_group AS (
    SELECT max_members
    FROM im_chat_groups
    WHERE tenant_id = $1 AND organization_id = $2 AND group_id = $3
    FOR UPDATE
),
member_count AS (
    SELECT COUNT(*)::bigint AS current_count
    FROM im_group_members
    WHERE tenant_id = $1 AND organization_id = $2 AND group_id = $3
)
SELECT lg.max_members, mc.current_count
FROM locked_group lg, member_count mc
"#;

const GROUP_MEMBER_GET_SQL: &str = r#"
SELECT tenant_id, organization_id, group_id, user_id, role, nickname, mute_until::text, joined_at, updated_at
FROM im_group_members
WHERE tenant_id = $1 AND organization_id = $2 AND group_id = $3 AND user_id = $4
"#;

const GROUP_MEMBER_LIST_SQL: &str = r#"
SELECT tenant_id, organization_id, group_id, user_id, role, nickname, mute_until::text, joined_at, updated_at
FROM im_group_members
WHERE tenant_id = $1 AND organization_id = $2 AND group_id = $3
  AND ($4::timestamptz IS NULL OR (joined_at, user_id) > ($4::timestamptz, $5::text))
ORDER BY joined_at ASC, user_id ASC
LIMIT $6
"#;

const GROUP_MEMBER_COUNT_SQL: &str = r#"
SELECT COUNT(*) FROM im_group_members
WHERE tenant_id = $1 AND organization_id = $2 AND group_id = $3
"#;

const GROUP_MEMBER_UPDATE_SQL: &str = r#"
UPDATE im_group_members
SET role = $5, nickname = $6, mute_until = $7, updated_at = $8
WHERE tenant_id = $1 AND organization_id = $2 AND group_id = $3 AND user_id = $4
"#;

const GROUP_MEMBER_DELETE_SQL: &str = r#"
DELETE FROM im_group_members
WHERE tenant_id = $1 AND organization_id = $2 AND group_id = $3 AND user_id = $4
"#;

fn row_to_group_member_record(row: &postgres::Row) -> GroupMemberRecord {
    GroupMemberRecord {
        tenant_id: row.get("tenant_id"),
        organization_id: row.get("organization_id"),
        group_id: row.get("group_id"),
        user_id: row.get("user_id"),
        role: row.get("role"),
        nickname: row.get("nickname"),
        mute_until: row.get("mute_until"),
        joined_at: row.get("joined_at"),
        updated_at: row.get("updated_at"),
    }
}

#[derive(Clone)]
pub struct PostgresGroupMemberStore {
    pool: Arc<Pool<SocialPostgresConnectionManager>>,
}

impl PostgresGroupMemberStore {
    pub fn new(pool: Arc<Pool<SocialPostgresConnectionManager>>) -> Self {
        Self { pool }
    }
}

impl GroupMemberStore for PostgresGroupMemberStore {
    fn insert(&self, record: &GroupMemberRecord) -> Result<(), ContractError> {
        let pool = self.pool.clone();
        let r = record.clone();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "insert_group_member")?;
            client
                .execute(
                    GROUP_MEMBER_INSERT_SQL,
                    &[
                        &r.tenant_id,
                        &r.organization_id,
                        &r.group_id,
                        &r.user_id,
                        &r.role,
                        &r.nickname,
                        &r.mute_until,
                        &r.joined_at,
                        &r.updated_at,
                    ],
                )
                .map_err(|e| postgres_unavailable("insert_group_member", e))?;
            Ok(())
        })
    }

    fn get_by_id(
        &self,
        tenant_id: &str,
        org_id: &str,
        group_id: i64,
        user_id: &str,
    ) -> Result<Option<GroupMemberRecord>, ContractError> {
        let pool = self.pool.clone();
        let tid = tenant_id.to_string();
        let oid = org_id.to_string();
        let uid = user_id.to_string();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "get_group_member")?;
            let row = client
                .query_opt(GROUP_MEMBER_GET_SQL, &[&tid, &oid, &group_id, &uid])
                .map_err(|e| postgres_unavailable("get_group_member", e))?;
            Ok(row.map(|row| row_to_group_member_record(&row)))
        })
    }

    fn list_by_group(
        &self,
        tenant_id: &str,
        org_id: &str,
        group_id: i64,
        cursor_joined_at: Option<&str>,
        cursor_user_id: Option<&str>,
        limit: i64,
    ) -> Result<Vec<GroupMemberRecord>, ContractError> {
        let pool = self.pool.clone();
        let tid = tenant_id.to_string();
        let oid = org_id.to_string();
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
            let mut client = postgres_pool_client(&pool, "list_group_members")?;
            let rows = client
                .query(
                    GROUP_MEMBER_LIST_SQL,
                    &[
                        &tid,
                        &oid,
                        &group_id,
                        &cursor_ts_parsed,
                        &cursor_user_id,
                        &limit,
                    ],
                )
                .map_err(|e| postgres_unavailable("list_group_members", e))?;
            Ok(rows.iter().map(row_to_group_member_record).collect())
        })
    }

    fn count_by_group(
        &self,
        tenant_id: &str,
        org_id: &str,
        group_id: i64,
    ) -> Result<i64, ContractError> {
        let pool = self.pool.clone();
        let tid = tenant_id.to_string();
        let oid = org_id.to_string();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "count_group_members")?;
            let row = client
                .query_one(GROUP_MEMBER_COUNT_SQL, &[&tid, &oid, &group_id])
                .map_err(|e| postgres_unavailable("count_group_members", e))?;
            Ok(row.get::<_, i64>(0))
        })
    }

    fn insert_within_capacity(
        &self,
        record: &GroupMemberRecord,
        max_members: i32,
    ) -> Result<MemberInsertOutcome, ContractError> {
        let pool = self.pool.clone();
        let record = record.clone();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "insert_group_member_within_capacity")?;
            let mut transaction = client.transaction().map_err(|error| {
                postgres_unavailable("insert_group_member_within_capacity", error)
            })?;

            let existing = transaction
                .query_opt(
                    GROUP_MEMBER_GET_SQL,
                    &[
                        &record.tenant_id,
                        &record.organization_id,
                        &record.group_id,
                        &record.user_id,
                    ],
                )
                .map_err(|error| {
                    postgres_unavailable("insert_group_member_within_capacity", error)
                })?;
            if existing.is_some() {
                transaction.rollback().map_err(|error| {
                    postgres_unavailable("insert_group_member_within_capacity", error)
                })?;
                return Ok(MemberInsertOutcome::AlreadyExists);
            }

            let capacity_row = transaction
                .query_opt(
                    GROUP_MEMBER_RESERVE_CAPACITY_SQL,
                    &[&record.tenant_id, &record.organization_id, &record.group_id],
                )
                .map_err(|error| {
                    postgres_unavailable("insert_group_member_within_capacity", error)
                })?;
            let Some(capacity_row) = capacity_row else {
                transaction.rollback().map_err(|error| {
                    postgres_unavailable("insert_group_member_within_capacity", error)
                })?;
                return Err(ContractError::Invalid("group not found".to_owned()));
            };
            let group_max: i32 = capacity_row.get("max_members");
            let current_count: i64 = capacity_row.get("current_count");
            let effective_max = i32::min(group_max, max_members);
            if current_count >= i64::from(effective_max) {
                transaction.rollback().map_err(|error| {
                    postgres_unavailable("insert_group_member_within_capacity", error)
                })?;
                return Ok(MemberInsertOutcome::CapacityFull);
            }

            transaction
                .execute(
                    GROUP_MEMBER_INSERT_SQL,
                    &[
                        &record.tenant_id,
                        &record.organization_id,
                        &record.group_id,
                        &record.user_id,
                        &record.role,
                        &record.nickname,
                        &record.mute_until,
                        &record.joined_at,
                        &record.updated_at,
                    ],
                )
                .map_err(|error| {
                    postgres_unavailable("insert_group_member_within_capacity", error)
                })?;
            transaction.commit().map_err(|error| {
                postgres_unavailable("insert_group_member_within_capacity", error)
            })?;
            Ok(MemberInsertOutcome::Inserted)
        })
    }

    fn update(&self, record: &GroupMemberRecord) -> Result<(), ContractError> {
        let pool = self.pool.clone();
        let r = record.clone();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "update_group_member")?;
            client
                .execute(
                    GROUP_MEMBER_UPDATE_SQL,
                    &[
                        &r.tenant_id,
                        &r.organization_id,
                        &r.group_id,
                        &r.user_id,
                        &r.role,
                        &r.nickname,
                        &r.mute_until,
                        &r.updated_at,
                    ],
                )
                .map_err(|e| postgres_unavailable("update_group_member", e))?;
            Ok(())
        })
    }

    fn delete(
        &self,
        tenant_id: &str,
        org_id: &str,
        group_id: i64,
        user_id: &str,
    ) -> Result<(), ContractError> {
        let pool = self.pool.clone();
        let tid = tenant_id.to_string();
        let oid = org_id.to_string();
        let uid = user_id.to_string();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "delete_group_member")?;
            client
                .execute(GROUP_MEMBER_DELETE_SQL, &[&tid, &oid, &group_id, &uid])
                .map_err(|e| postgres_unavailable("delete_group_member", e))?;
            Ok(())
        })
    }
}
