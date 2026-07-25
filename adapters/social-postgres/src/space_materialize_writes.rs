//! Normalized Space and group writes executed inside the journal-owned transaction.

use im_domain_events::space::{
    GroupCreatedPayload, GroupDeletedPayload, GroupMemberJoinedPayload, GroupMemberRemovedPayload,
    GroupMemberUpdatedPayload, GroupOwnerTransferredPayload, GroupUpdatedPayload,
    SpaceCreatedPayload, SpaceDeletedPayload, SpaceMemberJoinedPayload, SpaceMemberRemovedPayload,
    SpaceMemberUpdatedPayload, SpaceUpdatedPayload,
};
use im_platform_contracts::CommitEnvelope;

use crate::governance_store::SpaceMemberRecord;
use crate::organization_store::{GroupMemberRecord, GroupRecord, SpaceRecord};
use crate::wire_id::parse_social_entity_id;

fn space_entity_id(value: &str) -> Result<i64, String> {
    parse_social_entity_id(value).map_err(|error| format!("{error:?}"))
}

const SPACE_MEMBER_CAPACITY_FULL: &str = "space member capacity full during materialization";
const GROUP_MEMBER_CAPACITY_FULL: &str = "group member capacity full during materialization";

#[derive(Debug, PartialEq, Eq)]
pub enum SpaceMaterializationError {
    CapacityFull,
    Persistence(String),
}

impl std::fmt::Display for SpaceMaterializationError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CapacityFull => formatter.write_str("space or group member capacity is full"),
            Self::Persistence(message) => formatter.write_str(message),
        }
    }
}

impl std::error::Error for SpaceMaterializationError {}

const SPACE_INSERT_SQL: &str = r#"
INSERT INTO im_spaces (tenant_id, organization_id, space_id, space_name, space_type, owner_user_id, description, avatar_url, max_members, settings_json, created_at, updated_at)
VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
ON CONFLICT (tenant_id, organization_id, space_id) DO NOTHING
"#;

const SPACE_GET_BY_ID_SQL: &str = r#"
SELECT tenant_id, organization_id, space_id, space_name, space_type, owner_user_id, description, avatar_url, max_members, settings_json, created_at::text, updated_at::text
FROM im_spaces WHERE tenant_id = $1 AND organization_id = $2 AND space_id = $3
"#;

const SPACE_UPDATE_SQL: &str = r#"
UPDATE im_spaces SET space_name = $4, description = $5, avatar_url = $6, max_members = $7, settings_json = $8, updated_at = $9
WHERE tenant_id = $1 AND organization_id = $2 AND space_id = $3
"#;

const SPACE_DELETE_SQL: &str = r#"
DELETE FROM im_spaces WHERE tenant_id = $1 AND organization_id = $2 AND space_id = $3
"#;

const SPACE_MEMBER_INSERT_SQL: &str = r#"
INSERT INTO im_space_members (
    tenant_id, organization_id, space_id, user_id, role, nickname, joined_at, updated_at
) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
ON CONFLICT (tenant_id, organization_id, space_id, user_id) DO NOTHING
"#;

const SPACE_MEMBER_LOCK_PARENT_SQL: &str = r#"
SELECT max_members
FROM im_spaces
WHERE tenant_id = $1 AND organization_id = $2 AND space_id = $3
FOR UPDATE
"#;

const SPACE_MEMBER_COUNT_SQL: &str = r#"
SELECT COUNT(*)::bigint AS current_count
FROM im_space_members
WHERE tenant_id = $1 AND organization_id = $2 AND space_id = $3
"#;

const SPACE_MEMBER_GET_SQL: &str = r#"
SELECT tenant_id, organization_id, space_id, user_id, role, nickname, joined_at::text, updated_at::text
FROM im_space_members
WHERE tenant_id = $1 AND organization_id = $2 AND space_id = $3 AND user_id = $4
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

const GROUP_INSERT_SQL: &str = r#"
INSERT INTO im_chat_groups (tenant_id, organization_id, group_id, space_id, group_name, group_type, owner_user_id, conversation_id, max_members, description, avatar_url, announcement, settings_json, created_at, updated_at)
VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15)
ON CONFLICT (tenant_id, organization_id, group_id) DO NOTHING
"#;

const GROUP_GET_BY_ID_SQL: &str = r#"
SELECT tenant_id, organization_id, group_id, space_id, group_name, group_type, owner_user_id, conversation_id, max_members, description, avatar_url, announcement, settings_json, created_at::text, updated_at::text
FROM im_chat_groups WHERE tenant_id = $1 AND organization_id = $2 AND group_id = $3
"#;

