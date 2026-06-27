use im_domain_core::conversation::{ConversationMember, ConversationScenario, MembershipRole};
use im_domain_core::message::Message;
use im_domain_core::room::{RoomKind, is_room_business_type, room_kind_from_business_type};

use super::{ConversationState, RuntimeError};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum MessagePostPolicy {
    GenericPost,
    SystemChannelPublish,
}

pub(super) fn ensure_generic_creatable_conversation_type(
    conversation_type: &str,
) -> Result<(), RuntimeError> {
    match ConversationScenario::from_conversation_type(conversation_type) {
        ConversationScenario::Group | ConversationScenario::Direct => Ok(()),
        ConversationScenario::Thread
        | ConversationScenario::AgentDialog
        | ConversationScenario::AgentHandoff
        | ConversationScenario::SystemChannel => Err(RuntimeError::ConversationTypeInvalid(
            format!("conversation type {conversation_type} requires a dedicated create command"),
        )),
        ConversationScenario::Unknown => Err(RuntimeError::ConversationTypeInvalid(format!(
            "unsupported conversation type: {conversation_type}"
        ))),
    }
}

pub(super) fn ensure_agent_dialog_requester_kind(requester_kind: &str) -> Result<(), RuntimeError> {
    if requester_kind == "user" {
        return Ok(());
    }

    Err(RuntimeError::PermissionDenied(format!(
        "agent dialog requires user requester, got {requester_kind}"
    )))
}

pub(super) fn ensure_agent_handoff_source_kind(source_kind: &str) -> Result<(), RuntimeError> {
    if source_kind == "agent" {
        return Ok(());
    }

    Err(RuntimeError::PermissionDenied(format!(
        "agent handoff requires agent source, got {source_kind}"
    )))
}

pub(super) fn ensure_agent_handoff_target_kind(target_kind: &str) -> Result<(), RuntimeError> {
    if matches!(target_kind, "user" | "agent") {
        return Ok(());
    }

    Err(RuntimeError::PermissionDenied(format!(
        "agent handoff target kind must be user or agent, got {target_kind}"
    )))
}

pub(super) fn ensure_system_channel_requester_kind(
    requester_kind: &str,
) -> Result<(), RuntimeError> {
    if requester_kind == "system" {
        return Ok(());
    }

    Err(RuntimeError::PermissionDenied(format!(
        "system channel requires system requester, got {requester_kind}"
    )))
}

pub(super) fn ensure_direct_chat_binding_requester_kind(
    requester_kind: &str,
) -> Result<(), RuntimeError> {
    if requester_kind == "system" {
        return Ok(());
    }

    Err(RuntimeError::PermissionDenied(format!(
        "direct chat binding requires system requester, got {requester_kind}"
    )))
}

pub(super) fn ensure_shared_channel_sync_requester_kind(
    requester_kind: &str,
) -> Result<(), RuntimeError> {
    if requester_kind == "system" {
        return Ok(());
    }

    Err(RuntimeError::PermissionDenied(format!(
        "shared channel linked-member sync requires system requester, got {requester_kind}"
    )))
}

pub(super) fn ensure_agent_handoff_conversation(
    conversation: &ConversationState,
) -> Result<(), RuntimeError> {
    if conversation.aggregate.scenario() == ConversationScenario::AgentHandoff {
        return Ok(());
    }

    Err(RuntimeError::ConversationTypeInvalid(format!(
        "conversation type {} is not agent_handoff",
        conversation.aggregate.conversation_type()
    )))
}

pub(super) fn ensure_actor_kind_matches_member(
    actor_member: &ConversationMember,
    actor_kind: &str,
) -> Result<(), RuntimeError> {
    if actor_member.principal_kind == actor_kind {
        return Ok(());
    }

    Err(RuntimeError::PermissionDenied(format!(
        "actor kind {} does not match member principal kind {}",
        actor_kind, actor_member.principal_kind
    )))
}

pub(super) fn is_closed_agent_handoff(conversation: &ConversationState) -> bool {
    conversation.aggregate.has_closed_handoff()
}

