use std::collections::BTreeSet;
use std::sync::{Arc, Mutex};

use im_adapters_local_memory::MemoryAutomationExecutionStore;
use im_app_context::AppContext;
use im_domain_events::CommitEnvelope;
use im_platform_contracts::{CommitJournal, CommitPosition, ContractError};

#[derive(Clone, Default)]
struct RecordingJournal {
    events: Arc<Mutex<Vec<CommitEnvelope>>>,
}

impl CommitJournal for RecordingJournal {
    fn append(&self, envelope: CommitEnvelope) -> Result<CommitPosition, ContractError> {
        let mut events = self.events.lock().expect("journal should lock");
        events.push(envelope);
        Ok(CommitPosition::new("p0", events.len() as u64))
    }
}

#[test]
fn test_runtime_restores_automation_projection_on_rebuild_with_shared_store() {
    let journal = Arc::new(RecordingJournal::default());
    let execution_store = Arc::new(MemoryAutomationExecutionStore::default());
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

    let runtime_before = automation_service::AutomationRuntime::with_journal_and_store(
        journal.clone(),
        execution_store.clone(),
    );

    runtime_before
        .request_execution(
            &auth,
            automation_service::RequestAutomationExecution {
                execution_id: "ae_rebuild".into(),
                trigger_type: "webhook.manual".into(),
                target_kind: "workflow".into(),
                target_ref: "wf_rebuild".into(),
                input_payload: Some(r#"{"conversationId":"c_demo"}"#.into()),
            },
        )
        .expect("execution request should succeed");

    let runtime_after =
        automation_service::AutomationRuntime::with_journal_and_store(journal, execution_store);

    let execution = runtime_after
        .get_execution(&auth, "ae_rebuild")
        .expect("execution should restore after rebuild");
    assert_eq!(execution.execution_id, "ae_rebuild");
    assert_eq!(execution.state.as_str(), "succeeded");
}

#[test]
fn test_runtime_restores_principal_kind_isolated_executions_on_rebuild_with_shared_store() {
    let journal = Arc::new(RecordingJournal::default());
    let execution_store = Arc::new(MemoryAutomationExecutionStore::default());
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

    let runtime_before = automation_service::AutomationRuntime::with_journal_and_store(
        journal.clone(),
        execution_store.clone(),
    );

    runtime_before
        .request_execution(
            &user_auth,
            automation_service::RequestAutomationExecution {
                execution_id: "ae_rebuild_kind_isolation".into(),
                trigger_type: "webhook.manual".into(),
                target_kind: "workflow".into(),
                target_ref: "wf_rebuild".into(),
                input_payload: Some(r#"{"conversationId":"c_demo"}"#.into()),
            },
        )
        .expect("user execution request should succeed");
    runtime_before
        .request_execution(
            &system_auth,
            automation_service::RequestAutomationExecution {
                execution_id: "ae_rebuild_kind_isolation".into(),
                trigger_type: "webhook.manual".into(),
                target_kind: "workflow".into(),
                target_ref: "wf_rebuild".into(),
                input_payload: Some(r#"{"conversationId":"c_demo"}"#.into()),
            },
        )
        .expect("system execution request should succeed");

    let runtime_after =
        automation_service::AutomationRuntime::with_journal_and_store(journal, execution_store);

    let user_execution = runtime_after
        .get_execution(&user_auth, "ae_rebuild_kind_isolation")
        .expect("user execution should restore after rebuild");
    let system_execution = runtime_after
        .get_execution(&system_auth, "ae_rebuild_kind_isolation")
        .expect("system execution should restore after rebuild");
    assert_eq!(user_execution.principal_kind, "user");
    assert_eq!(system_execution.principal_kind, "system");
}
