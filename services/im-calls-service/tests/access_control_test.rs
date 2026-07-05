//! TECH-53 regression tests for conversation-bound RTC member authorization.

use std::sync::Arc;

use calls_service::CallingRuntime;
use im_app_context::local_service_app_context;
use im_platform_contracts::{
    AggregateStoreConversationMemberAccessGate, ConversationAggregateState,
    ConversationAggregateStore, ConversationMemberAccessGate, ConversationMemberRecord,
    ReadCursorRecord,
};
use sdkwork_im_contract_core::ContractError;

struct DenyAllMembersStore;

impl ConversationAggregateStore for DenyAllMembersStore {
    fn load_members(
        &self,
        _: &str,
        _: &str,
        _: &str,
    ) -> Result<Vec<ConversationMemberRecord>, ContractError> {
        Ok(Vec::new())
    }

    fn load_member(
        &self,
        _: &str,
        _: &str,
        _: &str,
        _: &str,
        _: &str,
    ) -> Result<Option<ConversationMemberRecord>, ContractError> {
        Ok(None)
    }

    fn upsert_member(&self, _: ConversationMemberRecord) -> Result<(), ContractError> {
        Err(ContractError::UnsupportedCapability(
            "not implemented in test store".into(),
        ))
    }

    fn remove_member(
        &self,
        _: &str,
        _: &str,
        _: &str,
        _: &str,
        _: &str,
        _: &str,
    ) -> Result<(), ContractError> {
        Err(ContractError::UnsupportedCapability(
            "not implemented in test store".into(),
        ))
    }

    fn load_read_cursors(
        &self,
        _: &str,
        _: &str,
        _: &str,
    ) -> Result<Vec<ReadCursorRecord>, ContractError> {
        Ok(Vec::new())
    }

    fn load_read_cursor(
        &self,
        _: &str,
        _: &str,
        _: &str,
        _: i64,
    ) -> Result<Option<ReadCursorRecord>, ContractError> {
        Ok(None)
    }

    fn upsert_read_cursor(&self, _: ReadCursorRecord) -> Result<(), ContractError> {
        Err(ContractError::UnsupportedCapability(
            "not implemented in test store".into(),
        ))
    }

    fn load_aggregate_state(
        &self,
        _: &str,
        _: &str,
        _: &str,
    ) -> Result<ConversationAggregateState, ContractError> {
        Err(ContractError::UnsupportedCapability(
            "not implemented in test store".into(),
        ))
    }

    fn allocate_member_id(&self) -> Result<i64, ContractError> {
        Ok(1)
    }

    fn conversation_exists(&self, _: &str, _: &str, _: &str) -> Result<bool, ContractError> {
        Ok(false)
    }
}

fn runtime_with_deny_gate() -> CallingRuntime {
    let gate: Arc<dyn ConversationMemberAccessGate> = Arc::new(
        AggregateStoreConversationMemberAccessGate::new(Arc::new(DenyAllMembersStore)),
    );
    CallingRuntime::default().with_conversation_member_gate(Some(gate))
}

#[test]
fn non_member_cannot_create_conversation_bound_rtc_session() {
    let runtime = runtime_with_deny_gate();
    let auth = local_service_app_context("100001", "user-a", "user", None, Vec::<&str>::new());
    let request = calls_service::dto::CreateRtcSessionRequest {
        rtc_session_id: "rtc-session-1".into(),
        conversation_id: Some("conv-1".into()),
        rtc_mode: "voice".into(),
    };

    let error = runtime
        .create_session_with_outcome(&auth, request)
        .expect_err("non-member must be rejected before RTC state write");

    assert_eq!(error.code(), "conversation_permission_denied");
}

#[test]
fn pure_rtc_session_mutations_do_not_require_conversation_membership() {
    let runtime = runtime_with_deny_gate();
    let auth = local_service_app_context("100001", "user-a", "user", None, Vec::<&str>::new());

    runtime
        .create_session_with_outcome(
            &auth,
            calls_service::dto::CreateRtcSessionRequest {
                rtc_session_id: "rtc-session-2".into(),
                conversation_id: None,
                rtc_mode: "voice".into(),
            },
        )
        .expect("pure RTC session should be creatable without conversation gate");

    let outcome = runtime
        .invite_session_with_outcome(
            &auth,
            "rtc-session-2",
            calls_service::dto::InviteRtcSessionRequest {
                participant_ids: vec!["user-b".into()],
                signaling_stream_id: Some("stream-1".into()),
            },
        )
        .expect("pure RTC invite must not require conversation membership");

    assert!(outcome.applied);
}

