use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::Duration;

use conversation_runtime::{
    AcceptAgentHandoffCommand, AddConversationMemberCommand, AddMessageReactionCommand,
    ApplyConversationPolicyCommand, BindDirectChatConversationCommand,
    ChangeAgentHandoffStatusView, ChangeConversationMemberRoleCommand, CloseAgentHandoffCommand,
    ConversationBusinessBinding, ConversationRuntime, CreateAgentDialogCommand,
    CreateAgentHandoffCommand, CreateConversationCommand, CreateSystemChannelCommand,
    CreateThreadConversationCommand, EditMessageCommand, LeaveConversationCommand,
    PinMessageCommand, PostMessageCommand, PublishSystemChannelMessageCommand,
    RecallMessageCommand, RemoveConversationMemberCommand, RemoveMessageReactionCommand,
    ResolveAgentHandoffCommand, RuntimeError, SyncSharedChannelLinkedMemberCommand,
    TransferConversationOwnerCommand, UnpinMessageCommand, UpdateReadCursorCommand,
};
use im_domain_core::conversation::{
    ConversationMember, ConversationPolicy, MembershipRole, MembershipState,
};
use im_domain_core::message::{ContentPart, MessageBody, MessageType, Sender};
use im_domain_events::{AggregateType, CommitEnvelope, EventActor};
use im_platform_contracts::{CommitJournal, CommitPosition, ContractError};

#[derive(Clone, Default)]
struct InMemoryJournal {
    events: Arc<Mutex<Vec<CommitEnvelope>>>,
}

impl InMemoryJournal {
    fn recorded(&self) -> Vec<CommitEnvelope> {
        self.events.lock().expect("journal should lock").clone()
    }
}

impl CommitJournal for InMemoryJournal {
    fn append(&self, envelope: CommitEnvelope) -> Result<CommitPosition, ContractError> {
        let mut events = self.events.lock().expect("journal should lock");
        events.push(envelope);
        Ok(CommitPosition::new("p0", events.len() as u64))
    }
}

#[derive(Clone)]
struct FailAfterNJournal {
    inner: InMemoryJournal,
    append_count: Arc<Mutex<usize>>,
    fail_at: usize,
}

impl FailAfterNJournal {
    fn new(fail_at: usize) -> Self {
        Self {
            inner: InMemoryJournal::default(),
            append_count: Arc::new(Mutex::new(0)),
            fail_at,
        }
    }

    fn recorded(&self) -> Vec<CommitEnvelope> {
        self.inner.recorded()
    }
}

impl CommitJournal for FailAfterNJournal {
    fn append(&self, envelope: CommitEnvelope) -> Result<CommitPosition, ContractError> {
        let mut append_count = self.append_count.lock().expect("append count should lock");
        *append_count += 1;
        if *append_count == self.fail_at {
            return Err(ContractError::Unavailable(
                "forced journal append failure".into(),
            ));
        }
        drop(append_count);
        self.inner.append(envelope)
    }
}

fn list_all_messages<J: CommitJournal>(
    runtime: &ConversationRuntime<J>,
    tenant_id: &str,
    conversation_id: &str,
    principal_id: &str,
) -> Result<conversation_runtime::MessageHistoryResult, RuntimeError> {
    runtime.list_messages_window(tenant_id, "default", conversation_id, principal_id, None, 100)
}

fn canonical_bind_direct_chat_command(
    tenant_id: &str,
    left_actor_id: &str,
    right_actor_id: &str,
) -> BindDirectChatConversationCommand {
    BindDirectChatConversationCommand {
        tenant_id: tenant_id.into(),
        organization_id: "0".into(),
        conversation_id: String::new(),
        direct_chat_id: String::new(),
        left_actor_id: left_actor_id.into(),
        left_actor_kind: "user".into(),
        right_actor_id: right_actor_id.into(),
        right_actor_kind: "user".into(),
        bound_by: "svc_control".into(),
    }
}

fn canonical_agent_dialog_command(requester_id: &str, agent_id: &str) -> CreateAgentDialogCommand {
    CreateAgentDialogCommand {
        tenant_id: "100001".into(),
        organization_id: "0".into(),
        conversation_id: String::new(),
        requester_id: requester_id.into(),
        agent_id: agent_id.into(),
    }
}

#[test]
fn test_message_history_window_rejects_invalid_limit_at_runtime_boundary() {
    let runtime = ConversationRuntime::new(InMemoryJournal::default());
    runtime
        .create_conversation(CreateConversationCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_history_limit_guard".into(),
            creator_id: "1".into(),
            conversation_type: "group".into(),
        })
        .expect("conversation should be created");

    for invalid_limit in [0, 1001] {
        let result = runtime.list_messages_window("100001", "default", "c_history_limit_guard",
            "1",
            None,
            invalid_limit,
        );
        assert!(matches!(
            result,
            Err(RuntimeError::InvalidInput(message))
                if message == format!("message history limit must be between 1 and 1000: {invalid_limit}")
        ));
    }
}

#[derive(Clone)]
struct FailNextBatchJournal {
    inner: InMemoryJournal,
    fail_batches_remaining: Arc<Mutex<usize>>,
}

impl FailNextBatchJournal {
    fn new(fail_batches_remaining: usize) -> Self {
        Self {
            inner: InMemoryJournal::default(),
            fail_batches_remaining: Arc::new(Mutex::new(fail_batches_remaining)),
        }
    }

    fn fail_next_batch(&self) {
        *self
            .fail_batches_remaining
            .lock()
            .expect("batch failure counter should lock") += 1;
    }

    fn recorded(&self) -> Vec<CommitEnvelope> {
        self.inner.recorded()
    }
}

impl CommitJournal for FailNextBatchJournal {
    fn append(&self, envelope: CommitEnvelope) -> Result<CommitPosition, ContractError> {
        self.inner.append(envelope)
    }

    fn append_batch(
        &self,
        envelopes: Vec<CommitEnvelope>,
    ) -> Result<Vec<CommitPosition>, ContractError> {
        let mut fail_batches_remaining = self
            .fail_batches_remaining
            .lock()
            .expect("batch failure counter should lock");
        if *fail_batches_remaining > 0 {
            *fail_batches_remaining -= 1;
            return Err(ContractError::Unavailable(
                "forced journal batch append failure".into(),
            ));
        }
        drop(fail_batches_remaining);

        let mut positions = Vec::with_capacity(envelopes.len());
        for envelope in envelopes {
            positions.push(self.inner.append(envelope)?);
        }
        Ok(positions)
    }
}

#[test]
fn test_create_conversation_does_not_leak_state_when_batch_commit_fails() {
    let journal = FailNextBatchJournal::new(1);
    let runtime = ConversationRuntime::new(journal.clone());

    let create_attempt = runtime.create_conversation(CreateConversationCommand {
        tenant_id: "100001".into(),
        organization_id: "0".into(),
        conversation_id: "c_group_create_batch_fail".into(),
        creator_id: "1".into(),
        conversation_type: "group".into(),
    });
    assert!(matches!(
        create_attempt,
        Err(RuntimeError::Contract(ContractError::Unavailable(message)))
            if message == "forced journal batch append failure"
    ));
    assert!(matches!(
        runtime.list_members("100001", "default", "c_group_create_batch_fail"),
        Err(RuntimeError::ConversationNotFound(conversation_id))
            if conversation_id == "c_group_create_batch_fail"
    ));
    assert!(
        journal.recorded().is_empty(),
        "failed create must not durably append any creation event"
    );

    let created = runtime
        .create_conversation(CreateConversationCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_group_create_batch_fail".into(),
            creator_id: "1".into(),
            conversation_type: "group".into(),
        })
        .expect("retry should succeed after failed batch");

    assert_eq!(created.conversation_id, "c_group_create_batch_fail");
    let members = runtime
        .list_members("100001", "default", "c_group_create_batch_fail")
        .expect("members should exist after retry");
    assert_eq!(members.len(), 1);
    assert_eq!(members[0].principal_id, "1");
    assert_eq!(journal.recorded().len(), 2);
}

#[test]
fn test_bind_direct_chat_does_not_leak_state_when_batch_commit_fails() {
    let journal = FailNextBatchJournal::new(1);
    let runtime = ConversationRuntime::new(journal.clone());

    let bind_attempt = runtime.bind_direct_chat_conversation_with_binder_kind(
        canonical_bind_direct_chat_command("100001", "actor_a", "actor_b"),
        "system",
    );
    assert!(matches!(
        bind_attempt,
        Err(RuntimeError::Contract(ContractError::Unavailable(message)))
            if message == "forced journal batch append failure"
    ));
    assert!(
        journal.recorded().is_empty(),
        "failed direct chat bind must not durably append any creation event"
    );

    let created = runtime
        .bind_direct_chat_conversation_with_binder_kind(
            canonical_bind_direct_chat_command("100001", "actor_a", "actor_b"),
            "system",
        )
        .expect("retry should succeed after failed direct chat bind");

    let conversation_id = created.conversation_id.clone();
    assert!(conversation_id.starts_with("c_direct_"));
    let binding = runtime
        .conversation_business_binding("100001", "default", conversation_id.as_str())
        .expect("binding should exist after retry");
    assert_eq!(binding.business_type, "direct_chat");
    assert!(!binding.business_id.starts_with("pc-dc-"));
    let members = runtime
        .list_members("100001", "default", conversation_id.as_str())
        .expect("direct chat members should exist after retry");
    assert_eq!(members.len(), 2);
    assert_eq!(journal.recorded().len(), 3);
}

#[test]
fn test_create_thread_does_not_leak_state_when_batch_commit_fails() {
    let journal = FailNextBatchJournal::new(0);
    let runtime = ConversationRuntime::new(journal.clone());

    runtime
        .create_conversation(CreateConversationCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_parent_thread_batch_fail".into(),
            creator_id: "1".into(),
            conversation_type: "group".into(),
        })
        .expect("parent conversation should succeed");
    runtime
        .add_member(AddConversationMemberCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_parent_thread_batch_fail".into(),
            principal_id: "1051".into(),
            principal_kind: "user".into(),
            role: MembershipRole::Member,
            invited_by: "1".into(),
        })
        .expect("root author should join parent conversation");
    let root_message = runtime
        .post_message(PostMessageCommand {
            tenant_id: "100001".into(),
           organization_id: "0".into(),
            conversation_id: "c_parent_thread_batch_fail".into(),
            sender: Sender {
                id: "1051".into(),
                kind: "user".into(),
                member_id: None,
                device_id: Some("d_root_author".into()),
                session_id: Some("s_root_author".into()),
                metadata: Default::default(),
            },
            client_msg_id: Some("client_thread_batch_fail_root".into()),
            message_type: MessageType::Standard,
            body: MessageBody {
                summary: Some("root".into()),
                parts: vec![ContentPart::text("root")],
                render_hints: Default::default(),
                reply_to: None,
            },
        })
        .expect("root message should succeed");

    journal.fail_next_batch();

    let create_attempt = runtime.create_thread_conversation_with_creator_kind(
        CreateThreadConversationCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_thread_batch_fail".into(),
            parent_conversation_id: "c_parent_thread_batch_fail".into(),
            root_message_id: root_message.message_id.clone(),
            creator_id: "1".into(),
        },
        "user",
    );
    assert!(matches!(
        create_attempt,
        Err(RuntimeError::Contract(ContractError::Unavailable(message)))
            if message == "forced journal batch append failure"
    ));
    assert!(matches!(
        runtime.list_members("100001", "default", "c_thread_batch_fail"),
        Err(RuntimeError::ConversationNotFound(conversation_id))
            if conversation_id == "c_thread_batch_fail"
    ));
    assert!(matches!(
        runtime.conversation_business_binding("100001", "default", "c_thread_batch_fail"),
        Err(RuntimeError::ConversationNotFound(conversation_id))
            if conversation_id == "c_thread_batch_fail"
    ));
    assert_eq!(
        journal.recorded().len(),
        4,
        "failed thread create must not append any additional events beyond parent setup"
    );

    let created = runtime
        .create_thread_conversation_with_creator_kind(
            CreateThreadConversationCommand {
                tenant_id: "100001".into(),
                organization_id: "0".into(),
                conversation_id: "c_thread_batch_fail".into(),
                parent_conversation_id: "c_parent_thread_batch_fail".into(),
                root_message_id: root_message.message_id.clone(),
                creator_id: "1".into(),
            },
            "user",
        )
        .expect("retry should succeed after failed thread batch");

    assert_eq!(created.conversation_id, "c_thread_batch_fail");
    let binding = runtime
        .conversation_business_binding("100001", "default", "c_thread_batch_fail")
        .expect("thread binding should exist after retry");
    assert_eq!(binding.business_type, "thread");
    assert_eq!(binding.business_id, root_message.message_id);
    let members = runtime
        .list_members("100001", "default", "c_thread_batch_fail")
        .expect("thread members should exist after retry");
    assert_eq!(members.len(), 2);
    assert_eq!(journal.recorded().len(), 7);
}

#[test]
fn test_create_conversation_and_post_message_emits_commit_events_in_order() {
    let journal = InMemoryJournal::default();
    let runtime = ConversationRuntime::new(journal.clone());

    let conversation = runtime
        .create_conversation(CreateConversationCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_demo".into(),
            creator_id: "1".into(),
            conversation_type: "group".into(),
        })
        .expect("create conversation should succeed");

    assert_eq!(conversation.conversation_id, "c_demo");

    let message = runtime
        .post_message(PostMessageCommand {
            tenant_id: "100001".into(),
           organization_id: "0".into(),
            conversation_id: "c_demo".into(),
            sender: Sender {
                id: "1".into(),
                kind: "user".into(),
                member_id: Some("cm_demo".into()),
                device_id: Some("d_demo".into()),
                session_id: Some("s_demo".into()),
                metadata: Default::default(),
            },
            client_msg_id: Some("client_demo".into()),
            message_type: MessageType::Standard,
            body: MessageBody {
                summary: Some("hello".into()),
                parts: vec![ContentPart::text("hello")],
                render_hints: Default::default(),
                reply_to: None,
            },
        })
        .expect("post message should succeed");

    assert_eq!(message.message_seq, 1);
    assert_eq!(message.message_id, "msg_c_demo_1");

    let events = journal.recorded();
    assert_eq!(events.len(), 3);
    assert_eq!(events[0].event_type, "conversation.created");
    assert_eq!(events[1].event_type, "conversation.member_joined");
    assert_eq!(events[2].event_type, "message.posted");
    assert_eq!(events[2].ordering_seq, 1);
}

#[test]
fn test_duplicate_create_conversation_is_idempotent_and_conflicting_retry_is_rejected() {
    let journal = InMemoryJournal::default();
    let runtime = ConversationRuntime::new(journal.clone());

    let first = runtime
        .create_conversation(CreateConversationCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_create_retry".into(),
            creator_id: "1".into(),
            conversation_type: "group".into(),
        })
        .expect("first create should succeed");

    assert_eq!(first.delivery_status.as_ref().unwrap().as_str(), "applied");
    assert_eq!(
        first.proof_version.as_deref(),
        Some("conversation.create.delivery-proof.v1")
    );
    assert_eq!(
        first.request_key.as_deref(),
        Some("6#1000014#user1#119#create-conversation14#c_create_retry")
    );

    let duplicate = runtime
        .create_conversation(CreateConversationCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_create_retry".into(),
            creator_id: "1".into(),
            conversation_type: "group".into(),
        })
        .expect("duplicate create should be idempotent");

    assert_eq!(duplicate.conversation_id, first.conversation_id);
    assert_eq!(duplicate.event_id, first.event_id);
    assert_eq!(duplicate.request_key, first.request_key);
    assert_eq!(duplicate.proof_version, first.proof_version);
    assert_eq!(
        duplicate.delivery_status.as_ref().unwrap().as_str(),
        "replayed"
    );

    let members = runtime
        .list_members("100001", "default", "c_create_retry")
        .expect("members should list");
    assert_eq!(members.len(), 1);
    assert_eq!(members[0].principal_id, "1");

    let conflicting_retry = runtime.create_conversation(CreateConversationCommand {
        tenant_id: "100001".into(),
        organization_id: "0".into(),
        conversation_id: "c_create_retry".into(),
        creator_id: "1".into(),
        conversation_type: "direct".into(),
    });
    assert!(matches!(conflicting_retry, Err(RuntimeError::Conflict(_))));

    let events = journal.recorded();
    assert_eq!(
        events.len(),
        2,
        "duplicate create retry must not append another conversation.created/member_joined pair"
    );
}

#[test]
fn test_conversation_scope_key_is_segment_safe_for_delimiter_bearing_ids() {
    let runtime = ConversationRuntime::new(InMemoryJournal::default());

    let first = runtime
        .create_conversation(CreateConversationCommand {
            tenant_id: "tenant:a".into(),
            organization_id: "0".into(),
            conversation_id: "b".into(),
            creator_id: "1052".into(),
            conversation_type: "group".into(),
        })
        .expect("first delimiter-bearing conversation should be created");
    let second = runtime
        .create_conversation(CreateConversationCommand {
            tenant_id: "tenant".into(),
            organization_id: "0".into(),
            conversation_id: "a:b".into(),
            creator_id: "1053".into(),
            conversation_type: "group".into(),
        })
        .expect("second delimiter-bearing conversation should not collide with first");

    assert_eq!(first.conversation_id, "b");
    assert_eq!(second.conversation_id, "a:b");
    assert_eq!(
        first.request_key.as_deref(),
        Some("8#tenant:a4#user4#105219#create-conversation1#b")
    );
    assert_eq!(
        second.request_key.as_deref(),
        Some("6#tenant4#user4#105319#create-conversation3#a:b")
    );

    let first_members = runtime
        .list_members("tenant:a", "default", "b")
        .expect("first conversation members should list");
    let second_members = runtime
        .list_members("tenant", "default", "a:b")
        .expect("second conversation members should list");
    assert_eq!(first_members.len(), 1);
    assert_eq!(first_members[0].principal_id, "1052");
    assert_eq!(second_members.len(), 1);
    assert_eq!(second_members[0].principal_id, "1053");
}

#[test]
fn test_duplicate_post_message_is_idempotent_and_conflicting_retry_is_rejected() {
    let journal = InMemoryJournal::default();
    let runtime = ConversationRuntime::new(journal.clone());

    runtime
        .create_conversation(CreateConversationCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_post_retry".into(),
            creator_id: "1".into(),
            conversation_type: "group".into(),
        })
        .expect("create conversation should succeed");

    let first = runtime
        .post_message(PostMessageCommand {
            tenant_id: "100001".into(),
           organization_id: "0".into(),
            conversation_id: "c_post_retry".into(),
            sender: Sender {
                id: "1".into(),
                kind: "user".into(),
                member_id: None,
                device_id: Some("d_demo".into()),
                session_id: Some("s_demo".into()),
                metadata: Default::default(),
            },
            client_msg_id: Some("client_post_retry".into()),
            message_type: MessageType::Standard,
            body: MessageBody {
                summary: Some("hello".into()),
                parts: vec![ContentPart::text("hello")],
                render_hints: Default::default(),
                reply_to: None,
            },
        })
        .expect("first post should succeed");

    let duplicate = runtime
        .post_message(PostMessageCommand {
            tenant_id: "100001".into(),
           organization_id: "0".into(),
            conversation_id: "c_post_retry".into(),
            sender: Sender {
                id: "1".into(),
                kind: "user".into(),
                member_id: None,
                device_id: Some("d_demo".into()),
                session_id: Some("s_demo_retry".into()),
                metadata: Default::default(),
            },
            client_msg_id: Some("client_post_retry".into()),
            message_type: MessageType::Standard,
            body: MessageBody {
                summary: Some("hello".into()),
                parts: vec![ContentPart::text("hello")],
                render_hints: Default::default(),
                reply_to: None,
            },
        })
        .expect("duplicate same-input post should be idempotent");

    assert_eq!(
        duplicate.message_id, first.message_id,
        "idempotent retry should resolve to the original message id"
    );
    assert_eq!(
        duplicate.message_seq, first.message_seq,
        "idempotent retry should resolve to the original message seq"
    );
    assert_eq!(
        duplicate.event_id, first.event_id,
        "idempotent retry should resolve to the original event id"
    );

    let history = list_all_messages(&runtime, "100001", "c_post_retry", "1")
        .expect("history should list");
    assert_eq!(
        history.items.len(),
        1,
        "duplicate same-input retry must not append a second stored message"
    );
    assert_eq!(history.high_watermark, 1);

    let conflicting_retry = runtime.post_message(PostMessageCommand {
        tenant_id: "100001".into(),
       organization_id: "0".into(),
        conversation_id: "c_post_retry".into(),
        sender: Sender {
            id: "1".into(),
            kind: "user".into(),
            member_id: None,
            device_id: Some("d_demo".into()),
            session_id: Some("s_demo_retry_conflict".into()),
            metadata: Default::default(),
        },
        client_msg_id: Some("client_post_retry".into()),
        message_type: MessageType::Standard,
        body: MessageBody {
            summary: Some("hello conflict".into()),
            parts: vec![ContentPart::text("hello conflict")],
            render_hints: Default::default(),
            reply_to: None,
        },
    });

    assert!(matches!(conflicting_retry, Err(RuntimeError::Conflict(_))));

    let events = journal.recorded();
    assert_eq!(
        events.len(),
        3,
        "duplicate post retry must not append another message.posted event"
    );
}

#[test]
fn test_rtc_signal_message_backfills_top_level_rtc_session_id_from_signal_payload() {
    let runtime = ConversationRuntime::new(InMemoryJournal::default());

    runtime
        .create_conversation(CreateConversationCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_rtc_signal_backfill".into(),
            creator_id: "1".into(),
            conversation_type: "group".into(),
        })
        .expect("create conversation should succeed");

    runtime
        .post_message(PostMessageCommand {
            tenant_id: "100001".into(),
           organization_id: "0".into(),
            conversation_id: "c_rtc_signal_backfill".into(),
            sender: Sender {
                id: "1".into(),
                kind: "user".into(),
                member_id: None,
                device_id: Some("d_demo".into()),
                session_id: Some("s_demo".into()),
                metadata: Default::default(),
            },
            client_msg_id: None,
            message_type: MessageType::Signal,
            body: MessageBody {
                summary: Some("rtc.accept".into()),
                parts: vec![ContentPart::Signal(im_domain_core::message::SignalPart {
                    signal_type: "rtc.accept".into(),
                    schema_ref: Some("rtc.signal.v1".into()),
                    payload: r#"{"rtcSessionId":"rtc_runtime_backfill","state":"accepted"}"#.into(),
                })],
                render_hints: BTreeMap::from([("channel".into(), "rtc".into())]),
                reply_to: None,
            },
        })
        .expect("signal message should post");

    let history = list_all_messages(&runtime, "100001", "c_rtc_signal_backfill", "1")
        .expect("history should list");
    assert_eq!(history.items.len(), 1);
    assert_eq!(
        history.items[0].message.rtc_session_id.as_deref(),
        Some("rtc_runtime_backfill")
    );
}

#[test]
fn test_runtime_replays_recorded_conversation_events_after_rebuild() {
    let source_journal = InMemoryJournal::default();
    let source_runtime = ConversationRuntime::new(source_journal.clone());

    source_runtime
        .create_conversation(CreateConversationCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_replay".into(),
            creator_id: "1".into(),
            conversation_type: "group".into(),
        })
        .expect("create conversation should succeed");

    source_runtime
        .post_message(PostMessageCommand {
            tenant_id: "100001".into(),
           organization_id: "0".into(),
            conversation_id: "c_replay".into(),
            sender: Sender {
                id: "1".into(),
                kind: "user".into(),
                member_id: None,
                device_id: None,
                session_id: Some("s_demo".into()),
                metadata: Default::default(),
            },
            client_msg_id: Some("client_replay_1".into()),
            message_type: MessageType::Standard,
            body: MessageBody {
                summary: Some("first".into()),
                parts: vec![ContentPart::text("first")],
                render_hints: Default::default(),
                reply_to: None,
            },
        })
        .expect("first post should succeed");

    let replay_journal = InMemoryJournal::default();
    let replay_runtime = ConversationRuntime::new(replay_journal);
    for envelope in source_journal.recorded() {
        replay_runtime
            .apply_recovered_envelope(&envelope)
            .expect("replay should succeed");
    }

    let members = replay_runtime
        .list_members("100001", "default", "c_replay")
        .expect("members should exist after replay");
    assert_eq!(members.len(), 1);
    assert_eq!(members[0].principal_id, "1");

    let posted = replay_runtime
        .post_message(PostMessageCommand {
            tenant_id: "100001".into(),
           organization_id: "0".into(),
            conversation_id: "c_replay".into(),
            sender: Sender {
                id: "1".into(),
                kind: "user".into(),
                member_id: None,
                device_id: None,
                session_id: Some("s_demo_new".into()),
                metadata: Default::default(),
            },
            client_msg_id: Some("client_replay_2".into()),
            message_type: MessageType::Standard,
            body: MessageBody {
                summary: Some("second".into()),
                parts: vec![ContentPart::text("second")],
                render_hints: Default::default(),
                reply_to: None,
            },
        })
        .expect("post after replay should succeed");
    assert_eq!(posted.message_seq, 2);
    assert_eq!(posted.message_id, "msg_c_replay_2");
}

#[test]
fn test_same_conversation_id_is_isolated_per_tenant() {
    let journal = InMemoryJournal::default();
    let runtime = ConversationRuntime::new(journal);

    runtime
        .create_conversation(CreateConversationCommand {
            tenant_id: "t_alpha".into(),
            organization_id: "0".into(),
            conversation_id: "c_shared".into(),
            creator_id: "1045".into(),
            conversation_type: "group".into(),
        })
        .expect("tenant alpha conversation should succeed");

    runtime
        .create_conversation(CreateConversationCommand {
            tenant_id: "t_beta".into(),
            organization_id: "0".into(),
            conversation_id: "c_shared".into(),
            creator_id: "1046".into(),
            conversation_type: "group".into(),
        })
        .expect("tenant beta conversation should succeed");

    let alpha_message = runtime
        .post_message(PostMessageCommand {
            tenant_id: "t_alpha".into(),
           organization_id: "0".into(),
            conversation_id: "c_shared".into(),
            sender: Sender {
                id: "1045".into(),
                kind: "user".into(),
                member_id: Some("cm_alpha".into()),
                device_id: Some("d_alpha".into()),
                session_id: Some("s_alpha".into()),
                metadata: Default::default(),
            },
            client_msg_id: Some("client_alpha".into()),
            message_type: MessageType::Standard,
            body: MessageBody {
                summary: Some("hello alpha".into()),
                parts: vec![ContentPart::text("hello alpha")],
                render_hints: Default::default(),
                reply_to: None,
            },
        })
        .expect("tenant alpha message should succeed");

    let beta_message = runtime
        .post_message(PostMessageCommand {
            tenant_id: "t_beta".into(),
           organization_id: "0".into(),
            conversation_id: "c_shared".into(),
            sender: Sender {
                id: "1046".into(),
                kind: "user".into(),
                member_id: Some("cm_beta".into()),
                device_id: None,
                session_id: Some("s_beta".into()),
                metadata: Default::default(),
            },
            client_msg_id: Some("client_beta".into()),
            message_type: MessageType::Standard,
            body: MessageBody {
                summary: Some("hello beta".into()),
                parts: vec![ContentPart::text("hello beta")],
                render_hints: Default::default(),
                reply_to: None,
            },
        })
        .expect("tenant beta message should succeed");

    assert_eq!(alpha_message.message_seq, 1);
    assert_eq!(beta_message.message_seq, 1);
}

