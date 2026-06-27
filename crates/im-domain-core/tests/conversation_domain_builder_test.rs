use std::collections::BTreeMap;

use im_domain_core::conversation::{
    AgentHandoffStateView, ChangeAgentHandoffStatusView, ConversationAggregateState,
    ConversationBusinessBinding, ConversationHandoffLifecycle,
    ConversationHandoffTransitionOutcome, ConversationPolicy, ConversationRoster,
    ConversationScenario, MembershipRole, MembershipState, build_conversation_member,
    build_conversation_member_with_attributes, build_default_read_cursor, member_episode_id,
    member_id,
};
use im_domain_core::message::{
    ContentPart, ConversationMessageLog, DataPart, Message, MessageBody, MessageEdited,
    MessageLocatorIndex, MessageReactionAdded, MessageReactionRemoved, MessageRecalled,
    MessageType, SDKWORK_IM_MESSAGE_SCHEMA_CARD, SDKWORK_IM_MESSAGE_SCHEMA_LOCATION, Sender,
};

#[test]
fn test_conversation_member_builder_defaults_to_joined_state() {
    let member = build_conversation_member(
        "100001",
        "c_demo",
        member_id("c_demo", "user", "1"),
        "1",
        "user",
        MembershipRole::Owner,
        Some("u_inviter".into()),
        "2026-04-07T12:00:00.000Z".into(),
    );

    assert_eq!(member.tenant_id, "100001");
    assert_eq!(member.conversation_id, "c_demo");
    assert_eq!(member.member_id, "cm_c_demo_user_1");
    assert_eq!(member.principal_id, "1");
    assert_eq!(member.principal_kind, "user");
    assert_eq!(member.role, MembershipRole::Owner);
    assert_eq!(member.state, MembershipState::Joined);
    assert_eq!(member.invited_by.as_deref(), Some("u_inviter"));
    assert_eq!(member.joined_at, "2026-04-07T12:00:00.000Z");
    assert_eq!(member.removed_at, None);
    assert!(member.attributes.is_empty());
}

#[test]
fn test_conversation_member_builder_keeps_custom_attributes() {
    let member = build_conversation_member_with_attributes(
        "100001",
        "c_agent",
        member_episode_id("c_agent", "agent", "agent_demo", 2),
        "agent_demo",
        "agent",
        MembershipRole::Member,
        Some("1".into()),
        "2026-04-07T12:01:00.000Z".into(),
        BTreeMap::from([
            ("dialogRole".into(), "assistant".into()),
            ("agentId".into(), "agent_demo".into()),
        ]),
    );

    assert_eq!(member.member_id, "cm_c_agent_agent_agent_demo_e2");
    assert_eq!(
        member.attributes.get("dialogRole").map(String::as_str),
        Some("assistant")
    );
    assert_eq!(
        member.attributes.get("agentId").map(String::as_str),
        Some("agent_demo")
    );
}

#[test]
fn test_default_read_cursor_reuses_member_identity() {
    let member = build_conversation_member(
        "100001",
        "c_demo",
        member_id("c_demo", "user", "1"),
        "1",
        "user",
        MembershipRole::Member,
        None,
        "2026-04-07T12:02:00.000Z".into(),
    );

    let cursor = build_default_read_cursor(&member);

    assert_eq!(cursor.tenant_id, member.tenant_id);
    assert_eq!(cursor.conversation_id, member.conversation_id);
    assert_eq!(cursor.member_id, member.member_id);
    assert_eq!(cursor.principal_id, member.principal_id);
    assert_eq!(cursor.read_seq, 0);
    assert_eq!(cursor.last_read_message_id, None);
    assert_eq!(cursor.updated_at, member.joined_at);
}

#[test]
fn test_member_episode_id_adds_suffix_only_after_first_episode() {
    assert_eq!(
        member_id("c_demo", "user", "1"),
        "cm_c_demo_user_1"
    );
    assert_eq!(
        member_episode_id("c_demo", "user", "1", 1),
        "cm_c_demo_user_1"
    );
    assert_eq!(
        member_episode_id("c_demo", "user", "1", 3),
        "cm_c_demo_user_1_e3"
    );
}

