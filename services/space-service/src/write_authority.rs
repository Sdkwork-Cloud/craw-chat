//! Atomic PostgreSQL write authority for Space normalized state and audit events.

use std::sync::Arc;

use im_adapters_social_postgres::governance_store::SpaceMemberRecord;
use im_adapters_social_postgres::member_capacity::MemberInsertOutcome;
use im_adapters_social_postgres::organization_store::{
    GroupMemberRecord, GroupRecord, SpaceRecord,
};
use im_adapters_postgres_journal::PostgresCommitJournal;
use im_adapters_social_postgres::{
    SpaceMaterializationError, materialize_space_commits_on_transaction,
};
use im_app_context::AppContext;
use im_domain_events::space::{
    GroupCreatedPayload, GroupDeletedPayload, GroupMemberJoinedPayload, GroupMemberRemovedPayload,
    GroupMemberUpdatedPayload, GroupOwnerTransferredPayload, GroupUpdatedPayload,
    SpaceCommitEnvelopeInput, SpaceCreatedPayload, SpaceDeletedPayload, SpaceEventType,
    SpaceMemberJoinedPayload, SpaceMemberRemovedPayload, SpaceMemberUpdatedPayload,
    SpaceUpdatedPayload, space_commit_envelope,
};
use im_domain_events::{AggregateType, CommitEnvelope, EventActor};
use im_platform_contracts::{ContractError, IdGenerator};
use sdkwork_routes_web_framework_backend_api::response::ApiProblem;

use crate::http::AppState;
const MEMBER_CAPACITY_CONFLICT: &str = "space-write-member-capacity-full";

pub struct SpaceWriteAuthority {
    journal: PostgresCommitJournal,
}

struct SpaceCommitBuildCommand<'a> {
    id_generator: &'a Arc<dyn IdGenerator>,
    auth: &'a AppContext,
    aggregate_type: AggregateType,
    aggregate_id: &'a str,
    event_type: SpaceEventType,
    occurred_at: &'a str,
    payload_json: &'a str,
}

impl SpaceWriteAuthority {
    pub fn new(journal: PostgresCommitJournal) -> Self {
        Self { journal }
    }

    pub fn persist_space_created(
        &self,
        id_generator: &Arc<dyn IdGenerator>,
        auth: &AppContext,
        record: &SpaceRecord,
        owner_member: &SpaceMemberRecord,
    ) -> Result<(), ApiProblem> {
        let now = record.created_at.as_str();
        let space_payload = SpaceCreatedPayload {
            space_id: record.space_id.to_string(),
            space_name: record.space_name.clone(),
            space_type: record.space_type.clone(),
            owner_user_id: record.owner_user_id.clone(),
            description: record.description.clone(),
            avatar_url: record.avatar_url.clone(),
            max_members: record.max_members,
            settings_json: record.settings_json.clone(),
            created_at: record.created_at.clone(),
            updated_at: record.updated_at.clone(),
        };
        let member_payload = SpaceMemberJoinedPayload {
            space_id: record.space_id.to_string(),
            user_id: owner_member.user_id.clone(),
            role: owner_member.role.clone(),
            nickname: owner_member.nickname.clone(),
            joined_at: owner_member.joined_at.clone(),
            updated_at: owner_member.updated_at.clone(),
        };
        let commits = vec![
            self.build_commit(SpaceCommitBuildCommand {
                id_generator,
                auth,
                aggregate_type: AggregateType::Space,
                aggregate_id: record.space_id.to_string().as_str(),
                event_type: SpaceEventType::SpaceCreated,
                occurred_at: now,
                payload_json: &serde_json::to_string(&space_payload).map_err(serialize_error)?,
            })?,
            self.build_commit(SpaceCommitBuildCommand {
                id_generator,
                auth,
                aggregate_type: AggregateType::Space,
                aggregate_id: record.space_id.to_string().as_str(),
                event_type: SpaceEventType::SpaceMemberJoined,
                occurred_at: now,
                payload_json: &serde_json::to_string(&member_payload).map_err(serialize_error)?,
            })?,
        ];
        self.append_and_materialize(commits)
    }