const GROUP_UPDATE_SQL: &str = r#"
UPDATE im_chat_groups SET group_name = $4, description = $5, avatar_url = $6, announcement = $7, max_members = $8, settings_json = $9, updated_at = $10
WHERE tenant_id = $1 AND organization_id = $2 AND group_id = $3
"#;

const GROUP_DELETE_SQL: &str = r#"
DELETE FROM im_chat_groups WHERE tenant_id = $1 AND organization_id = $2 AND group_id = $3
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

const GROUP_MEMBER_INSERT_SQL: &str = r#"
INSERT INTO im_group_members (
    tenant_id, organization_id, group_id, user_id, role, nickname, mute_until, joined_at, updated_at
) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
ON CONFLICT (tenant_id, organization_id, group_id, user_id) DO NOTHING
"#;

const GROUP_MEMBER_LOCK_PARENT_SQL: &str = r#"
SELECT max_members
FROM im_chat_groups
WHERE tenant_id = $1 AND organization_id = $2 AND group_id = $3
FOR UPDATE
"#;

const GROUP_MEMBER_COUNT_SQL: &str = r#"
SELECT COUNT(*)::bigint AS current_count
FROM im_group_members
WHERE tenant_id = $1 AND organization_id = $2 AND group_id = $3
"#;

const GROUP_MEMBER_GET_SQL: &str = r#"
SELECT tenant_id, organization_id, group_id, user_id, role, nickname, mute_until::text, joined_at::text, updated_at::text
FROM im_group_members
WHERE tenant_id = $1 AND organization_id = $2 AND group_id = $3 AND user_id = $4
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

pub fn materialize_space_commits_on_transaction(
    txn: &mut postgres::Transaction<'_>,
    commits: &[CommitEnvelope],
) -> Result<(), SpaceMaterializationError> {
    for commit in commits {
        materialize_space_commit_on(txn, commit).map_err(classify_materialization_error)?;
    }
    Ok(())
}

fn classify_materialization_error(error: String) -> SpaceMaterializationError {
    if error == SPACE_MEMBER_CAPACITY_FULL || error == GROUP_MEMBER_CAPACITY_FULL {
        SpaceMaterializationError::CapacityFull
    } else {
        SpaceMaterializationError::Persistence(error)
    }
}

fn materialize_space_commit_on(
    txn: &mut postgres::Transaction<'_>,
    commit: &CommitEnvelope,
) -> Result<(), String> {
    match commit.event_type.as_str() {
        "space.created" => materialize_space_created(txn, commit),
        "space.updated" => materialize_space_updated(txn, commit),
        "space.deleted" => materialize_space_deleted(txn, commit),
        "space.member_joined" => materialize_space_member_joined(txn, commit),
        "space.member_updated" => materialize_space_member_updated(txn, commit),
        "space.member_removed" => materialize_space_member_removed(txn, commit),
        "group.created" => materialize_group_created(txn, commit),
        "group.updated" => materialize_group_updated(txn, commit),
        "group.deleted" => materialize_group_deleted(txn, commit),
        "group.member_joined" => materialize_group_member_joined(txn, commit),
        "group.member_updated" => materialize_group_member_updated(txn, commit),
        "group.member_removed" => materialize_group_member_removed(txn, commit),
        "group.owner_transferred" => materialize_group_owner_transferred(txn, commit),
        event_type => Err(format!(
            "unsupported space normalized write event type {event_type}"
        )),
    }
}

fn materialize_space_created(
    txn: &mut postgres::Transaction<'_>,
    commit: &CommitEnvelope,
) -> Result<(), String> {
    let payload: SpaceCreatedPayload = serde_json::from_str(commit.payload.as_str())
        .map_err(|error| format!("invalid space.created payload: {error}"))?;
    txn.execute(
        SPACE_INSERT_SQL,
        &[
            &commit.tenant_id,
            &commit.organization_id,
            &space_entity_id(payload.space_id.as_str())?,
            &payload.space_name,
            &payload.space_type,
            &payload.owner_user_id,
            &payload.description,
            &payload.avatar_url,
            &payload.max_members,
            &payload.settings_json,
            &payload.created_at,
            &payload.updated_at,
        ],
    )
    .map_err(|error| format!("space insert failed: {error}"))
    .map(|_| ())
}

