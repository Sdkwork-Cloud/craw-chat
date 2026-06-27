use sdkwork_im_runtime_link::{
    LinkConnectionState, LinkSession, OutboundQueuePolicy, ResumeWindow,
};
use sdkwork_im_runtime_route::{RouteBindingRequest, RouteDirectory};

#[test]
fn test_step04_runtime_plane_skeleton_supports_link_lifecycle_and_route_epoch_migration() {
    let queue_policy = OutboundQueuePolicy::new(16, 64).expect("queue policy should be valid");
    let mut session = LinkSession::new(
        "100001",
        "1008",
        "user",
        "device_demo",
        Some("session_demo"),
        queue_policy.clone(),
    );
    assert_eq!(session.state(), LinkConnectionState::Connected);
    assert_eq!(session.queue_policy(), &queue_policy);

    session.mark_hello_negotiated();
    assert_eq!(session.state(), LinkConnectionState::HelloNegotiated);

    session.mark_authenticated();
    assert_eq!(session.state(), LinkConnectionState::Authenticated);

    let resume_window = ResumeWindow::new(12, 8);
    session.activate(resume_window.clone());
    assert_eq!(session.state(), LinkConnectionState::Active);
    assert_eq!(session.resume_window(), &resume_window);

    session.mark_draining();
    assert_eq!(session.state(), LinkConnectionState::Draining);

    let routes = RouteDirectory::default();
    routes.register_node("node_a");
    routes.register_node("node_b");

    let first_bind = routes
        .bind(RouteBindingRequest::new(
            "100001",
            "1008",
            "user",
            "device_demo",
            "node_a",
        ))
        .expect("initial route bind should succeed");
    assert_eq!(first_bind.route_epoch, 1);
    assert_eq!(first_bind.owner_node_id, "node_a");

    let draining = routes
        .mark_node_draining("node_a")
        .expect("source node should enter draining");
    assert_eq!(draining.drain_status, "draining");
    assert_eq!(draining.rebalance_state, "moving_routes");
    assert_eq!(draining.owned_route_count, 1);

    let migration = routes
        .migrate_routes("node_a", "node_b")
        .expect("route migration should succeed");
    assert_eq!(migration.migrated_route_count, 1);
    assert_eq!(migration.source_drain_status, "drained");
    assert_eq!(migration.source_rebalance_state, "stable");
    assert_eq!(migration.target_drain_status, "active");
    assert_eq!(migration.target_rebalance_state, "stable");

    let rebound = routes
        .lookup("100001", "default", "1008", "user", "device_demo")
        .expect("route should remain present after migration");
    assert_eq!(rebound.owner_node_id, "node_b");
    assert_eq!(rebound.route_epoch, 2);
}