#[test]
fn test_post_message_rejects_sender_kind_mismatch_against_member_principal_kind() {
    let journal = InMemoryJournal::default();
    let runtime = ConversationRuntime::new(journal);

    runtime
        .create_conversation(CreateConversationCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_kind_guard".into(),
            conversation_type: "group".into(),
            creator_id: "1".into(),
        })
        .expect("group create should succeed");

    let post = runtime.post_message(PostMessageCommand {
        tenant_id: "100001".into(),
       organization_id: "0".into(),
        conversation_id: "c_kind_guard".into(),
        sender: Sender {
            id: "1".into(),
            kind: "agent".into(),
            member_id: None,
            device_id: None,
            session_id: Some("s_demo".into()),
            metadata: Default::default(),
        },
        client_msg_id: Some("client_kind_mismatch".into()),
        message_type: MessageType::Standard,
        body: MessageBody {
            summary: Some("should fail".into()),
            parts: vec![ContentPart::text("should fail")],
            render_hints: Default::default(),
            reply_to: None,
        },
    });

    assert!(matches!(post, Err(RuntimeError::PermissionDenied(_))));
}

#[test]
fn test_edit_message_rejects_editor_kind_mismatch_against_member_principal_kind() {
    let journal = InMemoryJournal::default();
    let runtime = ConversationRuntime::new(journal);

    runtime
        .create_conversation(CreateConversationCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_edit_kind_guard".into(),
            conversation_type: "group".into(),
            creator_id: "1".into(),
        })
        .expect("group create should succeed");

    let posted = runtime
        .post_message(PostMessageCommand {
            tenant_id: "100001".into(),
           organization_id: "0".into(),
            conversation_id: "c_edit_kind_guard".into(),
            sender: Sender {
                id: "1".into(),
                kind: "user".into(),
                member_id: None,
                device_id: None,
                session_id: Some("s_demo".into()),
                metadata: Default::default(),
            },
            client_msg_id: Some("client_edit_kind_guard".into()),
            message_type: MessageType::Standard,
            body: MessageBody {
                summary: Some("before edit".into()),
                parts: vec![ContentPart::text("before edit")],
                render_hints: Default::default(),
                reply_to: None,
            },
        })
        .expect("post message should succeed");

    let edit = runtime.edit_message(EditMessageCommand {
        tenant_id: "100001".into(),
        organization_id: "0".into(),
        message_id: posted.message_id,
        editor: Sender {
            id: "1".into(),
            kind: "agent".into(),
            member_id: None,
            device_id: None,
            session_id: Some("s_demo".into()),
            metadata: Default::default(),
        },
        body: MessageBody {
            summary: Some("should fail".into()),
            parts: vec![ContentPart::text("should fail")],
            render_hints: Default::default(),
            reply_to: None,
        },
    });

    assert!(matches!(edit, Err(RuntimeError::PermissionDenied(_))));
}

#[test]
fn test_recall_message_rejects_actor_kind_mismatch_against_member_principal_kind() {
    let journal = InMemoryJournal::default();
    let runtime = ConversationRuntime::new(journal);

    runtime
        .create_conversation(CreateConversationCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_recall_kind_guard".into(),
            conversation_type: "group".into(),
            creator_id: "1".into(),
        })
        .expect("group create should succeed");

    let posted = runtime
        .post_message(PostMessageCommand {
            tenant_id: "100001".into(),
           organization_id: "0".into(),
            conversation_id: "c_recall_kind_guard".into(),
            sender: Sender {
                id: "1".into(),
                kind: "user".into(),
                member_id: None,
                device_id: None,
                session_id: Some("s_demo".into()),
                metadata: Default::default(),
            },
            client_msg_id: Some("client_recall_kind_guard".into()),
            message_type: MessageType::Standard,
            body: MessageBody {
                summary: Some("before recall".into()),
                parts: vec![ContentPart::text("before recall")],
                render_hints: Default::default(),
                reply_to: None,
            },
        })
        .expect("post message should succeed");

    let recall = runtime.recall_message(RecallMessageCommand {
        tenant_id: "100001".into(),
        organization_id: "0".into(),
        message_id: posted.message_id,
        recalled_by: Sender {
            id: "1".into(),
            kind: "agent".into(),
            member_id: None,
            device_id: None,
            session_id: Some("s_demo".into()),
            metadata: Default::default(),
        },
    });

    assert!(matches!(recall, Err(RuntimeError::PermissionDenied(_))));
}

#[test]
fn test_generic_create_rejects_unknown_and_reserved_special_conversation_types() {
    let journal = InMemoryJournal::default();
    let runtime = ConversationRuntime::new(journal.clone());

    for (conversation_id, conversation_type) in [
        ("c_unknown_type", "workspace"),
        ("c_agent_dialog_type", "agent_dialog"),
        ("c_agent_handoff_type", "agent_handoff"),
        ("c_system_channel_type", "system_channel"),
    ] {
        let create = runtime.create_conversation(CreateConversationCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: conversation_id.into(),
            creator_id: "1".into(),
            conversation_type: conversation_type.into(),
        });

        assert!(
            create.is_err(),
            "conversation type should be rejected: {conversation_type}"
        );
    }
    assert!(journal.recorded().is_empty());
}

#[test]
fn test_create_agent_dialog_rejects_non_standard_agent_id() {
    let journal = InMemoryJournal::default();
    let runtime = ConversationRuntime::new(journal.clone());

    let create = runtime.create_agent_dialog_with_requester_kind(
        CreateAgentDialogCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_agent_dialog_invalid_agent_id".into(),
            requester_id: "1".into(),
            agent_id: "ag_demo".into(),
        },
        "user",
    );

    assert!(matches!(
        create,
        Err(RuntimeError::AgentIdInvalid(message))
            if message == "agentId must start with agent."
    ));
    assert!(journal.recorded().is_empty());
}

#[test]
fn test_create_agent_dialog_creates_requester_and_agent_members() {
    let journal = InMemoryJournal::default();
    let runtime = ConversationRuntime::new(journal.clone());

    let created = runtime
        .create_agent_dialog_with_requester_kind(
            canonical_agent_dialog_command("1", "agent.demo"),
            "user",
        )
        .expect("agent dialog create should succeed");

    let conversation_id = created.conversation_id.clone();
    assert!(conversation_id.starts_with("c_agent_"));

    let members = runtime
        .list_members("100001", "default", conversation_id.as_str())
        .expect("agent dialog members should list");
    assert_eq!(members.len(), 2);

    let requester = members
        .iter()
        .find(|member| member.principal_id == "1")
        .expect("requester member should exist");
    assert_eq!(requester.principal_kind, "user");
    assert_eq!(requester.role, MembershipRole::Owner);
    assert_eq!(requester.state, MembershipState::Joined);

    let agent = members
        .iter()
        .find(|member| member.principal_id == "agent.demo")
        .expect("agent member should exist");
    assert_eq!(agent.principal_kind, "agent");
    assert_eq!(agent.role, MembershipRole::Member);
    assert_eq!(agent.state, MembershipState::Joined);
    assert_eq!(agent.invited_by.as_deref(), Some("1"));

    let events = journal.recorded();
    assert_eq!(events.len(), 3);
    assert_eq!(events[0].event_type, "conversation.created");
    assert_eq!(events[1].event_type, "conversation.member_joined");
    assert_eq!(events[2].event_type, "conversation.member_joined");
}

#[test]
fn test_duplicate_create_agent_dialog_is_idempotent_and_conflicting_retry_is_rejected() {
    let source_journal = InMemoryJournal::default();
    let source_runtime = ConversationRuntime::new(source_journal.clone());

    let first = source_runtime
        .create_agent_dialog_with_requester_kind(
            canonical_agent_dialog_command("1", "agent.demo"),
            "user",
        )
        .expect("first agent dialog create should succeed");

    assert_eq!(first.delivery_status.as_ref().unwrap().as_str(), "applied");
    assert_eq!(
        first.proof_version.as_deref(),
        Some("conversation.create.delivery-proof.v1")
    );
    assert!(first.request_key.is_some());

    let conversation_id = first.conversation_id.clone();

    let duplicate = source_runtime
        .create_agent_dialog_with_requester_kind(
            canonical_agent_dialog_command("1", "agent.demo"),
            "user",
        )
        .expect("duplicate agent dialog create should replay");

    assert_eq!(duplicate.conversation_id, first.conversation_id);
    assert_eq!(duplicate.event_id, first.event_id);
    assert_eq!(duplicate.request_key, first.request_key);
    assert_eq!(duplicate.proof_version, first.proof_version);
    assert_eq!(
        duplicate.delivery_status.as_ref().unwrap().as_str(),
        "replayed"
    );

    let conflicting_retry = source_runtime.create_agent_dialog_with_requester_kind(
        CreateAgentDialogCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: conversation_id.clone(),
            requester_id: "1".into(),
            agent_id: "agent.other".into(),
        },
        "user",
    );
    assert!(matches!(conflicting_retry, Err(RuntimeError::InvalidInput(_))));

    let replay_runtime = ConversationRuntime::new(InMemoryJournal::default());
    for envelope in source_journal.recorded() {
        replay_runtime
            .apply_recovered_envelope(&envelope)
            .expect("agent dialog replay should succeed");
    }

    let recovered_duplicate = replay_runtime
        .create_agent_dialog_with_requester_kind(
            canonical_agent_dialog_command("1", "agent.demo"),
            "user",
        )
        .expect("recovered duplicate agent dialog create should replay");
    assert_eq!(recovered_duplicate.event_id, first.event_id);
    assert_eq!(recovered_duplicate.request_key, first.request_key);
    assert_eq!(
        recovered_duplicate
            .delivery_status
            .as_ref()
            .unwrap()
            .as_str(),
        "replayed"
    );

    let events = source_journal.recorded();
    assert_eq!(
        events.len(),
        3,
        "duplicate agent dialog create retry must not append another conversation.created/member_joined pair"
    );
}

#[test]
fn test_create_agent_dialog_rejects_non_user_requester_kind() {
    let journal = InMemoryJournal::default();
    let runtime = ConversationRuntime::new(journal.clone());

    let create = runtime.create_agent_dialog_with_requester_kind(
        CreateAgentDialogCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_agent_dialog_invalid".into(),
            requester_id: "svc_ops".into(),
            agent_id: "agent.demo".into(),
        },
        "system",
    );

    assert!(matches!(create, Err(RuntimeError::PermissionDenied(_))));
    assert!(journal.recorded().is_empty());
}

#[test]
fn test_create_system_channel_creates_system_and_subscriber_members() {
    let journal = InMemoryJournal::default();
    let runtime = ConversationRuntime::new(journal.clone());

    let created = runtime
        .create_system_channel_with_requester_kind(
            CreateSystemChannelCommand {
                tenant_id: "100001".into(),
                organization_id: "0".into(),
                conversation_id: "c_system_channel".into(),
                requester_id: "svc_ops".into(),
                subscriber_id: "1".into(),
            },
            "system",
        )
        .expect("system channel create should succeed");

    assert_eq!(created.conversation_id, "c_system_channel");

    let members = runtime
        .list_members("100001", "default", "c_system_channel")
        .expect("system channel members should list");
    assert_eq!(members.len(), 2);

    let publisher = members
        .iter()
        .find(|member| member.principal_id == "svc_ops")
        .expect("system publisher should exist");
    assert_eq!(publisher.principal_kind, "system");
    assert_eq!(publisher.role, MembershipRole::Owner);
    assert_eq!(
        publisher.attributes.get("channelRole").map(String::as_str),
        Some("publisher")
    );

    let subscriber = members
        .iter()
        .find(|member| member.principal_id == "1")
        .expect("subscriber should exist");
    assert_eq!(subscriber.principal_kind, "user");
    assert_eq!(subscriber.role, MembershipRole::Member);
    assert_eq!(subscriber.invited_by.as_deref(), Some("svc_ops"));
    assert_eq!(
        subscriber.attributes.get("channelRole").map(String::as_str),
        Some("subscriber")
    );

    let cursor = runtime
        .read_cursor_view("100001", "default", "c_system_channel", "1")
        .expect("subscriber read cursor should be initialized");
    assert_eq!(cursor.member_id, subscriber.member_id);
    assert_eq!(cursor.read_seq, 0);

    let events = journal.recorded();
    assert_eq!(events.len(), 3);
    assert_eq!(events[0].event_type, "conversation.created");
    assert_eq!(events[0].actor.actor_kind, "system");
    assert_eq!(events[1].event_type, "conversation.member_joined");
    assert_eq!(events[1].actor.actor_kind, "system");
    assert_eq!(events[2].event_type, "conversation.member_joined");
    assert_eq!(events[2].actor.actor_kind, "system");
}

#[test]
fn test_create_system_channel_rejects_non_system_requester_kind() {
    let journal = InMemoryJournal::default();
    let runtime = ConversationRuntime::new(journal.clone());

    let create = runtime.create_system_channel_with_requester_kind(
        CreateSystemChannelCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_system_channel_invalid".into(),
            requester_id: "1".into(),
            subscriber_id: "1042".into(),
        },
        "user",
    );

    assert!(matches!(create, Err(RuntimeError::PermissionDenied(_))));
    assert!(journal.recorded().is_empty());
}

#[test]
fn test_duplicate_create_system_channel_is_idempotent_and_conflicting_retry_is_rejected() {
    let source_journal = InMemoryJournal::default();
    let source_runtime = ConversationRuntime::new(source_journal.clone());

    let first = source_runtime
        .create_system_channel_with_requester_kind(
            CreateSystemChannelCommand {
                tenant_id: "100001".into(),
                organization_id: "0".into(),
                conversation_id: "c_system_channel_retry".into(),
                requester_id: "svc_ops".into(),
                subscriber_id: "1".into(),
            },
            "system",
        )
        .expect("first system channel create should succeed");

    assert_eq!(first.delivery_status.as_ref().unwrap().as_str(), "applied");
    assert_eq!(
        first.proof_version.as_deref(),
        Some("conversation.create.delivery-proof.v1")
    );
    assert_eq!(
        first.request_key.as_deref(),
        Some("6#1000016#system7#svc_ops21#create-system_channel22#c_system_channel_retry")
    );

    let duplicate = source_runtime
        .create_system_channel_with_requester_kind(
            CreateSystemChannelCommand {
                tenant_id: "100001".into(),
                organization_id: "0".into(),
                conversation_id: "c_system_channel_retry".into(),
                requester_id: "svc_ops".into(),
                subscriber_id: "1".into(),
            },
            "system",
        )
        .expect("duplicate system channel create should replay");

    assert_eq!(duplicate.conversation_id, first.conversation_id);
    assert_eq!(duplicate.event_id, first.event_id);
    assert_eq!(duplicate.request_key, first.request_key);
    assert_eq!(duplicate.proof_version, first.proof_version);
    assert_eq!(
        duplicate.delivery_status.as_ref().unwrap().as_str(),
        "replayed"
    );

    let conflicting_retry = source_runtime.create_system_channel_with_requester_kind(
        CreateSystemChannelCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_system_channel_retry".into(),
            requester_id: "svc_ops".into(),
            subscriber_id: "1041".into(),
        },
        "system",
    );
    assert!(matches!(conflicting_retry, Err(RuntimeError::Conflict(_))));

    let replay_runtime = ConversationRuntime::new(InMemoryJournal::default());
    for envelope in source_journal.recorded() {
        replay_runtime
            .apply_recovered_envelope(&envelope)
            .expect("system channel replay should succeed");
    }

    let recovered_duplicate = replay_runtime
        .create_system_channel_with_requester_kind(
            CreateSystemChannelCommand {
                tenant_id: "100001".into(),
                organization_id: "0".into(),
                conversation_id: "c_system_channel_retry".into(),
                requester_id: "svc_ops".into(),
                subscriber_id: "1".into(),
            },
            "system",
        )
        .expect("recovered duplicate system channel create should replay");
    assert_eq!(recovered_duplicate.event_id, first.event_id);
    assert_eq!(recovered_duplicate.request_key, first.request_key);
    assert_eq!(
        recovered_duplicate
            .delivery_status
            .as_ref()
            .unwrap()
            .as_str(),
        "replayed"
    );

    let events = source_journal.recorded();
    assert_eq!(
        events.len(),
        3,
        "duplicate system channel create retry must not append another conversation.created/member_joined pair"
    );
}

#[test]
fn test_create_agent_handoff_creates_source_agent_and_target_members() {
    let journal = InMemoryJournal::default();
    let runtime = ConversationRuntime::new(journal.clone());

    let created = runtime
        .create_agent_handoff_with_source_kind(
            CreateAgentHandoffCommand {
                tenant_id: "100001".into(),
                organization_id: "0".into(),
                conversation_id: "c_agent_handoff".into(),
                source_id: "ag_source".into(),
                target_id: "1".into(),
                target_kind: "user".into(),
                handoff_session_id: "hs_demo".into(),
                handoff_reason: Some("manual_escalation".into()),
            },
            "agent",
        )
        .expect("agent handoff create should succeed");

    assert_eq!(created.conversation_id, "c_agent_handoff");

    let members = runtime
        .list_members("100001", "default", "c_agent_handoff")
        .expect("agent handoff members should list");
    assert_eq!(members.len(), 2);

    let source = members
        .iter()
        .find(|member| member.principal_id == "ag_source")
        .expect("source agent should exist");
    assert_eq!(source.principal_kind, "agent");
    assert_eq!(source.role, MembershipRole::Owner);
    assert_eq!(
        source.attributes.get("handoffRole").map(String::as_str),
        Some("source")
    );
    assert_eq!(
        source
            .attributes
            .get("handoffSessionId")
            .map(String::as_str),
        Some("hs_demo")
    );

    let target = members
        .iter()
        .find(|member| member.principal_id == "1")
        .expect("target member should exist");
    assert_eq!(target.principal_kind, "user");
    assert_eq!(target.role, MembershipRole::Member);
    assert_eq!(target.invited_by.as_deref(), Some("ag_source"));
    assert_eq!(
        target.attributes.get("handoffRole").map(String::as_str),
        Some("target")
    );
    assert_eq!(
        target.attributes.get("sourceAgentId").map(String::as_str),
        Some("ag_source")
    );
    assert_eq!(
        target.attributes.get("handoffReason").map(String::as_str),
        Some("manual_escalation")
    );

    let cursor = runtime
        .read_cursor_view("100001", "default", "c_agent_handoff", "1")
        .expect("target read cursor should be initialized");
    assert_eq!(cursor.member_id, target.member_id);
    assert_eq!(cursor.read_seq, 0);

    let events = journal.recorded();
    assert_eq!(events.len(), 3);
    assert_eq!(events[0].event_type, "conversation.created");
    assert_eq!(events[0].actor.actor_kind, "agent");
    assert_eq!(events[1].event_type, "conversation.member_joined");
    assert_eq!(events[1].actor.actor_kind, "agent");
    assert_eq!(events[2].event_type, "conversation.member_joined");
    assert_eq!(events[2].actor.actor_kind, "agent");
}

#[test]
fn test_create_agent_handoff_rejects_non_agent_source_kind() {
    let journal = InMemoryJournal::default();
    let runtime = ConversationRuntime::new(journal.clone());

    let create = runtime.create_agent_handoff_with_source_kind(
        CreateAgentHandoffCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_agent_handoff_invalid".into(),
            source_id: "svc_ops".into(),
            target_id: "1".into(),
            target_kind: "user".into(),
            handoff_session_id: "hs_invalid".into(),
            handoff_reason: Some("manual_escalation".into()),
        },
        "system",
    );

    assert!(matches!(create, Err(RuntimeError::PermissionDenied(_))));
    assert!(journal.recorded().is_empty());
}

#[test]
fn test_duplicate_create_agent_handoff_is_idempotent_and_conflicting_retry_is_rejected() {
    let source_journal = InMemoryJournal::default();
    let source_runtime = ConversationRuntime::new(source_journal.clone());

    let first = source_runtime
        .create_agent_handoff_with_source_kind(
            CreateAgentHandoffCommand {
                tenant_id: "100001".into(),
                organization_id: "0".into(),
                conversation_id: "c_agent_handoff_retry".into(),
                source_id: "ag_source".into(),
                target_id: "1".into(),
                target_kind: "user".into(),
                handoff_session_id: "hs_retry".into(),
                handoff_reason: Some("manual_escalation".into()),
            },
            "agent",
        )
        .expect("first agent handoff create should succeed");

    assert_eq!(first.delivery_status.as_ref().unwrap().as_str(), "applied");
    assert_eq!(
        first.proof_version.as_deref(),
        Some("conversation.create.delivery-proof.v1")
    );
    assert_eq!(
        first.request_key.as_deref(),
        Some("6#1000015#agent9#ag_source20#create-agent_handoff21#c_agent_handoff_retry")
    );

    let duplicate = source_runtime
        .create_agent_handoff_with_source_kind(
            CreateAgentHandoffCommand {
                tenant_id: "100001".into(),
                organization_id: "0".into(),
                conversation_id: "c_agent_handoff_retry".into(),
                source_id: "ag_source".into(),
                target_id: "1".into(),
                target_kind: "user".into(),
                handoff_session_id: "hs_retry".into(),
                handoff_reason: Some("manual_escalation".into()),
            },
            "agent",
        )
        .expect("duplicate agent handoff create should replay");

    assert_eq!(duplicate.conversation_id, first.conversation_id);
    assert_eq!(duplicate.event_id, first.event_id);
    assert_eq!(duplicate.request_key, first.request_key);
    assert_eq!(duplicate.proof_version, first.proof_version);
    assert_eq!(
        duplicate.delivery_status.as_ref().unwrap().as_str(),
        "replayed"
    );

    let conflicting_retry = source_runtime.create_agent_handoff_with_source_kind(
        CreateAgentHandoffCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_agent_handoff_retry".into(),
            source_id: "ag_source".into(),
            target_id: "1041".into(),
            target_kind: "user".into(),
            handoff_session_id: "hs_retry".into(),
            handoff_reason: Some("manual_escalation".into()),
        },
        "agent",
    );
    assert!(matches!(conflicting_retry, Err(RuntimeError::Conflict(_))));

    let replay_runtime = ConversationRuntime::new(InMemoryJournal::default());
    for envelope in source_journal.recorded() {
        replay_runtime
            .apply_recovered_envelope(&envelope)
            .expect("agent handoff replay should succeed");
    }

    let recovered_duplicate = replay_runtime
        .create_agent_handoff_with_source_kind(
            CreateAgentHandoffCommand {
                tenant_id: "100001".into(),
                organization_id: "0".into(),
                conversation_id: "c_agent_handoff_retry".into(),
                source_id: "ag_source".into(),
                target_id: "1".into(),
                target_kind: "user".into(),
                handoff_session_id: "hs_retry".into(),
                handoff_reason: Some("manual_escalation".into()),
            },
            "agent",
        )
        .expect("recovered duplicate agent handoff create should replay");
    assert_eq!(recovered_duplicate.event_id, first.event_id);
    assert_eq!(recovered_duplicate.request_key, first.request_key);
    assert_eq!(
        recovered_duplicate
            .delivery_status
            .as_ref()
            .unwrap()
            .as_str(),
        "replayed"
    );

    let events = source_journal.recorded();
    assert_eq!(
        events.len(),
        3,
        "duplicate agent handoff create retry must not append another conversation.created/member_joined pair"
    );
}

#[test]
fn test_agent_handoff_allows_source_and_target_posts() {
    let journal = InMemoryJournal::default();
    let runtime = ConversationRuntime::new(journal.clone());

    runtime
        .create_agent_handoff_with_source_kind(
            CreateAgentHandoffCommand {
                tenant_id: "100001".into(),
                organization_id: "0".into(),
                conversation_id: "c_agent_handoff_post".into(),
                source_id: "ag_source".into(),
                target_id: "1".into(),
                target_kind: "user".into(),
                handoff_session_id: "hs_post".into(),
                handoff_reason: Some("manual_escalation".into()),
            },
            "agent",
        )
        .expect("agent handoff create should succeed");

    let source_post = runtime
        .post_message(PostMessageCommand {
            tenant_id: "100001".into(),
           organization_id: "0".into(),
            conversation_id: "c_agent_handoff_post".into(),
            sender: Sender {
                id: "ag_source".into(),
                kind: "agent".into(),
                member_id: None,
                device_id: None,
                session_id: Some("s_agent".into()),
                metadata: Default::default(),
            },
            client_msg_id: Some("client_handoff_source".into()),
            message_type: MessageType::Standard,
            body: MessageBody {
                summary: Some("source".into()),
                parts: vec![ContentPart::text("source")],
                render_hints: Default::default(),
                reply_to: None,
            },
        })
        .expect("source agent post should succeed");
    assert_eq!(source_post.message_seq, 1);

    let target_post = runtime
        .post_message(PostMessageCommand {
            tenant_id: "100001".into(),
           organization_id: "0".into(),
            conversation_id: "c_agent_handoff_post".into(),
            sender: Sender {
                id: "1".into(),
                kind: "user".into(),
                member_id: None,
                device_id: None,
                session_id: Some("s_target".into()),
                metadata: Default::default(),
            },
            client_msg_id: Some("client_handoff_target".into()),
            message_type: MessageType::Standard,
            body: MessageBody {
                summary: Some("target".into()),
                parts: vec![ContentPart::text("target")],
                render_hints: Default::default(),
                reply_to: None,
            },
        })
        .expect("target post should succeed");
    assert_eq!(target_post.message_seq, 2);

    let events = journal.recorded();
    assert_eq!(
        events
            .iter()
            .filter(|event| event.event_type == "message.posted")
            .count(),
        2
    );
}

