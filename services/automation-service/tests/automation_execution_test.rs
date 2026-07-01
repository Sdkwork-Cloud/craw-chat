use std::collections::BTreeSet;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::sleep;
use std::time::Duration;

use im_app_context::AppContext;
use im_domain_events::{AggregateType, CommitEnvelope};
use im_platform_contracts::{CommitJournal, CommitPosition, ContractError};

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

#[derive(Clone)]
struct BlockingJournal {
    events: Arc<Mutex<Vec<CommitEnvelope>>>,
    should_block_first_requested: Arc<AtomicBool>,
    first_requested_started_tx: Arc<Mutex<Option<Sender<()>>>>,
    continue_rx: Arc<Mutex<Receiver<()>>>,
}

impl BlockingJournal {
    fn new(first_requested_started_tx: Sender<()>, continue_rx: Receiver<()>) -> Self {
        Self {
            events: Arc::new(Mutex::new(Vec::new())),
            should_block_first_requested: Arc::new(AtomicBool::new(true)),
            first_requested_started_tx: Arc::new(Mutex::new(Some(first_requested_started_tx))),
            continue_rx: Arc::new(Mutex::new(continue_rx)),
        }
    }
}

impl CommitJournal for BlockingJournal {
    fn append(&self, envelope: CommitEnvelope) -> Result<CommitPosition, ContractError> {
        if envelope.event_type == "automation.execution_requested"
            && self
                .should_block_first_requested
                .swap(false, Ordering::SeqCst)
        {
            if let Some(tx) = self
                .first_requested_started_tx
                .lock()
                .expect("blocking journal signal should lock")
                .take()
            {
                let _ = tx.send(());
            }
            self.continue_rx
                .lock()
                .expect("blocking journal gate should lock")
                .recv()
                .expect("blocking journal gate should receive release signal");
        }

        let mut events = self.events.lock().expect("blocking journal should lock");
        events.push(envelope);
        Ok(CommitPosition::new("p0", events.len() as u64))
    }
}

#[test]
fn test_request_execution_appends_requested_and_completed_events() {
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
        session_id: Some("s_demo".into()),
        device_id: None,
        permission_scope: BTreeSet::from(["automation.execute".to_string()]),
    };

    let execution = runtime
        .request_execution(
            &auth,
            automation_service::RequestAutomationExecution {
                execution_id: "ae_demo".into(),
                trigger_type: "webhook.manual".into(),
                target_kind: "workflow".into(),
                target_ref: "wf_demo".into(),
                input_payload: Some(r#"{"conversationId":"c_demo"}"#.into()),
            },
        )
        .expect("execution request should succeed");

    assert_eq!(execution.execution_id, "ae_demo");
    assert_eq!(execution.state.as_str(), "succeeded");
    assert_eq!(execution.retry_count, 0);

    let events = journal.recorded();
    assert_eq!(events.len(), 2);
    assert_eq!(events[0].event_type, "automation.execution_requested");
    assert_eq!(events[1].event_type, "automation.execution_completed");
    assert_eq!(events[0].aggregate_type, AggregateType::AutomationExecution);
    assert_eq!(events[0].actor.actor_id, "1");
    assert_eq!(
        events[0].idempotency_key.as_deref(),
        Some("6#1000014#user1#17#ae_demo30#automation.execution_requested")
    );
    assert_eq!(
        events[1].idempotency_key.as_deref(),
        Some("6#1000014#user1#17#ae_demo30#automation.execution_completed")
    );

    let payload: serde_json::Value =
        serde_json::from_str(&events[1].payload).expect("payload should be valid json");
    assert_eq!(payload["executionId"], "ae_demo");
    assert_eq!(payload["targetKind"], "workflow");
    assert_eq!(payload["state"], "succeeded");
}

