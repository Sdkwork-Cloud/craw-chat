//! Realtime auth resolution: IAM database dual-token verification when configured,
//! with `im-app-context` env bootstrap fallback for dev/private control-plane.

use std::sync::Arc;

use axum::http::{HeaderMap, header};
use im_app_context::{
    AppContext, AppContextError, allows_header_only_app_context_fallback,
    app_context_from_web_principal, resolve_app_context,
};
use sdkwork_iam_web_adapter::{
    resolve_iam_app_context_from_dual_tokens, resolve_iam_postgres_pool_from_env,
    web_request_principal_from_iam,
};
use sqlx::PgPool;

#[derive(Clone, Default)]
pub struct RealtimeAuthContextResolver {
    iam_pool: Option<Arc<PgPool>>,
}

impl RealtimeAuthContextResolver {
    pub fn new(iam_pool: Option<Arc<PgPool>>) -> Self {
        Self { iam_pool }
    }

    pub fn iam_pool(&self) -> Option<&PgPool> {
        self.iam_pool.as_deref()
    }

    pub async fn resolve_from_headers(
        &self,
        headers: &HeaderMap,
    ) -> Result<AppContext, AppContextError> {
        if let Some(pool) = self.iam_pool.as_deref() {
            let auth_token = extract_bearer_token(headers)
                .ok_or_else(AppContextError::auth_token_missing)?;
            let access_token = extract_access_token(headers)
                .ok_or_else(AppContextError::access_token_missing)?;
            if let Some(iam_context) =
                resolve_iam_app_context_from_dual_tokens(pool, &auth_token, &access_token).await
            {
                let principal = web_request_principal_from_iam(iam_context);
                return Ok(app_context_from_web_principal(&principal));
            }
            return Err(AppContextError::invalid(
                "invalid or expired IAM session for realtime auth",
            ));
        }
        if !allows_header_only_app_context_fallback() {
            return Err(AppContextError::invalid(
                "IAM database pool is required for realtime auth outside dev/test environments",
            ));
        }
        resolve_app_context(headers)
    }
}

fn extract_bearer_token(headers: &HeaderMap) -> Option<String> {
    headers
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .map(str::trim)
        .and_then(|value| {
            let (scheme, token) = value.split_once(' ')?;
            if scheme.eq_ignore_ascii_case("bearer") && !token.trim().is_empty() {
                return Some(token.trim().to_owned());
            }
            None
        })
}

fn extract_access_token(headers: &HeaderMap) -> Option<String> {
    headers
        .get("access-token")
        .or_else(|| headers.get("Access-Token"))
        .and_then(|value| value.to_str().ok())
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(|value| {
            value
                .strip_prefix("Bearer ")
                .or_else(|| value.strip_prefix("bearer "))
                .unwrap_or(value)
                .trim()
                .to_owned()
        })
}

pub async fn resolve_iam_auth_pool_from_env() -> Option<Arc<PgPool>> {
    resolve_iam_postgres_pool_from_env().await
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use axum::http::{HeaderMap, HeaderValue, header};
    use serde_json::json;
    use sdkwork_utils_rust::base64url_encode;

    use super::*;

    static TEST_ENV_LOCK: Mutex<()> = Mutex::new(());

    struct TestDevEnvironment {
        _guard: std::sync::MutexGuard<'static, ()>,
    }

    fn test_dev_environment() -> TestDevEnvironment {
        let guard = TEST_ENV_LOCK.lock().expect("test env lock");
        unsafe {
            std::env::set_var("SDKWORK_IM_ENVIRONMENT", "dev");
        }
        TestDevEnvironment { _guard: guard }
    }

    fn local_token(claims: serde_json::Value) -> String {
        let mut claims = claims;
        if let Some(object) = claims.as_object_mut() {
            object
                .entry("token_version")
                .or_insert(json!(sdkwork_web_core::stamp_token_version()));
        }
        let header_segment = base64url_encode(r#"{"alg":"none","typ":"JWT"}"#.as_bytes());
        let payload = base64url_encode(claims.to_string().as_bytes());
        format!("{header_segment}.{payload}.local")
    }

    fn dual_token_headers() -> HeaderMap {
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
            "permission_scope": ["ops.read"],
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

    #[tokio::test]
    async fn resolver_without_iam_pool_uses_im_app_context() {
        let _env = test_dev_environment();
        let resolver = RealtimeAuthContextResolver::default();
        assert!(resolver.iam_pool().is_none());

        let context = resolver
            .resolve_from_headers(&dual_token_headers())
            .await
            .expect("dev dual-token headers must resolve through im-app-context fallback");
        assert_eq!(context.tenant_id, "100001");
        assert_eq!(context.user_id, "1");
    }

    #[tokio::test]
    async fn resolver_without_iam_pool_rejects_missing_access_token() {
        let _env = test_dev_environment();
        let resolver = RealtimeAuthContextResolver::default();
        let mut headers = dual_token_headers();
        headers.remove("Access-Token");

        let error = resolver
            .resolve_from_headers(&headers)
            .await
            .expect_err("missing access token must fail closed");
        assert_eq!(error.code(), "access_token_missing");
    }

    #[tokio::test]
    async fn resolver_without_iam_pool_rejects_production_environment() {
        let _guard = TEST_ENV_LOCK.lock().expect("test env lock");
        unsafe {
            std::env::set_var("SDKWORK_IM_ENVIRONMENT", "production");
        }
        let resolver = RealtimeAuthContextResolver::default();
        assert!(resolver.iam_pool().is_none());

        let error = resolver
            .resolve_from_headers(&dual_token_headers())
            .await
            .expect_err("production realtime auth must require IAM database pool");
        assert_eq!(error.code(), "app_context_invalid");
        assert!(error.message().contains("IAM database pool"));
        unsafe {
            std::env::set_var("SDKWORK_IM_ENVIRONMENT", "dev");
        }
    }
}