#[test]
fn test_conversation_roster_tracks_active_member_and_default_cursor() {
    let mut roster = ConversationRoster::default();
    let member = build_conversation_member(
        "100001",
        "c_demo",
        member_id("c_demo", "user", "1"),
        "1",
        "user",
        MembershipRole::Member,
        None,
        "2026-04-07T12:03:00.000Z".into(),
    );

    roster.upsert_member(member.clone());
    roster.ensure_default_read_cursor(&member);

    assert_eq!(
        roster.resolve_active_member_id("1").as_deref(),
        Some(member.member_id.as_str())
    );
    assert_eq!(
        roster
            .read_cursor(member.member_id.as_str())
            .map(|cursor| cursor.updated_at.as_str()),
        Some("2026-04-07T12:03:00.000Z")
    );

    let removed_member = im_domain_core::conversation::ConversationMember {
        state: MembershipState::Removed,
        removed_at: Some("2026-04-07T12:04:00.000Z".into()),
        ..member.clone()
    };
    roster.deactivate_member(removed_member.clone());

    assert!(roster.resolve_active_member("1").is_none());
    assert_eq!(
        roster.member(member.member_id.as_str()),
        Some(&removed_member)
    );
}

#[test]
fn test_conversation_roster_next_member_episode_counts_existing_memberships() {
    let mut roster = ConversationRoster::default();
    let first_member = build_conversation_member(
        "100001",
        "c_demo",
        member_id("c_demo", "user", "1"),
        "1",
        "user",
        MembershipRole::Member,
        None,
        "2026-04-07T12:05:00.000Z".into(),
    );
    roster.upsert_member(first_member.clone());
    roster.deactivate_member(im_domain_core::conversation::ConversationMember {
        state: MembershipState::Left,
        removed_at: Some("2026-04-07T12:06:00.000Z".into()),
        ..first_member
    });

    assert_eq!(roster.next_member_episode("1", "user"), 2);
}

#[test]
fn test_conversation_roster_member_episode_isolated_by_principal_kind() {
    let mut roster = ConversationRoster::default();
    let user_member = build_conversation_member(
        "100001",
        "c_typed_episode",
        member_id("c_typed_episode", "user", "shared_id"),
        "shared_id",
        "user",
        MembershipRole::Member,
        None,
        "2026-04-07T12:05:00.000Z".into(),
    );
    let agent_member = build_conversation_member(
        "100001",
        "c_typed_episode",
        member_id("c_typed_episode", "agent", "shared_id"),
        "shared_id",
        "agent",
        MembershipRole::Member,
        None,
        "2026-04-07T12:05:01.000Z".into(),
    );

    roster.upsert_member(user_member);
    roster.upsert_member(agent_member);

    assert_eq!(
        roster.next_member_episode("shared_id", "user"),
        2,
        "user episode should only count existing user memberships"
    );
    assert_eq!(
        roster.next_member_episode("shared_id", "agent"),
        2,
        "agent episode should only count existing agent memberships"
    );
    assert_eq!(
        roster.next_member_episode("shared_id", "system"),
        1,
        "unseen principal kind should start at the first episode"
    );
}

#[test]
fn test_conversation_roster_keeps_invited_member_history_visible_but_not_active() {
    let mut roster = ConversationRoster::default();
    let mut invited_member = build_conversation_member(
        "100001",
        "c_demo",
        member_id("c_demo", "user", "u_invited"),
        "u_invited",
        "user",
        MembershipRole::Member,
        Some("1".into()),
        "2026-04-07T12:06:30.000Z".into(),
    );
    invited_member.state = MembershipState::Invited;

    roster.upsert_member(invited_member.clone());
    roster.ensure_default_read_cursor(&invited_member);

    assert_eq!(roster.active_principal_count(), 0);
    assert!(roster.resolve_active_member("u_invited").is_none());
    assert_eq!(
        roster.resolve_current_member("u_invited"),
        Some(invited_member.clone())
    );
    assert_eq!(
        roster.resolve_history_visible_member("u_invited"),
        Some(invited_member.clone())
    );
    assert_eq!(
        roster
            .read_cursor(invited_member.member_id.as_str())
            .map(|cursor| cursor.updated_at.as_str()),
        Some("2026-04-07T12:06:30.000Z")
    );
}