fn materialize_space_updated(
    txn: &mut postgres::Transaction<'_>,
    commit: &CommitEnvelope,
) -> Result<(), String> {
    let payload: SpaceUpdatedPayload = serde_json::from_str(commit.payload.as_str())
        .map_err(|error| format!("invalid space.updated payload: {error}"))?;
    let space_id = space_entity_id(payload.space_id.as_str())?;
    let existing = load_space(
        txn,
        commit.tenant_id.as_str(),
        commit.organization_id.as_str(),
        space_id,
    )?;
    txn.execute(
        SPACE_UPDATE_SQL,
        &[
            &commit.tenant_id,
            &commit.organization_id,
            &space_id,
            &payload.space_name,
            &payload.description,
            &payload.avatar_url,
            &payload.max_members,
            &payload.settings_json,
            &payload.updated_at,
        ],
    )
    .map_err(|error| format!("space update failed: {error}"))?;
    let _ = existing;
    Ok(())
}

fn materialize_space_deleted(
    txn: &mut postgres::Transaction<'_>,
    commit: &CommitEnvelope,
) -> Result<(), String> {
    let payload: SpaceDeletedPayload = serde_json::from_str(commit.payload.as_str())
        .map_err(|error| format!("invalid space.deleted payload: {error}"))?;
    txn.execute(
        SPACE_DELETE_SQL,
        &[
            &commit.tenant_id,
            &commit.organization_id,
            &space_entity_id(payload.space_id.as_str())?,
        ],
    )
    .map_err(|error| format!("space delete failed: {error}"))
    .map(|_| ())
}

fn materialize_space_member_joined(
    txn: &mut postgres::Transaction<'_>,
    commit: &CommitEnvelope,
) -> Result<(), String> {
    let payload: SpaceMemberJoinedPayload = serde_json::from_str(commit.payload.as_str())
        .map_err(|error| format!("invalid space.member_joined payload: {error}"))?;
    let record = SpaceMemberRecord {
        tenant_id: commit.tenant_id.clone(),
        organization_id: commit.organization_id.clone(),
        space_id: space_entity_id(payload.space_id.as_str())?,
        user_id: payload.user_id,
        role: payload.role,
        nickname: payload.nickname,
        joined_at: payload.joined_at,
        updated_at: payload.updated_at,
    };
    insert_space_member_within_capacity(txn, &record)
}

fn materialize_space_member_updated(
    txn: &mut postgres::Transaction<'_>,
    commit: &CommitEnvelope,
) -> Result<(), String> {
    let payload: SpaceMemberUpdatedPayload = serde_json::from_str(commit.payload.as_str())
        .map_err(|error| format!("invalid space.member_updated payload: {error}"))?;
    let space_id = space_entity_id(payload.space_id.as_str())?;
    let _existing = load_space_member(
        txn,
        commit.tenant_id.as_str(),
        commit.organization_id.as_str(),
        space_id,
        payload.user_id.as_str(),
    )?;
    txn.execute(
        SPACE_MEMBER_UPDATE_SQL,
        &[
            &commit.tenant_id,
            &commit.organization_id,
            &space_id,
            &payload.user_id,
            &payload.role,
            &payload.nickname,
            &payload.updated_at,
        ],
    )
    .map_err(|error| format!("space member update failed: {error}"))
    .map(|_| ())
}

fn materialize_space_member_removed(
    txn: &mut postgres::Transaction<'_>,
    commit: &CommitEnvelope,
) -> Result<(), String> {
    let payload: SpaceMemberRemovedPayload = serde_json::from_str(commit.payload.as_str())
        .map_err(|error| format!("invalid space.member_removed payload: {error}"))?;
    txn.execute(
        SPACE_MEMBER_DELETE_SQL,
        &[
            &commit.tenant_id,
            &commit.organization_id,
            &space_entity_id(payload.space_id.as_str())?,
            &payload.user_id,
        ],
    )
    .map_err(|error| format!("space member delete failed: {error}"))
    .map(|_| ())
}

