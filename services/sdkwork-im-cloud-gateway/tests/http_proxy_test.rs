use axum::{
    Json, Router,
    body::Body,
    extract::State,
    http::{HeaderMap, Method, Request, StatusCode, header},
    response::{IntoResponse, Response},
    routing::{any, get},
};
use http_body_util::BodyExt;
use im_app_context::{
    AppContext, build_dual_token_headers_for_context, local_service_app_context,
    resolve_app_context,
};
use sdkwork_im_api_registry::HttpMethod;
use sdkwork_im_cloud_gateway_config::{GatewayRuntimeMode, WebGatewayConfig, service_upstream};
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
struct UpstreamState {
    service_id: Arc<str>,
}

fn gateway_test_app_context() -> AppContext {
    let mut context = local_service_app_context(
        "100001",
        "30",
        "user",
        Some("device_real"),
        ["*"],
    );
    context.session_id = Some("session_real".to_owned());
    context.app_id = Some("sdkwork-im-pc".to_owned());
    context
}

fn gateway_test_auth_headers() -> HeaderMap {
    let context = gateway_test_app_context();
    build_dual_token_headers_for_context(&context, context.permission_scope.iter())
}

fn gateway_test_authorization_header() -> String {
    gateway_test_auth_headers()
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .expect("test auth token should be present")
        .to_owned()
}

fn gateway_test_access_token_header() -> String {
    gateway_test_auth_headers()
        .get("access-token")
        .and_then(|value| value.to_str().ok())
        .expect("test access token should be present")
        .to_owned()
}

fn gateway_numeric_auth_headers() -> HeaderMap {
    let mut context = local_service_app_context(
        "100001",
        "user_numeric",
        "user",
        Some("device_numeric"),
        ["*"],
    );
    context.organization_id = "30001".to_owned();
    context.session_id = Some("session_numeric".to_owned());
    context.app_id = Some("sdkwork-im-pc".to_owned());
    build_dual_token_headers_for_context(&context, context.permission_scope.iter())
}

fn gateway_numeric_authorization_header() -> String {
    gateway_numeric_auth_headers()
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .expect("numeric test auth token should be present")
        .to_owned()
}

fn gateway_numeric_access_token_header() -> String {
    gateway_numeric_auth_headers()
        .get("access-token")
        .and_then(|value| value.to_str().ok())
        .expect("numeric test access token should be present")
        .to_owned()
}

#[tokio::test]
async fn gateway_exposes_health_and_readiness_endpoints() {
    let app = web_gateway::build_app(test_gateway_config(Vec::new()));

    let healthz = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/healthz")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("healthz request should succeed");
    assert_eq!(healthz.status(), StatusCode::OK);

    let readyz = app
        .oneshot(
            Request::builder()
                .uri("/readyz")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("readyz request should succeed");
    assert_eq!(readyz.status(), StatusCode::OK);
}

#[tokio::test]
async fn gateway_routes_control_requests_to_control_plane_api() {
    let control_plane = spawn_upstream("governance-service").await;
    let app = web_gateway::build_app(test_gateway_config(vec![service_upstream(
        "governance-service",
        control_plane.base_url.as_str(),
    )]));
    let auth_headers = gateway_numeric_auth_headers();

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::GET)
                .uri("/backend/v3/api/control/protocol-registry")
                .header(
                    header::AUTHORIZATION,
                    auth_headers
                        .get(header::AUTHORIZATION)
                        .expect("auth header"),
                )
                .header(
                    "Access-Token",
                    auth_headers
                        .get("access-token")
                        .expect("access token header"),
                )
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("gateway request should succeed");

    assert_eq!(response.status(), StatusCode::OK);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("response body should collect")
        .to_bytes();
    let value: serde_json::Value =
        serde_json::from_slice(&body).expect("response body should be valid json");
    assert_eq!(value["serviceId"], "governance-service");
}