#[test]
fn test_conversation_message_log_owns_high_watermark_and_posted_messages() {
    let mut log = ConversationMessageLog::default();
    assert_eq!(log.high_watermark(), 0);

    let next_seq = log.next_message_seq();
    assert_eq!(next_seq, 1);

    let message = demo_message(1);
    log.store_posted(message.clone());

    let stored = log
        .message(message.message_id.as_str())
        .expect("stored message should exist");
    assert_eq!(log.high_watermark(), 1);
    assert_eq!(stored.message, message);
    assert!(!stored.recalled);
    assert_eq!(log.unread_count_since(0), 1);
}

#[test]
fn test_conversation_message_log_returns_bounded_sequence_windows_without_full_history_shape() {
    let mut log = ConversationMessageLog::default();
    log.store_posted(demo_message(2));
    log.store_posted(demo_message(1));
    log.store_posted(demo_message(3));

    let first = log.message_window_after(0, 1);
    assert_eq!(
        first
            .items
            .iter()
            .map(|stored| stored.message.message_seq)
            .collect::<Vec<_>>(),
        vec![1]
    );
    assert_eq!(first.next_after_seq, Some(1));
    assert!(first.has_more);
    assert_eq!(first.high_watermark, 3);

    let second = log.message_window_after(1, 2);
    assert_eq!(
        second
            .items
            .iter()
            .map(|stored| stored.message.message_seq)
            .collect::<Vec<_>>(),
        vec![2, 3]
    );
    assert_eq!(second.next_after_seq, Some(3));
    assert!(!second.has_more);
    assert_eq!(second.high_watermark, 3);
}

#[test]
fn test_conversation_message_log_replaces_stale_sequence_index_when_message_id_is_overwritten() {
    let mut log = ConversationMessageLog::default();
    let mut replacement = demo_message(2);
    replacement.message_id = "msg_c_demo_1".into();

    log.store_posted(demo_message(1));
    log.store_posted(replacement);

    assert_eq!(
        log.messages_in_order()
            .iter()
            .map(|stored| stored.message.message_seq)
            .collect::<Vec<_>>(),
        vec![2],
        "overwriting a message id must remove the stale sequence index"
    );
    assert!(log.message_window_after(0, 10).items[0].message.message_id == "msg_c_demo_1");
}

#[test]
fn test_conversation_message_log_applies_edit_and_recall_mutations() {
    let mut log = ConversationMessageLog::default();
    let message = demo_message(7);
    log.store_posted(message.clone());

    let edited = MessageEdited {
        tenant_id: message.tenant_id.clone(),
        conversation_id: message.conversation_id.clone(),
        message_id: message.message_id.clone(),
        message_seq: message.message_seq,
        body: MessageBody {
            summary: Some("edited".into()),
            parts: vec![ContentPart::text("edited body")],
            render_hints: BTreeMap::new(),
            reply_to: None,
        },
        editor: demo_sender(),
        edited_at: "2026-04-07T12:08:00.000Z".into(),
    };
    assert!(log.apply_edited(&edited).is_some());

    let edited_message = log
        .message(message.message_id.as_str())
        .expect("edited message should remain stored");
    assert_eq!(
        edited_message.message.body.summary.as_deref(),
        Some("edited")
    );
    assert_eq!(
        edited_message.message.committed_at.as_deref(),
        Some("2026-04-07T12:08:00.000Z")
    );

    let recalled = MessageRecalled {
        tenant_id: message.tenant_id.clone(),
        conversation_id: message.conversation_id.clone(),
        message_id: message.message_id.clone(),
        message_seq: message.message_seq,
        recalled_by: demo_sender(),
        recalled_at: "2026-04-07T12:09:00.000Z".into(),
    };
    assert!(log.apply_recalled(&recalled).is_some());

    let recalled_message = log
        .message(message.message_id.as_str())
        .expect("recalled message should remain stored");
    assert!(recalled_message.recalled);
    assert_eq!(
        recalled_message.message.body.summary.as_deref(),
        Some("[recalled]")
    );
    assert_eq!(
        recalled_message.message.committed_at.as_deref(),
        Some("2026-04-07T12:09:00.000Z")
    );
    assert_eq!(log.high_watermark(), 7);
    assert_eq!(log.unread_count_since(3), 4);
}