pub(super) fn ensure_member_add_actor_allowed(
    conversation: &ConversationState,
    actor_member: &ConversationMember,
) -> Result<(), RuntimeError> {
    match conversation.aggregate.scenario() {
        ConversationScenario::Group | ConversationScenario::Thread => {
            if matches!(
                actor_member.role,
                MembershipRole::Owner | MembershipRole::Admin
            ) {
                return Ok(());
            }

            Err(RuntimeError::PermissionDenied(format!(
                "member {} cannot add members in group conversation",
                actor_member.principal_id
            )))
        }
        ConversationScenario::Direct => {
            if matches!(actor_member.role, MembershipRole::Owner) {
                return Ok(());
            }

            Err(RuntimeError::PermissionDenied(format!(
                "member {} cannot add peer in direct conversation",
                actor_member.principal_id
            )))
        }
        _ => Err(RuntimeError::PermissionDenied(format!(
            "conversation type {} does not support generic member add",
            conversation.aggregate.conversation_type()
        ))),
    }
}

pub(super) fn ensure_member_add_request_allowed(
    conversation: &ConversationState,
    actor_member: &ConversationMember,
    requested_role: &MembershipRole,
) -> Result<(), RuntimeError> {
    match conversation.aggregate.scenario() {
        ConversationScenario::Group | ConversationScenario::Thread => match actor_member.role {
            MembershipRole::Owner => {
                if matches!(requested_role, MembershipRole::Owner) {
                    return Err(RuntimeError::PermissionDenied(
                        "group conversation does not support creating a second owner".into(),
                    ));
                }

                Ok(())
            }
            MembershipRole::Admin => {
                if matches!(
                    requested_role,
                    MembershipRole::Member | MembershipRole::Guest
                ) {
                    return Ok(());
                }

                Err(RuntimeError::PermissionDenied(format!(
                    "admin member {} cannot assign elevated role",
                    actor_member.principal_id
                )))
            }
            _ => Err(RuntimeError::PermissionDenied(format!(
                "member {} cannot add members",
                actor_member.principal_id
            ))),
        },
        ConversationScenario::Direct => {
            if conversation.roster.active_principal_count() >= 2 {
                return Err(RuntimeError::PermissionDenied(
                    "direct conversation already has the maximum number of active participants"
                        .into(),
                ));
            }
            if matches!(
                requested_role,
                MembershipRole::Owner | MembershipRole::Admin
            ) {
                return Err(RuntimeError::PermissionDenied(
                    "direct conversation peer cannot be assigned owner/admin role".into(),
                ));
            }

            Ok(())
        }
        _ => Err(RuntimeError::PermissionDenied(format!(
            "conversation type {} does not support generic member add",
            conversation.aggregate.conversation_type()
        ))),
    }
}

pub(super) fn ensure_member_remove_allowed(
    conversation: &ConversationState,
    actor_member: &ConversationMember,
    target_member: &ConversationMember,
) -> Result<(), RuntimeError> {
    match conversation.aggregate.scenario() {
        ConversationScenario::Group | ConversationScenario::Thread => match actor_member.role {
            MembershipRole::Owner => {
                if matches!(target_member.role, MembershipRole::Owner) {
                    return Err(RuntimeError::PermissionDenied(
                        "group conversation owner cannot be removed via remove_member".into(),
                    ));
                }

                Ok(())
            }
            MembershipRole::Admin => {
                if matches!(
                    target_member.role,
                    MembershipRole::Member | MembershipRole::Guest
                ) {
                    return Ok(());
                }

                Err(RuntimeError::PermissionDenied(format!(
                    "admin member {} cannot remove privileged member {}",
                    actor_member.principal_id, target_member.principal_id
                )))
            }
            _ => Err(RuntimeError::PermissionDenied(format!(
                "member {} cannot remove members in group conversation",
                actor_member.principal_id
            ))),
        },
        ConversationScenario::Direct => Err(RuntimeError::PermissionDenied(
            "direct conversation does not support member removal".into(),
        )),
        _ => Err(RuntimeError::PermissionDenied(format!(
            "conversation type {} does not support generic member removal",
            conversation.aggregate.conversation_type()
        ))),
    }
}

pub(super) fn ensure_current_active_member_target(
    conversation: &ConversationState,
    target_member: &ConversationMember,
) -> Result<(), RuntimeError> {
    let active_member_id = conversation
        .roster
        .resolve_active_member_id_with_kind(
            target_member.principal_id.as_str(),
            target_member.principal_kind.as_str(),
        )
        .ok_or_else(|| RuntimeError::MemberNotFound(target_member.member_id.clone()))?;
    if active_member_id != target_member.member_id.as_str() || !target_member.is_active() {
        return Err(RuntimeError::MemberNotFound(
            target_member.member_id.clone(),
        ));
    }

    Ok(())
}

