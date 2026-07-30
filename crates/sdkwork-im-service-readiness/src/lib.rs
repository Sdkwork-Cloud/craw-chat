use std::future::Future;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, OnceLock};

use im_app_context::resolve_web_environment_from_process_env;
use sdkwork_database_sqlx::DatabasePool;
use sdkwork_web_bootstrap::{CompositeReadinessCheck, ReadinessCheck, ReadinessFuture};
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
                DatabasePool::Postgres(postgres, _) => {
                    sqlx::query("SELECT 1")
                        .execute(postgres)
                        .await
                        .map_err(|error| format!("im postgres readiness failed: {error}"))?;
                }
                #[allow(unreachable_patterns)]
                _ => {
                    return Err(format!(
                        "IM server readiness rejected unsupported '{}' persistence",
                        pool.engine()
                    ));
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
            ping_redis_url(url.as_str()).map_err(|error| format!("redis readiness failed: {error}"))
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
            Err(format!(
                "required dependency is not configured: {dependency}"
            ))
        })
    }
}

#[derive(Clone)]
struct UnavailableDependencyReadinessCheck {
    dependency: &'static str,
}

impl UnavailableDependencyReadinessCheck {
    fn new(dependency: &'static str) -> Self {
        Self { dependency }
    }
}

impl ReadinessCheck for UnavailableDependencyReadinessCheck {
    fn check(&self) -> ReadinessFuture<'_> {
        let dependency = self.dependency;
        Box::pin(async move { Err(format!("{dependency} is unavailable")) })
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
        .and_then(|client| client.get_connection().map_err(|error| error.to_string()))
        .and_then(|mut connection| {
            redis::cmd("PING")
                .query::<String>(&mut connection)
                .map_err(|error| error.to_string())
        })
        .and_then(|response| {
            if response.eq_ignore_ascii_case("PONG") {
                Ok(())
            } else {
                Err(format!(
                    "redis ping returned unexpected payload: {response}"
                ))
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

/// Default readiness probe for IM HTTP service processes.
pub fn im_env_readiness_check() -> Arc<dyn ReadinessCheck> {
    Arc::new(ImEnvReadinessCheck)
}

/// Synchronous dependency probe for cloud service processes that expose `/readyz`
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

    false
}

fn resolve_im_database_url_from_env() -> Option<String> {
    std::env::var("SDKWORK_DATABASE_URL")
        .ok()
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())
}

fn ping_postgres_url(database_url: &str) -> Result<(), String> {
    use postgres::Client;

    let tls = make_tls_connector()
        .map_err(|error| format!("postgres TLS connector build failed: {error}"))?;
    let mut client = Client::connect(database_url, tls)
        .map_err(|error| format!("postgres connect failed: {error}"))?;
    client
        .simple_query("SELECT 1")
        .map_err(|error| format!("postgres ping failed: {error}"))?;
    Ok(())
}

/// Build a `native-tls` connector for PostgreSQL.
///
/// Uses the system trust store for certificate verification. The actual TLS
/// negotiation is gated by the `sslmode` URL parameter: when `sslmode=disable`
/// the `postgres` crate never invokes this connector.
fn make_tls_connector() -> Result<postgres_native_tls::MakeTlsConnector, native_tls::Error> {
    let connector = native_tls::TlsConnector::builder().build()?;
    Ok(postgres_native_tls::MakeTlsConnector::new(connector))
}

/// Initialize structured logging and optional OTel export for IM service processes.
pub fn init_im_service_tracing_from_env() {
    sdkwork_web_bootstrap::init_tracing_from_env();
}

/// Enable the canonical process pool before any IM or embedded module bootstrap.
pub fn enable_process_shared_database_pool() {
    sdkwork_database_sqlx::enable_process_shared_database_pool();
}

/// Install shared IM sqlx + r2d2 pools when PostgreSQL is configured.
///
/// Every IM HTTP/RPC process in standalone and cloud deployments
/// SHOULD call this before assembling routes or opening PostgreSQL adapters.
pub async fn bootstrap_im_service_database_from_env() -> Result<(), String> {
    enable_process_shared_database_pool();
    sdkwork_im_database_pool::bootstrap_im_process_database_pools_from_env()
        .await
        .map(|_| ())
}

#[derive(Clone)]
struct BooleanReadinessCheck {
    label: String,
    healthy: Arc<AtomicBool>,
}

impl ReadinessCheck for BooleanReadinessCheck {
    fn check(&self) -> ReadinessFuture<'_> {
        let label = self.label.clone();
        let healthy = self.healthy.clone();
        Box::pin(async move {
            if healthy.load(Ordering::Acquire) {
                Ok(())
            } else {
                Err(format!("{label} is unhealthy"))
            }
        })
    }
}

static PROCESS_READINESS_CHECKS: OnceLock<Mutex<Vec<Arc<dyn ReadinessCheck>>>> = OnceLock::new();

pub fn register_im_process_boolean_readiness_check(
    label: impl Into<String>,
    healthy: Arc<AtomicBool>,
) -> Result<(), String> {
    let label = label.into();
    if label.trim().is_empty() {
        return Err("readiness check label must not be empty".into());
    }
    PROCESS_READINESS_CHECKS
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .map_err(|_| "process readiness registry lock is poisoned".to_string())?
        .push(Arc::new(BooleanReadinessCheck { label, healthy }));
    Ok(())
}

fn registered_im_process_readiness_checks() -> Vec<Arc<dyn ReadinessCheck>> {
    PROCESS_READINESS_CHECKS
        .get()
        .and_then(|checks| checks.lock().ok().map(|checks| checks.clone()))
        .unwrap_or_default()
}

/// Collapses required IM process checks into one fail-closed readiness check.
///
/// Dependency details returned by these checks are server-log data. HTTP callers
/// must use `sdkwork-web-bootstrap`, which replaces them with the canonical
/// client-safe readiness failure detail.
pub fn compose_im_required_readiness_checks(
    mut checks: Vec<Arc<dyn ReadinessCheck>>,
) -> Arc<dyn ReadinessCheck> {
    match checks.len() {
        0 => Arc::new(MissingDependencyReadinessCheck::new(
            "process readiness checks",
        )),
        1 => checks.pop().expect("single readiness check"),
        _ => Arc::new(CompositeReadinessCheck::new(checks)),
    }
}

/// Runs all required startup work before a process claims its TCP address.
///
/// A failed dependency preflight must leave the configured port available for
/// a corrected process invocation. Hosts use this for database/bootstrap,
/// generated dependency SDK validation, and durable relay readiness before
/// accepting any network traffic.
pub async fn complete_preflight_then_bind_tcp_listener<T, F>(
    bind_addr: &str,
    service_name: &str,
    preflight: F,
) -> Result<(T, tokio::net::TcpListener), String>
where
    F: Future<Output = Result<T, String>>,
{
    let prepared = preflight.await?;
    let listener = tokio::net::TcpListener::bind(bind_addr)
        .await
        .map_err(|error| format!("{service_name} failed to bind listener: {error}"))?;
    Ok((prepared, listener))
}

pub async fn resolve_im_service_readiness_check() -> Arc<dyn ReadinessCheck> {
    let environment = resolve_web_environment_from_process_env();
    let mut checks: Vec<Arc<dyn ReadinessCheck>> = Vec::new();

    if let Some(pool) = resolve_iam_auth_pool_from_env().await {
        checks.push(Arc::new(PgPoolReadinessCheck { pool, label: "iam" }));
    }

    match sdkwork_im_database_pool::create_im_database_pool_from_env().await {
        Ok(pool) => checks.push(Arc::new(DatabasePoolReadinessCheck { pool })),
        Err(_) => {
            checks.push(Arc::new(UnavailableDependencyReadinessCheck::new(
                "im database",
            )));
        }
    }

    match resolve_im_redis_url_from_env() {
        Some(url) => checks.push(Arc::new(RedisUrlReadinessCheck { url })),
        None if redis_required_in_production(&environment) => {
            checks.push(Arc::new(MissingDependencyReadinessCheck::new("redis")));
        }
        None => {}
    }
    checks.extend(registered_im_process_readiness_checks());

    compose_im_required_readiness_checks(checks)
}

pub async fn resolve_gateway_readiness_check() -> Arc<dyn ReadinessCheck> {
    resolve_im_service_readiness_check().await
}

/// Adds gateway-owned runtime checks to the canonical IM dependency checks.
///
/// The standalone gateway uses this for embedded domain state and worker
/// lifecycles that are not represented by database or Redis connectivity alone.
pub async fn resolve_gateway_readiness_check_with_required_checks(
    required_checks: Vec<Arc<dyn ReadinessCheck>>,
) -> Arc<dyn ReadinessCheck> {
    let mut checks = Vec::with_capacity(required_checks.len() + 1);
    checks.push(resolve_im_service_readiness_check().await);
    checks.extend(required_checks);
    compose_im_required_readiness_checks(checks)
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

/// Graceful shutdown signal for IM services.
///
/// Waits for SIGTERM or SIGINT (Ctrl+C). On Unix both signals initiate
/// graceful drain; on Windows Ctrl+C is used.
pub async fn shutdown_signal() {
    #[cfg(unix)]
    {
        use tokio::signal::unix::{SignalKind, signal};

        let mut sigterm = signal(SignalKind::terminate())
            .expect("failed to install SIGTERM handler for graceful shutdown");
        let mut sigint = signal(SignalKind::interrupt())
            .expect("failed to install SIGINT handler for graceful shutdown");

        tokio::select! {
            _ = sigterm.recv() => {}
            _ = sigint.recv() => {}
        }
    }

    #[cfg(not(unix))]
    {
        let _ = tokio::signal::ctrl_c().await;
    }
}

#[cfg(test)]
mod identity_tests {
    use super::*;

    #[test]
    fn ensure_im_service_process_identity_sets_defaults_when_unset() {
        let prior_service = std::env::var("SDKWORK_IM_SERVICE_NAME").ok();
        let prior_otel = std::env::var("OTEL_SERVICE_NAME").ok();
        // SAFETY: Tests are single-threaded; mutating the process environment here
        // cannot race with other threads in this binary.
        unsafe {
            std::env::remove_var("SDKWORK_IM_SERVICE_NAME");
            std::env::remove_var("OTEL_SERVICE_NAME");
        }
        ensure_im_service_process_identity("test-service");
        assert_eq!(
            std::env::var("SDKWORK_IM_SERVICE_NAME").ok().as_deref(),
            Some("test-service")
        );
        assert_eq!(
            std::env::var("OTEL_SERVICE_NAME").ok().as_deref(),
            Some("test-service")
        );
        // SAFETY: see above.
        unsafe {
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

    #[tokio::test]
    async fn failed_preflight_does_not_claim_the_configured_listener_port() {
        let reservation = tokio::net::TcpListener::bind("127.0.0.1:0")
            .await
            .expect("reserve test port");
        let bind_addr = reservation
            .local_addr()
            .expect("resolve reserved test port")
            .to_string();
        drop(reservation);

        let failure = complete_preflight_then_bind_tcp_listener(
            bind_addr.as_str(),
            "readiness-test",
            async { Err::<(), _>("knowledgebase lifecycle preflight failed".to_owned()) },
        )
        .await;

        assert!(failure.is_err());
        let rebound = tokio::net::TcpListener::bind(bind_addr.as_str())
            .await
            .expect("failed preflight must leave the configured port unclaimed");
        drop(rebound);
    }
}

#[cfg(test)]
mod composition_tests {
    use super::*;

    #[derive(Clone)]
    struct FixedReadinessCheck(Result<(), String>);

    impl ReadinessCheck for FixedReadinessCheck {
        fn check(&self) -> ReadinessFuture<'_> {
            let result = self.0.clone();
            Box::pin(async move { result })
        }
    }

    #[tokio::test]
    async fn required_readiness_failure_propagates_from_composite() {
        let check = compose_im_required_readiness_checks(vec![
            Arc::new(FixedReadinessCheck(Ok(()))),
            Arc::new(FixedReadinessCheck(Err(
                "embedded agents state is unavailable".to_owned(),
            ))),
        ]);

        let error = check
            .check()
            .await
            .expect_err("a required dependency failure must fail readiness");
        assert_eq!(error, "embedded agents state is unavailable");
    }

    #[tokio::test]
    async fn empty_required_readiness_set_fails_closed() {
        let error = compose_im_required_readiness_checks(Vec::new())
            .check()
            .await
            .expect_err("an empty required readiness set must not report ready");
        assert!(error.contains("process readiness checks"));
    }
}
