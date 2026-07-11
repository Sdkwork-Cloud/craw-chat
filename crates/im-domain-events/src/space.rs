use serde::{Deserialize, Serialize};

use crate::{AggregateType, CommitEnvelope, EventActor};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SpaceEventType {
    SpaceCreated,
    SpaceUpdated,
    SpaceDeleted,
    SpaceMemberJoined,
    SpaceMemberUpdated,
    SpaceMemberRemoved,
    GroupCreated,
    GroupUpdated,
    GroupDeleted,
    GroupMemberJoined,
    GroupMemberUpdated,
    GroupMemberRemoved,
    GroupOwnerTransferred,
}

impl SpaceEventType {
    pub fn as_wire_value(&self) -> &'static str {
        match self {
            Self::SpaceCreated => "space.created",
            Self::SpaceUpdated => "space.updated",
            Self::SpaceDeleted => "space.deleted",
            Self::SpaceMemberJoined => "space.member_joined",
            Self::SpaceMemberUpdated => "space.member_updated",
            Self::SpaceMemberRemoved => "space.member_removed",
            Self::GroupCreated => "group.created",
            Self::GroupUpdated => "group.updated",
            Self::GroupDeleted => "group.deleted",
            Self::GroupMemberJoined => "group.member_joined",
            Self::GroupMemberUpdated => "group.member_updated",
            Self::GroupMemberRemoved => "group.member_removed",
            Self::GroupOwnerTransferred => "group.owner_transferred",
        }
    }

    pub fn payload_schema(&self) -> &'static str {
        match self {
            Self::SpaceCreated => "space.space.created.v1",
            Self::SpaceUpdated => "space.space.updated.v1",
            Self::SpaceDeleted => "space.space.deleted.v1",
            Self::SpaceMemberJoined => "space.space_member.joined.v1",
            Self::SpaceMemberUpdated => "space.space_member.updated.v1",
            Self::SpaceMemberRemoved => "space.space_member.removed.v1",
            Self::GroupCreated => "space.group.created.v1",
            Self::GroupUpdated => "space.group.updated.v1",
            Self::GroupDeleted => "space.group.deleted.v1",
            Self::GroupMemberJoined => "space.group_member.joined.v1",
            Self::GroupMemberUpdated => "space.group_member.updated.v1",
            Self::GroupMemberRemoved => "space.group_member.removed.v1",
            Self::GroupOwnerTransferred => "space.group.owner_transferred.v1",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpaceCreatedPayload {
    pub space_id: String,
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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpaceUpdatedPayload {
    pub space_id: String,
    pub space_name: String,
    pub description: Option<String>,
    pub avatar_url: Option<String>,
    pub max_members: i32,
    pub settings_json: String,
    pub updated_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpaceDeletedPayload {
    pub space_id: String,
    pub deleted_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpaceMemberJoinedPayload {
    pub space_id: String,
    pub user_id: String,
    pub role: String,
    pub nickname: Option<String>,
    pub joined_at: String,
    pub updated_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpaceMemberUpdatedPayload {
    pub space_id: String,
    pub user_id: String,
    pub role: String,
    pub nickname: Option<String>,
    pub updated_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpaceMemberRemovedPayload {
    pub space_id: String,
    pub user_id: String,
    pub removed_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupCreatedPayload {
    pub group_id: String,
    pub space_id: Option<String>,
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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupUpdatedPayload {
    pub group_id: String,
    pub group_name: String,
    #[serde(default)]
    pub conversation_id: Option<String>,
    pub description: Option<String>,
    pub avatar_url: Option<String>,
    pub announcement: Option<String>,
    pub max_members: i32,
    pub settings_json: String,
    pub updated_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupDeletedPayload {
    pub group_id: String,
    pub deleted_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupMemberJoinedPayload {
    pub group_id: String,
    pub user_id: String,
    pub role: String,
    pub nickname: Option<String>,
    pub mute_until: Option<String>,
    pub joined_at: String,
    pub updated_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupMemberUpdatedPayload {
    pub group_id: String,
    pub user_id: String,
    pub role: String,
    pub nickname: Option<String>,
    pub mute_until: Option<String>,
    pub updated_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupMemberRemovedPayload {
    pub group_id: String,
    pub user_id: String,
    pub removed_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupOwnerTransferredPayload {
    pub group_id: String,
    pub current_owner_user_id: String,
    pub new_owner_user_id: String,
    pub transferred_at: String,
}

pub struct SpaceCommitEnvelopeInput<'a> {
    pub event_id: &'a str,
    pub tenant_id: &'a str,
    pub organization_id: &'a str,
    pub aggregate_type: AggregateType,
    pub aggregate_id: &'a str,
    pub event_type: SpaceEventType,
    pub ordering_seq: u64,
    pub actor: EventActor,
    pub occurred_at: &'a str,
    pub committed_at: &'a str,
    pub payload: &'a str,
}

pub fn space_commit_envelope(input: SpaceCommitEnvelopeInput<'_>) -> CommitEnvelope {
    let SpaceCommitEnvelopeInput {
        event_id,
        tenant_id,
        organization_id,
        aggregate_type,
        aggregate_id,
        event_type,
        ordering_seq,
        actor,
        occurred_at,
        committed_at,
        payload,
    } = input;
    let scope_type = aggregate_type.as_wire_value();
    CommitEnvelope {
        event_id: event_id.into(),
        tenant_id: tenant_id.into(),
        organization_id: crate::normalize_commit_organization_id(organization_id),
        event_type: event_type.as_wire_value().into(),
        event_version: 1,
        aggregate_type,
        aggregate_id: aggregate_id.into(),
        scope_type: scope_type.into(),
        scope_id: aggregate_id.into(),
        ordering_key: CommitEnvelope::ordering_key(tenant_id, aggregate_id),
        ordering_seq,
        causation_id: None,
        correlation_id: None,
        idempotency_key: None,
        actor,
        occurred_at: occurred_at.into(),
        committed_at: committed_at.into(),
        payload_schema: Some(event_type.payload_schema().into()),
        payload: payload.into(),
        retention_class: "standard".into(),
        audit_class: "space".into(),
    }
}
