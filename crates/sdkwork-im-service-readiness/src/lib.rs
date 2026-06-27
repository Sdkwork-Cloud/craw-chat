use std::sync::Arc;

use im_app_context::resolve_web_environment_from_process_env;
use sdkwork_database_sqlx::DatabasePool;
use sdkwork_web_bootstrap::{AlwaysReady, CompositeReadinessCheck, ReadinessCheck, ReadinessFuture};
use sdkwork_web_core::WebEnvironment;
use session_gateway::resolve_iam_auth_pool_from_env;
use sqlx::PgPool;

struct PgPoolReadinessCheck {
    pool: Arc<PgPool>,
    label: &'static str,
}

impl ReadinessCheck for PgPoolReadinessCheck {
    fn check(&self) -> ReadinessFuture<'_> {
        let pool = self.pool.clone();
        let label = self.label;
        Box::pin(async move {
            sqlx::query("SELECT 1")
                .execute(pool.as_ref())
                .await
                .map_err(|error| format!("{label} database readiness failed: {error}"))?;
            Ok(())
        })
    }
}

#[derive(Clone)]
struct DatabasePoolReadinessCheck {
    pool: DatabasePool,
}

impl ReadinessCheck for DatabasePoolReadinessCheck {
    fn check(&self) -> ReadinessFuture<'_> {
        let pool = self.pool.clone();
        Box::pin(async move {
            match &pool {
                DatabasePool::Sqlite(sqlite, _) => {
                    sqlx::query("SELECT 1")
                        .execute(sqlite)
                        .await
                        .map_err(|error| format!("im sqlite readiness failed: {error}"))?;
                }
                DatabasePool::Postgres(postgres, _) => {
                    sqlx::query("SELECT 1")
                        .execute(postgres)
                        .await
                        .map_err(|error| format!("im postgres readiness failed: {error}"))?;
                }
            }
            Ok(())
        })
    }
}

#[derive(Clone)]
struct RedisUrlReadinessCheck {
    url: String,
}

impl ReadinessCheck for RedisUrlReadinessCheck {
    fn check(&self) -> ReadinessFuture<'_> {
        let url = self.url.clone();
        Box::pin(async move {
            ping_redis_url(url.as_str())
                .map_err(|error| format!("redis readiness failed: {error}"))
        })
    }
}

#[derive(Clone)]
struct MissingDependencyReadinessCheck {
    dependency: &'static str,
}

impl MissingDependencyReadinessCheck {
    fn new(dependency: &'static str) -> Self {
        Self { dependency }
    }
}

impl ReadinessCheck for MissingDependencyReadinessCheck {
    fn check(&self) -> ReadinessFuture<'_> {
        let dependency = self.dependency;
        Box::pin(async move {
            Err(format!("required dependency is not configured: {dependency}"))
        })
    }
}

pub fn resolve_im_redis_url_from_env() -> Option<String> {
    let enabled = std::env::var("SDKWORK_IM_REDIS_ENABLED")
        .ok()
        .map(|value| {
            matches!(
                value.trim().to_ascii_lowercase().as_str(),
                "1" | "true" | "yes" | "on"
            )
        })
        .unwrap_or(false);
    if !enabled {
        return None;
    }

    std::env::var("SDKWORK_IM_REDIS_URL")
        .ok()
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())
}

fn redis_required_in_production(environment: &WebEnvironment) -> bool {
    matches!(environment, WebEnvironment::Prod)
        && std::env::var("SDKWORK_IM_DEPLOYMENT_PROFILE")
            .ok()
            .map(|value| value.trim().eq_ignore_ascii_case("cloud"))
            .unwrap_or(false)
}

fn ping_redis_url(redis_url: &str) -> Result<(), String> {
    redis::Client::open(redis_url)
        .map_err(|error| error.to_string())
        .and_then(|client| {
            client
                .get_connection()
                .map_err(|error| error.to_string())
        })
        .and_then(|mut connection| {
            redis::cmd("PING")
                .query::<String>(&mut connection)
                .map_err(|error| error.to_string())
        })
        .and_then(|response| {
            if response.eq_ignore_ascii_case("PONG") {
                Ok(())
            } else {
                Err(format!("redis ping returned unexpected payload: {response}"))
            }
        })
}

/// Readiness label for JSON and plain-text `/readyz` handlers.
pub fn im_service_readiness_status_label() -> &'static str {
    if evaluate_im_runtime_dependency_health_from_env() {
        "ok"
    } else {
        "unavailable"
    }
}

#[derive(Clone, Default)]
struct ImEnvReadinessCheck;

impl ReadinessCheck for ImEnvReadinessCheck {
    fn check(&self) -> ReadinessFuture<'_> {
        Box::pin(async {
            if evaluate_im_runtime_dependency_health_from_env() {
                Ok(())
            } else {
                Err("im runtime dependencies are unavailable".into())
            }
        })
    }
}

/// Default readiness probe for split IM HTTP service processes.
pub fn im_env_readiness_check() -> Arc<dyn ReadinessCheck> {
    Arc::new(ImEnvReadinessCheck)
}

