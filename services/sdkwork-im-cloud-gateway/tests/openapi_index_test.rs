use axum::{
    Json, Router,
    body::Body,
    extract::State,
    http::{Method, Request, StatusCode},
    routing::any,
};
use http_body_util::BodyExt;
use sdkwork_im_cloud_gateway_config::{GatewayRuntimeMode, WebGatewayConfig, service_upstream};
use sdkwork_im_cloud_gateway_observability::{
    build_startup_summary_with_registry, format_startup_summary,
};
use sdkwork_im_runtime_link::LINK_WEBSOCKET_SUBPROTOCOL;
use serde_json::json;
use std::sync::Arc;
use tower::ServiceExt;

fn ensure_gateway_test_web_environment() {
    static INIT: std::sync::Once = std::sync::Once::new();
    INIT.call_once(|| {
        unsafe {
            std::env::set_var("SDKWORK_IM_ENVIRONMENT", "test");
            std::env::set_var("SDKWORK_ENV", "test");
        }
    });
}

#[derive(Clone)]
struct OpenApiUpstreamState {
    service_id: Arc<str>,
    openapi: serde_json::Value,
}

#[tokio::test]
async fn gateway_exposes_aggregate_openapi_json() {
    let control_plane = spawn_openapi_upstream(
        "governance-service",
        json!({
            "openapi": "3.1.0",
            "info": { "title": "Control Plane API", "version": "0.1.0" },
            "paths": {
                "/backend/v3/api/control/protocol-registry": {
                    "get": { "summary": "Get protocol registry", "responses": { "200": { "description": "ok" } } }
                }
            }
        }),
    )
    .await;
    let projection = spawn_openapi_upstream(
        "projection-service",
        json!({
            "openapi": "3.1.0",
            "info": { "title": "Sdkwork IM Projection Service API", "version": "0.1.0" },
            "paths": {
                "/im/v3/api/chat/conversations/{conversation_id}/messages": {
                    "get": { "summary": "Get messages", "responses": { "200": { "description": "ok" } } }
                }
            }
        }),
    )
    .await;
    let runtime = spawn_openapi_upstream(
        "comms-conversation-service",
        json!({
            "openapi": "3.1.0",
            "info": { "title": "Sdkwork IM Conversation Runtime API", "version": "0.1.0" },
            "paths": {
                "/im/v3/api/chat/conversations/{conversation_id}/messages": {
                    "post": { "summary": "Post message", "responses": { "200": { "description": "ok" } } }
                }
            }
        }),
    )
    .await;
    let app = web_gateway::build_app(test_gateway_config(vec![
        service_upstream("governance-service", control_plane.base_url.as_str()),
        service_upstream("projection-service", projection.base_url.as_str()),
        service_upstream("comms-conversation-service", runtime.base_url.as_str()),
    ]));

    let response = app
        .oneshot(
            Request::builder()
                .uri("/openapi.json")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("aggregate openapi request should succeed");

    assert_eq!(response.status(), StatusCode::OK);
    let value: serde_json::Value = serde_json::from_slice(
        &response
            .into_body()
            .collect()
            .await
            .expect("aggregate openapi body should collect")
            .to_bytes(),
    )
    .expect("aggregate openapi should be valid json");

    assert_eq!(value["openapi"], "3.1.0");
    assert_eq!(value["info"]["title"], "Sdkwork IM Unified Gateway API");
    assert!(value["paths"]["/openapi/runtime-summary.json"]["get"].is_object());
    assert_eq!(
        value["paths"]["/openapi/index.json"]["get"]["responses"]["200"]["content"]["application/json"]
            ["schema"]["$ref"],
        "#/components/schemas/GatewayOpenapiIndex"
    );
    assert_eq!(
        value["paths"]["/openapi/runtime-summary.json"]["get"]["responses"]["200"]["content"]["application/json"]
            ["schema"]["$ref"],
        "#/components/schemas/GatewayRuntimeSummary"
    );
    assert_eq!(
        value["components"]["schemas"]["GatewayOpenapiIndex"]["properties"]["services"]["items"]["$ref"],
        "#/components/schemas/GatewayServiceSchemaIndexEntry"
    );
    assert_eq!(
        value["components"]["schemas"]["GatewayOpenapiIndex"]["properties"]["sdkContracts"]["items"]
            ["$ref"],
        "#/components/schemas/GatewaySdkContractSummary"
    );
    assert_eq!(
        value["components"]["schemas"]["GatewayOpenapiIndex"]["properties"]["routes"]["items"]["$ref"],
        "#/components/schemas/GatewayRouteSummary"
    );
    assert_eq!(
        value["components"]["schemas"]["GatewayOpenapiIndex"]["properties"]["surfaceGroups"]["items"]
            ["$ref"],
        "#/components/schemas/GatewaySurfaceGroupSummary"
    );
    assert_eq!(
        value["components"]["schemas"]["GatewayRuntimeSummary"]["properties"]["surfaceGroups"]["items"]
            ["$ref"],
        "#/components/schemas/GatewaySurfaceGroupSummary"
    );
    assert_eq!(
        value["components"]["schemas"]["GatewayRuntimeSummary"]["properties"]["sdkContracts"]["items"]
            ["$ref"],
        "#/components/schemas/GatewaySdkContractSummary"
    );
    assert!(
        value["components"]["schemas"]["GatewayServiceSchemaIndexEntry"]["properties"]["contractKind"]
            .is_object()
    );
    assert!(
        value["components"]["schemas"]["GatewayServiceContractSummary"]["properties"]["contractKind"]
            .is_object()
    );
    assert!(value["paths"]["/backend/v3/api/control/protocol-registry"]["get"].is_object());
    assert!(
        value["paths"]["/im/v3/api/chat/conversations/{conversation_id}/messages"]["get"]
            .is_object()
    );
    assert!(
        value["paths"]["/im/v3/api/chat/conversations/{conversation_id}/messages"]["post"]
            .is_object()
    );
}

#[tokio::test]
async fn gateway_exposes_openapi_service_index_and_service_schema_proxy() {
    let control_plane = spawn_openapi_upstream(
        "governance-service",
        json!({
            "openapi": "3.1.0",
            "info": { "title": "Control Plane API", "version": "0.1.0" },
            "paths": {
                "/backend/v3/api/control/protocol-registry": {
                    "get": { "summary": "Get protocol registry", "responses": { "200": { "description": "ok" } } }
                }
            }
        }),
    )
    .await;
    let app = web_gateway::build_app(test_gateway_config(vec![service_upstream(
        "governance-service",
        control_plane.base_url.as_str(),
    )]));

    let index_response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/openapi/index.json")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("openapi index request should succeed");
    assert_eq!(index_response.status(), StatusCode::OK);
    let index_value: serde_json::Value = serde_json::from_slice(
        &index_response
            .into_body()
            .collect()
            .await
            .expect("openapi index body should collect")
            .to_bytes(),
    )
    .expect("openapi index should be valid json");
    assert_eq!(index_value["sdkContracts"][0]["groupId"], "im-open-api");
    assert_eq!(
        index_value["sdkContracts"][0]["schemaUrl"],
        "/im/v3/openapi.json"
    );
    assert_eq!(index_value["sdkContracts"][0]["apiPrefix"], "/im/v3/api");
    assert_eq!(index_value["sdkContracts"][0]["sdkTarget"], "sdkworkImSdk");
    assert_eq!(index_value["sdkContracts"][1]["groupId"], "im-app-api");
    assert_eq!(
        index_value["sdkContracts"][1]["schemaUrl"],
        "/app/v3/openapi.json"
    );
    assert_eq!(index_value["sdkContracts"][1]["apiPrefix"], "/app/v3/api");
    assert_eq!(
        index_value["sdkContracts"][1]["sdkTarget"],
        "sdkworkImAppSdk"
    );
    assert_eq!(index_value["sdkContracts"][2]["groupId"], "im-backend-api");
    assert_eq!(
        index_value["sdkContracts"][2]["schemaUrl"],
        "/backend/v3/openapi.json"
    );
    assert_eq!(
        index_value["sdkContracts"][2]["apiPrefix"],
        "/backend/v3/api"
    );
    assert_eq!(
        index_value["sdkContracts"][2]["sdkTarget"],
        "sdkworkImBackendSdk"
    );
    assert_eq!(
        index_value["services"][0]["serviceId"],
        "governance-service"
    );
    assert_eq!(
        index_value["services"][0]["contractKind"],
        "upstreamOperational"
    );
    assert_eq!(
        index_value["services"][0]["schemaUrl"],
        "/openapi/services/governance-service.openapi.json"
    );
    assert_eq!(
        index_value["services"][0]["docsUrl"],
        "/docs/services/governance-service"
    );
    assert_eq!(index_value["services"][0]["visibility"], "internal");
    assert_eq!(index_value["services"][0]["routeCount"], 1);
    assert_eq!(
        index_value["services"][0]["operationGroups"],
        json!(["control"])
    );
    assert_eq!(
        index_value["services"][0]["sdkTargets"],
        json!(["sdkworkImBackendSdk"])
    );
    assert_eq!(index_value["services"][0]["protocols"], json!(["http"]));
    assert!(
        index_value["routes"]
            .as_array()
            .expect("routes should be an array")
            .iter()
            .any(|route| {
                route["serviceId"] == "governance-service"
                    && route["operationGroup"] == "control"
                    && route["pathPattern"] == "/backend/v3/api/control/{*path}"
                    && route["methods"]
                        == json!(["delete", "get", "head", "options", "patch", "post", "put"])
                    && route["protocol"] == "http"
                    && route["sdkTargets"] == json!(["sdkworkImBackendSdk"])
            })
    );
    assert!(
        index_value["surfaceGroups"]
            .as_array()
            .expect("surface groups should be an array")
            .iter()
            .any(|group| {
                group["serviceId"] == "governance-service"
                    && group["operationGroup"] == "control"
                    && group["visibility"] == "internal"
                    && group["routeCount"] == 1
                    && group["sdkTargets"] == json!(["sdkworkImBackendSdk"])
                    && group["protocols"] == json!(["http"])
            })
    );

    let service_response = app
        .oneshot(
            Request::builder()
                .uri("/openapi/services/governance-service.openapi.json")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("service schema proxy request should succeed");
    assert_eq!(service_response.status(), StatusCode::OK);
    let service_value: serde_json::Value = serde_json::from_slice(
        &service_response
            .into_body()
            .collect()
            .await
            .expect("service schema body should collect")
            .to_bytes(),
    )
    .expect("service schema should be valid json");
    assert_eq!(service_value["info"]["title"], "Control Plane API");
}

#[tokio::test]
async fn gateway_service_index_surfaces_session_websocket_metadata() {
    let session_gateway = spawn_openapi_upstream(
        "session-gateway",
        json!({
            "openapi": "3.1.0",
            "info": { "title": "Sdkwork IM Session Gateway API", "version": "0.1.0" },
            "paths": {
                "/im/v3/api/presence/me": {
                    "get": { "summary": "Get current presence", "responses": { "200": { "description": "ok" } } }
                },
                "/im/v3/api/realtime/ws": {
                    "get": {
                        "summary": "Open realtime websocket session",
                        "responses": { "101": { "description": "websocket upgrade successful" } },
                        "x-sdkwork-im-protocol": "websocket",
                        "x-sdkwork-im-websocket-subprotocols": ["ccp"]
                    }
                }
            }
        }),
    )
    .await;
    let app = web_gateway::build_app(test_gateway_config(vec![service_upstream(
        "session-gateway",
        session_gateway.base_url.as_str(),
    )]));

    let index_response = app
        .oneshot(
            Request::builder()
                .uri("/openapi/index.json")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("openapi index request should succeed");
    assert_eq!(index_response.status(), StatusCode::OK);
    let index_value: serde_json::Value = serde_json::from_slice(
        &index_response
            .into_body()
            .collect()
            .await
            .expect("openapi index body should collect")
            .to_bytes(),
    )
    .expect("openapi index should be valid json");

    assert_eq!(index_value["services"][0]["serviceId"], "session-gateway");
    assert_eq!(index_value["services"][0]["visibility"], "public");
    assert_eq!(index_value["services"][0]["routeCount"], 3);
    assert_eq!(
        index_value["services"][0]["operationGroups"],
        json!(["presence", "realtime"])
    );
    assert_eq!(
        index_value["services"][0]["sdkTargets"],
        json!(["sdkworkImSdk"])
    );
    assert_eq!(
        index_value["services"][0]["protocols"],
        json!(["http", "websocket"])
    );
    assert!(
        index_value["routes"]
            .as_array()
            .expect("routes should be an array")
            .iter()
            .any(|route| {
                route["serviceId"] == "session-gateway"
                    && route["operationGroup"] == "realtime"
                    && route["pathPattern"] == "/im/v3/api/realtime/ws"
                    && route["protocol"] == "websocket"
                    && route["websocketSubprotocols"] == json!([LINK_WEBSOCKET_SUBPROTOCOL])
            })
    );
    assert!(
        index_value["surfaceGroups"]
            .as_array()
            .expect("surface groups should be an array")
            .iter()
            .any(|group| {
                group["serviceId"] == "session-gateway"
                    && group["operationGroup"] == "realtime"
                    && group["routeCount"] == 2
                    && group["protocols"] == json!(["http", "websocket"])
                    && group["websocketSubprotocols"] == json!([LINK_WEBSOCKET_SUBPROTOCOL])
            })
    );
    let websocket_subprotocols = index_value["services"][0]["websocketSubprotocols"]
        .as_array()
        .expect("websocket subprotocols should be an array");
    assert_eq!(websocket_subprotocols.len(), 1);
    assert!(
        websocket_subprotocols[0]
            .as_str()
            .is_some_and(|value| !value.is_empty()),
        "websocket subprotocol should be non-empty"
    );
}

#[tokio::test]
async fn gateway_service_index_does_not_surface_projection_device_metadata() {
    let projection = spawn_openapi_upstream(
        "projection-service",
        json!({
            "openapi": "3.1.0",
            "info": { "title": "Sdkwork IM Projection Service API", "version": "0.1.0" },
            "paths": {
                "/im/v3/api/chat/inbox": {
                    "get": { "summary": "Get inbox", "responses": { "200": { "description": "ok" } } }
                }
            }
        }),
    )
    .await;
    let app = web_gateway::build_app(test_gateway_config(vec![service_upstream(
        "projection-service",
        projection.base_url.as_str(),
    )]));

    let index_response = app
        .oneshot(
            Request::builder()
                .uri("/openapi/index.json")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("openapi index request should succeed");
    assert_eq!(index_response.status(), StatusCode::OK);
    let index_value: serde_json::Value = serde_json::from_slice(
        &index_response
            .into_body()
            .collect()
            .await
            .expect("openapi index body should collect")
            .to_bytes(),
    )
    .expect("openapi index should be valid json");

    assert_eq!(
        index_value["services"][0]["serviceId"],
        "projection-service"
    );
    assert_eq!(index_value["services"][0]["visibility"], "public");
    assert_eq!(index_value["services"][0]["routeCount"], 3);
    assert_eq!(
        index_value["services"][0]["operationGroups"],
        json!(["conversations"])
    );
    assert_eq!(
        index_value["services"][0]["sdkTargets"],
        json!(["sdkworkImSdk"])
    );
    assert_eq!(index_value["services"][0]["protocols"], json!(["http"]));
    let routes = index_value["routes"]
        .as_array()
        .expect("routes should be an array");
    assert!(
        !routes
            .iter()
            .any(|route| route["operationGroup"] == "devices"
                || route["pathPattern"] == "/im/v3/api/devices/register"
                || route["pathPattern"] == "/im/v3/api/devices/{device_id}/sync_feed"),
        "gateway index must not surface retired IM client route endpoints"
    );
    let surface_groups = index_value["surfaceGroups"]
        .as_array()
        .expect("surface groups should be an array");
    assert!(
        !surface_groups
            .iter()
            .any(|group| group["operationGroup"] == "devices"),
        "gateway surface groups must not contain the retired devices group"
    );
}

#[tokio::test]
async fn gateway_exposes_runtime_summary_json() {
    let control_plane = spawn_openapi_upstream(
        "governance-service",
        json!({
            "openapi": "3.1.0",
            "info": { "title": "Control Plane API", "version": "0.1.0" },
            "paths": {
                "/backend/v3/api/control/protocol-registry": {
                    "get": { "summary": "Get protocol registry", "responses": { "200": { "description": "ok" } } }
                }
            }
        }),
    )
    .await;
    let session_gateway = spawn_openapi_upstream(
        "session-gateway",
        json!({
            "openapi": "3.1.0",
            "info": { "title": "Sdkwork IM Session Gateway API", "version": "0.1.0" },
            "paths": {
                "/im/v3/api/presence/me": {
                    "get": { "summary": "Get current presence", "responses": { "200": { "description": "ok" } } }
                },
                "/im/v3/api/realtime/ws": {
                    "get": {
                        "summary": "Open realtime websocket session",
                        "responses": { "101": { "description": "websocket upgrade successful" } },
                        "x-sdkwork-im-protocol": "websocket",
                        "x-sdkwork-im-websocket-subprotocols": ["ccp"]
                    }
                }
            }
        }),
    )
    .await;
    let app = web_gateway::build_app(test_gateway_config(vec![
        service_upstream("governance-service", control_plane.base_url.as_str()),
        service_upstream("session-gateway", session_gateway.base_url.as_str()),
    ]));

    let response = app
        .oneshot(
            Request::builder()
                .uri("/openapi/runtime-summary.json")
                .header("host", "gateway.example:18079")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("runtime summary request should succeed");
    assert_eq!(response.status(), StatusCode::OK);
    let value: serde_json::Value = serde_json::from_slice(
        &response
            .into_body()
            .collect()
            .await
            .expect("runtime summary body should collect")
            .to_bytes(),
    )
    .expect("runtime summary should be valid json");

    assert_eq!(value["baseUrl"], "http://gateway.example:18079");
    assert_eq!(value["sdkContracts"][0]["groupId"], "im-open-api");
    assert_eq!(
        value["sdkContracts"][0]["schemaUrl"],
        "http://gateway.example:18079/im/v3/openapi.json"
    );
    assert_eq!(value["sdkContracts"][1]["groupId"], "im-app-api");
    assert_eq!(
        value["sdkContracts"][1]["schemaUrl"],
        "http://gateway.example:18079/app/v3/openapi.json"
    );
    assert_eq!(value["sdkContracts"][2]["groupId"], "im-backend-api");
    assert_eq!(
        value["sdkContracts"][2]["schemaUrl"],
        "http://gateway.example:18079/backend/v3/openapi.json"
    );
    assert_eq!(
        value["aggregateOpenapiUrl"],
        "http://gateway.example:18079/openapi.json"
    );
    assert_eq!(
        value["openapiIndexUrl"],
        "http://gateway.example:18079/openapi/index.json"
    );
    assert_eq!(
        value["runtimeSummaryUrl"],
        "http://gateway.example:18079/openapi/runtime-summary.json"
    );
    assert_eq!(
        value["serviceContracts"][0]["schemaUrl"],
        "http://gateway.example:18079/openapi/services/governance-service.openapi.json"
    );
    assert_eq!(
        value["serviceContracts"][0]["contractKind"],
        "upstreamOperational"
    );
    assert!(
        value["publicEndpoints"]
            .as_array()
            .expect("public endpoints should be an array")
            .iter()
            .any(|endpoint| {
                endpoint["pathPattern"] == "/im/v3/api/realtime/ws"
                    && endpoint["protocol"] == "websocket"
                    && endpoint["visibility"] == "public"
            })
    );
    assert!(
        value["surfaceGroups"]
            .as_array()
            .expect("surface groups should be an array")
            .iter()
            .any(|group| {
                group["serviceId"] == "session-gateway"
                    && group["operationGroup"] == "realtime"
                    && group["routeCount"] == 2
            })
    );
    assert!(
        value["surfaceGroups"]
            .as_array()
            .expect("surface groups should be an array")
            .iter()
            .any(|group| {
                group["serviceId"] == "governance-service"
                    && group["operationGroup"] == "control"
                    && group["visibility"] == "internal"
            })
    );
}

#[test]
fn startup_summary_lists_gateway_openapi_endpoints() {
    let registry =
        web_gateway::build_gateway_registry().expect("gateway route registry should build");
    let config = test_gateway_config(vec![service_upstream(
        "governance-service",
        "http://127.0.0.1:18081",
    )]);
    let summary = build_startup_summary_with_registry(&config, &registry, "http://127.0.0.1:18079");
    let text = format_startup_summary(&summary);

    assert!(text.contains("OpenAPI 3.1 Schemas"));
    assert!(text.contains("SDK Contracts"));
    let sdk_contracts_index = text
        .lines()
        .position(|line| line == "SDK Contracts")
        .expect("startup summary should include SDK Contracts section");
    let upstream_status_index = text
        .lines()
        .position(|line| line == "Upstream Status")
        .expect("startup summary should include Upstream Status section");
    assert!(
        sdk_contracts_index < upstream_status_index,
        "SDK contracts should be listed before upstream status"
    );
    assert!(
        text.contains("im-open-api schema: http://127.0.0.1:18079/im/v3/openapi.json [sdk:sdkworkImSdk] [prefix:/im/v3/api]")
    );
    assert!(
        text.contains("im-app-api schema: http://127.0.0.1:18079/app/v3/openapi.json [sdk:sdkworkImAppSdk] [prefix:/app/v3/api]")
    );
    assert!(
        text.contains("im-backend-api schema: http://127.0.0.1:18079/backend/v3/openapi.json [sdk:sdkworkImBackendSdk] [prefix:/backend/v3/api]")
    );
    assert!(text.contains("http://127.0.0.1:18079/openapi.json"));
    assert!(text.contains("http://127.0.0.1:18079/openapi/index.json"));
    assert!(text.contains("http://127.0.0.1:18079/openapi/runtime-summary.json"));
    assert!(text.contains("http://127.0.0.1:18079/"));
    assert!(text.contains("http://127.0.0.1:18079/admin/"));
    assert!(text.contains("Gateway Endpoints"));
    assert!(text.contains("/im/v3/api/realtime/ws"));
    assert!(text.contains("Gateway Surface Groups"));
    assert!(text.contains(
        "public session-gateway realtime [sdk:sdkworkImSdk] [protocols:http,websocket]: 2 routes"
    ));
    assert!(text.contains(
        "internal governance-service control [sdk:sdkworkImBackendSdk] [protocols:http]: 1 routes"
    ));
}

#[test]
fn startup_summary_hides_per_service_schema_and_docs_endpoints() {
    let registry =
        web_gateway::build_gateway_registry().expect("gateway route registry should build");
    let config = test_gateway_config(vec![
        service_upstream("governance-service", "http://127.0.0.1:18081"),
        service_upstream("comms-conversation-service", "http://127.0.0.1:18082"),
    ]);
    let summary = build_startup_summary_with_registry(&config, &registry, "http://127.0.0.1:18079");
    let text = format_startup_summary(&summary);

    assert!(!text.lines().any(|line| line == "Service Contracts"));
    assert!(!text.contains("Upstream Operational Service Contracts"));
    assert!(!text.contains("upstream schema:"));
    assert!(!text.contains("upstream docs:"));
    assert!(!text.contains("/openapi/services/governance-service.openapi.json"));
    assert!(!text.contains("/docs/services/governance-service"));
    assert!(!text.contains("/openapi/services/comms-conversation-service.openapi.json"));
    assert!(!text.contains("/docs/services/comms-conversation-service"));
}

fn test_gateway_config(
    upstreams: Vec<sdkwork_im_cloud_gateway_config::ServiceUpstreamConfig>,
) -> WebGatewayConfig {
    ensure_gateway_test_web_environment();
    WebGatewayConfig {
        bind_addr: "127.0.0.1:0".to_owned(),
        runtime_mode: GatewayRuntimeMode::Split,
        strict_startup: true,
        upstreams,
    }
}

struct TestOpenApiUpstream {
    base_url: String,
}

async fn spawn_openapi_upstream(
    service_id: &str,
    openapi: serde_json::Value,
) -> TestOpenApiUpstream {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("openapi upstream should bind local port");
    let local_addr = listener
        .local_addr()
        .expect("openapi upstream should expose local addr");
    let app = Router::new()
        .route("/", any(openapi_upstream))
        .route("/{*path}", any(openapi_upstream))
        .with_state(OpenApiUpstreamState {
            service_id: Arc::<str>::from(service_id),
            openapi,
        });

    tokio::spawn(async move {
        axum::serve(listener, app)
            .await
            .expect("openapi upstream server should run");
    });

    TestOpenApiUpstream {
        base_url: format!("http://{local_addr}"),
    }
}

async fn openapi_upstream(
    State(state): State<OpenApiUpstreamState>,
    method: Method,
    request: Request<Body>,
) -> Json<serde_json::Value> {
    if method == Method::GET && request.uri().path() == "/openapi.json" {
        return Json(state.openapi);
    }

    Json(json!({
        "serviceId": state.service_id.as_ref(),
        "path": request.uri().path()
    }))
}
