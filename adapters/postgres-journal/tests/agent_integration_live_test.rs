//! Live PostgreSQL coverage for IM-owned Agents projection, binding, and dispatch state.

use std::collections::BTreeMap;
use std::sync::{Arc, Barrier};
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};

use im_adapters_postgres_journal::{
    PostgresAgentIntegrationStore, PostgresDurableMessagePostWriter, PostgresJournalConfig,
};
use im_domain_core::message::{ContentPart, MentionPart, MentionTargetKind, MessageBody};
use im_domain_events::{AggregateType, CommitEnvelope, EventActor};
use im_platform_contracts::{AgentDispatchReplyCompletion, IdGenerator, StoredMessageRecord};
use sdkwork_im_contract_agent::{
    AGENT_MENTION_DISPATCH_SCHEMA_VERSION, AgentAssignmentSource, AgentBindingStatus,
    AgentIntegrationStore, AgentMentionDispatchRequest, AgentMentionDispatchTarget,
    ConversationAgentBindingRecord, ConversationAgentProjectionItem,
    ReplaceConversationAgentProjection,
};

fn test_numeric_scope() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time must be after epoch")
        .as_millis() as u64
}

#[tokio::test(flavor = "multi_thread")]
#[ignore = "requires live PostgreSQL via SDKWORK_IM_DATABASE_URL"]
async fn agents_projection_binding_and_dispatch_are_idempotent_scoped_and_leased() {
    let database_url = std::env::var("SDKWORK_IM_DATABASE_URL")
        .expect("SDKWORK_IM_DATABASE_URL must be set for live integration test");
    sdkwork_im_database_pool::bootstrap_im_process_database_pools_from_env()
        .await
        .expect("shared IM database pools should bootstrap");
    let pool = PostgresJournalConfig::new(database_url)
        .connect_pool()
        .expect("postgres journal pool should connect");
    let id_generator: Arc<dyn IdGenerator> = Arc::new(
        sdkwork_im_runtime_id::RuntimeSnowflakeIdGenerator::with_node_id(901)
            .expect("test id generator should initialize"),
    );
    let store = PostgresAgentIntegrationStore::from_pool(pool.clone(), id_generator);
    let tenant_id = test_numeric_scope();
    let organization_id = 0;
    let conversation_id = format!("c_agent_integration_{tenant_id}");
    let agent_id = "agent.im.commercial";
    let now = chrono::Utc::now();
    let now_text = now.to_rfc3339();

    let projection = ReplaceConversationAgentProjection {
        tenant_id,
        organization_id,
        conversation_id: conversation_id.clone(),
        assignment_source: AgentAssignmentSource::ConversationOverride,
        assignment_generation: 1,
        assigned_by: tenant_id,
        assigned_at: now_text.clone(),
        source_event_id: format!("evt_agent_projection_{tenant_id}"),
        source_aggregate_version: 1,
        payload_hash: format!("projection-hash-{tenant_id}"),
        items: vec![ConversationAgentProjectionItem {
            agent_id: agent_id.into(),
            agent_revision_ref: None,
            position: 0,
        }],
    };
    store
        .replace_conversation_agents(projection.clone())
        .expect("projection should apply");
    store
        .replace_conversation_agents(projection.clone())
        .expect("same projection should replay");
    let mut stale = projection.clone();
    stale.source_aggregate_version = 0;
    stale.payload_hash = "stale".into();
    assert!(store.replace_conversation_agents(stale).is_err());
    assert_eq!(
        store
            .list_conversation_agents(tenant_id, organization_id, &conversation_id, 20)
            .expect("projection should list")
            .len(),
        1
    );

    let dispatch_id = format!("dispatch.{tenant_id}");
    let request = AgentMentionDispatchRequest {
        schema_version: AGENT_MENTION_DISPATCH_SCHEMA_VERSION,
        tenant_id: tenant_id.to_string(),
        organization_id: organization_id.to_string(),
        conversation_id: conversation_id.clone(),
        message_id: tenant_id.saturating_add(1).to_string(),
        message_seq: 1,
        causation_event_id: format!("evt_message_{tenant_id}"),
        sender_principal_id: tenant_id.to_string(),
        sender_principal_kind: "user".into(),
        assignment_generation: 1,
        targets: vec![AgentMentionDispatchTarget {
            dispatch_id: dispatch_id.clone(),
            agent_id: agent_id.into(),
            revision_id: None,
        }],
        body: MessageBody {
            summary: Some("run agent".into()),
            parts: vec![ContentPart::Mention(MentionPart {
                target_kind: MentionTargetKind::Agent,
                target_id: agent_id.into(),
                display_text: "@Agent".into(),
                assignment_generation: 1,
            })],
            render_hints: BTreeMap::new(),
            reply_to: None,
        },
        requested_at: now_text.clone(),
    };
    let first = store
        .enqueue_dispatches(&request, 3)
        .expect("dispatch should enqueue");
    let replay = store
        .enqueue_dispatches(&request, 3)
        .expect("dispatch should replay");
    assert_eq!(first[0].dispatch_id, replay[0].dispatch_id);

    let store = Arc::new(store);
    let barrier = Arc::new(Barrier::new(3));
    let mut workers = Vec::new();
    for worker_id in ["worker-a", "worker-b"] {
        let store = store.clone();
        let barrier = barrier.clone();
        let now_text = now_text.clone();
        let expires = (now + chrono::Duration::seconds(30)).to_rfc3339();
        workers.push(thread::spawn(move || {
            barrier.wait();
            store.claim_dispatches(
                tenant_id,
                organization_id,
                worker_id,
                &now_text,
                &expires,
                1,
            )
        }));
    }
    barrier.wait();
    let claims = workers
        .into_iter()
        .flat_map(|worker| {
            worker
                .join()
                .expect("claim worker should not panic")
                .unwrap()
        })
        .collect::<Vec<_>>();
    assert_eq!(claims.len(), 1);
    assert_eq!(claims[0].dispatch_id, dispatch_id);

    let binding_id = format!("binding.{tenant_id}");
    let binding = store
        .save_binding(ConversationAgentBindingRecord {
            binding_id: binding_id.clone(),
            tenant_id,
            organization_id,
            conversation_id: conversation_id.clone(),
            agent_id: agent_id.into(),
            agent_revision_ref: None,
            assignment_generation: 1,
            agents_session_id: None,
            status: AgentBindingStatus::Pending,
            idempotency_key: format!("binding-idempotency-{tenant_id}"),
            payload_hash: format!("binding-hash-{tenant_id}"),
            created_by: tenant_id,
            updated_by: tenant_id,
            last_error_code: None,
            last_error_detail: None,
            version: 0,
            created_at: now_text.clone(),
            updated_at: now_text.clone(),
        })
        .expect("binding should insert");
    let mut active = binding;
    active.status = AgentBindingStatus::Active;
    let agents_session_id = format!("session.im.{tenant_id}");
    active.agents_session_id = Some(agents_session_id.clone());
    active.version += 1;
    active.updated_at = (now + chrono::Duration::seconds(1)).to_rfc3339();
    assert_eq!(
        store
            .save_binding(active)
            .expect("binding should activate")
            .status,
        AgentBindingStatus::Active
    );

    let first_claim = claims[0].clone();
    let running_at = (now + chrono::Duration::seconds(1)).to_rfc3339();
    store
        .mark_dispatch_running(
            &first_claim,
            first_claim
                .lease_owner
                .as_deref()
                .expect("claim should have a lease owner"),
            &binding_id,
            &agents_session_id,
            &running_at,
        )
        .expect("dispatch should enter running state");
    let reconcile_at = (now + chrono::Duration::seconds(2)).to_rfc3339();
    store
        .defer_dispatch_reconciliation(
            &first_claim,
            first_claim
                .lease_owner
                .as_deref()
                .expect("claim should have a lease owner"),
            Some("turn.im.reconciliation"),
            "Agents turn remains in progress",
            &reconcile_at,
            &running_at,
        )
        .expect("running dispatch should defer reconciliation");
    let reclaim_at = (now + chrono::Duration::seconds(3)).to_rfc3339();
    let reclaim_expires_at = (now + chrono::Duration::seconds(33)).to_rfc3339();
    let reconciled = store
        .claim_dispatches(
            tenant_id,
            organization_id,
            "worker-reconcile",
            &reclaim_at,
            &reclaim_expires_at,
            1,
        )
        .expect("deferred dispatch should become claimable");
    assert_eq!(reconciled.len(), 1);
    assert_eq!(reconciled[0].attempt_count, first_claim.attempt_count);
    assert_eq!(
        reconciled[0].agents_turn_id.as_deref(),
        Some("turn.im.reconciliation")
    );

    let cleanup_pool = pool.clone();
    tokio::task::spawn_blocking(move || {
        let mut client = cleanup_pool.get().expect("cleanup connection");
        client
            .execute(
                "delete from im_agent_dispatch where tenant_id = $1",
                &[&(tenant_id as i64)],
            )
            .expect("dispatch cleanup");
        client
            .execute(
                "delete from im_conversation_agent_binding where tenant_id = $1",
                &[&(tenant_id as i64)],
            )
            .expect("binding cleanup");
        client
            .execute(
                "delete from im_projection_conversation_agent where tenant_id = $1",
                &[&(tenant_id as i64)],
            )
            .expect("projection cleanup");
    })
    .await
    .expect("cleanup should not panic");
}

