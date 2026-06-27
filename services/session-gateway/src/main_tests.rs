//! White-box unit tests for session-gateway main entrypoint.
//!
//! Extracted from the implementation file so it stays focused on logic
//! while the tests (which exercise private items) live beside it. Mounted
//! back via `#[cfg(test)] #[path = "main_tests.rs"] mod tests;` so
//! `use super::{...}` still resolves to the parent module unchanged.

use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use super::resolve_bind_addr_from_env;

#[test]
fn resolve_bind_addr_prefers_service_specific_env_value() {
        let resolved = resolve_bind_addr_from_env(Some("0.0.0.0:28080"), Some("127.0.0.1:18080"))
        .expect("service-specific bind addr should parse");

    assert_eq!(
        resolved,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 28080)
    );
}

#[test]
fn resolve_bind_addr_falls_back_to_topology_bind_env() {
    let resolved = resolve_bind_addr_from_env(None, Some("127.0.0.1:18080"))
        .expect("topology bind env should parse");

    assert_eq!(
        resolved,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 18080)
    );
}

#[test]
fn resolve_bind_addr_uses_default_when_no_env_values_are_present() {
    let resolved = resolve_bind_addr_from_env(None, None).expect("default bind addr should parse");

    assert_eq!(
        resolved,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 18080)
    );
}

#[test]
fn resolve_bind_addr_rejects_invalid_values() {
    let error = resolve_bind_addr_from_env(Some("not-a-socket-addr"), None)
        .expect_err("invalid bind addr should fail");

    assert!(
        error.contains("invalid session-gateway bind address"),
        "unexpected error: {error}"
    );
}