    pub fn persist_space_updated(
        &self,
        id_generator: &Arc<dyn IdGenerator>,
        auth: &AppContext,
        record: &SpaceRecord,
    ) -> Result<(), ApiProblem> {
        let payload = SpaceUpdatedPayload {
            space_id: record.space_id.to_string(),
            space_name: record.space_name.clone(),
            description: record.description.clone(),
            avatar_url: record.avatar_url.clone(),
            max_members: record.max_members,
            settings_json: record.settings_json.clone(),
            updated_at: record.updated_at.clone(),
        };
        let commit = self.build_commit(SpaceCommitBuildCommand {
            id_generator,
            auth,
            aggregate_type: AggregateType::Space,
            aggregate_id: record.space_id.to_string().as_str(),
            event_type: SpaceEventType::SpaceUpdated,
            occurred_at: record.updated_at.as_str(),
            payload_json: &serde_json::to_string(&payload).map_err(serialize_error)?,
        })?;
        self.append_and_materialize(vec![commit])
    }

    pub fn persist_space_deleted(
        &self,
        id_generator: &Arc<dyn IdGenerator>,
        auth: &AppContext,
        space_id: i64,
        deleted_at: &str,
    ) -> Result<(), ApiProblem> {
        let payload = SpaceDeletedPayload {
            space_id: space_id.to_string(),
            deleted_at: deleted_at.to_owned(),
        };
        let commit = self.build_commit(SpaceCommitBuildCommand {
            id_generator,
            auth,
            aggregate_type: AggregateType::Space,
            aggregate_id: space_id.to_string().as_str(),
            event_type: SpaceEventType::SpaceDeleted,
            occurred_at: deleted_at,
            payload_json: &serde_json::to_string(&payload).map_err(serialize_error)?,
        })?;
        self.append_and_materialize(vec![commit])
    }

    pub fn persist_space_member_joined(
        &self,
        id_generator: &Arc<dyn IdGenerator>,
        auth: &AppContext,
        record: &SpaceMemberRecord,
    ) -> Result<(), ApiProblem> {
        let payload = SpaceMemberJoinedPayload {
            space_id: record.space_id.to_string(),
            user_id: record.user_id.clone(),
            role: record.role.clone(),
            nickname: record.nickname.clone(),
            joined_at: record.joined_at.clone(),
            updated_at: record.updated_at.clone(),
        };
        let commit = self.build_commit(SpaceCommitBuildCommand {
            id_generator,
            auth,
            aggregate_type: AggregateType::Space,
            aggregate_id: record.space_id.to_string().as_str(),
            event_type: SpaceEventType::SpaceMemberJoined,
            occurred_at: record.joined_at.as_str(),
            payload_json: &serde_json::to_string(&payload).map_err(serialize_error)?,
        })?;
        self.append_and_materialize(vec![commit])
    }

    pub fn persist_space_member_updated(
        &self,
        id_generator: &Arc<dyn IdGenerator>,
        auth: &AppContext,
        record: &SpaceMemberRecord,
    ) -> Result<(), ApiProblem> {
        let payload = SpaceMemberUpdatedPayload {
            space_id: record.space_id.to_string(),
            user_id: record.user_id.clone(),
            role: record.role.clone(),
            nickname: record.nickname.clone(),
            updated_at: record.updated_at.clone(),
        };
        let commit = self.build_commit(SpaceCommitBuildCommand {
            id_generator,
            auth,
            aggregate_type: AggregateType::Space,
            aggregate_id: record.space_id.to_string().as_str(),
            event_type: SpaceEventType::SpaceMemberUpdated,
            occurred_at: record.updated_at.as_str(),
            payload_json: &serde_json::to_string(&payload).map_err(serialize_error)?,
        })?;
        self.append_and_materialize(vec![commit])
    }

