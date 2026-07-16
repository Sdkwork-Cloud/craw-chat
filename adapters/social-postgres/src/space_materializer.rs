//! Materialize space/group commit envelopes into supplemental PostgreSQL stores.

use std::sync::Arc;

use crate::SocialPostgresPool;
use crate::governance_store::{SpaceMemberRecord, SpaceMemberStore};
use crate::member_capacity::MemberInsertOutcome;
use crate::organization_store::{
    GroupMemberRecord, GroupMemberStore, GroupRecord, GroupStore, SpaceRecord, SpaceStore,
};
use crate::wire_id::social_entity_id_to_i64;
use im_domain_events::space::{
    GroupCreatedPayload, GroupDeletedPayload, GroupMemberJoinedPayload, GroupMemberRemovedPayload,
    GroupMemberUpdatedPayload, GroupOwnerTransferredPayload, GroupUpdatedPayload,
    SpaceCreatedPayload, SpaceDeletedPayload, SpaceMemberJoinedPayload, SpaceMemberRemovedPayload,
    SpaceMemberUpdatedPayload, SpaceUpdatedPayload,
};
use im_platform_contracts::CommitEnvelope;

pub struct SpacePostgresMaterializer {
    pool: SocialPostgresPool,
    space_store: Arc<dyn SpaceStore>,
    space_member_store: Arc<dyn SpaceMemberStore>,
    group_store: Arc<dyn GroupStore>,
    group_member_store: Arc<dyn GroupMemberStore>,
}

impl SpacePostgresMaterializer {
    pub fn from_pool(pool: SocialPostgresPool) -> Self {
        let pool_arc = Arc::new(pool.inner().clone());
        Self {
            pool,
            space_store: Arc::new(crate::organization_store::PostgresSpaceStore::new(
                pool_arc.clone(),
            )),
            space_member_store: Arc::new(crate::governance_store::PostgresSpaceMemberStore::new(
                pool_arc.clone(),
            )),
            group_store: Arc::new(crate::organization_store::PostgresGroupStore::new(
                pool_arc.clone(),
            )),
            group_member_store: Arc::new(crate::organization_store::PostgresGroupMemberStore::new(
                pool_arc,
            )),
        }
    }

    pub fn try_materialize_commits(&self, commits: &[CommitEnvelope]) -> usize {
        let mut failures = 0usize;
        for commit in commits {
            if let Err(error) = self.try_materialize_commit(commit) {
                failures += 1;
                tracing::error!(
                    event_id = commit.event_id.as_str(),
                    event_type = commit.event_type.as_str(),
                    error = %error,
                    "space postgres materialization failed for commit"
                );
            }
        }
        failures
    }

    /// Materializes commits and returns the first error (used by write authority to surface cap violations).
    pub fn materialize_commits(&self, commits: &[CommitEnvelope]) -> Result<(), String> {
        if commits.len() > 1 {
            return crate::space_materialize_writes::materialize_space_commits_in_transaction(
                &self.pool, commits,
            );
        }
        for commit in commits {
            self.try_materialize_commit(commit)?;
        }
        Ok(())
    }

    fn try_materialize_commit(&self, commit: &CommitEnvelope) -> Result<(), String> {
        match commit.event_type.as_str() {
            "space.created" => self.materialize_space_created(commit),
            "space.updated" => self.materialize_space_updated(commit),
            "space.deleted" => self.materialize_space_deleted(commit),
            "space.member_joined" => self.materialize_space_member_joined(commit),
            "space.member_updated" => self.materialize_space_member_updated(commit),
            "space.member_removed" => self.materialize_space_member_removed(commit),
            "group.created" => self.materialize_group_created(commit),
            "group.updated" => self.materialize_group_updated(commit),
            "group.deleted" => self.materialize_group_deleted(commit),
            "group.member_joined" => self.materialize_group_member_joined(commit),
            "group.member_updated" => self.materialize_group_member_updated(commit),
            "group.member_removed" => self.materialize_group_member_removed(commit),
            "group.owner_transferred" => self.materialize_group_owner_transferred(commit),
            _ => Ok(()),
        }
    }

