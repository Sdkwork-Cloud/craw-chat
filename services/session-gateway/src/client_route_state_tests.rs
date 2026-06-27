//! White-box unit tests for client route state registry.
//!
//! Extracted from the implementation file so it stays focused on logic
//! while the tests (which exercise private items) live beside it. Mounted
//! back via `#[cfg(test)] #[path = "client_route_state_tests.rs"] mod tests;` so
//! `use super::{...}` still resolves to the parent module unchanged.

use super::*;

fn auth_context(principal_id: &str, actor_kind: &str, device_id: &str) -> AppContext {
    AppContext {
        tenant_id: "100001".into(),
        organization_id: None,
        user_id: principal_id.into(),
        actor_id: principal_id.into(),
        actor_kind: actor_kind.into(),
        session_id: Some(format!("s_{actor_kind}")),
        app_id: None,
        environment: None,
        deployment_mode: None,
        auth_level: None,
        data_scope: Default::default(),
        permission_scope: Default::default(),
        device_id: Some(device_id.into()),
    }
}

#[test]
fn test_register_route_key_recovers_from_poisoned_route_key_lock() {
    let state = ClientRouteState::default();
    let _ = std::panic::catch_unwind({
        let registered_route_keys = state.registered_route_keys.clone();
        move || {
            let _guard = registered_route_keys
                .lock()
                .expect("client route key store should lock");
            panic!("poison client route key store lock");
        }
    });

    let auth = auth_context("1", "user", "d_poison");
    state.register_route_key(&auth, "d_poison");
    assert!(state.has_registered_route_key(&auth, "d_poison"));
}

#[test]
fn test_client_route_state_isolated_by_actor_kind_for_same_actor_id() {
    let state = ClientRouteState::default();
    let user_auth = auth_context("1", "user", "d_user");
    let agent_auth = auth_context("1", "agent", "d_agent");

    state.register_route_key(&user_auth, "d_user");
    state.register_route_key(&agent_auth, "d_agent");

    let user_state = state
        .client_route_state_snapshot(&user_auth, Some("d_user"))
        .expect("user sync state should resolve");
    let agent_state = state
        .client_route_state_snapshot(&agent_auth, Some("d_agent"))
        .expect("agent sync state should resolve");

    assert_eq!(user_state.registered_route_keys, vec!["d_user"]);
    assert_eq!(agent_state.registered_route_keys, vec!["d_agent"]);
}

#[test]
fn test_route_key_conflict_rejected_for_same_actor_and_actor_kind_change() {
    let state = ClientRouteState::default();
    let user_auth = auth_context("1", "user", "d_shared");
    let agent_auth = auth_context("1", "agent", "d_shared");

    state.register_route_key(&user_auth, "d_shared");
    let error = state
        .ensure_route_key_available(&agent_auth, "d_shared")
        .expect_err("different actor kind should be rejected for same actor route key");
    assert_eq!(error.code, "client_route_scope_conflict");
}

#[test]
fn test_route_key_owner_conflict_rejected_for_different_actor_same_route_key() {
    let state = ClientRouteState::default();
    let owner_a = auth_context("1001", "user", "d_shared");
    let owner_b = auth_context("1002", "user", "d_shared");

    state.register_route_key(&owner_a, "d_shared");
    let error = state
        .ensure_route_key_available(&owner_b, "d_shared")
        .expect_err("different owner should be rejected for same tenant route key");
    assert_eq!(error.code, "client_route_scope_conflict");
}