fn materialize_group_created(
    txn: &mut postgres::Transaction<'_>,
    commit: &CommitEnvelope,
) -> Result<(), String> {
    let payload: GroupCreatedPayload = serde_json::from_str(commit.payload.as_str())
        .map_err(|error| format!("invalid group.created payload: {error}"))?;
    let group_id = space_entity_id(payload.group_id.as_str())?;
    let space_id = payload
        .space_id
        .as_deref()
        .map(space_entity_id)
        .transpose()?;
    txn.execute(
        GROUP_INSERT_SQL,
        &[
            &commit.tenant_id,
            &commit.organization_id,
            &group_id,
            &space_id,
            &payload.group_name,
            &payload.group_type,
            &payload.owner_user_id,
            &payload.conversation_id,
            &payload.max_members,
            &payload.description,
            &payload.avatar_url,
            &payload.announcement,
            &payload.settings_json,
            &payload.created_at,
            &payload.updated_at,
        ],
    )
    .map_err(|error| format!("group insert failed: {error}"))?;
    txn.execute(
        GROUP_MEMBER_INSERT_SQL,
        &[
            &commit.tenant_id,
            &commit.organization_id,
            &group_id,
            &payload.owner_user_id,
            &"owner".to_string(),
            &None::<String>,
            &None::<String>,
            &payload.created_at,
            &payload.updated_at,
        ],
    )
    .map_err(|error| format!("group owner member insert failed: {error}"))
    .map(|_| ())
}

fn materialize_group_updated(
    txn: &mut postgres::Transaction<'_>,
    commit: &CommitEnvelope,
) -> Result<(), String> {
    let payload: GroupUpdatedPayload = serde_json::from_str(commit.payload.as_str())
        .map_err(|error| format!("invalid group.updated payload: {error}"))?;
    let group_id = space_entity_id(payload.group_id.as_str())?;
    let _existing = load_group(
        txn,
        commit.tenant_id.as_str(),
        commit.organization_id.as_str(),
        group_id,
    )?;
    txn.execute(
        GROUP_UPDATE_SQL,
        &[
            &commit.tenant_id,
            &commit.organization_id,
            &group_id,
            &payload.group_name,
            &payload.description,
            &payload.avatar_url,
            &payload.announcement,
            &payload.max_members,
            &payload.settings_json,
            &payload.updated_at,
        ],
    )
    .map_err(|error| format!("group update failed: {error}"))
    .map(|_| ())
}

fn materialize_group_deleted(
    txn: &mut postgres::Transaction<'_>,
    commit: &CommitEnvelope,
) -> Result<(), String> {
    let payload: GroupDeletedPayload = serde_json::from_str(commit.payload.as_str())
        .map_err(|error| format!("invalid group.deleted payload: {error}"))?;
    txn.execute(
        GROUP_DELETE_SQL,
        &[
            &commit.tenant_id,
            &commit.organization_id,
            &space_entity_id(payload.group_id.as_str())?,
        ],
    )
    .map_err(|error| format!("group delete failed: {error}"))
    .map(|_| ())
}

fn materialize_group_member_joined(
    txn: &mut postgres::Transaction<'_>,
    commit: &CommitEnvelope,
) -> Result<(), String> {
    let payload: GroupMemberJoinedPayload = serde_json::from_str(commit.payload.as_str())
        .map_err(|error| format!("invalid group.member_joined payload: {error}"))?;
    let record = GroupMemberRecord {
        tenant_id: commit.tenant_id.clone(),
        organization_id: commit.organization_id.clone(),
        group_id: space_entity_id(payload.group_id.as_str())?,
        user_id: payload.user_id,
        role: payload.role,
        nickname: payload.nickname,
        mute_until: payload.mute_until,
        joined_at: payload.joined_at,
        updated_at: payload.updated_at,
    };
    insert_group_member_within_capacity(txn, &record)
}

fn materialize_group_member_updated(
    txn: &mut postgres::Transaction<'_>,
    commit: &CommitEnvelope,
) -> Result<(), String> {
    let payload: GroupMemberUpdatedPayload = serde_json::from_str(commit.payload.as_str())
        .map_err(|error| format!("invalid group.member_updated payload: {error}"))?;
    let group_id = space_entity_id(payload.group_id.as_str())?;
    let _existing = load_group_member(
        txn,
        commit.tenant_id.as_str(),
        commit.organization_id.as_str(),
        group_id,
        payload.user_id.as_str(),
    )?;
    txn.execute(
        GROUP_MEMBER_UPDATE_SQL,
        &[
            &commit.tenant_id,
            &commit.organization_id,
            &group_id,
            &payload.user_id,
            &payload.role,
            &payload.nickname,
            &payload.mute_until,
            &payload.updated_at,
        ],
    )
    .map_err(|error| format!("group member update failed: {error}"))
    .map(|_| ())
}