    fn materialize_space_created(&self, commit: &CommitEnvelope) -> Result<(), String> {
        let payload: SpaceCreatedPayload = serde_json::from_str(commit.payload.as_str())
            .map_err(|error| format!("invalid space.created payload: {error}"))?;
        let record = SpaceRecord {
            tenant_id: commit.tenant_id.clone(),
            organization_id: commit.organization_id.clone(),
            space_id: social_entity_id_to_i64(payload.space_id.as_str()),
            space_name: payload.space_name,
            space_type: payload.space_type,
            owner_user_id: payload.owner_user_id,
            description: payload.description,
            avatar_url: payload.avatar_url,
            max_members: payload.max_members,
            settings_json: payload.settings_json,
            created_at: payload.created_at,
            updated_at: payload.updated_at,
        };
        self.space_store
            .insert(&record)
            .map_err(|error| format!("space insert failed: {error:?}"))
    }

    fn materialize_space_updated(&self, commit: &CommitEnvelope) -> Result<(), String> {
        let payload: SpaceUpdatedPayload = serde_json::from_str(commit.payload.as_str())
            .map_err(|error| format!("invalid space.updated payload: {error}"))?;
        let space_id = social_entity_id_to_i64(payload.space_id.as_str());
        let existing = self
            .space_store
            .get_by_id(
                commit.tenant_id.as_str(),
                commit.organization_id.as_str(),
                space_id,
            )
            .map_err(|error| format!("space load failed: {error:?}"))?
            .ok_or_else(|| format!("space {space_id} not found for update"))?;
        let record = SpaceRecord {
            space_name: payload.space_name,
            description: payload.description,
            avatar_url: payload.avatar_url,
            max_members: payload.max_members,
            settings_json: payload.settings_json,
            updated_at: payload.updated_at,
            ..existing
        };
        self.space_store
            .update(&record)
            .map_err(|error| format!("space update failed: {error:?}"))
    }

    fn materialize_space_deleted(&self, commit: &CommitEnvelope) -> Result<(), String> {
        let payload: SpaceDeletedPayload = serde_json::from_str(commit.payload.as_str())
            .map_err(|error| format!("invalid space.deleted payload: {error}"))?;
        self.space_store
            .delete(
                commit.tenant_id.as_str(),
                commit.organization_id.as_str(),
                social_entity_id_to_i64(payload.space_id.as_str()),
            )
            .map_err(|error| format!("space delete failed: {error:?}"))
    }

    fn materialize_space_member_joined(&self, commit: &CommitEnvelope) -> Result<(), String> {
        let payload: SpaceMemberJoinedPayload = serde_json::from_str(commit.payload.as_str())
            .map_err(|error| format!("invalid space.member_joined payload: {error}"))?;
        let record = SpaceMemberRecord {
            tenant_id: commit.tenant_id.clone(),
            organization_id: commit.organization_id.clone(),
            space_id: social_entity_id_to_i64(payload.space_id.as_str()),
            user_id: payload.user_id,
            role: payload.role,
            nickname: payload.nickname,
            joined_at: payload.joined_at,
            updated_at: payload.updated_at,
        };
        let max_members = self
            .space_store
            .get_by_id(
                commit.tenant_id.as_str(),
                commit.organization_id.as_str(),
                record.space_id,
            )
            .map_err(|error| format!("space load failed: {error:?}"))?
            .map(|space| space.max_members)
            .unwrap_or(i32::MAX);
        match self
            .space_member_store
            .insert_within_capacity(&record, max_members)
            .map_err(|error| format!("space member insert failed: {error:?}"))?
        {
            MemberInsertOutcome::Inserted | MemberInsertOutcome::AlreadyExists => Ok(()),
            MemberInsertOutcome::CapacityFull => {
                Err("space member capacity full during materialization".to_owned())
            }
        }
    }

    fn materialize_space_member_updated(&self, commit: &CommitEnvelope) -> Result<(), String> {
        let payload: SpaceMemberUpdatedPayload = serde_json::from_str(commit.payload.as_str())
            .map_err(|error| format!("invalid space.member_updated payload: {error}"))?;
        let space_id = social_entity_id_to_i64(payload.space_id.as_str());
        let existing = self
            .space_member_store
            .get_by_id(
                commit.tenant_id.as_str(),
                commit.organization_id.as_str(),
                space_id,
                payload.user_id.as_str(),
            )
            .map_err(|error| format!("space member load failed: {error:?}"))?
            .ok_or_else(|| {
                format!(
                    "space member {} in space {space_id} not found for update",
                    payload.user_id
                )
            })?;
        let record = SpaceMemberRecord {
            role: payload.role,
            nickname: payload.nickname,
            updated_at: payload.updated_at,
            ..existing
        };
        self.space_member_store
            .update(&record)
            .map_err(|error| format!("space member update failed: {error:?}"))
    }

