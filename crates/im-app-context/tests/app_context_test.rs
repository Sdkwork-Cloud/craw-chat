use axum::http::{HeaderMap, HeaderValue, header};
use im_app_context::{
    AppContextSignatureConfig, DualTokenRequestBuilderExt, build_dual_token_headers_for_context,
    local_service_app_context, require_app_context_signature, resolve_app_context,
    resolve_app_context_for_request, resolve_app_context_with_signature_config,
    sign_app_context_headers,
};
use sdkwork_utils_rust::base64url_encode;
use serde_json::{Value, json};

static TEST_ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

const TEST_JWT_ISSUER: &str = "https://iam.sdkwork.test";
const TEST_JWT_AUDIENCE: &str = "sdkwork-im";

fn lock_test_env() -> std::sync::MutexGuard<'static, ()> {
    TEST_ENV_LOCK
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
}

fn ensure_test_dev_environment() {
    unsafe {
        std::env::set_var("SDKWORK_IM_ENVIRONMENT", "dev");
        std::env::remove_var("SDKWORK_IM_APP_CONTEXT_JWT_TENANT_ID");
        std::env::remove_var("SDKWORK_IM_APP_CONTEXT_JWT_KEY_ID");
        std::env::remove_var("SDKWORK_IM_APP_CONTEXT_JWT_SIGNING_SECRET");
        std::env::remove_var("SDKWORK_IM_JWT_EXPECTED_ISSUERS");
        std::env::remove_var("SDKWORK_IM_JWT_EXPECTED_AUDIENCES");
    }
}

fn configure_production_jwt_signing_env() {
    unsafe {
        std::env::set_var("SDKWORK_IM_ENVIRONMENT", "production");
        std::env::set_var("SDKWORK_IM_APP_CONTEXT_JWT_TENANT_ID", "100001");
        std::env::set_var("SDKWORK_IM_APP_CONTEXT_JWT_KEY_ID", "bootstrap");
        std::env::set_var("SDKWORK_IM_APP_CONTEXT_JWT_SIGNING_SECRET", "prod-secret");
        std::env::set_var("SDKWORK_IM_JWT_EXPECTED_ISSUERS", TEST_JWT_ISSUER);
        std::env::set_var("SDKWORK_IM_JWT_EXPECTED_AUDIENCES", TEST_JWT_AUDIENCE);
    }
}

fn configure_production_dev_jwt_secret_env() {
    unsafe {
        std::env::set_var("SDKWORK_IM_ENVIRONMENT", "production");
        std::env::set_var("SDKWORK_IM_APP_CONTEXT_JWT_TENANT_ID", "100001");
        std::env::set_var("SDKWORK_IM_APP_CONTEXT_JWT_KEY_ID", "bootstrap");
        std::env::set_var(
            "SDKWORK_IM_APP_CONTEXT_JWT_SIGNING_SECRET",
            "sdkwork-im-dev-jwt-secret-not-for-production-use",
        );
        std::env::set_var("SDKWORK_IM_JWT_EXPECTED_ISSUERS", TEST_JWT_ISSUER);
        std::env::set_var("SDKWORK_IM_JWT_EXPECTED_AUDIENCES", TEST_JWT_AUDIENCE);
    }
}

struct TestDevEnvironment {
    _guard: std::sync::MutexGuard<'static, ()>,
}

fn test_dev_environment() -> TestDevEnvironment {
    let guard = lock_test_env();
    ensure_test_dev_environment();
    TestDevEnvironment { _guard: guard }
}

struct TestProductionJwtEnvironment {
    _guard: std::sync::MutexGuard<'static, ()>,
}

impl Drop for TestProductionJwtEnvironment {
    fn drop(&mut self) {
        ensure_test_dev_environment();
    }
}

fn test_production_jwt_environment() -> TestProductionJwtEnvironment {
    let guard = lock_test_env();
    configure_production_jwt_signing_env();
    TestProductionJwtEnvironment { _guard: guard }
}

struct TestProductionDevJwtSecretEnvironment {
    _guard: std::sync::MutexGuard<'static, ()>,
}

impl Drop for TestProductionDevJwtSecretEnvironment {
    fn drop(&mut self) {
        ensure_test_dev_environment();
    }
}