pub(super) fn ensure_member_leave_allowed(
    conversation: &ConversationState,
    member: &ConversationMember,
) -> Result<(), RuntimeError> {
    match conversation.aggregate.scenario() {
        ConversationScenario::Group | ConversationScenario::Thread => {
            if matches!(member.role, MembershipRole::Owner) {
                return Err(RuntimeError::PermissionDenied(
                    "group conversation owner cannot leave until owner transfer is supported"
                        .into(),
                ));
            }

            Ok(())
        }
        ConversationScenario::Direct => Err(RuntimeError::PermissionDenied(
            "direct conversation does not support self leave".into(),
        )),
        _ => Err(RuntimeError::PermissionDenied(format!(
            "conversation type {} does not support generic self leave",
            conversation.aggregate.conversation_type()
        ))),
    }
}

pub(super) fn ensure_owner_transfer_allowed(
    conversation: &ConversationState,
    actor_member: &ConversationMember,
    target_member: &ConversationMember,
) -> Result<(), RuntimeError> {
    match conversation.aggregate.scenario() {
        ConversationScenario::Group | ConversationScenario::Thread => {
            if !matches!(actor_member.role, MembershipRole::Owner) {
                return Err(RuntimeError::PermissionDenied(format!(
                    "member {} cannot transfer group ownership",
                    actor_member.principal_id
                )));
            }
            if !target_member.is_active() {
                return Err(RuntimeError::PermissionDenied(
                    "owner transfer target must be an active member".into(),
                ));
            }
            if actor_member.member_id == target_member.member_id {
                return Err(RuntimeError::PermissionDenied(
                    "owner transfer target must be another active member".into(),
                ));
            }

            Ok(())
        }
        ConversationScenario::Direct => Err(RuntimeError::PermissionDenied(
            "direct conversation does not support owner transfer".into(),
        )),
        _ => Err(RuntimeError::PermissionDenied(format!(
            "conversation type {} does not support owner transfer",
            conversation.aggregate.conversation_type()
        ))),
    }
}

pub(super) fn ensure_member_role_change_allowed(
    conversation: &ConversationState,
    actor_member: &ConversationMember,
    target_member: &ConversationMember,
    requested_role: &MembershipRole,
) -> Result<(), RuntimeError> {
    match conversation.aggregate.scenario() {
        ConversationScenario::Group | ConversationScenario::Thread => {
            if !matches!(actor_member.role, MembershipRole::Owner) {
                return Err(RuntimeError::PermissionDenied(format!(
                    "member {} cannot change member roles in group conversation",
                    actor_member.principal_id
                )));
            }
            if matches!(target_member.role, MembershipRole::Owner)
                || matches!(requested_role, MembershipRole::Owner)
            {
                return Err(RuntimeError::PermissionDenied(
                    "group owner role must be changed via owner transfer".into(),
                ));
            }
            if target_member.role == *requested_role {
                return Err(RuntimeError::PermissionDenied(
                    "target member already has the requested role".into(),
                ));
            }

            Ok(())
        }
        ConversationScenario::Direct => Err(RuntimeError::PermissionDenied(
            "direct conversation does not support generic member role change".into(),
        )),
        _ => Err(RuntimeError::PermissionDenied(format!(
            "conversation type {} does not support generic member role change",
            conversation.aggregate.conversation_type()
        ))),
    }
}

pub(super) fn ensure_message_post_allowed(
    conversation: &ConversationState,
    actor_member: &ConversationMember,
) -> Result<(), RuntimeError> {
    if is_closed_agent_handoff(conversation) {
        return Err(RuntimeError::Conflict(format!(
            "agent handoff {} is already closed",
            actor_member.conversation_id
        )));
    }

    match conversation.aggregate.scenario() {
        ConversationScenario::Group
        | ConversationScenario::Thread
        | ConversationScenario::Direct
        | ConversationScenario::AgentDialog
        | ConversationScenario::AgentHandoff => Ok(()),
        ConversationScenario::SystemChannel => Err(RuntimeError::PermissionDenied(
            "system channel requires dedicated publish command".into(),
        )),
        ConversationScenario::Unknown => Err(RuntimeError::PermissionDenied(format!(
            "conversation type {} does not support message post",
            conversation.aggregate.conversation_type()
        ))),
    }
}