#[test]
fn test_agent_handoff_accept_resolve_close_state_machine_and_closed_handoff_rejects_posts() {
    let journal = InMemoryJournal::default();
    let runtime = ConversationRuntime::new(journal.clone());

    runtime
        .create_agent_handoff_with_source_kind(
            CreateAgentHandoffCommand {
                tenant_id: "100001".into(),
                organization_id: "0".into(),
                conversation_id: "c_agent_handoff_state".into(),
                source_id: "ag_source".into(),
                target_id: "1".into(),
                target_kind: "user".into(),
                handoff_session_id: "hs_state".into(),
                handoff_reason: Some("manual_escalation".into()),
            },
            "agent",
        )
        .expect("agent handoff create should succeed");

    let opened = runtime
        .get_agent_handoff_state("100001", "default", "c_agent_handoff_state", "ag_source")
        .expect("source should read handoff state");
    assert_eq!(opened.status, "open");
    assert!(opened.accepted_at.is_none());
    assert!(opened.resolved_at.is_none());
    assert!(opened.closed_at.is_none());

    let accepted = runtime
        .accept_agent_handoff_with_actor_kind(
            AcceptAgentHandoffCommand {
                tenant_id: "100001".into(),
                organization_id: "0".into(),
                conversation_id: "c_agent_handoff_state".into(),
                accepted_by: "1".into(),
            },
            "user",
        )
        .expect("target should accept handoff");
    assert_eq!(accepted.status, "accepted");
    assert_eq!(
        accepted.accepted_by,
        Some(ChangeAgentHandoffStatusView {
            id: "1".into(),
            kind: "user".into(),
        })
    );
    assert!(accepted.accepted_at.is_some());

    let resolved = runtime
        .resolve_agent_handoff_with_actor_kind(
            ResolveAgentHandoffCommand {
                tenant_id: "100001".into(),
                organization_id: "0".into(),
                conversation_id: "c_agent_handoff_state".into(),
                resolved_by: "1".into(),
            },
            "user",
        )
        .expect("target should resolve handoff");
    assert_eq!(resolved.status, "resolved");
    assert_eq!(
        resolved.resolved_by,
        Some(ChangeAgentHandoffStatusView {
            id: "1".into(),
            kind: "user".into(),
        })
    );
    assert!(resolved.resolved_at.is_some());

    let closed = runtime
        .close_agent_handoff_with_actor_kind(
            CloseAgentHandoffCommand {
                tenant_id: "100001".into(),
                organization_id: "0".into(),
                conversation_id: "c_agent_handoff_state".into(),
                closed_by: "ag_source".into(),
            },
            "agent",
        )
        .expect("source should close handoff");
    assert_eq!(closed.status, "closed");
    assert_eq!(
        closed.closed_by,
        Some(ChangeAgentHandoffStatusView {
            id: "ag_source".into(),
            kind: "agent".into(),
        })
    );
    assert!(closed.closed_at.is_some());

    let post_after_close = runtime.post_message(PostMessageCommand {
        tenant_id: "100001".into(),
       organization_id: "0".into(),
        conversation_id: "c_agent_handoff_state".into(),
        sender: Sender {
            id: "1".into(),
            kind: "user".into(),
            member_id: None,
            device_id: None,
            session_id: Some("s_target".into()),
            metadata: Default::default(),
        },
        client_msg_id: Some("client_handoff_closed".into()),
        message_type: MessageType::Standard,
        body: MessageBody {
            summary: Some("should fail".into()),
            parts: vec![ContentPart::text("should fail")],
            render_hints: Default::default(),
            reply_to: None,
        },
    });
    assert!(matches!(post_after_close, Err(RuntimeError::Conflict(_))));

    let events = journal.recorded();
    let status_events: Vec<_> = events
        .iter()
        .filter(|event| event.event_type == "conversation.agent_handoff_status_changed")
        .collect();
    assert_eq!(status_events.len(), 3);
}

#[test]
fn test_agent_handoff_accept_requires_target_and_resolve_requires_accepted_state() {
    let journal = InMemoryJournal::default();
    let runtime = ConversationRuntime::new(journal.clone());

    runtime
        .create_agent_handoff_with_source_kind(
            CreateAgentHandoffCommand {
                tenant_id: "100001".into(),
                organization_id: "0".into(),
                conversation_id: "c_agent_handoff_policy".into(),
                source_id: "ag_source".into(),
                target_id: "1".into(),
                target_kind: "user".into(),
                handoff_session_id: "hs_policy".into(),
                handoff_reason: Some("manual_escalation".into()),
            },
            "agent",
        )
        .expect("agent handoff create should succeed");

    let source_accept = runtime.accept_agent_handoff_with_actor_kind(
        AcceptAgentHandoffCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_agent_handoff_policy".into(),
            accepted_by: "ag_source".into(),
        },
        "agent",
    );
    assert!(matches!(
        source_accept,
        Err(RuntimeError::PermissionDenied(_))
    ));

    let resolve_before_accept = runtime.resolve_agent_handoff_with_actor_kind(
        ResolveAgentHandoffCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_agent_handoff_policy".into(),
            resolved_by: "1".into(),
        },
        "user",
    );
    assert!(matches!(
        resolve_before_accept,
        Err(RuntimeError::Conflict(_))
    ));

    let target_close = runtime
        .close_agent_handoff_with_actor_kind(
            CloseAgentHandoffCommand {
                tenant_id: "100001".into(),
                organization_id: "0".into(),
                conversation_id: "c_agent_handoff_policy".into(),
                closed_by: "1".into(),
            },
            "user",
        )
        .expect("target should be allowed to close open handoff");
    assert_eq!(target_close.status, "closed");
}

#[test]
fn test_create_group_member_joined_event_preserves_system_actor_kind() {
    let journal = InMemoryJournal::default();
    let runtime = ConversationRuntime::new(journal.clone());

    runtime
        .create_conversation_with_creator_kind(
            CreateConversationCommand {
                tenant_id: "100001".into(),
                organization_id: "0".into(),
                conversation_id: "c_group_actor_kind".into(),
                creator_id: "svc_ops".into(),
                conversation_type: "group".into(),
            },
            "system",
        )
        .expect("system actor should be able to create group conversation");

    let member_joined = journal
        .recorded()
        .into_iter()
        .find(|event| {
            event.event_type == "conversation.member_joined"
                && event.aggregate_id == "c_group_actor_kind"
        })
        .expect("creator join event should be recorded");
    assert_eq!(member_joined.actor.actor_id, "svc_ops");
    assert_eq!(member_joined.actor.actor_kind, "system");
}

#[test]
fn test_conversation_membership_lifecycle_tracks_creator_and_member_changes() {
    let journal = InMemoryJournal::default();
    let runtime = ConversationRuntime::new(journal.clone());

    runtime
        .create_conversation(CreateConversationCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_members".into(),
            creator_id: "1".into(),
            conversation_type: "group".into(),
        })
        .expect("create conversation should succeed");

    let members = runtime
        .list_members("100001", "default", "c_members")
        .expect("list members should succeed");
    assert_eq!(members.len(), 1);
    assert_eq!(members[0].member_id, "cm_c_members_user_1");
    assert_eq!(members[0].principal_id, "1");
    assert_eq!(members[0].role, MembershipRole::Owner);
    assert_eq!(members[0].state, MembershipState::Joined);

    let added_member = runtime
        .add_member(AddConversationMemberCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_members".into(),
            principal_id: "1043".into(),
            principal_kind: "user".into(),
            role: MembershipRole::Member,
            invited_by: "1".into(),
        })
        .expect("add member should succeed");

    assert_eq!(added_member.member_id, "cm_c_members_user_1043");
    assert_eq!(added_member.principal_id, "1043");
    assert_eq!(added_member.role, MembershipRole::Member);
    assert_eq!(added_member.state, MembershipState::Joined);
    assert_eq!(added_member.invited_by.as_deref(), Some("1"));

    let members_after_add = runtime
        .list_members("100001", "default", "c_members")
        .expect("list members after add should succeed");
    assert_eq!(members_after_add.len(), 2);

    let removed_member = runtime
        .remove_member(RemoveConversationMemberCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_members".into(),
            member_id: added_member.member_id.clone(),
            removed_by: "1".into(),
        })
        .expect("remove member should succeed");

    assert_eq!(removed_member.member_id, "cm_c_members_user_1043");
    assert_eq!(removed_member.state, MembershipState::Removed);
    assert!(removed_member.removed_at.is_some());

    let members_after_remove = runtime
        .list_members("100001", "default", "c_members")
        .expect("list members after remove should succeed");
    assert_eq!(members_after_remove.len(), 1);
    assert_eq!(
        members_after_remove[0].member_id,
        "cm_c_members_user_1"
    );

    let events = journal.recorded();
    assert_eq!(events.len(), 4);
    assert_eq!(events[1].event_type, "conversation.member_joined");
    assert_eq!(events[2].event_type, "conversation.member_joined");
    assert_eq!(events[3].event_type, "conversation.member_removed");
}

#[test]
fn test_conversation_membership_allows_same_actor_id_with_different_principal_kind() {
    let journal = InMemoryJournal::default();
    let runtime = ConversationRuntime::new(journal);

    runtime
        .create_conversation(CreateConversationCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_members_typed_principal".into(),
            creator_id: "1".into(),
            conversation_type: "group".into(),
        })
        .expect("create conversation should succeed");

    let added_agent = runtime
        .add_member(AddConversationMemberCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_members_typed_principal".into(),
            principal_id: "1".into(),
            principal_kind: "agent".into(),
            role: MembershipRole::Member,
            invited_by: "1".into(),
        })
        .expect(
            "same actor id with different principal kind should be treated as a distinct member",
        );

    assert_eq!(added_agent.principal_id, "1");
    assert_eq!(added_agent.principal_kind, "agent");
    assert_eq!(added_agent.role, MembershipRole::Member);

    let members = runtime
        .list_members("100001", "default", "c_members_typed_principal")
        .expect("list members should succeed");
    let typed_owner_members = members
        .iter()
        .filter(|member| member.principal_id == "1")
        .collect::<Vec<_>>();

    assert_eq!(typed_owner_members.len(), 2);
    assert!(
        typed_owner_members.iter().any(|member| {
            member.principal_kind == "user" && member.role == MembershipRole::Owner
        })
    );
    assert!(typed_owner_members.iter().any(|member| {
        member.principal_kind == "agent" && member.role == MembershipRole::Member
    }));
}

#[test]
fn test_read_cursor_advances_monotonically_for_active_member() {
    let journal = InMemoryJournal::default();
    let runtime = ConversationRuntime::new(journal.clone());

    runtime
        .create_conversation(CreateConversationCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_cursor".into(),
            creator_id: "1".into(),
            conversation_type: "group".into(),
        })
        .expect("create conversation should succeed");

    runtime
        .post_message(PostMessageCommand {
            tenant_id: "100001".into(),
           organization_id: "0".into(),
            conversation_id: "c_cursor".into(),
            sender: Sender {
                id: "1".into(),
                kind: "user".into(),
                member_id: None,
                device_id: None,
                session_id: Some("s_owner".into()),
                metadata: Default::default(),
            },
            client_msg_id: Some("client_1".into()),
            message_type: MessageType::Standard,
            body: MessageBody {
                summary: Some("one".into()),
                parts: vec![ContentPart::text("one")],
                render_hints: Default::default(),
                reply_to: None,
            },
        })
        .expect("first message should succeed");

    runtime
        .post_message(PostMessageCommand {
            tenant_id: "100001".into(),
           organization_id: "0".into(),
            conversation_id: "c_cursor".into(),
            sender: Sender {
                id: "1".into(),
                kind: "user".into(),
                member_id: None,
                device_id: None,
                session_id: Some("s_owner".into()),
                metadata: Default::default(),
            },
            client_msg_id: Some("client_2".into()),
            message_type: MessageType::Standard,
            body: MessageBody {
                summary: Some("two".into()),
                parts: vec![ContentPart::text("two")],
                render_hints: Default::default(),
                reply_to: None,
            },
        })
        .expect("second message should succeed");

    let cursor = runtime
        .update_read_cursor(UpdateReadCursorCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_cursor".into(),
            principal_id: "1".into(),
            read_seq: 1,
            last_read_message_id: Some("msg_c_cursor_1".into()),
        })
        .expect("read cursor update should succeed");

    assert_eq!(cursor.member_id, "cm_c_cursor_user_1");
    assert_eq!(cursor.read_seq, 1);
    assert_eq!(
        cursor.last_read_message_id.as_deref(),
        Some("msg_c_cursor_1")
    );

    let regressed = runtime
        .update_read_cursor(UpdateReadCursorCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_cursor".into(),
            principal_id: "1".into(),
            read_seq: 0,
            last_read_message_id: Some("msg_c_cursor_0".into()),
        })
        .expect("regressed read cursor update should be idempotent");

    assert_eq!(regressed.read_seq, 1);
    assert_eq!(
        regressed.last_read_message_id.as_deref(),
        Some("msg_c_cursor_1")
    );

    let advanced = runtime
        .update_read_cursor(UpdateReadCursorCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_cursor".into(),
            principal_id: "1".into(),
            read_seq: 2,
            last_read_message_id: Some("msg_c_cursor_2".into()),
        })
        .expect("advanced read cursor update should succeed");

    assert_eq!(advanced.read_seq, 2);
    assert_eq!(
        advanced.last_read_message_id.as_deref(),
        Some("msg_c_cursor_2")
    );

    let events = journal.recorded();
    let read_events: Vec<_> = events
        .iter()
        .filter(|event| event.event_type == "conversation.read_cursor_updated")
        .collect();
    assert_eq!(read_events.len(), 2);
    assert_eq!(read_events[0].ordering_seq, 1);
    assert_eq!(read_events[1].ordering_seq, 2);
}

#[test]
fn test_read_cursor_unread_count_excludes_messages_sent_by_current_principal() {
    let journal = InMemoryJournal::default();
    let runtime = ConversationRuntime::new(journal);

    runtime
        .create_conversation(CreateConversationCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_received_unread_only".into(),
            creator_id: "1".into(),
            conversation_type: "group".into(),
        })
        .expect("create conversation should succeed");
    runtime
        .add_member(AddConversationMemberCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_received_unread_only".into(),
            principal_id: "1054".into(),
            principal_kind: "user".into(),
            role: MembershipRole::Member,
            invited_by: "1".into(),
        })
        .expect("add friend member should succeed");

    runtime
        .post_message(PostMessageCommand {
            tenant_id: "100001".into(),
           organization_id: "0".into(),
            conversation_id: "c_received_unread_only".into(),
            sender: Sender {
                id: "1".into(),
                kind: "user".into(),
                member_id: None,
                device_id: None,
                session_id: Some("s_owner".into()),
                metadata: Default::default(),
            },
            client_msg_id: Some("client_received_unread_owner_1".into()),
            message_type: MessageType::Standard,
            body: MessageBody {
                summary: Some("owner note".into()),
                parts: vec![ContentPart::text("owner note")],
                render_hints: Default::default(),
                reply_to: None,
            },
        })
        .expect("owner message should succeed");

    let owner_cursor_after_own_message = runtime
        .read_cursor_view("100001", "default", "c_received_unread_only", "1")
        .expect("owner cursor view should succeed");
    assert_eq!(
        owner_cursor_after_own_message.unread_count, 0,
        "a principal's own sent message must not become unread for that same principal"
    );
    let friend_cursor_after_owner_message = runtime
        .read_cursor_view("100001", "default", "c_received_unread_only", "1054")
        .expect("friend cursor view should succeed");
    assert_eq!(
        friend_cursor_after_owner_message.unread_count, 1,
        "the same message must remain unread for receiving members"
    );

    runtime
        .post_message(PostMessageCommand {
            tenant_id: "100001".into(),
           organization_id: "0".into(),
            conversation_id: "c_received_unread_only".into(),
            sender: Sender {
                id: "1054".into(),
                kind: "user".into(),
                member_id: None,
                device_id: None,
                session_id: Some("s_friend".into()),
                metadata: Default::default(),
            },
            client_msg_id: Some("client_received_unread_friend_1".into()),
            message_type: MessageType::Standard,
            body: MessageBody {
                summary: Some("friend reply".into()),
                parts: vec![ContentPart::text("friend reply")],
                render_hints: Default::default(),
                reply_to: None,
            },
        })
        .expect("friend message should succeed");

    let owner_cursor_after_friend_message = runtime
        .read_cursor_view("100001", "default", "c_received_unread_only", "1")
        .expect("owner cursor view should still succeed");
    assert_eq!(
        owner_cursor_after_friend_message.unread_count, 1,
        "only the friend's received message should be unread for the owner"
    );
    let friend_cursor_after_reply = runtime
        .read_cursor_view("100001", "default", "c_received_unread_only", "1054")
        .expect("friend cursor view should still succeed");
    assert_eq!(
        friend_cursor_after_reply.unread_count, 1,
        "the friend should keep the owner's received message unread while excluding their own reply"
    );
}

#[test]
fn test_read_cursor_rejects_actor_kind_mismatch_against_member_principal_kind() {
    let journal = InMemoryJournal::default();
    let runtime = ConversationRuntime::new(journal);

    runtime
        .create_conversation(CreateConversationCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_cursor_actor_kind_guard".into(),
            creator_id: "1".into(),
            conversation_type: "group".into(),
        })
        .expect("create conversation should succeed");

    runtime
        .post_message(PostMessageCommand {
            tenant_id: "100001".into(),
           organization_id: "0".into(),
            conversation_id: "c_cursor_actor_kind_guard".into(),
            sender: Sender {
                id: "1".into(),
                kind: "user".into(),
                member_id: None,
                device_id: None,
                session_id: Some("s_owner".into()),
                metadata: Default::default(),
            },
            client_msg_id: Some("client_cursor_actor_kind_guard".into()),
            message_type: MessageType::Standard,
            body: MessageBody {
                summary: Some("one".into()),
                parts: vec![ContentPart::text("one")],
                render_hints: Default::default(),
                reply_to: None,
            },
        })
        .expect("message should succeed");

    let update_attempt = runtime.update_read_cursor_with_actor_kind(
        UpdateReadCursorCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_cursor_actor_kind_guard".into(),
            principal_id: "1".into(),
            read_seq: 1,
            last_read_message_id: Some("msg_c_cursor_actor_kind_guard_1".into()),
        },
        "agent",
    );

    assert!(matches!(
        update_attempt,
        Err(RuntimeError::PermissionDenied(_))
    ));
}

#[test]
fn test_conversation_bound_write_capability_gate_rejects_actor_kind_mismatch() {
    let journal = InMemoryJournal::default();
    let runtime = ConversationRuntime::new(journal);

    runtime
        .create_conversation(CreateConversationCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_write_capability_actor_kind_guard".into(),
            creator_id: "1".into(),
            conversation_type: "group".into(),
        })
        .expect("create conversation should succeed");

    let gate_attempt = runtime.ensure_conversation_bound_write_allowed_with_actor_kind("100001", "default", "c_write_capability_actor_kind_guard",
        "1",
        "agent",
        "stream.append",
    );

    assert!(matches!(
        gate_attempt,
        Err(RuntimeError::PermissionDenied(_))
    ));
}

#[test]
fn test_recovered_conversation_policy_capability_flags_disable_pin_after_replay() {
    let source_journal = InMemoryJournal::default();
    let source_runtime = ConversationRuntime::new(source_journal.clone());

    source_runtime
        .create_conversation(CreateConversationCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_policy_replay".into(),
            creator_id: "1".into(),
            conversation_type: "group".into(),
        })
        .expect("create conversation should succeed");

    let posted = source_runtime
        .post_message(PostMessageCommand {
            tenant_id: "100001".into(),
           organization_id: "0".into(),
            conversation_id: "c_policy_replay".into(),
            sender: Sender {
                id: "1".into(),
                kind: "user".into(),
                member_id: None,
                device_id: Some("d_owner".into()),
                session_id: Some("s_owner".into()),
                metadata: Default::default(),
            },
            client_msg_id: Some("client_policy_replay".into()),
            message_type: MessageType::Standard,
            body: MessageBody {
                summary: Some("policy target".into()),
                parts: vec![ContentPart::text("policy target")],
                render_hints: Default::default(),
                reply_to: None,
            },
        })
        .expect("post message should succeed");

    let replay_journal = InMemoryJournal::default();
    let replay_runtime = ConversationRuntime::new(replay_journal);
    let policy_event = CommitEnvelope {
        event_id: "evt_c_policy_replay_policy_1".into(),
        tenant_id: "100001".into(),
        organization_id: "0".into(),
        event_type: "conversation.policy_applied".into(),
        event_version: 1,
        aggregate_type: AggregateType::Conversation,
        aggregate_id: "c_policy_replay".into(),
        scope_type: "conversation".into(),
        scope_id: "c_policy_replay".into(),
        ordering_key: CommitEnvelope::ordering_key("100001", "c_policy_replay"),
        ordering_seq: 1,
        causation_id: None,
        correlation_id: None,
        idempotency_key: None,
        actor: EventActor {
            actor_id: "1".into(),
            actor_kind: "user".into(),
            actor_session_id: None,
        },
        occurred_at: "2026-04-10T00:00:00.000Z".into(),
        committed_at: "2026-04-10T00:00:00.000Z".into(),
        payload_schema: Some("conversation.policy_applied.v1".into()),
        payload: serde_json::json!({
            "conversationId": "c_policy_replay",
            "policyVersion": "group.policy.v1",
            "capabilityFlags": ["message.reaction"],
            "historyVisibility": "joined",
            "retentionPolicyRef": "tenant.standard"
        })
        .to_string(),
        retention_class: "standard".into(),
        audit_class: "default".into(),
    };

    for envelope in source_journal.recorded() {
        replay_runtime
            .apply_recovered_envelope(&envelope)
            .expect("replay should succeed");
    }
    replay_runtime
        .apply_recovered_envelope(&policy_event)
        .expect("policy replay should succeed");

    let reaction = replay_runtime
        .add_message_reaction(AddMessageReactionCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            message_id: posted.message_id.clone(),
            reaction_key: "thumbs_up".into(),
            reacted_by: Sender {
                id: "1".into(),
                kind: "user".into(),
                member_id: None,
                device_id: Some("d_owner".into()),
                session_id: Some("s_owner".into()),
                metadata: Default::default(),
            },
        })
        .expect("reaction should stay enabled");
    assert!(reaction.changed);

    let denied_pin = replay_runtime.pin_message(PinMessageCommand {
        tenant_id: "100001".into(),
        organization_id: "0".into(),
        message_id: posted.message_id,
        pinned_by: Sender {
            id: "1".into(),
            kind: "user".into(),
            member_id: None,
            device_id: Some("d_owner".into()),
            session_id: Some("s_owner".into()),
            metadata: Default::default(),
        },
    });
    assert!(matches!(denied_pin, Err(RuntimeError::PermissionDenied(_))));
}

#[test]
fn test_applied_retention_policy_ref_propagates_to_subsequent_message_commit_envelopes() {
    let source_journal = InMemoryJournal::default();
    let runtime = ConversationRuntime::new(source_journal.clone());

    runtime
        .create_conversation(CreateConversationCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_retention_policy".into(),
            creator_id: "1".into(),
            conversation_type: "group".into(),
        })
        .expect("create conversation should succeed");

    runtime
        .apply_conversation_policy_with_actor_kind(
            ApplyConversationPolicyCommand {
                tenant_id: "100001".into(),
                organization_id: "0".into(),
                conversation_id: "c_retention_policy".into(),
                applied_by: "1".into(),
                policy: ConversationPolicy {
                    policy_version: "group.policy.v1".into(),
                    capability_flags: None,
                    history_visibility: "joined".into(),
                    retention_policy_ref: "tenant.compliance".into(),
                },
            },
            "user",
        )
        .expect("apply conversation policy should succeed");

    runtime
        .post_message(PostMessageCommand {
            tenant_id: "100001".into(),
           organization_id: "0".into(),
            conversation_id: "c_retention_policy".into(),
            sender: Sender {
                id: "1".into(),
                kind: "user".into(),
                member_id: None,
                device_id: Some("d_owner".into()),
                session_id: Some("s_owner".into()),
                metadata: Default::default(),
            },
            client_msg_id: Some("client_retention_policy_1".into()),
            message_type: MessageType::Standard,
            body: MessageBody {
                summary: Some("retained".into()),
                parts: vec![ContentPart::text("retained")],
                render_hints: Default::default(),
                reply_to: None,
            },
        })
        .expect("post message should succeed");

    let recorded = source_journal.recorded();
    let policy_event = recorded
        .iter()
        .find(|event| event.event_type == "conversation.policy_applied")
        .expect("policy event should exist");
    assert_eq!(policy_event.retention_class, "compliance");

    let posted_event = recorded
        .iter()
        .find(|event| {
            event.event_type == "message.posted" && event.aggregate_id == "c_retention_policy"
        })
        .expect("posted event should exist");
    assert_eq!(posted_event.retention_class, "compliance");

    let replay_journal = InMemoryJournal::default();
    let replay_runtime = ConversationRuntime::new(replay_journal.clone());
    for envelope in recorded {
        replay_runtime
            .apply_recovered_envelope(&envelope)
            .expect("replay should succeed");
    }

    replay_runtime
        .post_message(PostMessageCommand {
            tenant_id: "100001".into(),
           organization_id: "0".into(),
            conversation_id: "c_retention_policy".into(),
            sender: Sender {
                id: "1".into(),
                kind: "user".into(),
                member_id: None,
                device_id: Some("d_owner".into()),
                session_id: Some("s_owner_replay".into()),
                metadata: Default::default(),
            },
            client_msg_id: Some("client_retention_policy_2".into()),
            message_type: MessageType::Standard,
            body: MessageBody {
                summary: Some("retained after replay".into()),
                parts: vec![ContentPart::text("retained after replay")],
                render_hints: Default::default(),
                reply_to: None,
            },
        })
        .expect("post after replay should succeed");

    let replay_posted_event = replay_journal
        .recorded()
        .into_iter()
        .find(|event| event.event_type == "message.posted")
        .expect("replay posted event should exist");
    assert_eq!(replay_posted_event.retention_class, "compliance");
}