fn materialize_group_member_removed(
    txn: &mut postgres::Transaction<'_>,
    commit: &CommitEnvelope,
) -> Result<(), String> {
    let payload: GroupMemberRemovedPayload = serde_json::from_str(commit.payload.as_str())
        .map_err(|error| format!("invalid group.member_removed payload: {error}"))?;
    txn.execute(
        GROUP_MEMBER_DELETE_SQL,
        &[
            &commit.tenant_id,
            &commit.organization_id,
            &space_entity_id(payload.group_id.as_str())?,
            &payload.user_id,
        ],
    )
    .map_err(|error| format!("group member delete failed: {error}"))
    .map(|_| ())
}

fn materialize_group_owner_transferred(
    txn: &mut postgres::Transaction<'_>,
    commit: &CommitEnvelope,
) -> Result<(), String> {
    let payload: GroupOwnerTransferredPayload = serde_json::from_str(commit.payload.as_str())
        .map_err(|error| format!("invalid group.owner_transferred payload: {error}"))?;
    let group_id = space_entity_id(payload.group_id.as_str())?;
    let updated_rows = txn
        .execute(
            GROUP_TRANSFER_OWNER_SQL,
            &[
                &commit.tenant_id,
                &commit.organization_id,
                &group_id,
                &payload.new_owner_user_id,
                &payload.transferred_at,
                &payload.current_owner_user_id,
            ],
        )
        .map_err(|error| format!("group owner transfer failed: {error}"))?;
    if updated_rows == 0 {
        return Err("group owner mismatch or group not found".to_owned());
    }
    txn.execute(
        GROUP_MEMBER_DEMOTE_OWNER_SQL,
        &[
            &commit.tenant_id,
            &commit.organization_id,
            &group_id,
            &payload.current_owner_user_id,
            &payload.transferred_at,
        ],
    )
    .map_err(|error| format!("group owner demote failed: {error}"))?;
    let promoted_rows = txn
        .execute(
            GROUP_MEMBER_PROMOTE_OWNER_SQL,
            &[
                &commit.tenant_id,
                &commit.organization_id,
                &group_id,
                &payload.new_owner_user_id,
                &payload.transferred_at,
            ],
        )
        .map_err(|error| format!("group owner promote failed: {error}"))?;
    if promoted_rows == 0 {
        return Err("new owner must be an existing group member".to_owned());
    }
    Ok(())
}

fn load_space(
    txn: &mut postgres::Transaction<'_>,
    tenant_id: &str,
    organization_id: &str,
    space_id: i64,
) -> Result<SpaceRecord, String> {
    let row = txn
        .query_opt(
            SPACE_GET_BY_ID_SQL,
            &[&tenant_id, &organization_id, &space_id],
        )
        .map_err(|error| format!("space load failed: {error}"))?
        .ok_or_else(|| format!("space {space_id} not found"))?;
    Ok(SpaceRecord {
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
    })
}

fn load_space_member(
    txn: &mut postgres::Transaction<'_>,
    tenant_id: &str,
    organization_id: &str,
    space_id: i64,
    user_id: &str,
) -> Result<SpaceMemberRecord, String> {
    let row = txn
        .query_opt(
            SPACE_MEMBER_GET_SQL,
            &[&tenant_id, &organization_id, &space_id, &user_id],
        )
        .map_err(|error| format!("space member load failed: {error}"))?
        .ok_or_else(|| format!("space member {user_id} in space {space_id} not found"))?;
    Ok(SpaceMemberRecord {
        tenant_id: row.get("tenant_id"),
        organization_id: row.get("organization_id"),
        space_id: row.get("space_id"),
        user_id: row.get("user_id"),
        role: row.get("role"),
        nickname: row.get("nickname"),
        joined_at: row.get("joined_at"),
        updated_at: row.get("updated_at"),
    })
}

fn load_group(
    txn: &mut postgres::Transaction<'_>,
    tenant_id: &str,
    organization_id: &str,
    group_id: i64,
) -> Result<GroupRecord, String> {
    let row = txn
        .query_opt(
            GROUP_GET_BY_ID_SQL,
            &[&tenant_id, &organization_id, &group_id],
        )
        .map_err(|error| format!("group load failed: {error}"))?
        .ok_or_else(|| format!("group {group_id} not found"))?;
    Ok(GroupRecord {
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
    })
}