    pub fn persist_space_member_removed(
        &self,
        id_generator: &Arc<dyn IdGenerator>,
        auth: &AppContext,
        space_id: i64,
        user_id: &str,
        removed_at: &str,
    ) -> Result<(), ApiProblem> {
        let payload = SpaceMemberRemovedPayload {
            space_id: space_id.to_string(),
            user_id: user_id.to_owned(),
            removed_at: removed_at.to_owned(),
        };
        let commit = self.build_commit(SpaceCommitBuildCommand {
            id_generator,
            auth,
            aggregate_type: AggregateType::Space,
            aggregate_id: space_id.to_string().as_str(),
            event_type: SpaceEventType::SpaceMemberRemoved,
            occurred_at: removed_at,
            payload_json: &serde_json::to_string(&payload).map_err(serialize_error)?,
        })?;
        self.append_and_materialize(vec![commit])
    }

    pub fn persist_group_created(
        &self,
        id_generator: &Arc<dyn IdGenerator>,
        auth: &AppContext,
        record: &GroupRecord,
    ) -> Result<(), ApiProblem> {
        let payload = GroupCreatedPayload {
            group_id: record.group_id.to_string(),
            space_id: record.space_id.map(|value| value.to_string()),
            group_name: record.group_name.clone(),
            group_type: record.group_type.clone(),
            owner_user_id: record.owner_user_id.clone(),
            conversation_id: record.conversation_id.clone(),
            max_members: record.max_members,
            description: record.description.clone(),
            avatar_url: record.avatar_url.clone(),
            announcement: record.announcement.clone(),
            settings_json: record.settings_json.clone(),
            created_at: record.created_at.clone(),
            updated_at: record.updated_at.clone(),
        };
        let commit = self.build_commit(SpaceCommitBuildCommand {
            id_generator,
            auth,
            aggregate_type: AggregateType::ChatGroup,
            aggregate_id: record.group_id.to_string().as_str(),
            event_type: SpaceEventType::GroupCreated,
            occurred_at: record.created_at.as_str(),
            payload_json: &serde_json::to_string(&payload).map_err(serialize_error)?,
        })?;
        self.append_and_materialize(vec![commit])
    }

    pub fn persist_group_updated(
        &self,
        id_generator: &Arc<dyn IdGenerator>,
        auth: &AppContext,
        record: &GroupRecord,
    ) -> Result<(), ApiProblem> {
        let payload = GroupUpdatedPayload {
            group_id: record.group_id.to_string(),
            group_name: record.group_name.clone(),
            conversation_id: record.conversation_id.clone(),
            description: record.description.clone(),
            avatar_url: record.avatar_url.clone(),
            announcement: record.announcement.clone(),
            max_members: record.max_members,
            settings_json: record.settings_json.clone(),
            updated_at: record.updated_at.clone(),
        };
        let commit = self.build_commit(SpaceCommitBuildCommand {
            id_generator,
            auth,
            aggregate_type: AggregateType::ChatGroup,
            aggregate_id: record.group_id.to_string().as_str(),
            event_type: SpaceEventType::GroupUpdated,
            occurred_at: record.updated_at.as_str(),
            payload_json: &serde_json::to_string(&payload).map_err(serialize_error)?,
        })?;
        self.append_and_materialize(vec![commit])
    }

    pub fn persist_group_deleted(
        &self,
        id_generator: &Arc<dyn IdGenerator>,
        auth: &AppContext,
        group_id: i64,
        deleted_at: &str,
    ) -> Result<(), ApiProblem> {
        let payload = GroupDeletedPayload {
            group_id: group_id.to_string(),
            deleted_at: deleted_at.to_owned(),
        };
        let commit = self.build_commit(SpaceCommitBuildCommand {
            id_generator,
            auth,
            aggregate_type: AggregateType::ChatGroup,
            aggregate_id: group_id.to_string().as_str(),
            event_type: SpaceEventType::GroupDeleted,
            occurred_at: deleted_at,
            payload_json: &serde_json::to_string(&payload).map_err(serialize_error)?,
        })?;
        self.append_and_materialize(vec![commit])
    }