fn test_production_dev_jwt_secret_environment() -> TestProductionDevJwtSecretEnvironment {
    let guard = lock_test_env();
    configure_production_dev_jwt_secret_env();
    TestProductionDevJwtSecretEnvironment { _guard: guard }
}

struct TestProductionEnvironment {
    _guard: std::sync::MutexGuard<'static, ()>,
}

impl Drop for TestProductionEnvironment {
    fn drop(&mut self) {
        ensure_test_dev_environment();
    }
}

fn test_production_environment() -> TestProductionEnvironment {
    let guard = lock_test_env();
    unsafe {
        std::env::set_var("SDKWORK_IM_ENVIRONMENT", "production");
    }
    TestProductionEnvironment { _guard: guard }
}

fn local_token(claims: Value) -> String {
    let mut claims = claims;
    if let Some(object) = claims.as_object_mut() {
        object
            .entry("token_version")
            .or_insert(json!(sdkwork_web_core::stamp_token_version()));
    }
    let header = base64url_encode(r#"{"alg":"none","typ":"JWT"}"#.as_bytes());
    let payload = base64url_encode(claims.to_string().as_bytes());
    format!("{header}.{payload}.local")
}

fn build_token_headers() -> HeaderMap {
    let claims = json!({
        "tenant_id": "100001",
        "organization_id": "o_demo",
        "login_scope": "ORGANIZATION",
        "user_id": "1",
        "session_id": "as_demo",
        "device_id": "d_demo",
        "app_id": "sdkwork-im",
        "environment": "dev",
        "deployment_mode": "private",
        "auth_level": "password",
        "actor_id": "1",
        "actor_kind": "user",
        "permission_scope": ["ops.read", "audit.*", "media.write"],
        "data_scope": ["tenant"]
    });
    let token = local_token(claims);
    let mut headers = HeaderMap::new();
    headers.insert(
        header::AUTHORIZATION,
        HeaderValue::from_str(format!("Bearer {token}").as_str()).expect("auth header"),
    );
    headers.insert(
        "Access-Token",
        HeaderValue::from_str(token.as_str()).expect("access token header"),
    );
    headers
}

#[test]
fn test_build_dual_token_headers_for_context_emits_only_dual_token_credentials() {
    let context = local_service_app_context(
        "100001",
        "1",
        "user",
        Some("d_demo"),
        ["chat.write", "chat.read"],
    );
    let headers = build_dual_token_headers_for_context(&context, ["chat.write"]);

    assert!(headers.contains_key(header::AUTHORIZATION));
    assert!(headers.contains_key("Access-Token"));
    assert_eq!(headers.len(), 2);

    let access_token = headers
        .get("Access-Token")
        .and_then(|value| value.to_str().ok())
        .expect("access token header");
    let payload = access_token
        .split('.')
        .nth(1)
        .expect("access token payload segment");
    let claims: Value = serde_json::from_slice(
        &sdkwork_utils_rust::base64url_decode(payload).expect("access token payload"),
    )
    .expect("access token claims json");
    assert_eq!(
        claims
            .get("token_version")
            .and_then(Value::as_u64)
            .expect("token_version claim"),
        u64::from(sdkwork_web_core::stamp_token_version()),
    );
}

#[test]
fn test_dual_token_permission_scope_builder_splits_comma_and_whitespace() {
    let _env = test_dev_environment();
    let request = axum::http::Request::builder()
        .with_dual_token_context("100001", "1", "user", Some("d_demo"), ["chat.read"])
        .with_dual_token_permission_scope("ops.read, audit.*\nmedia.write")
        .body(())
        .expect("request should build");

    let context = resolve_app_context(request.headers()).expect("app context should resolve");

    assert!(context.has_permission("ops.read"));
    assert!(context.has_permission("audit.write"));
    assert!(context.has_permission("media.write"));
    assert!(!context.has_permission("chat.read"));
}

#[test]
fn test_dual_token_builder_accepts_owned_string_values() {
    let _env = test_dev_environment();
    let request = axum::http::Request::builder()
        .with_dual_token_context("100001", "1", "user", None, ["chat.read"])
        .with_dual_token_session(format!("s_{}", "owned"))
        .with_dual_token_device(format!("d_{}", "owned"))
        .with_dual_token_permission_scope("chat.write")
        .body(())
        .expect("request should build");

    let context = resolve_app_context(request.headers()).expect("app context should resolve");

    assert_eq!(context.session_id.as_deref(), Some("s_owned"));
    assert_eq!(context.device_id.as_deref(), Some("d_owned"));
    assert!(context.has_permission("chat.write"));
}

#[test]
fn test_resolve_app_context_uses_dual_token_claims() {
    let _env = test_dev_environment();
    let headers = build_token_headers();

    let context = resolve_app_context(&headers).expect("app context should resolve");

    assert_eq!(context.tenant_id, "100001");
    assert_eq!(context.organization_id, "o_demo");
    assert_eq!(context.user_id, "1");
    assert_eq!(context.actor_id, "1");
    assert_eq!(context.actor_kind, "user");
    assert_eq!(context.session_id.as_deref(), Some("as_demo"));
    assert_eq!(context.device_id.as_deref(), Some("d_demo"));
    assert_eq!(context.app_id.as_deref(), Some("sdkwork-im"));
    assert_eq!(context.environment.as_deref(), Some("dev"));
    assert_eq!(context.deployment_mode.as_deref(), Some("private"));
    assert_eq!(context.auth_level.as_deref(), Some("password"));
}

#[test]
fn test_signature_config_compatibility_resolves_dual_token_claims() {
    let _env = test_dev_environment();
    let mut headers = build_token_headers();
    headers.insert("x-sdkwork-tenant-id", HeaderValue::from_static("100001"));
    headers.insert("x-sdkwork-user-id", HeaderValue::from_static("1"));
    headers.insert("x-sdkwork-session-id", HeaderValue::from_static("as_demo"));
    headers.insert("x-sdkwork-actor-id", HeaderValue::from_static("1"));
    headers.insert("x-sdkwork-actor-kind", HeaderValue::from_static("user"));
    headers.insert("x-sdkwork-device-id", HeaderValue::from_static("d_demo"));
    headers.insert("x-sdkwork-app-id", HeaderValue::from_static("sdkwork-im"));
    let signature = sign_app_context_headers(&headers, "gateway-signing-secret")
        .expect("signature should be generated for signed token headers");
    headers.insert(
        "x-sdkwork-context-signature",
        HeaderValue::from_str(signature.as_str()).expect("signature must be a header value"),
    );

    let context = resolve_app_context_with_signature_config(
        &headers,
        AppContextSignatureConfig {
            require_signature: true,
            shared_secret: Some("gateway-signing-secret".to_owned()),
        },
    )
    .expect("legacy signature config callers should still resolve dual token context");

    assert_eq!(context.tenant_id, "100001");
    assert_eq!(context.user_id, "1");
    assert_eq!(context.session_id.as_deref(), Some("as_demo"));
}

#[test]
fn test_app_context_signature_verifies_canonical_projection_headers() {
    let mut headers = HeaderMap::new();
    headers.insert("x-sdkwork-app-id", HeaderValue::from_static("sdkwork-im"));
    headers.insert("x-sdkwork-tenant-id", HeaderValue::from_static("100001"));
    headers.insert("x-sdkwork-user-id", HeaderValue::from_static("1"));
    headers.insert("x-sdkwork-session-id", HeaderValue::from_static("s_demo"));
    headers.insert("x-sdkwork-actor-id", HeaderValue::from_static("1"));
    headers.insert("x-sdkwork-actor-kind", HeaderValue::from_static("user"));
    headers.insert("x-sdkwork-device-id", HeaderValue::from_static("d_demo"));
    headers.insert(
        "x-sdkwork-permission-scope",
        HeaderValue::from_static("chat.write"),
    );

    let signature =
        sign_app_context_headers(&headers, "demo-secret").expect("signature should be generated");
    headers.insert(
        "x-sdkwork-context-signature",
        HeaderValue::from_str(signature.as_str()).expect("signature must be a header value"),
    );
    require_app_context_signature(
        &headers,
        &AppContextSignatureConfig {
            require_signature: true,
            shared_secret: Some("demo-secret".to_owned()),
        },
    )
    .expect("matching signature should verify");

    headers.insert(
        "x-sdkwork-context-signature",
        HeaderValue::from_static("invalid-signature"),
    );
    let error = require_app_context_signature(
        &headers,
        &AppContextSignatureConfig {
            require_signature: true,
            shared_secret: Some("demo-secret".to_owned()),
        },
    )
    .expect_err("invalid signature must fail");
    assert_eq!(error.code(), "app_context_invalid");
    assert!(
        error.message().contains("signature validation failed"),
        "unexpected error message: {}",
        error.message()
    );
}

#[test]
fn test_resolve_app_context_for_request_exposes_appbase_context() {
    let _env = test_dev_environment();
    let headers = build_token_headers();

    let resolved =
        resolve_app_context_for_request(&headers, "/app/v3/api/messages", "POST").expect("context");

    assert_eq!(
        resolved.app_request_context.transport.path,
        "/app/v3/api/messages"
    );
    assert_eq!(resolved.app_request_context.transport.method, "POST");
    assert!(resolved.app_request_context.transport.auth_token_present);
    assert!(resolved.app_request_context.transport.access_token_present);
    let principal = resolved
        .app_request_context
        .principal
        .as_ref()
        .expect("principal");
    assert_eq!(principal.tenant_id(), "100001");
    assert_eq!(principal.organization_id(), Some("o_demo"));
    assert_eq!(principal.user_id(), "1");
    assert_eq!(principal.app_id(), "sdkwork-im");
}

#[test]
fn test_resolve_app_context_rejects_missing_access_token() {
    let _env = test_dev_environment();
    let mut headers = build_token_headers();
    headers.remove("Access-Token");

    let error = resolve_app_context(&headers).expect_err("access token must be required");

    assert_eq!(error.code(), "access_token_missing");
    assert!(
        error.message().to_ascii_lowercase().contains("access"),
        "unexpected error message: {}",
        error.message()
    );
}

#[test]
fn test_resolve_app_context_rejects_mismatched_user() {
    let _env = test_dev_environment();
    let mut headers = HeaderMap::new();
    let auth = local_token(json!({
        "tenant_id": "100001",
        "user_id": "u_auth",
        "session_id": "as_demo",
        "app_id": "sdkwork-im"
    }));
    let access = local_token(json!({
        "tenant_id": "100001",
        "user_id": "u_access",
        "session_id": "as_demo",
        "app_id": "sdkwork-im"
    }));
    headers.insert(
        header::AUTHORIZATION,
        HeaderValue::from_str(format!("Bearer {auth}").as_str()).expect("auth header"),
    );
    headers.insert(
        "Access-Token",
        HeaderValue::from_str(access.as_str()).expect("access token header"),
    );

    let error = resolve_app_context(&headers).expect_err("mismatch must fail");

    assert_eq!(error.code(), "app_context_invalid");
    assert!(error.message().contains("user_id"));
}

#[test]
fn test_app_context_projects_ccp_authority_fields() {
    let _env = test_dev_environment();
    let headers = build_token_headers();

    let context = resolve_app_context(&headers).expect("app context should resolve");
    let authority = context.ccp_authority();

    assert_eq!(authority.tenant_id, "100001");
    assert_eq!(authority.actor.actor_id, "1");
    assert_eq!(authority.actor.actor_kind, "user");
    assert_eq!(authority.sender.principal_id, "1");
    assert_eq!(authority.sender.device_id.as_deref(), Some("d_demo"));
    assert_eq!(authority.sender.session_id.as_deref(), Some("as_demo"));
    assert_eq!(authority.sender.sender_id(), "1:d_demo");
}

#[test]
fn test_app_context_permissions_support_exact_and_wildcard_matches() {
    let _env = test_dev_environment();
    let headers = build_token_headers();

    let context = resolve_app_context(&headers).expect("app context should resolve");

    assert!(context.has_permission("ops.read"));
    assert!(context.has_permission("audit.read"));
    assert!(context.has_permission("media.write"));
    assert!(!context.has_permission("ops.write"));
}

#[test]
fn test_resolve_app_context_rejects_expired_jwt() {
    let _env = test_dev_environment();
    let mut claims = json!({
        "tenant_id": "100001",
        "organization_id": "o_demo",
        "login_scope": "ORGANIZATION",
        "user_id": "1",
        "session_id": "as_demo",
        "device_id": "d_demo",
        "app_id": "sdkwork-im",
        "environment": "dev",
        "deployment_mode": "private",
        "auth_level": "password",
        "actor_id": "1",
        "actor_kind": "user",
        "permission_scope": ["ops.read"],
        "data_scope": ["tenant"],
        "exp": 1
    });
    let token = local_token(claims.take());
    let mut headers = HeaderMap::new();
    headers.insert(
        header::AUTHORIZATION,
        HeaderValue::from_str(format!("Bearer {token}").as_str()).expect("auth header"),
    );
    headers.insert(
        "Access-Token",
        HeaderValue::from_str(token.as_str()).expect("access token header"),
    );

    let error = resolve_app_context(&headers).expect_err("expired token must fail");
    assert_eq!(error.code(), "app_context_invalid");
    assert!(error.message().contains("expired"));
}

#[test]
fn test_resolve_app_context_rejects_unsigned_local_jwt_in_production() {
    let _env = test_production_environment();
    unsafe {
        std::env::remove_var("SDKWORK_IM_APP_CONTEXT_JWT_TENANT_ID");
        std::env::remove_var("SDKWORK_IM_APP_CONTEXT_JWT_KEY_ID");
        std::env::remove_var("SDKWORK_IM_APP_CONTEXT_JWT_SIGNING_SECRET");
    }
    let headers = build_token_headers();
    let error = resolve_app_context(&headers).expect_err("unsigned local jwt must fail");
    assert_eq!(error.code(), "app_context_invalid");
    assert!(error.message().contains("unsigned local JWT"));
}

fn signed_dual_token_headers(tenant_id: &str, secret: &str, key_id: &str) -> HeaderMap {
    use sdkwork_web_core::encode_hs256_test_jwt_with_kid;

    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|duration| duration.as_secs() as i64)
        .unwrap_or(0);
    let claims = json!({
        "tenant_id": tenant_id,
        "organization_id": "o_demo",
        "login_scope": "ORGANIZATION",
        "user_id": "1",
        "session_id": "as_demo",
        "device_id": "d_demo",
        "app_id": "sdkwork-im",
        "environment": "production",
        "deployment_mode": "private",
        "auth_level": "password",
        "actor_id": "1",
        "actor_kind": "user",
        "permission_scope": ["ops.read"],
        "data_scope": ["tenant"],
        "iss": TEST_JWT_ISSUER,
        "aud": TEST_JWT_AUDIENCE,
        "exp": now + 3600
    });
    let token = encode_hs256_test_jwt_with_kid(secret, key_id, claims);
    let mut headers = HeaderMap::new();
    headers.insert(
        header::AUTHORIZATION,
        HeaderValue::from_str(format!("Bearer {token}").as_str()).expect("auth header"),
    );
    headers.insert(
        "Access-Token",
        HeaderValue::from_str(token.as_str()).expect("access token header"),
    );
    headers
}