fn message_envelope(
    tenant_id: u64,
    organization_id: u64,
    conversation_id: &str,
    event_id: &str,
    ordering_seq: u64,
    actor_id: &str,
    actor_kind: &str,
    occurred_at: &str,
) -> CommitEnvelope {
    let tenant_id = tenant_id.to_string();
    CommitEnvelope {
        event_id: event_id.into(),
        tenant_id: tenant_id.clone(),
        organization_id: organization_id.to_string(),
        event_type: "message.posted".into(),
        event_version: 1,
        aggregate_type: AggregateType::Conversation,
        aggregate_id: conversation_id.into(),
        scope_type: "conversation".into(),
        scope_id: conversation_id.into(),
        ordering_key: CommitEnvelope::ordering_key(&tenant_id, conversation_id),
        ordering_seq,
        causation_id: None,
        correlation_id: None,
        idempotency_key: Some(event_id.into()),
        actor: EventActor {
            actor_id: actor_id.into(),
            actor_kind: actor_kind.into(),
            actor_session_id: None,
        },
        occurred_at: occurred_at.into(),
        committed_at: occurred_at.into(),
        payload_schema: Some("message.posted.v1".into()),
        payload: format!(r#"{{"eventId":"{event_id}"}}"#),
        retention_class: "standard".into(),
        audit_class: "default".into(),
    }
}

#[tokio::test(flavor = "multi_thread")]
#[ignore = "requires live PostgreSQL via SDKWORK_IM_DATABASE_URL"]
async fn agent_reply_and_dispatch_completion_commit_and_rollback_atomically() {
    let database_url = std::env::var("SDKWORK_IM_DATABASE_URL")
        .expect("SDKWORK_IM_DATABASE_URL must be set for live integration test");
    sdkwork_im_database_pool::bootstrap_im_process_database_pools_from_env()
        .await
        .expect("shared IM database pools should bootstrap");
    let pool = PostgresJournalConfig::new(database_url)
        .connect_pool()
        .expect("postgres journal pool should connect");
    let store = PostgresAgentIntegrationStore::from_pool(
        pool.clone(),
        Arc::new(
            sdkwork_im_runtime_id::RuntimeSnowflakeIdGenerator::with_node_id(902)
                .expect("test id generator should initialize"),
        ),
    );
    let writer = PostgresDurableMessagePostWriter::new(pool.clone(), Arc::from(""));
    let tenant_id = test_numeric_scope();
    let organization_id = 0;
    let conversation_id = format!("c_agent_reply_{tenant_id}");
    let agent_id = "agent.im.atomic";
    let session_id = format!("session.im.{tenant_id}");
    let turn_id = format!("turn.im.{tenant_id}");
    let dispatch_id = format!("amd_atomic_{tenant_id}");
    let source_message_id = tenant_id + 10;
    let reply_message_id = tenant_id + 11;
    let rolled_back_message_id = tenant_id + 12;
    let now = chrono::Utc::now();
    let now_text = now.to_rfc3339();
    let source_body = MessageBody {
        summary: Some("question".into()),
        parts: vec![
            ContentPart::text("question"),
            ContentPart::Mention(MentionPart {
                target_kind: MentionTargetKind::Agent,
                target_id: agent_id.into(),
                display_text: "@agent".into(),
                assignment_generation: 1,
            }),
        ],
        render_hints: BTreeMap::new(),
        reply_to: None,
    };
    let request = AgentMentionDispatchRequest {
        schema_version: AGENT_MENTION_DISPATCH_SCHEMA_VERSION,
        tenant_id: tenant_id.to_string(),
        organization_id: organization_id.to_string(),
        conversation_id: conversation_id.clone(),
        message_id: source_message_id.to_string(),
        message_seq: 1,
        causation_event_id: format!("evt_source_{tenant_id}"),
        sender_principal_id: tenant_id.to_string(),
        sender_principal_kind: "user".into(),
        assignment_generation: 1,
        targets: vec![AgentMentionDispatchTarget {
            dispatch_id: dispatch_id.clone(),
            agent_id: agent_id.into(),
            revision_id: None,
        }],
        body: source_body.clone(),
        requested_at: now_text.clone(),
    };
    writer
        .persist_message_post_batch_with_agent_dispatch(
            vec![message_envelope(
                tenant_id,
                organization_id,
                &conversation_id,
                &format!("evt_source_{tenant_id}"),
                1,
                &tenant_id.to_string(),
                "user",
                &now_text,
            )],
            StoredMessageRecord {
                tenant_id: tenant_id.to_string(),
                organization_id: organization_id.to_string(),
                conversation_id: conversation_id.clone(),
                message_id: source_message_id as i64,
                message_seq: 1,
                sender_principal_kind: "user".into(),
                sender_principal_id: tenant_id.to_string(),
                sender_device_id: None,
                client_msg_id: Some(format!("client.source.{tenant_id}")),
                message_type: "standard".into(),
                payload_json: serde_json::to_string(&source_body)
                    .expect("source body should encode"),
                payload_hash: format!("source-hash-{tenant_id}"),
                created_at: now_text.clone(),
                updated_at: now_text.clone(),
                deleted_at: None,
                retention_until: None,
            },
            Vec::new(),
            Some(request),
            3,
        )
        .expect("source message and dispatch should commit");

    let binding_id = format!("binding.atomic.{tenant_id}");
    store
        .save_binding(ConversationAgentBindingRecord {
            binding_id: binding_id.clone(),
            tenant_id,
            organization_id,
            conversation_id: conversation_id.clone(),
            agent_id: agent_id.into(),
            agent_revision_ref: None,
            assignment_generation: 1,
            agents_session_id: Some(session_id.clone()),
            status: AgentBindingStatus::Active,
            idempotency_key: format!("binding.atomic.idempotency.{tenant_id}"),
            payload_hash: format!("binding-atomic-hash-{tenant_id}"),
            created_by: tenant_id,
            updated_by: tenant_id,
            last_error_code: None,
            last_error_detail: None,
            version: 0,
            created_at: now_text.clone(),
            updated_at: now_text.clone(),
        })
        .expect("active binding should insert");
    let lease_owner = format!("worker.atomic.{tenant_id}");
    let claimed = store
        .claim_dispatches(
            tenant_id,
            organization_id,
            &lease_owner,
            &now_text,
            &(now + chrono::Duration::seconds(90)).to_rfc3339(),
            1,
        )
        .expect("dispatch should claim")
        .into_iter()
        .next()
        .expect("dispatch should exist");
    store
        .mark_dispatch_running(&claimed, &lease_owner, &binding_id, &session_id, &now_text)
        .expect("dispatch should run");

    let reply_body = MessageBody {
        summary: Some("answer".into()),
        parts: vec![ContentPart::text("answer")],
        render_hints: BTreeMap::new(),
        reply_to: None,
    };
    let completion = AgentDispatchReplyCompletion {
        tenant_id,
        organization_id,
        conversation_id: conversation_id.clone(),
        dispatch_id: dispatch_id.clone(),
        lease_owner: lease_owner.clone(),
        agent_id: agent_id.into(),
        agent_revision_ref: None,
        assignment_generation: 1,
        agents_session_id: session_id.clone(),
        agents_turn_id: turn_id.clone(),
    };
    let reply_record = StoredMessageRecord {
        tenant_id: tenant_id.to_string(),
        organization_id: organization_id.to_string(),
        conversation_id: conversation_id.clone(),
        message_id: reply_message_id as i64,
        message_seq: 2,
        sender_principal_kind: "agent".into(),
        sender_principal_id: agent_id.into(),
        sender_device_id: None,
        client_msg_id: Some(format!("agent-dispatch-reply:{dispatch_id}")),
        message_type: "standard".into(),
        payload_json: serde_json::to_string(&reply_body).expect("reply body should encode"),
        payload_hash: format!("reply-hash-{tenant_id}"),
        created_at: now_text.clone(),
        updated_at: now_text.clone(),
        deleted_at: None,
        retention_until: None,
    };
    writer
        .persist_agent_reply_and_complete_dispatch(
            vec![message_envelope(
                tenant_id,
                organization_id,
                &conversation_id,
                &format!("evt_reply_{tenant_id}"),
                2,
                agent_id,
                "agent",
                &now_text,
            )],
            reply_record.clone(),
            Vec::new(),
            completion.clone(),
        )
        .expect("reply and completion should commit");
    writer
        .persist_agent_reply_and_complete_dispatch(
            vec![message_envelope(
                tenant_id,
                organization_id,
                &conversation_id,
                &format!("evt_reply_{tenant_id}"),
                2,
                agent_id,
                "agent",
                &now_text,
            )],
            reply_record,
            Vec::new(),
            completion,
        )
        .expect("exact reply completion should replay");

    let rollback_body = MessageBody {
        summary: Some("must rollback".into()),
        parts: vec![ContentPart::text("must rollback")],
        render_hints: BTreeMap::new(),
        reply_to: None,
    };
    let rollback_result = writer.persist_agent_reply_and_complete_dispatch(
        vec![message_envelope(
            tenant_id,
            organization_id,
            &conversation_id,
            &format!("evt_reply_rollback_{tenant_id}"),
            3,
            agent_id,
            "agent",
            &now_text,
        )],
        StoredMessageRecord {
            tenant_id: tenant_id.to_string(),
            organization_id: organization_id.to_string(),
            conversation_id: conversation_id.clone(),
            message_id: rolled_back_message_id as i64,
            message_seq: 3,
            sender_principal_kind: "agent".into(),
            sender_principal_id: agent_id.into(),
            sender_device_id: None,
            client_msg_id: Some(format!("agent-dispatch-reply-rollback:{dispatch_id}")),
            message_type: "standard".into(),
            payload_json: serde_json::to_string(&rollback_body)
                .expect("rollback body should encode"),
            payload_hash: format!("rollback-hash-{tenant_id}"),
            created_at: now_text.clone(),
            updated_at: now_text.clone(),
            deleted_at: None,
            retention_until: None,
        },
        Vec::new(),
        AgentDispatchReplyCompletion {
            tenant_id,
            organization_id,
            conversation_id: conversation_id.clone(),
            dispatch_id: dispatch_id.clone(),
            lease_owner: "wrong-worker".into(),
            agent_id: agent_id.into(),
            agent_revision_ref: None,
            assignment_generation: 1,
            agents_session_id: session_id,
            agents_turn_id: turn_id,
        },
    );
    assert!(rollback_result.is_err());

    let cleanup_pool = pool.clone();
    tokio::task::spawn_blocking(move || {
        let mut client = cleanup_pool.get().expect("verification connection");
        let completion = client
            .query_one(
                "select status, agents_turn_id, reply_message_id, reply_message_seq from im_agent_dispatch where tenant_id = $1 and dispatch_id = $2",
                &[&(tenant_id as i64), &dispatch_id],
            )
            .expect("completed dispatch should load");
        assert_eq!(completion.get::<_, i16>(0), 4);
        assert_eq!(completion.get::<_, Option<String>>(1), Some(format!("turn.im.{tenant_id}")));
        assert_eq!(completion.get::<_, Option<i64>>(2), Some(reply_message_id as i64));
        assert_eq!(completion.get::<_, Option<i64>>(3), Some(2));
        assert_eq!(
            client
                .query_one(
                    "select count(*) from im_conversation_messages where tenant_id = $1 and message_id = $2",
                    &[&(tenant_id.to_string()), &(rolled_back_message_id as i64)],
                )
                .expect("rolled back message count should load")
                .get::<_, i64>(0),
            0
        );
        client
            .execute("delete from im_agent_dispatch where tenant_id = $1", &[&(tenant_id as i64)])
            .expect("dispatch cleanup");
        client
            .execute("delete from im_conversation_agent_binding where tenant_id = $1", &[&(tenant_id as i64)])
            .expect("binding cleanup");
        client
            .execute("delete from im_conversation_messages where tenant_id = $1", &[&tenant_id.to_string()])
            .expect("message cleanup");
        client
            .execute("delete from im_commit_journal where tenant_id = $1", &[&tenant_id.to_string()])
            .expect("journal cleanup");
    })
    .await
    .expect("verification should not panic");
}