#[test]
fn test_system_channel_requires_dedicated_publish_command_and_allows_only_publisher() {
    let journal = InMemoryJournal::default();
    let runtime = ConversationRuntime::new(journal.clone());

    runtime
        .create_system_channel_with_requester_kind(
            CreateSystemChannelCommand {
                tenant_id: "100001".into(),
                organization_id: "0".into(),
                conversation_id: "c_system_channel_post".into(),
                requester_id: "svc_ops".into(),
                subscriber_id: "1".into(),
            },
            "system",
        )
        .expect("system channel create should succeed");

    let subscriber_post = runtime.post_message(PostMessageCommand {
        tenant_id: "100001".into(),
       organization_id: "0".into(),
        conversation_id: "c_system_channel_post".into(),
        sender: Sender {
            id: "1".into(),
            kind: "user".into(),
            member_id: None,
            device_id: None,
            session_id: Some("s_subscriber".into()),
            metadata: Default::default(),
        },
        client_msg_id: Some("client_subscriber_post".into()),
        message_type: MessageType::Standard,
        body: MessageBody {
            summary: Some("should fail".into()),
            parts: vec![ContentPart::text("should fail")],
            render_hints: Default::default(),
            reply_to: None,
        },
    });
    assert!(matches!(
        subscriber_post,
        Err(RuntimeError::PermissionDenied(_))
    ));

    let system_post = runtime.post_message(PostMessageCommand {
        tenant_id: "100001".into(),
       organization_id: "0".into(),
        conversation_id: "c_system_channel_post".into(),
        sender: Sender {
            id: "svc_ops".into(),
            kind: "system".into(),
            member_id: None,
            device_id: None,
            session_id: Some("s_system".into()),
            metadata: Default::default(),
        },
        client_msg_id: Some("client_system_post".into()),
        message_type: MessageType::Standard,
        body: MessageBody {
            summary: Some("system notice".into()),
            parts: vec![ContentPart::text("system notice")],
            render_hints: Default::default(),
            reply_to: None,
        },
    });
    assert!(matches!(
        system_post,
        Err(RuntimeError::PermissionDenied(_))
    ));

    let subscriber_publish =
        runtime.publish_system_channel_message(PublishSystemChannelMessageCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_system_channel_post".into(),
            publisher: Sender {
                id: "1".into(),
                kind: "user".into(),
                member_id: None,
                device_id: None,
                session_id: Some("s_subscriber".into()),
                metadata: Default::default(),
            },
            client_msg_id: Some("client_subscriber_publish".into()),
            body: MessageBody {
                summary: Some("should fail".into()),
                parts: vec![ContentPart::text("should fail")],
                render_hints: Default::default(),
                reply_to: None,
            },
        });
    assert!(matches!(
        subscriber_publish,
        Err(RuntimeError::PermissionDenied(_))
    ));

    let system_publish = runtime
        .publish_system_channel_message(PublishSystemChannelMessageCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_system_channel_post".into(),
            publisher: Sender {
                id: "svc_ops".into(),
                kind: "system".into(),
                member_id: None,
                device_id: None,
                session_id: Some("s_system".into()),
                metadata: Default::default(),
            },
            client_msg_id: Some("client_system_publish".into()),
            body: MessageBody {
                summary: Some("system notice".into()),
                parts: vec![ContentPart::text("system notice")],
                render_hints: Default::default(),
                reply_to: None,
            },
        })
        .expect("system publisher dedicated publish should succeed");

    assert_eq!(system_publish.message_seq, 1);
    let events = journal.recorded();
    assert_eq!(
        events
            .iter()
            .filter(|event| event.event_type == "message.posted")
            .count(),
        1
    );
}

#[test]
fn test_read_cursor_event_preserves_agent_actor_kind() {
    let journal = InMemoryJournal::default();
    let runtime = ConversationRuntime::new(journal.clone());

    runtime
        .create_agent_dialog_with_requester_kind(
            CreateAgentDialogCommand {
                tenant_id: "100001".into(),
                organization_id: "0".into(),
                conversation_id: "c_agent_cursor".into(),
                requester_id: "1055".into(),
                agent_id: "agent.demo".into(),
            },
            "user",
        )
        .expect("agent dialog create should succeed");

    runtime
        .post_message(PostMessageCommand {
            tenant_id: "100001".into(),
           organization_id: "0".into(),
            conversation_id: "c_agent_cursor".into(),
            sender: Sender {
                id: "1055".into(),
                kind: "user".into(),
                member_id: None,
                device_id: None,
                session_id: Some("s_requester".into()),
                metadata: Default::default(),
            },
            client_msg_id: Some("client_agent_cursor".into()),
            message_type: MessageType::Standard,
            body: MessageBody {
                summary: Some("question".into()),
                parts: vec![ContentPart::text("question")],
                render_hints: Default::default(),
                reply_to: None,
            },
        })
        .expect("message should succeed");

    runtime
        .update_read_cursor(UpdateReadCursorCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_agent_cursor".into(),
            principal_id: "agent.demo".into(),
            read_seq: 1,
            last_read_message_id: Some("msg_c_agent_cursor_1".into()),
        })
        .expect("agent read cursor update should succeed");

    let read_cursor_event = journal
        .recorded()
        .into_iter()
        .find(|event| {
            event.event_type == "conversation.read_cursor_updated"
                && event.aggregate_id == "c_agent_cursor"
                && event.ordering_seq == 1
        })
        .expect("read cursor update event should exist");
    assert_eq!(read_cursor_event.actor.actor_id, "agent.demo");
    assert_eq!(read_cursor_event.actor.actor_kind, "agent");
}

#[test]
fn test_edit_and_recall_message_emit_mutation_events_without_changing_sequence() {
    let journal = InMemoryJournal::default();
    let runtime = ConversationRuntime::new(journal.clone());

    runtime
        .create_conversation(CreateConversationCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_mutation".into(),
            creator_id: "1".into(),
            conversation_type: "group".into(),
        })
        .expect("create conversation should succeed");

    let posted = runtime
        .post_message(PostMessageCommand {
            tenant_id: "100001".into(),
           organization_id: "0".into(),
            conversation_id: "c_mutation".into(),
            sender: Sender {
                id: "1".into(),
                kind: "user".into(),
                member_id: None,
                device_id: Some("d_demo".into()),
                session_id: Some("s_demo".into()),
                metadata: Default::default(),
            },
            client_msg_id: Some("client_mutation".into()),
            message_type: MessageType::Standard,
            body: MessageBody {
                summary: Some("hello".into()),
                parts: vec![ContentPart::text("hello")],
                render_hints: Default::default(),
                reply_to: None,
            },
        })
        .expect("post message should succeed");

    let edited = runtime
        .edit_message(EditMessageCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            message_id: posted.message_id.clone(),
            editor: Sender {
                id: "1".into(),
                kind: "user".into(),
                member_id: None,
                device_id: Some("d_demo".into()),
                session_id: Some("s_demo".into()),
                metadata: Default::default(),
            },
            body: MessageBody {
                summary: Some("edited".into()),
                parts: vec![ContentPart::text("edited")],
                render_hints: Default::default(),
                reply_to: None,
            },
        })
        .expect("edit message should succeed");

    let recalled = runtime
        .recall_message(RecallMessageCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            message_id: posted.message_id.clone(),
            recalled_by: Sender {
                id: "1".into(),
                kind: "user".into(),
                member_id: None,
                device_id: Some("d_demo".into()),
                session_id: Some("s_demo".into()),
                metadata: Default::default(),
            },
        })
        .expect("recall message should succeed");

    assert_eq!(edited.message_id, posted.message_id);
    assert_eq!(edited.message_seq, 1);
    assert_eq!(recalled.message_id, posted.message_id);
    assert_eq!(recalled.message_seq, 1);

    let events = journal.recorded();
    assert_eq!(events.len(), 5);
    assert_eq!(events[2].event_type, "message.posted");
    assert_eq!(events[3].event_type, "message.edited");
    assert_eq!(events[3].ordering_seq, 1);
    assert_eq!(events[4].event_type, "message.recalled");
    assert_eq!(events[4].ordering_seq, 1);
}

#[test]
fn test_generated_message_id_stays_within_runtime_contract_for_max_length_conversation_ids() {
    let journal = InMemoryJournal::default();
    let runtime = ConversationRuntime::new(journal);
    let conversation_id = "c".repeat(256);

    runtime
        .create_conversation(CreateConversationCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: conversation_id.clone(),
            creator_id: "1".into(),
            conversation_type: "group".into(),
        })
        .expect("create conversation should succeed");

    let posted = runtime
        .post_message(PostMessageCommand {
            tenant_id: "100001".into(),
           organization_id: "0".into(),
            conversation_id: conversation_id.clone(),
            sender: Sender {
                id: "1".into(),
                kind: "user".into(),
                member_id: None,
                device_id: Some("d_demo".into()),
                session_id: Some("s_demo".into()),
                metadata: Default::default(),
            },
            client_msg_id: Some("client_long_message_id".into()),
            message_type: MessageType::Standard,
            body: MessageBody {
                summary: Some("hello".into()),
                parts: vec![ContentPart::text("hello")],
                render_hints: Default::default(),
                reply_to: None,
            },
        })
        .expect("post message should succeed");

    assert!(
        posted.message_id.len() <= 256,
        "generated message id must stay within runtime contract: {}",
        posted.message_id.len()
    );

    let edited = runtime
        .edit_message(EditMessageCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            message_id: posted.message_id.clone(),
            editor: Sender {
                id: "1".into(),
                kind: "user".into(),
                member_id: None,
                device_id: Some("d_demo".into()),
                session_id: Some("s_demo".into()),
                metadata: Default::default(),
            },
            body: MessageBody {
                summary: Some("edited".into()),
                parts: vec![ContentPart::text("edited")],
                render_hints: Default::default(),
                reply_to: None,
            },
        })
        .expect("generated message id should remain editable");

    let recalled = runtime
        .recall_message(RecallMessageCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            message_id: posted.message_id.clone(),
            recalled_by: Sender {
                id: "1".into(),
                kind: "user".into(),
                member_id: None,
                device_id: Some("d_demo".into()),
                session_id: Some("s_demo".into()),
                metadata: Default::default(),
            },
        })
        .expect("generated message id should remain recallable");

    assert_eq!(edited.message_id, posted.message_id);
    assert_eq!(recalled.message_id, posted.message_id);
}

#[test]
fn test_non_member_cannot_post_message_to_conversation() {
    let journal = InMemoryJournal::default();
    let runtime = ConversationRuntime::new(journal);

    runtime
        .create_conversation(CreateConversationCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_private".into(),
            creator_id: "1".into(),
            conversation_type: "group".into(),
        })
        .expect("create conversation should succeed");

    let result = runtime.post_message(PostMessageCommand {
        tenant_id: "100001".into(),
       organization_id: "0".into(),
        conversation_id: "c_private".into(),
        sender: Sender {
            id: "1056".into(),
            kind: "user".into(),
            member_id: None,
            device_id: Some("d_intruder".into()),
            session_id: Some("s_intruder".into()),
            metadata: Default::default(),
        },
        client_msg_id: Some("client_intruder".into()),
        message_type: MessageType::Standard,
        body: MessageBody {
            summary: Some("unauthorized".into()),
            parts: vec![ContentPart::text("unauthorized")],
            render_hints: Default::default(),
            reply_to: None,
        },
    });

    assert!(matches!(result, Err(RuntimeError::PermissionDenied(_))));
}

#[test]
fn test_non_member_cannot_edit_or_recall_message() {
    let journal = InMemoryJournal::default();
    let runtime = ConversationRuntime::new(journal);

    runtime
        .create_conversation(CreateConversationCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_private_mutation".into(),
            creator_id: "1".into(),
            conversation_type: "group".into(),
        })
        .expect("create conversation should succeed");

    let posted = runtime
        .post_message(PostMessageCommand {
            tenant_id: "100001".into(),
           organization_id: "0".into(),
            conversation_id: "c_private_mutation".into(),
            sender: Sender {
                id: "1".into(),
                kind: "user".into(),
                member_id: None,
                device_id: Some("d_owner".into()),
                session_id: Some("s_owner".into()),
                metadata: Default::default(),
            },
            client_msg_id: Some("client_owner".into()),
            message_type: MessageType::Standard,
            body: MessageBody {
                summary: Some("hello".into()),
                parts: vec![ContentPart::text("hello")],
                render_hints: Default::default(),
                reply_to: None,
            },
        })
        .expect("owner message should succeed");

    let edit = runtime.edit_message(EditMessageCommand {
        tenant_id: "100001".into(),
        organization_id: "0".into(),
        message_id: posted.message_id.clone(),
        editor: Sender {
            id: "1056".into(),
            kind: "user".into(),
            member_id: None,
            device_id: Some("d_intruder".into()),
            session_id: Some("s_intruder".into()),
            metadata: Default::default(),
        },
        body: MessageBody {
            summary: Some("edited by intruder".into()),
            parts: vec![ContentPart::text("edited by intruder")],
            render_hints: Default::default(),
            reply_to: None,
        },
    });
    assert!(matches!(edit, Err(RuntimeError::PermissionDenied(_))));

    let recall = runtime.recall_message(RecallMessageCommand {
        tenant_id: "100001".into(),
        organization_id: "0".into(),
        message_id: posted.message_id,
        recalled_by: Sender {
            id: "1056".into(),
            kind: "user".into(),
            member_id: None,
            device_id: Some("d_intruder".into()),
            session_id: Some("s_intruder".into()),
            metadata: Default::default(),
        },
    });
    assert!(matches!(recall, Err(RuntimeError::PermissionDenied(_))));
}

#[test]
fn test_member_cannot_edit_or_recall_other_members_message() {
    let journal = InMemoryJournal::default();
    let runtime = ConversationRuntime::new(journal);

    runtime
        .create_conversation(CreateConversationCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_group_mutation".into(),
            creator_id: "1".into(),
            conversation_type: "group".into(),
        })
        .expect("create conversation should succeed");

    runtime
        .add_member(AddConversationMemberCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_group_mutation".into(),
            principal_id: "1043".into(),
            principal_kind: "user".into(),
            role: MembershipRole::Member,
            invited_by: "1".into(),
        })
        .expect("add member should succeed");

    let posted = runtime
        .post_message(PostMessageCommand {
            tenant_id: "100001".into(),
           organization_id: "0".into(),
            conversation_id: "c_group_mutation".into(),
            sender: Sender {
                id: "1".into(),
                kind: "user".into(),
                member_id: None,
                device_id: Some("d_owner".into()),
                session_id: Some("s_owner".into()),
                metadata: Default::default(),
            },
            client_msg_id: Some("client_owner".into()),
            message_type: MessageType::Standard,
            body: MessageBody {
                summary: Some("hello".into()),
                parts: vec![ContentPart::text("hello")],
                render_hints: Default::default(),
                reply_to: None,
            },
        })
        .expect("owner message should succeed");

    let edit = runtime.edit_message(EditMessageCommand {
        tenant_id: "100001".into(),
        organization_id: "0".into(),
        message_id: posted.message_id.clone(),
        editor: Sender {
            id: "1043".into(),
            kind: "user".into(),
            member_id: None,
            device_id: Some("d_member".into()),
            session_id: Some("s_member".into()),
            metadata: Default::default(),
        },
        body: MessageBody {
            summary: Some("edited by member".into()),
            parts: vec![ContentPart::text("edited by member")],
            render_hints: Default::default(),
            reply_to: None,
        },
    });
    assert!(matches!(edit, Err(RuntimeError::PermissionDenied(_))));

    let recall = runtime.recall_message(RecallMessageCommand {
        tenant_id: "100001".into(),
        organization_id: "0".into(),
        message_id: posted.message_id,
        recalled_by: Sender {
            id: "1043".into(),
            kind: "user".into(),
            member_id: None,
            device_id: Some("d_member".into()),
            session_id: Some("s_member".into()),
            metadata: Default::default(),
        },
    });
    assert!(matches!(recall, Err(RuntimeError::PermissionDenied(_))));
}

#[test]
fn test_group_owner_can_recall_but_not_edit_other_members_message() {
    let journal = InMemoryJournal::default();
    let runtime = ConversationRuntime::new(journal);

    runtime
        .create_conversation(CreateConversationCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_group_owner_override".into(),
            creator_id: "1".into(),
            conversation_type: "group".into(),
        })
        .expect("create conversation should succeed");

    runtime
        .add_member(AddConversationMemberCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_group_owner_override".into(),
            principal_id: "1043".into(),
            principal_kind: "user".into(),
            role: MembershipRole::Member,
            invited_by: "1".into(),
        })
        .expect("add member should succeed");

    let posted = runtime
        .post_message(PostMessageCommand {
            tenant_id: "100001".into(),
           organization_id: "0".into(),
            conversation_id: "c_group_owner_override".into(),
            sender: Sender {
                id: "1043".into(),
                kind: "user".into(),
                member_id: None,
                device_id: Some("d_member".into()),
                session_id: Some("s_member".into()),
                metadata: Default::default(),
            },
            client_msg_id: Some("client_member".into()),
            message_type: MessageType::Standard,
            body: MessageBody {
                summary: Some("member hello".into()),
                parts: vec![ContentPart::text("member hello")],
                render_hints: Default::default(),
                reply_to: None,
            },
        })
        .expect("member message should succeed");

    let edit = runtime.edit_message(EditMessageCommand {
        tenant_id: "100001".into(),
        organization_id: "0".into(),
        message_id: posted.message_id.clone(),
        editor: Sender {
            id: "1".into(),
            kind: "user".into(),
            member_id: None,
            device_id: Some("d_owner".into()),
            session_id: Some("s_owner".into()),
            metadata: Default::default(),
        },
        body: MessageBody {
            summary: Some("owner edit".into()),
            parts: vec![ContentPart::text("owner edit")],
            render_hints: Default::default(),
            reply_to: None,
        },
    });
    assert!(matches!(edit, Err(RuntimeError::PermissionDenied(_))));

    let recall = runtime
        .recall_message(RecallMessageCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            message_id: posted.message_id,
            recalled_by: Sender {
                id: "1".into(),
                kind: "user".into(),
                member_id: None,
                device_id: Some("d_owner".into()),
                session_id: Some("s_owner".into()),
                metadata: Default::default(),
            },
        })
        .expect("owner should be able to recall member message in group conversation");
    assert_eq!(recall.conversation_id, "c_group_owner_override");
    assert_eq!(recall.message_seq, 1);
}

#[test]
fn test_direct_conversation_owner_cannot_recall_other_members_message() {
    let journal = InMemoryJournal::default();
    let runtime = ConversationRuntime::new(journal);

    runtime
        .create_conversation(CreateConversationCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_direct_mutation".into(),
            creator_id: "1".into(),
            conversation_type: "direct".into(),
        })
        .expect("create conversation should succeed");

    runtime
        .add_member(AddConversationMemberCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_direct_mutation".into(),
            principal_id: "1057".into(),
            principal_kind: "user".into(),
            role: MembershipRole::Member,
            invited_by: "1".into(),
        })
        .expect("add peer should succeed");

    let posted = runtime
        .post_message(PostMessageCommand {
            tenant_id: "100001".into(),
           organization_id: "0".into(),
            conversation_id: "c_direct_mutation".into(),
            sender: Sender {
                id: "1057".into(),
                kind: "user".into(),
                member_id: None,
                device_id: Some("d_peer".into()),
                session_id: Some("s_peer".into()),
                metadata: Default::default(),
            },
            client_msg_id: Some("client_peer".into()),
            message_type: MessageType::Standard,
            body: MessageBody {
                summary: Some("peer hello".into()),
                parts: vec![ContentPart::text("peer hello")],
                render_hints: Default::default(),
                reply_to: None,
            },
        })
        .expect("peer message should succeed");

    let recall = runtime.recall_message(RecallMessageCommand {
        tenant_id: "100001".into(),
        organization_id: "0".into(),
        message_id: posted.message_id,
        recalled_by: Sender {
            id: "1".into(),
            kind: "user".into(),
            member_id: None,
            device_id: Some("d_owner".into()),
            session_id: Some("s_owner".into()),
            metadata: Default::default(),
        },
    });
    assert!(matches!(recall, Err(RuntimeError::PermissionDenied(_))));
}

#[test]
fn test_group_member_cannot_manage_other_members() {
    let journal = InMemoryJournal::default();
    let runtime = ConversationRuntime::new(journal);

    runtime
        .create_conversation(CreateConversationCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_group_member_governance".into(),
            creator_id: "1".into(),
            conversation_type: "group".into(),
        })
        .expect("create conversation should succeed");

    runtime
        .add_member(AddConversationMemberCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_group_member_governance".into(),
            principal_id: "1043".into(),
            principal_kind: "user".into(),
            role: MembershipRole::Member,
            invited_by: "1".into(),
        })
        .expect("owner should be able to add regular member");

    let target = runtime
        .add_member(AddConversationMemberCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_group_member_governance".into(),
            principal_id: "1058".into(),
            principal_kind: "user".into(),
            role: MembershipRole::Member,
            invited_by: "1".into(),
        })
        .expect("owner should be able to add target member");

    let add_attempt = runtime.add_member(AddConversationMemberCommand {
        tenant_id: "100001".into(),
        organization_id: "0".into(),
        conversation_id: "c_group_member_governance".into(),
        principal_id: "1059".into(),
        principal_kind: "user".into(),
        role: MembershipRole::Member,
        invited_by: "1043".into(),
    });
    assert!(matches!(
        add_attempt,
        Err(RuntimeError::PermissionDenied(_))
    ));

    let remove_attempt = runtime.remove_member(RemoveConversationMemberCommand {
        tenant_id: "100001".into(),
        organization_id: "0".into(),
        conversation_id: "c_group_member_governance".into(),
        member_id: target.member_id,
        removed_by: "1043".into(),
    });
    assert!(matches!(
        remove_attempt,
        Err(RuntimeError::PermissionDenied(_))
    ));
}

#[test]
fn test_governance_writes_reject_actor_kind_mismatch() {
    let journal = InMemoryJournal::default();
    let runtime = ConversationRuntime::new(journal);

    runtime
        .create_conversation(CreateConversationCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_group_actor_kind_governance".into(),
            creator_id: "1".into(),
            conversation_type: "group".into(),
        })
        .expect("create conversation should succeed");

    let member = runtime
        .add_member(AddConversationMemberCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_group_actor_kind_governance".into(),
            principal_id: "1043".into(),
            principal_kind: "user".into(),
            role: MembershipRole::Member,
            invited_by: "1".into(),
        })
        .expect("owner should be able to add member");

    let target = runtime
        .add_member(AddConversationMemberCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_group_actor_kind_governance".into(),
            principal_id: "1058".into(),
            principal_kind: "user".into(),
            role: MembershipRole::Member,
            invited_by: "1".into(),
        })
        .expect("owner should be able to add transfer target");

    let add_attempt = runtime.add_member_with_actor_kind(
        AddConversationMemberCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_group_actor_kind_governance".into(),
            principal_id: "1059".into(),
            principal_kind: "user".into(),
            role: MembershipRole::Member,
            invited_by: "1".into(),
        },
        "agent",
    );
    assert!(matches!(
        add_attempt,
        Err(RuntimeError::PermissionDenied(_))
    ));

    let remove_attempt = runtime.remove_member_with_actor_kind(
        RemoveConversationMemberCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_group_actor_kind_governance".into(),
            member_id: target.member_id.clone(),
            removed_by: "1".into(),
        },
        "agent",
    );
    assert!(matches!(
        remove_attempt,
        Err(RuntimeError::PermissionDenied(_))
    ));

    let leave_attempt = runtime.leave_conversation_with_actor_kind(
        LeaveConversationCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_group_actor_kind_governance".into(),
            principal_id: "1043".into(),
        },
        "agent",
    );
    assert!(matches!(
        leave_attempt,
        Err(RuntimeError::PermissionDenied(_))
    ));

    let transfer_attempt = runtime.transfer_conversation_owner_with_actor_kind(
        TransferConversationOwnerCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_group_actor_kind_governance".into(),
            target_member_id: target.member_id.clone(),
            transferred_by: "1".into(),
        },
        "agent",
    );
    assert!(matches!(
        transfer_attempt,
        Err(RuntimeError::PermissionDenied(_))
    ));

    let role_change_attempt = runtime.change_conversation_member_role_with_actor_kind(
        ChangeConversationMemberRoleCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_group_actor_kind_governance".into(),
            target_member_id: member.member_id.clone(),
            new_role: MembershipRole::Admin,
            changed_by: "1".into(),
        },
        "agent",
    );
    assert!(matches!(
        role_change_attempt,
        Err(RuntimeError::PermissionDenied(_))
    ));

    let members = runtime
        .list_members("100001", "default", "c_group_actor_kind_governance")
        .expect("list members should succeed");
    assert_eq!(members.len(), 3);
    let owner = members
        .iter()
        .find(|item| item.principal_id == "1")
        .expect("owner should exist");
    assert_eq!(owner.role, MembershipRole::Owner);
    let member_state = members
        .iter()
        .find(|item| item.principal_id == "1043")
        .expect("member should exist");
    assert_eq!(member_state.role, MembershipRole::Member);
    let target_state = members
        .iter()
        .find(|item| item.principal_id == "1058")
        .expect("target should exist");
    assert_eq!(target_state.role, MembershipRole::Member);
}

#[test]
fn test_add_member_does_not_leak_membership_when_journal_append_fails() {
    let journal = FailAfterNJournal::new(3);
    let runtime = ConversationRuntime::new(journal.clone());

    runtime
        .create_conversation(CreateConversationCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_group_add_commit_fail".into(),
            creator_id: "1".into(),
            conversation_type: "group".into(),
        })
        .expect("create conversation should succeed before forced failure");

    let add_attempt = runtime.add_member(AddConversationMemberCommand {
        tenant_id: "100001".into(),
        organization_id: "0".into(),
        conversation_id: "c_group_add_commit_fail".into(),
        principal_id: "1043".into(),
        principal_kind: "user".into(),
        role: MembershipRole::Member,
        invited_by: "1".into(),
    });
    assert!(matches!(
        add_attempt,
        Err(RuntimeError::Contract(ContractError::Unavailable(message)))
            if message == "forced journal append failure"
    ));

    let members = runtime
        .list_members("100001", "default", "c_group_add_commit_fail")
        .expect("list members should still succeed");
    assert_eq!(members.len(), 1, "failed add must not leak a new member");
    assert_eq!(members[0].principal_id, "1");
    assert_eq!(journal.recorded().len(), 2);
}

