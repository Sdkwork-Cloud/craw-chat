use im_domain_events::normalize_commit_organization_id;

use super::*;

pub(super) fn conversation_scope_key(tenant_id: &str, organization_id: &str, conversation_id: &str) -> String {
    encode_conversation_key_segments([
        tenant_id,
        normalize_commit_organization_id(organization_id).as_str(),
        conversation_id,
    ])
}

pub(super) fn conversation_scope_key_for_envelope(envelope: &CommitEnvelope) -> String {
    conversation_scope_key(
        envelope.tenant_id.as_str(),
        envelope.organization_id.as_str(),
        envelope.scope_id.as_str(),
    )
}

pub(super) fn conversation_business_scope_key(
    tenant_id: &str,
    business_type: &str,
    business_id: &str,
) -> String {
    encode_conversation_key_segments([tenant_id, business_type, business_id])
}

pub(super) fn encode_conversation_key_segments<'a>(
    segments: impl IntoIterator<Item = &'a str>,
) -> String {
    let mut encoded = String::new();
    for segment in segments {
        encoded.push_str(segment.len().to_string().as_str());
        encoded.push('#');
        encoded.push_str(segment);
    }
    encoded
}

pub(super) fn retention_class_from_policy_ref(retention_policy_ref: &str) -> String {
    im_domain_core::retention::retention_class_from_policy_ref(retention_policy_ref)
}

pub(super) fn conversation_retention_class(conversation: &ConversationState) -> String {
    conversation
        .aggregate
        .policy()
        .map(|policy| retention_class_from_policy_ref(policy.retention_policy_ref.as_str()))
        .unwrap_or_else(|| "standard".into())
}

pub(super) fn upsert_member(conversation: &mut ConversationState, member: ConversationMember) {
    conversation.roster.upsert_member(member);
}

pub(super) fn next_member_episode(
    conversation: &ConversationState,
    principal_id: &str,
    principal_kind: &str,
) -> u64 {
    conversation
        .roster
        .next_member_episode(principal_id, principal_kind)
}

pub(super) fn resolve_active_member_id(
    conversation: &ConversationState,
    principal_id: &str,
) -> Result<String, RuntimeError> {
    conversation
        .roster
        .resolve_active_member_id(principal_id)
        .ok_or_else(|| {
            RuntimeError::PermissionDenied(format!(
                "principal is not active conversation member: {principal_id}"
            ))
        })
}

pub(super) fn resolve_active_member_id_with_kind(
    conversation: &ConversationState,
    principal_id: &str,
    principal_kind: &str,
) -> Result<String, RuntimeError> {
    conversation
        .roster
        .resolve_active_member_id_with_kind(principal_id, principal_kind)
        .ok_or_else(|| {
            RuntimeError::PermissionDenied(format!(
                "principal is not active conversation member: {principal_kind}:{principal_id}"
            ))
        })
}

pub(super) fn resolve_active_member(
    conversation: &ConversationState,
    principal_id: &str,
) -> Result<ConversationMember, RuntimeError> {
    conversation
        .roster
        .resolve_active_member(principal_id)
        .ok_or_else(|| {
            RuntimeError::PermissionDenied(format!(
                "principal is not active conversation member: {principal_id}"
            ))
        })
}

pub(super) fn resolve_active_member_with_kind(
    conversation: &ConversationState,
    principal_id: &str,
    principal_kind: &str,
) -> Result<ConversationMember, RuntimeError> {
    conversation
        .roster
        .resolve_active_member_with_kind(principal_id, principal_kind)
        .ok_or_else(|| {
            RuntimeError::PermissionDenied(format!(
                "principal is not active conversation member: {principal_kind}:{principal_id}"
            ))
        })
}

pub(super) fn upsert_read_cursor(
    conversation: &mut ConversationState,
    cursor: ConversationReadCursor,
) {
    conversation.roster.upsert_read_cursor(cursor);
}

