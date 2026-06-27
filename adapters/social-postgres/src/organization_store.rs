//! PostgreSQL store for spaces, groups, channels, invitations, and bans.

use std::sync::Arc;

use im_domain_core::space::*;
use im_platform_contracts::ContractError;
use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;

use crate::{NoTls, postgres_pool_client, postgres_unavailable, run_postgres_io};

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
    pool: Arc<Pool<PostgresConnectionManager<NoTls>>>,
}

impl PostgresSpaceStore {
    pub fn new(pool: Arc<Pool<PostgresConnectionManager<NoTls>>>) -> Self {
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
FROM im_chat_groups WHERE tenant_id = $1 AND organization_id = $2 AND space_id = $3 ORDER BY created_at DESC LIMIT $4
"#;

const GROUP_LIST_BY_OWNER_SQL: &str = r#"
SELECT tenant_id, organization_id, group_id, space_id, group_name, group_type, owner_user_id, conversation_id, max_members, description, avatar_url, announcement, settings_json, created_at, updated_at
FROM im_chat_groups WHERE tenant_id = $1 AND organization_id = $2 AND owner_user_id = $3 ORDER BY created_at DESC LIMIT $4
"#;

const GROUP_UPDATE_SQL: &str = r#"
UPDATE im_chat_groups SET group_name = $4, description = $5, avatar_url = $6, announcement = $7, max_members = $8, settings_json = $9, updated_at = $10
WHERE tenant_id = $1 AND organization_id = $2 AND group_id = $3
"#;

const GROUP_DELETE_SQL: &str = r#"
DELETE FROM im_chat_groups WHERE tenant_id = $1 AND organization_id = $2 AND group_id = $3
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
    pool: Arc<Pool<PostgresConnectionManager<NoTls>>>,
}

impl PostgresGroupStore {
    pub fn new(pool: Arc<Pool<PostgresConnectionManager<NoTls>>>) -> Self {
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
        limit: i64,
    ) -> Result<Vec<GroupRecord>, ContractError> {
        let pool = self.pool.clone();
        let tid = tenant_id.to_string();
        let oid = org_id.to_string();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "list_groups_by_space")?;
            let rows = client
                .query(GROUP_LIST_BY_SPACE_SQL, &[&tid, &oid, &space_id, &limit])
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

    fn delete(&self, tenant_id: &str, org_id: &str, group_id: i64) -> Result<(), ContractError> {
        let pool = self.pool.clone();
        let tid = tenant_id.to_string();
        let oid = org_id.to_string();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "delete_group")?;
            client
                .execute(GROUP_DELETE_SQL, &[&tid, &oid, &group_id])
                .map_err(|e| postgres_unavailable("delete_group", e))?;
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
FROM im_chat_channels WHERE tenant_id = $1 AND organization_id = $2 AND space_id = $3 ORDER BY position, channel_name LIMIT $4
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
    pool: Arc<Pool<PostgresConnectionManager<NoTls>>>,
}

impl PostgresChannelStore {
    pub fn new(pool: Arc<Pool<PostgresConnectionManager<NoTls>>>) -> Self {
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
        limit: i64,
    ) -> Result<Vec<ChannelRecord>, ContractError> {
        let pool = self.pool.clone();
        let tid = tenant_id.to_string();
        let oid = org_id.to_string();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "list_channels_by_space")?;
            let rows = client
                .query(CHANNEL_LIST_BY_SPACE_SQL, &[&tid, &oid, &space_id, &limit])
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
