use im_app_context::DualTokenRequestBuilderExt;
use std::sync::Arc;

use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use session_gateway::{
    RealtimeClusterBridge, RealtimeDeliveryRuntime, RealtimeSubscriptionItemInput,
};
use tower::ServiceExt;

#[tokio::test]
async fn test_control_plane_can_drain_and_migrate_routes() {
    let cluster = Arc::new(RealtimeClusterBridge::default());
    let runtime_a = Arc::new(RealtimeDeliveryRuntime::default());
    let runtime_b = Arc::new(RealtimeDeliveryRuntime::default());
    cluster.bind_node_runtime("node_a", runtime_a.clone());
    cluster.bind_node_runtime("node_b", runtime_b.clone());

    let _ = runtime_a.sync_subscriptions_for_principal_kind(
        "100001",
        "default",
        "1",
        "user",
        "d_pad",
        vec![RealtimeSubscriptionItemInput {
            scope_type: "conversation".into(),
            scope_id: "c_demo".into(),
            event_types: vec!["message.posted".into()],
        }],
    );
    cluster
        .bind_client_route_for_principal_kind(
            "100001",
            "default",
            "1",
            "user",
            "d_pad",
            "node_a",
            None,
            "websocket",
        )
        .expect("route bind should succeed");

    let app = governance_service::build_app_with_cluster(cluster.clone());

    let drain_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/control/nodes/node_a/drain")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("control.write")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("drain request should succeed");
    assert_eq!(drain_response.status(), StatusCode::OK);
    let drain_body = drain_response
        .into_body()
        .collect()
        .await
        .expect("drain body should collect")
        .to_bytes();
    let drain_json: serde_json::Value =
        serde_json::from_slice(&drain_body).expect("drain body should be valid json");
    assert_eq!(drain_json["nodeId"], "node_a");
    assert_eq!(drain_json["drainStatus"], "draining");
    assert_eq!(drain_json["rebalanceState"], "moving_routes");
    assert_eq!(drain_json["ownedRouteCount"], 1);

    let migrate_response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/control/nodes/node_a/routes/migrate")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("control.write")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"targetNodeId":"node_b"}"#))
                .unwrap(),
        )
        .await
        .expect("migrate request should succeed");
    assert_eq!(migrate_response.status(), StatusCode::OK);
    let migrate_body = migrate_response
        .into_body()
        .collect()
        .await
        .expect("migrate body should collect")
        .to_bytes();
    let migrate_json: serde_json::Value =
        serde_json::from_slice(&migrate_body).expect("migrate body should be valid json");
    assert_eq!(migrate_json["sourceNodeId"], "node_a");
    assert_eq!(migrate_json["targetNodeId"], "node_b");
    assert_eq!(migrate_json["migratedRouteCount"], 1);
    assert_eq!(migrate_json["sourceDrainStatus"], "drained");
    assert_eq!(migrate_json["targetDrainStatus"], "active");

    let migrated_route = cluster
        .resolve_client_route_for_principal_kind("100001", "default", "1", "user", "d_pad")
        .expect("route should exist after migration");
    assert_eq!(migrated_route.owner_node_id, "node_b");
}

#[tokio::test]
async fn test_control_plane_rejects_unknown_node_lifecycle_writes() {
    let app =
        governance_service::build_app_with_cluster(Arc::new(RealtimeClusterBridge::default()));

    let drain_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/control/nodes/node_missing/drain")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("control.write")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("unknown-node drain request should return response");
    assert_eq!(drain_response.status(), StatusCode::NOT_FOUND);
    let drain_body = drain_response
        .into_body()
        .collect()
        .await
        .expect("unknown-node drain body should collect")
        .to_bytes();
    let drain_json: serde_json::Value =
        serde_json::from_slice(&drain_body).expect("unknown-node drain body should be valid json");
    assert_eq!(drain_json["code"], "node_not_found");

    let activate_response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/control/nodes/node_missing/activate")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("control.write")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("unknown-node activate request should return response");
    assert_eq!(activate_response.status(), StatusCode::NOT_FOUND);
    let activate_body = activate_response
        .into_body()
        .collect()
        .await
        .expect("unknown-node activate body should collect")
        .to_bytes();
    let activate_json: serde_json::Value = serde_json::from_slice(&activate_body)
        .expect("unknown-node activate body should be valid json");
    assert_eq!(activate_json["code"], "node_not_found");
}

#[tokio::test]
async fn test_control_plane_rejects_migrate_when_source_node_is_not_draining() {
    let cluster = Arc::new(RealtimeClusterBridge::default());
    let runtime_a = Arc::new(RealtimeDeliveryRuntime::default());
    let runtime_b = Arc::new(RealtimeDeliveryRuntime::default());
    cluster.bind_node_runtime("node_a", runtime_a);
    cluster.bind_node_runtime("node_b", runtime_b);

    let app = governance_service::build_app_with_cluster(cluster.clone());

    let migrate_response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/control/nodes/node_a/routes/migrate")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("control.write")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"targetNodeId":"node_b"}"#))
                .unwrap(),
        )
        .await
        .expect("migrate request should return response");
    assert_eq!(migrate_response.status(), StatusCode::CONFLICT);
    let migrate_body = migrate_response
        .into_body()
        .collect()
        .await
        .expect("migrate body should collect")
        .to_bytes();
    let migrate_json: serde_json::Value =
        serde_json::from_slice(&migrate_body).expect("migrate body should be valid json");
    assert_eq!(migrate_json["code"], "node_not_draining");

    let source = cluster
        .node_lifecycle("node_a")
        .expect("source node lifecycle should remain");
    assert_eq!(source.drain_status, "active");
    assert_eq!(source.rebalance_state, "stable");
}
