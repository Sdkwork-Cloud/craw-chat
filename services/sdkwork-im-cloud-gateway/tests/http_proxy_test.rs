use axum::{
    Json, Router,
    body::{Body, Bytes},
    extract::State,
    http::{HeaderMap, Method, Request, StatusCode, header},
    response::{IntoResponse, Response},
    routing::{any, get},
};
use futures_util::stream;
use http_body_util::BodyExt;
use im_app_context::{
    AppContext, build_dual_token_headers_for_context, local_service_app_context,
    resolve_app_context,
};
use sdkwork_im_api_registry::{HttpMethod, SdkTarget};
use sdkwork_im_cloud_gateway_config::{WebGatewayConfig, service_upstream};
use serde_json::json;
use std::{convert::Infallible, sync::Arc, time::Duration};
use tokio::sync::Notify;
use tower::ServiceExt;

fn ensure_gateway_test_web_environment() {
    static INIT: std::sync::Once = std::sync::Once::new();
    INIT.call_once(|| unsafe {
        std::env::set_var("SDKWORK_IM_ENVIRONMENT", "test");
        std::env::set_var("SDKWORK_ENV", "test");
    });
}

#[derive(Clone)]
struct UpstreamState {
    service_id: Arc<str>,
}

#[derive(Clone)]
struct StreamingAuditState {
    release_final_chunk: Arc<Notify>,
}