#[test]
fn test_conversation_message_log_reactions_are_isolated_by_actor_kind() {
    let mut log = ConversationMessageLog::default();
    let message = demo_message(1);
    log.store_posted(message.clone());

    let user_added = MessageReactionAdded {
        tenant_id: message.tenant_id.clone(),
        conversation_id: message.conversation_id.clone(),
        message_id: message.message_id.clone(),
        message_seq: message.message_seq,
        reaction_key: "thumbs_up".into(),
        reacted_by: Sender {
            id: "shared_actor".into(),
            kind: "user".into(),
            member_id: Some("cm_c_demo_user_shared_actor".into()),
            device_id: None,
            session_id: None,
            metadata: BTreeMap::new(),
        },
        reacted_at: "2026-04-07T12:09:01.000Z".into(),
    };
    let agent_added = MessageReactionAdded {
        reacted_by: Sender {
            kind: "agent".into(),
            member_id: Some("cm_c_demo_agent_shared_actor".into()),
            ..user_added.reacted_by.clone()
        },
        reacted_at: "2026-04-07T12:09:02.000Z".into(),
        ..user_added.clone()
    };

    assert_eq!(log.apply_reaction_added(&user_added), Some(true));
    assert_eq!(
        log.apply_reaction_added(&agent_added),
        Some(true),
        "same actor id with a different actor kind must be a distinct reaction identity"
    );

    let stored = log
        .message(message.message_id.as_str())
        .expect("stored message should exist");
    assert_eq!(stored.reactions["thumbs_up"].len(), 2);

    let user_removed = MessageReactionRemoved {
        tenant_id: message.tenant_id.clone(),
        conversation_id: message.conversation_id.clone(),
        message_id: message.message_id.clone(),
        message_seq: message.message_seq,
        reaction_key: "thumbs_up".into(),
        removed_by: user_added.reacted_by,
        removed_at: "2026-04-07T12:09:03.000Z".into(),
    };
    assert_eq!(log.apply_reaction_removed(&user_removed), Some(true));

    let stored = log
        .message(message.message_id.as_str())
        .expect("stored message should exist");
    assert_eq!(
        stored.reactions["thumbs_up"].len(),
        1,
        "removing the user reaction must not remove the same-id agent reaction"
    );
}

#[test]
fn test_conversation_message_log_derives_summary_for_missing_rich_message_summaries() {
    let mut log = ConversationMessageLog::default();
    let mut message = demo_message(8);
    message.body = MessageBody {
        summary: None,
        parts: vec![ContentPart::Data(DataPart {
            schema_ref: SDKWORK_IM_MESSAGE_SCHEMA_LOCATION.into(),
            encoding: "application/json".into(),
            payload: serde_json::json!({
                "name": "The Bund",
                "latitude": 31.2400,
                "longitude": 121.4900
            })
            .to_string(),
        })],
        render_hints: BTreeMap::new(),
        reply_to: None,
    };
    log.store_posted(message.clone());

    let stored = log
        .message(message.message_id.as_str())
        .expect("stored message should exist after derived summary");
    assert_eq!(
        stored.message.body.summary.as_deref(),
        Some("Location: The Bund")
    );

    let edited = MessageEdited {
        tenant_id: message.tenant_id.clone(),
        conversation_id: message.conversation_id.clone(),
        message_id: message.message_id.clone(),
        message_seq: message.message_seq,
        body: MessageBody {
            summary: None,
            parts: vec![ContentPart::Data(DataPart {
                schema_ref: SDKWORK_IM_MESSAGE_SCHEMA_CARD.into(),
                encoding: "application/json".into(),
                payload: serde_json::json!({
                    "title": "Escalation runbook"
                })
                .to_string(),
            })],
            render_hints: BTreeMap::new(),
            reply_to: None,
        },
        editor: demo_sender(),
        edited_at: "2026-04-07T12:08:30.000Z".into(),
    };
    log.apply_edited(&edited)
        .expect("edited message should stay stored");

    let stored_after_edit = log
        .message(message.message_id.as_str())
        .expect("stored message should exist after edit");
    assert_eq!(
        stored_after_edit.message.body.summary.as_deref(),
        Some("Card: Escalation runbook")
    );
}