#[test]
fn test_remove_member_does_not_leak_removed_state_when_journal_append_fails() {
    let journal = FailAfterNJournal::new(4);
    let runtime = ConversationRuntime::new(journal.clone());

    runtime
        .create_conversation(CreateConversationCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_group_remove_commit_fail".into(),
            creator_id: "1".into(),
            conversation_type: "group".into(),
        })
        .expect("create conversation should succeed before forced failure");
    let joined = runtime
        .add_member(AddConversationMemberCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_group_remove_commit_fail".into(),
            principal_id: "1043".into(),
            principal_kind: "user".into(),
            role: MembershipRole::Member,
            invited_by: "1".into(),
        })
        .expect("member add should succeed before forced failure");

    let remove_attempt = runtime.remove_member(RemoveConversationMemberCommand {
        tenant_id: "100001".into(),
        organization_id: "0".into(),
        conversation_id: "c_group_remove_commit_fail".into(),
        member_id: joined.member_id.clone(),
        removed_by: "1".into(),
    });
    assert!(matches!(
        remove_attempt,
        Err(RuntimeError::Contract(ContractError::Unavailable(message)))
            if message == "forced journal append failure"
    ));

    let members = runtime
        .list_members("100001", "default", "c_group_remove_commit_fail")
        .expect("list members should still succeed");
    assert_eq!(members.len(), 2, "failed remove must keep target active");
    assert!(
        members
            .iter()
            .any(|member| member.member_id == joined.member_id && member.is_active())
    );
    assert_eq!(journal.recorded().len(), 3);
}

#[test]
fn test_leave_conversation_does_not_leak_left_state_when_journal_append_fails() {
    let journal = FailAfterNJournal::new(4);
    let runtime = ConversationRuntime::new(journal.clone());

    runtime
        .create_conversation(CreateConversationCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_group_leave_commit_fail".into(),
            creator_id: "1".into(),
            conversation_type: "group".into(),
        })
        .expect("create conversation should succeed before forced failure");
    let joined = runtime
        .add_member(AddConversationMemberCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_group_leave_commit_fail".into(),
            principal_id: "1043".into(),
            principal_kind: "user".into(),
            role: MembershipRole::Member,
            invited_by: "1".into(),
        })
        .expect("member add should succeed before forced failure");

    let leave_attempt = runtime.leave_conversation(LeaveConversationCommand {
        tenant_id: "100001".into(),
        organization_id: "0".into(),
        conversation_id: "c_group_leave_commit_fail".into(),
        principal_id: "1043".into(),
    });
    assert!(matches!(
        leave_attempt,
        Err(RuntimeError::Contract(ContractError::Unavailable(message)))
            if message == "forced journal append failure"
    ));

    let members = runtime
        .list_members("100001", "default", "c_group_leave_commit_fail")
        .expect("list members should still succeed");
    assert_eq!(members.len(), 2, "failed leave must keep leaver active");
    assert!(
        members
            .iter()
            .any(|member| member.member_id == joined.member_id && member.is_active())
    );
    assert_eq!(journal.recorded().len(), 3);
}

#[test]
fn test_transfer_owner_does_not_leak_role_swap_when_journal_append_fails() {
    let journal = FailAfterNJournal::new(4);
    let runtime = ConversationRuntime::new(journal.clone());

    runtime
        .create_conversation(CreateConversationCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_group_transfer_commit_fail".into(),
            creator_id: "1".into(),
            conversation_type: "group".into(),
        })
        .expect("create conversation should succeed before forced failure");
    let target = runtime
        .add_member(AddConversationMemberCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_group_transfer_commit_fail".into(),
            principal_id: "1058".into(),
            principal_kind: "user".into(),
            role: MembershipRole::Member,
            invited_by: "1".into(),
        })
        .expect("member add should succeed before forced failure");

    let transfer_attempt = runtime.transfer_conversation_owner(TransferConversationOwnerCommand {
        tenant_id: "100001".into(),
        organization_id: "0".into(),
        conversation_id: "c_group_transfer_commit_fail".into(),
        target_member_id: target.member_id.clone(),
        transferred_by: "1".into(),
    });
    assert!(matches!(
        transfer_attempt,
        Err(RuntimeError::Contract(ContractError::Unavailable(message)))
            if message == "forced journal append failure"
    ));

    let members = runtime
        .list_members("100001", "default", "c_group_transfer_commit_fail")
        .expect("list members should still succeed");
    let owner = members
        .iter()
        .find(|member| member.principal_id == "1")
        .expect("owner should remain present");
    assert_eq!(
        owner.role,
        MembershipRole::Owner,
        "failed transfer must preserve original owner role"
    );
    let target_state = members
        .iter()
        .find(|member| member.member_id == target.member_id)
        .expect("target should remain present");
    assert_eq!(
        target_state.role,
        MembershipRole::Member,
        "failed transfer must preserve target role"
    );
    assert_eq!(journal.recorded().len(), 3);
}

#[test]
fn test_role_change_does_not_leak_updated_role_when_journal_append_fails() {
    let journal = FailAfterNJournal::new(4);
    let runtime = ConversationRuntime::new(journal.clone());

    runtime
        .create_conversation(CreateConversationCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_group_role_commit_fail".into(),
            creator_id: "1".into(),
            conversation_type: "group".into(),
        })
        .expect("create conversation should succeed before forced failure");
    let member = runtime
        .add_member(AddConversationMemberCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_group_role_commit_fail".into(),
            principal_id: "1043".into(),
            principal_kind: "user".into(),
            role: MembershipRole::Member,
            invited_by: "1".into(),
        })
        .expect("member add should succeed before forced failure");

    let role_change_attempt =
        runtime.change_conversation_member_role(ChangeConversationMemberRoleCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_group_role_commit_fail".into(),
            target_member_id: member.member_id.clone(),
            new_role: MembershipRole::Admin,
            changed_by: "1".into(),
        });
    assert!(matches!(
        role_change_attempt,
        Err(RuntimeError::Contract(ContractError::Unavailable(message)))
            if message == "forced journal append failure"
    ));

    let members = runtime
        .list_members("100001", "default", "c_group_role_commit_fail")
        .expect("list members should still succeed");
    let member_state = members
        .iter()
        .find(|item| item.member_id == member.member_id)
        .expect("member should remain present");
    assert_eq!(
        member_state.role,
        MembershipRole::Member,
        "failed role change must preserve original role"
    );
    assert_eq!(journal.recorded().len(), 3);
}

#[test]
fn test_read_cursor_does_not_advance_when_journal_append_fails() {
    let journal = FailAfterNJournal::new(5);
    let runtime = ConversationRuntime::new(journal.clone());

    runtime
        .create_conversation(CreateConversationCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_group_cursor_commit_fail".into(),
            creator_id: "1".into(),
            conversation_type: "group".into(),
        })
        .expect("create conversation should succeed before forced failure");
    runtime
        .add_member(AddConversationMemberCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_group_cursor_commit_fail".into(),
            principal_id: "1043".into(),
            principal_kind: "user".into(),
            role: MembershipRole::Member,
            invited_by: "1".into(),
        })
        .expect("member add should succeed before forced failure");
    runtime
        .post_message(PostMessageCommand {
            tenant_id: "100001".into(),
           organization_id: "0".into(),
            conversation_id: "c_group_cursor_commit_fail".into(),
            sender: Sender {
                id: "1043".into(),
                kind: "user".into(),
                member_id: None,
                device_id: Some("d_member".into()),
                session_id: Some("s_member".into()),
                metadata: Default::default(),
            },
            client_msg_id: Some("client_cursor_commit_fail".into()),
            message_type: MessageType::Standard,
            body: MessageBody {
                summary: Some("hello".into()),
                parts: vec![ContentPart::text("hello")],
                render_hints: Default::default(),
                reply_to: None,
            },
        })
        .expect("post message should succeed before forced failure");

    let update_attempt = runtime.update_read_cursor(UpdateReadCursorCommand {
        tenant_id: "100001".into(),
        organization_id: "0".into(),
        conversation_id: "c_group_cursor_commit_fail".into(),
        principal_id: "1".into(),
        read_seq: 1,
        last_read_message_id: Some("msg_c_group_cursor_commit_fail_1".into()),
    });
    assert!(matches!(
        update_attempt,
        Err(RuntimeError::Contract(ContractError::Unavailable(message)))
            if message == "forced journal append failure"
    ));

    let cursor = runtime
        .read_cursor_view("100001", "default", "c_group_cursor_commit_fail", "1")
        .expect("cursor view should still succeed");
    assert_eq!(
        cursor.read_seq, 0,
        "failed update must not advance read seq"
    );
    assert_eq!(
        cursor.unread_count, 1,
        "failed update must preserve unread count until durable commit succeeds"
    );
    assert_eq!(journal.recorded().len(), 4);
}

#[test]
fn test_post_message_does_not_leak_message_when_journal_append_fails() {
    let journal = FailAfterNJournal::new(3);
    let runtime = ConversationRuntime::new(journal.clone());

    runtime
        .create_conversation(CreateConversationCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_group_post_commit_fail".into(),
            creator_id: "1".into(),
            conversation_type: "group".into(),
        })
        .expect("create conversation should succeed before forced failure");

    let post_attempt = runtime.post_message(PostMessageCommand {
        tenant_id: "100001".into(),
       organization_id: "0".into(),
        conversation_id: "c_group_post_commit_fail".into(),
        sender: Sender {
            id: "1".into(),
            kind: "user".into(),
            member_id: None,
            device_id: Some("d_owner".into()),
            session_id: Some("s_owner".into()),
            metadata: Default::default(),
        },
        client_msg_id: Some("client_post_commit_fail".into()),
        message_type: MessageType::Standard,
        body: MessageBody {
            summary: Some("hello".into()),
            parts: vec![ContentPart::text("hello")],
            render_hints: Default::default(),
            reply_to: None,
        },
    });
    assert!(matches!(
        post_attempt,
        Err(RuntimeError::Contract(ContractError::Unavailable(message)))
            if message == "forced journal append failure"
    ));

    let history = list_all_messages(&runtime, "100001", "c_group_post_commit_fail", "1")
        .expect("history should still load");
    assert_eq!(history.high_watermark, 0);
    assert!(
        history.items.is_empty(),
        "failed post must not leak a message"
    );
    assert_eq!(journal.recorded().len(), 2);
}

#[test]
fn test_edit_message_does_not_leak_body_change_when_journal_append_fails() {
    let journal = FailAfterNJournal::new(4);
    let runtime = ConversationRuntime::new(journal.clone());

    runtime
        .create_conversation(CreateConversationCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_group_edit_commit_fail".into(),
            creator_id: "1".into(),
            conversation_type: "group".into(),
        })
        .expect("create conversation should succeed before forced failure");
    let posted = runtime
        .post_message(PostMessageCommand {
            tenant_id: "100001".into(),
           organization_id: "0".into(),
            conversation_id: "c_group_edit_commit_fail".into(),
            sender: Sender {
                id: "1".into(),
                kind: "user".into(),
                member_id: None,
                device_id: Some("d_owner".into()),
                session_id: Some("s_owner".into()),
                metadata: Default::default(),
            },
            client_msg_id: Some("client_edit_commit_fail".into()),
            message_type: MessageType::Standard,
            body: MessageBody {
                summary: Some("hello".into()),
                parts: vec![ContentPart::text("hello")],
                render_hints: Default::default(),
                reply_to: None,
            },
        })
        .expect("post should succeed before forced failure");

    let edit_attempt = runtime.edit_message(EditMessageCommand {
        tenant_id: "100001".into(),
        organization_id: "0".into(),
        message_id: posted.message_id.clone(),
        editor: Sender {
            id: "1".into(),
            kind: "user".into(),
            member_id: None,
            device_id: Some("d_owner".into()),
            session_id: Some("s_owner".into()),
            metadata: Default::default(),
        },
        body: MessageBody {
            summary: Some("edited".into()),
            parts: vec![ContentPart::text("edited")],
            render_hints: Default::default(),
            reply_to: None,
        },
    });
    assert!(matches!(
        edit_attempt,
        Err(RuntimeError::Contract(ContractError::Unavailable(message)))
            if message == "forced journal append failure"
    ));

    let history = list_all_messages(&runtime, "100001", "c_group_edit_commit_fail", "1")
        .expect("history should still load");
    assert_eq!(history.items.len(), 1);
    assert_eq!(
        history.items[0].message.body.summary.as_deref(),
        Some("hello")
    );
    assert_eq!(
        history.items[0].message.body.parts,
        vec![ContentPart::text("hello")]
    );
    assert_eq!(journal.recorded().len(), 3);
}

#[test]
fn test_recall_message_does_not_leak_recalled_state_when_journal_append_fails() {
    let journal = FailAfterNJournal::new(4);
    let runtime = ConversationRuntime::new(journal.clone());

    runtime
        .create_conversation(CreateConversationCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_group_recall_commit_fail".into(),
            creator_id: "1".into(),
            conversation_type: "group".into(),
        })
        .expect("create conversation should succeed before forced failure");
    let posted = runtime
        .post_message(PostMessageCommand {
            tenant_id: "100001".into(),
           organization_id: "0".into(),
            conversation_id: "c_group_recall_commit_fail".into(),
            sender: Sender {
                id: "1".into(),
                kind: "user".into(),
                member_id: None,
                device_id: Some("d_owner".into()),
                session_id: Some("s_owner".into()),
                metadata: Default::default(),
            },
            client_msg_id: Some("client_recall_commit_fail".into()),
            message_type: MessageType::Standard,
            body: MessageBody {
                summary: Some("hello".into()),
                parts: vec![ContentPart::text("hello")],
                render_hints: Default::default(),
                reply_to: None,
            },
        })
        .expect("post should succeed before forced failure");

    let recall_attempt = runtime.recall_message(RecallMessageCommand {
        tenant_id: "100001".into(),
        organization_id: "0".into(),
        message_id: posted.message_id,
        recalled_by: Sender {
            id: "1".into(),
            kind: "user".into(),
            member_id: None,
            device_id: Some("d_owner".into()),
            session_id: Some("s_owner".into()),
            metadata: Default::default(),
        },
    });
    assert!(matches!(
        recall_attempt,
        Err(RuntimeError::Contract(ContractError::Unavailable(message)))
            if message == "forced journal append failure"
    ));

    let history = list_all_messages(&runtime, "100001", "c_group_recall_commit_fail", "1")
        .expect("history should still load");
    assert_eq!(history.items.len(), 1);
    assert!(!history.items[0].recalled);
    assert_eq!(
        history.items[0].message.body.summary.as_deref(),
        Some("hello")
    );
    assert_eq!(journal.recorded().len(), 3);
}

#[test]
fn test_add_reaction_does_not_leak_reaction_when_journal_append_fails() {
    let journal = FailAfterNJournal::new(4);
    let runtime = ConversationRuntime::new(journal.clone());

    runtime
        .create_conversation(CreateConversationCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_group_reaction_add_commit_fail".into(),
            creator_id: "1".into(),
            conversation_type: "group".into(),
        })
        .expect("create conversation should succeed before forced failure");
    let posted = runtime
        .post_message(PostMessageCommand {
            tenant_id: "100001".into(),
           organization_id: "0".into(),
            conversation_id: "c_group_reaction_add_commit_fail".into(),
            sender: Sender {
                id: "1".into(),
                kind: "user".into(),
                member_id: None,
                device_id: Some("d_owner".into()),
                session_id: Some("s_owner".into()),
                metadata: Default::default(),
            },
            client_msg_id: Some("client_reaction_add_commit_fail".into()),
            message_type: MessageType::Standard,
            body: MessageBody {
                summary: Some("hello".into()),
                parts: vec![ContentPart::text("hello")],
                render_hints: Default::default(),
                reply_to: None,
            },
        })
        .expect("post should succeed before forced failure");

    let reaction_attempt = runtime.add_message_reaction(AddMessageReactionCommand {
        tenant_id: "100001".into(),
        organization_id: "0".into(),
        message_id: posted.message_id.clone(),
        reaction_key: "thumbs_up".into(),
        reacted_by: Sender {
            id: "1".into(),
            kind: "user".into(),
            member_id: None,
            device_id: Some("d_owner".into()),
            session_id: Some("s_owner".into()),
            metadata: Default::default(),
        },
    });
    assert!(matches!(
        reaction_attempt,
        Err(RuntimeError::Contract(ContractError::Unavailable(message)))
            if message == "forced journal append failure"
    ));

    let history = list_all_messages(
        &runtime,
        "100001",
        "c_group_reaction_add_commit_fail",
        "1",
    )
    .expect("history should still load");
    assert_eq!(history.items.len(), 1);
    assert!(
        history.items[0].reactions.is_empty(),
        "failed reaction add must not leak reaction state"
    );
    assert_eq!(journal.recorded().len(), 3);
}

#[test]
fn test_remove_reaction_does_not_leak_reaction_removal_when_journal_append_fails() {
    let journal = FailAfterNJournal::new(5);
    let runtime = ConversationRuntime::new(journal.clone());

    runtime
        .create_conversation(CreateConversationCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_group_reaction_remove_commit_fail".into(),
            creator_id: "1".into(),
            conversation_type: "group".into(),
        })
        .expect("create conversation should succeed before forced failure");
    let posted = runtime
        .post_message(PostMessageCommand {
            tenant_id: "100001".into(),
           organization_id: "0".into(),
            conversation_id: "c_group_reaction_remove_commit_fail".into(),
            sender: Sender {
                id: "1".into(),
                kind: "user".into(),
                member_id: None,
                device_id: Some("d_owner".into()),
                session_id: Some("s_owner".into()),
                metadata: Default::default(),
            },
            client_msg_id: Some("client_reaction_remove_commit_fail".into()),
            message_type: MessageType::Standard,
            body: MessageBody {
                summary: Some("hello".into()),
                parts: vec![ContentPart::text("hello")],
                render_hints: Default::default(),
                reply_to: None,
            },
        })
        .expect("post should succeed before forced failure");
    runtime
        .add_message_reaction(AddMessageReactionCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            message_id: posted.message_id.clone(),
            reaction_key: "thumbs_up".into(),
            reacted_by: Sender {
                id: "1".into(),
                kind: "user".into(),
                member_id: None,
                device_id: Some("d_owner".into()),
                session_id: Some("s_owner".into()),
                metadata: Default::default(),
            },
        })
        .expect("reaction add should succeed before forced failure");

    let remove_attempt = runtime.remove_message_reaction(RemoveMessageReactionCommand {
        tenant_id: "100001".into(),
        organization_id: "0".into(),
        message_id: posted.message_id,
        reaction_key: "thumbs_up".into(),
        removed_by: Sender {
            id: "1".into(),
            kind: "user".into(),
            member_id: None,
            device_id: Some("d_owner".into()),
            session_id: Some("s_owner".into()),
            metadata: Default::default(),
        },
    });
    assert!(matches!(
        remove_attempt,
        Err(RuntimeError::Contract(ContractError::Unavailable(message)))
            if message == "forced journal append failure"
    ));

    let history = list_all_messages(
        &runtime,
        "100001",
        "c_group_reaction_remove_commit_fail",
        "1",
    )
    .expect("history should still load");
    assert_eq!(history.items.len(), 1);
    assert_eq!(
        history.items[0]
            .reactions
            .get("thumbs_up")
            .map(|actors| actors.len()),
        Some(1),
        "failed reaction remove must preserve prior reaction state"
    );
    assert_eq!(journal.recorded().len(), 4);
}

#[test]
fn test_pin_message_does_not_leak_pin_state_when_journal_append_fails() {
    let journal = FailAfterNJournal::new(4);
    let runtime = ConversationRuntime::new(journal.clone());

    runtime
        .create_conversation(CreateConversationCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_group_pin_commit_fail".into(),
            creator_id: "1".into(),
            conversation_type: "group".into(),
        })
        .expect("create conversation should succeed before forced failure");
    let posted = runtime
        .post_message(PostMessageCommand {
            tenant_id: "100001".into(),
           organization_id: "0".into(),
            conversation_id: "c_group_pin_commit_fail".into(),
            sender: Sender {
                id: "1".into(),
                kind: "user".into(),
                member_id: None,
                device_id: Some("d_owner".into()),
                session_id: Some("s_owner".into()),
                metadata: Default::default(),
            },
            client_msg_id: Some("client_pin_commit_fail".into()),
            message_type: MessageType::Standard,
            body: MessageBody {
                summary: Some("hello".into()),
                parts: vec![ContentPart::text("hello")],
                render_hints: Default::default(),
                reply_to: None,
            },
        })
        .expect("post should succeed before forced failure");

    let pin_attempt = runtime.pin_message(PinMessageCommand {
        tenant_id: "100001".into(),
        organization_id: "0".into(),
        message_id: posted.message_id,
        pinned_by: Sender {
            id: "1".into(),
            kind: "user".into(),
            member_id: None,
            device_id: Some("d_owner".into()),
            session_id: Some("s_owner".into()),
            metadata: Default::default(),
        },
    });
    assert!(matches!(
        pin_attempt,
        Err(RuntimeError::Contract(ContractError::Unavailable(message)))
            if message == "forced journal append failure"
    ));

    let history = list_all_messages(&runtime, "100001", "c_group_pin_commit_fail", "1")
        .expect("history should still load");
    assert_eq!(history.items.len(), 1);
    assert!(history.items[0].pin.is_none());
    assert_eq!(journal.recorded().len(), 3);
}

#[test]
fn test_unpin_message_does_not_leak_pin_removal_when_journal_append_fails() {
    let journal = FailAfterNJournal::new(5);
    let runtime = ConversationRuntime::new(journal.clone());

    runtime
        .create_conversation(CreateConversationCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_group_unpin_commit_fail".into(),
            creator_id: "1".into(),
            conversation_type: "group".into(),
        })
        .expect("create conversation should succeed before forced failure");
    let posted = runtime
        .post_message(PostMessageCommand {
            tenant_id: "100001".into(),
           organization_id: "0".into(),
            conversation_id: "c_group_unpin_commit_fail".into(),
            sender: Sender {
                id: "1".into(),
                kind: "user".into(),
                member_id: None,
                device_id: Some("d_owner".into()),
                session_id: Some("s_owner".into()),
                metadata: Default::default(),
            },
            client_msg_id: Some("client_unpin_commit_fail".into()),
            message_type: MessageType::Standard,
            body: MessageBody {
                summary: Some("hello".into()),
                parts: vec![ContentPart::text("hello")],
                render_hints: Default::default(),
                reply_to: None,
            },
        })
        .expect("post should succeed before forced failure");
    runtime
        .pin_message(PinMessageCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            message_id: posted.message_id.clone(),
            pinned_by: Sender {
                id: "1".into(),
                kind: "user".into(),
                member_id: None,
                device_id: Some("d_owner".into()),
                session_id: Some("s_owner".into()),
                metadata: Default::default(),
            },
        })
        .expect("pin should succeed before forced failure");

    let unpin_attempt = runtime.unpin_message(UnpinMessageCommand {
        tenant_id: "100001".into(),
        organization_id: "0".into(),
        message_id: posted.message_id,
        unpinned_by: Sender {
            id: "1".into(),
            kind: "user".into(),
            member_id: None,
            device_id: Some("d_owner".into()),
            session_id: Some("s_owner".into()),
            metadata: Default::default(),
        },
    });
    assert!(matches!(
        unpin_attempt,
        Err(RuntimeError::Contract(ContractError::Unavailable(message)))
            if message == "forced journal append failure"
    ));

    let history = list_all_messages(&runtime, "100001", "c_group_unpin_commit_fail", "1")
        .expect("history should still load");
    assert_eq!(history.items.len(), 1);
    assert!(
        history.items[0].pin.is_some(),
        "failed unpin must preserve prior pin state"
    );
    assert_eq!(journal.recorded().len(), 4);
}

#[test]
fn test_group_admin_can_manage_regular_members_but_cannot_escalate_roles() {
    let journal = InMemoryJournal::default();
    let runtime = ConversationRuntime::new(journal);

    runtime
        .create_conversation(CreateConversationCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_group_admin_governance".into(),
            creator_id: "1".into(),
            conversation_type: "group".into(),
        })
        .expect("create conversation should succeed");

    runtime
        .add_member(AddConversationMemberCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_group_admin_governance".into(),
            principal_id: "1003".into(),
            principal_kind: "user".into(),
            role: MembershipRole::Admin,
            invited_by: "1".into(),
        })
        .expect("owner should be able to add admin");

    let admin_peer = runtime
        .add_member(AddConversationMemberCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_group_admin_governance".into(),
            principal_id: "1004".into(),
            principal_kind: "user".into(),
            role: MembershipRole::Admin,
            invited_by: "1".into(),
        })
        .expect("owner should be able to add another admin");

    let joined = runtime
        .add_member(AddConversationMemberCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_group_admin_governance".into(),
            principal_id: "1043".into(),
            principal_kind: "user".into(),
            role: MembershipRole::Member,
            invited_by: "1003".into(),
        })
        .expect("admin should be able to add regular member");
    assert_eq!(joined.role, MembershipRole::Member);

    let admin_escalation = runtime.add_member(AddConversationMemberCommand {
        tenant_id: "100001".into(),
        organization_id: "0".into(),
        conversation_id: "c_group_admin_governance".into(),
        principal_id: "1005".into(),
        principal_kind: "user".into(),
        role: MembershipRole::Admin,
        invited_by: "1003".into(),
    });
    assert!(matches!(
        admin_escalation,
        Err(RuntimeError::PermissionDenied(_))
    ));

    let owner_escalation = runtime.add_member(AddConversationMemberCommand {
        tenant_id: "100001".into(),
        organization_id: "0".into(),
        conversation_id: "c_group_admin_governance".into(),
        principal_id: "1006".into(),
        principal_kind: "user".into(),
        role: MembershipRole::Owner,
        invited_by: "1003".into(),
    });
    assert!(matches!(
        owner_escalation,
        Err(RuntimeError::PermissionDenied(_))
    ));

    let admin_remove_admin = runtime.remove_member(RemoveConversationMemberCommand {
        tenant_id: "100001".into(),
        organization_id: "0".into(),
        conversation_id: "c_group_admin_governance".into(),
        member_id: admin_peer.member_id,
        removed_by: "1003".into(),
    });
    assert!(matches!(
        admin_remove_admin,
        Err(RuntimeError::PermissionDenied(_))
    ));

    let removed = runtime
        .remove_member(RemoveConversationMemberCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_group_admin_governance".into(),
            member_id: joined.member_id,
            removed_by: "1003".into(),
        })
        .expect("admin should be able to remove regular member");
    assert_eq!(removed.state, MembershipState::Removed);
}

#[test]
fn test_group_owner_cannot_create_second_owner() {
    let journal = InMemoryJournal::default();
    let runtime = ConversationRuntime::new(journal);

    runtime
        .create_conversation(CreateConversationCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_group_owner_governance".into(),
            creator_id: "1".into(),
            conversation_type: "group".into(),
        })
        .expect("create conversation should succeed");

    let second_owner = runtime.add_member(AddConversationMemberCommand {
        tenant_id: "100001".into(),
        organization_id: "0".into(),
        conversation_id: "c_group_owner_governance".into(),
        principal_id: "1006".into(),
        principal_kind: "user".into(),
        role: MembershipRole::Owner,
        invited_by: "1".into(),
    });
    assert!(matches!(
        second_owner,
        Err(RuntimeError::PermissionDenied(_))
    ));
}

