use sdkwork_web_core::{
    EnvBootstrapTenantSigningKeyLookup, WebAuthLevel, WebDeploymentMode, WebEnvironment,
};

pub(crate) const APP_CONTEXT_REQUIRE_SIGNATURE_ENV: &str = "SDKWORK_IM_APP_CONTEXT_REQUIRE_SIGNATURE";
pub(crate) const APP_CONTEXT_SIGNATURE_SECRET_ENV: &str = "SDKWORK_IM_APP_CONTEXT_SIGNATURE_SECRET";
pub(crate) const APP_CONTEXT_SIGNATURE_SECRET_FILE_ENV: &str =
    "SDKWORK_IM_APP_CONTEXT_SIGNATURE_SECRET_FILE";
pub(crate) const APP_CONTEXT_JWT_TENANT_ID_ENV: &str = "SDKWORK_IM_APP_CONTEXT_JWT_TENANT_ID";
pub(crate) const APP_CONTEXT_JWT_KEY_ID_ENV: &str = "SDKWORK_IM_APP_CONTEXT_JWT_KEY_ID";
pub(crate) const APP_CONTEXT_JWT_SIGNING_SECRET_ENV: &str = "SDKWORK_IM_APP_CONTEXT_JWT_SIGNING_SECRET";
pub(crate) const APP_CONTEXT_JWT_SIGNING_SECRET_FILE_ENV: &str =
    "SDKWORK_IM_APP_CONTEXT_JWT_SIGNING_SECRET_FILE";
pub(crate) const APP_CONTEXT_JWT_KEY_ID_DEFAULT: &str = "bootstrap";

/// Env override for the dev/test JWT signing secret. When set, this value
/// replaces the built-in dev secret so local developers can rotate or
/// personalize the key without code changes. Production MUST NOT rely on this
/// fallback — production requires `SDKWORK_IM_APP_CONTEXT_JWT_SIGNING_SECRET`.
pub(crate) const DEV_JWT_SIGNING_SECRET_ENV: &str = "SDKWORK_IM_DEV_JWT_SIGNING_SECRET";
pub(crate) const DEV_JWT_SIGNING_SECRET_FILE_ENV: &str = "SDKWORK_IM_DEV_JWT_SIGNING_SECRET_FILE";

/// Resolves the dev/test JWT signing secret.
///
/// Resolution order:
/// 1. `SDKWORK_IM_DEV_JWT_SIGNING_SECRET_FILE` (Docker/K8s secrets pattern)
/// 2. `SDKWORK_IM_DEV_JWT_SIGNING_SECRET` (direct env override)
/// 3. Built-in deterministic dev secret (see `DEV_JWT_SIGNING_SECRET_FALLBACK`)
///
/// The built-in fallback is intentionally public and non-secret — it only
/// protects dev/test from the `alg=none` bypass, never production. Production
/// fails closed via `validate_jwt_token` when tenant-bound signing env is
/// absent (see `jwt.rs`). Overriding via env is recommended for shared dev
/// environments.
pub(crate) fn resolve_dev_jwt_signing_secret() -> Vec<u8> {
    if let Some(secret) = resolve_secret_from_env_or_file(
        DEV_JWT_SIGNING_SECRET_ENV,
        DEV_JWT_SIGNING_SECRET_FILE_ENV,
    ) {
        return secret.into_bytes();
    }
    DEV_JWT_SIGNING_SECRET_FALLBACK.as_bytes().to_vec()
}

/// Built-in, intentionally non-secret dev/test fallback. This value is public
/// in the source tree by design — it is NOT a security boundary. The security
/// boundary is the production fail-closed path in `validate_jwt_token`.
pub(crate) const DEV_JWT_SIGNING_SECRET_FALLBACK: &str =
    "sdkwork-im-dev-jwt-secret-not-for-production-use";

pub(crate) const SDKWORK_CONTEXT_SIGNATURE_HEADER: &str = "x-sdkwork-context-signature";
pub(crate) const SIGNED_APP_CONTEXT_HEADER_NAMES: &[&str] = &[
    "x-sdkwork-app-id",
    "x-sdkwork-tenant-id",
    "x-sdkwork-organization-id",
    "x-sdkwork-user-id",
    "x-sdkwork-session-id",
    "x-sdkwork-environment",
    "x-sdkwork-deployment-mode",
    "x-sdkwork-auth-level",
    "x-sdkwork-data-scope",
    "x-sdkwork-permission-scope",
    "x-sdkwork-actor-id",
    "x-sdkwork-actor-kind",
    "x-sdkwork-device-id",
];

pub(crate) fn parse_truthy_env_flag(raw: Option<String>) -> bool {
    raw.as_deref().map(str::trim).is_some_and(|value| {
        matches!(
            value.to_ascii_lowercase().as_str(),
            "1" | "true" | "yes" | "on"
        )
    })
}