    pub fn persist_group_member_joined(
        &self,
        id_generator: &Arc<dyn IdGenerator>,
        auth: &AppContext,
        record: &GroupMemberRecord,
    ) -> Result<(), ApiProblem> {
        let payload = GroupMemberJoinedPayload {
            group_id: record.group_id.to_string(),
            user_id: record.user_id.clone(),
            role: record.role.clone(),
            nickname: record.nickname.clone(),
            mute_until: record.mute_until.clone(),
            joined_at: record.joined_at.clone(),
            updated_at: record.updated_at.clone(),
        };
        let commit = self.build_commit(SpaceCommitBuildCommand {
            id_generator,
            auth,
            aggregate_type: AggregateType::ChatGroup,
            aggregate_id: record.group_id.to_string().as_str(),
            event_type: SpaceEventType::GroupMemberJoined,
            occurred_at: record.joined_at.as_str(),
            payload_json: &serde_json::to_string(&payload).map_err(serialize_error)?,
        })?;
        self.append_and_materialize(vec![commit])
    }

    pub fn persist_group_member_updated(
        &self,
        id_generator: &Arc<dyn IdGenerator>,
        auth: &AppContext,
        record: &GroupMemberRecord,
    ) -> Result<(), ApiProblem> {
        let payload = GroupMemberUpdatedPayload {
            group_id: record.group_id.to_string(),
            user_id: record.user_id.clone(),
            role: record.role.clone(),
            nickname: record.nickname.clone(),
            mute_until: record.mute_until.clone(),
            updated_at: record.updated_at.clone(),
        };
        let commit = self.build_commit(SpaceCommitBuildCommand {
            id_generator,
            auth,
            aggregate_type: AggregateType::ChatGroup,
            aggregate_id: record.group_id.to_string().as_str(),
            event_type: SpaceEventType::GroupMemberUpdated,
            occurred_at: record.updated_at.as_str(),
            payload_json: &serde_json::to_string(&payload).map_err(serialize_error)?,
        })?;
        self.append_and_materialize(vec![commit])
    }

    pub fn persist_group_member_removed(
        &self,
        id_generator: &Arc<dyn IdGenerator>,
        auth: &AppContext,
        group_id: i64,
        user_id: &str,
        removed_at: &str,
    ) -> Result<(), ApiProblem> {
        let payload = GroupMemberRemovedPayload {
            group_id: group_id.to_string(),
            user_id: user_id.to_owned(),
            removed_at: removed_at.to_owned(),
        };
        let commit = self.build_commit(SpaceCommitBuildCommand {
            id_generator,
            auth,
            aggregate_type: AggregateType::ChatGroup,
            aggregate_id: group_id.to_string().as_str(),
            event_type: SpaceEventType::GroupMemberRemoved,
            occurred_at: removed_at,
            payload_json: &serde_json::to_string(&payload).map_err(serialize_error)?,
        })?;
        self.append_and_materialize(vec![commit])
    }

    pub fn persist_group_owner_transferred(
        &self,
        id_generator: &Arc<dyn IdGenerator>,
        auth: &AppContext,
        group_id: i64,
        current_owner_user_id: &str,
        new_owner_user_id: &str,
        transferred_at: &str,
    ) -> Result<(), ApiProblem> {
        let payload = GroupOwnerTransferredPayload {
            group_id: group_id.to_string(),
            current_owner_user_id: current_owner_user_id.to_owned(),
            new_owner_user_id: new_owner_user_id.to_owned(),
            transferred_at: transferred_at.to_owned(),
        };
        let commit = self.build_commit(SpaceCommitBuildCommand {
            id_generator,
            auth,
            aggregate_type: AggregateType::ChatGroup,
            aggregate_id: group_id.to_string().as_str(),
            event_type: SpaceEventType::GroupOwnerTransferred,
            occurred_at: transferred_at,
            payload_json: &serde_json::to_string(&payload).map_err(serialize_error)?,
        })?;
        self.append_and_materialize(vec![commit])
    }