#[tokio::test]
async fn gateway_routes_conversation_reads_and_writes_to_different_upstreams() {
    let appbase = spawn_app_upstream(Router::new().route(
        "/app/v3/api/auth/sessions/current",
        get(appbase_current_session),
    ))
    .await;
    let projection = spawn_upstream("projection-service").await;
    let conversation_runtime = spawn_upstream("comms-conversation-service").await;
    let app = web_gateway::build_app(test_gateway_config(vec![
        service_upstream("sdkwork-iam-app-api", appbase.base_url.as_str()),
        service_upstream("projection-service", projection.base_url.as_str()),
        service_upstream(
            "comms-conversation-service",
            conversation_runtime.base_url.as_str(),
        ),
    ]));

    let read_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method(Method::GET)
                .uri("/im/v3/api/chat/conversations/c_1/messages")
                .header(header::AUTHORIZATION, gateway_test_authorization_header())
                .header("access-token", gateway_test_access_token_header())
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("read request should succeed");
    assert_eq!(read_response.status(), StatusCode::OK);
    let read_value: serde_json::Value = serde_json::from_slice(
        &read_response
            .into_body()
            .collect()
            .await
            .expect("read response body should collect")
            .to_bytes(),
    )
    .expect("read response should be valid json");
    assert_eq!(read_value["serviceId"], "projection-service");

    let write_response = app
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/im/v3/api/chat/conversations/c_1/messages")
                .header(header::AUTHORIZATION, gateway_test_authorization_header())
                .header("access-token", gateway_test_access_token_header())
                .body(Body::from("{}"))
                .unwrap(),
        )
        .await
        .expect("write request should succeed");
    assert_eq!(write_response.status(), StatusCode::OK);
    let write_value: serde_json::Value = serde_json::from_slice(
        &write_response
            .into_body()
            .collect()
            .await
            .expect("write response body should collect")
            .to_bytes(),
    )
    .expect("write response should be valid json");
    assert_eq!(write_value["serviceId"], "comms-conversation-service");
}

#[tokio::test]
async fn gateway_routes_im_app_iam_requests_to_appbase_app_api() {
    let appbase = spawn_upstream("sdkwork-iam-app-api").await;
    let app = web_gateway::build_app(test_gateway_config(vec![service_upstream(
        "sdkwork-iam-app-api",
        appbase.base_url.as_str(),
    )]));

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/app/v3/api/auth/sessions")
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from("{}"))
                .unwrap(),
        )
        .await
        .expect("gateway request should succeed");

    assert_eq!(response.status(), StatusCode::OK);
    let value: serde_json::Value = serde_json::from_slice(
        &response
            .into_body()
            .collect()
            .await
            .expect("response body should collect")
            .to_bytes(),
    )
    .expect("response body should be valid json");
    assert_eq!(value["serviceId"], "sdkwork-iam-app-api");
    assert_eq!(value["path"], "/app/v3/api/auth/sessions");
}

#[tokio::test]
async fn gateway_derives_proxied_im_http_context_from_appbase_dual_tokens_not_client_headers() {
    let appbase = spawn_app_upstream(Router::new().route(
        "/app/v3/api/auth/sessions/current",
        get(appbase_current_session),
    ))
    .await;
    let session_gateway = spawn_app_upstream(
        Router::new().route("/im/v3/api/realtime/events", get(echo_context_upstream)),
    )
    .await;
    let app = web_gateway::build_app(test_gateway_config(vec![
        service_upstream("session-gateway", session_gateway.base_url.as_str()),
        service_upstream("sdkwork-iam-app-api", appbase.base_url.as_str()),
    ]));

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::GET)
                .uri("/im/v3/api/realtime/events?afterSeq=0&limit=1")
                .header(header::AUTHORIZATION, gateway_test_authorization_header())
                .header("access-token", gateway_test_access_token_header())
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("gateway request should succeed");

    assert_eq!(response.status(), StatusCode::OK);
    let context = read_json_body(response).await;
    assert_eq!(context["tenantId"], "100001");
    assert_eq!(context["userId"], "30");
    assert_eq!(context["sessionId"], "session_real");
    assert_eq!(context["sdkworkInternalHeadersForwarded"], false);
}

