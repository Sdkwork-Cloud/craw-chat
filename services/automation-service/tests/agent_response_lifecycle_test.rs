use axum::response::IntoResponse;
use std::collections::{BTreeMap, BTreeSet};
use std::sync::{Arc, Mutex};

use http_body_util::BodyExt;
use im_app_context::AppContext;
use im_domain_events::CommitEnvelope;
use im_platform_contracts::{CommitJournal, CommitPosition, ContractError};
use sdkwork_im_contract_agent::AgentSubject;

#[derive(Clone, Default)]
struct RecordingJournal {
    events: Arc<Mutex<Vec<CommitEnvelope>>>,
}

impl RecordingJournal {
    fn recorded(&self) -> Vec<CommitEnvelope> {
        self.events.lock().expect("journal should lock").clone()
    }
}

impl CommitJournal for RecordingJournal {
    fn append(&self, envelope: CommitEnvelope) -> Result<CommitPosition, ContractError> {
        let mut events = self.events.lock().expect("journal should lock");
        events.push(envelope);
        Ok(CommitPosition::new("p0", events.len() as u64))
    }
}

#[test]
fn test_agent_response_stream_and_tool_call_lifecycle_are_verifiable() {
    let journal = Arc::new(RecordingJournal::default());
    let runtime = automation_service::AutomationRuntime::with_journal(journal.clone());
    let auth = AppContext {
        tenant_id: "100001".into(),
        organization_id: "0".to_owned(),
        user_id: "1".into(),
        app_id: None,
        environment: None,
        deployment_mode: None,
        auth_level: None,
        data_scope: Default::default(),
        actor_id: "1".into(),
        actor_kind: "user".into(),
        session_id: Some("s_user".into()),
        device_id: None,
        permission_scope: BTreeSet::from([
            "automation.execute".to_string(),
            "automation.read".to_string(),
        ]),
    };

    runtime
        .request_execution(
            &auth,
            automation_service::RequestAutomationExecution {
                execution_id: "ae_agent".into(),
                trigger_type: "agent.manual".into(),
                target_kind: "conversation".into(),
                target_ref: "c_demo".into(),
                input_payload: Some(r#"{"prompt":"hello"}"#.into()),
            },
        )
        .expect("execution request should succeed");

    let started = runtime
        .start_agent_response(
            &auth,
            automation_service::StartAgentResponseRequest {
                execution_id: "ae_agent".into(),
                stream_id: "st_agent_demo".into(),
                stream_type: "agent.response.delta".into(),
                conversation_id: "c_demo".into(),
                schema_ref: Some("schema://agent/response.delta".into()),
                member_id: Some("cm_agent".into()),
                agent: AgentSubject {
                    agent_id: "agent.demo".into(),
                    session_id: Some("s_agent".into()),
                    metadata: BTreeMap::from([
                        ("agentMode".into(), "assistant".into()),
                        ("capabilityProfileId".into(), "stable-agent".into()),
                    ]),
                },
            },
        )
        .expect("agent response stream should start");
    assert_eq!(started.stream_id, "st_agent_demo");
    assert_eq!(started.state.as_wire_value(), "opened");

    let delta = runtime
        .append_agent_response_delta(
            &auth,
            "st_agent_demo",
            automation_service::AppendAgentResponseDeltaRequest {
                frame_seq: 1,
                frame_type: "delta.text".into(),
                schema_ref: Some("schema://agent/response.delta#chunk".into()),
                encoding: "json".into(),
                payload: r#"{"delta":"hello"}"#.into(),
                attributes: BTreeMap::from([("chunk".into(), "1".into())]),
            },
        )
        .expect("agent response delta should append");
    assert_eq!(delta.sender.id, "agent.demo");
    assert_eq!(delta.sender.kind, "agent");
    assert_eq!(delta.sender.member_id.as_deref(), Some("cm_agent"));
    assert_eq!(delta.sender.session_id.as_deref(), Some("s_agent"));

    let tool_requested = runtime
        .request_agent_tool_call(
            &auth,
            automation_service::RequestAgentToolCallRequest {
                execution_id: "ae_agent".into(),
                tool_call_id: "tc_lookup".into(),
                tool_name: "knowledge.search".into(),
                arguments_payload: r#"{"query":"hello"}"#.into(),
            },
        )
        .expect("tool call request should succeed");
    assert_eq!(tool_requested.state.as_str(), "requested");

    let tool_completed = runtime
        .complete_agent_tool_call(
            &auth,
            "ae_agent",
            "tc_lookup",
            automation_service::CompleteAgentToolCallRequest {
                result_payload: r#"{"hits":[{"id":"doc_1"}]}"#.into(),
            },
        )
        .expect("tool call completion should succeed");
    assert_eq!(tool_completed.state.as_str(), "completed");

    let completed = runtime
        .complete_agent_response(
            &auth,
            "st_agent_demo",
            automation_service::CompleteAgentResponseRequest {
                frame_seq: 1,
                result_message_id: Some("m_agent_final".into()),
            },
        )
        .expect("agent response stream should complete");
    assert_eq!(completed.state.as_wire_value(), "completed");
    assert_eq!(
        completed.result_message_id.as_deref(),
        Some("m_agent_final")
    );

    let events = journal.recorded();
    let event_types = events
        .iter()
        .map(|event| event.event_type.as_str())
        .collect::<Vec<_>>();
    assert_eq!(
        event_types,
        vec![
            "automation.execution_requested",
            "automation.execution_completed",
            "automation.agent_response_started",
            "automation.agent_response_delta",
            "automation.agent_tool_call_requested",
            "automation.agent_tool_call_completed",
            "automation.agent_response_completed",
        ]
    );

    let delta_payload: serde_json::Value =
        serde_json::from_str(&events[3].payload).expect("delta payload should be valid json");
    assert_eq!(delta_payload["sender"]["kind"], "agent");
    assert_eq!(delta_payload["sender"]["id"], "agent.demo");
    assert_eq!(delta_payload["frameType"], "delta.text");

    let tool_payload: serde_json::Value =
        serde_json::from_str(&events[5].payload).expect("tool payload should be valid json");
    assert_eq!(tool_payload["toolCallId"], "tc_lookup");
    assert_eq!(tool_payload["state"], "completed");
}

#[test]
fn test_same_event_type_records_use_distinct_idempotency_keys() {
    let journal = Arc::new(RecordingJournal::default());
    let runtime = automation_service::AutomationRuntime::with_journal(journal.clone());
    let auth = AppContext {
        tenant_id: "100001".into(),
        organization_id: "0".to_owned(),
        user_id: "1".into(),
        app_id: None,
        environment: None,
        deployment_mode: None,
        auth_level: None,
        data_scope: Default::default(),
        actor_id: "1".into(),
        actor_kind: "user".into(),
        session_id: Some("s_user".into()),
        device_id: None,
        permission_scope: BTreeSet::from([
            "automation.execute".to_string(),
            "automation.read".to_string(),
        ]),
    };

    runtime
        .request_execution(
            &auth,
            automation_service::RequestAutomationExecution {
                execution_id: "ae_idempotency_key_scope".into(),
                trigger_type: "agent.manual".into(),
                target_kind: "conversation".into(),
                target_ref: "c_demo".into(),
                input_payload: Some(r#"{"prompt":"hello"}"#.into()),
            },
        )
        .expect("execution request should succeed");
    runtime
        .start_agent_response(
            &auth,
            automation_service::StartAgentResponseRequest {
                execution_id: "ae_idempotency_key_scope".into(),
                stream_id: "st_idempotency_key_scope".into(),
                stream_type: "agent.response.delta".into(),
                conversation_id: "c_demo".into(),
                schema_ref: Some("schema://agent/response.delta".into()),
                member_id: Some("cm_agent".into()),
                agent: AgentSubject {
                    agent_id: "agent.demo".into(),
                    session_id: Some("s_agent".into()),
                    metadata: BTreeMap::from([("agentMode".into(), "assistant".into())]),
                },
            },
        )
        .expect("agent response stream should start");
    runtime
        .append_agent_response_delta(
            &auth,
            "st_idempotency_key_scope",
            automation_service::AppendAgentResponseDeltaRequest {
                frame_seq: 1,
                frame_type: "delta.text".into(),
                schema_ref: None,
                encoding: "json".into(),
                payload: r#"{"delta":"a"}"#.into(),
                attributes: BTreeMap::new(),
            },
        )
        .expect("first delta should append");
    runtime
        .append_agent_response_delta(
            &auth,
            "st_idempotency_key_scope",
            automation_service::AppendAgentResponseDeltaRequest {
                frame_seq: 2,
                frame_type: "delta.text".into(),
                schema_ref: None,
                encoding: "json".into(),
                payload: r#"{"delta":"b"}"#.into(),
                attributes: BTreeMap::new(),
            },
        )
        .expect("second delta should append");
    runtime
        .request_agent_tool_call(
            &auth,
            automation_service::RequestAgentToolCallRequest {
                execution_id: "ae_idempotency_key_scope".into(),
                tool_call_id: "tc_one".into(),
                tool_name: "knowledge.search".into(),
                arguments_payload: r#"{"query":"one"}"#.into(),
            },
        )
        .expect("first tool call should append");
    runtime
        .request_agent_tool_call(
            &auth,
            automation_service::RequestAgentToolCallRequest {
                execution_id: "ae_idempotency_key_scope".into(),
                tool_call_id: "tc_two".into(),
                tool_name: "knowledge.search".into(),
                arguments_payload: r#"{"query":"two"}"#.into(),
            },
        )
        .expect("second tool call should append");

    let events = journal.recorded();
    let delta_keys = events
        .iter()
        .filter(|event| event.event_type == "automation.agent_response_delta")
        .map(|event| {
            event
                .idempotency_key
                .clone()
                .expect("delta events should include idempotency_key")
        })
        .collect::<Vec<_>>();
    assert_eq!(delta_keys.len(), 2);
    let delta_key_unique_count = delta_keys.iter().collect::<BTreeSet<_>>().len();
    assert_eq!(
        delta_key_unique_count, 2,
        "delta events with same type must not reuse idempotency_key"
    );

    let tool_keys = events
        .iter()
        .filter(|event| event.event_type == "automation.agent_tool_call_requested")
        .map(|event| {
            event
                .idempotency_key
                .clone()
                .expect("tool events should include idempotency_key")
        })
        .collect::<Vec<_>>();
    assert_eq!(tool_keys.len(), 2);
    let tool_key_unique_count = tool_keys.iter().collect::<BTreeSet<_>>().len();
    assert_eq!(
        tool_key_unique_count, 2,
        "tool request events with same type must not reuse idempotency_key"
    );
}

#[test]
fn test_start_agent_response_rejects_stream_id_reuse_across_executions() {
    let runtime = automation_service::AutomationRuntime::default();
    let auth = AppContext {
        tenant_id: "100001".into(),
        organization_id: "0".to_owned(),
        user_id: "1".into(),
        app_id: None,
        environment: None,
        deployment_mode: None,
        auth_level: None,
        data_scope: Default::default(),
        actor_id: "1".into(),
        actor_kind: "user".into(),
        session_id: Some("s_user".into()),
        device_id: None,
        permission_scope: BTreeSet::from([
            "automation.execute".to_string(),
            "automation.read".to_string(),
        ]),
    };

    runtime
        .request_execution(
            &auth,
            automation_service::RequestAutomationExecution {
                execution_id: "ae_stream_reuse_1".into(),
                trigger_type: "agent.manual".into(),
                target_kind: "conversation".into(),
                target_ref: "c_demo".into(),
                input_payload: Some(r#"{"prompt":"first"}"#.into()),
            },
        )
        .expect("first execution request should succeed");
    runtime
        .request_execution(
            &auth,
            automation_service::RequestAutomationExecution {
                execution_id: "ae_stream_reuse_2".into(),
                trigger_type: "agent.manual".into(),
                target_kind: "conversation".into(),
                target_ref: "c_demo".into(),
                input_payload: Some(r#"{"prompt":"second"}"#.into()),
            },
        )
        .expect("second execution request should succeed");

    runtime
        .start_agent_response(
            &auth,
            automation_service::StartAgentResponseRequest {
                execution_id: "ae_stream_reuse_1".into(),
                stream_id: "st_reused".into(),
                stream_type: "agent.response.delta".into(),
                conversation_id: "c_demo".into(),
                schema_ref: Some("schema://agent/response.delta".into()),
                member_id: Some("cm_agent".into()),
                agent: AgentSubject {
                    agent_id: "agent.demo".into(),
                    session_id: Some("s_agent".into()),
                    metadata: BTreeMap::new(),
                },
            },
        )
        .expect("first stream start should succeed");

    let reused = runtime
        .start_agent_response(
            &auth,
            automation_service::StartAgentResponseRequest {
                execution_id: "ae_stream_reuse_2".into(),
                stream_id: "st_reused".into(),
                stream_type: "agent.response.delta".into(),
                conversation_id: "c_demo".into(),
                schema_ref: Some("schema://agent/response.delta".into()),
                member_id: Some("cm_agent".into()),
                agent: AgentSubject {
                    agent_id: "agent.demo".into(),
                    session_id: Some("s_agent".into()),
                    metadata: BTreeMap::new(),
                },
            },
        )
        .expect_err("stream id reuse across executions must be rejected");
    let reused_response = reused.into_response();
    assert_eq!(reused_response.status(), axum::http::StatusCode::CONFLICT);
}

#[test]
fn test_agent_response_stream_isolation_across_principal_kind() {
    let runtime = automation_service::AutomationRuntime::default();
    let user_auth = AppContext {
        tenant_id: "100001".into(),
        organization_id: "0".to_owned(),
        user_id: "1".into(),
        app_id: None,
        environment: None,
        deployment_mode: None,
        auth_level: None,
        data_scope: Default::default(),
        actor_id: "1".into(),
        actor_kind: "user".into(),
        session_id: Some("s_user".into()),
        device_id: None,
        permission_scope: BTreeSet::from([
            "automation.execute".to_string(),
            "automation.read".to_string(),
        ]),
    };
    let system_auth = AppContext {
        actor_kind: "system".into(),
        session_id: Some("s_system".into()),
        ..user_auth.clone()
    };

    runtime
        .request_execution(
            &user_auth,
            automation_service::RequestAutomationExecution {
                execution_id: "ae_stream_kind_isolation".into(),
                trigger_type: "agent.manual".into(),
                target_kind: "conversation".into(),
                target_ref: "c_demo".into(),
                input_payload: Some(r#"{"prompt":"hello"}"#.into()),
            },
        )
        .expect("user execution should succeed");
    runtime
        .start_agent_response(
            &user_auth,
            automation_service::StartAgentResponseRequest {
                execution_id: "ae_stream_kind_isolation".into(),
                stream_id: "st_kind_isolation".into(),
                stream_type: "agent.response.delta".into(),
                conversation_id: "c_demo".into(),
                schema_ref: Some("schema://agent/response.delta".into()),
                member_id: Some("cm_agent".into()),
                agent: AgentSubject {
                    agent_id: "agent.demo".into(),
                    session_id: Some("s_agent".into()),
                    metadata: BTreeMap::new(),
                },
            },
        )
        .expect("user stream should start");

    let hidden = runtime
        .append_agent_response_delta(
            &system_auth,
            "st_kind_isolation",
            automation_service::AppendAgentResponseDeltaRequest {
                frame_seq: 1,
                frame_type: "delta.text".into(),
                schema_ref: None,
                encoding: "json".into(),
                payload: r#"{"delta":"x"}"#.into(),
                attributes: BTreeMap::new(),
            },
        )
        .expect_err("different principal_kind should not access stream")
        .into_response();
    assert_eq!(hidden.status(), axum::http::StatusCode::NOT_FOUND);
}

#[test]
fn test_agent_response_scope_key_is_segment_safe_for_delimiter_bearing_ids() {
    let runtime = automation_service::AutomationRuntime::default();
    let first_auth = AppContext {
        tenant_id: "100001".into(),
        organization_id: "0".to_owned(),
        user_id: "1".into(),
        app_id: None,
        environment: None,
        deployment_mode: None,
        auth_level: None,
        data_scope: Default::default(),
        actor_id: "u:demo".into(),
        actor_kind: "user".into(),
        session_id: Some("s_first".into()),
        device_id: None,
        permission_scope: BTreeSet::from([
            "automation.execute".to_string(),
            "automation.read".to_string(),
        ]),
    };
    let second_auth = AppContext {
        actor_id: "u".into(),
        session_id: Some("s_second".into()),
        ..first_auth.clone()
    };

    runtime
        .request_execution(
            &first_auth,
            automation_service::RequestAutomationExecution {
                execution_id: "ae:first".into(),
                trigger_type: "agent.manual".into(),
                target_kind: "conversation".into(),
                target_ref: "c_demo".into(),
                input_payload: Some(r#"{"prompt":"first"}"#.into()),
            },
        )
        .expect("first execution should succeed");
    runtime
        .request_execution(
            &second_auth,
            automation_service::RequestAutomationExecution {
                execution_id: "ae:second".into(),
                trigger_type: "agent.manual".into(),
                target_kind: "conversation".into(),
                target_ref: "c_demo".into(),
                input_payload: Some(r#"{"prompt":"second"}"#.into()),
            },
        )
        .expect("second execution should succeed");

    runtime
        .start_agent_response(
            &first_auth,
            automation_service::StartAgentResponseRequest {
                execution_id: "ae:first".into(),
                stream_id: "st:demo".into(),
                stream_type: "agent.response.delta".into(),
                conversation_id: "c_demo".into(),
                schema_ref: Some("schema://agent/response.delta".into()),
                member_id: Some("cm_first".into()),
                agent: AgentSubject {
                    agent_id: "ag_first".into(),
                    session_id: Some("s_agent_first".into()),
                    metadata: BTreeMap::new(),
                },
            },
        )
        .expect("first delimiter-bearing stream should start");
    runtime
        .start_agent_response(
            &second_auth,
            automation_service::StartAgentResponseRequest {
                execution_id: "ae:second".into(),
                stream_id: "demo:st:demo".into(),
                stream_type: "agent.response.delta".into(),
                conversation_id: "c_demo".into(),
                schema_ref: Some("schema://agent/response.delta".into()),
                member_id: Some("cm_second".into()),
                agent: AgentSubject {
                    agent_id: "ag_second".into(),
                    session_id: Some("s_agent_second".into()),
                    metadata: BTreeMap::new(),
                },
            },
        )
        .expect("second delimiter-bearing stream should not collide with first");

    let first_delta = runtime
        .append_agent_response_delta(
            &first_auth,
            "st:demo",
            automation_service::AppendAgentResponseDeltaRequest {
                frame_seq: 1,
                frame_type: "delta.text".into(),
                schema_ref: None,
                encoding: "json".into(),
                payload: r#"{"delta":"first"}"#.into(),
                attributes: BTreeMap::new(),
            },
        )
        .expect("first stream append should use first stream state");
    let second_delta = runtime
        .append_agent_response_delta(
            &second_auth,
            "demo:st:demo",
            automation_service::AppendAgentResponseDeltaRequest {
                frame_seq: 1,
                frame_type: "delta.text".into(),
                schema_ref: None,
                encoding: "json".into(),
                payload: r#"{"delta":"second"}"#.into(),
                attributes: BTreeMap::new(),
            },
        )
        .expect("second stream append should use second stream state");

    assert_eq!(first_delta.stream_id, "st:demo");
    assert_eq!(first_delta.sender.id, "ag_first");
    assert_eq!(first_delta.sender.member_id.as_deref(), Some("cm_first"));
    assert_eq!(second_delta.stream_id, "demo:st:demo");
    assert_eq!(second_delta.sender.id, "ag_second");
    assert_eq!(second_delta.sender.member_id.as_deref(), Some("cm_second"));
}

#[test]
fn test_agent_tool_call_scope_key_is_segment_safe_for_delimiter_bearing_ids() {
    let runtime = automation_service::AutomationRuntime::default();
    let first_auth = AppContext {
        tenant_id: "100001".into(),
        organization_id: "0".to_owned(),
        user_id: "1".into(),
        app_id: None,
        environment: None,
        deployment_mode: None,
        auth_level: None,
        data_scope: Default::default(),
        actor_id: "u:demo".into(),
        actor_kind: "user".into(),
        session_id: Some("s_first".into()),
        device_id: None,
        permission_scope: BTreeSet::from([
            "automation.execute".to_string(),
            "automation.read".to_string(),
        ]),
    };
    let second_auth = AppContext {
        actor_id: "u".into(),
        session_id: Some("s_second".into()),
        ..first_auth.clone()
    };

    runtime
        .request_execution(
            &first_auth,
            automation_service::RequestAutomationExecution {
                execution_id: "ae:first".into(),
                trigger_type: "agent.manual".into(),
                target_kind: "conversation".into(),
                target_ref: "c_demo".into(),
                input_payload: Some(r#"{"prompt":"first"}"#.into()),
            },
        )
        .expect("first execution should succeed");
    runtime
        .start_agent_response(
            &first_auth,
            automation_service::StartAgentResponseRequest {
                execution_id: "ae:first".into(),
                stream_id: "st:first".into(),
                stream_type: "agent.response.delta".into(),
                conversation_id: "c_demo".into(),
                schema_ref: Some("schema://agent/response.delta".into()),
                member_id: Some("cm_first".into()),
                agent: AgentSubject {
                    agent_id: "ag_first".into(),
                    session_id: Some("s_agent_first".into()),
                    metadata: BTreeMap::new(),
                },
            },
        )
        .expect("first stream should start");
    runtime
        .request_execution(
            &second_auth,
            automation_service::RequestAutomationExecution {
                execution_id: "demo:ae:first".into(),
                trigger_type: "agent.manual".into(),
                target_kind: "conversation".into(),
                target_ref: "c_demo".into(),
                input_payload: Some(r#"{"prompt":"second"}"#.into()),
            },
        )
        .expect("second execution should succeed");
    runtime
        .start_agent_response(
            &second_auth,
            automation_service::StartAgentResponseRequest {
                execution_id: "demo:ae:first".into(),
                stream_id: "st:second".into(),
                stream_type: "agent.response.delta".into(),
                conversation_id: "c_demo".into(),
                schema_ref: Some("schema://agent/response.delta".into()),
                member_id: Some("cm_second".into()),
                agent: AgentSubject {
                    agent_id: "ag_second".into(),
                    session_id: Some("s_agent_second".into()),
                    metadata: BTreeMap::new(),
                },
            },
        )
        .expect("second stream should start");

    let first_tool = runtime
        .request_agent_tool_call(
            &first_auth,
            automation_service::RequestAgentToolCallRequest {
                execution_id: "ae:first".into(),
                tool_call_id: "tc:lookup".into(),
                tool_name: "knowledge.search".into(),
                arguments_payload: r#"{"query":"first"}"#.into(),
            },
        )
        .expect("first delimiter-bearing tool call should succeed");
    let second_tool = runtime
        .request_agent_tool_call(
            &second_auth,
            automation_service::RequestAgentToolCallRequest {
                execution_id: "demo:ae:first".into(),
                tool_call_id: "tc:lookup".into(),
                tool_name: "knowledge.search".into(),
                arguments_payload: r#"{"query":"second"}"#.into(),
            },
        )
        .expect("second delimiter-bearing tool call should not collide with first");

    assert_eq!(first_tool.agent_id, "ag_first");
    assert_eq!(first_tool.arguments_payload, r#"{"query":"first"}"#);
    assert_eq!(second_tool.agent_id, "ag_second");
    assert_eq!(second_tool.arguments_payload, r#"{"query":"second"}"#);

    let first_completed = runtime
        .complete_agent_tool_call(
            &first_auth,
            "ae:first",
            "tc:lookup",
            automation_service::CompleteAgentToolCallRequest {
                result_payload: r#"{"result":"first"}"#.into(),
            },
        )
        .expect("first tool call should complete independently");
    let second_completed = runtime
        .complete_agent_tool_call(
            &second_auth,
            "demo:ae:first",
            "tc:lookup",
            automation_service::CompleteAgentToolCallRequest {
                result_payload: r#"{"result":"second"}"#.into(),
            },
        )
        .expect("second tool call should complete independently");

    assert_eq!(
        first_completed.result_payload.as_deref(),
        Some(r#"{"result":"first"}"#)
    );
    assert_eq!(
        second_completed.result_payload.as_deref(),
        Some(r#"{"result":"second"}"#)
    );
}

#[test]
fn test_start_agent_response_rejects_second_stream_for_same_execution() {
    let runtime = automation_service::AutomationRuntime::default();
    let auth = AppContext {
        tenant_id: "100001".into(),
        organization_id: "0".to_owned(),
        user_id: "1".into(),
        app_id: None,
        environment: None,
        deployment_mode: None,
        auth_level: None,
        data_scope: Default::default(),
        actor_id: "1".into(),
        actor_kind: "user".into(),
        session_id: Some("s_user".into()),
        device_id: None,
        permission_scope: BTreeSet::from([
            "automation.execute".to_string(),
            "automation.read".to_string(),
        ]),
    };

    runtime
        .request_execution(
            &auth,
            automation_service::RequestAutomationExecution {
                execution_id: "ae_single_stream".into(),
                trigger_type: "agent.manual".into(),
                target_kind: "conversation".into(),
                target_ref: "c_demo".into(),
                input_payload: Some(r#"{"prompt":"hello"}"#.into()),
            },
        )
        .expect("execution should succeed");

    runtime
        .start_agent_response(
            &auth,
            automation_service::StartAgentResponseRequest {
                execution_id: "ae_single_stream".into(),
                stream_id: "st_primary".into(),
                stream_type: "agent.response.delta".into(),
                conversation_id: "c_demo".into(),
                schema_ref: Some("schema://agent/response.delta".into()),
                member_id: Some("cm_agent".into()),
                agent: AgentSubject {
                    agent_id: "agent.demo".into(),
                    session_id: Some("s_agent".into()),
                    metadata: BTreeMap::new(),
                },
            },
        )
        .expect("primary stream should start");

    let duplicated = runtime
        .start_agent_response(
            &auth,
            automation_service::StartAgentResponseRequest {
                execution_id: "ae_single_stream".into(),
                stream_id: "st_secondary".into(),
                stream_type: "agent.response.delta".into(),
                conversation_id: "c_demo".into(),
                schema_ref: Some("schema://agent/response.delta".into()),
                member_id: Some("cm_agent".into()),
                agent: AgentSubject {
                    agent_id: "agent.demo".into(),
                    session_id: Some("s_agent".into()),
                    metadata: BTreeMap::new(),
                },
            },
        )
        .expect_err("same execution must not accept a second stream");
    let duplicated_response = duplicated.into_response();
    assert_eq!(
        duplicated_response.status(),
        axum::http::StatusCode::CONFLICT
    );
}

#[test]
fn test_request_tool_call_rejects_when_agent_response_stream_completed() {
    let runtime = automation_service::AutomationRuntime::default();
    let auth = AppContext {
        tenant_id: "100001".into(),
        organization_id: "0".to_owned(),
        user_id: "1".into(),
        app_id: None,
        environment: None,
        deployment_mode: None,
        auth_level: None,
        data_scope: Default::default(),
        actor_id: "1".into(),
        actor_kind: "user".into(),
        session_id: Some("s_user".into()),
        device_id: None,
        permission_scope: BTreeSet::from([
            "automation.execute".to_string(),
            "automation.read".to_string(),
        ]),
    };

    runtime
        .request_execution(
            &auth,
            automation_service::RequestAutomationExecution {
                execution_id: "ae_tool_after_complete".into(),
                trigger_type: "agent.manual".into(),
                target_kind: "conversation".into(),
                target_ref: "c_demo".into(),
                input_payload: Some(r#"{"prompt":"hello"}"#.into()),
            },
        )
        .expect("execution should succeed");
    runtime
        .start_agent_response(
            &auth,
            automation_service::StartAgentResponseRequest {
                execution_id: "ae_tool_after_complete".into(),
                stream_id: "st_tool_after_complete".into(),
                stream_type: "agent.response.delta".into(),
                conversation_id: "c_demo".into(),
                schema_ref: Some("schema://agent/response.delta".into()),
                member_id: Some("cm_agent".into()),
                agent: AgentSubject {
                    agent_id: "agent.demo".into(),
                    session_id: Some("s_agent".into()),
                    metadata: BTreeMap::new(),
                },
            },
        )
        .expect("stream should start");
    runtime
        .complete_agent_response(
            &auth,
            "st_tool_after_complete",
            automation_service::CompleteAgentResponseRequest {
                frame_seq: 0,
                result_message_id: Some("m_done".into()),
            },
        )
        .expect("stream completion should succeed");

    let closed = runtime
        .request_agent_tool_call(
            &auth,
            automation_service::RequestAgentToolCallRequest {
                execution_id: "ae_tool_after_complete".into(),
                tool_call_id: "tc_after_complete".into(),
                tool_name: "knowledge.search".into(),
                arguments_payload: r#"{"query":"hello"}"#.into(),
            },
        )
        .expect_err("closed stream must reject new tool call")
        .into_response();
    assert_eq!(closed.status(), axum::http::StatusCode::BAD_REQUEST);
}

#[test]
fn test_complete_agent_response_rejects_when_tool_call_pending() {
    let runtime = automation_service::AutomationRuntime::default();
    let auth = AppContext {
        tenant_id: "100001".into(),
        organization_id: "0".to_owned(),
        user_id: "1".into(),
        app_id: None,
        environment: None,
        deployment_mode: None,
        auth_level: None,
        data_scope: Default::default(),
        actor_id: "1".into(),
        actor_kind: "user".into(),
        session_id: Some("s_user".into()),
        device_id: None,
        permission_scope: BTreeSet::from([
            "automation.execute".to_string(),
            "automation.read".to_string(),
        ]),
    };

    runtime
        .request_execution(
            &auth,
            automation_service::RequestAutomationExecution {
                execution_id: "ae_pending_tool_guard".into(),
                trigger_type: "agent.manual".into(),
                target_kind: "conversation".into(),
                target_ref: "c_demo".into(),
                input_payload: Some(r#"{"prompt":"hello"}"#.into()),
            },
        )
        .expect("execution should succeed");
    runtime
        .start_agent_response(
            &auth,
            automation_service::StartAgentResponseRequest {
                execution_id: "ae_pending_tool_guard".into(),
                stream_id: "st_pending_tool_guard".into(),
                stream_type: "agent.response.delta".into(),
                conversation_id: "c_demo".into(),
                schema_ref: Some("schema://agent/response.delta".into()),
                member_id: Some("cm_agent".into()),
                agent: AgentSubject {
                    agent_id: "agent.demo".into(),
                    session_id: Some("s_agent".into()),
                    metadata: BTreeMap::new(),
                },
            },
        )
        .expect("stream should start");
    runtime
        .request_agent_tool_call(
            &auth,
            automation_service::RequestAgentToolCallRequest {
                execution_id: "ae_pending_tool_guard".into(),
                tool_call_id: "tc_pending".into(),
                tool_name: "knowledge.search".into(),
                arguments_payload: r#"{"query":"hello"}"#.into(),
            },
        )
        .expect("tool call should be requested");

    let blocked = runtime
        .complete_agent_response(
            &auth,
            "st_pending_tool_guard",
            automation_service::CompleteAgentResponseRequest {
                frame_seq: 0,
                result_message_id: Some("m_done".into()),
            },
        )
        .expect_err("pending tool call must block stream completion")
        .into_response();
    assert_eq!(blocked.status(), axum::http::StatusCode::BAD_REQUEST);

    runtime
        .complete_agent_tool_call(
            &auth,
            "ae_pending_tool_guard",
            "tc_pending",
            automation_service::CompleteAgentToolCallRequest {
                result_payload: r#"{"hits":[]}"#.into(),
            },
        )
        .expect("tool call completion should succeed");

    let completed = runtime
        .complete_agent_response(
            &auth,
            "st_pending_tool_guard",
            automation_service::CompleteAgentResponseRequest {
                frame_seq: 0,
                result_message_id: Some("m_done".into()),
            },
        )
        .expect("stream completion should succeed after tool call completion");
    assert_eq!(completed.state.as_wire_value(), "completed");
}

#[tokio::test]
async fn test_restricted_tool_call_requires_operator_override_and_is_auditable() {
    let journal = Arc::new(RecordingJournal::default());
    let runtime = automation_service::AutomationRuntime::with_journal(journal.clone());
    let auth = AppContext {
        tenant_id: "100001".into(),
        organization_id: "0".to_owned(),
        user_id: "1".into(),
        app_id: None,
        environment: None,
        deployment_mode: None,
        auth_level: None,
        data_scope: Default::default(),
        actor_id: "1".into(),
        actor_kind: "user".into(),
        session_id: Some("s_user".into()),
        device_id: None,
        permission_scope: BTreeSet::from([
            "automation.execute".to_string(),
            "automation.read".to_string(),
        ]),
    };

    runtime
        .request_execution(
            &auth,
            automation_service::RequestAutomationExecution {
                execution_id: "ae_guardrail".into(),
                trigger_type: "agent.manual".into(),
                target_kind: "conversation".into(),
                target_ref: "c_demo".into(),
                input_payload: Some(r#"{"prompt":"shutdown"}"#.into()),
            },
        )
        .expect("execution request should succeed");

    runtime
        .start_agent_response(
            &auth,
            automation_service::StartAgentResponseRequest {
                execution_id: "ae_guardrail".into(),
                stream_id: "st_guardrail".into(),
                stream_type: "agent.response.delta".into(),
                conversation_id: "c_demo".into(),
                schema_ref: Some("schema://agent/response.delta".into()),
                member_id: Some("cm_agent".into()),
                agent: AgentSubject {
                    agent_id: "agent.demo".into(),
                    session_id: Some("s_agent".into()),
                    metadata: BTreeMap::from([
                        ("agentMode".into(), "assistant".into()),
                        ("capabilityProfileId".into(), "stable-agent".into()),
                    ]),
                },
            },
        )
        .expect("agent response stream should start");

    let denied = runtime
        .request_agent_tool_call(
            &auth,
            automation_service::RequestAgentToolCallRequest {
                execution_id: "ae_guardrail".into(),
                tool_call_id: "tc_guardrail_denied".into(),
                tool_name: "ops.shutdown".into(),
                arguments_payload: r#"{"scope":"tenant"}"#.into(),
            },
        )
        .expect_err("restricted tool call should be rejected before operator override");
    let denied_response = denied.into_response();
    assert_eq!(denied_response.status(), axum::http::StatusCode::FORBIDDEN);
    let denied_body = denied_response
        .into_body()
        .collect()
        .await
        .expect("guardrail denied body should collect")
        .to_bytes();
    let denied_json: serde_json::Value =
        serde_json::from_slice(&denied_body).expect("guardrail denied body should be valid json");
    assert_eq!(denied_json["code"], "automation_guardrail_denied");

    let override_auth = AppContext {
        permission_scope: BTreeSet::from([
            "automation.execute".to_string(),
            "automation.read".to_string(),
            "automation.operator_override".to_string(),
        ]),
        ..auth.clone()
    };

    let tool_requested = runtime
        .request_agent_tool_call(
            &override_auth,
            automation_service::RequestAgentToolCallRequest {
                execution_id: "ae_guardrail".into(),
                tool_call_id: "tc_guardrail_allowed".into(),
                tool_name: "ops.shutdown".into(),
                arguments_payload: r#"{"scope":"tenant"}"#.into(),
            },
        )
        .expect("operator override should allow restricted tool call");
    assert_eq!(tool_requested.state.as_str(), "requested");

    let events = journal.recorded();
    let event_types = events
        .iter()
        .map(|event| event.event_type.as_str())
        .collect::<Vec<_>>();
    assert_eq!(
        event_types,
        vec![
            "automation.execution_requested",
            "automation.execution_completed",
            "automation.agent_response_started",
            "automation.guardrail_denied",
            "automation.operator_override_applied",
            "automation.agent_tool_call_requested",
        ]
    );

    let denied_payload: serde_json::Value = serde_json::from_str(&events[3].payload)
        .expect("guardrail denied payload should be valid json");
    assert_eq!(denied_payload["toolName"], "ops.shutdown");
    assert_eq!(
        denied_payload["operatorOverridePermission"],
        "automation.operator_override"
    );

    let override_payload: serde_json::Value =
        serde_json::from_str(&events[4].payload).expect("override payload should be valid json");
    assert_eq!(override_payload["toolName"], "ops.shutdown");
    assert_eq!(override_payload["operatorOverrideActive"], true);
}

#[tokio::test]
async fn test_agent_runtime_rejects_oversized_delta_and_tool_payloads() {
    let runtime = automation_service::AutomationRuntime::default();
    let auth = AppContext {
        tenant_id: "100001".into(),
        organization_id: "0".to_owned(),
        user_id: "1".into(),
        app_id: None,
        environment: None,
        deployment_mode: None,
        auth_level: None,
        data_scope: Default::default(),
        actor_id: "1".into(),
        actor_kind: "user".into(),
        session_id: Some("s_user".into()),
        device_id: None,
        permission_scope: BTreeSet::from([
            "automation.execute".to_string(),
            "automation.read".to_string(),
        ]),
    };

    runtime
        .request_execution(
            &auth,
            automation_service::RequestAutomationExecution {
                execution_id: "ae_payload_guard".into(),
                trigger_type: "agent.manual".into(),
                target_kind: "conversation".into(),
                target_ref: "c_demo".into(),
                input_payload: Some(r#"{"prompt":"hello"}"#.into()),
            },
        )
        .expect("execution request should succeed");

    runtime
        .start_agent_response(
            &auth,
            automation_service::StartAgentResponseRequest {
                execution_id: "ae_payload_guard".into(),
                stream_id: "st_payload_guard".into(),
                stream_type: "agent.response.delta".into(),
                conversation_id: "c_demo".into(),
                schema_ref: Some("schema://agent/response.delta".into()),
                member_id: Some("cm_agent".into()),
                agent: AgentSubject {
                    agent_id: "agent.demo".into(),
                    session_id: Some("s_agent".into()),
                    metadata: BTreeMap::from([
                        ("agentMode".into(), "assistant".into()),
                        ("capabilityProfileId".into(), "stable-agent".into()),
                    ]),
                },
            },
        )
        .expect("agent response stream should start");

    let oversized_delta = runtime
        .append_agent_response_delta(
            &auth,
            "st_payload_guard",
            automation_service::AppendAgentResponseDeltaRequest {
                frame_seq: 1,
                frame_type: "delta.text".into(),
                schema_ref: Some("schema://agent/response.delta#chunk".into()),
                encoding: "json".into(),
                payload: "x".repeat(262145),
                attributes: BTreeMap::new(),
            },
        )
        .expect_err("oversized frame payload should be rejected")
        .into_response();
    assert_eq!(
        oversized_delta.status(),
        axum::http::StatusCode::PAYLOAD_TOO_LARGE
    );
    let oversized_delta_body = oversized_delta
        .into_body()
        .collect()
        .await
        .expect("oversized delta body should collect")
        .to_bytes();
    let oversized_delta_json: serde_json::Value = serde_json::from_slice(&oversized_delta_body)
        .expect("oversized delta body should be valid json");
    assert_eq!(oversized_delta_json["code"], "payload_too_large");

    let oversized_attributes = runtime
        .append_agent_response_delta(
            &auth,
            "st_payload_guard",
            automation_service::AppendAgentResponseDeltaRequest {
                frame_seq: 1,
                frame_type: "delta.text".into(),
                schema_ref: Some("schema://agent/response.delta#chunk".into()),
                encoding: "json".into(),
                payload: r#"{"delta":"hello"}"#.into(),
                attributes: BTreeMap::from([("trace".into(), "x".repeat(65537))]),
            },
        )
        .expect_err("oversized delta attributes should be rejected")
        .into_response();
    assert_eq!(
        oversized_attributes.status(),
        axum::http::StatusCode::PAYLOAD_TOO_LARGE
    );

    let oversized_tool_arguments = runtime
        .request_agent_tool_call(
            &auth,
            automation_service::RequestAgentToolCallRequest {
                execution_id: "ae_payload_guard".into(),
                tool_call_id: "tc_payload_guard_reject".into(),
                tool_name: "knowledge.search".into(),
                arguments_payload: "x".repeat(131073),
            },
        )
        .expect_err("oversized tool arguments payload should be rejected")
        .into_response();
    assert_eq!(
        oversized_tool_arguments.status(),
        axum::http::StatusCode::PAYLOAD_TOO_LARGE
    );

    runtime
        .request_agent_tool_call(
            &auth,
            automation_service::RequestAgentToolCallRequest {
                execution_id: "ae_payload_guard".into(),
                tool_call_id: "tc_payload_guard_complete".into(),
                tool_name: "knowledge.search".into(),
                arguments_payload: r#"{"query":"hello"}"#.into(),
            },
        )
        .expect("normal tool request should succeed");

    let oversized_tool_result = runtime
        .complete_agent_tool_call(
            &auth,
            "ae_payload_guard",
            "tc_payload_guard_complete",
            automation_service::CompleteAgentToolCallRequest {
                result_payload: "x".repeat(262145),
            },
        )
        .expect_err("oversized tool result payload should be rejected")
        .into_response();
    assert_eq!(
        oversized_tool_result.status(),
        axum::http::StatusCode::PAYLOAD_TOO_LARGE
    );
}

#[test]
fn test_append_agent_response_delta_rejects_oversized_stream_id_path() {
    let runtime = automation_service::AutomationRuntime::default();
    let auth = AppContext {
        tenant_id: "100001".into(),
        organization_id: "0".to_owned(),
        user_id: "1".into(),
        app_id: None,
        environment: None,
        deployment_mode: None,
        auth_level: None,
        data_scope: Default::default(),
        actor_id: "1".into(),
        actor_kind: "user".into(),
        session_id: Some("s_user".into()),
        device_id: None,
        permission_scope: BTreeSet::from([
            "automation.execute".to_string(),
            "automation.read".to_string(),
        ]),
    };

    let error = runtime
        .append_agent_response_delta(
            &auth,
            "s".repeat(257).as_str(),
            automation_service::AppendAgentResponseDeltaRequest {
                frame_seq: 1,
                frame_type: "delta.text".into(),
                schema_ref: Some("schema://agent/response.delta#chunk".into()),
                encoding: "json".into(),
                payload: r#"{"delta":"hello"}"#.into(),
                attributes: BTreeMap::new(),
            },
        )
        .expect_err("oversized stream id should be rejected")
        .into_response();
    assert_eq!(error.status(), axum::http::StatusCode::PAYLOAD_TOO_LARGE);
}

#[test]
fn test_complete_agent_response_rejects_oversized_stream_id_path() {
    let runtime = automation_service::AutomationRuntime::default();
    let auth = AppContext {
        tenant_id: "100001".into(),
        organization_id: "0".to_owned(),
        user_id: "1".into(),
        app_id: None,
        environment: None,
        deployment_mode: None,
        auth_level: None,
        data_scope: Default::default(),
        actor_id: "1".into(),
        actor_kind: "user".into(),
        session_id: Some("s_user".into()),
        device_id: None,
        permission_scope: BTreeSet::from([
            "automation.execute".to_string(),
            "automation.read".to_string(),
        ]),
    };

    let error = runtime
        .complete_agent_response(
            &auth,
            "s".repeat(257).as_str(),
            automation_service::CompleteAgentResponseRequest {
                frame_seq: 1,
                result_message_id: Some("m_done".into()),
            },
        )
        .expect_err("oversized stream id should be rejected")
        .into_response();
    assert_eq!(error.status(), axum::http::StatusCode::PAYLOAD_TOO_LARGE);
}

#[test]
fn test_complete_agent_tool_call_rejects_oversized_path_ids() {
    let runtime = automation_service::AutomationRuntime::default();
    let auth = AppContext {
        tenant_id: "100001".into(),
        organization_id: "0".to_owned(),
        user_id: "1".into(),
        app_id: None,
        environment: None,
        deployment_mode: None,
        auth_level: None,
        data_scope: Default::default(),
        actor_id: "1".into(),
        actor_kind: "user".into(),
        session_id: Some("s_user".into()),
        device_id: None,
        permission_scope: BTreeSet::from([
            "automation.execute".to_string(),
            "automation.read".to_string(),
        ]),
    };

    for (field, execution_id, tool_call_id) in [
        ("executionId", "e".repeat(257), "tc_demo".to_string()),
        ("toolCallId", "ae_demo".to_string(), "t".repeat(257)),
    ] {
        let error = runtime
            .complete_agent_tool_call(
                &auth,
                execution_id.as_str(),
                tool_call_id.as_str(),
                automation_service::CompleteAgentToolCallRequest {
                    result_payload: r#"{"hits":[{"id":"doc_1"}]}"#.into(),
                },
            )
            .expect_err("oversized path id should be rejected")
            .into_response();
        assert_eq!(
            error.status(),
            axum::http::StatusCode::PAYLOAD_TOO_LARGE,
            "{field} should be rejected with payload_too_large"
        );
    }
}

#[tokio::test]
async fn test_agent_runtime_rejects_oversized_delta_contract_fields() {
    let runtime = automation_service::AutomationRuntime::default();
    let auth = AppContext {
        tenant_id: "100001".into(),
        organization_id: "0".to_owned(),
        user_id: "1".into(),
        app_id: None,
        environment: None,
        deployment_mode: None,
        auth_level: None,
        data_scope: Default::default(),
        actor_id: "1".into(),
        actor_kind: "user".into(),
        session_id: Some("s_user".into()),
        device_id: None,
        permission_scope: BTreeSet::from([
            "automation.execute".to_string(),
            "automation.read".to_string(),
        ]),
    };

    runtime
        .request_execution(
            &auth,
            automation_service::RequestAutomationExecution {
                execution_id: "ae_delta_contract_guard".into(),
                trigger_type: "agent.manual".into(),
                target_kind: "conversation".into(),
                target_ref: "c_demo".into(),
                input_payload: Some(r#"{"prompt":"hello"}"#.into()),
            },
        )
        .expect("execution request should succeed");

    runtime
        .start_agent_response(
            &auth,
            automation_service::StartAgentResponseRequest {
                execution_id: "ae_delta_contract_guard".into(),
                stream_id: "st_delta_contract_guard".into(),
                stream_type: "agent.response.delta".into(),
                conversation_id: "c_demo".into(),
                schema_ref: Some("schema://agent/response.delta".into()),
                member_id: Some("cm_agent".into()),
                agent: AgentSubject {
                    agent_id: "agent.demo".into(),
                    session_id: Some("s_agent".into()),
                    metadata: BTreeMap::from([
                        ("agentMode".into(), "assistant".into()),
                        ("capabilityProfileId".into(), "stable-agent".into()),
                    ]),
                },
            },
        )
        .expect("agent response stream should start");

    let cases = [
        (
            "frameType",
            automation_service::AppendAgentResponseDeltaRequest {
                frame_seq: 1,
                frame_type: "f".repeat(65),
                schema_ref: Some("schema://agent/response.delta#chunk".into()),
                encoding: "json".into(),
                payload: r#"{"delta":"hello"}"#.into(),
                attributes: BTreeMap::new(),
            },
        ),
        (
            "encoding",
            automation_service::AppendAgentResponseDeltaRequest {
                frame_seq: 2,
                frame_type: "delta.text".into(),
                schema_ref: Some("schema://agent/response.delta#chunk".into()),
                encoding: "j".repeat(33),
                payload: r#"{"delta":"hello"}"#.into(),
                attributes: BTreeMap::new(),
            },
        ),
        (
            "schemaRef",
            automation_service::AppendAgentResponseDeltaRequest {
                frame_seq: 3,
                frame_type: "delta.text".into(),
                schema_ref: Some("s".repeat(257)),
                encoding: "json".into(),
                payload: r#"{"delta":"hello"}"#.into(),
                attributes: BTreeMap::new(),
            },
        ),
    ];

    for (field, request) in cases {
        let error = runtime.append_agent_response_delta(&auth, "st_delta_contract_guard", request);
        let error = match error {
            Ok(frame) => panic!("{field} should be rejected: {frame:?}"),
            Err(error) => error,
        };
        let response = error.into_response();
        assert_eq!(
            response.status(),
            axum::http::StatusCode::PAYLOAD_TOO_LARGE,
            "{field} should be rejected with payload_too_large"
        );
    }
}

#[test]
fn test_start_agent_response_rejects_oversized_stream_id() {
    let runtime = automation_service::AutomationRuntime::default();
    let auth = AppContext {
        tenant_id: "100001".into(),
        organization_id: "0".to_owned(),
        user_id: "1".into(),
        app_id: None,
        environment: None,
        deployment_mode: None,
        auth_level: None,
        data_scope: Default::default(),
        actor_id: "1".into(),
        actor_kind: "user".into(),
        session_id: Some("s_user".into()),
        device_id: None,
        permission_scope: BTreeSet::from([
            "automation.execute".to_string(),
            "automation.read".to_string(),
        ]),
    };

    runtime
        .request_execution(
            &auth,
            automation_service::RequestAutomationExecution {
                execution_id: "ae_oversized_stream_id".into(),
                trigger_type: "agent.manual".into(),
                target_kind: "conversation".into(),
                target_ref: "c_demo".into(),
                input_payload: Some(r#"{"prompt":"hello"}"#.into()),
            },
        )
        .expect("execution request should succeed");

    let error = runtime
        .start_agent_response(
            &auth,
            automation_service::StartAgentResponseRequest {
                execution_id: "ae_oversized_stream_id".into(),
                stream_id: "s".repeat(257),
                stream_type: "agent.response.delta".into(),
                conversation_id: "c_demo".into(),
                schema_ref: Some("schema://agent/response.delta".into()),
                member_id: Some("cm_agent".into()),
                agent: AgentSubject {
                    agent_id: "agent.demo".into(),
                    session_id: Some("s_agent".into()),
                    metadata: BTreeMap::from([
                        ("agentMode".into(), "assistant".into()),
                        ("capabilityProfileId".into(), "stable-agent".into()),
                    ]),
                },
            },
        )
        .expect_err("oversized stream id should be rejected");
    let response = axum::response::IntoResponse::into_response(error);
    assert_eq!(response.status(), axum::http::StatusCode::PAYLOAD_TOO_LARGE);
}

#[test]
fn test_start_agent_response_rejects_oversized_stream_contract_fields() {
    let runtime = automation_service::AutomationRuntime::default();
    let auth = AppContext {
        tenant_id: "100001".into(),
        organization_id: "0".to_owned(),
        user_id: "1".into(),
        app_id: None,
        environment: None,
        deployment_mode: None,
        auth_level: None,
        data_scope: Default::default(),
        actor_id: "1".into(),
        actor_kind: "user".into(),
        session_id: Some("s_user".into()),
        device_id: None,
        permission_scope: BTreeSet::from([
            "automation.execute".to_string(),
            "automation.read".to_string(),
        ]),
    };
    let cases = [
        (
            "streamType",
            "ae_oversized_stream_type",
            "st_oversized_stream_type",
            "x".repeat(129),
            "c_demo".to_string(),
            Some("schema://agent/response.delta".to_string()),
        ),
        (
            "conversationId",
            "ae_oversized_conversation_id",
            "st_oversized_conversation_id",
            "agent.response.delta".to_string(),
            "c".repeat(257),
            Some("schema://agent/response.delta".to_string()),
        ),
        (
            "schemaRef",
            "ae_oversized_schema_ref",
            "st_oversized_schema_ref",
            "agent.response.delta".to_string(),
            "c_demo".to_string(),
            Some("s".repeat(257)),
        ),
    ];

    for (field, execution_id, stream_id, stream_type, conversation_id, schema_ref) in cases {
        runtime
            .request_execution(
                &auth,
                automation_service::RequestAutomationExecution {
                    execution_id: execution_id.into(),
                    trigger_type: "agent.manual".into(),
                    target_kind: "conversation".into(),
                    target_ref: "c_demo".into(),
                    input_payload: Some(r#"{"prompt":"hello"}"#.into()),
                },
            )
            .expect("execution request should succeed");

        let error = runtime.start_agent_response(
            &auth,
            automation_service::StartAgentResponseRequest {
                execution_id: execution_id.into(),
                stream_id: stream_id.into(),
                stream_type,
                conversation_id,
                schema_ref,
                member_id: Some("cm_agent".into()),
                agent: AgentSubject {
                    agent_id: "agent.demo".into(),
                    session_id: Some("s_agent".into()),
                    metadata: BTreeMap::from([
                        ("agentMode".into(), "assistant".into()),
                        ("capabilityProfileId".into(), "stable-agent".into()),
                    ]),
                },
            },
        );
        let error = match error {
            Ok(session) => panic!("{field} should be rejected: {session:?}"),
            Err(error) => error,
        };
        let response = axum::response::IntoResponse::into_response(error);
        assert_eq!(
            response.status(),
            axum::http::StatusCode::PAYLOAD_TOO_LARGE,
            "{field} should be rejected with payload_too_large"
        );
    }
}

#[test]
fn test_start_agent_response_rejects_oversized_member_id() {
    let runtime = automation_service::AutomationRuntime::default();
    let auth = AppContext {
        tenant_id: "100001".into(),
        organization_id: "0".to_owned(),
        user_id: "1".into(),
        app_id: None,
        environment: None,
        deployment_mode: None,
        auth_level: None,
        data_scope: Default::default(),
        actor_id: "1".into(),
        actor_kind: "user".into(),
        session_id: Some("s_user".into()),
        device_id: None,
        permission_scope: BTreeSet::from([
            "automation.execute".to_string(),
            "automation.read".to_string(),
        ]),
    };

    runtime
        .request_execution(
            &auth,
            automation_service::RequestAutomationExecution {
                execution_id: "ae_oversized_member_id".into(),
                trigger_type: "agent.manual".into(),
                target_kind: "conversation".into(),
                target_ref: "c_demo".into(),
                input_payload: Some(r#"{"prompt":"hello"}"#.into()),
            },
        )
        .expect("execution request should succeed");

    let error = runtime.start_agent_response(
        &auth,
        automation_service::StartAgentResponseRequest {
            execution_id: "ae_oversized_member_id".into(),
            stream_id: "st_oversized_member_id".into(),
            stream_type: "agent.response.delta".into(),
            conversation_id: "c_demo".into(),
            schema_ref: Some("schema://agent/response.delta".into()),
            member_id: Some("m".repeat(257)),
            agent: AgentSubject {
                agent_id: "agent.demo".into(),
                session_id: Some("s_agent".into()),
                metadata: BTreeMap::from([
                    ("agentMode".into(), "assistant".into()),
                    ("capabilityProfileId".into(), "stable-agent".into()),
                ]),
            },
        },
    );
    let error = match error {
        Ok(session) => panic!("memberId should be rejected: {session:?}"),
        Err(error) => error,
    };
    let response = axum::response::IntoResponse::into_response(error);
    assert_eq!(response.status(), axum::http::StatusCode::PAYLOAD_TOO_LARGE);
}

#[test]
fn test_start_agent_response_rejects_oversized_execution_id() {
    let runtime = automation_service::AutomationRuntime::default();
    let auth = AppContext {
        tenant_id: "100001".into(),
        organization_id: "0".to_owned(),
        user_id: "1".into(),
        app_id: None,
        environment: None,
        deployment_mode: None,
        auth_level: None,
        data_scope: Default::default(),
        actor_id: "1".into(),
        actor_kind: "user".into(),
        session_id: Some("s_user".into()),
        device_id: None,
        permission_scope: BTreeSet::from([
            "automation.execute".to_string(),
            "automation.read".to_string(),
        ]),
    };

    let error = runtime
        .start_agent_response(
            &auth,
            automation_service::StartAgentResponseRequest {
                execution_id: "e".repeat(257),
                stream_id: "st_oversized_start_execution_id".into(),
                stream_type: "agent.response.delta".into(),
                conversation_id: "c_demo".into(),
                schema_ref: Some("schema://agent/response.delta".into()),
                member_id: Some("cm_agent".into()),
                agent: AgentSubject {
                    agent_id: "agent.demo".into(),
                    session_id: Some("s_agent".into()),
                    metadata: BTreeMap::from([
                        ("agentMode".into(), "assistant".into()),
                        ("capabilityProfileId".into(), "stable-agent".into()),
                    ]),
                },
            },
        )
        .expect_err("oversized execution id should be rejected");
    let response = axum::response::IntoResponse::into_response(error);
    assert_eq!(response.status(), axum::http::StatusCode::PAYLOAD_TOO_LARGE);
}

#[test]
fn test_start_agent_response_rejects_oversized_agent_metadata() {
    let runtime = automation_service::AutomationRuntime::default();
    let auth = AppContext {
        tenant_id: "100001".into(),
        organization_id: "0".to_owned(),
        user_id: "1".into(),
        app_id: None,
        environment: None,
        deployment_mode: None,
        auth_level: None,
        data_scope: Default::default(),
        actor_id: "1".into(),
        actor_kind: "user".into(),
        session_id: Some("s_user".into()),
        device_id: None,
        permission_scope: BTreeSet::from([
            "automation.execute".to_string(),
            "automation.read".to_string(),
        ]),
    };

    runtime
        .request_execution(
            &auth,
            automation_service::RequestAutomationExecution {
                execution_id: "ae_oversized_agent_metadata".into(),
                trigger_type: "agent.manual".into(),
                target_kind: "conversation".into(),
                target_ref: "c_demo".into(),
                input_payload: Some(r#"{"prompt":"hello"}"#.into()),
            },
        )
        .expect("execution request should succeed");

    let error = runtime.start_agent_response(
        &auth,
        automation_service::StartAgentResponseRequest {
            execution_id: "ae_oversized_agent_metadata".into(),
            stream_id: "st_oversized_agent_metadata".into(),
            stream_type: "agent.response.delta".into(),
            conversation_id: "c_demo".into(),
            schema_ref: Some("schema://agent/response.delta".into()),
            member_id: Some("cm_agent".into()),
            agent: AgentSubject {
                agent_id: "agent.demo".into(),
                session_id: Some("s_agent".into()),
                metadata: BTreeMap::from([("trace".into(), "x".repeat(65_537))]),
            },
        },
    );
    let error = match error {
        Ok(session) => panic!("agent.metadata should be rejected: {session:?}"),
        Err(error) => error,
    };
    let response = axum::response::IntoResponse::into_response(error);
    assert_eq!(response.status(), axum::http::StatusCode::PAYLOAD_TOO_LARGE);
}

#[test]
fn test_complete_agent_response_rejects_oversized_result_message_id() {
    let runtime = automation_service::AutomationRuntime::default();
    let auth = AppContext {
        tenant_id: "100001".into(),
        organization_id: "0".to_owned(),
        user_id: "1".into(),
        app_id: None,
        environment: None,
        deployment_mode: None,
        auth_level: None,
        data_scope: Default::default(),
        actor_id: "1".into(),
        actor_kind: "user".into(),
        session_id: Some("s_user".into()),
        device_id: None,
        permission_scope: BTreeSet::from([
            "automation.execute".to_string(),
            "automation.read".to_string(),
        ]),
    };

    runtime
        .request_execution(
            &auth,
            automation_service::RequestAutomationExecution {
                execution_id: "ae_oversized_result_message_id".into(),
                trigger_type: "agent.manual".into(),
                target_kind: "conversation".into(),
                target_ref: "c_demo".into(),
                input_payload: Some(r#"{"prompt":"hello"}"#.into()),
            },
        )
        .expect("execution request should succeed");

    runtime
        .start_agent_response(
            &auth,
            automation_service::StartAgentResponseRequest {
                execution_id: "ae_oversized_result_message_id".into(),
                stream_id: "st_oversized_result_message_id".into(),
                stream_type: "agent.response.delta".into(),
                conversation_id: "c_demo".into(),
                schema_ref: Some("schema://agent/response.delta".into()),
                member_id: Some("cm_agent".into()),
                agent: AgentSubject {
                    agent_id: "agent.demo".into(),
                    session_id: Some("s_agent".into()),
                    metadata: BTreeMap::from([
                        ("agentMode".into(), "assistant".into()),
                        ("capabilityProfileId".into(), "stable-agent".into()),
                    ]),
                },
            },
        )
        .expect("agent response stream should start");

    let error = runtime
        .complete_agent_response(
            &auth,
            "st_oversized_result_message_id",
            automation_service::CompleteAgentResponseRequest {
                frame_seq: 1,
                result_message_id: Some("m".repeat(257)),
            },
        )
        .expect_err("oversized result message id should be rejected");
    let response = axum::response::IntoResponse::into_response(error);
    assert_eq!(response.status(), axum::http::StatusCode::PAYLOAD_TOO_LARGE);
}

#[test]
fn test_start_agent_response_rejects_oversized_agent_identity_fields() {
    for (field, agent_id, session_id) in [
        (
            "agent.agent_id",
            "a".repeat(257),
            Some("s_agent".to_string()),
        ),
        (
            "agent.session_id",
            "agent.demo".to_string(),
            Some("s".repeat(257)),
        ),
    ] {
        let runtime = automation_service::AutomationRuntime::default();
        let auth = AppContext {
            tenant_id: "100001".into(),
            organization_id: "0".to_owned(),
            user_id: "1".into(),
            app_id: None,
            environment: None,
            deployment_mode: None,
            auth_level: None,
            data_scope: Default::default(),
            actor_id: "1".into(),
            actor_kind: "user".into(),
            session_id: Some("s_user".into()),
            device_id: None,
            permission_scope: BTreeSet::from([
                "automation.execute".to_string(),
                "automation.read".to_string(),
            ]),
        };

        runtime
            .request_execution(
                &auth,
                automation_service::RequestAutomationExecution {
                    execution_id: format!("ae_{}", field.replace('.', "_")),
                    trigger_type: "agent.manual".into(),
                    target_kind: "conversation".into(),
                    target_ref: "c_demo".into(),
                    input_payload: Some(r#"{"prompt":"hello"}"#.into()),
                },
            )
            .expect("execution request should succeed");

        let error = runtime.start_agent_response(
            &auth,
            automation_service::StartAgentResponseRequest {
                execution_id: format!("ae_{}", field.replace('.', "_")),
                stream_id: format!("st_{}", field.replace('.', "_")),
                stream_type: "agent.response.delta".into(),
                conversation_id: "c_demo".into(),
                schema_ref: Some("schema://agent/response.delta".into()),
                member_id: Some("cm_agent".into()),
                agent: AgentSubject {
                    agent_id,
                    session_id,
                    metadata: BTreeMap::from([
                        ("agentMode".into(), "assistant".into()),
                        ("capabilityProfileId".into(), "stable-agent".into()),
                    ]),
                },
            },
        );
        let error = match error {
            Ok(session) => panic!("{field} should be rejected: {session:?}"),
            Err(error) => error,
        };
        let response = axum::response::IntoResponse::into_response(error);
        assert_eq!(
            response.status(),
            axum::http::StatusCode::PAYLOAD_TOO_LARGE,
            "{field} should be rejected with payload_too_large"
        );
    }
}

#[test]
fn test_request_agent_tool_call_rejects_oversized_tool_call_id() {
    let runtime = automation_service::AutomationRuntime::default();
    let auth = AppContext {
        tenant_id: "100001".into(),
        organization_id: "0".to_owned(),
        user_id: "1".into(),
        app_id: None,
        environment: None,
        deployment_mode: None,
        auth_level: None,
        data_scope: Default::default(),
        actor_id: "1".into(),
        actor_kind: "user".into(),
        session_id: Some("s_user".into()),
        device_id: None,
        permission_scope: BTreeSet::from([
            "automation.execute".to_string(),
            "automation.read".to_string(),
        ]),
    };

    runtime
        .request_execution(
            &auth,
            automation_service::RequestAutomationExecution {
                execution_id: "ae_oversized_tool_call_id".into(),
                trigger_type: "agent.manual".into(),
                target_kind: "conversation".into(),
                target_ref: "c_demo".into(),
                input_payload: Some(r#"{"prompt":"hello"}"#.into()),
            },
        )
        .expect("execution request should succeed");

    runtime
        .start_agent_response(
            &auth,
            automation_service::StartAgentResponseRequest {
                execution_id: "ae_oversized_tool_call_id".into(),
                stream_id: "st_oversized_tool_call_id".into(),
                stream_type: "agent.response.delta".into(),
                conversation_id: "c_demo".into(),
                schema_ref: Some("schema://agent/response.delta".into()),
                member_id: Some("cm_agent".into()),
                agent: AgentSubject {
                    agent_id: "agent.demo".into(),
                    session_id: Some("s_agent".into()),
                    metadata: BTreeMap::from([
                        ("agentMode".into(), "assistant".into()),
                        ("capabilityProfileId".into(), "stable-agent".into()),
                    ]),
                },
            },
        )
        .expect("agent response stream should start");

    let error = runtime
        .request_agent_tool_call(
            &auth,
            automation_service::RequestAgentToolCallRequest {
                execution_id: "ae_oversized_tool_call_id".into(),
                tool_call_id: "t".repeat(257),
                tool_name: "knowledge.search".into(),
                arguments_payload: r#"{"query":"hello"}"#.into(),
            },
        )
        .expect_err("oversized tool call id should be rejected");
    let response = axum::response::IntoResponse::into_response(error);
    assert_eq!(response.status(), axum::http::StatusCode::PAYLOAD_TOO_LARGE);
}

#[test]
fn test_request_agent_tool_call_rejects_oversized_execution_id() {
    let runtime = automation_service::AutomationRuntime::default();
    let auth = AppContext {
        tenant_id: "100001".into(),
        organization_id: "0".to_owned(),
        user_id: "1".into(),
        app_id: None,
        environment: None,
        deployment_mode: None,
        auth_level: None,
        data_scope: Default::default(),
        actor_id: "1".into(),
        actor_kind: "user".into(),
        session_id: Some("s_user".into()),
        device_id: None,
        permission_scope: BTreeSet::from([
            "automation.execute".to_string(),
            "automation.read".to_string(),
        ]),
    };

    let error = runtime
        .request_agent_tool_call(
            &auth,
            automation_service::RequestAgentToolCallRequest {
                execution_id: "e".repeat(257),
                tool_call_id: "tc_oversized_execution_id".into(),
                tool_name: "knowledge.search".into(),
                arguments_payload: r#"{"query":"hello"}"#.into(),
            },
        )
        .expect_err("oversized execution id should be rejected");
    let response = axum::response::IntoResponse::into_response(error);
    assert_eq!(response.status(), axum::http::StatusCode::PAYLOAD_TOO_LARGE);
}

#[test]
fn test_request_agent_tool_call_rejects_oversized_tool_name() {
    let runtime = automation_service::AutomationRuntime::default();
    let auth = AppContext {
        tenant_id: "100001".into(),
        organization_id: "0".to_owned(),
        user_id: "1".into(),
        app_id: None,
        environment: None,
        deployment_mode: None,
        auth_level: None,
        data_scope: Default::default(),
        actor_id: "1".into(),
        actor_kind: "user".into(),
        session_id: Some("s_user".into()),
        device_id: None,
        permission_scope: BTreeSet::from([
            "automation.execute".to_string(),
            "automation.read".to_string(),
        ]),
    };

    runtime
        .request_execution(
            &auth,
            automation_service::RequestAutomationExecution {
                execution_id: "ae_oversized_tool_name".into(),
                trigger_type: "agent.manual".into(),
                target_kind: "conversation".into(),
                target_ref: "c_demo".into(),
                input_payload: Some(r#"{"prompt":"hello"}"#.into()),
            },
        )
        .expect("execution request should succeed");

    runtime
        .start_agent_response(
            &auth,
            automation_service::StartAgentResponseRequest {
                execution_id: "ae_oversized_tool_name".into(),
                stream_id: "st_oversized_tool_name".into(),
                stream_type: "agent.response.delta".into(),
                conversation_id: "c_demo".into(),
                schema_ref: Some("schema://agent/response.delta".into()),
                member_id: Some("cm_agent".into()),
                agent: AgentSubject {
                    agent_id: "agent.demo".into(),
                    session_id: Some("s_agent".into()),
                    metadata: BTreeMap::from([
                        ("agentMode".into(), "assistant".into()),
                        ("capabilityProfileId".into(), "stable-agent".into()),
                    ]),
                },
            },
        )
        .expect("agent response stream should start");

    let error = runtime
        .request_agent_tool_call(
            &auth,
            automation_service::RequestAgentToolCallRequest {
                execution_id: "ae_oversized_tool_name".into(),
                tool_call_id: "tc_oversized_name".into(),
                tool_name: "t".repeat(257),
                arguments_payload: r#"{"query":"hello"}"#.into(),
            },
        )
        .expect_err("oversized tool name should be rejected");
    let response = axum::response::IntoResponse::into_response(error);
    assert_eq!(response.status(), axum::http::StatusCode::PAYLOAD_TOO_LARGE);
}