#[test]
fn test_direct_conversation_owner_can_add_only_single_non_elevated_peer() {
    let journal = InMemoryJournal::default();
    let runtime = ConversationRuntime::new(journal);

    runtime
        .create_conversation(CreateConversationCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_direct_member_governance".into(),
            creator_id: "1".into(),
            conversation_type: "direct".into(),
        })
        .expect("create conversation should succeed");

    let peer = runtime
        .add_member(AddConversationMemberCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_direct_member_governance".into(),
            principal_id: "1057".into(),
            principal_kind: "user".into(),
            role: MembershipRole::Member,
            invited_by: "1".into(),
        })
        .expect("owner should be able to add the direct conversation peer");
    assert_eq!(peer.role, MembershipRole::Member);

    let third_participant = runtime.add_member(AddConversationMemberCommand {
        tenant_id: "100001".into(),
        organization_id: "0".into(),
        conversation_id: "c_direct_member_governance".into(),
        principal_id: "1060".into(),
        principal_kind: "user".into(),
        role: MembershipRole::Member,
        invited_by: "1".into(),
    });
    assert!(matches!(
        third_participant,
        Err(RuntimeError::PermissionDenied(_))
    ));

    let elevated_peer = runtime.add_member(AddConversationMemberCommand {
        tenant_id: "100001".into(),
        organization_id: "0".into(),
        conversation_id: "c_direct_member_governance".into(),
        principal_id: "1061".into(),
        principal_kind: "user".into(),
        role: MembershipRole::Admin,
        invited_by: "1".into(),
    });
    assert!(matches!(
        elevated_peer,
        Err(RuntimeError::PermissionDenied(_))
    ));
}

#[test]
fn test_direct_conversation_rejects_member_removal() {
    let journal = InMemoryJournal::default();
    let runtime = ConversationRuntime::new(journal);

    runtime
        .create_conversation(CreateConversationCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_direct_remove_governance".into(),
            creator_id: "1".into(),
            conversation_type: "direct".into(),
        })
        .expect("create conversation should succeed");

    let peer = runtime
        .add_member(AddConversationMemberCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_direct_remove_governance".into(),
            principal_id: "1057".into(),
            principal_kind: "user".into(),
            role: MembershipRole::Member,
            invited_by: "1".into(),
        })
        .expect("owner should be able to add the direct conversation peer");

    let remove_attempt = runtime.remove_member(RemoveConversationMemberCommand {
        tenant_id: "100001".into(),
        organization_id: "0".into(),
        conversation_id: "c_direct_remove_governance".into(),
        member_id: peer.member_id,
        removed_by: "1".into(),
    });
    assert!(matches!(
        remove_attempt,
        Err(RuntimeError::PermissionDenied(_))
    ));
}

#[test]
fn test_group_member_can_leave_and_loses_access() {
    let journal = InMemoryJournal::default();
    let runtime = ConversationRuntime::new(journal);

    runtime
        .create_conversation(CreateConversationCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_group_leave".into(),
            creator_id: "1".into(),
            conversation_type: "group".into(),
        })
        .expect("create conversation should succeed");

    runtime
        .add_member(AddConversationMemberCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_group_leave".into(),
            principal_id: "1043".into(),
            principal_kind: "user".into(),
            role: MembershipRole::Member,
            invited_by: "1".into(),
        })
        .expect("owner should be able to add member");

    let left_member = runtime
        .leave_conversation(LeaveConversationCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_group_leave".into(),
            principal_id: "1043".into(),
        })
        .expect("member should be able to leave group conversation");
    assert_eq!(left_member.state, MembershipState::Left);
    assert!(left_member.removed_at.is_some());

    let members = runtime
        .list_members("100001", "default", "c_group_leave")
        .expect("list members should succeed");
    assert_eq!(members.len(), 1);
    assert_eq!(members[0].principal_id, "1");

    let post_after_leave = runtime.post_message(PostMessageCommand {
        tenant_id: "100001".into(),
       organization_id: "0".into(),
        conversation_id: "c_group_leave".into(),
        sender: Sender {
            id: "1043".into(),
            kind: "user".into(),
            member_id: None,
            device_id: Some("d_member".into()),
            session_id: Some("s_member".into()),
            metadata: Default::default(),
        },
        client_msg_id: Some("client_after_leave".into()),
        message_type: MessageType::Standard,
        body: MessageBody {
            summary: Some("after leave".into()),
            parts: vec![ContentPart::text("after leave")],
            render_hints: Default::default(),
            reply_to: None,
        },
    });
    assert!(matches!(
        post_after_leave,
        Err(RuntimeError::PermissionDenied(_))
    ));
}

#[test]
fn test_group_owner_cannot_leave_without_transfer() {
    let journal = InMemoryJournal::default();
    let runtime = ConversationRuntime::new(journal);

    runtime
        .create_conversation(CreateConversationCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_group_owner_leave".into(),
            creator_id: "1".into(),
            conversation_type: "group".into(),
        })
        .expect("create conversation should succeed");

    let leave = runtime.leave_conversation(LeaveConversationCommand {
        tenant_id: "100001".into(),
        organization_id: "0".into(),
        conversation_id: "c_group_owner_leave".into(),
        principal_id: "1".into(),
    });
    assert!(matches!(leave, Err(RuntimeError::PermissionDenied(_))));
}

#[test]
fn test_direct_conversation_rejects_leave_for_now() {
    let journal = InMemoryJournal::default();
    let runtime = ConversationRuntime::new(journal);

    runtime
        .create_conversation(CreateConversationCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_direct_leave".into(),
            creator_id: "1".into(),
            conversation_type: "direct".into(),
        })
        .expect("create conversation should succeed");

    runtime
        .add_member(AddConversationMemberCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_direct_leave".into(),
            principal_id: "1057".into(),
            principal_kind: "user".into(),
            role: MembershipRole::Member,
            invited_by: "1".into(),
        })
        .expect("owner should be able to add direct peer");

    let leave = runtime.leave_conversation(LeaveConversationCommand {
        tenant_id: "100001".into(),
        organization_id: "0".into(),
        conversation_id: "c_direct_leave".into(),
        principal_id: "1057".into(),
    });
    assert!(matches!(leave, Err(RuntimeError::PermissionDenied(_))));
}

#[test]
fn test_group_owner_can_transfer_ownership_and_then_leave() {
    let journal = InMemoryJournal::default();
    let runtime = ConversationRuntime::new(journal);

    runtime
        .create_conversation(CreateConversationCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_group_transfer_owner".into(),
            creator_id: "1".into(),
            conversation_type: "group".into(),
        })
        .expect("create conversation should succeed");

    let promoted_member = runtime
        .add_member(AddConversationMemberCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_group_transfer_owner".into(),
            principal_id: "1043".into(),
            principal_kind: "user".into(),
            role: MembershipRole::Member,
            invited_by: "1".into(),
        })
        .expect("owner should be able to add member");

    let transfer = runtime
        .transfer_conversation_owner(TransferConversationOwnerCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_group_transfer_owner".into(),
            target_member_id: promoted_member.member_id,
            transferred_by: "1".into(),
        })
        .expect("owner transfer should succeed");
    assert_eq!(transfer.previous_owner.role, MembershipRole::Admin);
    assert_eq!(transfer.new_owner.role, MembershipRole::Owner);

    let leave = runtime
        .leave_conversation(LeaveConversationCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_group_transfer_owner".into(),
            principal_id: "1".into(),
        })
        .expect("previous owner should be able to leave after transfer");
    assert_eq!(leave.state, MembershipState::Left);

    let members = runtime
        .list_members("100001", "default", "c_group_transfer_owner")
        .expect("list members should succeed");
    assert_eq!(members.len(), 1);
    assert_eq!(members[0].principal_id, "1043");
    assert_eq!(members[0].role, MembershipRole::Owner);
}

#[test]
fn test_owner_transfer_event_preserves_system_actor_kind() {
    let journal = InMemoryJournal::default();
    let runtime = ConversationRuntime::new(journal.clone());

    runtime
        .create_conversation_with_creator_kind(
            CreateConversationCommand {
                tenant_id: "100001".into(),
                organization_id: "0".into(),
                conversation_id: "c_group_owner_system".into(),
                creator_id: "svc_ops".into(),
                conversation_type: "group".into(),
            },
            "system",
        )
        .expect("system actor should be able to create group conversation");

    let member = runtime
        .add_member(AddConversationMemberCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_group_owner_system".into(),
            principal_id: "1043".into(),
            principal_kind: "user".into(),
            role: MembershipRole::Member,
            invited_by: "svc_ops".into(),
        })
        .expect("system owner should be able to add member");

    runtime
        .transfer_conversation_owner(TransferConversationOwnerCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_group_owner_system".into(),
            target_member_id: member.member_id,
            transferred_by: "svc_ops".into(),
        })
        .expect("system owner should be able to transfer ownership");

    let transfer_event = journal
        .recorded()
        .into_iter()
        .find(|event| {
            event.event_type == "conversation.owner_transferred"
                && event.aggregate_id == "c_group_owner_system"
        })
        .expect("owner transfer event should exist");
    assert_eq!(transfer_event.actor.actor_id, "svc_ops");
    assert_eq!(transfer_event.actor.actor_kind, "system");
}

#[test]
fn test_group_admin_cannot_transfer_ownership() {
    let journal = InMemoryJournal::default();
    let runtime = ConversationRuntime::new(journal);

    runtime
        .create_conversation(CreateConversationCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_group_transfer_owner_forbidden".into(),
            creator_id: "1".into(),
            conversation_type: "group".into(),
        })
        .expect("create conversation should succeed");

    let admin = runtime
        .add_member(AddConversationMemberCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_group_transfer_owner_forbidden".into(),
            principal_id: "1003".into(),
            principal_kind: "user".into(),
            role: MembershipRole::Admin,
            invited_by: "1".into(),
        })
        .expect("owner should be able to add admin");

    let member = runtime
        .add_member(AddConversationMemberCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_group_transfer_owner_forbidden".into(),
            principal_id: "1043".into(),
            principal_kind: "user".into(),
            role: MembershipRole::Member,
            invited_by: "1".into(),
        })
        .expect("owner should be able to add member");

    let transfer = runtime.transfer_conversation_owner(TransferConversationOwnerCommand {
        tenant_id: "100001".into(),
        organization_id: "0".into(),
        conversation_id: "c_group_transfer_owner_forbidden".into(),
        target_member_id: member.member_id,
        transferred_by: admin.principal_id,
    });
    assert!(matches!(transfer, Err(RuntimeError::PermissionDenied(_))));
}

#[test]
fn test_direct_conversation_rejects_owner_transfer() {
    let journal = InMemoryJournal::default();
    let runtime = ConversationRuntime::new(journal);

    runtime
        .create_conversation(CreateConversationCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_direct_transfer_owner".into(),
            creator_id: "1".into(),
            conversation_type: "direct".into(),
        })
        .expect("create conversation should succeed");

    let peer = runtime
        .add_member(AddConversationMemberCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_direct_transfer_owner".into(),
            principal_id: "1057".into(),
            principal_kind: "user".into(),
            role: MembershipRole::Member,
            invited_by: "1".into(),
        })
        .expect("owner should be able to add direct peer");

    let transfer = runtime.transfer_conversation_owner(TransferConversationOwnerCommand {
        tenant_id: "100001".into(),
        organization_id: "0".into(),
        conversation_id: "c_direct_transfer_owner".into(),
        target_member_id: peer.member_id,
        transferred_by: "1".into(),
    });
    assert!(matches!(transfer, Err(RuntimeError::PermissionDenied(_))));
}

#[test]
fn test_group_owner_can_change_non_owner_member_roles() {
    let journal = InMemoryJournal::default();
    let runtime = ConversationRuntime::new(journal.clone());

    runtime
        .create_conversation(CreateConversationCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_group_role_change".into(),
            creator_id: "1".into(),
            conversation_type: "group".into(),
        })
        .expect("create conversation should succeed");

    let member = runtime
        .add_member(AddConversationMemberCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_group_role_change".into(),
            principal_id: "1043".into(),
            principal_kind: "user".into(),
            role: MembershipRole::Member,
            invited_by: "1".into(),
        })
        .expect("owner should be able to add member");

    let promote = runtime
        .change_conversation_member_role(ChangeConversationMemberRoleCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_group_role_change".into(),
            target_member_id: member.member_id.clone(),
            new_role: MembershipRole::Admin,
            changed_by: "1".into(),
        })
        .expect("owner should be able to promote member");
    assert_eq!(promote.previous_member.role, MembershipRole::Member);
    assert_eq!(promote.updated_member.role, MembershipRole::Admin);

    let demote = runtime
        .change_conversation_member_role(ChangeConversationMemberRoleCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_group_role_change".into(),
            target_member_id: member.member_id.clone(),
            new_role: MembershipRole::Guest,
            changed_by: "1".into(),
        })
        .expect("owner should be able to demote admin");
    assert_eq!(demote.previous_member.role, MembershipRole::Admin);
    assert_eq!(demote.updated_member.role, MembershipRole::Guest);
    assert_ne!(demote.event_id, promote.event_id);

    let members = runtime
        .list_members("100001", "default", "c_group_role_change")
        .expect("list members should succeed");
    let target = members
        .into_iter()
        .find(|item| item.principal_id == "1043")
        .expect("target member should exist");
    assert_eq!(target.role, MembershipRole::Guest);

    let events = journal.recorded();
    assert_eq!(
        events
            .iter()
            .filter(|event| event.event_type == "conversation.member_role_changed")
            .count(),
        2
    );
}

#[test]
fn test_member_role_changed_event_preserves_system_actor_kind() {
    let journal = InMemoryJournal::default();
    let runtime = ConversationRuntime::new(journal.clone());

    runtime
        .create_conversation_with_creator_kind(
            CreateConversationCommand {
                tenant_id: "100001".into(),
                organization_id: "0".into(),
                conversation_id: "c_group_role_system".into(),
                creator_id: "svc_ops".into(),
                conversation_type: "group".into(),
            },
            "system",
        )
        .expect("system actor should be able to create group conversation");

    let member = runtime
        .add_member(AddConversationMemberCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_group_role_system".into(),
            principal_id: "1043".into(),
            principal_kind: "user".into(),
            role: MembershipRole::Member,
            invited_by: "svc_ops".into(),
        })
        .expect("system owner should be able to add member");

    runtime
        .change_conversation_member_role(ChangeConversationMemberRoleCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_group_role_system".into(),
            target_member_id: member.member_id,
            new_role: MembershipRole::Admin,
            changed_by: "svc_ops".into(),
        })
        .expect("system owner should be able to change member role");

    let role_changed_event = journal
        .recorded()
        .into_iter()
        .find(|event| {
            event.event_type == "conversation.member_role_changed"
                && event.aggregate_id == "c_group_role_system"
        })
        .expect("member role changed event should exist");
    assert_eq!(role_changed_event.actor.actor_id, "svc_ops");
    assert_eq!(role_changed_event.actor.actor_kind, "system");
}

#[test]
fn test_group_admin_cannot_change_member_roles() {
    let journal = InMemoryJournal::default();
    let runtime = ConversationRuntime::new(journal);

    runtime
        .create_conversation(CreateConversationCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_group_role_change_forbidden".into(),
            creator_id: "1".into(),
            conversation_type: "group".into(),
        })
        .expect("create conversation should succeed");

    let admin = runtime
        .add_member(AddConversationMemberCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_group_role_change_forbidden".into(),
            principal_id: "1003".into(),
            principal_kind: "user".into(),
            role: MembershipRole::Admin,
            invited_by: "1".into(),
        })
        .expect("owner should be able to add admin");

    let member = runtime
        .add_member(AddConversationMemberCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_group_role_change_forbidden".into(),
            principal_id: "1043".into(),
            principal_kind: "user".into(),
            role: MembershipRole::Member,
            invited_by: "1".into(),
        })
        .expect("owner should be able to add member");

    let change = runtime.change_conversation_member_role(ChangeConversationMemberRoleCommand {
        tenant_id: "100001".into(),
        organization_id: "0".into(),
        conversation_id: "c_group_role_change_forbidden".into(),
        target_member_id: member.member_id,
        new_role: MembershipRole::Guest,
        changed_by: admin.principal_id,
    });
    assert!(matches!(change, Err(RuntimeError::PermissionDenied(_))));
}

#[test]
fn test_group_role_change_rejects_owner_target_and_direct_conversation() {
    let journal = InMemoryJournal::default();
    let runtime = ConversationRuntime::new(journal);

    runtime
        .create_conversation(CreateConversationCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_group_role_change_owner_target".into(),
            creator_id: "1".into(),
            conversation_type: "group".into(),
        })
        .expect("group create conversation should succeed");

    let owner_target =
        runtime.change_conversation_member_role(ChangeConversationMemberRoleCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_group_role_change_owner_target".into(),
            target_member_id: "cm_c_group_role_change_owner_target_user_1".into(),
            new_role: MembershipRole::Admin,
            changed_by: "1".into(),
        });
    assert!(matches!(
        owner_target,
        Err(RuntimeError::PermissionDenied(_))
    ));

    runtime
        .create_conversation(CreateConversationCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_direct_role_change".into(),
            creator_id: "1".into(),
            conversation_type: "direct".into(),
        })
        .expect("direct create conversation should succeed");

    let peer = runtime
        .add_member(AddConversationMemberCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_direct_role_change".into(),
            principal_id: "1057".into(),
            principal_kind: "user".into(),
            role: MembershipRole::Member,
            invited_by: "1".into(),
        })
        .expect("owner should be able to add direct peer");

    let direct_change =
        runtime.change_conversation_member_role(ChangeConversationMemberRoleCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_direct_role_change".into(),
            target_member_id: peer.member_id,
            new_role: MembershipRole::Guest,
            changed_by: "1".into(),
        });
    assert!(matches!(
        direct_change,
        Err(RuntimeError::PermissionDenied(_))
    ));
}

#[test]
fn test_stale_member_id_cannot_change_rejoined_member_role() {
    let journal = InMemoryJournal::default();
    let runtime = ConversationRuntime::new(journal);

    runtime
        .create_conversation(CreateConversationCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_group_role_change_rejoin_guard".into(),
            creator_id: "1".into(),
            conversation_type: "group".into(),
        })
        .expect("create conversation should succeed");

    let first_join = runtime
        .add_member(AddConversationMemberCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_group_role_change_rejoin_guard".into(),
            principal_id: "1043".into(),
            principal_kind: "user".into(),
            role: MembershipRole::Member,
            invited_by: "1".into(),
        })
        .expect("owner should be able to add member");

    runtime
        .leave_conversation(LeaveConversationCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_group_role_change_rejoin_guard".into(),
            principal_id: "1043".into(),
        })
        .expect("member should be able to leave");

    let rejoined = runtime
        .add_member(AddConversationMemberCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_group_role_change_rejoin_guard".into(),
            principal_id: "1043".into(),
            principal_kind: "user".into(),
            role: MembershipRole::Member,
            invited_by: "1".into(),
        })
        .expect("owner should be able to re-add left member");
    assert_ne!(rejoined.member_id, first_join.member_id);

    let change_stale =
        runtime.change_conversation_member_role(ChangeConversationMemberRoleCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_group_role_change_rejoin_guard".into(),
            target_member_id: first_join.member_id.clone(),
            new_role: MembershipRole::Admin,
            changed_by: "1".into(),
        });
    assert!(matches!(
        change_stale,
        Err(RuntimeError::MemberNotFound(member_id)) if member_id == first_join.member_id
    ));

    let members = runtime
        .list_members("100001", "default", "c_group_role_change_rejoin_guard")
        .expect("list members should succeed");
    let target = members
        .into_iter()
        .find(|item| item.principal_id == "1043")
        .expect("target member should exist");
    assert_eq!(target.member_id, rejoined.member_id);
    assert_eq!(target.role, MembershipRole::Member);
}

#[test]
fn test_left_member_rejoin_creates_new_membership_episode() {
    let journal = InMemoryJournal::default();
    let runtime = ConversationRuntime::new(journal);

    runtime
        .create_conversation(CreateConversationCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_group_rejoin".into(),
            creator_id: "1".into(),
            conversation_type: "group".into(),
        })
        .expect("create conversation should succeed");

    let first_join = runtime
        .add_member(AddConversationMemberCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_group_rejoin".into(),
            principal_id: "1043".into(),
            principal_kind: "user".into(),
            role: MembershipRole::Member,
            invited_by: "1".into(),
        })
        .expect("owner should be able to add member");

    let left_member = runtime
        .leave_conversation(LeaveConversationCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_group_rejoin".into(),
            principal_id: "1043".into(),
        })
        .expect("member should be able to leave");
    assert_eq!(left_member.member_id, first_join.member_id);
    assert_eq!(left_member.state, MembershipState::Left);

    let rejoined = runtime
        .add_member(AddConversationMemberCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_group_rejoin".into(),
            principal_id: "1043".into(),
            principal_kind: "user".into(),
            role: MembershipRole::Member,
            invited_by: "1".into(),
        })
        .expect("owner should be able to re-add left member");

    assert_ne!(rejoined.member_id, first_join.member_id);
    assert_eq!(rejoined.state, MembershipState::Joined);
    assert!(rejoined.removed_at.is_none());

    let view = runtime
        .read_cursor_view("100001", "default", "c_group_rejoin", "1043")
        .expect("rejoined member read cursor view should succeed");
    assert_eq!(view.member_id, rejoined.member_id);
    assert_eq!(view.read_seq, 0);
}

#[test]
fn test_stale_member_id_cannot_remove_rejoined_active_member() {
    let journal = InMemoryJournal::default();
    let runtime = ConversationRuntime::new(journal);

    runtime
        .create_conversation(CreateConversationCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_group_rejoin_remove_guard".into(),
            creator_id: "1".into(),
            conversation_type: "group".into(),
        })
        .expect("create conversation should succeed");

    let first_join = runtime
        .add_member(AddConversationMemberCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_group_rejoin_remove_guard".into(),
            principal_id: "1043".into(),
            principal_kind: "user".into(),
            role: MembershipRole::Member,
            invited_by: "1".into(),
        })
        .expect("owner should be able to add member");

    runtime
        .leave_conversation(LeaveConversationCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_group_rejoin_remove_guard".into(),
            principal_id: "1043".into(),
        })
        .expect("member should be able to leave");

    let rejoined = runtime
        .add_member(AddConversationMemberCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_group_rejoin_remove_guard".into(),
            principal_id: "1043".into(),
            principal_kind: "user".into(),
            role: MembershipRole::Member,
            invited_by: "1".into(),
        })
        .expect("owner should be able to re-add left member");
    assert_ne!(rejoined.member_id, first_join.member_id);

    let remove_stale = runtime.remove_member(RemoveConversationMemberCommand {
        tenant_id: "100001".into(),
        organization_id: "0".into(),
        conversation_id: "c_group_rejoin_remove_guard".into(),
        member_id: first_join.member_id.clone(),
        removed_by: "1".into(),
    });
    assert!(matches!(
        remove_stale,
        Err(RuntimeError::MemberNotFound(member_id)) if member_id == first_join.member_id
    ));

    let members = runtime
        .list_members("100001", "default", "c_group_rejoin_remove_guard")
        .expect("list members should succeed");
    assert_eq!(members.len(), 2);
    assert!(
        members
            .iter()
            .any(|member| member.member_id == rejoined.member_id)
    );
    assert!(members.iter().all(ConversationMember::is_active));
}

#[test]
fn test_posted_message_timestamps_advance_between_distinct_messages() {
    let journal = InMemoryJournal::default();
    let runtime = ConversationRuntime::new(journal.clone());

    runtime
        .create_conversation(CreateConversationCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_posted_time".into(),
            creator_id: "1".into(),
            conversation_type: "group".into(),
        })
        .expect("create conversation should succeed");

    runtime
        .post_message(PostMessageCommand {
            tenant_id: "100001".into(),
           organization_id: "0".into(),
            conversation_id: "c_posted_time".into(),
            sender: Sender {
                id: "1".into(),
                kind: "user".into(),
                member_id: None,
                device_id: Some("d_demo".into()),
                session_id: Some("s_demo".into()),
                metadata: Default::default(),
            },
            client_msg_id: Some("client_time_1".into()),
            message_type: MessageType::Standard,
            body: MessageBody {
                summary: Some("one".into()),
                parts: vec![ContentPart::text("one")],
                render_hints: Default::default(),
                reply_to: None,
            },
        })
        .expect("first message should succeed");

    sleep(Duration::from_millis(5));

    runtime
        .post_message(PostMessageCommand {
            tenant_id: "100001".into(),
           organization_id: "0".into(),
            conversation_id: "c_posted_time".into(),
            sender: Sender {
                id: "1".into(),
                kind: "user".into(),
                member_id: None,
                device_id: Some("d_demo".into()),
                session_id: Some("s_demo".into()),
                metadata: Default::default(),
            },
            client_msg_id: Some("client_time_2".into()),
            message_type: MessageType::Standard,
            body: MessageBody {
                summary: Some("two".into()),
                parts: vec![ContentPart::text("two")],
                render_hints: Default::default(),
                reply_to: None,
            },
        })
        .expect("second message should succeed");

    let events = journal.recorded();
    let posted_events: Vec<_> = events
        .iter()
        .filter(|event| event.event_type == "message.posted")
        .collect();
    assert_eq!(posted_events.len(), 2);
    assert_ne!(
        posted_events[0].occurred_at, posted_events[1].occurred_at,
        "separate posted messages must not reuse a fixed occurred_at timestamp"
    );
    assert_ne!(
        posted_events[0].committed_at, posted_events[1].committed_at,
        "separate posted messages must not reuse a fixed committed_at timestamp"
    );
}

