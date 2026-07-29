use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use im_app_context::DualTokenRequestBuilderExt;
use std::sync::Arc;
use tower::ServiceExt;

fn ops_route_http_test_app() -> axum::Router {
    sdkwork_routes_im_ops_backend_api::build_public_app_with_runtime(Arc::new(
        ops_service::OpsRuntime::default(),
    ))
}

#[tokio::test]
async fn test_route_composition_exports_required_infrastructure_endpoints() {
    let app = ops_route_http_test_app();

    for path in ["/healthz", "/metrics", "/openapi.json", "/docs"] {
        let response = app
            .clone()
            .oneshot(Request::builder().uri(path).body(Body::empty()).unwrap())
            .await
            .expect("infrastructure request should succeed");
        assert_eq!(response.status(), StatusCode::OK, "endpoint {path}");
    }

    let readiness = app
        .oneshot(
            Request::builder()
                .uri("/readyz")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("readiness request should succeed");
    assert!(
        matches!(
            readiness.status(),
            StatusCode::OK | StatusCode::SERVICE_UNAVAILABLE
        ),
        "readiness endpoint must report actual dependency state"
    );
}

#[tokio::test]
async fn test_public_app_exports_live_openapi_json() {
    let app = ops_service::build_public_app();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/openapi.json")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("request should succeed");

    assert_eq!(response.status(), StatusCode::OK);

    let body = response
        .into_body()
        .collect()
        .await
        .expect("body should collect")
        .to_bytes();
    let value: serde_json::Value =
        serde_json::from_slice(&body).expect("body should be valid json");

    assert_eq!(value["openapi"], "3.1.0");
    assert_eq!(value["info"]["title"], "Sdkwork IM Ops Service API");
    assert!(value["paths"]["/backend/v3/api/ops/health"].is_object());
    assert!(value["paths"]["/backend/v3/api/ops/retention/purge"].is_object());
    let lag = &value["paths"]["/backend/v3/api/ops/lag"]["get"];
    assert_eq!(lag["operationId"], "lag.retrieve");
    assert_eq!(lag["parameters"].as_array().map(Vec::len), Some(2));
    assert_eq!(
        lag["responses"]["200"]["content"]["application/json"]["schema"]["$ref"],
        "#/components/schemas/LagListResponse"
    );
    assert!(value["components"]["parameters"]["PageSizeQuery"].is_object());
    assert!(value["components"]["parameters"]["CursorQuery"].is_object());
    assert!(value["components"]["schemas"]["LagPageData"].is_object());
    assert!(value["components"]["schemas"]["PageInfo"].is_object());
}

#[tokio::test]
async fn test_public_app_serves_docs_page_for_live_openapi() {
    let app = ops_service::build_public_app();

    let response = app
        .oneshot(Request::builder().uri("/docs").body(Body::empty()).unwrap())
        .await
        .expect("request should succeed");

    assert_eq!(response.status(), StatusCode::OK);

    let body = response
        .into_body()
        .collect()
        .await
        .expect("body should collect")
        .to_bytes();
    let html = String::from_utf8(body.to_vec()).expect("docs should be valid utf-8");

    assert!(html.contains("OpenAPI 3.1"));
    assert!(html.contains("Sdkwork IM Ops Service API"));
    assert!(html.contains("/openapi.json"));
}

#[tokio::test]
async fn test_public_app_exposes_retention_metrics() {
    let app = ops_service::build_public_app();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/metrics")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("metrics request should succeed");

    assert_eq!(response.status(), StatusCode::OK);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("metrics body should collect")
        .to_bytes();
    let text = String::from_utf8(body.to_vec()).expect("metrics should be utf-8");
    assert!(text.contains("im_retention_purge_batches_total"));
    assert!(text.contains("store=\"message_media_refs\""));
    assert!(text.contains("im_health_status"));
    assert!(text.contains("sdkwork_http_requests_total"));
    assert!(text.contains("sdkwork_health_status"));
}

#[tokio::test]
async fn test_retention_purge_route_requires_ops_write_over_http() {
    let app = ops_route_http_test_app();

    let forbidden = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/ops/retention/purge")
                .with_dual_token_tenant("100001")
                .with_dual_token_organization("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("ops.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("retention purge request should complete");
    assert_eq!(forbidden.status(), StatusCode::FORBIDDEN);

    let authorized = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/ops/retention/purge")
                .with_dual_token_tenant("100001")
                .with_dual_token_organization("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("ops.write")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("authorized retention purge request should complete");
    assert!(
        authorized.status() == StatusCode::OK
            || authorized.status() == StatusCode::SERVICE_UNAVAILABLE,
        "expected ok or database_unconfigured unavailable, got {}",
        authorized.status()
    );
}

#[tokio::test]
async fn test_cluster_lag_health_runtime_dir_and_diagnostics_over_http() {
    let app = ops_route_http_test_app();

    let health_response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/backend/v3/api/ops/health")
                .with_dual_token_tenant("100001")
                .with_dual_token_organization("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("ops.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("ops health should succeed");
    assert_eq!(health_response.status(), StatusCode::OK);
    let health_body = health_response
        .into_body()
        .collect()
        .await
        .expect("health body should collect")
        .to_bytes();
    let health_json: serde_json::Value =
        serde_json::from_slice(&health_body).expect("health body should be valid json");
    let health = &health_json["data"]["item"];
    assert_eq!(health["status"], "unavailable");
    assert!(health.get("projectionPlane").is_none());
    assert_eq!(health["realtimeInbox"]["status"], "unavailable");
    assert_eq!(health["realtimeInbox"]["pendingEventCount"], 0);
    assert_eq!(
        health["realtimeInbox"]["maxClientRouteWindowUsagePermille"],
        0
    );
    assert_eq!(health["realtimeInbox"]["capacityTrimmedEventCount"], 0);
    assert_eq!(health["realtimeInbox"]["maxCapacityTrimmedThroughSeq"], 0);
    assert!(health["realtimeInbox"]["lastCapacityTrimmedAt"].is_null());

    let cluster_response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/backend/v3/api/ops/cluster")
                .with_dual_token_tenant("100001")
                .with_dual_token_organization("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("ops.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("ops cluster should succeed");
    assert_eq!(cluster_response.status(), StatusCode::OK);
    let cluster_body = cluster_response
        .into_body()
        .collect()
        .await
        .expect("cluster body should collect")
        .to_bytes();
    let cluster_json: serde_json::Value =
        serde_json::from_slice(&cluster_body).expect("cluster body should be valid json");
    assert_eq!(
        cluster_json["data"]["item"]["nodes"]
            .as_array()
            .expect("cluster nodes should be an array")
            .len(),
        0,
        "unobserved nodes must not be fabricated"
    );

    let lag_response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/backend/v3/api/ops/lag")
                .with_dual_token_tenant("100001")
                .with_dual_token_organization("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("ops.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("ops lag should succeed");
    assert_eq!(lag_response.status(), StatusCode::OK);
    let lag_body = lag_response
        .into_body()
        .collect()
        .await
        .expect("lag body should collect")
        .to_bytes();
    let lag_json: serde_json::Value =
        serde_json::from_slice(&lag_body).expect("lag body should be valid json");
    assert_eq!(
        lag_json["data"]["items"].as_array().unwrap().len(),
        0,
        "ops lag should start empty until governance publishes real lag items"
    );
    assert_eq!(lag_json["data"]["pageInfo"]["mode"], "cursor");
    assert_eq!(lag_json["data"]["pageInfo"]["pageSize"], 20);
    assert_eq!(lag_json["data"]["pageInfo"]["hasMore"], false);

    for invalid_query in ["limit=1", "pageSize=1", "page_size=201"] {
        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri(format!("/backend/v3/api/ops/lag?{invalid_query}"))
                    .with_dual_token_tenant("100001")
                    .with_dual_token_organization("100001")
                    .with_dual_token_user("1")
                    .with_dual_token_actor_kind("user")
                    .with_dual_token_permission_scope("ops.read")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .expect("invalid pagination query should return a response");
        assert_eq!(
            response.status(),
            StatusCode::BAD_REQUEST,
            "forbidden or out-of-range pagination query must fail: {invalid_query}",
        );
    }

    let runtime_dir_response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/backend/v3/api/ops/runtime_dir")
                .with_dual_token_tenant("100001")
                .with_dual_token_organization("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("ops.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("ops runtime_dir inspection should succeed");
    assert_eq!(runtime_dir_response.status(), StatusCode::OK);
    let runtime_dir_body = runtime_dir_response
        .into_body()
        .collect()
        .await
        .expect("runtime_dir body should collect")
        .to_bytes();
    let runtime_dir_json: serde_json::Value =
        serde_json::from_slice(&runtime_dir_body).expect("runtime_dir body should be valid json");
    assert_eq!(runtime_dir_json["data"]["item"]["status"], "unmanaged");
    assert_eq!(
        runtime_dir_json["data"]["item"]["files"]
            .as_array()
            .unwrap()
            .len(),
        0
    );

    let provider_bindings_response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/backend/v3/api/ops/provider_bindings")
                .with_dual_token_tenant("100001")
                .with_dual_token_organization("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("ops.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("ops provider_bindings should succeed");
    assert_eq!(provider_bindings_response.status(), StatusCode::OK);
    let provider_bindings_body = provider_bindings_response
        .into_body()
        .collect()
        .await
        .expect("provider_bindings body should collect")
        .to_bytes();
    let provider_bindings_json: serde_json::Value = serde_json::from_slice(&provider_bindings_body)
        .expect("provider_bindings body should be valid json");
    assert_eq!(
        provider_bindings_json["data"]["items"]
            .as_array()
            .unwrap()
            .len(),
        0
    );
    assert_eq!(provider_bindings_json["data"]["pageInfo"]["mode"], "cursor");

    let provider_binding_drift_response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/backend/v3/api/ops/provider_bindings/drift")
                .with_dual_token_tenant("100001")
                .with_dual_token_organization("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("ops.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("ops provider_bindings drift should succeed");
    assert_eq!(provider_binding_drift_response.status(), StatusCode::OK);
    let provider_binding_drift_body = provider_binding_drift_response
        .into_body()
        .collect()
        .await
        .expect("provider_bindings drift body should collect")
        .to_bytes();
    let provider_binding_drift_json: serde_json::Value =
        serde_json::from_slice(&provider_binding_drift_body)
            .expect("provider_bindings drift body should be valid json");
    assert_eq!(
        provider_binding_drift_json["data"]["items"]
            .as_array()
            .unwrap()
            .len(),
        0
    );
    assert_eq!(
        provider_binding_drift_json["data"]["pageInfo"]["mode"],
        "cursor"
    );

    let diagnostics_response = app
        .oneshot(
            Request::builder()
                .uri("/backend/v3/api/ops/diagnostics")
                .with_dual_token_tenant("100001")
                .with_dual_token_organization("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("ops.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("ops diagnostics should succeed");
    assert_eq!(diagnostics_response.status(), StatusCode::OK);
    let diagnostics_body = diagnostics_response
        .into_body()
        .collect()
        .await
        .expect("diagnostics body should collect")
        .to_bytes();
    let diagnostics_json: serde_json::Value =
        serde_json::from_slice(&diagnostics_body).expect("diagnostics body should be valid json");
    let diagnostics = &diagnostics_json["data"]["item"];
    assert_eq!(diagnostics["profile"], "unconfigured");
    assert_eq!(diagnostics["clientRoutes"].as_array().unwrap().len(), 0);
    assert!(diagnostics.get("projectionPlane").is_none());
    assert_eq!(diagnostics["providerBindings"].as_array().unwrap().len(), 0);
    assert_eq!(
        diagnostics["providerBindingDrift"]["items"]
            .as_array()
            .unwrap()
            .len(),
        0
    );
    assert_eq!(
        diagnostics["sideEffectOutboxes"].as_array().unwrap().len(),
        0
    );
    assert_eq!(diagnostics["lag"].as_array().unwrap().len(), 0);
}