fn load_group_member(
    txn: &mut postgres::Transaction<'_>,
    tenant_id: &str,
    organization_id: &str,
    group_id: i64,
    user_id: &str,
) -> Result<GroupMemberRecord, String> {
    let row = txn
        .query_opt(
            GROUP_MEMBER_GET_SQL,
            &[&tenant_id, &organization_id, &group_id, &user_id],
        )
        .map_err(|error| format!("group member load failed: {error}"))?
        .ok_or_else(|| format!("group member {user_id} in group {group_id} not found"))?;
    Ok(GroupMemberRecord {
        tenant_id: row.get("tenant_id"),
        organization_id: row.get("organization_id"),
        group_id: row.get("group_id"),
        user_id: row.get("user_id"),
        role: row.get("role"),
        nickname: row.get("nickname"),
        mute_until: row.get("mute_until"),
        joined_at: row.get("joined_at"),
        updated_at: row.get("updated_at"),
    })
}

fn insert_space_member_within_capacity(
    txn: &mut postgres::Transaction<'_>,
    record: &SpaceMemberRecord,
) -> Result<(), String> {
    let parent_row = txn
        .query_opt(
            SPACE_MEMBER_LOCK_PARENT_SQL,
            &[&record.tenant_id, &record.organization_id, &record.space_id],
        )
        .map_err(|error| format!("space member parent lock failed: {error}"))?;
    let Some(parent_row) = parent_row else {
        return Err("space not found".to_owned());
    };
    let existing = txn
        .query_opt(
            SPACE_MEMBER_GET_SQL,
            &[
                &record.tenant_id,
                &record.organization_id,
                &record.space_id,
                &record.user_id,
            ],
        )
        .map_err(|error| format!("space member existence check failed: {error}"))?;
    if existing.is_some() {
        return Ok(());
    }
    let count_row = txn
        .query_one(
            SPACE_MEMBER_COUNT_SQL,
            &[&record.tenant_id, &record.organization_id, &record.space_id],
        )
        .map_err(|error| format!("space member capacity check failed: {error}"))?;
    let space_max: i32 = parent_row.get("max_members");
    let current_count: i64 = count_row.get("current_count");
    if current_count >= i64::from(space_max) {
        return Err(SPACE_MEMBER_CAPACITY_FULL.to_owned());
    }
    txn.execute(
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
    .map_err(|error| format!("space member insert failed: {error}"))
    .map(|_| ())
}

fn insert_group_member_within_capacity(
    txn: &mut postgres::Transaction<'_>,
    record: &GroupMemberRecord,
) -> Result<(), String> {
    let parent_row = txn
        .query_opt(
            GROUP_MEMBER_LOCK_PARENT_SQL,
            &[&record.tenant_id, &record.organization_id, &record.group_id],
        )
        .map_err(|error| format!("group member parent lock failed: {error}"))?;
    let Some(parent_row) = parent_row else {
        return Err("group not found".to_owned());
    };
    let existing = txn
        .query_opt(
            GROUP_MEMBER_GET_SQL,
            &[
                &record.tenant_id,
                &record.organization_id,
                &record.group_id,
                &record.user_id,
            ],
        )
        .map_err(|error| format!("group member existence check failed: {error}"))?;
    if existing.is_some() {
        return Ok(());
    }
    let count_row = txn
        .query_one(
            GROUP_MEMBER_COUNT_SQL,
            &[&record.tenant_id, &record.organization_id, &record.group_id],
        )
        .map_err(|error| format!("group member capacity check failed: {error}"))?;
    let group_max: i32 = parent_row.get("max_members");
    let current_count: i64 = count_row.get("current_count");
    if current_count >= i64::from(group_max) {
        return Err(GROUP_MEMBER_CAPACITY_FULL.to_owned());
    }
    txn.execute(
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
    .map_err(|error| format!("group member insert failed: {error}"))
    .map(|_| ())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn capacity_failures_remain_typed_at_the_transaction_boundary() {
        assert_eq!(
            classify_materialization_error(SPACE_MEMBER_CAPACITY_FULL.to_owned()),
            SpaceMaterializationError::CapacityFull
        );
        assert_eq!(
            classify_materialization_error(GROUP_MEMBER_CAPACITY_FULL.to_owned()),
            SpaceMaterializationError::CapacityFull
        );
        assert_eq!(
            classify_materialization_error("database unavailable".to_owned()),
            SpaceMaterializationError::Persistence("database unavailable".to_owned())
        );
    }
}