#[tokio::test]
async fn gateway_drops_sdkwork_internal_headers_when_signature_secret_is_configured() {
    let _signature_secret = ScopedEnvVar::set(
        "SDKWORK_IM_APP_CONTEXT_SIGNATURE_SECRET",
        "gateway-signing-secret",
    );
    let appbase = spawn_app_upstream(Router::new().route(
        "/app/v3/api/auth/sessions/current",
        get(appbase_current_session),
    ))
    .await;
    let session_gateway = spawn_app_upstream(Router::new().route(
        "/im/v3/api/realtime/events",
        get(echo_context_upstream),
    ))
    .await;
    let app = web_gateway::build_app(test_gateway_config(vec![
        service_upstream("session-gateway", session_gateway.base_url.as_str()),
        service_upstream("sdkwork-iam-app-api", appbase.base_url.as_str()),
    ]));

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::GET)
                .uri("/im/v3/api/realtime/events?afterSeq=0&limit=1")
                .header(header::AUTHORIZATION, gateway_test_authorization_header())
                .header("access-token", gateway_test_access_token_header())
                .header("x-sdkwork-tenant-id", "100001")
                .header("x-sdkwork-user-id", "user_test006_a_com")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("gateway request should succeed");

    assert_eq!(response.status(), StatusCode::OK);
    let context = read_json_body(response).await;
    assert_eq!(context["tenantId"], "100001");
    assert_eq!(context["userId"], "30");
    assert_eq!(context["sessionId"], "session_real");
    assert_eq!(context["sdkworkInternalHeadersForwarded"], false);
}

#[tokio::test]
async fn gateway_accepts_numeric_appbase_session_context_ids_for_proxied_im_routes() {
    let appbase = spawn_app_upstream(Router::new().route(
        "/app/v3/api/auth/sessions/current",
        get(appbase_numeric_current_session),
    ))
    .await;
    let session_gateway = spawn_app_upstream(
        Router::new().route("/im/v3/api/realtime/events", get(echo_context_upstream)),
    )
    .await;
    let app = web_gateway::build_app(test_gateway_config(vec![
        service_upstream("session-gateway", session_gateway.base_url.as_str()),
        service_upstream("sdkwork-iam-app-api", appbase.base_url.as_str()),
    ]));

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::GET)
                .uri("/im/v3/api/realtime/events?afterSeq=0&limit=1")
                .header(
                    header::AUTHORIZATION,
                    gateway_numeric_authorization_header(),
                )
                .header("access-token", gateway_numeric_access_token_header())
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("gateway request should succeed");

    assert_eq!(response.status(), StatusCode::OK);
    let context = read_json_body(response).await;
    assert_eq!(context["tenantId"], "100001");
    assert_eq!(context["organizationId"], "30001");
    assert_eq!(context["userId"], "user_numeric");
    assert_eq!(context["sdkworkInternalHeadersForwarded"], false);
}

#[tokio::test]
async fn gateway_derives_proxied_im_calls_context_from_appbase_dual_tokens_not_client_headers() {
    let appbase = spawn_app_upstream(Router::new().route(
        "/app/v3/api/auth/sessions/current",
        get(appbase_current_session),
    ))
    .await;
    let rtc = spawn_app_upstream(
        Router::new()
            .route(
                "/im/v3/api/calls/sessions/rtc_demo/signals",
                any(echo_context_upstream),
            )
            .route(
                "/im/v3/api/calls/sessions/rtc_demo/credentials",
                any(echo_context_upstream),
            ),
    )
    .await;
    let app = web_gateway::build_app(test_gateway_config(vec![
        service_upstream("im-calls-service", rtc.base_url.as_str()),
        service_upstream("sdkwork-iam-app-api", appbase.base_url.as_str()),
    ]));

    for (method, path) in [
        (Method::GET, "/im/v3/api/calls/sessions/rtc_demo/signals"),
        (
            Method::POST,
            "/im/v3/api/calls/sessions/rtc_demo/credentials",
        ),
    ] {
        let body = if method == Method::GET {
            Body::empty()
        } else {
            Body::from(json!({ "participantId": "30" }).to_string())
        };
        let is_get = method == Method::GET;
        let mut request_builder = Request::builder()
            .method(method)
            .uri(path)
            .header(header::AUTHORIZATION, gateway_test_authorization_header())
            .header("access-token", gateway_test_access_token_header());
        if !is_get {
            request_builder = request_builder.header(header::CONTENT_TYPE, "application/json");
        }
        let response = app
            .clone()
            .oneshot(request_builder.body(body).unwrap())
            .await
            .unwrap_or_else(|error| {
                panic!("gateway IM calls request should succeed for {path}: {error}")
            });

        assert_eq!(
            response.status(),
            StatusCode::OK,
            "gateway IM calls request should succeed for {path}"
        );
        let context = read_json_body(response).await;
        assert_eq!(context["tenantId"], "100001", "{path} tenant context");
        assert_eq!(context["userId"], "30", "{path} user context");
        assert_eq!(
            context["sessionId"], "session_real",
            "{path} session context"
        );
        assert_eq!(
            context["sdkworkInternalHeadersForwarded"], false,
            "{path} must not receive client-supplied SDKWork SDKWork internal headers"
        );
        assert_ne!(context["tenantId"], "100001");
        assert_ne!(context["userId"], "user_test006_a_com");
    }
}