#[test]
fn test_message_locator_index_resolves_message_to_conversation() {
    let mut locator = MessageLocatorIndex::default();
    let message = demo_message(9);

    locator.register_message(&message);

    assert_eq!(
        locator.conversation_id("100001", message.message_id.as_str()),
        Some("c_demo")
    );
    assert_eq!(locator.conversation_id("100001", "msg_missing"), None);
}

#[test]
fn test_conversation_policy_normalize_accepts_invited_history_visibility() {
    let normalized = ConversationPolicy {
        history_visibility: "invited".into(),
        ..ConversationPolicy::default()
    }
    .normalize()
    .expect("invited visibility should normalize");

    assert_eq!(normalized.history_visibility, "invited");
}

#[test]
fn test_conversation_policy_normalize_accepts_shared_history_visibility() {
    let normalized = ConversationPolicy {
        history_visibility: "shared".into(),
        ..ConversationPolicy::default()
    }
    .normalize()
    .expect("shared visibility should normalize");

    assert_eq!(normalized.history_visibility, "shared");
}

#[test]
fn test_history_read_allowed_matches_policy_modes() {
    use im_domain_core::conversation::{
        MembershipRole, MembershipState, build_conversation_member_with_attributes,
        history_read_allowed,
    };

    let active = build_conversation_member_with_attributes(
        "100001",
        "c_demo",
        "cm_active".into(),
        "u_active",
        "user",
        MembershipRole::Member,
        None,
        "2026-01-01T00:00:00.000Z".into(),
        Default::default(),
    );
    let invited = build_conversation_member_with_attributes(
        "100001",
        "c_demo",
        "cm_invited".into(),
        "u_invited",
        "user",
        MembershipRole::Guest,
        Some("1".into()),
        "2026-01-01T00:00:00.000Z".into(),
        Default::default(),
    );
    let mut invited_member = invited;
    invited_member.state = MembershipState::Invited;

    assert!(history_read_allowed("world_readable", None));
    assert!(history_read_allowed("joined", Some(&active)));
    assert!(!history_read_allowed("joined", Some(&invited_member)));
    assert!(history_read_allowed("invited", Some(&invited_member)));
}

fn demo_sender() -> Sender {
    Sender {
        id: "1".into(),
        kind: "user".into(),
        member_id: Some("cm_c_demo_user_1".into()),
        device_id: Some("device_demo".into()),
        session_id: Some("session_demo".into()),
        metadata: BTreeMap::new(),
    }
}

fn demo_message(message_seq: u64) -> Message {
    Message {
        tenant_id: "100001".into(),
        conversation_id: "c_demo".into(),
        message_id: format!("msg_c_demo_{message_seq}"),
        message_seq,
        sender: demo_sender(),
        message_type: MessageType::Standard,
        delivery_mode: "discrete".into(),
        client_msg_id: Some(format!("client_{message_seq}")),
        stream_session_id: None,
        rtc_session_id: None,
        body: MessageBody {
            summary: Some("hello".into()),
            parts: vec![ContentPart::text("hello world")],
            render_hints: BTreeMap::new(),
            reply_to: None,
        },
        attributes: BTreeMap::new(),
        metadata: BTreeMap::new(),
        occurred_at: "2026-04-07T12:07:00.000Z".into(),
        committed_at: Some("2026-04-07T12:07:00.000Z".into()),
    }
}

