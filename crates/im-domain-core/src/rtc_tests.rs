//! RTC state-record unit tests.
//!
//! Extracted from `rtc.rs` so the implementation file stays focused on domain
//! logic while the white-box tests (which construct `RtcStateRecord` /
//! `RtcSignalEvent` via their public fields) live beside it. Mounted back into
//! `rtc` via `#[cfg(test)] #[path = "rtc_tests.rs"] mod tests;` in `rtc.rs`,
//! which keeps `use super::*` resolving to the `rtc` module unchanged.

use super::*;

#[test]
fn state_record_merge_preserves_accepted_session_over_stale_reject() {
    let accepted = rtc_state_record(
        RtcSessionState::Accepted,
        "2026-05-06T00:00:03.000Z",
        vec![rtc_signal_event(1), rtc_signal_event(2)],
    );
    let stale_reject = rtc_state_record(
        RtcSessionState::Rejected,
        "2026-05-06T00:00:02.000Z",
        vec![rtc_signal_event(1)],
    );

    let merged = accepted.merge_monotonic(stale_reject);

    assert_eq!(merged.session.state, RtcSessionState::Accepted);
    assert_eq!(merged.updated_at, "2026-05-06T00:00:03.000Z");
    assert_eq!(
        merged
            .signals
            .iter()
            .map(|signal| signal.signal_seq)
            .collect::<Vec<_>>(),
        vec![1, 2]
    );
}

#[test]
fn state_record_merge_compares_updated_at_by_rfc3339_instant() {
    let whole_second = rtc_state_record(
        RtcSessionState::Accepted,
        "2026-05-06T00:00:00Z",
        vec![rtc_signal_event(1)],
    );
    let later_fraction = rtc_state_record(
        RtcSessionState::Accepted,
        "2026-05-06T00:00:00.100Z",
        vec![rtc_signal_event(2)],
    );

    let merged = whole_second.merge_monotonic(later_fraction);

    assert_eq!(merged.updated_at, "2026-05-06T00:00:00.100Z");
    assert_eq!(
        merged
            .signals
            .iter()
            .map(|signal| signal.signal_seq)
            .collect::<Vec<_>>(),
        vec![1, 2]
    );
}

fn rtc_state_record(
    state: RtcSessionState,
    updated_at: &str,
    signals: Vec<RtcSignalEvent>,
) -> RtcStateRecord {
    RtcStateRecord {
        tenant_id: "100001".into(),
        rtc_session_id: "rtc_demo".into(),
        session: RtcSession {
            tenant_id: "100001".into(),
            rtc_session_id: "rtc_demo".into(),
            conversation_id: Some("c_demo".into()),
            rtc_mode: "voice".into(),
            initiator_id: "1".into(),
            initiator_kind: "user".into(),
            provider_plugin_id: Some("webrtc".into()),
            provider_session_id: Some("ps_demo".into()),
            access_endpoint: Some("wss://rtc.example.test/session/ps_demo".into()),
            provider_region: Some("cn-shanghai".into()),
            state,
            signaling_stream_id: Some("st_demo".into()),
            artifact_message_id: None,
            started_at: "2026-05-06T00:00:00.000Z".into(),
            ended_at: None,
        },
        signals,
        updated_at: updated_at.into(),
    }
}

fn rtc_signal_event(signal_seq: u64) -> RtcSignalEvent {
    RtcSignalEvent {
        tenant_id: "100001".into(),
        rtc_session_id: "rtc_demo".into(),
        signal_seq,
        conversation_id: Some("c_demo".into()),
        rtc_mode: "voice".into(),
        signal_type: format!("rtc.signal.{signal_seq}"),
        schema_ref: Some("webrtc.signal.v1".into()),
        payload: format!("{{\"seq\":{signal_seq}}}"),
        sender: RtcSignalSender {
            id: "1".into(),
            kind: "user".into(),
            member_id: None,
            device_id: Some("d_demo".into()),
            session_id: Some("s_demo".into()),
            metadata: Default::default(),
        },
        signaling_stream_id: Some("st_demo".into()),
        occurred_at: format!("2026-05-06T00:00:0{signal_seq}.000Z"),
    }
}
