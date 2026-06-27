use std::collections::{BTreeMap, HashMap};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MembershipRole {
    Owner,
    Admin,
    Member,
    Guest,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MembershipState {
    Joined,
    Invited,
    Linked,
    Left,
    Removed,
}

impl MembershipState {
    pub fn is_active(&self) -> bool {
        matches!(self, Self::Joined)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConversationMember {
    pub tenant_id: String,
    pub conversation_id: String,
    pub member_id: String,
    pub principal_id: String,
    pub principal_kind: String,
    pub role: MembershipRole,
    pub state: MembershipState,
    pub invited_by: Option<String>,
    pub joined_at: String,
    pub removed_at: Option<String>,
    pub attributes: BTreeMap<String, String>,
}

impl ConversationMember {
    pub fn is_active(&self) -> bool {
        self.state.is_active()
    }

    pub fn can_read_invited_history(&self) -> bool {
        matches!(
            self.state,
            MembershipState::Joined | MembershipState::Invited
        )
    }

    pub fn can_read_shared_history(&self) -> bool {
        self.is_active()
            || (matches!(self.state, MembershipState::Linked) && self.has_shared_history_anchor())
    }

    fn has_shared_history_anchor(&self) -> bool {
        self.attributes
            .get("sharedChannelPolicyId")
            .is_some_and(|value| !value.trim().is_empty())
            && self
                .attributes
                .get("externalConnectionId")
                .is_some_and(|value| !value.trim().is_empty())
            && self
                .attributes
                .get("externalMemberId")
                .is_some_and(|value| !value.trim().is_empty())
    }
}

// These helpers intentionally mirror the persisted membership record fields so
// runtime and projection call sites stay explicit when constructing roster
// state from events and recovery snapshots.
#[allow(clippy::too_many_arguments)]
pub fn build_conversation_member(
    tenant_id: &str,
    conversation_id: &str,
    member_id: String,
    principal_id: &str,
    principal_kind: &str,
    role: MembershipRole,
    invited_by: Option<String>,
    joined_at: String,
) -> ConversationMember {
    build_conversation_member_with_attributes(
        tenant_id,
        conversation_id,
        member_id,
        principal_id,
        principal_kind,
        role,
        invited_by,
        joined_at,
        BTreeMap::new(),
    )
}

#[allow(clippy::too_many_arguments)]
pub fn build_conversation_member_with_attributes(
    tenant_id: &str,
    conversation_id: &str,
    member_id: String,
    principal_id: &str,
    principal_kind: &str,
    role: MembershipRole,
    invited_by: Option<String>,
    joined_at: String,
    attributes: BTreeMap<String, String>,
) -> ConversationMember {
    ConversationMember {
        tenant_id: tenant_id.into(),
        conversation_id: conversation_id.into(),
        member_id,
        principal_id: principal_id.into(),
        principal_kind: principal_kind.into(),
        role,
        state: MembershipState::Joined,
        invited_by,
        joined_at,
        removed_at: None,
        attributes,
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConversationReadCursor {
    pub tenant_id: String,
    pub conversation_id: String,
    pub member_id: String,
    pub principal_id: String,
    pub principal_kind: String,
    pub read_seq: u64,
    pub last_read_message_id: Option<String>,
    pub updated_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConversationReadCursorView {
    pub tenant_id: String,
    pub conversation_id: String,
    pub member_id: String,
    pub principal_id: String,
    pub principal_kind: String,
    pub read_seq: u64,
    pub last_read_message_id: Option<String>,
    pub updated_at: String,
    pub unread_count: u64,
}

impl ConversationReadCursorView {
    pub fn from_cursor(cursor: &ConversationReadCursor, unread_count: u64) -> Self {
        Self {
            tenant_id: cursor.tenant_id.clone(),
            conversation_id: cursor.conversation_id.clone(),
            member_id: cursor.member_id.clone(),
            principal_id: cursor.principal_id.clone(),
            principal_kind: cursor.principal_kind.clone(),
            read_seq: cursor.read_seq,
            last_read_message_id: cursor.last_read_message_id.clone(),
            updated_at: cursor.updated_at.clone(),
            unread_count,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConversationPolicy {
    pub policy_version: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capability_flags: Option<Vec<String>>,
    pub history_visibility: String,
    pub retention_policy_ref: String,
}

impl Default for ConversationPolicy {
    fn default() -> Self {
        Self {
            policy_version: "default.v1".into(),
            capability_flags: None,
            history_visibility: "joined".into(),
            retention_policy_ref: "tenant.standard".into(),
        }
    }
}

impl ConversationPolicy {
    pub fn normalize(mut self) -> Result<Self, String> {
        self.policy_version = self.policy_version.trim().to_owned();
        self.history_visibility = self.history_visibility.trim().to_owned();
        self.retention_policy_ref = self.retention_policy_ref.trim().to_owned();

        if self.policy_version.is_empty() {
            return Err("conversation policy version must not be empty".into());
        }
        if self.retention_policy_ref.is_empty() {
            return Err("conversation retention policy ref must not be empty".into());
        }
        match self.history_visibility.as_str() {
            "joined" | "world_readable" | "invited" | "shared" => {}
            _ => {
                return Err(format!(
                    "unsupported conversation history visibility: {}",
                    self.history_visibility
                ));
            }
        }

        if let Some(flags) = self.capability_flags.as_mut() {
            for flag in flags.iter_mut() {
                *flag = flag.trim().to_owned();
                if flag.is_empty() {
                    return Err("conversation capability flag must not be empty".into());
                }
            }
            flags.sort();
            flags.dedup();
        }

        Ok(self)
    }

    pub fn allows_capability(&self, capability: &str) -> bool {
        match self.capability_flags.as_ref() {
            None => true,
            Some(flags) => flags.iter().any(|flag| flag == capability),
        }
    }
}

/// Returns whether a principal represented by `member` may read conversation history under
/// the configured `history_visibility` policy mode.
pub fn history_read_allowed(history_visibility: &str, member: Option<&ConversationMember>) -> bool {
    match history_visibility.trim() {
        "world_readable" => true,
        "joined" => member.is_some_and(ConversationMember::is_active),
        "invited" => member.is_some_and(ConversationMember::can_read_invited_history),
        "shared" => member.is_some_and(ConversationMember::can_read_shared_history),
        _ => false,
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConversationBusinessBinding {
    pub business_type: String,
    pub business_id: String,
}

pub fn build_default_read_cursor(member: &ConversationMember) -> ConversationReadCursor {
    ConversationReadCursor {
        tenant_id: member.tenant_id.clone(),
        conversation_id: member.conversation_id.clone(),
        member_id: member.member_id.clone(),
        principal_id: member.principal_id.clone(),
        principal_kind: member.principal_kind.clone(),
        read_seq: 0,
        last_read_message_id: None,
        updated_at: member.joined_at.clone(),
    }
}

pub fn principal_member_key(principal_id: &str, principal_kind: &str) -> String {
    encode_conversation_key_segments([principal_kind, principal_id])
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ConversationRoster {
    members: BTreeMap<String, ConversationMember>,
    principal_members: HashMap<String, String>,
    read_cursors: BTreeMap<String, ConversationReadCursor>,
}

impl ConversationRoster {
    pub fn members(&self) -> &BTreeMap<String, ConversationMember> {
        &self.members
    }

    pub fn members_mut(&mut self) -> &mut BTreeMap<String, ConversationMember> {
        &mut self.members
    }

    pub fn read_cursors(&self) -> &BTreeMap<String, ConversationReadCursor> {
        &self.read_cursors
    }

    pub fn read_cursors_mut(&mut self) -> &mut BTreeMap<String, ConversationReadCursor> {
        &mut self.read_cursors
    }

    pub fn active_principal_count(&self) -> usize {
        self.principal_members
            .values()
            .filter(|member_id| {
                self.members
                    .get(member_id.as_str())
                    .is_some_and(ConversationMember::is_active)
            })
            .count()
    }

    pub fn upsert_member(&mut self, member: ConversationMember) {
        self.principal_members.insert(
            principal_member_key(member.principal_id.as_str(), member.principal_kind.as_str()),
            member.member_id.clone(),
        );
        self.members.insert(member.member_id.clone(), member);
    }

    pub fn deactivate_member(&mut self, member: ConversationMember) {
        self.principal_members.remove(
            principal_member_key(member.principal_id.as_str(), member.principal_kind.as_str())
                .as_str(),
        );
        self.members.insert(member.member_id.clone(), member);
    }

    pub fn next_member_episode(&self, principal_id: &str, principal_kind: &str) -> u64 {
        self.members
            .values()
            .filter(|member| {
                member.principal_id == principal_id && member.principal_kind == principal_kind
            })
            .count() as u64
            + 1
    }

    pub fn resolve_active_member_id(&self, principal_id: &str) -> Option<String> {
        let member = self.resolve_current_member(principal_id)?;
        if !member.is_active() {
            return None;
        }

        Some(member.member_id)
    }

    pub fn resolve_active_member_id_with_kind(
        &self,
        principal_id: &str,
        principal_kind: &str,
    ) -> Option<String> {
        let member = self.resolve_current_member_with_kind(principal_id, principal_kind)?;
        if !member.is_active() {
            return None;
        }

        Some(member.member_id)
    }

    pub fn resolve_active_member(&self, principal_id: &str) -> Option<ConversationMember> {
        let member = self.resolve_current_member(principal_id)?;
        if !member.is_active() {
            return None;
        }

        Some(member)
    }

    pub fn resolve_active_member_with_kind(
        &self,
        principal_id: &str,
        principal_kind: &str,
    ) -> Option<ConversationMember> {
        let member = self.resolve_current_member_with_kind(principal_id, principal_kind)?;
        if !member.is_active() {
            return None;
        }

        Some(member)
    }

    pub fn resolve_current_member(&self, principal_id: &str) -> Option<ConversationMember> {
        let mut matches = self
            .principal_members
            .values()
            .filter_map(|member_id| self.members.get(member_id.as_str()))
            .filter(|member| member.principal_id == principal_id)
            .cloned();
        let member = matches.next()?;
        if matches.next().is_some() {
            return None;
        }

        Some(member)
    }

    pub fn resolve_current_member_with_kind(
        &self,
        principal_id: &str,
        principal_kind: &str,
    ) -> Option<ConversationMember> {
        let member_id = self
            .principal_members
            .get(principal_member_key(principal_id, principal_kind).as_str())?;
        self.members.get(member_id.as_str()).cloned()
    }

    pub fn resolve_history_visible_member(&self, principal_id: &str) -> Option<ConversationMember> {
        let member = self.resolve_current_member(principal_id)?;
        if !member.can_read_invited_history() {
            return None;
        }

        Some(member)
    }

    pub fn resolve_history_visible_member_with_kind(
        &self,
        principal_id: &str,
        principal_kind: &str,
    ) -> Option<ConversationMember> {
        let member = self.resolve_current_member_with_kind(principal_id, principal_kind)?;
        if !member.can_read_invited_history() {
            return None;
        }

        Some(member)
    }

    pub fn resolve_shared_history_visible_member(
        &self,
        principal_id: &str,
    ) -> Option<ConversationMember> {
        let member = self.resolve_current_member(principal_id)?;
        if !member.can_read_shared_history() {
            return None;
        }

        Some(member)
    }

    pub fn resolve_shared_history_visible_member_with_kind(
        &self,
        principal_id: &str,
        principal_kind: &str,
    ) -> Option<ConversationMember> {
        let member = self.resolve_current_member_with_kind(principal_id, principal_kind)?;
        if !member.can_read_shared_history() {
            return None;
        }

        Some(member)
    }

    pub fn member(&self, member_id: &str) -> Option<&ConversationMember> {
        self.members.get(member_id)
    }

    pub fn read_cursor(&self, member_id: &str) -> Option<&ConversationReadCursor> {
        self.read_cursors.get(member_id)
    }

    pub fn upsert_read_cursor(&mut self, cursor: ConversationReadCursor) {
        self.read_cursors.insert(cursor.member_id.clone(), cursor);
    }

    pub fn ensure_default_read_cursor(&mut self, member: &ConversationMember) {
        self.read_cursors
            .entry(member.member_id.clone())
            .or_insert_with(|| build_default_read_cursor(member));
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeAgentHandoffStatusView {
    pub id: String,
    pub kind: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentHandoffStateView {
    pub tenant_id: String,
    pub conversation_id: String,
    pub status: String,
    pub source: ChangeAgentHandoffStatusView,
    pub target: ChangeAgentHandoffStatusView,
    pub handoff_session_id: String,
    pub handoff_reason: Option<String>,
    pub accepted_at: Option<String>,
    pub accepted_by: Option<ChangeAgentHandoffStatusView>,
    pub resolved_at: Option<String>,
    pub resolved_by: Option<ChangeAgentHandoffStatusView>,
    pub closed_at: Option<String>,
    pub closed_by: Option<ChangeAgentHandoffStatusView>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ConversationHandoffTransitionOutcome {
    Idempotent,
    Mutated,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ConversationHandoffTransitionError {
    PermissionDenied(String),
    Conflict(String),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConversationHandoffLifecycle {
    Accept,
    Resolve,
    Close,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConversationHandoffStatusTransition {
    pub previous_status: String,
    pub ordering_seq: u64,
    pub outcome: ConversationHandoffTransitionOutcome,
    pub state: AgentHandoffStateView,
}

impl AgentHandoffStateView {
    pub fn is_closed(&self) -> bool {
        self.status == "closed"
    }

    pub fn accept(
        &mut self,
        actor: &ChangeAgentHandoffStatusView,
        changed_at: String,
    ) -> Result<ConversationHandoffTransitionOutcome, ConversationHandoffTransitionError> {
        if &self.target != actor {
            return Err(ConversationHandoffTransitionError::PermissionDenied(
                format!("actor {} is not the handoff target", actor.id),
            ));
        }
        if self.status == "accepted" && self.accepted_by.as_ref() == Some(actor) {
            return Ok(ConversationHandoffTransitionOutcome::Idempotent);
        }
        if self.status != "open" {
            return Err(ConversationHandoffTransitionError::Conflict(format!(
                "agent handoff cannot accept from status {}",
                self.status
            )));
        }

        self.status = "accepted".into();
        self.accepted_at = Some(changed_at);
        self.accepted_by = Some(actor.clone());
        Ok(ConversationHandoffTransitionOutcome::Mutated)
    }

    pub fn resolve(
        &mut self,
        actor: &ChangeAgentHandoffStatusView,
        changed_at: String,
    ) -> Result<ConversationHandoffTransitionOutcome, ConversationHandoffTransitionError> {
        if &self.target != actor {
            return Err(ConversationHandoffTransitionError::PermissionDenied(
                format!("actor {} is not the handoff target", actor.id),
            ));
        }
        if self.status == "resolved" && self.resolved_by.as_ref() == Some(actor) {
            return Ok(ConversationHandoffTransitionOutcome::Idempotent);
        }
        if self.status != "accepted" {
            return Err(ConversationHandoffTransitionError::Conflict(format!(
                "agent handoff cannot resolve from status {}",
                self.status
            )));
        }

        self.status = "resolved".into();
        self.resolved_at = Some(changed_at);
        self.resolved_by = Some(actor.clone());
        Ok(ConversationHandoffTransitionOutcome::Mutated)
    }

    pub fn close(
        &mut self,
        actor: &ChangeAgentHandoffStatusView,
        changed_at: String,
    ) -> Result<ConversationHandoffTransitionOutcome, ConversationHandoffTransitionError> {
        if &self.source != actor && &self.target != actor {
            return Err(ConversationHandoffTransitionError::PermissionDenied(
                format!("actor {} is neither handoff source nor target", actor.id),
            ));
        }
        if self.status == "closed" {
            return Ok(ConversationHandoffTransitionOutcome::Idempotent);
        }

        self.status = "closed".into();
        self.closed_at = Some(changed_at);
        self.closed_by = Some(actor.clone());
        Ok(ConversationHandoffTransitionOutcome::Mutated)
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConversationAggregateState {
    conversation_type: String,
    member_epoch: u64,
    policy_epoch: u64,
    policy: Option<ConversationPolicy>,
    business_binding: Option<ConversationBusinessBinding>,
    handoff_status_epoch: u64,
    handoff_state: Option<AgentHandoffStateView>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConversationScenario {
    Group,
    Direct,
    Thread,
    AgentDialog,
    AgentHandoff,
    SystemChannel,
    Unknown,
}

impl ConversationScenario {
    pub fn from_conversation_type(conversation_type: &str) -> Self {
        match conversation_type {
            "group" => Self::Group,
            "direct" => Self::Direct,
            "thread" => Self::Thread,
            "agent_dialog" => Self::AgentDialog,
            "agent_handoff" => Self::AgentHandoff,
            "system_channel" => Self::SystemChannel,
            _ => Self::Unknown,
        }
    }
}

impl ConversationAggregateState {
    pub fn new(conversation_type: impl Into<String>) -> Self {
        Self {
            conversation_type: conversation_type.into(),
            ..Self::default()
        }
    }

    pub fn new_agent_handoff(handoff_state: AgentHandoffStateView) -> Self {
        Self {
            conversation_type: "agent_handoff".into(),
            handoff_state: Some(handoff_state),
            ..Self::default()
        }
    }

    pub fn conversation_type(&self) -> &str {
        self.conversation_type.as_str()
    }

    pub fn scenario(&self) -> ConversationScenario {
        ConversationScenario::from_conversation_type(self.conversation_type.as_str())
    }

    pub fn member_epoch(&self) -> u64 {
        self.member_epoch
    }

    pub fn next_member_epoch(&mut self) -> u64 {
        self.member_epoch += 1;
        self.member_epoch
    }

    pub fn observe_member_epoch(&mut self, ordering_seq: u64) {
        self.member_epoch = self.member_epoch.max(ordering_seq);
    }

    pub fn policy_epoch(&self) -> u64 {
        self.policy_epoch
    }

    pub fn next_policy_epoch(&mut self) -> u64 {
        self.policy_epoch += 1;
        self.policy_epoch
    }

    pub fn observe_policy_epoch(&mut self, ordering_seq: u64) {
        self.policy_epoch = self.policy_epoch.max(ordering_seq);
    }

    pub fn policy(&self) -> Option<&ConversationPolicy> {
        self.policy.as_ref()
    }

    pub fn replace_policy(&mut self, policy: Option<ConversationPolicy>) {
        self.policy = policy;
    }

    pub fn business_binding(&self) -> Option<&ConversationBusinessBinding> {
        self.business_binding.as_ref()
    }

    pub fn replace_business_binding(
        &mut self,
        business_binding: Option<ConversationBusinessBinding>,
    ) {
        self.business_binding = business_binding;
    }

    pub fn handoff_status_epoch(&self) -> u64 {
        self.handoff_status_epoch
    }

    pub fn observe_handoff_status_epoch(&mut self, ordering_seq: u64) {
        self.handoff_status_epoch = self.handoff_status_epoch.max(ordering_seq);
    }

    pub fn handoff_state(&self) -> Option<&AgentHandoffStateView> {
        self.handoff_state.as_ref()
    }

    pub fn replace_handoff_state(&mut self, handoff_state: Option<AgentHandoffStateView>) {
        self.handoff_state = handoff_state;
    }

    pub fn has_closed_handoff(&self) -> bool {
        self.handoff_state
            .as_ref()
            .is_some_and(AgentHandoffStateView::is_closed)
    }

    pub fn transition_handoff_status(
        &mut self,
        action: ConversationHandoffLifecycle,
        actor: &ChangeAgentHandoffStatusView,
        changed_at: String,
    ) -> Result<ConversationHandoffStatusTransition, ConversationHandoffTransitionError> {
        let (previous_status, outcome, state) = {
            let handoff_state = self.handoff_state.as_mut().ok_or_else(|| {
                ConversationHandoffTransitionError::Conflict("agent handoff state missing".into())
            })?;
            let previous_status = handoff_state.status.clone();
            let outcome = match action {
                ConversationHandoffLifecycle::Accept => {
                    handoff_state.accept(actor, changed_at.clone())?
                }
                ConversationHandoffLifecycle::Resolve => {
                    handoff_state.resolve(actor, changed_at.clone())?
                }
                ConversationHandoffLifecycle::Close => handoff_state.close(actor, changed_at)?,
            };
            let state = handoff_state.clone();
            (previous_status, outcome, state)
        };

        let ordering_seq = if outcome == ConversationHandoffTransitionOutcome::Mutated {
            self.handoff_status_epoch += 1;
            self.handoff_status_epoch
        } else {
            self.handoff_status_epoch
        };

        Ok(ConversationHandoffStatusTransition {
            previous_status,
            ordering_seq,
            outcome,
            state,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConversationActorView {
    pub id: String,
    pub kind: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConversationAgentHandoffView {
    pub status: String,
    pub source: ConversationActorView,
    pub target: ConversationActorView,
    pub handoff_session_id: String,
    pub handoff_reason: Option<String>,
    pub accepted_at: Option<String>,
    pub accepted_by: Option<ConversationActorView>,
    pub resolved_at: Option<String>,
    pub resolved_by: Option<ConversationActorView>,
    pub closed_at: Option<String>,
    pub closed_by: Option<ConversationActorView>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConversationInboxPeerView {
    pub principal_kind: String,
    pub principal_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub relationship_state: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConversationInboxPreferencesView {
    pub is_pinned: bool,
    pub is_muted: bool,
    pub is_marked_unread: bool,
    pub is_hidden: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConversationInboxEntry {
    pub tenant_id: String,
    pub principal_id: String,
    pub member_id: String,
    pub conversation_id: String,
    pub conversation_type: String,
    pub message_count: u64,
    pub last_message_id: Option<String>,
    pub last_message_seq: u64,
    pub last_sender_id: Option<String>,
    pub last_sender_kind: Option<String>,
    pub last_summary: Option<String>,
    pub unread_count: u64,
    pub last_activity_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_source: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub peer: Option<ConversationInboxPeerView>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preferences: Option<ConversationInboxPreferencesView>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agent_handoff: Option<ConversationAgentHandoffView>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientRouteSyncFeedEntry {
    pub tenant_id: String,
    pub principal_id: String,
    pub device_id: String,
    pub sync_seq: u64,
    pub origin_event_id: String,
    pub origin_event_type: String,
    pub conversation_id: Option<String>,
    pub message_id: Option<String>,
    pub message_seq: Option<u64>,
    pub member_id: Option<String>,
    pub read_seq: Option<u64>,
    pub last_read_message_id: Option<String>,
    pub actor_id: Option<String>,
    pub actor_kind: Option<String>,
    pub actor_device_id: Option<String>,
    pub summary: Option<String>,
    pub payload_schema: Option<String>,
    pub payload: Option<String>,
    pub occurred_at: String,
}

pub fn member_id(conversation_id: &str, principal_kind: &str, principal_id: &str) -> String {
    member_episode_id(conversation_id, principal_kind, principal_id, 1)
}

pub fn member_episode_id(
    conversation_id: &str,
    principal_kind: &str,
    principal_id: &str,
    episode: u64,
) -> String {
    if episode <= 1 {
        return format!("cm_{conversation_id}_{principal_kind}_{principal_id}");
    }

    format!("cm_{conversation_id}_{principal_kind}_{principal_id}_e{episode}")
}

fn encode_conversation_key_segments<'a>(segments: impl IntoIterator<Item = &'a str>) -> String {
    let mut encoded = String::new();
    for segment in segments {
        encoded.push_str(segment.len().to_string().as_str());
        encoded.push('#');
        encoded.push_str(segment);
    }
    encoded
}
