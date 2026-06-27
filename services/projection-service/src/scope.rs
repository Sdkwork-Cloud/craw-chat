use im_domain_events::CommitEnvelope;

use im_platform_contracts::normalize_realtime_organization_id;

use im_time::utc_now_rfc3339_millis;



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



pub(super) fn projection_organization_id_for_event(event: &CommitEnvelope) -> String {

    im_domain_events::normalize_commit_organization_id(event.organization_id.as_str())

}



pub(super) fn scope_key(

    tenant_id: &str,

    organization_id: &str,

    conversation_id: &str,

) -> String {

    encode_projection_key_segments([

        tenant_id,

        normalize_realtime_organization_id(organization_id).as_str(),

        conversation_id,

    ])

}



pub(super) fn scope_key_for_event(event: &CommitEnvelope) -> String {

    scope_key(

        event.tenant_id.as_str(),

        projection_organization_id_for_event(event).as_str(),

        event.scope_id.as_str(),

    )

}



pub(super) fn scope_key_for_event_conversation(

    event: &CommitEnvelope,

    conversation_id: &str,

) -> String {

    scope_key(

        event.tenant_id.as_str(),

        projection_organization_id_for_event(event).as_str(),

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



pub(super) fn registered_client_route_at() -> String {

    utc_now_rfc3339_millis()

}



pub(super) fn tracked_live_projection_lag_scope_id(event: &CommitEnvelope) -> Option<String> {

    if event.scope_type != "conversation" {

        return None;

    }



    if matches!(

        event.event_type.as_str(),

        "conversation.created"

            | "conversation.policy_applied"

            | "conversation.agent_handoff_status_changed"

            | "message.posted"

            | "message.edited"

            | "message.recalled"

            | "message.reaction_added"

            | "message.reaction_removed"

            | "message.pin_added"

            | "message.pin_removed"

            | "conversation.member_joined"

            | "conversation.member_role_changed"

            | "conversation.member_removed"

            | "conversation.member_left"

            | "conversation.read_cursor_updated"

    ) {

        Some(scope_key_for_event(event))

    } else {

        None

    }

}



pub(super) fn encode_projection_key_segments<'a>(
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

pub(crate) fn decode_projection_key_segments(encoded: &str) -> Option<Vec<String>> {
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

            client_route_feed_scope_key("100001", "org_a", "user", "1", "d_pad")

                .principal_kind

                == "user",

            "principal_kind must precede principal_id in scope key shape"

        );

    }



    #[test]

    fn test_conversation_scope_keys_isolate_organizations() {

        assert_ne!(

            scope_key("100001", "org_a", "c_shared"),

            scope_key("100001", "org_b", "c_shared"),

            "conversation projection scope keys must isolate organizations"

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