/// Synchronous dependency probe for split-service processes that expose `/readyz`
/// without async startup wiring.
pub fn evaluate_im_runtime_dependency_health_from_env() -> bool {
    let environment = resolve_web_environment_from_process_env();

    if let Some(redis_url) = resolve_im_redis_url_from_env() {
        if ping_redis_url(redis_url.as_str()).is_err() {
            return false;
        }
    } else if redis_required_in_production(&environment) {
        return false;
    }

    if let Some(database_url) = resolve_im_database_url_from_env() {
        if ping_postgres_url(database_url.as_str()).is_err() {
            return false;
        }
        return true;
    }

    if matches!(environment, WebEnvironment::Prod) {
        return false;
    }

    matches!(environment, WebEnvironment::Dev | WebEnvironment::Test)
}

fn resolve_im_database_url_from_env() -> Option<String> {
    std::env::var("SDKWORK_IM_DATABASE_URL")
        .ok()
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())
}

fn ping_postgres_url(database_url: &str) -> Result<(), String> {
    use postgres::{Client, NoTls};

    let mut client = Client::connect(database_url, NoTls)
        .map_err(|error| format!("postgres connect failed: {error}"))?;
    client
        .simple_query("SELECT 1")
        .map_err(|error| format!("postgres ping failed: {error}"))?;
    Ok(())
}

/// Initialize structured logging and optional OTel export for IM service processes.
pub fn init_im_service_tracing_from_env() {
    sdkwork_web_bootstrap::init_tracing_from_env();
}

pub async fn resolve_im_service_readiness_check() -> Arc<dyn ReadinessCheck> {
    let environment = resolve_web_environment_from_process_env();
    let mut checks: Vec<Arc<dyn ReadinessCheck>> = Vec::new();

    if let Some(pool) = resolve_iam_auth_pool_from_env().await {
        checks.push(Arc::new(PgPoolReadinessCheck {
            pool,
            label: "iam",
        }));
    }

    if let Ok(pool) = sdkwork_im_database_pool::create_im_database_pool_from_env().await {
        checks.push(Arc::new(DatabasePoolReadinessCheck { pool }));
    }

    match resolve_im_redis_url_from_env() {
        Some(url) => checks.push(Arc::new(RedisUrlReadinessCheck { url })),
        None if redis_required_in_production(&environment) => {
            checks.push(Arc::new(MissingDependencyReadinessCheck::new("redis")));
        }
        None => {}
    }

    match checks.len() {
        0 if matches!(environment, WebEnvironment::Dev | WebEnvironment::Test) => {
            Arc::new(AlwaysReady)
        }
        0 => Arc::new(MissingDependencyReadinessCheck::new(
            "iam or im database connectivity",
        )),
        1 => checks.pop().expect("single readiness check"),
        _ => Arc::new(CompositeReadinessCheck::new(checks)),
    }
}

pub async fn resolve_gateway_readiness_check() -> Arc<dyn ReadinessCheck> {
    resolve_im_service_readiness_check().await
}

/// Sets `SDKWORK_IM_SERVICE_NAME` and `OTEL_SERVICE_NAME` when unset so metrics and traces use a stable service id.
pub fn ensure_im_service_process_identity(service_name: &str) {
    let service_name = service_name.trim();
    if service_name.is_empty() {
        return;
    }
    if std::env::var("SDKWORK_IM_SERVICE_NAME").is_err() {
        unsafe {
            std::env::set_var("SDKWORK_IM_SERVICE_NAME", service_name);
        }
    }
    if std::env::var("OTEL_SERVICE_NAME").is_err() {
        unsafe {
            std::env::set_var("OTEL_SERVICE_NAME", service_name);
        }
    }
}

#[cfg(test)]
mod identity_tests {
    use super::*;

    #[test]
    fn ensure_im_service_process_identity_sets_defaults_when_unset() {
        let prior_service = std::env::var("SDKWORK_IM_SERVICE_NAME").ok();
        let prior_otel = std::env::var("OTEL_SERVICE_NAME").ok();
        std::env::remove_var("SDKWORK_IM_SERVICE_NAME");
        std::env::remove_var("OTEL_SERVICE_NAME");
        ensure_im_service_process_identity("test-service");
        assert_eq!(
            std::env::var("SDKWORK_IM_SERVICE_NAME").ok().as_deref(),
            Some("test-service")
        );
        assert_eq!(
            std::env::var("OTEL_SERVICE_NAME").ok().as_deref(),
            Some("test-service")
        );
        match prior_service {
            Some(value) => std::env::set_var("SDKWORK_IM_SERVICE_NAME", value),
            None => std::env::remove_var("SDKWORK_IM_SERVICE_NAME"),
        }
        match prior_otel {
            Some(value) => std::env::set_var("OTEL_SERVICE_NAME", value),
            None => std::env::remove_var("OTEL_SERVICE_NAME"),
        }
    }
}