#[allow(clippy::too_many_arguments)]
pub(super) fn build_member_envelope(
    tenant_id: &str,
    organization_id: &str,
    conversation_id: &str,
    event_type: &'static str,
    member: ConversationMember,
    ordering_seq: u64,
    retention_class: &str,
    actor_id: &str,
    actor_kind: &str,
) -> CommitEnvelope {
    let event_suffix = match event_type {
        "conversation.member_removed" => "removed",
        "conversation.member_left" => "left",
        _ => "joined",
    };
    let event_timestamp = if matches!(
        event_type,
        "conversation.member_removed" | "conversation.member_left"
    ) {
        member
            .removed_at
            .clone()
            .unwrap_or_else(|| member.joined_at.clone())
    } else {
        member.joined_at.clone()
    };

    CommitEnvelope {
        event_id: format!("evt_{}_{}", member.member_id, event_suffix),
        tenant_id: tenant_id.into(),
        organization_id: normalize_commit_organization_id(organization_id),
        event_type: event_type.into(),
        event_version: 1,
        aggregate_type: AggregateType::Conversation,
        aggregate_id: conversation_id.into(),
        scope_type: "conversation".into(),
        scope_id: conversation_id.into(),
        ordering_key: CommitEnvelope::ordering_key(tenant_id, conversation_id),
        ordering_seq,
        causation_id: None,
        correlation_id: None,
        idempotency_key: None,
        actor: EventActor {
            actor_id: actor_id.into(),
            actor_kind: actor_kind.into(),
            actor_session_id: None,
        },
        occurred_at: event_timestamp.clone(),
        committed_at: event_timestamp,
        payload_schema: Some("conversation.member.v1".into()),
        payload: serde_json::to_string(&member)
            .expect("conversation member payload should serialize into commit envelope"),
        retention_class: retention_class.into(),
        audit_class: "default".into(),
    }
}

pub(super) fn build_read_cursor_envelope(
    tenant_id: &str,
    organization_id: &str,
    conversation_id: &str,
    cursor: ConversationReadCursor,
    ordering_seq: u64,
    retention_class: &str,
    actor_id: &str,
    actor_kind: &str,
) -> CommitEnvelope {
    CommitEnvelope {
        event_id: format!("evt_{}_cursor_{}", cursor.member_id, ordering_seq),
        tenant_id: tenant_id.into(),
        organization_id: normalize_commit_organization_id(organization_id),
        event_type: "conversation.read_cursor_updated".into(),
        event_version: 1,
        aggregate_type: AggregateType::Conversation,
        aggregate_id: conversation_id.into(),
        scope_type: "conversation".into(),
        scope_id: conversation_id.into(),
        ordering_key: CommitEnvelope::ordering_key(tenant_id, conversation_id),
        ordering_seq,
        causation_id: None,
        correlation_id: None,
        idempotency_key: None,
        actor: EventActor {
            actor_id: actor_id.into(),
            actor_kind: actor_kind.into(),
            actor_session_id: None,
        },
        occurred_at: cursor.updated_at.clone(),
        committed_at: cursor.updated_at.clone(),
        payload_schema: Some("conversation.read_cursor.v1".into()),
        payload: serde_json::to_string(&cursor)
            .expect("read cursor payload should serialize into commit envelope"),
        retention_class: retention_class.into(),
        audit_class: "default".into(),
    }
}

pub(super) fn build_conversation_policy_applied_envelope(
    tenant_id: &str,
    organization_id: &str,
    payload: ConversationPolicyAppliedPayload,
    ordering_seq: u64,
    applied_at: &str,
    actor_id: &str,
    actor_kind: &str,
) -> CommitEnvelope {
    CommitEnvelope {
        event_id: format!("evt_{}_policy_{}", payload.conversation_id, ordering_seq),
        tenant_id: tenant_id.into(),
        organization_id: normalize_commit_organization_id(organization_id),
        event_type: "conversation.policy_applied".into(),
        event_version: 1,
        aggregate_type: AggregateType::Conversation,
        aggregate_id: payload.conversation_id.clone(),
        scope_type: "conversation".into(),
        scope_id: payload.conversation_id.clone(),
        ordering_key: CommitEnvelope::ordering_key(tenant_id, payload.conversation_id.as_str()),
        ordering_seq,
        causation_id: None,
        correlation_id: None,
        idempotency_key: None,
        actor: EventActor {
            actor_id: actor_id.into(),
            actor_kind: actor_kind.into(),
            actor_session_id: None,
        },
        occurred_at: applied_at.into(),
        committed_at: applied_at.into(),
        payload_schema: Some("conversation.policy_applied.v1".into()),
        payload: serde_json::to_string(&payload)
            .expect("conversation policy payload should serialize into commit envelope"),
        retention_class: retention_class_from_policy_ref(payload.retention_policy_ref.as_str()),
        audit_class: "default".into(),
    }
}

