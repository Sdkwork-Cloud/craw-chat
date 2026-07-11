use super::*;

const SOAK_ITERATIONS: usize = 96;
const SOAK_MAX_CONVERSATIONS: usize = 8;
const SOAK_BYTE_BUDGET: usize = 768 * 1024;

fn soak_sender(id: String) -> Sender {
    Sender {
        id,
        kind: "user".into(),
        member_id: None,
        device_id: None,
        session_id: None,
        metadata: BTreeMap::new(),
    }
}

fn soak_body(size_bytes: usize) -> MessageBody {
    MessageBody {
        summary: None,
        parts: vec![ContentPart::text("x".repeat(size_bytes))],
        render_hints: BTreeMap::new(),
        reply_to: None,
    }
}

fn build_soak_conversation(index: usize) -> (ConversationState, ConversationMember) {
    let conversation_id = format!("c_soak_{index}");
    let message_id = format!("m_soak_{index}");
    let binding = ConversationBusinessBinding {
        business_type: "thread".into(),
        business_id: format!("root_soak_{index}"),
    };
    let member = ConversationMember {
        tenant_id: "100001".into(),
        conversation_id: conversation_id.clone(),
        member_id: format!("member_soak_{index}"),
        principal_id: "shared_soak_actor".into(),
        principal_kind: "user".into(),
        role: MembershipRole::Member,
        state: MembershipState::Joined,
        invited_by: None,
        joined_at: "2026-07-10T00:00:00.000Z".into(),
        removed_at: None,
        attributes: BTreeMap::new(),
    };
    let sender = soak_sender(format!("sender_{index}"));
    let mut conversation = ConversationState {
        last_accessed_at_ms: (index + 1) as u64,
        ..Default::default()
    };
    conversation
        .aggregate
        .replace_business_binding(Some(binding));
    conversation.roster.upsert_member(member.clone());
    conversation.message_log.store_posted(Message {
        tenant_id: "100001".into(),
        conversation_id: conversation_id.clone(),
        message_id: message_id.clone(),
        message_seq: 1,
        sender: sender.clone(),
        message_type: MessageType::Standard,
        delivery_mode: "discrete".into(),
        client_msg_id: Some(format!("client_soak_{index}")),
        stream_session_id: None,
        rtc_session_id: None,
        body: soak_body(64 * 1024),
        attributes: BTreeMap::new(),
        metadata: BTreeMap::new(),
        occurred_at: "2026-07-10T00:00:00.000Z".into(),
        committed_at: Some("2026-07-10T00:00:00.000Z".into()),
    });

    let edited = MessageEdited {
        tenant_id: "100001".into(),
        conversation_id: conversation_id.clone(),
        message_id: message_id.clone(),
        message_seq: 1,
        body: soak_body(96 * 1024),
        editor: sender,
        edited_at: "2026-07-10T00:00:01.000Z".into(),
    };
    let edit_outcome = conversation
        .message_log
        .apply_edited(&edited)
        .expect("soak message should remain available for edit");
    assert!(edit_outcome.evicted_message_ids.is_empty());

    for reaction_index in 0..8 {
        let reaction = MessageReactionAdded {
            tenant_id: "100001".into(),
            conversation_id: conversation_id.clone(),
            message_id: message_id.clone(),
            message_seq: 1,
            reaction_key: format!("reaction_{reaction_index}"),
            reacted_by: soak_sender(format!("reaction_actor_{reaction_index}")),
            reacted_at: "2026-07-10T00:00:02.000Z".into(),
        };
        let reaction_outcome = conversation
            .message_log
            .apply_reaction_added(&reaction)
            .expect("soak message should remain available for reactions");
        assert!(reaction_outcome.evicted_message_ids.is_empty());
    }

    for replay_index in 0..8 {
        conversation.posted_message_requests.insert(
            format!("post_soak_{index}_{replay_index}"),
            PostedMessageReplayRecord {
                sender_id: "shared_soak_actor".into(),
                sender_kind: "user".into(),
                message_type: MessageType::Standard,
                body: soak_body(8 * 1024),
                message_id: message_id.clone(),
            },
        );
        conversation.message_mutation_requests.insert(
            format!("mutation_soak_{index}_{replay_index}"),
            MessageMutationReplayRecord {
                result: MessageMutationResult {
                    conversation_id: conversation_id.clone(),
                    message_id: message_id.clone(),
                    message_seq: 1,
                    event_id: format!("evt_soak_{index}_{replay_index}"),
                },
            },
        );
    }

    (conversation, member)
}

#[test]
fn bounded_runtime_pressure_soak_keeps_companion_state_within_budget() {
    let runtime = ConversationRuntime::new(InMemoryJournal::default());

    for index in 0..SOAK_ITERATIONS {
        let conversation_id = format!("c_soak_{index}");
        let message_id = format!("m_soak_{index}");
        let binding_scope =
            conversation_business_scope_key("100001", "thread", format!("root_soak_{index}").as_str());
        let scope = conversation_scope_key("100001", "0", conversation_id.as_str());
        let (conversation, member) = build_soak_conversation(index);
        {
            let mut state =
                write_runtime_state(&runtime.state, "bounded runtime pressure soak insert");
            state.insert_conversation(scope, conversation);
            state
                .business_index
                .insert(binding_scope, conversation_id.clone());
            state.sync_actor_inbox_member("0", &member);
            state
                .message_locator
                .register("100001", message_id.as_str(), conversation_id.as_str());
        }

        runtime.evict_idle_conversations_with_limits(
            SOAK_MAX_CONVERSATIONS,
            SOAK_BYTE_BUDGET,
        );
        let snapshot = runtime.runtime_metrics_snapshot();

        assert!(
            snapshot.estimated_conversation_bytes <= SOAK_BYTE_BUDGET,
            "iteration {index} exceeded the runtime byte budget"
        );
        assert!(snapshot.conversation_entries <= SOAK_MAX_CONVERSATIONS);
        assert_eq!(
            snapshot.message_locator_entries,
            snapshot.conversation_entries
        );
        assert_eq!(
            snapshot.business_binding_entries,
            snapshot.conversation_entries
        );
        assert!(snapshot.actor_inbox_actor_entries <= 1);
        assert_eq!(
            snapshot.actor_inbox_conversation_entries,
            snapshot.conversation_entries
        );
        assert!(snapshot.message_cache_entries <= snapshot.conversation_entries);
        assert!(snapshot.replay_cache_entries <= snapshot.conversation_entries * 16);
        assert!(snapshot.replay_cache_bytes <= snapshot.estimated_conversation_bytes);
    }

    let final_snapshot = runtime.runtime_metrics_snapshot();
    assert!(final_snapshot.conversation_evictions_bytes_total > 0);
    assert!(final_snapshot.conversation_evicted_bytes_total > 0);
    assert!(final_snapshot.eviction_operations_total > 0);
}