fn gateway_test_app_context() -> AppContext {
    let mut context = local_service_app_context("100001", "30", "user", Some("device_real"), ["*"]);
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
async fn gateway_streams_external_audit_export_without_buffering_the_complete_body() {
    let release_final_chunk = Arc::new(Notify::new());
    let audit_upstream = spawn_app_upstream(
        Router::new()
            .route("/backend/v3/api/audit/export", get(streaming_audit_export))
            .with_state(StreamingAuditState {
                release_final_chunk: release_final_chunk.clone(),
            }),
    )
    .await;
    let app = web_gateway::build_app(test_gateway_config(vec![service_upstream(
        "audit-service",
        audit_upstream.base_url.as_str(),
    )]));
    let auth_headers = gateway_numeric_auth_headers();
    let mut request_task = tokio::spawn(
        app.oneshot(
            Request::builder()
                .method(Method::GET)
                .uri("/backend/v3/api/audit/export")
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
        ),
    );

    let response = tokio::time::timeout(Duration::from_secs(2), &mut request_task)
        .await
        .expect("gateway should return response headers before the upstream body completes")
        .expect("gateway request task should not panic")
        .expect("gateway request should succeed");
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        response
            .headers()
            .get("x-sdkwork-im-upstream-service")
            .and_then(|value| value.to_str().ok()),
        Some("audit-service")
    );

    let mut body = response.into_body();
    let first_frame = tokio::time::timeout(Duration::from_secs(2), body.frame())
        .await
        .expect("gateway should forward the first export chunk immediately")
        .expect("stream should contain a first frame")
        .expect("first stream frame should succeed");
    let first_chunk = first_frame
        .data_ref()
        .expect("first frame should contain data")
        .clone();
    assert_eq!(
        first_chunk,
        Bytes::from_static(br#"{"code":0,"data":{"tenantId":"100001","items":["#)
    );

    release_final_chunk.notify_one();
    let remaining = body
        .collect()
        .await
        .expect("remaining export body should collect")
        .to_bytes();
    let mut payload = first_chunk.to_vec();
    payload.extend_from_slice(&remaining);
    let json: serde_json::Value =
        serde_json::from_slice(&payload).expect("proxied export should remain valid json");
    assert_eq!(json["data"]["total"], 0);
    assert_eq!(json["data"]["chainValid"], true);
}

#[tokio::test]
async fn gateway_routes_conversation_messages_reads_and_writes_to_runtime_upstream() {
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
    assert_eq!(read_value["serviceId"], "comms-conversation-service");

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
async fn gateway_delegates_conversation_messages_to_embedded_runtime_when_upstream_is_missing() {
    let product_runtime = Router::new().route(
        "/im/v3/api/chat/conversations/{conversationId}/messages",
        get(|| async { Json(json!({ "servedBy": "embedded-application" })) }),
    );
    let app = web_gateway::build_app_with_registry_and_product_runtime(
        test_gateway_config(Vec::new()),
        web_gateway::build_gateway_registry().expect("gateway route registry should build"),
        Some(product_runtime),
    );

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::GET)
                .uri("/im/v3/api/chat/conversations/c_direct_09a8255a1fd3632675c2d355/messages")
                .header(header::AUTHORIZATION, gateway_test_authorization_header())
                .header("access-token", gateway_test_access_token_header())
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("gateway request should succeed");

    assert_eq!(response.status(), StatusCode::OK);
    let value = read_json_body(response).await;
    assert_eq!(value["servedBy"], "embedded-application");
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
                .uri("/im/v3/api/realtime/events?afterSeq=0&page_size=1")
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
                .uri("/im/v3/api/realtime/events?afterSeq=0&page_size=1")
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
                .uri("/im/v3/api/realtime/events?afterSeq=0&page_size=1")
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
            .header("access-token", gateway_test_access_token_header())
            .header("x-sdkwork-tenant-id", "tenant_forge_ignored")
            .header("x-sdkwork-user-id", "user_test006_a_com");
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
        assert_ne!(context["tenantId"], "tenant_forge_ignored");
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
            "comms-conversation-service",
            Method::GET,
            "/im/v3/api/chat/conversations/c_demo/messages",
            "/im/v3/api/chat/conversations/{id}/messages",
        ),
        (
            "comms-conversation-service",
            Method::POST,
            "/app/v3/api/chat/conversations/c_demo/knowledgebase/launch",
            "/app/v3/api/chat/conversations/{conversationId}/knowledgebase/launch",
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
        (
            "sdkwork-voice-app-api",
            Method::GET,
            "/app/v3/api/voice/audio_assets",
            "/app/v3/api/voice/{*path}",
        ),
        (
            "sdkwork-agents-app-api",
            Method::POST,
            "/app/v3/api/ai/agents",
            "/app/v3/api/ai/{*path}",
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
            "/im/v3/api/realtime/events?afterSeq=0&page_size=1",
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
            "comms-conversation-service",
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
        (
            "sdkwork-voice-app-api",
            Method::GET,
            "/app/v3/api/voice/audio_assets",
            "/app/v3/api/voice/{*path}",
        ),
        (
            "sdkwork-agents-app-api",
            Method::POST,
            "/app/v3/api/ai/agents",
            "/app/v3/api/ai/{*path}",
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
    assert_eq!(
        courses.expect("courses route").service_id,
        "sdkwork-course-app-api"
    );
}

#[test]
fn gateway_registry_routes_ai_agents_to_agents_app_api() {
    let registry = web_gateway::build_gateway_registry().expect("registry should build");

    let create_route = registry
        .resolve(HttpMethod::Post, "/app/v3/api/ai/agents")
        .expect("agents create route should resolve");
    assert_eq!(create_route.service_id, "sdkwork-agents-app-api");
    assert_eq!(
        create_route.sdk_targets,
        vec![SdkTarget::SdkworkAgentsAppSdk]
    );
    assert_eq!(create_route.operation_group, "agents");

    let list_route = registry
        .resolve(HttpMethod::Get, "/app/v3/api/ai/agents")
        .expect("agents list route should resolve");
    assert_eq!(list_route.service_id, "sdkwork-agents-app-api");

    let nested_route = registry
        .resolve(
            HttpMethod::Post,
            "/app/v3/api/ai/agents/agent_1/provider_bindings",
        )
        .expect("nested agents route should resolve");
    assert_eq!(nested_route.service_id, "sdkwork-agents-app-api");
}

#[test]
fn gateway_registry_routes_group_knowledgebase_app_api_to_conversation_runtime() {
    let registry = web_gateway::build_gateway_registry().expect("registry should build");

    for (method, path) in [
        (
            HttpMethod::Get,
            "/app/v3/api/chat/conversations/c_demo/knowledgebase",
        ),
        (
            HttpMethod::Post,
            "/app/v3/api/chat/conversations/c_demo/knowledgebase",
        ),
        (
            HttpMethod::Post,
            "/app/v3/api/chat/conversations/c_demo/knowledgebase/launch",
        ),
    ] {
        let route = registry
            .resolve(method, path)
            .unwrap_or_else(|| panic!("group knowledgebase route should resolve for {path}"));
        assert_eq!(route.service_id, "comms-conversation-service");
        assert_eq!(route.sdk_targets, vec![SdkTarget::SdkworkImAppSdk]);
        assert_eq!(route.operation_group, "group-knowledgebase");
    }
}

#[test]
fn gateway_registry_routes_message_favorites_to_projection_service() {
    let registry = web_gateway::build_gateway_registry().expect("registry should build");

    let list_route = registry
        .resolve(HttpMethod::Get, "/im/v3/api/chat/messages/favorites")
        .expect("favorites list route should resolve");
    assert_eq!(list_route.service_id, "projection-service");

    let create_route = registry
        .resolve(HttpMethod::Post, "/im/v3/api/chat/messages/msg_1/favorites")
        .expect("favorite create route should resolve");
    assert_eq!(create_route.service_id, "projection-service");

    let delete_route = registry
        .resolve(
            HttpMethod::Delete,
            "/im/v3/api/chat/messages/favorites/fav_1",
        )
        .expect("favorite delete route should resolve");
    assert_eq!(delete_route.service_id, "projection-service");
}

#[test]
fn gateway_registry_keeps_conversation_truth_reads_on_conversation_runtime() {
    let registry = web_gateway::build_gateway_registry().expect("registry should build");

    for path in [
        "/im/v3/api/chat/conversations/c_demo/messages",
        "/im/v3/api/chat/conversations/c_demo/members",
        "/im/v3/api/chat/conversations/c_demo/read_cursor",
        "/im/v3/api/chat/conversations/c_demo/binding",
        "/im/v3/api/chat/conversations/c_demo/agent_handoff",
    ] {
        let route = registry
            .resolve(HttpMethod::Get, path)
            .unwrap_or_else(|| panic!("conversation truth read route should resolve for {path}"));
        assert_eq!(
            route.service_id, "comms-conversation-service",
            "{path} must be served by the conversation runtime"
        );
    }
}

#[test]
fn gateway_registry_keeps_projection_only_conversation_reads_on_projection_service() {
    let registry = web_gateway::build_gateway_registry().expect("registry should build");

    for path in [
        "/im/v3/api/chat/conversations/c_demo",
        "/im/v3/api/chat/conversations/c_demo/member_directory",
        "/im/v3/api/chat/conversations/c_demo/pins",
        "/im/v3/api/chat/conversations/c_demo/messages/msg_1/interaction_summary",
        "/im/v3/api/chat/conversations/c_demo/profile",
        "/im/v3/api/chat/conversations/c_demo/preferences",
        "/im/v3/api/chat/messages/search",
    ] {
        let route = registry
            .resolve(HttpMethod::Get, path)
            .unwrap_or_else(|| panic!("projection read route should resolve for {path}"));
        assert_eq!(
            route.service_id, "projection-service",
            "{path} must be served by projection-service"
        );
    }
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
        runtime_mode: sdkwork_im_cloud_gateway_config::GatewayRuntimeMode::SingleIngress,
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

async fn streaming_audit_export(State(state): State<StreamingAuditState>) -> Response {
    let stream = stream::unfold(
        (0u8, state.release_final_chunk),
        |(step, release_final_chunk)| async move {
            match step {
                0 => Some((
                    Ok::<Bytes, Infallible>(Bytes::from_static(
                        br#"{"code":0,"data":{"tenantId":"100001","items":["#,
                    )),
                    (1, release_final_chunk),
                )),
                1 => {
                    release_final_chunk.notified().await;
                    Some((
                        Ok(Bytes::from_static(
                            br#"],"total":0,"chainHeadHash":null,"chainValid":true},"traceId":"trace-upstream"}"#,
                        )),
                        (2, release_final_chunk),
                    ))
                }
                _ => None,
            }
        },
    );
    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/json; charset=utf-8")
        .body(Body::from_stream(stream))
        .expect("streaming audit response should build")
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

/// Verifies that gateway 50301 responses comply with `API_SPEC.md` §15.2 and
/// `OBSERVABILITY_SPEC.md` §2:
/// - `instance` uses the route template, not the raw path with business ids.
/// - `operationId` is present when the gateway resolved the matched route.
/// - `traceId` is generated by the gateway and does not trust caller headers.
/// - `code` is the numeric platform code 50301.
#[tokio::test]
async fn gateway_problem_response_uses_route_template_and_correlation_for_50301() {
    // Configure an upstream that always returns 500 to trip the circuit breaker.
    let upstream = spawn_app_upstream(Router::new().route(
        "/{*path}",
        any(|| async { (StatusCode::INTERNAL_SERVER_ERROR, "upstream down") }),
    ))
    .await;

    // Lower the circuit breaker threshold so a single failure trips it.
    let _threshold = ScopedEnvVar::set("SDKWORK_IM_GATEWAY_CIRCUIT_BREAKER_THRESHOLD", "1");

    let app = web_gateway::build_app(test_gateway_config(vec![service_upstream(
        "comms-conversation-service",
        upstream.base_url.as_str(),
    )]));

    let conversation_path =
        "/im/v3/api/chat/conversations/c_direct_09a8255a1fd3632675c2d355/messages";

    // First request trips the breaker by hitting the failing upstream.
    let _ = app
        .clone()
        .oneshot(
            Request::builder()
                .method(Method::GET)
                .uri(conversation_path)
                .header(header::AUTHORIZATION, gateway_test_authorization_header())
                .header("access-token", gateway_test_access_token_header())
                .header(
                    "traceparent",
                    "00-4bf92f3577b34da6a3ce929d0e0e4736-00f067aa0ba902b7-01",
                )
                .header("x-request-id", "client-request-1")
                .body(Body::empty())
                .unwrap(),
        )
        .await;

    // Second request is rejected by the circuit breaker → 50301.
    let response = app
        .oneshot(
            Request::builder()
                .method(Method::GET)
                .uri(conversation_path)
                .header(header::AUTHORIZATION, gateway_test_authorization_header())
                .header("access-token", gateway_test_access_token_header())
                .header(
                    "traceparent",
                    "00-4bf92f3577b34da6a3ce929d0e0e4736-00f067aa0ba902b7-01",
                )
                .header("x-request-id", "client-request-2")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("gateway circuit-breaker request should succeed");

    assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);

    let value = read_json_body(response).await;

    // Numeric platform code (API_SPEC.md §15.3)
    assert_eq!(value["code"].as_i64(), Some(50301));

    // traceId is server-owned and must not be copied from client correlation headers.
    let trace_id = value["traceId"]
        .as_str()
        .expect("gateway problem response must include traceId");
    assert!(
        !trace_id.trim().is_empty(),
        "traceId must be non-empty: {value}"
    );
    assert_ne!(trace_id, "4bf92f3577b34da6a3ce929d0e0e4736");
    assert_ne!(trace_id, "client-request-2");

    // instance must use the route template, not the raw path with business ids
    // (OBSERVABILITY_SPEC.md §2, API_SPEC.md §15.2)
    let instance = value["instance"]
        .as_str()
        .expect("instance field should be present");
    assert!(
        !instance.contains("c_direct_09a8255a1fd3632675c2d355"),
        "instance must not leak business resource id, got: {instance}"
    );
    assert!(
        instance.contains("{conversationId}") || instance.contains("{*path}"),
        "instance should use route template, got: {instance}"
    );

    // operationId should be present when the gateway resolved the route
    // (API_SPEC.md §15.2)
    assert!(
        value.get("operationId").is_some(),
        "operationId should be present, got: {value}"
    );

    // i18nKey must follow the `errors.result.<code>` convention
    // (I18N_SPEC.md, API_SPEC.md §15.2)
    assert_eq!(
        value["i18nKey"].as_str(),
        Some("errors.result.50301"),
        "i18nKey should be derived from numeric code, got: {value}"
    );
}