struct AllowMembersStore {
    members: Vec<ConversationMemberRecord>,
}

impl ConversationAggregateStore for AllowMembersStore {
    fn load_members(
        &self,
        _: &str,
        _: &str,
        _: &str,
    ) -> Result<Vec<ConversationMemberRecord>, ContractError> {
        Ok(self.members.clone())
    }

    fn load_member(
        &self,
        _: &str,
        _: &str,
        _: &str,
        principal_kind: &str,
        principal_id: &str,
    ) -> Result<Option<ConversationMemberRecord>, ContractError> {
        Ok(self
            .members
            .iter()
            .find(|member| member.principal_kind == principal_kind && member.principal_id == principal_id)
            .cloned())
    }

    fn upsert_member(&self, _: ConversationMemberRecord) -> Result<(), ContractError> {
        Err(ContractError::UnsupportedCapability("test".into()))
    }

    fn remove_member(
        &self,
        _: &str,
        _: &str,
        _: &str,
        _: &str,
        _: &str,
        _: &str,
    ) -> Result<(), ContractError> {
        Err(ContractError::UnsupportedCapability("test".into()))
    }

    fn load_read_cursors(
        &self,
        _: &str,
        _: &str,
        _: &str,
    ) -> Result<Vec<ReadCursorRecord>, ContractError> {
        Ok(Vec::new())
    }

    fn load_read_cursor(
        &self,
        _: &str,
        _: &str,
        _: &str,
        _: i64,
    ) -> Result<Option<ReadCursorRecord>, ContractError> {
        Ok(None)
    }

    fn upsert_read_cursor(&self, _: ReadCursorRecord) -> Result<(), ContractError> {
        Err(ContractError::UnsupportedCapability("test".into()))
    }

    fn load_aggregate_state(
        &self,
        _: &str,
        _: &str,
        _: &str,
    ) -> Result<ConversationAggregateState, ContractError> {
        Err(ContractError::UnsupportedCapability("test".into()))
    }

    fn allocate_member_id(&self) -> Result<i64, ContractError> {
        Ok(1)
    }

    fn conversation_exists(&self, _: &str, _: &str, _: &str) -> Result<bool, ContractError> {
        Ok(true)
    }
}

fn runtime_with_roster(members: Vec<ConversationMemberRecord>) -> CallingRuntime {
    let store = Arc::new(AllowMembersStore { members });
    let gate: Arc<dyn ConversationMemberAccessGate> = Arc::new(
        AggregateStoreConversationMemberAccessGate::new(store.clone()),
    );
    CallingRuntime::default()
        .with_conversation_member_gate(Some(gate))
        .with_conversation_aggregate_store(Some(store))
}

#[test]
fn invite_rejects_participant_outside_conversation_roster() {
    let runtime = runtime_with_roster(vec![ConversationMemberRecord {
        tenant_id: "100001".into(),
        organization_id: "org-1".into(),
        conversation_id: "conv-1".into(),
        principal_kind: "user".into(),
        principal_id: "user-a".into(),
        member_id: 1,
        membership_role: "owner".into(),
        membership_state: "joined".into(),
        invited_by: None,
        joined_at: "2026-01-01T00:00:00Z".into(),
        removed_at: None,
        attributes_json: "{}".into(),
    }]);
    let auth = local_service_app_context("100001", "user-a", "user", None, Vec::<&str>::new());

    runtime
        .create_session_with_outcome(
            &auth,
            calls_service::dto::CreateRtcSessionRequest {
                rtc_session_id: "rtc-session-3".into(),
                conversation_id: Some("conv-1".into()),
                rtc_mode: "voice".into(),
            },
        )
        .expect("create conversation-bound rtc session");

    let error = runtime
        .invite_session_with_outcome(
            &auth,
            "rtc-session-3",
            calls_service::dto::InviteRtcSessionRequest {
                participant_ids: vec!["user-outsider".into()],
                signaling_stream_id: Some("stream-1".into()),
            },
        )
        .expect_err("outsider must be rejected");

    assert_eq!(error.code(), "participant_not_in_conversation_roster");
}
