use axum::{
    body::Body,
    http::{Request, StatusCode, header},
};
use http_body_util::BodyExt;
use sdkwork_im_cloud_gateway_config::{GatewayRuntimeMode, WebGatewayConfig};
use serde_json::{Value, json};
use tower::ServiceExt;

fn test_gateway_config() -> WebGatewayConfig {
    static INIT: std::sync::Once = std::sync::Once::new();
    INIT.call_once(|| unsafe {
        std::env::set_var("SDKWORK_IM_ENVIRONMENT", "test");
        std::env::set_var("SDKWORK_ENV", "test");
    });
    WebGatewayConfig {
        bind_addr: "127.0.0.1:0".to_owned(),
        runtime_mode: GatewayRuntimeMode::SingleIngress,
        strict_startup: true,
        upstreams: Vec::new(),
    }
}

async fn get_json(app: axum::Router, path: &str) -> Value {
    let response = app
        .oneshot(
            Request::builder()
                .uri(path)
                .body(Body::empty())
                .expect("SDK contract schema request should build"),
        )
        .await
        .expect("SDK contract schema request should complete");

    assert_eq!(response.status(), StatusCode::OK, "{path} should be public");
    assert_eq!(
        response.headers().get(header::CONTENT_TYPE),
        Some(&header::HeaderValue::from_static("application/json")),
        "{path} should serve JSON"
    );
    serde_json::from_slice(
        &response
            .into_body()
            .collect()
            .await
            .expect("SDK contract schema response should collect")
            .to_bytes(),
    )
    .expect("SDK contract schema response should be valid JSON")
}

#[tokio::test]
async fn gateway_serves_advertised_sdk_contract_documents_without_widening_public_methods() {
    let app = web_gateway::build_app(test_gateway_config());

    let im_document = get_json(app.clone(), "/im/v3/openapi.json").await;
    let app_document = get_json(app.clone(), "/app/v3/openapi.json").await;
    let backend_document = get_json(app.clone(), "/backend/v3/openapi.json").await;

    for document in [&im_document, &app_document, &backend_document] {
        assert!(
            document["openapi"]
                .as_str()
                .is_some_and(|version| version.starts_with("3.")),
            "SDK contract should expose an OpenAPI 3 document"
        );
    }
    assert_eq!(
        im_document["components"]["schemas"]["CreateConversationRequest"]["properties"]["initializeKnowledgebase"]
            ["type"],
        "boolean"
    );
    assert_eq!(
        im_document["components"]["schemas"]["CreateConversationRequest"]["properties"]["initializeKnowledgebase"]
            ["default"],
        false
    );
    assert_eq!(
        im_document["components"]["schemas"]["CreateConversationResult"]["properties"]["knowledgebaseInitialization"]
            ["enum"],
        json!(["active", "provisioning", "failed"])
    );

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/openapi.json")
                .body(Body::empty())
                .expect("non-GET schema request should build"),
        )
        .await
        .expect("non-GET schema request should complete");
    assert_eq!(
        response.status(),
        StatusCode::UNAUTHORIZED,
        "only the exact GET SDK schema route is public"
    );
}