    fn build_commit(
        &self,
        command: SpaceCommitBuildCommand<'_>,
    ) -> Result<CommitEnvelope, ApiProblem> {
        let event_id = command
            .id_generator
            .next_id()
            .map(|value| value.to_string())
            .map_err(|error| {
                tracing::error!(?error, "space commit event id generation failed");
                ApiProblem::dependency_unavailable("space commit event id generation failed")
            })?;
        Ok(space_commit_envelope(SpaceCommitEnvelopeInput {
            event_id: event_id.as_str(),
            tenant_id: command.auth.tenant_id.as_str(),
            organization_id: command.auth.organization_id.as_str(),
            aggregate_type: command.aggregate_type,
            aggregate_id: command.aggregate_id,
            event_type: command.event_type,
            ordering_seq: 1,
            actor: event_actor_from_auth(command.auth),
            occurred_at: command.occurred_at,
            committed_at: command.occurred_at,
            payload: command.payload_json,
        }))
    }

    fn append_and_materialize(&self, commits: Vec<CommitEnvelope>) -> Result<(), ApiProblem> {
        if commits.is_empty() {
            return Ok(());
        }

        let commit_count = commits.len();
        let result = self
            .journal
            .append_batch_with_allocated_sequences_in_transaction(
                commits,
                |txn, sequenced_commits| {
                    materialize_space_commits_on_transaction(txn, sequenced_commits).map_err(
                        |error| match error {
                            SpaceMaterializationError::CapacityFull => {
                                ContractError::Conflict(MEMBER_CAPACITY_CONFLICT.into())
                            }
                            SpaceMaterializationError::Persistence(message) => {
                                ContractError::Unavailable(format!(
                                    "space normalized PostgreSQL write failed: {message}"
                                ))
                            }
                        },
                    )
                },
            );
        if let Err(error) = result {
            if !is_member_capacity_conflict(&error) {
                crate::space_materializer_metrics::record_postgres_atomic_write_failures(
                    commit_count as u64,
                );
            }
            return Err(coordinated_write_error(error));
        }
        Ok(())
    }
}

pub fn event_actor_from_auth(auth: &AppContext) -> EventActor {
    EventActor {
        actor_id: auth.actor_id.clone(),
        actor_kind: auth.actor_kind.clone(),
        actor_session_id: auth.session_id.clone(),
    }
}

fn serialize_error(error: serde_json::Error) -> ApiProblem {
    tracing::error!(?error, "space commit payload serialization failed");
    ApiProblem::internal_server_error("space commit payload serialization failed")
}

fn coordinated_write_error(error: ContractError) -> ApiProblem {
    match error {
        ContractError::Conflict(message) if message == MEMBER_CAPACITY_CONFLICT => {
            ApiProblem::bad_request("member limit reached")
        }
        other => {
            tracing::error!(?other, "atomic space PostgreSQL write failed");
            ApiProblem::dependency_unavailable("atomic space PostgreSQL write failed")
        }
    }
}

fn is_member_capacity_conflict(error: &ContractError) -> bool {
    matches!(error, ContractError::Conflict(message) if message == MEMBER_CAPACITY_CONFLICT)
}

pub fn persist_space_created(
    state: &AppState,
    auth: &AppContext,
    record: &SpaceRecord,
    owner_member: &SpaceMemberRecord,
) -> Result<(), ApiProblem> {
    if let Some(authority) = state.write_authority.as_ref() {
        return authority.persist_space_created(&state.id_generator, auth, record, owner_member);
    }
    state.space_store.insert(record).map_err(|error| {
        tracing::error!(error = ?error, "failed to insert space record");
        ApiProblem::internal_server_error("failed to insert space")
    })?;
    state.space_member_store.insert(owner_member).map_err(|error| {
        tracing::error!(error = ?error, space_id = record.space_id, "failed to insert space owner member");
        ApiProblem::internal_server_error("failed to insert space owner member")
    })
}

pub fn persist_space_updated(
    state: &AppState,
    auth: &AppContext,
    record: &SpaceRecord,
) -> Result<(), ApiProblem> {
    if let Some(authority) = state.write_authority.as_ref() {
        return authority.persist_space_updated(&state.id_generator, auth, record);
    }
    state.space_store.update(record).map_err(|error| {
        tracing::error!(error = ?error, space_id = record.space_id, "failed to update space");
        ApiProblem::internal_server_error("failed to update space")
    })
}

