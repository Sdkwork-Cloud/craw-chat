//! White-box unit tests for session-gateway principal scope keys.
//!
//! Extracted from the implementation file so it stays focused on logic
//! while the tests (which exercise private items) live beside it. Mounted
//! back via `#[cfg(test)] #[path = "principal_scope_tests.rs"] mod tests;` so
//! `use super::{...}` still resolves to the parent module unchanged.

use super::*;

#[test]
fn test_principal_scope_keys_are_not_delimiter_collision_prone() {
    assert_ne!(
        typed_principal_scope_key("t:a", "default", "b", "c"),
        typed_principal_scope_key("t", "default", "a:b", "c"),
        "typed principal scope keys must encode tenant/principal boundaries unambiguously"
    );
    assert_ne!(
        typed_client_route_scope_key("t", "default", "u:d", "user", "1"),
        typed_client_route_scope_key("t", "default", "u", "user", "d:1"),
        "typed device scope keys must encode principal/device boundaries unambiguously"
    );
    assert_ne!(
        tenant_client_route_scope_key("t:d", "1"),
        tenant_client_route_scope_key("t", "d:1"),
        "tenant device scope keys must encode tenant/device boundaries unambiguously"
    );
}

#[test]
fn test_principal_scope_keys_isolate_organizations() {
    assert_ne!(
        typed_client_route_scope_key("100001", "org_a", "1", "user", "d_pad"),
        typed_client_route_scope_key("100001", "org_b", "1", "user", "d_pad"),
        "client route scope keys must isolate organizations"
    );
    assert_eq!(
        typed_client_route_scope_key("100001", "", "1", "user", "d_pad"),
        typed_client_route_scope_key("100001", "default", "1", "user", "d_pad"),
        "empty organization_id must normalize to default"
    );
    assert!(
        typed_client_route_scope_key("100001", "org_a", "1", "user", "d_pad")
            .starts_with(
                typed_principal_scope_key("100001", "org_a", "1", "user").as_str()
            ),
        "client route scope key must extend principal scope key prefix"
    );
}