pub(super) fn build_owner_transfer_envelope(
    payload: TransferConversationOwnerPayload,
    ordering_seq: u64,
    retention_class: &str,
    actor_id: &str,
    actor_kind: &str,
) -> CommitEnvelope {
    CommitEnvelope {
        event_id: format!(
            "evt_{}_owner_transfer_{}",
            payload.conversation_id, ordering_seq
        ),
        tenant_id: payload.tenant_id.clone(),
        organization_id: normalize_commit_organization_id(payload.organization_id.as_str()),
        event_type: "conversation.owner_transferred".into(),
        event_version: 1,
        aggregate_type: AggregateType::Conversation,
        aggregate_id: payload.conversation_id.clone(),
        scope_type: "conversation".into(),
        scope_id: payload.conversation_id.clone(),
        ordering_key: CommitEnvelope::ordering_key(
            payload.tenant_id.as_str(),
            payload.conversation_id.as_str(),
        ),
        ordering_seq,
        causation_id: None,
        correlation_id: None,
        idempotency_key: None,
        actor: EventActor {
            actor_id: actor_id.into(),
            actor_kind: actor_kind.into(),
            actor_session_id: None,
        },
        occurred_at: payload.transferred_at.clone(),
        committed_at: payload.transferred_at.clone(),
        payload_schema: Some("conversation.owner_transferred.v1".into()),
        payload: serde_json::to_string(&payload)
            .expect("owner transfer payload should serialize into commit envelope"),
        retention_class: retention_class.into(),
        audit_class: "default".into(),
    }
}

pub(super) fn build_member_role_changed_envelope(
    payload: ChangeConversationMemberRolePayload,
    ordering_seq: u64,
    retention_class: &str,
    actor_id: &str,
    actor_kind: &str,
) -> CommitEnvelope {
    CommitEnvelope {
        event_id: format!(
            "evt_{}_role_change_{}",
            payload.updated_member.member_id, ordering_seq
        ),
        tenant_id: payload.tenant_id.clone(),
        organization_id: normalize_commit_organization_id(payload.organization_id.as_str()),
        event_type: "conversation.member_role_changed".into(),
        event_version: 1,
        aggregate_type: AggregateType::Conversation,
        aggregate_id: payload.conversation_id.clone(),
        scope_type: "conversation".into(),
        scope_id: payload.conversation_id.clone(),
        ordering_key: CommitEnvelope::ordering_key(
            payload.tenant_id.as_str(),
            payload.conversation_id.as_str(),
        ),
        ordering_seq,
        causation_id: None,
        correlation_id: None,
        idempotency_key: None,
        actor: EventActor {
            actor_id: actor_id.into(),
            actor_kind: actor_kind.into(),
            actor_session_id: None,
        },
        occurred_at: payload.changed_at.clone(),
        committed_at: payload.changed_at.clone(),
        payload_schema: Some("conversation.member_role_changed.v1".into()),
        payload: serde_json::to_string(&payload)
            .expect("member role changed payload should serialize into commit envelope"),
        retention_class: retention_class.into(),
        audit_class: "default".into(),
    }
}

pub(super) fn build_agent_handoff_status_changed_envelope(
    payload: AgentHandoffStatusChangedPayload,
    ordering_seq: u64,
    retention_class: &str,
    actor_id: &str,
    actor_kind: &str,
) -> CommitEnvelope {
    CommitEnvelope {
        event_id: format!(
            "evt_{}_agent_handoff_status_{}",
            payload.conversation_id, ordering_seq
        ),
        tenant_id: payload.tenant_id.clone(),
        organization_id: normalize_commit_organization_id(payload.organization_id.as_str()),
        event_type: "conversation.agent_handoff_status_changed".into(),
        event_version: 1,
        aggregate_type: AggregateType::Conversation,
        aggregate_id: payload.conversation_id.clone(),
        scope_type: "conversation".into(),
        scope_id: payload.conversation_id.clone(),
        ordering_key: CommitEnvelope::ordering_key(
            payload.tenant_id.as_str(),
            payload.conversation_id.as_str(),
        ),
        ordering_seq,
        causation_id: None,
        correlation_id: None,
        idempotency_key: None,
        actor: EventActor {
            actor_id: actor_id.into(),
            actor_kind: actor_kind.into(),
            actor_session_id: None,
        },
        occurred_at: payload.changed_at.clone(),
        committed_at: payload.changed_at.clone(),
        payload_schema: Some("conversation.agent_handoff_status_changed.v1".into()),
        payload: serde_json::to_string(&payload)
            .expect("agent handoff status payload should serialize into commit envelope"),
        retention_class: retention_class.into(),
        audit_class: "default".into(),
    }
}