pub fn persist_space_deleted(
    state: &AppState,
    auth: &AppContext,
    space_id: i64,
    deleted_at: &str,
) -> Result<(), ApiProblem> {
    if let Some(authority) = state.write_authority.as_ref() {
        return authority.persist_space_deleted(&state.id_generator, auth, space_id, deleted_at);
    }
    state
        .space_store
        .delete(
            auth.tenant_id.as_str(),
            auth.organization_id.as_str(),
            space_id,
        )
        .map_err(|error| {
            tracing::error!(error = ?error, space_id, "failed to delete space");
            ApiProblem::internal_server_error("failed to delete space")
        })
}

pub fn persist_space_member_joined(
    state: &AppState,
    auth: &AppContext,
    record: &SpaceMemberRecord,
    max_members: i32,
) -> Result<(), ApiProblem> {
    if let Some(authority) = state.write_authority.as_ref() {
        return authority.persist_space_member_joined(&state.id_generator, auth, record);
    }
    match state
        .space_member_store
        .insert_within_capacity(record, max_members)
    {
        Ok(MemberInsertOutcome::Inserted | MemberInsertOutcome::AlreadyExists) => Ok(()),
        Ok(MemberInsertOutcome::CapacityFull) => {
            Err(ApiProblem::bad_request("space member limit reached"))
        }
        Err(error) => {
            tracing::error!(error = ?error, "failed to insert space member");
            Err(ApiProblem::internal_server_error(
                "failed to insert space member",
            ))
        }
    }
}

pub fn persist_space_member_updated(
    state: &AppState,
    auth: &AppContext,
    record: &SpaceMemberRecord,
) -> Result<(), ApiProblem> {
    if let Some(authority) = state.write_authority.as_ref() {
        return authority.persist_space_member_updated(&state.id_generator, auth, record);
    }
    state.space_member_store.update(record).map_err(|error| {
        tracing::error!(error = ?error, "failed to update space member");
        ApiProblem::internal_server_error("failed to update space member")
    })
}

pub fn persist_space_member_removed(
    state: &AppState,
    auth: &AppContext,
    space_id: i64,
    user_id: &str,
    removed_at: &str,
) -> Result<(), ApiProblem> {
    if let Some(authority) = state.write_authority.as_ref() {
        return authority.persist_space_member_removed(
            &state.id_generator,
            auth,
            space_id,
            user_id,
            removed_at,
        );
    }
    state
        .space_member_store
        .delete(
            auth.tenant_id.as_str(),
            auth.organization_id.as_str(),
            space_id,
            user_id,
        )
        .map_err(|error| {
            tracing::error!(error = ?error, "failed to remove space member");
            ApiProblem::internal_server_error("failed to remove space member")
        })
}

pub fn persist_group_created(
    state: &AppState,
    auth: &AppContext,
    record: &GroupRecord,
    owner_member: &GroupMemberRecord,
) -> Result<(), ApiProblem> {
    if let Some(authority) = state.write_authority.as_ref() {
        return authority.persist_group_created(&state.id_generator, auth, record);
    }
    state
        .group_store
        .insert_with_owner_member(record, owner_member)
        .map_err(|error| {
            tracing::error!(error = ?error, "failed to insert group record with owner member");
            ApiProblem::internal_server_error("failed to insert group")
        })
}

pub fn persist_group_updated(
    state: &AppState,
    auth: &AppContext,
    record: &GroupRecord,
) -> Result<(), ApiProblem> {
    if let Some(authority) = state.write_authority.as_ref() {
        return authority.persist_group_updated(&state.id_generator, auth, record);
    }
    state.group_store.update(record).map_err(|error| {
        tracing::error!(error = ?error, group_id = record.group_id, "failed to update group");
        ApiProblem::internal_server_error("failed to update group")
    })
}