#[tokio::test]
async fn gateway_derives_proxied_chat_data_context_from_appbase_dual_tokens_not_client_headers() {
    for (service_id, method, path, route) in [
        (
            "comms-conversation-service",
            Method::POST,
            "/im/v3/api/chat/conversations/c_demo/messages",
            "/im/v3/api/chat/conversations/{id}/messages",
        ),
        (
            "comms-conversation-service",
            Method::POST,
            "/im/v3/api/chat/rooms",
            "/im/v3/api/chat/rooms",
        ),
        (
            "comms-conversation-service",
            Method::GET,
            "/im/v3/api/chat/rooms/r_demo",
            "/im/v3/api/chat/rooms/{roomId}",
        ),
        (
            "comms-conversation-service",
            Method::POST,
            "/im/v3/api/chat/rooms/r_demo/enter",
            "/im/v3/api/chat/rooms/{roomId}/enter",
        ),
        (
            "projection-service",
            Method::GET,
            "/im/v3/api/chat/conversations/c_demo/messages",
            "/im/v3/api/chat/conversations/{id}/messages",
        ),
        (
            "streaming-service",
            Method::POST,
            "/im/v3/api/streams",
            "/im/v3/api/streams",
        ),
        (
            "media-service",
            Method::POST,
            "/im/v3/api/media/uploads",
            "/im/v3/api/media/uploads",
        ),
        (
            "notification-service",
            Method::GET,
            "/app/v3/api/notifications",
            "/app/v3/api/notifications",
        ),
        (
            "automation-service",
            Method::POST,
            "/app/v3/api/automation/jobs",
            "/app/v3/api/automation/jobs",
        ),
        (
            "sdkwork-drive-app-api",
            Method::POST,
            "/app/v3/api/drive/uploader/uploads",
            "/app/v3/api/drive/uploader/uploads",
        ),
        (
            "sdkwork-catalog-app-api",
            Method::GET,
            "/app/v3/api/catalog/products",
            "/app/v3/api/catalog/products",
        ),
        (
            "sdkwork-order-app-api",
            Method::GET,
            "/app/v3/api/orders",
            "/app/v3/api/orders",
        ),
        (
            "sdkwork-mail-app-api",
            Method::GET,
            "/app/v3/api/mail/accounts",
            "/app/v3/api/mail/{*path}",
        ),
        (
            "sdkwork-community-app-api",
            Method::GET,
            "/app/v3/api/community/categories",
            "/app/v3/api/community/{*path}",
        ),
        (
            "sdkwork-course-app-api",
            Method::GET,
            "/app/v3/api/courses",
            "/app/v3/api/courses",
        ),
        (
            "sdkwork-knowledgebase-app-api",
            Method::GET,
            "/app/v3/api/knowledge/spaces",
            "/app/v3/api/knowledge/{*path}",
        ),
    ] {
        assert_gateway_derives_context_for_configured_upstream(
            service_id,
            method.clone(),
            path,
            route,
        )
        .await;
    }
}

