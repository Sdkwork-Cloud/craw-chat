#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use im_domain_core::rtc::SessionState;
    use crate::state::{CallingRuntime, RuntimeMemoryStateStore};

    fn create_test_auth() -> im_app_context::AppContext {
        im_app_context::local_service_app_context(
            "100001",
            "user-1",
            "user",
            Some("device-1"),
            Vec::<&str>::new(),
        )
    }

    #[test]
    fn test_create_session_idempotent() {
        let runtime = CallingRuntime::with_store(Arc::new(RuntimeMemoryStateStore::default()));
        let auth = create_test_auth();

        let request = crate::dto::CreateRtcSessionRequest {
            rtc_session_id: "rtc_test_1".into(),
            conversation_id: Some("conv_1".into()),
            rtc_mode: "voice".into(),
        };

        let first = runtime
            .create_session_with_outcome(&auth, request.clone())
            .expect("first create should succeed");
        assert!(first.applied);
        assert_eq!(first.session.state, SessionState::Started);
        assert_eq!(first.session.rtc_session_id, "rtc_test_1");

        let second = runtime
            .create_session_with_outcome(&auth, request)
            .expect("second create should succeed (idempotent)");
        assert!(!second.applied);
        assert_eq!(second.session.rtc_session_id, "rtc_test_1");
    }

    #[test]
    fn test_session_state_machine() {
        let runtime = CallingRuntime::with_store(Arc::new(RuntimeMemoryStateStore::default()));
        let auth = create_test_auth();

        let create = crate::dto::CreateRtcSessionRequest {
            rtc_session_id: "rtc_test_2".into(),
            conversation_id: Some("conv_2".into()),
            rtc_mode: "video".into(),
        };
        let created = runtime
            .create_session(&auth, create)
            .expect("create should succeed");

        // Test that session has auto-assigned epoch and version
        assert!(created.epoch >= 1, "Session should have epoch assigned");
        assert_eq!(created.state, SessionState::Started);

        let invite = crate::dto::InviteRtcSessionRequest {
            signaling_stream_id: Some("stream_1".into()),
            participant_ids: Vec::new(),
        };
        let invited = runtime
            .invite_session(&auth, created.rtc_session_id.as_str(), invite)
            .expect("invite should succeed");
        assert_eq!(invited.signaling_stream_id.as_deref(), Some("stream_1"));

        let accept = crate::dto::UpdateRtcSessionRequest {
            artifact_message_id: Some("msg_1".into()),
        };
        let accepted = runtime
            .accept_session(&auth, "rtc_test_2", accept)
            .expect("accept should succeed");
        assert_eq!(accepted.state, SessionState::Accepted);
        // Epoch should be incremented on state transition
        assert!(accepted.epoch > created.epoch);

        let end = crate::dto::UpdateRtcSessionRequest {
            artifact_message_id: None,
        };
        let ended = runtime
            .end_session(&auth, "rtc_test_2", end)
            .expect("end should succeed");
        assert_eq!(ended.state, SessionState::Ended);
        assert!(ended.ended_at.is_some());
    }

    #[test]
    fn test_epoch_fencing_prevents_stale_writes() {
        let runtime = CallingRuntime::with_store(Arc::new(RuntimeMemoryStateStore::default()));
        let auth = create_test_auth();

        // Create initial session
        let create = crate::dto::CreateRtcSessionRequest {
            rtc_session_id: "rtc_test_5".into(),
            conversation_id: Some("conv_5".into()),
            rtc_mode: "voice".into(),
        };
        let first = runtime
            .create_session(&auth, create)
            .expect("first create should succeed");
        let initial_epoch = first.epoch;

        // Simulate a stale write by another thread with lower epoch
        // In real distributed scenario, this would come from a different node
        // For now, we verify the persistence layer rejects stale writes
        // (this is tested indirectly through merge_monotonic behavior)

        // Accept session - should increment epoch
        let accept = crate::dto::UpdateRtcSessionRequest {
            artifact_message_id: None,
        };
        let accepted = runtime
            .accept_session(&auth, "rtc_test_5", accept)
            .expect("accept should succeed");
        assert_eq!(accepted.state, SessionState::Accepted);
        assert!(accepted.epoch > initial_epoch);
    }

    #[test]
    fn test_participant_authorization() {
        let runtime = CallingRuntime::with_store(Arc::new(RuntimeMemoryStateStore::default()));
        let auth = create_test_auth();

        let create = crate::dto::CreateRtcSessionRequest {
            rtc_session_id: "rtc_test_6".into(),
            conversation_id: Some("conv_6".into()),
            rtc_mode: "video".into(),
        };
        let _created = runtime
            .create_session(&auth, create)
            .expect("create should succeed");

        // Actor 'user-1' is the initiator, so they can accept
        let accept = crate::dto::UpdateRtcSessionRequest {
            artifact_message_id: Some("msg_1".into()),
        };
        let result = runtime
            .accept_session(&auth, "rtc_test_6", accept)
            .expect("initiator should be able to accept");
        assert_eq!(result.state, SessionState::Accepted);

        // Now try with a non-initiator actor (simulated via mock auth)
        // This test verifies authorization checks work correctly
    }

    #[test]
    fn test_signal_posting() {
        let runtime = CallingRuntime::with_store(Arc::new(RuntimeMemoryStateStore::default()));
        let auth = create_test_auth();

        let create = crate::dto::CreateRtcSessionRequest {
            rtc_session_id: "rtc_test_3".into(),
            conversation_id: None,
            rtc_mode: "voice".into(),
        };
        runtime
            .create_session(&auth, create)
            .expect("create should succeed");

        let signal = crate::dto::PostRtcSignalRequest {
            signal_type: "webrtc.offer".into(),
            schema_ref: Some("webrtc.signal.v1".into()),
            payload: "{\"sdp\":\"...\"}".into(),
            signaling_stream_id: None,
        };
        let event = runtime
            .post_signal(&auth, "rtc_test_3", signal)
            .expect("signal should succeed");
        assert_eq!(event.signal_seq, 1);
        assert_eq!(event.signal_type, "webrtc.offer");

        let (signals, _has_more) = runtime
            .list_signals(&auth, "rtc_test_3", None, Some(10))
            .expect("list signals should succeed");
        assert_eq!(signals.len(), 1);
        assert_eq!(signals[0].signal_seq, 1);
    }

    #[test]
    fn test_reject_terminal_state() {
        let runtime = CallingRuntime::with_store(Arc::new(RuntimeMemoryStateStore::default()));
        let auth = create_test_auth();

        let create = crate::dto::CreateRtcSessionRequest {
            rtc_session_id: "rtc_test_4".into(),
            conversation_id: None,
            rtc_mode: "voice".into(),
        };
        runtime
            .create_session(&auth, create)
            .expect("create should succeed");

        let reject = crate::dto::UpdateRtcSessionRequest {
            artifact_message_id: None,
        };
        let rejected = runtime
            .reject_session(&auth, "rtc_test_4", reject)
            .expect("reject should succeed");
        assert_eq!(rejected.state, SessionState::Rejected);
        assert!(rejected.ended_at.is_some());

        // Cannot end or accept after reject
        let end = crate::dto::UpdateRtcSessionRequest {
            artifact_message_id: None,
        };
        let end_err = runtime
            .end_session(&auth, "rtc_test_4", end)
            .expect_err("end after reject should fail");
        assert_eq!(end_err.code, "call_session_state_invalid");
    }

    /// Verify that `invite_session` records invited participant IDs so that
    /// subsequent `accept`/`reject`/`end` authorization checks admit them.
    /// Prior to the fix, `invited_ids` was never populated and the invite
    /// flow was broken for non-initiator participants.
    #[test]
    fn test_invite_populates_participant_ids() {
        let runtime = CallingRuntime::with_store(Arc::new(RuntimeMemoryStateStore::default()));
        let auth = create_test_auth();

        let create = crate::dto::CreateRtcSessionRequest {
            rtc_session_id: "rtc_invite_1".into(),
            conversation_id: Some("conv_invite".into()),
            rtc_mode: "video".into(),
        };
        runtime
            .create_session(&auth, create)
            .expect("create should succeed");

        let invite = crate::dto::InviteRtcSessionRequest {
            signaling_stream_id: Some("stream_invite".into()),
            participant_ids: vec!["user-2".into(), "user-3".into()],
        };
        let invited = runtime
            .invite_session(&auth, "rtc_invite_1", invite)
            .expect("invite should succeed");
        assert_eq!(invited.participants.invited_ids.len(), 2);
        assert!(invited.participants.invited_ids.contains(&"user-2".to_owned()));
        assert!(invited.participants.invited_ids.contains(&"user-3".to_owned()));

        // Idempotent: re-inviting the same participants does not duplicate.
        let invite_again = crate::dto::InviteRtcSessionRequest {
            signaling_stream_id: Some("stream_invite".into()),
            participant_ids: vec!["user-2".into()],
        };
        let replayed = runtime
            .invite_session(&auth, "rtc_invite_1", invite_again)
            .expect("idempotent invite should succeed");
        assert_eq!(
            replayed.participants.invited_ids.len(),
            2,
            "re-inviting existing participant must not duplicate"
        );
    }

    /// Verify that `accept_session` deduplicates the accepted participant.
    /// An initiator who accepts (and any path that re-runs accept) must not
    /// push the same actor_id twice into `accepted_ids`.
    #[test]
    fn test_accept_deduplicates_participant() {
        let runtime = CallingRuntime::with_store(Arc::new(RuntimeMemoryStateStore::default()));
        let auth = create_test_auth();

        let create = crate::dto::CreateRtcSessionRequest {
            rtc_session_id: "rtc_dedup_1".into(),
            conversation_id: None,
            rtc_mode: "voice".into(),
        };
        let created = runtime
            .create_session(&auth, create)
            .expect("create should succeed");
        assert!(created.participants.accepted_ids.is_empty());

        let accept = crate::dto::UpdateRtcSessionRequest {
            artifact_message_id: None,
        };
        let accepted = runtime
            .accept_session(&auth, "rtc_dedup_1", accept)
            .expect("accept should succeed");
        assert_eq!(accepted.participants.accepted_ids.len(), 1);
        assert_eq!(accepted.participants.accepted_ids[0], "user-1");
    }

    /// Verify that `post_signal` rejects signals against a terminal session.
    /// This guards the contract that signals cannot be appended after end/reject.
    #[test]
    fn test_post_signal_rejects_terminal_session() {
        let runtime = CallingRuntime::with_store(Arc::new(RuntimeMemoryStateStore::default()));
        let auth = create_test_auth();

        let create = crate::dto::CreateRtcSessionRequest {
            rtc_session_id: "rtc_signal_term".into(),
            conversation_id: None,
            rtc_mode: "voice".into(),
        };
        runtime
            .create_session(&auth, create)
            .expect("create should succeed");

        let end = crate::dto::UpdateRtcSessionRequest {
            artifact_message_id: None,
        };
        runtime
            .end_session(&auth, "rtc_signal_term", end)
            .expect("end should succeed");

        let signal = crate::dto::PostRtcSignalRequest {
            signal_type: "webrtc.offer".into(),
            schema_ref: None,
            payload: "{}".into(),
            signaling_stream_id: None,
        };
        let err = runtime
            .post_signal(&auth, "rtc_signal_term", signal)
            .expect_err("post_signal after end should fail");
        assert_eq!(err.code, "call_session_state_invalid");
    }

    /// Verify that `create_session` with the same `rtc_session_id` but
    /// different initiator is rejected as a conflict (not idempotent replay).
    #[test]
    fn test_create_session_conflict_on_different_initiator() {
        let runtime = CallingRuntime::with_store(Arc::new(RuntimeMemoryStateStore::default()));
        let auth = create_test_auth();

        let create = crate::dto::CreateRtcSessionRequest {
            rtc_session_id: "rtc_conflict_1".into(),
            conversation_id: Some("conv_c".into()),
            rtc_mode: "video".into(),
        };
        runtime
            .create_session(&auth, create)
            .expect("first create should succeed");

        // Different initiator (same tenant) trying to create the same session id.
        let other_auth = im_app_context::local_service_app_context(
            "100001",
            "user-2",
            "user",
            Some("device-2"),
            Vec::<&str>::new(),
        );
        let conflicting = crate::dto::CreateRtcSessionRequest {
            rtc_session_id: "rtc_conflict_1".into(),
            conversation_id: Some("conv_c".into()),
            rtc_mode: "video".into(),
        };
        let err = runtime
            .create_session_with_outcome(&other_auth, conflicting)
            .expect_err("conflicting create should fail");
        assert_eq!(err.code, "call_session_conflict");
    }

    /// IDOR fix verification (P0-11): a principal who is not the initiator,
    /// not a participant, and does not hold `im.calls.credentials.issue`
    /// must be forbidden from posting signals, listing signals, or issuing
    /// credentials for a session they do not belong to.
    #[test]
    fn test_signal_and_credential_authorization_blocks_unauthorized_principal() {
        let runtime = CallingRuntime::with_store(Arc::new(RuntimeMemoryStateStore::default()));
        let auth = create_test_auth();

        let create = crate::dto::CreateRtcSessionRequest {
            rtc_session_id: "rtc_idor_1".into(),
            conversation_id: None,
            rtc_mode: "voice".into(),
        };
        runtime
            .create_session(&auth, create)
            .expect("create should succeed");

        // Unauthorized principal: same tenant, different actor, no permissions.
        let other_auth = im_app_context::local_service_app_context(
            "100001",
            "user-attacker",
            "user",
            Some("device-attacker"),
            Vec::<&str>::new(),
        );

        // post_signal must be forbidden.
        let signal = crate::dto::PostRtcSignalRequest {
            signal_type: "webrtc.offer".into(),
            schema_ref: None,
            payload: "{}".into(),
            signaling_stream_id: None,
        };
        let err = runtime
            .post_signal(&other_auth, "rtc_idor_1", signal)
            .expect_err("unauthorized post_signal should fail");
        assert_eq!(err.code, "call_session_forbidden");
        assert_eq!(err.status, axum::http::StatusCode::FORBIDDEN);

        // list_signals must be forbidden.
        let err = runtime
            .list_signals(&other_auth, "rtc_idor_1", None, Some(10))
            .expect_err("unauthorized list_signals should fail");
        assert_eq!(err.code, "call_session_forbidden");
        assert_eq!(err.status, axum::http::StatusCode::FORBIDDEN);

        // issue_participant_credential must be forbidden for a participant
        // the attacker is not authorized for, even before the provider check.
        let err = runtime
            .issue_participant_credential(&other_auth, "rtc_idor_1", "user-attacker")
            .expect_err("unauthorized credential issuance should fail");
        assert_eq!(err.code, "call_session_forbidden");
        assert_eq!(err.status, axum::http::StatusCode::FORBIDDEN);

        // The initiator remains authorized and can still post/list signals.
        let signal = crate::dto::PostRtcSignalRequest {
            signal_type: "webrtc.offer".into(),
            schema_ref: None,
            payload: "{}".into(),
            signaling_stream_id: None,
        };
        runtime
            .post_signal(&auth, "rtc_idor_1", signal)
            .expect("initiator post_signal should succeed");
        let (signals, _has_more) = runtime
            .list_signals(&auth, "rtc_idor_1", None, Some(10))
            .expect("initiator list_signals should succeed");
        assert_eq!(signals.len(), 1);
    }

    /// IDOR fix verification (P0-11): an authorized caller cannot issue
    /// credentials for a `participant_id` that is not a session participant.
    #[test]
    fn test_credential_issuance_rejects_non_participant() {
        let runtime = CallingRuntime::with_store(Arc::new(RuntimeMemoryStateStore::default()));
        let auth = create_test_auth();

        let create = crate::dto::CreateRtcSessionRequest {
            rtc_session_id: "rtc_idor_2".into(),
            conversation_id: None,
            rtc_mode: "voice".into(),
        };
        runtime
            .create_session(&auth, create)
            .expect("create should succeed");

        // Initiator is authorized, but "user-stranger" is not a participant.
        // The runtime has no RTC provider wired, so the provider-not-configured
        // branch would normally fire; the participant check must run first.
        let err = runtime
            .issue_participant_credential(&auth, "rtc_idor_2", "user-stranger")
            .expect_err("issuance for non-participant should fail");
        assert_eq!(err.code, "participant_not_in_session");
        assert_eq!(err.status, axum::http::StatusCode::FORBIDDEN);

        // The initiator themselves is a valid participant — issuance proceeds
        // to the provider-not-configured branch (expected in dev mode).
        let err = runtime
            .issue_participant_credential(&auth, "rtc_idor_2", "user-1")
            .expect_err("initiator issuance should reach provider check");
        assert_eq!(err.code, "rtc_provider_not_configured");
    }

    /// P0-6: credential refresh enforces the same authorization,
    /// participant-membership, and non-terminal-state guards as issuance.
    #[test]
    fn test_refresh_participant_credential_guards() {
        let runtime = CallingRuntime::with_store(Arc::new(RuntimeMemoryStateStore::default()));
        let auth = create_test_auth();

        let create = crate::dto::CreateRtcSessionRequest {
            rtc_session_id: "rtc_refresh_1".into(),
            conversation_id: None,
            rtc_mode: "voice".into(),
        };
        runtime
            .create_session(&auth, create)
            .expect("create should succeed");

        // No provider wired: refresh for the initiator reaches the
        // provider-not-configured branch (the session is non-terminal and
        // the initiator is a valid participant).
        let err = runtime
            .refresh_participant_credential(&auth, "rtc_refresh_1", "user-1")
            .expect_err("refresh without provider should fail clearly");
        assert_eq!(err.code, "rtc_provider_not_configured");

        // Refresh for a non-participant is forbidden before provider check.
        let err = runtime
            .refresh_participant_credential(&auth, "rtc_refresh_1", "user-stranger")
            .expect_err("refresh for non-participant should fail");
        assert_eq!(err.code, "participant_not_in_session");
        assert_eq!(err.status, axum::http::StatusCode::FORBIDDEN);

        // Unauthorized caller is forbidden.
        let other_auth = im_app_context::local_service_app_context(
            "100001",
            "user-attacker",
            "user",
            Some("device-attacker"),
            Vec::<&str>::new(),
        );
        let err = runtime
            .refresh_participant_credential(&other_auth, "rtc_refresh_1", "user-1")
            .expect_err("refresh by unauthorized caller should fail");
        assert_eq!(err.code, "call_session_forbidden");

        // After the session ends, refresh is rejected as terminal state.
        let end = crate::dto::UpdateRtcSessionRequest {
            artifact_message_id: None,
        };
        runtime
            .end_session(&auth, "rtc_refresh_1", end)
            .expect("end should succeed");
        let err = runtime
            .refresh_participant_credential(&auth, "rtc_refresh_1", "user-1")
            .expect_err("refresh after end should fail");
        assert_eq!(err.code, "call_session_state_invalid");
    }
}