    fn materialize_space_member_removed(&self, commit: &CommitEnvelope) -> Result<(), String> {
        let payload: SpaceMemberRemovedPayload = serde_json::from_str(commit.payload.as_str())
            .map_err(|error| format!("invalid space.member_removed payload: {error}"))?;
        self.space_member_store
            .delete(
                commit.tenant_id.as_str(),
                commit.organization_id.as_str(),
                social_entity_id_to_i64(payload.space_id.as_str()),
                payload.user_id.as_str(),
            )
            .map_err(|error| format!("space member delete failed: {error:?}"))
    }

    fn materialize_group_created(&self, commit: &CommitEnvelope) -> Result<(), String> {
        let payload: GroupCreatedPayload = serde_json::from_str(commit.payload.as_str())
            .map_err(|error| format!("invalid group.created payload: {error}"))?;
        let group = GroupRecord {
            tenant_id: commit.tenant_id.clone(),
            organization_id: commit.organization_id.clone(),
            group_id: social_entity_id_to_i64(payload.group_id.as_str()),
            space_id: payload.space_id.as_deref().map(social_entity_id_to_i64),
            group_name: payload.group_name.clone(),
            group_type: payload.group_type,
            owner_user_id: payload.owner_user_id.clone(),
            conversation_id: payload.conversation_id,
            max_members: payload.max_members,
            description: payload.description,
            avatar_url: payload.avatar_url,
            announcement: payload.announcement,
            settings_json: payload.settings_json,
            created_at: payload.created_at.clone(),
            updated_at: payload.updated_at.clone(),
        };
        let owner_member = GroupMemberRecord {
            tenant_id: commit.tenant_id.clone(),
            organization_id: commit.organization_id.clone(),
            group_id: group.group_id,
            user_id: payload.owner_user_id,
            role: "owner".to_owned(),
            nickname: None,
            mute_until: None,
            joined_at: payload.created_at,
            updated_at: payload.updated_at,
        };
        self.group_store
            .insert_with_owner_member(&group, &owner_member)
            .map_err(|error| format!("group insert failed: {error:?}"))
    }

    fn materialize_group_updated(&self, commit: &CommitEnvelope) -> Result<(), String> {
        let payload: GroupUpdatedPayload = serde_json::from_str(commit.payload.as_str())
            .map_err(|error| format!("invalid group.updated payload: {error}"))?;
        let group_id = social_entity_id_to_i64(payload.group_id.as_str());
        let existing = self
            .group_store
            .get_by_id(
                commit.tenant_id.as_str(),
                commit.organization_id.as_str(),
                group_id,
            )
            .map_err(|error| format!("group load failed: {error:?}"))?
            .ok_or_else(|| format!("group {group_id} not found for update"))?;
        let record = GroupRecord {
            group_name: payload.group_name,
            description: payload.description,
            avatar_url: payload.avatar_url,
            announcement: payload.announcement,
            max_members: payload.max_members,
            settings_json: payload.settings_json,
            updated_at: payload.updated_at,
            ..existing
        };
        self.group_store
            .update(&record)
            .map_err(|error| format!("group update failed: {error:?}"))
    }

    fn materialize_group_deleted(&self, commit: &CommitEnvelope) -> Result<(), String> {
        let payload: GroupDeletedPayload = serde_json::from_str(commit.payload.as_str())
            .map_err(|error| format!("invalid group.deleted payload: {error}"))?;
        self.group_store
            .delete(
                commit.tenant_id.as_str(),
                commit.organization_id.as_str(),
                social_entity_id_to_i64(payload.group_id.as_str()),
            )
            .map_err(|error| format!("group delete failed: {error:?}"))
    }

    fn materialize_group_member_joined(&self, commit: &CommitEnvelope) -> Result<(), String> {
        let payload: GroupMemberJoinedPayload = serde_json::from_str(commit.payload.as_str())
            .map_err(|error| format!("invalid group.member_joined payload: {error}"))?;
        let record = GroupMemberRecord {
            tenant_id: commit.tenant_id.clone(),
            organization_id: commit.organization_id.clone(),
            group_id: social_entity_id_to_i64(payload.group_id.as_str()),
            user_id: payload.user_id,
            role: payload.role,
            nickname: payload.nickname,
            mute_until: payload.mute_until,
            joined_at: payload.joined_at,
            updated_at: payload.updated_at,
        };
        let max_members = self
            .group_store
            .get_by_id(
                commit.tenant_id.as_str(),
                commit.organization_id.as_str(),
                record.group_id,
            )
            .map_err(|error| format!("group load failed: {error:?}"))?
            .map(|group| group.max_members)
            .unwrap_or(i32::MAX);
        match self
            .group_member_store
            .insert_within_capacity(&record, max_members)
            .map_err(|error| format!("group member insert failed: {error:?}"))?
        {
            MemberInsertOutcome::Inserted | MemberInsertOutcome::AlreadyExists => Ok(()),
            MemberInsertOutcome::CapacityFull => {
                Err("group member capacity full during materialization".to_owned())
            }
        }
    }