#[test]
fn test_conversation_aggregate_state_owns_type_epochs_and_handoff_status() {
    let source = ChangeAgentHandoffStatusView {
        id: "agent_source".into(),
        kind: "agent".into(),
    };
    let target = ChangeAgentHandoffStatusView {
        id: "user_target".into(),
        kind: "user".into(),
    };
    let handoff = AgentHandoffStateView {
        tenant_id: "100001".into(),
        conversation_id: "c_demo".into(),
        status: "open".into(),
        source: source.clone(),
        target: target.clone(),
        handoff_session_id: "hs_demo".into(),
        handoff_reason: Some("escalation".into()),
        accepted_at: None,
        accepted_by: None,
        resolved_at: None,
        resolved_by: None,
        closed_at: None,
        closed_by: None,
    };

    let mut aggregate = ConversationAggregateState::new_agent_handoff(handoff);
    assert_eq!(aggregate.conversation_type(), "agent_handoff");
    assert_eq!(aggregate.member_epoch(), 0);
    assert_eq!(aggregate.next_member_epoch(), 1);

    aggregate.observe_member_epoch(5);
    assert_eq!(aggregate.member_epoch(), 5);
    assert_eq!(aggregate.next_member_epoch(), 6);

    let accept = aggregate
        .transition_handoff_status(
            ConversationHandoffLifecycle::Accept,
            &target,
            "2026-04-07T12:15:00.000Z".into(),
        )
        .expect("handoff accept should succeed");
    assert_eq!(accept.previous_status, "open");
    assert_eq!(accept.ordering_seq, 1);
    assert_eq!(
        accept.outcome,
        ConversationHandoffTransitionOutcome::Mutated
    );
    assert_eq!(accept.state.status, "accepted");
    assert_eq!(aggregate.handoff_status_epoch(), 1);
    assert_eq!(
        aggregate.handoff_state().map(|state| state.status.as_str()),
        Some("accepted")
    );

    let accept_idempotent = aggregate
        .transition_handoff_status(
            ConversationHandoffLifecycle::Accept,
            &target,
            "2026-04-07T12:15:01.000Z".into(),
        )
        .expect("handoff accept should be idempotent");
    assert_eq!(
        accept_idempotent.outcome,
        ConversationHandoffTransitionOutcome::Idempotent
    );
    assert_eq!(accept_idempotent.ordering_seq, 1);
    assert_eq!(aggregate.handoff_status_epoch(), 1);

    let close = aggregate
        .transition_handoff_status(
            ConversationHandoffLifecycle::Close,
            &source,
            "2026-04-07T12:16:00.000Z".into(),
        )
        .expect("handoff close should succeed");
    assert_eq!(close.previous_status, "accepted");
    assert_eq!(close.ordering_seq, 2);
    assert!(aggregate.has_closed_handoff());
}

#[test]
fn test_conversation_aggregate_state_tracks_recovered_epoch_floor() {
    let mut aggregate = ConversationAggregateState::new("group");
    assert_eq!(aggregate.conversation_type(), "group");
    assert_eq!(aggregate.member_epoch(), 0);
    assert_eq!(aggregate.handoff_status_epoch(), 0);
    assert!(aggregate.handoff_state().is_none());
    assert!(!aggregate.has_closed_handoff());

    aggregate.observe_member_epoch(4);
    aggregate.observe_member_epoch(2);
    assert_eq!(aggregate.member_epoch(), 4);

    let source = ChangeAgentHandoffStatusView {
        id: "agent_source".into(),
        kind: "agent".into(),
    };
    let target = ChangeAgentHandoffStatusView {
        id: "user_target".into(),
        kind: "user".into(),
    };
    aggregate.observe_handoff_status_epoch(8);
    aggregate.replace_handoff_state(Some(AgentHandoffStateView {
        tenant_id: "100001".into(),
        conversation_id: "c_demo".into(),
        status: "closed".into(),
        source,
        target,
        handoff_session_id: "hs_demo".into(),
        handoff_reason: None,
        accepted_at: None,
        accepted_by: None,
        resolved_at: None,
        resolved_by: None,
        closed_at: Some("2026-04-07T12:17:00.000Z".into()),
        closed_by: None,
    }));
    aggregate.observe_handoff_status_epoch(3);

    assert_eq!(aggregate.handoff_status_epoch(), 8);
    assert!(aggregate.has_closed_handoff());
}

#[test]
fn test_conversation_aggregate_state_tracks_business_binding() {
    let mut aggregate = ConversationAggregateState::new("direct");
    let binding = ConversationBusinessBinding {
        business_type: "direct_chat".into(),
        business_id: "dc_demo".into(),
    };

    assert!(aggregate.business_binding().is_none());

    aggregate.replace_business_binding(Some(binding.clone()));

    assert_eq!(aggregate.business_binding(), Some(&binding));
}