pub(super) fn ensure_system_channel_publish_command_allowed(
    conversation: &ConversationState,
    actor_member: &ConversationMember,
) -> Result<(), RuntimeError> {
    if conversation.aggregate.scenario() != ConversationScenario::SystemChannel {
        return Err(RuntimeError::ConversationTypeInvalid(format!(
            "conversation type {} does not support system channel publish",
            conversation.aggregate.conversation_type()
        )));
    }

    ensure_system_channel_publisher_write_allowed(actor_member, "system_channel.publish")
}

pub(super) fn ensure_message_edit_allowed(
    actor_id: &str,
    actor_member: &ConversationMember,
    scenario: ConversationScenario,
    handoff_closed: bool,
    message: &Message,
) -> Result<(), RuntimeError> {
    if scenario == ConversationScenario::AgentHandoff && handoff_closed {
        return Err(RuntimeError::Conflict(format!(
            "agent handoff {} is already closed",
            actor_member.conversation_id
        )));
    }
    if message.sender.id == actor_id {
        return Ok(());
    }

    Err(RuntimeError::PermissionDenied(format!(
        "member {} cannot edit message owned by {}",
        actor_member.principal_id, message.sender.id
    )))
}

pub(super) fn ensure_message_recall_allowed(
    actor_id: &str,
    actor_member: &ConversationMember,
    scenario: ConversationScenario,
    handoff_closed: bool,
    message: &Message,
) -> Result<(), RuntimeError> {
    if scenario == ConversationScenario::AgentHandoff && handoff_closed {
        return Err(RuntimeError::Conflict(format!(
            "agent handoff {} is already closed",
            actor_member.conversation_id
        )));
    }
    if message.sender.id == actor_id {
        return Ok(());
    }

    if matches!(
        scenario,
        ConversationScenario::Group | ConversationScenario::Thread
    ) && matches!(
        actor_member.role,
        MembershipRole::Owner | MembershipRole::Admin
    ) {
        return Ok(());
    }

    Err(RuntimeError::PermissionDenied(format!(
        "member {} cannot recall message owned by {}",
        actor_member.principal_id, message.sender.id
    )))
}

pub(super) fn ensure_message_reaction_allowed(
    conversation: &ConversationState,
    actor_member: &ConversationMember,
) -> Result<(), RuntimeError> {
    ensure_conversation_bound_write_allowed(conversation, actor_member, "message.reaction")?;
    ensure_message_post_allowed(conversation, actor_member)
}

pub(super) fn ensure_message_pin_allowed(
    conversation: &ConversationState,
    actor_member: &ConversationMember,
) -> Result<(), RuntimeError> {
    ensure_conversation_bound_write_allowed(conversation, actor_member, "message.pin")?;

    match conversation.aggregate.scenario() {
        ConversationScenario::Group | ConversationScenario::Thread => {
            if matches!(
                actor_member.role,
                MembershipRole::Owner | MembershipRole::Admin
            ) {
                return Ok(());
            }

            Err(RuntimeError::PermissionDenied(format!(
                "member {} cannot pin messages in group conversation",
                actor_member.principal_id
            )))
        }
        ConversationScenario::Direct => {
            if matches!(actor_member.role, MembershipRole::Owner) {
                return Ok(());
            }

            Err(RuntimeError::PermissionDenied(format!(
                "member {} cannot pin messages in direct conversation",
                actor_member.principal_id
            )))
        }
        _ => Err(RuntimeError::PermissionDenied(format!(
            "conversation type {} does not support message pin",
            conversation.aggregate.conversation_type()
        ))),
    }
}

pub(super) fn ensure_conversation_policy_write_allowed(
    conversation: &ConversationState,
    actor_member: &ConversationMember,
) -> Result<(), RuntimeError> {
    match conversation.aggregate.scenario() {
        ConversationScenario::Group | ConversationScenario::Thread => {
            if matches!(
                actor_member.role,
                MembershipRole::Owner | MembershipRole::Admin
            ) {
                return Ok(());
            }

            Err(RuntimeError::PermissionDenied(format!(
                "member {} cannot update conversation policy in group conversation",
                actor_member.principal_id
            )))
        }
        ConversationScenario::Direct
        | ConversationScenario::AgentDialog
        | ConversationScenario::AgentHandoff => {
            if matches!(actor_member.role, MembershipRole::Owner) {
                return Ok(());
            }

            Err(RuntimeError::PermissionDenied(format!(
                "member {} cannot update conversation policy in {} conversation",
                actor_member.principal_id,
                conversation.aggregate.conversation_type()
            )))
        }
        ConversationScenario::SystemChannel => {
            ensure_system_channel_publisher_write_allowed(actor_member, "conversation.policy.write")
        }
        ConversationScenario::Unknown => Err(RuntimeError::PermissionDenied(format!(
            "conversation type {} does not support conversation policy update",
            conversation.aggregate.conversation_type()
        ))),
    }
}

