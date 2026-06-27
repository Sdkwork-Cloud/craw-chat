//! White-box unit tests for session-gateway websocket upgrade.
//!
//! Extracted from the implementation file so it stays focused on logic
//! while the tests (which exercise private items) live beside it. Mounted
//! back via `#[cfg(test)] #[path = "websocket_upgrade_tests.rs"] mod tests;` so
//! `use super::{...}` still resolves to the parent module unchanged.

use std::sync::Arc;

use sdkwork_im_runtime_link::LinkWebsocketMode;
use im_app_context::AppContext;

use super::{
    prepare_realtime_websocket_upgrade, realtime_websocket_subprotocols,
    select_realtime_websocket_mode,
};
use crate::{RealtimeDeliveryRuntime, RealtimeWebsocketMode};

#[test]
fn test_realtime_websocket_upgrade_uses_runtime_link_owner_contract() {
    assert_eq!(
        realtime_websocket_subprotocols(),
        [crate::CCP_WEBSOCKET_SUBPROTOCOL]
    );
    assert_eq!(
        select_realtime_websocket_mode(Some(crate::CCP_WEBSOCKET_SUBPROTOCOL)),
        RealtimeWebsocketMode::CcpJson
    );
    assert_eq!(
        select_realtime_websocket_mode(Some("legacy.json")),
        RealtimeWebsocketMode::LegacyJson
    );
    assert_eq!(
        select_realtime_websocket_mode(None),
        RealtimeWebsocketMode::LegacyJson
    );
}

#[test]
fn test_realtime_websocket_upgrade_prepares_runtime_link_handoff_owner() {
    let runtime = Arc::new(RealtimeDeliveryRuntime::default());
    let handoff = prepare_realtime_websocket_upgrade(
        Some(crate::CCP_WEBSOCKET_SUBPROTOCOL),
        AppContext {
            tenant_id: "100001".into(),
            organization_id: None,
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
        },
        "d_pad".into(),
        runtime.clone(),
        crate::client_route_registration::ClientRouteRegistration::new(
            "node_a".into(),
            Arc::new(crate::RealtimeClusterBridge::default()),
            Arc::new(crate::PresenceRuntime::default()),
            runtime,
            crate::client_route_state::ClientRouteState::default(),
        ),
    );

    assert_eq!(handoff.mode(), LinkWebsocketMode::CcpJson);
    assert_eq!(handoff.context().auth.tenant_id, "100001");
    assert_eq!(handoff.context().auth.actor_id, "1");
    assert_eq!(handoff.context().device_id, "d_pad");
}