#[tokio::test]
async fn gateway_derives_context_for_protected_routes_without_appbase_session_lookup() {
    for (service_id, method, path, route) in [
        (
            "session-gateway",
            Method::GET,
            "/im/v3/api/realtime/events?afterSeq=0&limit=1",
            "/im/v3/api/realtime/events",
        ),
        (
            "comms-conversation-service",
            Method::POST,
            "/im/v3/api/chat/conversations/c_demo/messages",
            "/im/v3/api/chat/conversations/{id}/messages",
        ),
        (
            "comms-conversation-service",
            Method::POST,
            "/im/v3/api/chat/rooms",
            "/im/v3/api/chat/rooms",
        ),
        (
            "comms-conversation-service",
            Method::GET,
            "/im/v3/api/chat/rooms/r_demo",
            "/im/v3/api/chat/rooms/{roomId}",
        ),
        (
            "comms-conversation-service",
            Method::POST,
            "/im/v3/api/chat/rooms/r_demo/leave",
            "/im/v3/api/chat/rooms/{roomId}/leave",
        ),
        (
            "projection-service",
            Method::GET,
            "/im/v3/api/chat/conversations/c_demo/messages",
            "/im/v3/api/chat/conversations/{id}/messages",
        ),
        (
            "streaming-service",
            Method::POST,
            "/im/v3/api/streams",
            "/im/v3/api/streams",
        ),
        (
            "media-service",
            Method::POST,
            "/im/v3/api/media/uploads",
            "/im/v3/api/media/uploads",
        ),
        (
            "im-calls-service",
            Method::GET,
            "/im/v3/api/calls/sessions/rtc_demo/signals",
            "/im/v3/api/calls/sessions/{id}/signals",
        ),
        (
            "notification-service",
            Method::GET,
            "/app/v3/api/notifications",
            "/app/v3/api/notifications",
        ),
        (
            "automation-service",
            Method::POST,
            "/app/v3/api/automation/jobs",
            "/app/v3/api/automation/jobs",
        ),
        (
            "sdkwork-drive-app-api",
            Method::POST,
            "/app/v3/api/drive/uploader/uploads",
            "/app/v3/api/drive/uploader/uploads",
        ),
        (
            "sdkwork-catalog-app-api",
            Method::GET,
            "/app/v3/api/catalog/products",
            "/app/v3/api/catalog/products",
        ),
        (
            "sdkwork-order-app-api",
            Method::GET,
            "/app/v3/api/orders",
            "/app/v3/api/orders",
        ),
        (
            "sdkwork-mail-app-api",
            Method::GET,
            "/app/v3/api/mail/accounts",
            "/app/v3/api/mail/{*path}",
        ),
        (
            "sdkwork-community-app-api",
            Method::GET,
            "/app/v3/api/community/categories",
            "/app/v3/api/community/{*path}",
        ),
        (
            "sdkwork-course-app-api",
            Method::GET,
            "/app/v3/api/courses",
            "/app/v3/api/courses",
        ),
        (
            "sdkwork-knowledgebase-app-api",
            Method::GET,
            "/app/v3/api/knowledge/spaces",
            "/app/v3/api/knowledge/{*path}",
        ),
    ] {
        assert_gateway_derives_context_without_appbase_session_lookup(
            service_id,
            method.clone(),
            path,
            route,
        )
        .await;
    }
}

#[test]
fn gateway_registry_resolves_course_collection_paths() {
    let registry = web_gateway::build_gateway_registry().expect("registry should build");
    let orders = registry.resolve(HttpMethod::Get, "/app/v3/api/orders");
    let courses = registry.resolve(HttpMethod::Get, "/app/v3/api/courses");
    assert!(orders.is_some(), "orders route should resolve");
    assert!(
        courses.is_some(),
        "courses route should resolve for collection list paths"
    );
    assert_eq!(courses.expect("courses route").service_id, "sdkwork-course-app-api");
}