#[test]
fn test_read_cursor_timestamps_advance_between_distinct_updates() {
    let journal = InMemoryJournal::default();
    let runtime = ConversationRuntime::new(journal.clone());

    runtime
        .create_conversation(CreateConversationCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_cursor_time".into(),
            creator_id: "1".into(),
            conversation_type: "group".into(),
        })
        .expect("create conversation should succeed");

    runtime
        .post_message(PostMessageCommand {
            tenant_id: "100001".into(),
           organization_id: "0".into(),
            conversation_id: "c_cursor_time".into(),
            sender: Sender {
                id: "1".into(),
                kind: "user".into(),
                member_id: None,
                device_id: None,
                session_id: Some("s_demo".into()),
                metadata: Default::default(),
            },
            client_msg_id: Some("client_cursor_1".into()),
            message_type: MessageType::Standard,
            body: MessageBody {
                summary: Some("one".into()),
                parts: vec![ContentPart::text("one")],
                render_hints: Default::default(),
                reply_to: None,
            },
        })
        .expect("first message should succeed");
    runtime
        .post_message(PostMessageCommand {
            tenant_id: "100001".into(),
           organization_id: "0".into(),
            conversation_id: "c_cursor_time".into(),
            sender: Sender {
                id: "1".into(),
                kind: "user".into(),
                member_id: None,
                device_id: None,
                session_id: Some("s_demo".into()),
                metadata: Default::default(),
            },
            client_msg_id: Some("client_cursor_2".into()),
            message_type: MessageType::Standard,
            body: MessageBody {
                summary: Some("two".into()),
                parts: vec![ContentPart::text("two")],
                render_hints: Default::default(),
                reply_to: None,
            },
        })
        .expect("second message should succeed");

    let first = runtime
        .update_read_cursor(UpdateReadCursorCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_cursor_time".into(),
            principal_id: "1".into(),
            read_seq: 1,
            last_read_message_id: Some("msg_c_cursor_time_1".into()),
        })
        .expect("first read cursor update should succeed");

    sleep(Duration::from_millis(5));

    let second = runtime
        .update_read_cursor(UpdateReadCursorCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_cursor_time".into(),
            principal_id: "1".into(),
            read_seq: 2,
            last_read_message_id: Some("msg_c_cursor_time_2".into()),
        })
        .expect("second read cursor update should succeed");

    assert_ne!(
        first.updated_at, second.updated_at,
        "separate read cursor updates must not reuse a fixed updated_at timestamp"
    );

    let events = journal.recorded();
    let cursor_events: Vec<_> = events
        .iter()
        .filter(|event| event.event_type == "conversation.read_cursor_updated")
        .collect();
    assert_eq!(cursor_events.len(), 2);
    assert_ne!(
        cursor_events[0].occurred_at, cursor_events[1].occurred_at,
        "separate read cursor updates must not reuse a fixed envelope occurred_at timestamp"
    );
}

#[test]
fn test_membership_timestamps_advance_between_distinct_join_and_remove_mutations() {
    let journal = InMemoryJournal::default();
    let runtime = ConversationRuntime::new(journal);

    runtime
        .create_conversation(CreateConversationCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_member_time".into(),
            creator_id: "1".into(),
            conversation_type: "group".into(),
        })
        .expect("create conversation should succeed");

    let first = runtime
        .add_member(AddConversationMemberCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_member_time".into(),
            principal_id: "1062".into(),
            principal_kind: "user".into(),
            role: MembershipRole::Member,
            invited_by: "1".into(),
        })
        .expect("first add member should succeed");

    sleep(Duration::from_millis(5));

    let second = runtime
        .add_member(AddConversationMemberCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_member_time".into(),
            principal_id: "1063".into(),
            principal_kind: "user".into(),
            role: MembershipRole::Member,
            invited_by: "1".into(),
        })
        .expect("second add member should succeed");

    assert_ne!(
        first.joined_at, second.joined_at,
        "separate joined members must not reuse a fixed joined_at timestamp"
    );

    let removed_first = runtime
        .remove_member(RemoveConversationMemberCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_member_time".into(),
            member_id: first.member_id.clone(),
            removed_by: "1".into(),
        })
        .expect("first remove member should succeed");

    sleep(Duration::from_millis(5));

    let removed_second = runtime
        .remove_member(RemoveConversationMemberCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_member_time".into(),
            member_id: second.member_id.clone(),
            removed_by: "1".into(),
        })
        .expect("second remove member should succeed");

    assert_ne!(
        removed_first.removed_at, removed_second.removed_at,
        "separate removed members must not reuse a fixed removed_at timestamp"
    );
}

#[test]
fn test_message_edit_and_recall_timestamps_advance_between_distinct_mutations() {
    let journal = InMemoryJournal::default();
    let runtime = ConversationRuntime::new(journal.clone());

    runtime
        .create_conversation(CreateConversationCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_mutation_time".into(),
            creator_id: "1".into(),
            conversation_type: "group".into(),
        })
        .expect("create conversation should succeed");

    let first = runtime
        .post_message(PostMessageCommand {
            tenant_id: "100001".into(),
           organization_id: "0".into(),
            conversation_id: "c_mutation_time".into(),
            sender: Sender {
                id: "1".into(),
                kind: "user".into(),
                member_id: None,
                device_id: Some("d_demo".into()),
                session_id: Some("s_demo".into()),
                metadata: Default::default(),
            },
            client_msg_id: Some("client_mutation_1".into()),
            message_type: MessageType::Standard,
            body: MessageBody {
                summary: Some("one".into()),
                parts: vec![ContentPart::text("one")],
                render_hints: Default::default(),
                reply_to: None,
            },
        })
        .expect("first message should succeed");
    let second = runtime
        .post_message(PostMessageCommand {
            tenant_id: "100001".into(),
           organization_id: "0".into(),
            conversation_id: "c_mutation_time".into(),
            sender: Sender {
                id: "1".into(),
                kind: "user".into(),
                member_id: None,
                device_id: Some("d_demo".into()),
                session_id: Some("s_demo".into()),
                metadata: Default::default(),
            },
            client_msg_id: Some("client_mutation_2".into()),
            message_type: MessageType::Standard,
            body: MessageBody {
                summary: Some("two".into()),
                parts: vec![ContentPart::text("two")],
                render_hints: Default::default(),
                reply_to: None,
            },
        })
        .expect("second message should succeed");

    runtime
        .edit_message(EditMessageCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            message_id: first.message_id.clone(),
            editor: Sender {
                id: "1".into(),
                kind: "user".into(),
                member_id: None,
                device_id: Some("d_demo".into()),
                session_id: Some("s_demo".into()),
                metadata: Default::default(),
            },
            body: MessageBody {
                summary: Some("edited one".into()),
                parts: vec![ContentPart::text("edited one")],
                render_hints: Default::default(),
                reply_to: None,
            },
        })
        .expect("first edit should succeed");

    sleep(Duration::from_millis(5));

    runtime
        .edit_message(EditMessageCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            message_id: second.message_id.clone(),
            editor: Sender {
                id: "1".into(),
                kind: "user".into(),
                member_id: None,
                device_id: Some("d_demo".into()),
                session_id: Some("s_demo".into()),
                metadata: Default::default(),
            },
            body: MessageBody {
                summary: Some("edited two".into()),
                parts: vec![ContentPart::text("edited two")],
                render_hints: Default::default(),
                reply_to: None,
            },
        })
        .expect("second edit should succeed");

    runtime
        .recall_message(RecallMessageCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            message_id: first.message_id.clone(),
            recalled_by: Sender {
                id: "1".into(),
                kind: "user".into(),
                member_id: None,
                device_id: Some("d_demo".into()),
                session_id: Some("s_demo".into()),
                metadata: Default::default(),
            },
        })
        .expect("first recall should succeed");

    sleep(Duration::from_millis(5));

    runtime
        .recall_message(RecallMessageCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            message_id: second.message_id.clone(),
            recalled_by: Sender {
                id: "1".into(),
                kind: "user".into(),
                member_id: None,
                device_id: Some("d_demo".into()),
                session_id: Some("s_demo".into()),
                metadata: Default::default(),
            },
        })
        .expect("second recall should succeed");

    let events = journal.recorded();
    let edited_events: Vec<_> = events
        .iter()
        .filter(|event| event.event_type == "message.edited")
        .collect();
    assert_eq!(edited_events.len(), 2);
    assert_ne!(
        edited_events[0].occurred_at, edited_events[1].occurred_at,
        "separate edits must not reuse a fixed edited_at timestamp"
    );

    let recalled_events: Vec<_> = events
        .iter()
        .filter(|event| event.event_type == "message.recalled")
        .collect();
    assert_eq!(recalled_events.len(), 2);
    assert_ne!(
        recalled_events[0].occurred_at, recalled_events[1].occurred_at,
        "separate recalls must not reuse a fixed recalled_at timestamp"
    );
}

#[test]
fn test_add_and_remove_message_reaction_emit_events_and_are_idempotent() {
    let journal = InMemoryJournal::default();
    let runtime = ConversationRuntime::new(journal.clone());

    runtime
        .create_conversation(CreateConversationCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_reaction_flow".into(),
            creator_id: "1".into(),
            conversation_type: "group".into(),
        })
        .expect("create conversation should succeed");

    let posted = runtime
        .post_message(PostMessageCommand {
            tenant_id: "100001".into(),
           organization_id: "0".into(),
            conversation_id: "c_reaction_flow".into(),
            sender: Sender {
                id: "1".into(),
                kind: "user".into(),
                member_id: None,
                device_id: Some("d_owner".into()),
                session_id: Some("s_owner".into()),
                metadata: Default::default(),
            },
            client_msg_id: Some("client_reaction_flow".into()),
            message_type: MessageType::Standard,
            body: MessageBody {
                summary: Some("reaction target".into()),
                parts: vec![ContentPart::text("reaction target")],
                render_hints: Default::default(),
                reply_to: None,
            },
        })
        .expect("post message should succeed");

    let added = runtime
        .add_message_reaction(AddMessageReactionCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            message_id: posted.message_id.clone(),
            reaction_key: "thumbs_up".into(),
            reacted_by: Sender {
                id: "1".into(),
                kind: "user".into(),
                member_id: None,
                device_id: Some("d_owner".into()),
                session_id: Some("s_owner".into()),
                metadata: Default::default(),
            },
        })
        .expect("add reaction should succeed");
    assert!(added.changed);
    assert_eq!(added.message_id, posted.message_id);
    assert_eq!(added.message_seq, 1);
    assert_eq!(added.reaction_key, "thumbs_up");

    let duplicate_add = runtime
        .add_message_reaction(AddMessageReactionCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            message_id: posted.message_id.clone(),
            reaction_key: "thumbs_up".into(),
            reacted_by: Sender {
                id: "1".into(),
                kind: "user".into(),
                member_id: None,
                device_id: Some("d_owner".into()),
                session_id: Some("s_owner".into()),
                metadata: Default::default(),
            },
        })
        .expect("duplicate add should be idempotent");
    assert!(!duplicate_add.changed);

    let removed = runtime
        .remove_message_reaction(RemoveMessageReactionCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            message_id: posted.message_id.clone(),
            reaction_key: "thumbs_up".into(),
            removed_by: Sender {
                id: "1".into(),
                kind: "user".into(),
                member_id: None,
                device_id: Some("d_owner".into()),
                session_id: Some("s_owner".into()),
                metadata: Default::default(),
            },
        })
        .expect("remove reaction should succeed");
    assert!(removed.changed);
    assert_eq!(removed.message_id, posted.message_id);
    assert_eq!(removed.message_seq, 1);
    assert_eq!(removed.reaction_key, "thumbs_up");

    let duplicate_remove = runtime
        .remove_message_reaction(RemoveMessageReactionCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            message_id: posted.message_id.clone(),
            reaction_key: "thumbs_up".into(),
            removed_by: Sender {
                id: "1".into(),
                kind: "user".into(),
                member_id: None,
                device_id: Some("d_owner".into()),
                session_id: Some("s_owner".into()),
                metadata: Default::default(),
            },
        })
        .expect("duplicate remove should be idempotent");
    assert!(!duplicate_remove.changed);

    let events = journal.recorded();
    assert_eq!(
        events
            .iter()
            .filter(|event| event.event_type == "message.reaction_added")
            .count(),
        1
    );
    assert_eq!(
        events
            .iter()
            .filter(|event| event.event_type == "message.reaction_removed")
            .count(),
        1
    );
}

#[test]
fn test_pin_and_unpin_message_emit_events_and_require_privileged_member() {
    let journal = InMemoryJournal::default();
    let runtime = ConversationRuntime::new(journal.clone());

    runtime
        .create_conversation(CreateConversationCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_pin_flow".into(),
            creator_id: "1".into(),
            conversation_type: "group".into(),
        })
        .expect("create conversation should succeed");

    runtime
        .add_member(AddConversationMemberCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_pin_flow".into(),
            principal_id: "1043".into(),
            principal_kind: "user".into(),
            role: MembershipRole::Member,
            invited_by: "1".into(),
        })
        .expect("add member should succeed");

    let posted = runtime
        .post_message(PostMessageCommand {
            tenant_id: "100001".into(),
           organization_id: "0".into(),
            conversation_id: "c_pin_flow".into(),
            sender: Sender {
                id: "1043".into(),
                kind: "user".into(),
                member_id: None,
                device_id: None,
                session_id: Some("s_member".into()),
                metadata: Default::default(),
            },
            client_msg_id: Some("client_pin_flow".into()),
            message_type: MessageType::Standard,
            body: MessageBody {
                summary: Some("pin target".into()),
                parts: vec![ContentPart::text("pin target")],
                render_hints: Default::default(),
                reply_to: None,
            },
        })
        .expect("member post should succeed");

    let denied_pin = runtime.pin_message(PinMessageCommand {
        tenant_id: "100001".into(),
        organization_id: "0".into(),
        message_id: posted.message_id.clone(),
        pinned_by: Sender {
            id: "1043".into(),
            kind: "user".into(),
            member_id: None,
            device_id: None,
            session_id: Some("s_member".into()),
            metadata: Default::default(),
        },
    });
    assert!(matches!(denied_pin, Err(RuntimeError::PermissionDenied(_))));

    let pinned = runtime
        .pin_message(PinMessageCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            message_id: posted.message_id.clone(),
            pinned_by: Sender {
                id: "1".into(),
                kind: "user".into(),
                member_id: None,
                device_id: Some("d_owner".into()),
                session_id: Some("s_owner".into()),
                metadata: Default::default(),
            },
        })
        .expect("owner pin should succeed");
    assert!(pinned.changed);
    assert_eq!(pinned.message_id, posted.message_id);
    assert_eq!(pinned.message_seq, 1);

    let duplicate_pin = runtime
        .pin_message(PinMessageCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            message_id: posted.message_id.clone(),
            pinned_by: Sender {
                id: "1".into(),
                kind: "user".into(),
                member_id: None,
                device_id: Some("d_owner".into()),
                session_id: Some("s_owner".into()),
                metadata: Default::default(),
            },
        })
        .expect("duplicate pin should be idempotent");
    assert!(!duplicate_pin.changed);

    let unpinned = runtime
        .unpin_message(UnpinMessageCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            message_id: posted.message_id.clone(),
            unpinned_by: Sender {
                id: "1".into(),
                kind: "user".into(),
                member_id: None,
                device_id: Some("d_owner".into()),
                session_id: Some("s_owner".into()),
                metadata: Default::default(),
            },
        })
        .expect("owner unpin should succeed");
    assert!(unpinned.changed);

    let duplicate_unpin = runtime
        .unpin_message(UnpinMessageCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            message_id: posted.message_id.clone(),
            unpinned_by: Sender {
                id: "1".into(),
                kind: "user".into(),
                member_id: None,
                device_id: Some("d_owner".into()),
                session_id: Some("s_owner".into()),
                metadata: Default::default(),
            },
        })
        .expect("duplicate unpin should be idempotent");
    assert!(!duplicate_unpin.changed);

    let events = journal.recorded();
    assert_eq!(
        events
            .iter()
            .filter(|event| event.event_type == "message.pin_added")
            .count(),
        1
    );
    assert_eq!(
        events
            .iter()
            .filter(|event| event.event_type == "message.pin_removed")
            .count(),
        1
    );
}

#[test]
fn test_reaction_and_pin_state_survive_recovery_replay() {
    let source_journal = InMemoryJournal::default();
    let source_runtime = ConversationRuntime::new(source_journal.clone());

    source_runtime
        .create_conversation(CreateConversationCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_reaction_pin_replay".into(),
            creator_id: "1".into(),
            conversation_type: "group".into(),
        })
        .expect("create conversation should succeed");

    let posted = source_runtime
        .post_message(PostMessageCommand {
            tenant_id: "100001".into(),
           organization_id: "0".into(),
            conversation_id: "c_reaction_pin_replay".into(),
            sender: Sender {
                id: "1".into(),
                kind: "user".into(),
                member_id: None,
                device_id: Some("d_owner".into()),
                session_id: Some("s_owner".into()),
                metadata: Default::default(),
            },
            client_msg_id: Some("client_replay_reaction_pin".into()),
            message_type: MessageType::Standard,
            body: MessageBody {
                summary: Some("replay target".into()),
                parts: vec![ContentPart::text("replay target")],
                render_hints: Default::default(),
                reply_to: None,
            },
        })
        .expect("post message should succeed");

    source_runtime
        .add_message_reaction(AddMessageReactionCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            message_id: posted.message_id.clone(),
            reaction_key: "thumbs_up".into(),
            reacted_by: Sender {
                id: "1".into(),
                kind: "user".into(),
                member_id: None,
                device_id: Some("d_owner".into()),
                session_id: Some("s_owner".into()),
                metadata: Default::default(),
            },
        })
        .expect("add reaction should succeed");
    source_runtime
        .pin_message(PinMessageCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            message_id: posted.message_id.clone(),
            pinned_by: Sender {
                id: "1".into(),
                kind: "user".into(),
                member_id: None,
                device_id: Some("d_owner".into()),
                session_id: Some("s_owner".into()),
                metadata: Default::default(),
            },
        })
        .expect("pin should succeed");

    let replay_journal = InMemoryJournal::default();
    let replay_runtime = ConversationRuntime::new(replay_journal.clone());
    for envelope in source_journal.recorded() {
        replay_runtime
            .apply_recovered_envelope(&envelope)
            .expect("replay should succeed");
    }

    let duplicate_reaction = replay_runtime
        .add_message_reaction(AddMessageReactionCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            message_id: posted.message_id.clone(),
            reaction_key: "thumbs_up".into(),
            reacted_by: Sender {
                id: "1".into(),
                kind: "user".into(),
                member_id: None,
                device_id: Some("d_owner".into()),
                session_id: Some("s_owner".into()),
                metadata: Default::default(),
            },
        })
        .expect("replayed reaction should be idempotent");
    assert!(!duplicate_reaction.changed);

    let duplicate_pin = replay_runtime
        .pin_message(PinMessageCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            message_id: posted.message_id.clone(),
            pinned_by: Sender {
                id: "1".into(),
                kind: "user".into(),
                member_id: None,
                device_id: Some("d_owner".into()),
                session_id: Some("s_owner".into()),
                metadata: Default::default(),
            },
        })
        .expect("replayed pin should be idempotent");
    assert!(!duplicate_pin.changed);

    let removed = replay_runtime
        .remove_message_reaction(RemoveMessageReactionCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            message_id: posted.message_id.clone(),
            reaction_key: "thumbs_up".into(),
            removed_by: Sender {
                id: "1".into(),
                kind: "user".into(),
                member_id: None,
                device_id: Some("d_owner".into()),
                session_id: Some("s_owner".into()),
                metadata: Default::default(),
            },
        })
        .expect("remove reaction after replay should succeed");
    assert!(removed.changed);

    let unpinned = replay_runtime
        .unpin_message(UnpinMessageCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            message_id: posted.message_id.clone(),
            unpinned_by: Sender {
                id: "1".into(),
                kind: "user".into(),
                member_id: None,
                device_id: Some("d_owner".into()),
                session_id: Some("s_owner".into()),
                metadata: Default::default(),
            },
        })
        .expect("unpin after replay should succeed");
    assert!(unpinned.changed);

    let replay_events = replay_journal.recorded();
    assert_eq!(
        replay_events
            .iter()
            .filter(|event| event.event_type == "message.reaction_added")
            .count(),
        0
    );
    assert_eq!(
        replay_events
            .iter()
            .filter(|event| event.event_type == "message.pin_added")
            .count(),
        0
    );
    assert_eq!(
        replay_events
            .iter()
            .filter(|event| event.event_type == "message.reaction_removed")
            .count(),
        1
    );
    assert_eq!(
        replay_events
            .iter()
            .filter(|event| event.event_type == "message.pin_removed")
            .count(),
        1
    );
}

#[test]
fn test_bind_direct_chat_conversation_creates_business_bound_direct_runtime() {
    let journal = InMemoryJournal::default();
    let runtime = ConversationRuntime::new(journal.clone());

    let created = runtime
        .bind_direct_chat_conversation_with_binder_kind(
            canonical_bind_direct_chat_command("100001", "actor_a", "actor_b"),
            "system",
        )
        .expect("direct chat binding should succeed");

    let conversation_id = created.conversation_id.clone();
    let direct_chat_id = runtime
        .conversation_business_binding("100001", "default", conversation_id.as_str())
        .expect("binding should be queryable")
        .business_id;
    assert!(conversation_id.starts_with("c_direct_"));

    let binding = runtime
        .conversation_business_binding("100001", "default", conversation_id.as_str())
        .expect("binding should be queryable");
    assert_eq!(
        binding,
        ConversationBusinessBinding {
            business_type: "direct_chat".into(),
            business_id: direct_chat_id.clone(),
        }
    );

    let members = runtime
        .list_members("100001", "default", conversation_id.as_str())
        .expect("bound direct conversation should expose members");
    assert_eq!(members.len(), 2);
    assert!(
        members.iter().any(|member| {
            member.principal_id == "actor_a"
                && member.role == MembershipRole::Owner
                && member.attributes.get("directChatId").map(String::as_str) == Some(direct_chat_id.as_str())
        }),
        "left actor should become the anchor owner with direct chat binding metadata"
    );
    assert!(
        members.iter().any(|member| {
            member.principal_id == "actor_b"
                && member.role == MembershipRole::Member
                && member.attributes.get("directChatId").map(String::as_str) == Some(direct_chat_id.as_str())
        }),
        "right actor should become the peer member with direct chat binding metadata"
    );

    let events = journal.recorded();
    assert_eq!(events.len(), 3);
    assert_eq!(events[0].event_type, "conversation.created");
    let created_payload: serde_json::Value =
        serde_json::from_str(events[0].payload.as_str()).expect("created payload should be json");
    assert_eq!(created_payload["conversationType"], "direct");
    assert_eq!(created_payload["businessType"], "direct_chat");
    assert_eq!(created_payload["businessId"], direct_chat_id);
}

#[test]
fn test_create_thread_conversation_binds_parent_message_runtime_and_survives_recovery_replay() {
    let source_journal = InMemoryJournal::default();
    let runtime = ConversationRuntime::new(source_journal.clone());

    runtime
        .create_conversation(CreateConversationCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_parent_thread".into(),
            creator_id: "1".into(),
            conversation_type: "group".into(),
        })
        .expect("parent conversation should succeed");

    let root_message = runtime
        .post_message(PostMessageCommand {
            tenant_id: "100001".into(),
           organization_id: "0".into(),
            conversation_id: "c_parent_thread".into(),
            sender: Sender {
                id: "1".into(),
                kind: "user".into(),
                member_id: None,
                device_id: Some("d_owner".into()),
                session_id: Some("s_owner".into()),
                metadata: Default::default(),
            },
            client_msg_id: Some("client_thread_root".into()),
            message_type: MessageType::Standard,
            body: MessageBody {
                summary: Some("root".into()),
                parts: vec![ContentPart::text("root")],
                render_hints: Default::default(),
                reply_to: None,
            },
        })
        .expect("root message should succeed");

    let created = runtime
        .create_thread_conversation_with_creator_kind(
            CreateThreadConversationCommand {
                tenant_id: "100001".into(),
                organization_id: "0".into(),
                conversation_id: "c_thread_runtime".into(),
                parent_conversation_id: "c_parent_thread".into(),
                root_message_id: root_message.message_id.clone(),
                creator_id: "1".into(),
            },
            "user",
        )
        .expect("thread conversation should succeed");

    assert_eq!(created.conversation_id, "c_thread_runtime");

    let binding = runtime
        .conversation_business_binding("100001", "default", "c_thread_runtime")
        .expect("thread binding should be queryable");
    assert_eq!(
        binding,
        ConversationBusinessBinding {
            business_type: "thread".into(),
            business_id: root_message.message_id.clone(),
        }
    );

    let thread_members = runtime
        .list_members("100001", "default", "c_thread_runtime")
        .expect("thread members should be queryable");
    assert_eq!(thread_members.len(), 1);
    let owner = &thread_members[0];
    assert_eq!(owner.principal_id, "1");
    assert_eq!(owner.role, MembershipRole::Owner);
    assert_eq!(
        owner
            .attributes
            .get("parentConversationId")
            .map(String::as_str),
        Some("c_parent_thread")
    );
    assert_eq!(
        owner.attributes.get("rootMessageId").map(String::as_str),
        Some(root_message.message_id.as_str())
    );
    assert_eq!(
        owner.attributes.get("threadRole").map(String::as_str),
        Some("owner")
    );

    let reply = runtime
        .post_message(PostMessageCommand {
            tenant_id: "100001".into(),
           organization_id: "0".into(),
            conversation_id: "c_thread_runtime".into(),
            sender: Sender {
                id: "1".into(),
                kind: "user".into(),
                member_id: None,
                device_id: Some("d_owner".into()),
                session_id: Some("s_owner".into()),
                metadata: Default::default(),
            },
            client_msg_id: Some("client_thread_reply".into()),
            message_type: MessageType::Standard,
            body: MessageBody {
                summary: Some("reply".into()),
                parts: vec![ContentPart::text("reply")],
                render_hints: Default::default(),
                reply_to: None,
            },
        })
        .expect("thread reply should succeed");
    assert_eq!(reply.message_seq, 1);

    let created_event = source_journal
        .recorded()
        .into_iter()
        .find(|event| {
            event.event_type == "conversation.created"
                && event.aggregate_id == "c_thread_runtime"
                && event.scope_id == "c_thread_runtime"
        })
        .expect("thread created event should exist");
    let created_payload: serde_json::Value = serde_json::from_str(created_event.payload.as_str())
        .expect("thread created payload should be json");
    assert_eq!(created_payload["conversationType"], "thread");
    assert_eq!(created_payload["businessType"], "thread");
    assert_eq!(created_payload["businessId"], root_message.message_id);

    let replay_runtime = ConversationRuntime::new(InMemoryJournal::default());
    for envelope in source_journal.recorded() {
        replay_runtime
            .apply_recovered_envelope(&envelope)
            .expect("replay should succeed");
    }

    let replay_binding = replay_runtime
        .conversation_business_binding("100001", "default", "c_thread_runtime")
        .expect("replayed thread binding should exist");
    assert_eq!(replay_binding.business_type, "thread");
    assert_eq!(
        replay_binding.business_id,
        created_payload["businessId"].as_str().unwrap()
    );

    let replay_members = replay_runtime
        .list_members("100001", "default", "c_thread_runtime")
        .expect("replayed thread members should exist");
    assert_eq!(replay_members.len(), 1);
    assert_eq!(
        replay_members[0]
            .attributes
            .get("parentConversationId")
            .map(String::as_str),
        Some("c_parent_thread")
    );
}

