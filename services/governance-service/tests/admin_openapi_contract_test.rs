use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use tower::ServiceExt;

#[tokio::test]
async fn admin_openapi_contract_exposes_live_openapi_document_without_auth() {
    let app = governance_service::build_public_app();

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/openapi.json")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("openapi request should return a response");

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        response
            .headers()
            .get("content-type")
            .and_then(|value| value.to_str().ok()),
        Some("application/json")
    );

    let body = response
        .into_body()
        .collect()
        .await
        .expect("openapi body should collect")
        .to_bytes();
    let json: serde_json::Value =
        serde_json::from_slice(&body).expect("openapi body should be valid json");

    assert_eq!(json["openapi"], "3.1.2");
    assert_eq!(json["info"]["title"], "Control Plane API");
    assert_eq!(json["info"]["version"], env!("CARGO_PKG_VERSION"));

    let paths = json["paths"]
        .as_object()
        .expect("openapi document should expose paths");
    assert!(
        paths.contains_key("/backend/v3/api/control/protocol_registry"),
        "protocol registry path should be documented"
    );
    assert!(
        paths.contains_key("/backend/v3/api/control/protocol_governance"),
        "protocol governance path should be documented"
    );
    assert!(
        paths.contains_key("/backend/v3/api/control/provider_registry"),
        "provider registry path should be documented"
    );
    assert!(
        paths.contains_key("/backend/v3/api/control/nodes/{node_id}/drain"),
        "node control route group should be documented"
    );

    let tags = json["tags"]
        .as_array()
        .expect("openapi document should expose tags");
    let tag_names = tags
        .iter()
        .filter_map(|tag| tag["name"].as_str())
        .collect::<Vec<_>>();
    assert!(tag_names.contains(&"protocol"));
    assert!(tag_names.contains(&"providers"));
    assert!(tag_names.contains(&"nodes"));
}
