use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use im_app_context::DualTokenRequestBuilderExt;
use tower::ServiceExt;

#[tokio::test]
async fn test_control_plane_exposes_protocol_registry_snapshot_to_control_readers() {
    let app = governance_service::build_app();

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/backend/v3/api/control/protocol_registry")
                .with_dual_token_tenant("100001")
                .with_dual_token_organization("100001")
                .with_dual_token_user("1080")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("control.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("protocol registry request should return a response");

    assert_eq!(response.status(), StatusCode::OK);

    let body = response
        .into_body()
        .collect()
        .await
        .expect("protocol registry body should collect")
        .to_bytes();
    let json: serde_json::Value =
        serde_json::from_slice(&body).expect("protocol registry body should be valid json");

    assert_eq!(json["data"]["protocolVersion"], "ccp/1.0");

    let schemas = json["data"]["schemas"]
        .as_array()
        .expect("schemas should be returned as an array");
    let hello = schemas
        .iter()
        .find(|schema| schema["schema"] == "ccp.control.hello")
        .expect("hello schema should be present");
    assert_eq!(hello["stage"], "stable");

    let matrix = json["data"]["compatibilityMatrix"]
        .as_array()
        .expect("compatibility matrix should be returned as an array");
    let web = matrix
        .iter()
        .find(|entry| entry["clientType"] == "web")
        .expect("web compatibility entry should be present");
    assert_eq!(web["minimumProtocolVersion"], "ccp/1.0");
}
