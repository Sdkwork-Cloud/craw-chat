//! Interaction Service source structure test.
//!
//! Guards the deprecated-shim contract: the service must remain a thin
//! health-only router and must not regain business handlers for reactions,
//! pins, threads, or conversation settings.

#[test]
fn interaction_service_remains_health_only_router() {
    let lib_source = include_str!("../src/lib.rs").replace("\r\n", "\n");
    let http_source = include_str!("../src/http.rs").replace("\r\n", "\n");

    // The lib.rs must clearly document the deprecation.
    assert!(
        lib_source.contains("deprecated") || lib_source.contains("Deprecated"),
        "interaction-service lib.rs must document its deprecated status"
    );

    // The http.rs must only mount infra routes, not business routes.
    assert!(
        http_source.contains("mount_im_infra_routes"),
        "interaction-service http.rs must use mount_im_infra_routes for health-only routing"
    );

    // The service must not define reaction/pin/thread handlers.
    for forbidden in [
        "async fn list_reactions",
        "async fn create_reaction",
        "async fn delete_reaction",
        "async fn list_pins",
        "async fn create_pin",
        "async fn delete_pin",
        "async fn list_threads",
        "async fn create_thread",
    ] {
        assert!(
            !http_source.contains(forbidden),
            "interaction-service must not define business handler: {forbidden}"
        );
    }
}
