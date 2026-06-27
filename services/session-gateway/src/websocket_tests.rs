//! White-box unit tests for session-gateway realtime websocket loop.
//!
//! Extracted from the implementation file so it stays focused on logic
//! while the tests (which exercise private items) live beside it. Mounted
//! back via `#[cfg(test)] #[path = "websocket_tests.rs"] mod tests;` so
//! `use super::{...}` still resolves to the parent module unchanged.

use im_app_context::AppContext;
use sdkwork_im_ccp_control::HelloFrame;
use sdkwork_im_ccp_core::{CapabilitySet, ProtocolVersion, TransportBinding};
use sdkwork_im_runtime_link::{LinkConnectionState, OutboundQueuePolicy, ResumeWindow};

use super::*;

fn demo_auth_context() -> AppContext {
    AppContext {
        tenant_id: "100001".into(),
        organization_id: "0".into(),
        user_id: "1".into(),
        actor_id: "1".into(),
        actor_kind: "user".into(),
        session_id: Some("s_demo".into()),
        app_id: None,
        environment: None,
        deployment_mode: None,
        auth_level: None,
        data_scope: Default::default(),
        permission_scope: Default::default(),
        device_id: Some("d_pad".into()),
    }
}

#[test]
fn test_build_active_link_session_maps_checkpoint_into_runtime_link_owner() {
    let auth = demo_auth_context();
    let checkpoint = RealtimeWindowCheckpoint {
        latest_realtime_seq: 17,
        acked_through_seq: 9,
        trimmed_through_seq: 9,
    };

    let mut session = build_link_session(&auth, "d_pad");
    session.mark_authenticated();
    activate_link_session(&mut session, &checkpoint);

    assert_eq!(session.state(), LinkConnectionState::Active);
    assert_eq!(session.tenant_id, "100001");
    assert_eq!(session.principal_id, "1");
    assert_eq!(session.actor_kind, "user");
    assert_eq!(session.device_id, "d_pad");
    assert_eq!(session.session_id.as_deref(), Some("s_demo"));
    assert_eq!(session.resume_window(), &ResumeWindow::new(17, 9));
}

#[test]
fn test_build_link_session_uses_runtime_link_default_queue_owner_policy() {
    let auth = demo_auth_context();

    let session = build_link_session(&auth, "d_pad");

    assert_eq!(
        session.queue_policy(),
        &OutboundQueuePolicy::realtime_default()
    );
}

#[test]
fn test_build_link_session_preserves_actor_identity_for_runtime_link_auth_owner() {
    let auth = demo_auth_context();

    let session = build_link_session(&auth, "d_pad");

    assert!(session.matches_auth_bind("1", "user", Some("d_pad"), Some("s_demo")));
}

#[test]
fn test_build_link_session_negotiates_hello_via_runtime_link_owner_and_intersects_registry_capabilities()
 {
    let auth = demo_auth_context();
    let mut session = build_link_session(&auth, "d_pad");
    let hello = HelloFrame {
        protocol: ProtocolVersion::new("ccp", 1, 0),
        binding: TransportBinding::Ws1,
        capabilities: CapabilitySet::from_iter(["session.resume", "payload.json", "ignored"]),
        trace_id: Some("trace-hello".into()),
    };

    let hello_ack = session
        .negotiate_hello(&hello)
        .expect("runtime-link should accept supported hello frame");

    assert_eq!(session.state(), LinkConnectionState::HelloNegotiated);
    assert_eq!(hello_ack.protocol, ProtocolVersion::new("ccp", 1, 0));
    assert_eq!(hello_ack.binding, TransportBinding::Ws1);
    assert_eq!(
        hello_ack.capabilities,
        CapabilitySet::from_iter(["payload.json", "session.resume"])
    );
    assert!(
        !hello_ack.capabilities.supports("ignored"),
        "hello negotiation must drop capabilities outside the control-plane registry"
    );
    assert!(
        hello_ack.capabilities.supports("session.resume"),
        "control-plane registry publishes session.resume for runtime-link hello negotiation"
    );
    assert!(hello_ack.accepted);
}