pub(super) fn build_message_edited_envelope(
    message: &MessageEdited,
    organization_id: &str,
    event_id: &str,
    retention_class: &str,
) -> CommitEnvelope {
    CommitEnvelope {
        event_id: event_id.into(),
        tenant_id: message.tenant_id.clone(),
        organization_id: normalize_commit_organization_id(organization_id),
        event_type: "message.edited".into(),
        event_version: 1,
        aggregate_type: AggregateType::Conversation,
        aggregate_id: message.conversation_id.clone(),
        scope_type: "conversation".into(),
        scope_id: message.conversation_id.clone(),
        ordering_key: CommitEnvelope::ordering_key(
            message.tenant_id.as_str(),
            message.conversation_id.as_str(),
        ),
        ordering_seq: message.message_seq,
        causation_id: None,
        correlation_id: None,
        idempotency_key: None,
        actor: EventActor {
            actor_id: message.editor.id.clone(),
            actor_kind: message.editor.kind.clone(),
            actor_session_id: message.editor.session_id.clone(),
        },
        occurred_at: message.edited_at.clone(),
        committed_at: message.edited_at.clone(),
        payload_schema: Some("message.edited.v1".into()),
        payload: serde_json::to_string(message)
            .expect("message edited payload should serialize into commit envelope"),
        retention_class: retention_class.into(),
        audit_class: "default".into(),
    }
}

pub(super) fn build_message_recalled_envelope(
    message: &MessageRecalled,
    organization_id: &str,
    event_id: &str,
    retention_class: &str,
) -> CommitEnvelope {
    CommitEnvelope {
        event_id: event_id.into(),
        tenant_id: message.tenant_id.clone(),
        organization_id: normalize_commit_organization_id(organization_id),
        event_type: "message.recalled".into(),
        event_version: 1,
        aggregate_type: AggregateType::Conversation,
        aggregate_id: message.conversation_id.clone(),
        scope_type: "conversation".into(),
        scope_id: message.conversation_id.clone(),
        ordering_key: CommitEnvelope::ordering_key(
            message.tenant_id.as_str(),
            message.conversation_id.as_str(),
        ),
        ordering_seq: message.message_seq,
        causation_id: None,
        correlation_id: None,
        idempotency_key: None,
        actor: EventActor {
            actor_id: message.recalled_by.id.clone(),
            actor_kind: message.recalled_by.kind.clone(),
            actor_session_id: message.recalled_by.session_id.clone(),
        },
        occurred_at: message.recalled_at.clone(),
        committed_at: message.recalled_at.clone(),
        payload_schema: Some("message.recalled.v1".into()),
        payload: serde_json::to_string(message)
            .expect("message recalled payload should serialize into commit envelope"),
        retention_class: retention_class.into(),
        audit_class: "default".into(),
    }
}

pub(super) fn build_message_reaction_added_envelope(
    message: &MessageReactionAdded,
    organization_id: &str,
    event_id: &str,
    retention_class: &str,
) -> CommitEnvelope {
    CommitEnvelope {
        event_id: event_id.into(),
        tenant_id: message.tenant_id.clone(),
        organization_id: normalize_commit_organization_id(organization_id),
        event_type: "message.reaction_added".into(),
        event_version: 1,
        aggregate_type: AggregateType::Conversation,
        aggregate_id: message.conversation_id.clone(),
        scope_type: "conversation".into(),
        scope_id: message.conversation_id.clone(),
        ordering_key: CommitEnvelope::ordering_key(
            message.tenant_id.as_str(),
            message.conversation_id.as_str(),
        ),
        ordering_seq: message.message_seq,
        causation_id: None,
        correlation_id: None,
        idempotency_key: None,
        actor: EventActor {
            actor_id: message.reacted_by.id.clone(),
            actor_kind: message.reacted_by.kind.clone(),
            actor_session_id: message.reacted_by.session_id.clone(),
        },
        occurred_at: message.reacted_at.clone(),
        committed_at: message.reacted_at.clone(),
        payload_schema: Some("message.reaction_added.v1".into()),
        payload: serde_json::to_string(message)
            .expect("message reaction added payload should serialize into commit envelope"),
        retention_class: retention_class.into(),
        audit_class: "default".into(),
    }
}

