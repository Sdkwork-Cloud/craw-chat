#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use im_domain_core::rtc::RtcSessionState;
    use crate::state::{CallingRuntime, RuntimeMemoryRtcStateStore};

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
        let runtime = CallingRuntime::with_store(Arc::new(RuntimeMemoryRtcStateStore::default()));
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
        assert_eq!(first.session.state, RtcSessionState::Started);
        assert_eq!(first.session.rtc_session_id, "rtc_test_1");

        let second = runtime
            .create_session_with_outcome(&auth, request)
            .expect("second create should succeed (idempotent)");
        assert!(!second.applied);
        assert_eq!(second.session.rtc_session_id, "rtc_test_1");
    }

    #[test]
    fn test_session_state_machine() {
        let runtime = CallingRuntime::with_store(Arc::new(RuntimeMemoryRtcStateStore::default()));
        let auth = create_test_auth();

        let create = crate::dto::CreateRtcSessionRequest {
            rtc_session_id: "rtc_test_2".into(),
            conversation_id: Some("conv_2".into()),
            rtc_mode: "video".into(),
        };
        let created = runtime
            .create_session(&auth, create)
            .expect("create should succeed");

        let invite = crate::dto::InviteRtcSessionRequest {
            signaling_stream_id: Some("stream_1".into()),
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
        assert_eq!(accepted.state, RtcSessionState::Accepted);

        let end = crate::dto::UpdateRtcSessionRequest {
            artifact_message_id: None,
        };
        let ended = runtime
            .end_session(&auth, "rtc_test_2", end)
            .expect("end should succeed");
        assert_eq!(ended.state, RtcSessionState::Ended);
        assert!(ended.ended_at.is_some());
    }

    #[test]
    fn test_signal_posting() {
        let runtime = CallingRuntime::with_store(Arc::new(RuntimeMemoryRtcStateStore::default()));
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
        let runtime = CallingRuntime::with_store(Arc::new(RuntimeMemoryRtcStateStore::default()));
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
        assert_eq!(rejected.state, RtcSessionState::Rejected);
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
}