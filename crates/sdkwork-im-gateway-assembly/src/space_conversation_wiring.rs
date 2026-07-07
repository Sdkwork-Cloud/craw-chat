//! Wires space group and channel lifecycle into conversation-service.

use std::collections::BTreeMap;
use std::sync::Arc;

use chrono::{DateTime, Utc};
use conversation_runtime::{
    AddConversationMemberCommand, ApplyConversationPolicyCommand, ConversationCommitJournal,
    ConversationRuntime, CreateConversationCommand, CreateSystemChannelCommand,
    RemoveConversationMemberCommand, TransferConversationOwnerCommand,
};
use im_domain_core::conversation::{ConversationPolicy, MembershipRole, member_id};
use space_service::{
    CreateSpaceChannelConversationInput, CreateSpaceGroupConversationInput,
    SpaceChannelConversationBinder, SpaceGroupConversationBinder, SyncSpaceGroupMemberInput,
    TransferSpaceGroupOwnerInput,
};

const POSTING_RESTRICTED_ATTRIBUTE: &str = "postingRestricted";
const MUTE_UNTIL_ATTRIBUTE: &str = "muteUntil";

pub struct ConversationServiceSpaceConversationBinder {
    runtime: Arc<ConversationRuntime<ConversationCommitJournal>>,
}

impl ConversationServiceSpaceConversationBinder {
    pub fn new(runtime: Arc<ConversationRuntime<ConversationCommitJournal>>) -> Self {
        Self { runtime }
    }
}

fn member_posting_attributes(role: &str, mute_until: Option<&str>) -> BTreeMap<String, String> {
    let mut attributes = BTreeMap::new();
    if role == "muted" || is_mute_active(mute_until) {
        attributes.insert(POSTING_RESTRICTED_ATTRIBUTE.to_owned(), "true".to_owned());
    }
    if let Some(mute_until) = mute_until.filter(|value| !value.trim().is_empty()) {
        attributes.insert(MUTE_UNTIL_ATTRIBUTE.to_owned(), mute_until.to_owned());
    }
    attributes
}

fn is_mute_active(mute_until: Option<&str>) -> bool {
    let Some(raw) = mute_until.filter(|value| !value.trim().is_empty()) else {
        return false;
    };
    DateTime::parse_from_rfc3339(raw)
        .map(|deadline| deadline > Utc::now())
        .unwrap_or(true)
}

impl SpaceGroupConversationBinder for ConversationServiceSpaceConversationBinder {
    fn create_group_conversation(
        &self,
        input: CreateSpaceGroupConversationInput,
    ) -> Result<(), String> {
        self.runtime
            .create_conversation_with_creator_kind(
                CreateConversationCommand {
                    tenant_id: input.tenant_id.clone(),
                    organization_id: input.organization_id.clone(),
                    conversation_id: input.conversation_id.clone(),
                    creator_id: input.creator_user_id.clone(),
                    conversation_type: "group".to_owned(),
                },
                "system",
            )
            .map_err(|error| format!("{error:?}"))?;
        let mut policy = ConversationPolicy::default();
        policy.max_members = Some(input.max_members);
        self.runtime
            .apply_conversation_policy_with_actor_kind(
                ApplyConversationPolicyCommand {
                    tenant_id: input.tenant_id,
                    organization_id: input.organization_id,
                    conversation_id: input.conversation_id,
                    applied_by: input.creator_user_id,
                    policy,
                },
                "system",
            )
            .map_err(|error| format!("{error:?}"))?;
        Ok(())
    }

    fn add_group_member(&self, input: SyncSpaceGroupMemberInput) -> Result<(), String> {
        if input.role == "owner" {
            return Ok(());
        }
        let role = match input.role.as_str() {
            "admin" => MembershipRole::Admin,
            "muted" => MembershipRole::Member,
            _ => MembershipRole::Member,
        };
        let attributes =
            member_posting_attributes(input.role.as_str(), input.mute_until.as_deref());
        self.runtime
            .add_member_with_actor_kind_and_attributes(
                AddConversationMemberCommand {
                    tenant_id: input.tenant_id,
                    organization_id: input.organization_id,
                    conversation_id: input.conversation_id,
                    principal_id: input.user_id,
                    principal_kind: "user".to_owned(),
                    role,
                    invited_by: input.actor_user_id,
                },
                "system",
                attributes,
            )
            .map_err(|error| format!("{error:?}"))?;
        Ok(())
    }

    fn remove_group_member(&self, input: SyncSpaceGroupMemberInput) -> Result<(), String> {
        if input.role == "owner" {
            return Ok(());
        }
        let member_id = member_id(
            input.conversation_id.as_str(),
            "user",
            input.user_id.as_str(),
        );
        self.runtime
            .remove_member_with_actor_kind(
                RemoveConversationMemberCommand {
                    tenant_id: input.tenant_id,
                    organization_id: input.organization_id,
                    conversation_id: input.conversation_id,
                    member_id,
                    removed_by: input.actor_user_id,
                },
                "system",
            )
            .map_err(|error| format!("{error:?}"))?;
        Ok(())
    }

    fn transfer_group_owner(&self, input: TransferSpaceGroupOwnerInput) -> Result<(), String> {
        let target_member_id = member_id(
            input.conversation_id.as_str(),
            "user",
            input.new_owner_user_id.as_str(),
        );
        self.runtime
            .transfer_conversation_owner_with_actor_kind(
                TransferConversationOwnerCommand {
                    tenant_id: input.tenant_id,
                    organization_id: input.organization_id,
                    conversation_id: input.conversation_id,
                    target_member_id,
                    transferred_by: input.actor_user_id,
                },
                "system",
            )
            .map_err(|error| format!("{error:?}"))?;
        Ok(())
    }
}

impl SpaceChannelConversationBinder for ConversationServiceSpaceConversationBinder {
    fn create_channel_conversation(
        &self,
        input: CreateSpaceChannelConversationInput,
    ) -> Result<(), String> {
        self.runtime
            .create_system_channel_with_requester_kind(
                CreateSystemChannelCommand {
                    tenant_id: input.tenant_id,
                    organization_id: input.organization_id,
                    conversation_id: input.conversation_id,
                    requester_id: input.creator_user_id.clone(),
                    subscriber_id: input.creator_user_id,
                },
                "system",
            )
            .map_err(|error| format!("{error:?}"))?;
        Ok(())
    }
}

pub fn wire_space_conversation_binders(
    space_state: space_service::AppState,
    conversation_runtime: Arc<ConversationRuntime<ConversationCommitJournal>>,
) -> space_service::AppState {
    let binder = Arc::new(ConversationServiceSpaceConversationBinder::new(
        conversation_runtime,
    ));
    space_state
        .with_group_conversation_binder(binder.clone())
        .with_channel_conversation_binder(binder)
}