#[test]
fn test_resolve_app_context_rejects_dev_jwt_signing_secret_in_production() {
    let _env = test_production_dev_jwt_secret_environment();
    let headers = signed_dual_token_headers(
        "100001",
        "sdkwork-im-dev-jwt-secret-not-for-production-use",
        "bootstrap",
    );
    let error = resolve_app_context(&headers)
        .expect_err("production must reject the public dev JWT signing secret");
    assert_eq!(error.code(), "app_context_invalid");
    assert!(error.message().contains("dev/test JWT signing secret"));
}

#[test]
fn test_resolve_app_context_accepts_signed_jwt_in_production() {
    let _env = test_production_jwt_environment();
    let headers = signed_dual_token_headers("100001", "prod-secret", "bootstrap");
    let context = resolve_app_context(&headers).expect("signed jwt must resolve");
    assert_eq!(context.tenant_id, "100001");
    assert_eq!(context.user_id, "1");
}

#[test]
fn test_resolve_app_context_rejects_signed_jwt_without_signing_config_in_production() {
    let _env = test_production_environment();
    unsafe {
        std::env::remove_var("SDKWORK_IM_APP_CONTEXT_JWT_TENANT_ID");
        std::env::remove_var("SDKWORK_IM_APP_CONTEXT_JWT_KEY_ID");
        std::env::remove_var("SDKWORK_IM_APP_CONTEXT_JWT_SIGNING_SECRET");
    }
    let headers = signed_dual_token_headers("100001", "prod-secret", "bootstrap");
    let error = resolve_app_context(&headers).expect_err("signed jwt without config must fail");
    assert_eq!(error.code(), "app_context_invalid");
    assert!(error.message().contains("signed JWT verification requires"));
}

