use im_domain_events::{AggregateType, CommitEnvelope};

use im_platform_contracts::normalize_realtime_organization_id;

use im_time::utc_now_rfc3339_millis;

use crate::conversation_state::event_apply::ConversationStateError;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]

pub(super) struct ClientRoutePrincipalScopeKey {
    pub(super) tenant_id: String,

    pub(super) organization_id: String,

    pub(super) principal_kind: String,

    pub(super) principal_id: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]

pub(super) struct ClientRouteFeedScopeKey {
    pub(super) tenant_id: String,

    pub(super) organization_id: String,

    pub(super) principal_kind: String,

    pub(super) principal_id: String,

    pub(super) device_id: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]

pub(super) struct ContactOwnerScopeKey {
    pub(super) tenant_id: String,

    pub(super) organization_id: String,

    pub(super) owner_user_id: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(super) struct GroupScopeKey {
    pub(super) tenant_id: String,

    pub(super) organization_id: String,

    pub(super) group_id: String,
}

pub(super) fn conversation_state_organization_id_for_event(event: &CommitEnvelope) -> String {
    im_domain_events::normalize_commit_organization_id(event.organization_id.as_str())
}

pub(super) fn scope_key(tenant_id: &str, organization_id: &str, conversation_id: &str) -> String {
    encode_conversation_state_key_segments([
        tenant_id,
        normalize_realtime_organization_id(organization_id).as_str(),
        conversation_id,
    ])
}

pub(super) fn message_lookup_scope_key(
    tenant_id: &str,
    organization_id: &str,
    message_id: &str,
) -> String {
    encode_conversation_state_key_segments([
        tenant_id,
        normalize_realtime_organization_id(organization_id).as_str(),
        "msg",
        message_id,
    ])
}

pub(super) fn scope_key_for_event(event: &CommitEnvelope) -> String {
    scope_key(
        event.tenant_id.as_str(),
        conversation_state_organization_id_for_event(event).as_str(),
        event.scope_id.as_str(),
    )
}

pub(super) fn scope_key_for_event_conversation(
    event: &CommitEnvelope,

    conversation_id: &str,
) -> String {
    scope_key(
        event.tenant_id.as_str(),
        conversation_state_organization_id_for_event(event).as_str(),
        conversation_id,
    )
}

pub(super) fn client_route_principal_scope_key(
    tenant_id: &str,

    organization_id: &str,

    principal_kind: &str,

    principal_id: &str,
) -> ClientRoutePrincipalScopeKey {
    ClientRoutePrincipalScopeKey {
        tenant_id: tenant_id.into(),

        organization_id: normalize_realtime_organization_id(organization_id),

        principal_kind: principal_kind.into(),

        principal_id: principal_id.into(),
    }
}

pub(super) fn client_route_feed_scope_key(
    tenant_id: &str,

    organization_id: &str,

    principal_kind: &str,

    principal_id: &str,

    device_id: &str,
) -> ClientRouteFeedScopeKey {
    ClientRouteFeedScopeKey {
        tenant_id: tenant_id.into(),

        organization_id: normalize_realtime_organization_id(organization_id),

        principal_kind: principal_kind.into(),

        principal_id: principal_id.into(),

        device_id: device_id.into(),
    }
}

pub(super) fn contact_owner_scope_key(
    tenant_id: &str,

    organization_id: &str,

    owner_user_id: &str,
) -> ContactOwnerScopeKey {
    ContactOwnerScopeKey {
        tenant_id: tenant_id.into(),

        organization_id: normalize_realtime_organization_id(organization_id),

        owner_user_id: owner_user_id.into(),
    }
}

pub(super) fn group_scope_key(
    tenant_id: &str,

    organization_id: &str,

    group_id: &str,
) -> GroupScopeKey {
    GroupScopeKey {
        tenant_id: tenant_id.into(),

        organization_id: normalize_realtime_organization_id(organization_id),

        group_id: group_id.into(),
    }
}

pub(super) fn registered_client_route_at() -> String {
    utc_now_rfc3339_millis()
}

pub(super) fn is_conversation_conversation_state_event_type(event_type: &str) -> bool {
    matches!(
        event_type,
        "conversation.created"
            | "conversation.agents_replaced"
            | "conversation.policy_applied"
            | "conversation.agent_handoff_status_changed"
            | "conversation.member_joined"
            | "conversation.member_invitation_accepted"
            | "conversation.member_role_changed"
            | "conversation.member_removed"
            | "conversation.member_left"
            | "conversation.read_cursor_updated"
            | "message.posted"
            | "message.edited"
            | "message.recalled"
            | "message.reaction_added"
            | "message.reaction_removed"
            | "message.pin_added"
            | "message.pin_removed"
    )
}

pub(super) fn validate_conversation_conversation_state_envelope(
    event: &CommitEnvelope,
) -> Result<(), ConversationStateError> {
    let tenant_id = event.tenant_id.trim();
    let aggregate_id = event.aggregate_id.trim();
    if tenant_id.is_empty()
        || aggregate_id.is_empty()
        || event.tenant_id != tenant_id
        || event.aggregate_id != aggregate_id
        || event.aggregate_type != AggregateType::Conversation
        || event.scope_type != "conversation"
        || event.scope_id != event.aggregate_id
    {
        return Err(ConversationStateError::InvalidEvent(format!(
            "{} requires canonical conversation envelope scope",
            event.event_type
        )));
    }
    Ok(())
}

pub(super) fn validate_conversation_conversation_state_payload_scope(
    event: &CommitEnvelope,
    payload_tenant_id: &str,
    payload_conversation_id: &str,
) -> Result<(), ConversationStateError> {
    if payload_tenant_id != event.tenant_id || payload_conversation_id != event.aggregate_id {
        return Err(ConversationStateError::InvalidEvent(format!(
            "{} payload scope tenant={} conversation={} does not match envelope tenant={} conversation={}",
            event.event_type,
            payload_tenant_id,
            payload_conversation_id,
            event.tenant_id,
            event.aggregate_id
        )));
    }
    Ok(())
}

pub(super) fn encode_conversation_state_key_segments<'a>(
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

pub(crate) fn decode_conversation_state_key_segments(encoded: &str) -> Option<Vec<String>> {
    let mut segments = Vec::new();
    let mut rest = encoded;
    while !rest.is_empty() {
        let hash = rest.find('#')?;
        let len: usize = rest[..hash].parse().ok()?;
        rest = &rest[hash + 1..];
        if rest.len() < len {
            return None;
        }
        segments.push(rest[..len].to_string());
        rest = &rest[len..];
    }
    Some(segments)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]