pub(super) fn build_message_reaction_removed_envelope(
    message: &MessageReactionRemoved,
    organization_id: &str,
    event_id: &str,
    retention_class: &str,
) -> CommitEnvelope {
    CommitEnvelope {
        event_id: event_id.into(),
        tenant_id: message.tenant_id.clone(),
        organization_id: normalize_commit_organization_id(organization_id),
        event_type: "message.reaction_removed".into(),
        event_version: 1,
        aggregate_type: AggregateType::Conversation,
        aggregate_id: message.conversation_id.clone(),
        scope_type: "conversation".into(),
        scope_id: message.conversation_id.clone(),
        ordering_key: CommitEnvelope::ordering_key(
            message.tenant_id.as_str(),
            message.conversation_id.as_str(),
        ),
        ordering_seq: message.message_seq,
        causation_id: None,
        correlation_id: None,
        idempotency_key: None,
        actor: EventActor {
            actor_id: message.removed_by.id.clone(),
            actor_kind: message.removed_by.kind.clone(),
            actor_session_id: message.removed_by.session_id.clone(),
        },
        occurred_at: message.removed_at.clone(),
        committed_at: message.removed_at.clone(),
        payload_schema: Some("message.reaction_removed.v1".into()),
        payload: serde_json::to_string(message)
            .expect("message reaction removed payload should serialize into commit envelope"),
        retention_class: retention_class.into(),
        audit_class: "default".into(),
    }
}

pub(super) fn build_message_pinned_envelope(
    message: &MessagePinned,
    organization_id: &str,
    event_id: &str,
    retention_class: &str,
) -> CommitEnvelope {
    CommitEnvelope {
        event_id: event_id.into(),
        tenant_id: message.tenant_id.clone(),
        organization_id: normalize_commit_organization_id(organization_id),
        event_type: "message.pin_added".into(),
        event_version: 1,
        aggregate_type: AggregateType::Conversation,
        aggregate_id: message.conversation_id.clone(),
        scope_type: "conversation".into(),
        scope_id: message.conversation_id.clone(),
        ordering_key: CommitEnvelope::ordering_key(
            message.tenant_id.as_str(),
            message.conversation_id.as_str(),
        ),
        ordering_seq: message.message_seq,
        causation_id: None,
        correlation_id: None,
        idempotency_key: None,
        actor: EventActor {
            actor_id: message.pinned_by.id.clone(),
            actor_kind: message.pinned_by.kind.clone(),
            actor_session_id: message.pinned_by.session_id.clone(),
        },
        occurred_at: message.pinned_at.clone(),
        committed_at: message.pinned_at.clone(),
        payload_schema: Some("message.pin_added.v1".into()),
        payload: serde_json::to_string(message)
            .expect("message pinned payload should serialize into commit envelope"),
        retention_class: retention_class.into(),
        audit_class: "default".into(),
    }
}

pub(super) fn build_message_unpinned_envelope(
    message: &MessageUnpinned,
    organization_id: &str,
    event_id: &str,
    retention_class: &str,
) -> CommitEnvelope {
    CommitEnvelope {
        event_id: event_id.into(),
        tenant_id: message.tenant_id.clone(),
        organization_id: normalize_commit_organization_id(organization_id),
        event_type: "message.pin_removed".into(),
        event_version: 1,
        aggregate_type: AggregateType::Conversation,
        aggregate_id: message.conversation_id.clone(),
        scope_type: "conversation".into(),
        scope_id: message.conversation_id.clone(),
        ordering_key: CommitEnvelope::ordering_key(
            message.tenant_id.as_str(),
            message.conversation_id.as_str(),
        ),
        ordering_seq: message.message_seq,
        causation_id: None,
        correlation_id: None,
        idempotency_key: None,
        actor: EventActor {
            actor_id: message.unpinned_by.id.clone(),
            actor_kind: message.unpinned_by.kind.clone(),
            actor_session_id: message.unpinned_by.session_id.clone(),
        },
        occurred_at: message.unpinned_at.clone(),
        committed_at: message.unpinned_at.clone(),
        payload_schema: Some("message.pin_removed.v1".into()),
        payload: serde_json::to_string(message)
            .expect("message unpinned payload should serialize into commit envelope"),
        retention_class: retention_class.into(),
        audit_class: "default".into(),
    }
}

pub(super) fn event_id_component(value: &str) -> String {
    value
        .chars()
        .map(|ch| if ch.is_ascii_alphanumeric() { ch } else { '_' })
        .collect()
}

pub(super) fn conversation_timestamp() -> String {
    utc_now_rfc3339_millis()
}