#[test]
fn test_resolve_app_context_rejects_tampered_signed_jwt_in_production() {
    let _env = test_production_jwt_environment();
    let headers = signed_dual_token_headers("t_other", "prod-secret", "bootstrap");
    let error = resolve_app_context(&headers).expect_err("tenant mismatch must fail");
    assert_eq!(error.code(), "app_context_invalid");
    assert!(error.message().contains("tenant_id"));
}

#[test]
fn test_resolve_app_context_rejects_raw_json_bearer_outside_dev_test() {
    let _env = test_production_environment();
    let mut headers = HeaderMap::new();
    let json_token = r#"{"tenant_id":"t_evil","user_id":"u_evil","organization_id":"0","actor_id":"u_evil","actor_kind":"user","permission_scope":["*"],"data_scope":["tenant"]}"#;
    headers.insert(
        header::AUTHORIZATION,
        HeaderValue::from_str(format!("Bearer {json_token}").as_str()).expect("auth header"),
    );
    headers.insert(
        "Access-Token",
        HeaderValue::from_str(json_token).expect("access token header"),
    );
    let error = resolve_app_context(&headers).expect_err("raw JSON bearer must fail in production");
    assert_eq!(error.code(), "app_context_invalid");
    assert!(error.message().contains("JSON bearer"));
}