#[test]
fn test_get_execution_is_scoped_to_requesting_principal() {
    let runtime = automation_service::AutomationRuntime::default();
    let owner_auth = AppContext {
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
        session_id: Some("s_owner".into()),
        device_id: None,
        permission_scope: BTreeSet::from([
            "automation.execute".to_string(),
            "automation.read".to_string(),
        ]),
    };
    let other_auth = AppContext {
        tenant_id: "100001".into(),
        organization_id: "0".to_owned(),
        user_id: "1".into(),
        app_id: None,
        environment: None,
        deployment_mode: None,
        auth_level: None,
        data_scope: Default::default(),
        actor_id: "1110".into(),
        actor_kind: "user".into(),
        session_id: Some("s_other".into()),
        device_id: None,
        permission_scope: BTreeSet::from(["automation.read".to_string()]),
    };

    runtime
        .request_execution(
            &owner_auth,
            automation_service::RequestAutomationExecution {
                execution_id: "ae_principal_scoped".into(),
                trigger_type: "webhook.manual".into(),
                target_kind: "workflow".into(),
                target_ref: "wf_demo".into(),
                input_payload: Some(r#"{"conversationId":"c_demo"}"#.into()),
            },
        )
        .expect("execution request should succeed");

    let error = runtime
        .get_execution(&other_auth, "ae_principal_scoped")
        .expect_err("foreign principal should not read execution");
    let response = axum::response::IntoResponse::into_response(error);
    assert_eq!(response.status(), axum::http::StatusCode::NOT_FOUND);
}

#[test]
fn test_duplicate_request_execution_is_idempotent_when_payload_matches() {
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
        session_id: Some("s_demo".into()),
        device_id: None,
        permission_scope: BTreeSet::from([
            "automation.execute".to_string(),
            "automation.read".to_string(),
        ]),
    };

    let first = runtime
        .request_execution(
            &auth,
            automation_service::RequestAutomationExecution {
                execution_id: "ae_idempotent".into(),
                trigger_type: "webhook.manual".into(),
                target_kind: "workflow".into(),
                target_ref: "wf_demo".into(),
                input_payload: Some(r#"{"conversationId":"c_demo"}"#.into()),
            },
        )
        .expect("first execution request should succeed");
    let second = runtime
        .request_execution(
            &auth,
            automation_service::RequestAutomationExecution {
                execution_id: "ae_idempotent".into(),
                trigger_type: "webhook.manual".into(),
                target_kind: "workflow".into(),
                target_ref: "wf_demo".into(),
                input_payload: Some(r#"{"conversationId":"c_demo"}"#.into()),
            },
        )
        .expect("duplicate execution request should be idempotent");

    assert_eq!(second, first);

    let events = journal.recorded();
    assert_eq!(events.len(), 2);
}

#[test]
fn test_duplicate_request_execution_rejects_conflicting_payload() {
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
        session_id: Some("s_demo".into()),
        device_id: None,
        permission_scope: BTreeSet::from(["automation.execute".to_string()]),
    };

    runtime
        .request_execution(
            &auth,
            automation_service::RequestAutomationExecution {
                execution_id: "ae_conflict".into(),
                trigger_type: "webhook.manual".into(),
                target_kind: "workflow".into(),
                target_ref: "wf_demo".into(),
                input_payload: Some(r#"{"conversationId":"c_demo"}"#.into()),
            },
        )
        .expect("first execution request should succeed");

    let error = runtime
        .request_execution(
            &auth,
            automation_service::RequestAutomationExecution {
                execution_id: "ae_conflict".into(),
                trigger_type: "webhook.manual".into(),
                target_kind: "workflow".into(),
                target_ref: "wf_other".into(),
                input_payload: Some(r#"{"conversationId":"c_demo"}"#.into()),
            },
        )
        .expect_err("conflicting duplicate should be rejected");
    let response = axum::response::IntoResponse::into_response(error);
    assert_eq!(response.status(), axum::http::StatusCode::CONFLICT);

    let events = journal.recorded();
    assert_eq!(events.len(), 2);
}

#[test]
fn test_execution_timestamps_advance_between_distinct_requests() {
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
        session_id: Some("s_demo".into()),
        device_id: None,
        permission_scope: BTreeSet::from(["automation.execute".to_string()]),
    };

    let first = runtime
        .request_execution(
            &auth,
            automation_service::RequestAutomationExecution {
                execution_id: "ae_time_first".into(),
                trigger_type: "webhook.manual".into(),
                target_kind: "workflow".into(),
                target_ref: "wf_first".into(),
                input_payload: Some(r#"{"step":"first"}"#.into()),
            },
        )
        .expect("first execution should succeed");

    sleep(Duration::from_millis(5));

    let second = runtime
        .request_execution(
            &auth,
            automation_service::RequestAutomationExecution {
                execution_id: "ae_time_second".into(),
                trigger_type: "webhook.manual".into(),
                target_kind: "workflow".into(),
                target_ref: "wf_second".into(),
                input_payload: Some(r#"{"step":"second"}"#.into()),
            },
        )
        .expect("second execution should succeed");

    assert_ne!(
        first.requested_at, second.requested_at,
        "distinct execution requests must not reuse a fixed requested_at timestamp"
    );
    assert_ne!(
        first.completed_at, second.completed_at,
        "distinct execution requests must not reuse a fixed completed_at timestamp"
    );
}

#[test]
fn test_request_execution_with_outcome_exposes_applied_then_replayed_delivery_status() {
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
        session_id: Some("s_demo".into()),
        device_id: None,
        permission_scope: BTreeSet::from([
            "automation.execute".to_string(),
            "automation.read".to_string(),
        ]),
    };

    let first = runtime
        .request_execution_with_outcome(
            &auth,
            automation_service::RequestAutomationExecution {
                execution_id: "ae_outcome_state_machine".into(),
                trigger_type: "webhook.manual".into(),
                target_kind: "workflow".into(),
                target_ref: "wf_outcome".into(),
                input_payload: Some(r#"{"conversationId":"c_demo"}"#.into()),
            },
        )
        .expect("first execution request should succeed");
    assert!(first.is_new);
    assert_eq!(first.delivery_status.as_str(), "applied");
    assert_eq!(first.execution.state.as_str(), "succeeded");
    assert_eq!(
        first.request_key,
        "6#1000014#user1#124#ae_outcome_state_machine"
    );

    let replay = runtime
        .request_execution_with_outcome(
            &auth,
            automation_service::RequestAutomationExecution {
                execution_id: "ae_outcome_state_machine".into(),
                trigger_type: "webhook.manual".into(),
                target_kind: "workflow".into(),
                target_ref: "wf_outcome".into(),
                input_payload: Some(r#"{"conversationId":"c_demo"}"#.into()),
            },
        )
        .expect("duplicate execution request should return replay outcome");
    assert!(!replay.is_new);
    assert_eq!(replay.delivery_status.as_str(), "replayed");
    assert_eq!(replay.request_key, first.request_key);
    assert_eq!(replay.execution, first.execution);
}

#[test]
fn test_request_execution_with_outcome_surfaces_accepted_during_inflight_apply() {
    let (requested_started_tx, requested_started_rx) = mpsc::channel();
    let (continue_tx, continue_rx) = mpsc::channel();
    let journal = Arc::new(BlockingJournal::new(requested_started_tx, continue_rx));
    let runtime = Arc::new(automation_service::AutomationRuntime::with_journal(journal));
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
        session_id: Some("s_demo".into()),
        device_id: None,
        permission_scope: BTreeSet::from([
            "automation.execute".to_string(),
            "automation.read".to_string(),
        ]),
    };
    let request = automation_service::RequestAutomationExecution {
        execution_id: "ae_outcome_accepted".into(),
        trigger_type: "webhook.manual".into(),
        target_kind: "workflow".into(),
        target_ref: "wf_outcome".into(),
        input_payload: Some(r#"{"conversationId":"c_demo"}"#.into()),
    };

    let runtime_for_first = runtime.clone();
    let auth_for_first = auth.clone();
    let request_for_first = request.clone();
    let first_handle = thread::spawn(move || {
        runtime_for_first
            .request_execution_with_outcome(&auth_for_first, request_for_first)
            .expect("first execution request should eventually apply")
    });

    requested_started_rx
        .recv_timeout(Duration::from_secs(1))
        .expect("blocking journal should report first request reached requested append");

    let inflight = runtime
        .request_execution_with_outcome(&auth, request.clone())
        .expect("inflight duplicate should return accepted outcome");
    assert!(!inflight.is_new);
    assert_eq!(inflight.delivery_status.as_str(), "accepted");
    assert_eq!(inflight.execution.state.as_str(), "requested");

    continue_tx
        .send(())
        .expect("blocking journal should release first request");
    let first = first_handle
        .join()
        .expect("first execution request thread should join");
    assert!(first.is_new);
    assert_eq!(first.delivery_status.as_str(), "applied");
    assert_eq!(first.execution.state.as_str(), "succeeded");

    let replay = runtime
        .request_execution_with_outcome(&auth, request)
        .expect("post-apply duplicate should return replayed outcome");
    assert!(!replay.is_new);
    assert_eq!(replay.delivery_status.as_str(), "replayed");
}

#[test]
fn test_request_execution_rejects_oversized_input_payload() {
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
        session_id: Some("s_demo".into()),
        device_id: None,
        permission_scope: BTreeSet::from(["automation.execute".to_string()]),
    };
    let oversized_input = "x".repeat(131073);

    let error = runtime
        .request_execution(
            &auth,
            automation_service::RequestAutomationExecution {
                execution_id: "ae_oversized_input".into(),
                trigger_type: "webhook.manual".into(),
                target_kind: "workflow".into(),
                target_ref: "wf_demo".into(),
                input_payload: Some(oversized_input),
            },
        )
        .expect_err("oversized input payload should be rejected");
    let response = axum::response::IntoResponse::into_response(error);
    assert_eq!(response.status(), axum::http::StatusCode::PAYLOAD_TOO_LARGE);
}

#[test]
fn test_request_execution_rejects_oversized_execution_id() {
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
        session_id: Some("s_demo".into()),
        device_id: None,
        permission_scope: BTreeSet::from(["automation.execute".to_string()]),
    };
    let oversized_execution_id = "e".repeat(257);

    let error = runtime
        .request_execution(
            &auth,
            automation_service::RequestAutomationExecution {
                execution_id: oversized_execution_id,
                trigger_type: "webhook.manual".into(),
                target_kind: "workflow".into(),
                target_ref: "wf_demo".into(),
                input_payload: Some(r#"{"conversationId":"c_demo"}"#.into()),
            },
        )
        .expect_err("oversized execution id should be rejected");
    let response = axum::response::IntoResponse::into_response(error);
    assert_eq!(response.status(), axum::http::StatusCode::PAYLOAD_TOO_LARGE);
}

#[test]
fn test_get_execution_rejects_oversized_execution_id() {
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
        session_id: Some("s_demo".into()),
        device_id: None,
        permission_scope: BTreeSet::from(["automation.read".to_string()]),
    };
    let oversized_execution_id = "e".repeat(257);

    let error = runtime
        .get_execution(&auth, oversized_execution_id.as_str())
        .expect_err("oversized execution id should be rejected");
    let response = axum::response::IntoResponse::into_response(error);
    assert_eq!(response.status(), axum::http::StatusCode::PAYLOAD_TOO_LARGE);
}

#[test]
fn test_execution_requests_are_isolated_by_principal_kind_for_same_actor_id() {
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

    let user_request = runtime
        .request_execution_with_outcome(
            &user_auth,
            automation_service::RequestAutomationExecution {
                execution_id: "ae_kind_isolation".into(),
                trigger_type: "webhook.manual".into(),
                target_kind: "workflow".into(),
                target_ref: "wf_demo".into(),
                input_payload: Some(r#"{"conversationId":"c_demo"}"#.into()),
            },
        )
        .expect("user execution request should succeed");

    let system_request = runtime
        .request_execution_with_outcome(
            &system_auth,
            automation_service::RequestAutomationExecution {
                execution_id: "ae_kind_isolation".into(),
                trigger_type: "webhook.manual".into(),
                target_kind: "workflow".into(),
                target_ref: "wf_demo".into(),
                input_payload: Some(r#"{"conversationId":"c_demo"}"#.into()),
            },
        )
        .expect("system execution request should succeed");

    assert_eq!(
        user_request.request_key,
        "6#1000014#user1#117#ae_kind_isolation"
    );
    assert_eq!(
        system_request.request_key,
        "6#1000016#system1#117#ae_kind_isolation"
    );
    assert_eq!(user_request.execution.principal_kind, "user");
    assert_eq!(system_request.execution.principal_kind, "system");

    let user_execution = runtime
        .get_execution(&user_auth, "ae_kind_isolation")
        .expect("user execution should be readable");
    let system_execution = runtime
        .get_execution(&system_auth, "ae_kind_isolation")
        .expect("system execution should be readable");
    assert_eq!(user_execution.principal_kind, "user");
    assert_eq!(system_execution.principal_kind, "system");
    assert_eq!(user_execution.execution_id, "ae_kind_isolation");
    assert_eq!(system_execution.execution_id, "ae_kind_isolation");
}

#[test]
fn test_execution_scope_key_is_segment_safe_for_delimiter_bearing_ids() {
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

    let first = runtime
        .request_execution_with_outcome(
            &first_auth,
            automation_service::RequestAutomationExecution {
                execution_id: "demo".into(),
                trigger_type: "webhook.manual".into(),
                target_kind: "workflow".into(),
                target_ref: "wf_first".into(),
                input_payload: Some(r#"{"case":"first"}"#.into()),
            },
        )
        .expect("first delimiter-bearing execution should succeed");
    let second = runtime
        .request_execution_with_outcome(
            &second_auth,
            automation_service::RequestAutomationExecution {
                execution_id: "demo:demo".into(),
                trigger_type: "webhook.manual".into(),
                target_kind: "workflow".into(),
                target_ref: "wf_second".into(),
                input_payload: Some(r#"{"case":"second"}"#.into()),
            },
        )
        .expect("second delimiter-bearing execution should not collide with first");

    assert_eq!(first.execution.principal_id, "u:demo");
    assert_eq!(first.execution.execution_id, "demo");
    assert_eq!(second.execution.principal_id, "u");
    assert_eq!(second.execution.execution_id, "demo:demo");
    assert_eq!(first.request_key, "6#1000014#user6#u:demo4#demo");
    assert_eq!(second.request_key, "6#1000014#user1#u9#demo:demo");
    assert_ne!(first.request_key, second.request_key);

    let first_read = runtime
        .get_execution(&first_auth, "demo")
        .expect("first execution should remain readable by first actor");
    let second_read = runtime
        .get_execution(&second_auth, "demo:demo")
        .expect("second execution should remain readable by second actor");
    assert_eq!(first_read.principal_id, "u:demo");
    assert_eq!(second_read.principal_id, "u");
}

#[test]
fn test_execution_journal_metadata_is_isolated_by_principal_kind_for_same_actor_id() {
    let journal = Arc::new(RecordingJournal::default());
    let runtime = automation_service::AutomationRuntime::with_journal(journal.clone());
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
                execution_id: "ae_kind_audit".into(),
                trigger_type: "webhook.manual".into(),
                target_kind: "workflow".into(),
                target_ref: "wf_demo".into(),
                input_payload: Some(r#"{"conversationId":"c_demo"}"#.into()),
            },
        )
        .expect("user execution request should succeed");
    runtime
        .request_execution(
            &system_auth,
            automation_service::RequestAutomationExecution {
                execution_id: "ae_kind_audit".into(),
                trigger_type: "webhook.manual".into(),
                target_kind: "workflow".into(),
                target_ref: "wf_demo".into(),
                input_payload: Some(r#"{"conversationId":"c_demo"}"#.into()),
            },
        )
        .expect("system execution request should succeed");

    let events = journal.recorded();
    assert_eq!(events.len(), 4);

    let user_requested = &events[0];
    let user_completed = &events[1];
    let system_requested = &events[2];
    let system_completed = &events[3];

    assert_ne!(user_requested.event_id, system_requested.event_id);
    assert_ne!(user_completed.event_id, system_completed.event_id);
    assert_ne!(user_requested.aggregate_id, system_requested.aggregate_id);
    assert_ne!(user_requested.scope_id, system_requested.scope_id);
    assert_ne!(user_requested.ordering_key, system_requested.ordering_key);
    assert_ne!(
        user_requested.correlation_id,
        system_requested.correlation_id
    );
    assert_ne!(
        user_requested.idempotency_key,
        system_requested.idempotency_key
    );
    assert_ne!(
        user_completed.idempotency_key,
        system_completed.idempotency_key
    );
}
