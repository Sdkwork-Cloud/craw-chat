use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use im_app_context::DualTokenRequestBuilderExt;
use std::sync::OnceLock;
use tokio::sync::{Mutex, MutexGuard};
use tower::ServiceExt;

mod test_env;

const APP_CONTEXT_REQUIRE_SIGNATURE_ENV: &str = "SDKWORK_IM_APP_CONTEXT_REQUIRE_SIGNATURE";
const APP_CONTEXT_SIGNATURE_SECRET_ENV: &str = "SDKWORK_IM_APP_CONTEXT_SIGNATURE_SECRET";

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
        } else {
            unsafe {
                std::env::remove_var(self.name);
            }
        }
    }
}

fn app_context_signature_env_guard() -> &'static Mutex<()> {
    static GUARD: OnceLock<Mutex<()>> = OnceLock::new();
    GUARD.get_or_init(|| Mutex::new(()))
}

async fn lock_app_context_signature_env_guard() -> MutexGuard<'static, ()> {
    app_context_signature_env_guard().lock().await
}

fn signed_presence_request_builder() -> axum::http::request::Builder {
    Request::builder()
        .method("GET")
        .uri("/im/v3/api/presence/me")
        .header(axum::http::header::AUTHORIZATION, "Bearer auth_token")
        .header("access-token", "access_token")
        .with_dual_token_tenant("100001")
        .with_dual_token_user("1")
        .with_dual_token_actor_kind("user")
        .with_dual_token_session("s_demo")
        .with_dual_token_device("d_demo")
}

#[tokio::test]
async fn test_public_app_rejects_missing_or_invalid_context_signature_when_enabled() {
    let _dev_env = test_env::dev_test_environment();
    let _env_guard = lock_app_context_signature_env_guard().await;
    let _require_signature = ScopedEnvVar::set(APP_CONTEXT_REQUIRE_SIGNATURE_ENV, "true");
    let _signature_secret = ScopedEnvVar::set(APP_CONTEXT_SIGNATURE_SECRET_ENV, "demo-secret");
    let app = session_gateway::build_public_app();

    let missing_signature = app
        .clone()
        .oneshot(
            signed_presence_request_builder()
                .body(Body::empty())
                .expect("request should build"),
        )
        .await
        .expect("request should return response");
    assert_eq!(missing_signature.status(), StatusCode::UNAUTHORIZED);
    let missing_content_type = missing_signature
        .headers()
        .get(axum::http::header::CONTENT_TYPE)
        .and_then(|value| value.to_str().ok())
        .expect("problem detail response should include content-type");
    assert!(
        missing_content_type.starts_with("application/problem+json"),
        "error response must use problem+json content type, got {missing_content_type}"
    );
    let missing_body = missing_signature
        .into_body()
        .collect()
        .await
        .expect("response body should collect")
        .to_bytes();
    let missing_json: serde_json::Value =
        serde_json::from_slice(&missing_body).expect("response should be valid json");
    assert_eq!(missing_json["type"], "about:blank");
    assert_eq!(missing_json["status"], 401);
    assert_eq!(missing_json["title"], "Unauthorized");
    assert_eq!(missing_json["code"], "app_context_invalid");
    assert!(
        missing_json["message"]
            .as_str()
            .is_some_and(|message| message.contains("x-sdkwork-context-signature")),
        "missing signature should return explicit header requirement error"
    );

    let invalid_signature = app
        .oneshot(
            signed_presence_request_builder()
                .header("x-sdkwork-context-signature", "invalid-signature")
                .body(Body::empty())
                .expect("request should build"),
        )
        .await
        .expect("request should return response");
    assert_eq!(invalid_signature.status(), StatusCode::UNAUTHORIZED);
    let invalid_content_type = invalid_signature
        .headers()
        .get(axum::http::header::CONTENT_TYPE)
        .and_then(|value| value.to_str().ok())
        .expect("problem detail response should include content-type");
    assert!(
        invalid_content_type.starts_with("application/problem+json"),
        "error response must use problem+json content type, got {invalid_content_type}"
    );
    let invalid_body = invalid_signature
        .into_body()
        .collect()
        .await
        .expect("response body should collect")
        .to_bytes();
    let invalid_json: serde_json::Value =
        serde_json::from_slice(&invalid_body).expect("response should be valid json");
    assert_eq!(invalid_json["type"], "about:blank");
    assert_eq!(invalid_json["status"], 401);
    assert_eq!(invalid_json["title"], "Unauthorized");
    assert_eq!(invalid_json["code"], "app_context_invalid");
    assert!(
        invalid_json["message"]
            .as_str()
            .is_some_and(|message| message.contains("signature validation failed")),
        "invalid signature should return verification failure"
    );
}