#[test]
fn websocket_upgrade_auth_headers_detect_authorization_and_access_token_variants() {
    let mut headers = HeaderMap::new();
    assert!(!im_app_context::has_websocket_upgrade_auth_headers(
        &headers
    ));

    headers.insert(
        header::AUTHORIZATION,
        HeaderValue::from_static("Bearer auth-token"),
    );
    assert!(im_app_context::has_websocket_upgrade_auth_headers(&headers));

    headers.remove(header::AUTHORIZATION);
    headers.insert("Access-Token", HeaderValue::from_static("access-token"));
    assert!(im_app_context::has_websocket_upgrade_auth_headers(&headers));

    headers.remove("Access-Token");
    headers.insert("access-token", HeaderValue::from_static("access-token"));
    assert!(im_app_context::has_websocket_upgrade_auth_headers(&headers));
}

#[test]
fn websocket_query_device_id_is_extracted_from_path_and_query() {
    assert_eq!(
        im_app_context::websocket_query_device_id_from_path_and_query(
            "/im/v3/api/realtime/ws?deviceId=d_pad"
        ),
        Some("d_pad".to_owned())
    );
    assert_eq!(
        im_app_context::websocket_query_device_id_from_path_and_query(
            "/im/v3/api/realtime/ws?deviceId=d_pad&conversationId=c1"
        ),
        Some("d_pad".to_owned())
    );
    assert_eq!(
        im_app_context::websocket_query_device_id_from_path_and_query("/im/v3/api/realtime/ws"),
        None
    );
}

#[test]
fn coalesce_websocket_device_id_prefers_frame_then_query() {
    assert_eq!(
        im_app_context::coalesce_websocket_device_id(
            Some("frame_device".to_owned()),
            Some("query_device".to_owned()),
        ),
        Some("frame_device".to_owned())
    );
    assert_eq!(
        im_app_context::coalesce_websocket_device_id(None, Some("query_device".to_owned()),),
        Some("query_device".to_owned())
    );
    assert_eq!(
        im_app_context::coalesce_websocket_device_id(None, None),
        None
    );
}

#[test]
fn local_service_app_context_uses_iam_default_tenant_and_organization_scope() {
    let _env = test_dev_environment();
    let context = local_service_app_context("100001", "30", "user", None, ["*"]);
    assert_eq!(context.tenant_id, "100001");
    assert_eq!(context.organization_id, "0");
    assert_eq!(context.user_id, "30");

    let headers = build_dual_token_headers_for_context(&context, context.permission_scope.iter());
    let resolved = resolve_app_context(&headers).expect("resolve IAM-aligned local context");
    assert_eq!(resolved.tenant_id, "100001");
    assert_eq!(resolved.organization_id, "0");
    assert_eq!(resolved.user_id, "30");
}