#[test]
fn test_duplicate_create_thread_conversation_is_idempotent_and_conflicting_retry_is_rejected() {
    let source_journal = InMemoryJournal::default();
    let runtime = ConversationRuntime::new(source_journal.clone());

    runtime
        .create_conversation(CreateConversationCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_parent_thread_retry".into(),
            creator_id: "1".into(),
            conversation_type: "group".into(),
        })
        .expect("parent conversation should succeed");

    let first_root = runtime
        .post_message(PostMessageCommand {
            tenant_id: "100001".into(),
           organization_id: "0".into(),
            conversation_id: "c_parent_thread_retry".into(),
            sender: Sender {
                id: "1".into(),
                kind: "user".into(),
                member_id: None,
                device_id: Some("d_owner".into()),
                session_id: Some("s_owner".into()),
                metadata: Default::default(),
            },
            client_msg_id: Some("client_thread_retry_root_1".into()),
            message_type: MessageType::Standard,
            body: MessageBody {
                summary: Some("root-1".into()),
                parts: vec![ContentPart::text("root-1")],
                render_hints: Default::default(),
                reply_to: None,
            },
        })
        .expect("first root message should succeed");

    let second_root = runtime
        .post_message(PostMessageCommand {
            tenant_id: "100001".into(),
           organization_id: "0".into(),
            conversation_id: "c_parent_thread_retry".into(),
            sender: Sender {
                id: "1".into(),
                kind: "user".into(),
                member_id: None,
                device_id: Some("d_owner".into()),
                session_id: Some("s_owner".into()),
                metadata: Default::default(),
            },
            client_msg_id: Some("client_thread_retry_root_2".into()),
            message_type: MessageType::Standard,
            body: MessageBody {
                summary: Some("root-2".into()),
                parts: vec![ContentPart::text("root-2")],
                render_hints: Default::default(),
                reply_to: None,
            },
        })
        .expect("second root message should succeed");

    let first = runtime
        .create_thread_conversation_with_creator_kind(
            CreateThreadConversationCommand {
                tenant_id: "100001".into(),
                organization_id: "0".into(),
                conversation_id: "c_thread_retry".into(),
                parent_conversation_id: "c_parent_thread_retry".into(),
                root_message_id: first_root.message_id.clone(),
                creator_id: "1".into(),
            },
            "user",
        )
        .expect("first thread create should succeed");

    assert_eq!(first.delivery_status.as_ref().unwrap().as_str(), "applied");
    assert_eq!(
        first.proof_version.as_deref(),
        Some("conversation.create.delivery-proof.v1")
    );
    assert_eq!(
        first.request_key.as_deref(),
        Some("6#100001#4#user#1#1#13#create-thread#14#c_thread_retry")
    );

    let duplicate = runtime
        .create_thread_conversation_with_creator_kind(
            CreateThreadConversationCommand {
                tenant_id: "100001".into(),
                organization_id: "0".into(),
                conversation_id: "c_thread_retry".into(),
                parent_conversation_id: "c_parent_thread_retry".into(),
                root_message_id: first_root.message_id.clone(),
                creator_id: "1".into(),
            },
            "user",
        )
        .expect("duplicate thread create should replay");

    assert_eq!(duplicate.conversation_id, first.conversation_id);
    assert_eq!(duplicate.event_id, first.event_id);
    assert_eq!(duplicate.request_key, first.request_key);
    assert_eq!(duplicate.proof_version, first.proof_version);
    assert_eq!(
        duplicate.delivery_status.as_ref().unwrap().as_str(),
        "replayed"
    );

    let conflicting_retry = runtime.create_thread_conversation_with_creator_kind(
        CreateThreadConversationCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_thread_retry".into(),
            parent_conversation_id: "c_parent_thread_retry".into(),
            root_message_id: second_root.message_id.clone(),
            creator_id: "1".into(),
        },
        "user",
    );
    assert!(matches!(conflicting_retry, Err(RuntimeError::Conflict(_))));

    let replay_runtime = ConversationRuntime::new(InMemoryJournal::default());
    for envelope in source_journal.recorded() {
        replay_runtime
            .apply_recovered_envelope(&envelope)
            .expect("thread replay should succeed");
    }

    let recovered_duplicate = replay_runtime
        .create_thread_conversation_with_creator_kind(
            CreateThreadConversationCommand {
                tenant_id: "100001".into(),
                organization_id: "0".into(),
                conversation_id: "c_thread_retry".into(),
                parent_conversation_id: "c_parent_thread_retry".into(),
                root_message_id: first_root.message_id.clone(),
                creator_id: "1".into(),
            },
            "user",
        )
        .expect("recovered duplicate thread create should replay");
    assert_eq!(recovered_duplicate.event_id, first.event_id);
    assert_eq!(recovered_duplicate.request_key, first.request_key);
    assert_eq!(
        recovered_duplicate
            .delivery_status
            .as_ref()
            .unwrap()
            .as_str(),
        "replayed"
    );

    let events = source_journal.recorded();
    assert_eq!(
        events
            .iter()
            .filter(|event| event.aggregate_id == "c_thread_retry")
            .count(),
        2,
        "duplicate thread create retry must not append another conversation.created/member_joined pair for the thread conversation"
    );
}

#[test]
fn test_create_thread_conversation_auto_subscribes_root_message_author_for_notification_truth() {
    let source_journal = InMemoryJournal::default();
    let runtime = ConversationRuntime::new(source_journal.clone());

    runtime
        .create_conversation(CreateConversationCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_parent_thread_notify".into(),
            creator_id: "1".into(),
            conversation_type: "group".into(),
        })
        .expect("parent conversation should succeed");

    runtime
        .add_member(AddConversationMemberCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_parent_thread_notify".into(),
            principal_id: "1051".into(),
            principal_kind: "user".into(),
            role: MembershipRole::Member,
            invited_by: "1".into(),
        })
        .expect("root author should join parent conversation");

    let root_message = runtime
        .post_message(PostMessageCommand {
            tenant_id: "100001".into(),
           organization_id: "0".into(),
            conversation_id: "c_parent_thread_notify".into(),
            sender: Sender {
                id: "1051".into(),
                kind: "user".into(),
                member_id: None,
                device_id: Some("d_root_author".into()),
                session_id: Some("s_root_author".into()),
                metadata: Default::default(),
            },
            client_msg_id: Some("client_thread_root_notify".into()),
            message_type: MessageType::Standard,
            body: MessageBody {
                summary: Some("root notify".into()),
                parts: vec![ContentPart::text("root notify")],
                render_hints: Default::default(),
                reply_to: None,
            },
        })
        .expect("root author should post parent message");

    runtime
        .create_thread_conversation_with_creator_kind(
            CreateThreadConversationCommand {
                tenant_id: "100001".into(),
                organization_id: "0".into(),
                conversation_id: "c_thread_notify".into(),
                parent_conversation_id: "c_parent_thread_notify".into(),
                root_message_id: root_message.message_id.clone(),
                creator_id: "1".into(),
            },
            "user",
        )
        .expect("thread conversation should succeed");

    let thread_members = runtime
        .list_members("100001", "default", "c_thread_notify")
        .expect("thread members should be queryable");
    assert_eq!(thread_members.len(), 2);

    let owner = thread_members
        .iter()
        .find(|member| member.principal_id == "1")
        .expect("thread owner should exist");
    assert_eq!(owner.role, MembershipRole::Owner);
    assert_eq!(
        owner.attributes.get("threadRole").map(String::as_str),
        Some("owner")
    );

    let root_author = thread_members
        .iter()
        .find(|member| member.principal_id == "1051")
        .expect("root author should be auto-subscribed into thread");
    assert_eq!(root_author.role, MembershipRole::Member);
    assert_eq!(root_author.invited_by.as_deref(), Some("1"));
    assert_eq!(
        root_author
            .attributes
            .get("parentConversationId")
            .map(String::as_str),
        Some("c_parent_thread_notify")
    );
    assert_eq!(
        root_author
            .attributes
            .get("rootMessageId")
            .map(String::as_str),
        Some(root_message.message_id.as_str())
    );
    assert_eq!(
        root_author.attributes.get("threadRole").map(String::as_str),
        Some("root_author")
    );

    let read_cursor = runtime
        .read_cursor_view("100001", "default", "c_thread_notify", "1051")
        .expect("auto-subscribed thread member should get default read cursor");
    assert_eq!(read_cursor.principal_id, "1051");
    assert_eq!(read_cursor.read_seq, 0);

    let source_events = source_journal.recorded();
    let thread_join_events: Vec<_> = source_events
        .iter()
        .filter(|event| {
            event.event_type == "conversation.member_joined"
                && event.aggregate_id == "c_thread_notify"
        })
        .collect();
    assert_eq!(thread_join_events.len(), 2);
    assert!(thread_join_events.iter().any(|event| {
        let payload: serde_json::Value = serde_json::from_str(event.payload.as_str())
            .expect("thread member joined payload should be json");
        payload["principalId"] == "1051"
            && payload["attributes"]["threadRole"] == "root_author"
    }));

    let replay_runtime = ConversationRuntime::new(InMemoryJournal::default());
    for envelope in &source_events {
        replay_runtime
            .apply_recovered_envelope(envelope)
            .expect("replay should succeed");
    }

    let replay_members = replay_runtime
        .list_members("100001", "default", "c_thread_notify")
        .expect("replayed thread members should exist");
    assert_eq!(replay_members.len(), 2);
    assert!(replay_members.iter().any(|member| {
        member.principal_id == "1051"
            && member.attributes.get("threadRole").map(String::as_str) == Some("root_author")
    }));

    let reply = replay_runtime
        .post_message(PostMessageCommand {
            tenant_id: "100001".into(),
           organization_id: "0".into(),
            conversation_id: "c_thread_notify".into(),
            sender: Sender {
                id: "1051".into(),
                kind: "user".into(),
                member_id: None,
                device_id: Some("d_root_author".into()),
                session_id: Some("s_root_author".into()),
                metadata: Default::default(),
            },
            client_msg_id: Some("client_thread_notify_reply".into()),
            message_type: MessageType::Standard,
            body: MessageBody {
                summary: Some("reply from root author".into()),
                parts: vec![ContentPart::text("reply from root author")],
                render_hints: Default::default(),
                reply_to: None,
            },
        })
        .expect("replayed thread should allow auto-subscribed root author to reply");
    assert_eq!(reply.message_seq, 1);
}

#[test]
fn test_sync_shared_channel_linked_member_materializes_runtime_truth_and_survives_recovery_replay()
{
    let source_journal = InMemoryJournal::default();
    let runtime = ConversationRuntime::new(source_journal.clone());

    runtime
        .create_conversation(CreateConversationCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_shared_sync_runtime".into(),
            creator_id: "1".into(),
            conversation_type: "group".into(),
        })
        .expect("shared-sync conversation should succeed");

    runtime
        .apply_conversation_policy(ApplyConversationPolicyCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_shared_sync_runtime".into(),
            applied_by: "1".into(),
            policy: ConversationPolicy {
                policy_version: "group.policy.v1".into(),
                capability_flags: None,
                history_visibility: "shared".into(),
                retention_policy_ref: "tenant.standard".into(),
            },
        })
        .expect("shared history policy should apply");

    let posted = runtime
        .post_message(PostMessageCommand {
            tenant_id: "100001".into(),
           organization_id: "0".into(),
            conversation_id: "c_shared_sync_runtime".into(),
            sender: Sender {
                id: "1".into(),
                kind: "user".into(),
                member_id: None,
                device_id: Some("d_owner".into()),
                session_id: Some("s_owner".into()),
                metadata: Default::default(),
            },
            client_msg_id: Some("client_shared_sync_runtime".into()),
            message_type: MessageType::Standard,
            body: MessageBody {
                summary: Some("hello runtime sync".into()),
                parts: vec![ContentPart::text("hello runtime sync")],
                render_hints: Default::default(),
                reply_to: None,
            },
        })
        .expect("shared-sync root message should post");
    assert_eq!(posted.message_seq, 1);

    let linked_member = runtime
        .sync_shared_channel_linked_member_with_requester_kind(
            SyncSharedChannelLinkedMemberCommand {
                tenant_id: "100001".into(),
                organization_id: "0".into(),
                conversation_id: "c_shared_sync_runtime".into(),
                shared_channel_policy_id: "scp_runtime".into(),
                external_connection_id: "ec_runtime".into(),
                local_actor_id: "1064".into(),
                local_actor_kind: "user".into(),
                external_member_id: "partner::runtime-user".into(),
                synced_by: "svc_control".into(),
            },
            "system",
        )
        .expect("shared channel linked member sync should succeed");

    assert_eq!(linked_member.principal_id, "1064");
    assert_eq!(linked_member.principal_kind, "user");
    assert_eq!(linked_member.role, MembershipRole::Guest);
    assert_eq!(linked_member.state, MembershipState::Linked);
    assert_eq!(
        linked_member
            .attributes
            .get("sharedChannelPolicyId")
            .map(String::as_str),
        Some("scp_runtime")
    );
    assert_eq!(
        linked_member
            .attributes
            .get("externalConnectionId")
            .map(String::as_str),
        Some("ec_runtime")
    );
    assert_eq!(
        linked_member
            .attributes
            .get("externalMemberId")
            .map(String::as_str),
        Some("partner::runtime-user")
    );
    assert_eq!(
        linked_member
            .attributes
            .get("sharedChannelSyncRequestKey")
            .map(String::as_str),
        Some(
            "100001|c_shared_sync_runtime|scp_runtime|ec_runtime|1064|user|partner::runtime-user"
        )
    );

    let linked_history = list_all_messages(
        &runtime,
        "100001",
        "c_shared_sync_runtime",
        "1064",
    )
    .expect("linked member should read shared history after sync");
    assert_eq!(linked_history.items.len(), 1);
    assert_eq!(
        linked_history.items[0].message.message_id,
        posted.message_id
    );

    let source_events = source_journal.recorded();
    assert!(source_events.iter().any(|event| {
        event.event_type == "conversation.member_joined"
            && event.aggregate_id == "c_shared_sync_runtime"
            && serde_json::from_str::<serde_json::Value>(event.payload.as_str())
                .ok()
                .is_some_and(|payload| {
                    payload["principalId"] == "1064"
                        && payload["state"] == "linked"
                        && payload["attributes"]["sharedChannelPolicyId"] == "scp_runtime"
                })
    }));

    let replay_runtime = ConversationRuntime::new(InMemoryJournal::default());
    for envelope in &source_events {
        replay_runtime
            .apply_recovered_envelope(envelope)
            .expect("replay should succeed");
    }

    let replay_linked_history = list_all_messages(
        &replay_runtime,
        "100001",
        "c_shared_sync_runtime",
        "1064",
    )
    .expect("replayed linked member should still read shared history");
    assert_eq!(replay_linked_history.items.len(), 1);
    assert_eq!(
        replay_linked_history.items[0]
            .message
            .body
            .summary
            .as_deref(),
        Some("hello runtime sync")
    );
}

#[test]
fn test_bind_direct_chat_conversation_rejects_duplicate_business_binding() {
    let journal = InMemoryJournal::default();
    let runtime = ConversationRuntime::new(journal);

    let first = runtime
        .bind_direct_chat_conversation_with_binder_kind(
            canonical_bind_direct_chat_command("100001", "actor_a", "actor_b"),
            "system",
        )
        .expect("first direct chat binding should succeed");

    let duplicate = runtime.bind_direct_chat_conversation_with_binder_kind(
        canonical_bind_direct_chat_command("100001", "actor_a", "actor_b"),
        "system",
    )
    .expect("duplicate direct chat binding should replay");

    assert_eq!(duplicate.conversation_id, first.conversation_id);
    assert_eq!(
        duplicate.delivery_status.as_ref().unwrap().as_str(),
        "replayed"
    );
}

#[test]
fn test_duplicate_bind_direct_chat_conversation_is_idempotent_and_conflicting_retry_is_rejected() {
    let source_journal = InMemoryJournal::default();
    let source_runtime = ConversationRuntime::new(source_journal.clone());

    let first = source_runtime
        .bind_direct_chat_conversation_with_binder_kind(
            canonical_bind_direct_chat_command("100001", "actor_a", "actor_b"),
            "system",
        )
        .expect("first direct chat binding should succeed");

    assert_eq!(first.delivery_status.as_ref().unwrap().as_str(), "applied");
    assert_eq!(
        first.proof_version.as_deref(),
        Some("conversation.create.delivery-proof.v1")
    );
    assert!(first.request_key.is_some());
    let conversation_id = first.conversation_id.clone();

    let duplicate = source_runtime
        .bind_direct_chat_conversation_with_binder_kind(
            canonical_bind_direct_chat_command("100001", "actor_a", "actor_b"),
            "system",
        )
        .expect("duplicate direct chat binding should replay");

    assert_eq!(duplicate.conversation_id, first.conversation_id);
    assert_eq!(duplicate.event_id, first.event_id);
    assert_eq!(duplicate.request_key, first.request_key);
    assert_eq!(duplicate.proof_version, first.proof_version);
    assert_eq!(
        duplicate.delivery_status.as_ref().unwrap().as_str(),
        "replayed"
    );

    let conflicting_retry = source_runtime.bind_direct_chat_conversation_with_binder_kind(
        BindDirectChatConversationCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: conversation_id.clone(),
            direct_chat_id: String::new(),
            left_actor_id: "actor_a".into(),
            left_actor_kind: "user".into(),
            right_actor_id: "actor_c".into(),
            right_actor_kind: "user".into(),
            bound_by: "svc_control".into(),
        },
        "system",
    );
    assert!(matches!(conflicting_retry, Err(RuntimeError::InvalidInput(_))));

    let replay_runtime = ConversationRuntime::new(InMemoryJournal::default());
    for envelope in source_journal.recorded() {
        replay_runtime
            .apply_recovered_envelope(&envelope)
            .expect("direct chat replay should succeed");
    }

    let recovered_duplicate = replay_runtime
        .bind_direct_chat_conversation_with_binder_kind(
            canonical_bind_direct_chat_command("100001", "actor_a", "actor_b"),
            "system",
        )
        .expect("recovered duplicate direct chat binding should replay");
    assert_eq!(recovered_duplicate.event_id, first.event_id);
    assert_eq!(recovered_duplicate.request_key, first.request_key);
    assert_eq!(
        recovered_duplicate
            .delivery_status
            .as_ref()
            .unwrap()
            .as_str(),
        "replayed"
    );

    let events = source_journal.recorded();
    assert_eq!(
        events
            .iter()
            .filter(|event| event.aggregate_id == conversation_id)
            .count(),
        3,
        "duplicate direct chat binding retry must not append another conversation.created/member_joined pair"
    );
}

#[test]
fn test_direct_chat_business_scope_key_is_segment_safe_for_delimiter_bearing_ids() {
    let runtime = ConversationRuntime::new(InMemoryJournal::default());

    let first = runtime
        .bind_direct_chat_conversation_with_binder_kind(
            canonical_bind_direct_chat_command("tenant:a", "1052", "1065"),
            "system",
        )
        .expect("first direct chat binding should be created");
    let second = runtime
        .bind_direct_chat_conversation_with_binder_kind(
            canonical_bind_direct_chat_command("tenant", "1053", "1066"),
            "system",
        )
        .expect("second direct chat binding should not collide with first business key");

    assert_ne!(first.conversation_id, second.conversation_id);
    assert_ne!(first.request_key, second.request_key);

    let first_binding = runtime
        .conversation_business_binding("tenant:a", "default", first.conversation_id.as_str())
        .expect("first direct chat binding should be readable");
    let second_binding = runtime
        .conversation_business_binding("tenant", "default", second.conversation_id.as_str())
        .expect("second direct chat binding should be readable");
    assert_ne!(first_binding.business_id, second_binding.business_id);
    assert!(first_binding.business_id.contains('#'));
    assert!(second_binding.business_id.contains('#'));
}

#[test]
fn test_direct_chat_business_binding_survives_recovery_replay() {
    let source_journal = InMemoryJournal::default();
    let source_runtime = ConversationRuntime::new(source_journal.clone());

    let created = source_runtime
        .bind_direct_chat_conversation_with_binder_kind(
            canonical_bind_direct_chat_command("100001", "actor_a", "actor_b"),
            "system",
        )
        .expect("direct chat binding should succeed");
    let conversation_id = created.conversation_id.clone();
    let direct_chat_id = source_runtime
        .conversation_business_binding("100001", "default", conversation_id.as_str())
        .expect("binding should exist")
        .business_id;

    let replay_journal = InMemoryJournal::default();
    let replay_runtime = ConversationRuntime::new(replay_journal);
    for envelope in source_journal.recorded() {
        replay_runtime
            .apply_recovered_envelope(&envelope)
            .expect("replay should succeed");
    }

    let binding = replay_runtime
        .conversation_business_binding("100001", "default", conversation_id.as_str())
        .expect("replayed binding should exist");
    assert_eq!(binding.business_type, "direct_chat");
    assert_eq!(binding.business_id, direct_chat_id);

    let duplicate_after_replay = replay_runtime.bind_direct_chat_conversation_with_binder_kind(
        canonical_bind_direct_chat_command("100001", "actor_a", "actor_b"),
        "system",
    )
    .expect("replayed direct chat binding should be idempotent");

    assert_eq!(duplicate_after_replay.conversation_id, conversation_id);
    assert_eq!(
        duplicate_after_replay.delivery_status.as_ref().unwrap().as_str(),
        "replayed"
    );
}

#[test]
fn test_post_message_rejects_oversized_sender_session_id() {
    let runtime = ConversationRuntime::new(InMemoryJournal::default());

    runtime
        .create_conversation(CreateConversationCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_sender_session_oversized".into(),
            creator_id: "1".into(),
            conversation_type: "group".into(),
        })
        .expect("create conversation should succeed");

    let error = runtime
        .post_message(PostMessageCommand {
            tenant_id: "100001".into(),
           organization_id: "0".into(),
            conversation_id: "c_sender_session_oversized".into(),
            sender: Sender {
                id: "1".into(),
                kind: "user".into(),
                member_id: Some("cm_demo".into()),
                device_id: Some("d_demo".into()),
                session_id: Some("s".repeat(257)),
                metadata: Default::default(),
            },
            client_msg_id: Some("client_sender_session_oversized".into()),
            message_type: MessageType::Standard,
            body: MessageBody {
                summary: Some("hello".into()),
                parts: vec![ContentPart::text("hello")],
                render_hints: Default::default(),
                reply_to: None,
            },
        })
        .expect_err("oversized sender session id should be rejected");

    match error {
        RuntimeError::PayloadTooLarge(message) => {
            assert!(message.contains("senderSessionId"));
        }
        other => panic!("expected payload_too_large, got {other:?}"),
    }
}

#[test]
fn test_create_conversation_rejects_oversized_creator_attributes() {
    let runtime = ConversationRuntime::new(InMemoryJournal::default());

    let error = runtime
        .create_conversation_with_creator_kind_and_attributes(
            CreateConversationCommand {
                tenant_id: "100001".into(),
                organization_id: "0".into(),
                conversation_id: "c_creator_attributes_oversized".into(),
                creator_id: "1".into(),
                conversation_type: "group".into(),
            },
            "user",
            BTreeMap::from([("profile".into(), "x".repeat(70 * 1024))]),
        )
        .expect_err("oversized creator attributes should be rejected");

    match error {
        RuntimeError::PayloadTooLarge(message) => {
            assert!(message.contains("creatorAttributes"));
        }
        other => panic!("expected payload_too_large, got {other:?}"),
    }
}

#[test]
fn test_post_message_rejects_oversized_sender_metadata() {
    let runtime = ConversationRuntime::new(InMemoryJournal::default());

    runtime
        .create_conversation(CreateConversationCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_sender_metadata_oversized".into(),
            creator_id: "1".into(),
            conversation_type: "group".into(),
        })
        .expect("create conversation should succeed");

    let error = runtime
        .post_message(PostMessageCommand {
            tenant_id: "100001".into(),
           organization_id: "0".into(),
            conversation_id: "c_sender_metadata_oversized".into(),
            sender: Sender {
                id: "1".into(),
                kind: "user".into(),
                member_id: Some("cm_demo".into()),
                device_id: Some("d_demo".into()),
                session_id: Some("s_demo".into()),
                metadata: BTreeMap::from([("profile".into(), "x".repeat(70 * 1024))]),
            },
            client_msg_id: Some("client_sender_metadata_oversized".into()),
            message_type: MessageType::Standard,
            body: MessageBody {
                summary: Some("hello".into()),
                parts: vec![ContentPart::text("hello")],
                render_hints: Default::default(),
                reply_to: None,
            },
        })
        .expect_err("oversized sender metadata should be rejected");

    match error {
        RuntimeError::PayloadTooLarge(message) => {
            assert!(message.contains("senderMetadata"));
        }
        other => panic!("expected payload_too_large, got {other:?}"),
    }
}

#[test]
fn test_post_message_rejects_oversized_render_hints() {
    let runtime = ConversationRuntime::new(InMemoryJournal::default());

    runtime
        .create_conversation(CreateConversationCommand {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_render_hints_oversized".into(),
            creator_id: "1".into(),
            conversation_type: "group".into(),
        })
        .expect("create conversation should succeed");

    let error = runtime
        .post_message(PostMessageCommand {
            tenant_id: "100001".into(),
           organization_id: "0".into(),
            conversation_id: "c_render_hints_oversized".into(),
            sender: Sender {
                id: "1".into(),
                kind: "user".into(),
                member_id: Some("cm_demo".into()),
                device_id: Some("d_demo".into()),
                session_id: Some("s_demo".into()),
                metadata: BTreeMap::new(),
            },
            client_msg_id: Some("client_render_hints_oversized".into()),
            message_type: MessageType::Standard,
            body: MessageBody {
                summary: Some("hello".into()),
                parts: vec![ContentPart::text("hello")],
                render_hints: BTreeMap::from([("preview".into(), "x".repeat(70 * 1024))]),
                reply_to: None,
            },
        })
        .expect_err("oversized render hints should be rejected");

    match error {
        RuntimeError::PayloadTooLarge(message) => {
            assert!(message.contains("renderHints"));
        }
        other => panic!("expected payload_too_large, got {other:?}"),
    }
}