    fn test_client_route_scope_keys_isolate_organizations() {
        assert_ne!(
            client_route_feed_scope_key("100001", "org_a", "user", "1", "d_pad"),
            client_route_feed_scope_key("100001", "org_b", "user", "1", "d_pad"),
            "client route feed scope keys must isolate organizations"
        );

        assert_eq!(
            client_route_feed_scope_key("100001", "", "user", "1", "d_pad"),
            client_route_feed_scope_key("100001", "default", "user", "1", "d_pad"),
            "empty organization_id must normalize to default"
        );

        assert!(
            client_route_feed_scope_key("100001", "org_a", "user", "1", "d_pad").principal_kind
                == "user",
            "principal_kind must precede principal_id in scope key shape"
        );
    }

    #[test]

    fn test_conversation_scope_keys_isolate_organizations() {
        assert_ne!(
            scope_key("100001", "org_a", "c_shared"),
            scope_key("100001", "org_b", "c_shared"),
            "conversation conversation_state scope keys must isolate organizations"
        );

        assert_eq!(
            scope_key("100001", "", "c_shared"),
            scope_key("100001", "default", "c_shared"),
            "empty organization_id must normalize to default in conversation scope keys"
        );
    }

    #[test]

    fn test_contact_owner_scope_keys_isolate_organizations() {
        assert_ne!(
            contact_owner_scope_key("100001", "org_a", "1"),
            contact_owner_scope_key("100001", "org_b", "1"),
            "contact owner scope keys must isolate organizations"
        );

        assert_eq!(
            contact_owner_scope_key("100001", "", "1"),
            contact_owner_scope_key("100001", "default", "1"),
            "empty organization_id must normalize to default in contact owner scope keys"
        );
    }
}