pub(super) fn ensure_conversation_bound_write_allowed(
    conversation: &ConversationState,
    actor_member: &ConversationMember,
    capability: &str,
) -> Result<(), RuntimeError> {
    if conversation.aggregate.scenario() == ConversationScenario::AgentHandoff
        && is_closed_agent_handoff(conversation)
    {
        return Err(RuntimeError::Conflict(format!(
            "agent handoff {} is already closed for {capability}",
            actor_member.conversation_id
        )));
    }

    if conversation.aggregate.scenario() == ConversationScenario::SystemChannel {
        ensure_system_channel_publisher_write_allowed(actor_member, capability)?;
    }

    if let Some(policy) = conversation.aggregate.policy()
        && !policy.allows_capability(capability)
    {
        return Err(RuntimeError::PermissionDenied(format!(
            "conversation {} policy disables {capability}",
            actor_member.conversation_id
        )));
    }

    Ok(())
}

pub(super) fn ensure_history_read_allowed(
    conversation: &ConversationState,
    principal_id: &str,
) -> Result<(), RuntimeError> {
    let history_visibility = conversation
        .aggregate
        .policy()
        .map(|policy| policy.history_visibility.as_str())
        .unwrap_or("joined");

    match history_visibility {
        "world_readable" => Ok(()),
        "joined" => {
            if conversation
                .roster
                .resolve_active_member(principal_id)
                .is_some()
            {
                return Ok(());
            }

            Err(RuntimeError::PermissionDenied(format!(
                "principal is not allowed to read conversation history: {principal_id}"
            )))
        }
        "invited" => {
            if conversation
                .roster
                .resolve_history_visible_member(principal_id)
                .is_some()
            {
                return Ok(());
            }

            Err(RuntimeError::PermissionDenied(format!(
                "principal is not allowed to read conversation history: {principal_id}"
            )))
        }
        "shared" => {
            if conversation
                .roster
                .resolve_shared_history_visible_member(principal_id)
                .is_some()
            {
                return Ok(());
            }

            Err(RuntimeError::PermissionDenied(format!(
                "principal is not allowed to read conversation history: {principal_id}"
            )))
        }
        _ => Err(RuntimeError::PermissionDenied(format!(
            "unsupported conversation history visibility: {history_visibility}"
        ))),
    }
}

pub(super) fn ensure_history_read_allowed_with_kind(
    conversation: &ConversationState,
    principal_id: &str,
    principal_kind: &str,
) -> Result<(), RuntimeError> {
    let history_visibility = conversation
        .aggregate
        .policy()
        .map(|policy| policy.history_visibility.as_str())
        .unwrap_or("joined");

    match history_visibility {
        "world_readable" => Ok(()),
        "joined" => {
            if conversation
                .roster
                .resolve_active_member_with_kind(principal_id, principal_kind)
                .is_some()
            {
                return Ok(());
            }

            Err(RuntimeError::PermissionDenied(format!(
                "principal is not allowed to read conversation history: {principal_kind}:{principal_id}"
            )))
        }
        "invited" => {
            if conversation
                .roster
                .resolve_history_visible_member_with_kind(principal_id, principal_kind)
                .is_some()
            {
                return Ok(());
            }

            Err(RuntimeError::PermissionDenied(format!(
                "principal is not allowed to read conversation history: {principal_kind}:{principal_id}"
            )))
        }
        "shared" => {
            if conversation
                .roster
                .resolve_shared_history_visible_member_with_kind(principal_id, principal_kind)
                .is_some()
            {
                return Ok(());
            }

            Err(RuntimeError::PermissionDenied(format!(
                "principal is not allowed to read conversation history: {principal_kind}:{principal_id}"
            )))
        }
        _ => Err(RuntimeError::PermissionDenied(format!(
            "unsupported conversation history visibility: {history_visibility}"
        ))),
    }
}