pub fn persist_group_deleted(
    state: &AppState,
    auth: &AppContext,
    group_id: i64,
    deleted_at: &str,
) -> Result<(), ApiProblem> {
    if let Some(authority) = state.write_authority.as_ref() {
        return authority.persist_group_deleted(&state.id_generator, auth, group_id, deleted_at);
    }
    state
        .group_store
        .delete(
            auth.tenant_id.as_str(),
            auth.organization_id.as_str(),
            group_id,
        )
        .map_err(|error| {
            tracing::error!(error = ?error, group_id, "failed to delete group");
            ApiProblem::internal_server_error("failed to delete group")
        })
}

pub fn persist_group_member_joined(
    state: &AppState,
    auth: &AppContext,
    record: &GroupMemberRecord,
    max_members: i32,
) -> Result<(), ApiProblem> {
    if let Some(authority) = state.write_authority.as_ref() {
        return authority.persist_group_member_joined(&state.id_generator, auth, record);
    }
    match state
        .group_member_store
        .insert_within_capacity(record, max_members)
    {
        Ok(MemberInsertOutcome::Inserted | MemberInsertOutcome::AlreadyExists) => Ok(()),
        Ok(MemberInsertOutcome::CapacityFull) => {
            Err(ApiProblem::bad_request("group member limit reached"))
        }
        Err(error) => {
            tracing::error!(error = ?error, "failed to insert group member");
            Err(ApiProblem::internal_server_error(
                "failed to insert group member",
            ))
        }
    }
}

pub fn persist_group_member_updated(
    state: &AppState,
    auth: &AppContext,
    record: &GroupMemberRecord,
) -> Result<(), ApiProblem> {
    if let Some(authority) = state.write_authority.as_ref() {
        return authority.persist_group_member_updated(&state.id_generator, auth, record);
    }
    state.group_member_store.update(record).map_err(|error| {
        tracing::error!(error = ?error, "failed to update group member");
        ApiProblem::internal_server_error("failed to update group member")
    })
}

pub fn persist_group_member_removed(
    state: &AppState,
    auth: &AppContext,
    group_id: i64,
    user_id: &str,
    removed_at: &str,
) -> Result<(), ApiProblem> {
    if let Some(authority) = state.write_authority.as_ref() {
        return authority.persist_group_member_removed(
            &state.id_generator,
            auth,
            group_id,
            user_id,
            removed_at,
        );
    }
    state
        .group_member_store
        .delete(
            auth.tenant_id.as_str(),
            auth.organization_id.as_str(),
            group_id,
            user_id,
        )
        .map_err(|error| {
            tracing::error!(error = ?error, "failed to remove group member");
            ApiProblem::internal_server_error("failed to remove group member")
        })
}

pub fn persist_group_owner_transferred(
    state: &AppState,
    auth: &AppContext,
    group_id: i64,
    current_owner_user_id: &str,
    new_owner_user_id: &str,
    transferred_at: &str,
) -> Result<(), ApiProblem> {
    if let Some(authority) = state.write_authority.as_ref() {
        return authority.persist_group_owner_transferred(
            &state.id_generator,
            auth,
            group_id,
            current_owner_user_id,
            new_owner_user_id,
            transferred_at,
        );
    }
    state
        .group_store
        .transfer_owner(
            auth.tenant_id.as_str(),
            auth.organization_id.as_str(),
            group_id,
            current_owner_user_id,
            new_owner_user_id,
            transferred_at,
        )
        .map(|_| ())
        .map_err(|error| match error {
            im_platform_contracts::ContractError::Conflict(message) => {
                tracing::warn!(error = %message, group_id, "group owner transfer conflict");
                ApiProblem::forbidden("group ownership transfer rejected")
            }
            other => {
                tracing::error!(error = ?other, group_id, "failed to transfer group owner");
                ApiProblem::internal_server_error("failed to transfer group owner")
            }
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_the_typed_capacity_marker_is_a_business_rejection() {
        assert!(is_member_capacity_conflict(&ContractError::Conflict(
            MEMBER_CAPACITY_CONFLICT.into()
        )));
        assert!(!is_member_capacity_conflict(&ContractError::Conflict(
            "journal position conflict".into()
        )));
        assert!(!is_member_capacity_conflict(&ContractError::Unavailable(
            MEMBER_CAPACITY_CONFLICT.into()
        )));
    }
}