#[test]
fn test_conversation_aggregate_state_projects_direct_group_and_channel_scenarios() {
    assert_eq!(
        ConversationAggregateState::new("group").scenario(),
        ConversationScenario::Group
    );
    assert_eq!(
        ConversationAggregateState::new("direct").scenario(),
        ConversationScenario::Direct
    );
    assert_eq!(
        ConversationAggregateState::new("system_channel").scenario(),
        ConversationScenario::SystemChannel
    );
    assert_eq!(
        ConversationAggregateState::new("agent_dialog").scenario(),
        ConversationScenario::AgentDialog
    );
    assert_eq!(
        ConversationAggregateState::new("thread").scenario(),
        ConversationScenario::Thread
    );
    assert_eq!(
        ConversationAggregateState::new("unknown").scenario(),
        ConversationScenario::Unknown
    );
}

#[test]
fn test_conversation_handoff_view_runs_domain_state_machine() {
    let source = ChangeAgentHandoffStatusView {
        id: "agent_source".into(),
        kind: "agent".into(),
    };
    let target = ChangeAgentHandoffStatusView {
        id: "user_target".into(),
        kind: "user".into(),
    };
    let mut handoff = AgentHandoffStateView {
        tenant_id: "100001".into(),
        conversation_id: "c_demo".into(),
        status: "open".into(),
        source: source.clone(),
        target: target.clone(),
        handoff_session_id: "hs_demo".into(),
        handoff_reason: Some("escalation".into()),
        accepted_at: None,
        accepted_by: None,
        resolved_at: None,
        resolved_by: None,
        closed_at: None,
        closed_by: None,
    };

    let accept = handoff.accept(&target, "2026-04-07T12:10:00.000Z".into());
    assert_eq!(accept, Ok(ConversationHandoffTransitionOutcome::Mutated));
    assert_eq!(handoff.status, "accepted");
    assert_eq!(
        handoff.accepted_by.as_ref().map(|actor| actor.id.as_str()),
        Some("user_target")
    );

    let accept_idempotent = handoff.accept(&target, "2026-04-07T12:10:01.000Z".into());
    assert_eq!(
        accept_idempotent,
        Ok(ConversationHandoffTransitionOutcome::Idempotent)
    );

    let resolve = handoff.resolve(&target, "2026-04-07T12:11:00.000Z".into());
    assert_eq!(resolve, Ok(ConversationHandoffTransitionOutcome::Mutated));
    assert_eq!(handoff.status, "resolved");

    let close = handoff.close(&source, "2026-04-07T12:12:00.000Z".into());
    assert_eq!(close, Ok(ConversationHandoffTransitionOutcome::Mutated));
    assert_eq!(handoff.status, "closed");
    assert!(handoff.is_closed());
}

#[test]
fn test_conversation_handoff_view_rejects_invalid_domain_transition() {
    let source = ChangeAgentHandoffStatusView {
        id: "agent_source".into(),
        kind: "agent".into(),
    };
    let target = ChangeAgentHandoffStatusView {
        id: "user_target".into(),
        kind: "user".into(),
    };
    let other = ChangeAgentHandoffStatusView {
        id: "user_other".into(),
        kind: "user".into(),
    };
    let mut handoff = AgentHandoffStateView {
        tenant_id: "100001".into(),
        conversation_id: "c_demo".into(),
        status: "open".into(),
        source: source.clone(),
        target,
        handoff_session_id: "hs_demo".into(),
        handoff_reason: None,
        accepted_at: None,
        accepted_by: None,
        resolved_at: None,
        resolved_by: None,
        closed_at: None,
        closed_by: None,
    };

    let invalid_accept = handoff.accept(&other, "2026-04-07T12:13:00.000Z".into());
    assert!(invalid_accept.is_err());
    assert_eq!(handoff.status, "open");

    let invalid_resolve = handoff.resolve(&source, "2026-04-07T12:14:00.000Z".into());
    assert!(invalid_resolve.is_err());
    assert_eq!(handoff.status, "open");
}