#[tokio::test]
async fn gateway_handles_browser_cors_preflight_for_im_app_iam_routes() {
    let appbase = spawn_upstream("sdkwork-iam-app-api").await;
    let app = web_gateway::build_app(test_gateway_config(vec![service_upstream(
        "sdkwork-iam-app-api",
        appbase.base_url.as_str(),
    )]));
    let origin = "http://127.0.0.1:1620";

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::OPTIONS)
                .uri("/app/v3/api/oauth/device_authorizations")
                .header("origin", origin)
                .header("access-control-request-method", "POST")
                .header(
                    "access-control-request-headers",
                    "authorization,content-type,access-token",
                )
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("gateway CORS preflight should succeed");

    assert!(matches!(
        response.status(),
        StatusCode::OK | StatusCode::NO_CONTENT
    ));
    assert_eq!(
        response
            .headers()
            .get("access-control-allow-origin")
            .and_then(|value| value.to_str().ok()),
        Some(origin)
    );
    assert!(
        response
            .headers()
            .get("x-sdkwork-im-upstream-service")
            .is_none(),
        "gateway should answer browser preflight itself instead of proxying it to appbase"
    );

    let allow_methods = response
        .headers()
        .get("access-control-allow-methods")
        .and_then(|value| value.to_str().ok())
        .unwrap_or_default()
        .to_ascii_uppercase();
    assert!(allow_methods.contains("POST"));
    assert!(allow_methods.contains("OPTIONS"));

    let allow_headers = response
        .headers()
        .get("access-control-allow-headers")
        .and_then(|value| value.to_str().ok())
        .unwrap_or_default()
        .to_ascii_lowercase();
    for expected in ["authorization", "content-type", "access-token"] {
        assert!(
            allow_headers.contains(expected),
            "gateway CORS preflight must allow {expected}, got {allow_headers}"
        );
    }
}

#[tokio::test]
async fn gateway_adds_browser_cors_headers_to_im_app_iam_responses() {
    let appbase = spawn_upstream("sdkwork-iam-app-api").await;
    let app = web_gateway::build_app(test_gateway_config(vec![service_upstream(
        "sdkwork-iam-app-api",
        appbase.base_url.as_str(),
    )]));
    let origin = "http://127.0.0.1:1620";

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/app/v3/api/oauth/device_authorizations")
                .header("origin", origin)
                .header("content-type", "application/json")
                .body(Body::from("{}"))
                .unwrap(),
        )
        .await
        .expect("gateway request should succeed");

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        response
            .headers()
            .get("access-control-allow-origin")
            .and_then(|value| value.to_str().ok()),
        Some(origin)
    );
    assert_eq!(
        response
            .headers()
            .get("x-sdkwork-im-upstream-service")
            .and_then(|value| value.to_str().ok()),
        Some("sdkwork-iam-app-api")
    );
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

async fn read_json_body(response: axum::response::Response) -> serde_json::Value {
    serde_json::from_slice(
        &response
            .into_body()
            .collect()
            .await
            .expect("response body should collect")
            .to_bytes(),
    )
    .expect("response body should be valid json")
}

struct TestUpstream {
    base_url: String,
}

struct ScopedEnvVar {
    name: &'static str,
    previous: Option<String>,
}

impl ScopedEnvVar {
    fn set(name: &'static str, value: &str) -> Self {
        let previous = std::env::var(name).ok();
        unsafe {
            std::env::set_var(name, value);
        }
        Self { name, previous }
    }
}

impl Drop for ScopedEnvVar {
    fn drop(&mut self) {
        if let Some(previous) = &self.previous {
            unsafe {
                std::env::set_var(self.name, previous);
            }
            return;
        }

        unsafe {
            std::env::remove_var(self.name);
        }
    }
}

async fn spawn_upstream(service_id: &str) -> TestUpstream {
    spawn_app_upstream(
        Router::new()
            .route("/", any(echo_upstream))
            .route("/{*path}", any(echo_upstream))
            .with_state(UpstreamState {
                service_id: Arc::<str>::from(service_id),
            }),
    )
    .await
}