    fn materialize_group_member_updated(&self, commit: &CommitEnvelope) -> Result<(), String> {
        let payload: GroupMemberUpdatedPayload = serde_json::from_str(commit.payload.as_str())
            .map_err(|error| format!("invalid group.member_updated payload: {error}"))?;
        let group_id = social_entity_id_to_i64(payload.group_id.as_str());
        let existing = self
            .group_member_store
            .get_by_id(
                commit.tenant_id.as_str(),
                commit.organization_id.as_str(),
                group_id,
                payload.user_id.as_str(),
            )
            .map_err(|error| format!("group member load failed: {error:?}"))?
            .ok_or_else(|| {
                format!(
                    "group member {} in group {group_id} not found for update",
                    payload.user_id
                )
            })?;
        let record = GroupMemberRecord {
            role: payload.role,
            nickname: payload.nickname,
            mute_until: payload.mute_until,
            updated_at: payload.updated_at,
            ..existing
        };
        self.group_member_store
            .update(&record)
            .map_err(|error| format!("group member update failed: {error:?}"))
    }

    fn materialize_group_member_removed(&self, commit: &CommitEnvelope) -> Result<(), String> {
        let payload: GroupMemberRemovedPayload = serde_json::from_str(commit.payload.as_str())
            .map_err(|error| format!("invalid group.member_removed payload: {error}"))?;
        self.group_member_store
            .delete(
                commit.tenant_id.as_str(),
                commit.organization_id.as_str(),
                social_entity_id_to_i64(payload.group_id.as_str()),
                payload.user_id.as_str(),
            )
            .map_err(|error| format!("group member delete failed: {error:?}"))
    }

    fn materialize_group_owner_transferred(&self, commit: &CommitEnvelope) -> Result<(), String> {
        let payload: GroupOwnerTransferredPayload =
            serde_json::from_str(commit.payload.as_str())
                .map_err(|error| format!("invalid group.owner_transferred payload: {error}"))?;
        self.group_store
            .transfer_owner(
                commit.tenant_id.as_str(),
                commit.organization_id.as_str(),
                social_entity_id_to_i64(payload.group_id.as_str()),
                payload.current_owner_user_id.as_str(),
                payload.new_owner_user_id.as_str(),
                payload.transferred_at.as_str(),
            )
            .map_err(|error| format!("group owner transfer failed: {error:?}"))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use im_domain_events::space::{
        SpaceCommitEnvelopeInput, SpaceCreatedPayload, SpaceEventType, space_commit_envelope,
    };
    use im_domain_events::{AggregateType, EventActor};

    #[test]
    fn space_created_event_materializes_into_record_shape() {
        let payload = SpaceCreatedPayload {
            space_id: "330339707122622464".to_string(),
            space_name: "Engineering".to_string(),
            space_type: "organization".to_string(),
            owner_user_id: "user-1".to_string(),
            description: None,
            avatar_url: None,
            max_members: 100,
            settings_json: "{}".to_string(),
            created_at: "2026-04-09T00:00:00Z".to_string(),
            updated_at: "2026-04-09T00:00:00Z".to_string(),
        };
        let payload_json = serde_json::to_string(&payload).expect("payload serializes");
        let envelope = space_commit_envelope(SpaceCommitEnvelopeInput {
            event_id: "evt-space-1",
            tenant_id: "100001",
            organization_id: "0",
            aggregate_type: AggregateType::Space,
            aggregate_id: "330339707122622464",
            event_type: SpaceEventType::SpaceCreated,
            ordering_seq: 1,
            actor: EventActor {
                actor_id: "user-1".into(),
                actor_kind: "user".into(),
                actor_session_id: None,
            },
            occurred_at: "2026-04-09T00:00:00Z",
            committed_at: "2026-04-09T00:00:00Z",
            payload: payload_json.as_str(),
        });
        assert_eq!(envelope.event_type, "space.created");
        assert_eq!(envelope.aggregate_type, AggregateType::Space);
    }
}