/// Resolve a secret value from either a direct env var or a `_FILE` env var.
///
/// Follows the Docker/Kubernetes secrets pattern: if `<key>_FILE` is set,
/// the secret is read from the referenced file path; otherwise, the value
/// of `<key>` is used directly. The `_FILE` variant takes precedence.
pub(crate) fn resolve_secret_from_env_or_file(direct_key: &str, file_key: &str) -> Option<String> {
    // Check _FILE variant first (Docker/Kubernetes secrets pattern).
    if let Ok(file_path) = std::env::var(file_key) {
        let trimmed_path = file_path.trim();
        if !trimmed_path.is_empty() {
            return std::fs::read_to_string(trimmed_path)
                .ok()
                .map(|content| content.trim().to_owned())
                .filter(|value| !value.is_empty())
                .or_else(|| {
                    tracing::error!(
                        target: "sdkwork.im.app_context",
                        "failed to read secret file `{trimmed_path}` referenced by {file_key}"
                    );
                    None
                });
        }
    }
    // Fall back to direct env var.
    std::env::var(direct_key)
        .ok()
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())
}

pub(crate) fn tenant_signing_lookup_from_env() -> Option<EnvBootstrapTenantSigningKeyLookup> {
    let tenant_id = std::env::var(APP_CONTEXT_JWT_TENANT_ID_ENV)
        .ok()
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())?;
    let key_id = std::env::var(APP_CONTEXT_JWT_KEY_ID_ENV)
        .ok()
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| APP_CONTEXT_JWT_KEY_ID_DEFAULT.to_owned());
    let secret = resolve_secret_from_env_or_file(
        APP_CONTEXT_JWT_SIGNING_SECRET_ENV,
        APP_CONTEXT_JWT_SIGNING_SECRET_FILE_ENV,
    )?;
    Some(EnvBootstrapTenantSigningKeyLookup::new(
        tenant_id,
        key_id,
        secret.as_bytes(),
    ))
}

pub(crate) fn parse_environment(value: Option<String>) -> WebEnvironment {
    match value
        .as_deref()
        .unwrap_or("prod")
        .trim()
        .to_ascii_lowercase()
        .as_str()
    {
        "dev" | "development" => WebEnvironment::Dev,
        "test" | "testing" => WebEnvironment::Test,
        _ => WebEnvironment::Prod,
    }
}

pub(crate) fn parse_deployment_mode(value: Option<String>) -> WebDeploymentMode {
    match value
        .as_deref()
        .unwrap_or("saas")
        .trim()
        .to_ascii_lowercase()
        .as_str()
    {
        "local" => WebDeploymentMode::Local,
        "private" | "private_cloud" => WebDeploymentMode::Private,
        _ => WebDeploymentMode::Saas,
    }
}

pub(crate) fn parse_auth_level(value: Option<String>) -> WebAuthLevel {
    match value
        .as_deref()
        .unwrap_or("password")
        .trim()
        .to_ascii_lowercase()
        .as_str()
    {
        "anonymous" => WebAuthLevel::Anonymous,
        "mfa" => WebAuthLevel::Mfa,
        "system" => WebAuthLevel::System,
        "api_key" | "apikey" => WebAuthLevel::ApiKey,
        _ => WebAuthLevel::Password,
    }
}

pub(crate) fn format_environment(value: &WebEnvironment) -> &'static str {
    match value {
        WebEnvironment::Dev => "dev",
        WebEnvironment::Test => "test",
        WebEnvironment::Prod => "prod",
    }
}

pub(crate) fn format_deployment_mode(value: &WebDeploymentMode) -> &'static str {
    match value {
        WebDeploymentMode::Saas => "saas",
        WebDeploymentMode::Local => "local",
        WebDeploymentMode::Private => "private",
    }
}

pub(crate) fn format_auth_level(value: &WebAuthLevel) -> &'static str {
    match value {
        WebAuthLevel::Anonymous => "anonymous",
        WebAuthLevel::Password => "password",
        WebAuthLevel::Mfa => "mfa",
        WebAuthLevel::System => "system",
        WebAuthLevel::ApiKey => "api_key",
    }
}

/// Resolve the canonical SDKWork web environment from process env (`SDKWORK_IM_ENVIRONMENT`).
pub fn resolve_web_environment_from_process_env() -> WebEnvironment {
    parse_environment(std::env::var("SDKWORK_IM_ENVIRONMENT").ok())
}

/// Whether services may fall back to header-only AppContext resolution without IAM DB lookup.
pub fn allows_header_only_app_context_fallback() -> bool {
    matches!(
        resolve_web_environment_from_process_env(),
        WebEnvironment::Dev | WebEnvironment::Test
    )
}