async fn spawn_app_upstream(app: Router) -> TestUpstream {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("test upstream should bind local port");
    let local_addr = listener
        .local_addr()
        .expect("test upstream should expose local addr");

    tokio::spawn(async move {
        axum::serve(listener, app)
            .await
            .expect("test upstream server should run");
    });

    TestUpstream {
        base_url: format!("http://{local_addr}"),
    }
}

async fn assert_gateway_derives_context_for_configured_upstream(
    service_id: &str,
    method: Method,
    path: &str,
    upstream_route: &str,
) {
    let appbase = spawn_app_upstream(Router::new().route(
        "/app/v3/api/auth/sessions/current",
        get(appbase_current_session),
    ))
    .await;
    let upstream =
        spawn_app_upstream(Router::new().route(upstream_route, any(echo_context_upstream))).await;
    let app = web_gateway::build_app(test_gateway_config(vec![
        service_upstream(service_id, upstream.base_url.as_str()),
        service_upstream("sdkwork-iam-app-api", appbase.base_url.as_str()),
    ]));
    let body = if method == Method::GET {
        Body::empty()
    } else {
        Body::from("{}")
    };
    let is_get = method == Method::GET;
    let mut request_builder = Request::builder()
        .method(method)
        .uri(path)
        .header(header::AUTHORIZATION, gateway_test_authorization_header())
        .header("access-token", gateway_test_access_token_header());
    if !is_get {
        request_builder = request_builder.header(header::CONTENT_TYPE, "application/json");
    }
    let response = app
        .oneshot(request_builder.body(body).unwrap())
        .await
        .unwrap_or_else(|error| panic!("gateway request should succeed for {service_id}: {error}"));

    assert_eq!(
        response.status(),
        StatusCode::OK,
        "gateway request should succeed for {service_id}"
    );
    let context = read_json_body(response).await;
    assert_eq!(
        context["tenantId"], "100001",
        "{service_id} must receive dual-token tenant context"
    );
    assert_eq!(
        context["userId"], "30",
        "{service_id} must receive dual-token user context"
    );
    assert_eq!(
        context["sessionId"], "session_real",
        "{service_id} must receive dual-token session context"
    );
    assert_eq!(
        context["sdkworkInternalHeadersForwarded"], false,
        "{service_id} must not receive client-supplied SDKWork SDKWork internal headers"
    );
    assert_ne!(context["tenantId"], "100001");
    assert_ne!(context["userId"], "user_test006_a_com");
}

async fn assert_gateway_derives_context_without_appbase_session_lookup(
    service_id: &str,
    method: Method,
    path: &str,
    upstream_route: &str,
) {
    let upstream =
        spawn_app_upstream(Router::new().route(upstream_route, any(echo_context_upstream))).await;
    let app = web_gateway::build_app(test_gateway_config(vec![service_upstream(
        service_id,
        upstream.base_url.as_str(),
    )]));
    let body = if method == Method::GET {
        Body::empty()
    } else {
        Body::from("{}")
    };

    let is_get = method == Method::GET;
    let mut request_builder = Request::builder()
        .method(method)
        .uri(path)
        .header(header::AUTHORIZATION, gateway_test_authorization_header())
        .header("access-token", gateway_test_access_token_header());
    if !is_get {
        request_builder = request_builder.header(header::CONTENT_TYPE, "application/json");
    }
    let response = app
        .oneshot(request_builder.body(body).unwrap())
        .await
        .unwrap_or_else(|error| panic!("gateway request should succeed for {service_id}: {error}"));

    assert_eq!(
        response.status(),
        StatusCode::OK,
        "{service_id} must derive request context directly from dual tokens"
    );
    let context = read_json_body(response).await;
    assert_eq!(
        context["tenantId"], "100001",
        "{service_id} must receive dual-token tenant context without appbase session lookup"
    );
    assert_eq!(
        context["userId"], "30",
        "{service_id} must receive dual-token user context without appbase session lookup"
    );
    assert_eq!(
        context["sessionId"], "session_real",
        "{service_id} must receive dual-token session context without appbase session lookup"
    );
    assert_eq!(
        context["sdkworkInternalHeadersForwarded"], false,
        "{service_id} must not receive client-supplied SDKWork SDKWork internal headers"
    );
    assert_ne!(context["tenantId"], "100001");
    assert_ne!(context["userId"], "user_test006_a_com");
}