fn ensure_system_channel_publisher_write_allowed(
    actor_member: &ConversationMember,
    capability: &str,
) -> Result<(), RuntimeError> {
    let is_publisher = actor_member.principal_kind == "system"
        && actor_member
            .attributes
            .get("channelRole")
            .map(String::as_str)
            == Some("publisher");
    if is_publisher {
        return Ok(());
    }

    Err(RuntimeError::PermissionDenied(format!(
        "member {} cannot perform {capability} in system channel",
        actor_member.principal_id
    )))
}

pub(super) fn ensure_room_enter_allowed(
    conversation: &ConversationState,
    room_kind: RoomKind,
) -> Result<(), RuntimeError> {
    let binding = conversation
        .aggregate
        .business_binding()
        .ok_or_else(|| RuntimeError::ConversationBindingNotFound(
            "room enter requires a business binding".into(),
        ))?;
    if !is_room_business_type(binding.business_type.as_str()) {
        return Err(RuntimeError::InvalidInput(format!(
            "conversation business type {} is not a room binding",
            binding.business_type
        )));
    }
    let active_count = conversation.roster.active_principal_count();
    let max_members = room_kind.default_max_members();
    if active_count >= max_members {
        return Err(RuntimeError::PermissionDenied(format!(
            "room has reached the maximum active member count ({max_members})"
        )));
    }
    Ok(())
}

pub(super) fn ensure_room_message_post_allowed(
    conversation: &ConversationState,
    actor_member: &ConversationMember,
) -> Result<(), RuntimeError> {
    let Some(binding) = conversation.aggregate.business_binding() else {
        return Ok(());
    };
    let Some(room_kind) = room_kind_from_business_type(binding.business_type.as_str()) else {
        return Ok(());
    };

    if !actor_member.is_active() {
        return Err(RuntimeError::PermissionDenied(format!(
            "inactive member {} cannot post in room",
            actor_member.principal_id
        )));
    }

    if room_kind == RoomKind::Live {
        ensure_live_room_message_rate_allowed(actor_member)?;
    }

    Ok(())
}

fn ensure_live_room_message_rate_allowed(actor_member: &ConversationMember) -> Result<(), RuntimeError> {
    use std::collections::BTreeMap;
    use std::sync::{Mutex, OnceLock};
    use std::time::{Duration, Instant};

    const LIVE_ROOM_MESSAGE_RATE_LIMIT_ENV: &str = "SDKWORK_IM_LIVE_ROOM_MESSAGE_RATE_LIMIT";
    const LIVE_ROOM_MESSAGE_RATE_WINDOW_MS: u64 = 1_000;
    const LIVE_ROOM_MESSAGE_RATE_DEFAULT: u32 = 5;
    const LIVE_ROOM_MESSAGE_RATE_MAX: u32 = 60;

    #[derive(Default)]
    struct LiveRoomRateState {
        buckets: BTreeMap<String, (u32, Instant)>,
    }

    static STATE: OnceLock<Mutex<LiveRoomRateState>> = OnceLock::new();

    fn rate_limit() -> u32 {
        std::env::var(LIVE_ROOM_MESSAGE_RATE_LIMIT_ENV)
            .ok()
            .and_then(|value| value.parse::<u32>().ok())
            .filter(|value| *value > 0)
            .map(|value| value.min(LIVE_ROOM_MESSAGE_RATE_MAX))
            .unwrap_or(LIVE_ROOM_MESSAGE_RATE_DEFAULT)
    }

    let key = format!(
        "{}:{}",
        actor_member.principal_kind, actor_member.principal_id
    );
    let limit = rate_limit();
    let now = Instant::now();
    let mut state = STATE
        .get_or_init(|| Mutex::new(LiveRoomRateState::default()))
        .lock()
        .expect("live room rate limiter should lock");

    if let Some((count, window_start)) = state.buckets.get_mut(key.as_str()) {
        if now.duration_since(*window_start) > Duration::from_millis(LIVE_ROOM_MESSAGE_RATE_WINDOW_MS)
        {
            *count = 0;
            *window_start = now;
        }
        if *count >= limit {
            return Err(RuntimeError::PermissionDenied(format!(
                "live room message rate limit exceeded ({limit} messages per second)"
            )));
        }
        *count += 1;
    } else {
        state.buckets.insert(key, (1, now));
    }

    Ok(())
}