async fn echo_upstream(
    State(state): State<UpstreamState>,
    method: Method,
    request: Request<Body>,
) -> Json<serde_json::Value> {
    Json(json!({
        "serviceId": state.service_id.as_ref(),
        "method": method.as_str(),
        "path": request.uri().path(),
    }))
}

async fn appbase_current_session(headers: HeaderMap) -> Response {
    let Ok(context) = resolve_app_context(&headers) else {
        return (
            StatusCode::UNAUTHORIZED,
            Json(json!({
                "type": "about:blank",
                "title": "Unauthorized",
                "status": 401,
                "detail": "dual token session is required"
            })),
        )
            .into_response();
    };

    (
        StatusCode::OK,
        Json(json!({
            "data": {
                "context": {
                    "tenantId": context.tenant_id,
                    "organizationId": context.organization_id,
                    "userId": context.user_id,
                    "sessionId": context.session_id,
                    "appId": context.app_id,
                    "actorKind": context.actor_kind
                }
            }
        })),
    )
        .into_response()
}

async fn appbase_numeric_current_session(headers: HeaderMap) -> Response {
    let Ok(context) = resolve_app_context(&headers) else {
        return (
            StatusCode::UNAUTHORIZED,
            Json(json!({
                "type": "about:blank",
                "title": "Unauthorized",
                "status": 401,
                "detail": "dual token session is required"
            })),
        )
            .into_response();
    };

    (
        StatusCode::OK,
        Json(json!({
            "data": {
                "context": {
                    "tenantId": context.tenant_id,
                    "organizationId": context.organization_id,
                    "userId": context.user_id,
                    "sessionId": context.session_id,
                    "appId": context.app_id,
                    "actorKind": context.actor_kind
                }
            }
        })),
    )
        .into_response()
}

async fn echo_context_upstream(headers: HeaderMap) -> Json<serde_json::Value> {
    match resolve_app_context(&headers) {
        Ok(context) => Json(json!({
            "tenantId": context.tenant_id,
            "organizationId": context.organization_id,
            "userId": context.user_id,
            "sessionId": context.session_id,
            "sdkworkInternalHeadersForwarded": has_sdkwork_internal_header(&headers),
        })),
        Err(error) => Json(json!({
            "code": error.code(),
            "message": error.message(),
            "sdkworkInternalHeadersForwarded": has_sdkwork_internal_header(&headers),
        })),
    }
}

async fn require_signed_context_upstream(headers: HeaderMap) -> Response {
    match im_app_context::resolve_app_context_with_signature_config(
        &headers,
        im_app_context::AppContextSignatureConfig {
            require_signature: true,
            shared_secret: Some("gateway-signing-secret".to_owned()),
        },
    ) {
        Ok(context) => Json(json!({
            "tenantId": context.tenant_id,
            "userId": context.user_id,
            "sessionId": context.session_id,
            "signaturePresent": header_value(&headers, "x-sdkwork-context-signature").is_some(),
            "sdkworkInternalHeadersForwarded": has_sdkwork_internal_header(&headers),
        }))
        .into_response(),
        Err(error) => (
            StatusCode::UNAUTHORIZED,
            Json(json!({
                "code": error.code(),
                "message": error.message(),
            })),
        )
            .into_response(),
    }
}

fn header_value(headers: &HeaderMap, name: &str) -> Option<String> {
    headers
        .get(name)
        .and_then(|value| value.to_str().ok())
        .map(str::to_owned)
}

fn has_sdkwork_internal_header(headers: &HeaderMap) -> bool {
    [
        "x-sdkwork-tenant-id",
        "x-sdkwork-organization-id",
        "x-sdkwork-user-id",
        "x-sdkwork-session-id",
        "x-sdkwork-context-signature",
    ]
    .iter()
    .any(|name| headers.contains_key(*name))
}
