use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use axum::Json;
use axum::extract::State;
use serde::Serialize;
use sqlx::PgPool;

use crate::AppState;
use crate::api_error::ApiError;

const REDIS_READINESS_TIMEOUT: Duration = Duration::from_secs(1);
const POSTGRES_READINESS_TIMEOUT: Duration = Duration::from_secs(2);

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReadinessResponse {
    pub status: &'static str,
    pub service: &'static str,
    pub redis: Option<&'static str>,
    pub postgres: Option<&'static str>,
}

#[derive(Clone, Default)]
pub struct ServiceReadiness {
    redis_url: Option<String>,
    postgres_configured: bool,
    iam_auth_pool: Option<Arc<PgPool>>,
    draining: Arc<AtomicBool>,
}

impl ServiceReadiness {
    pub fn from_env() -> Self {
        Self {
            redis_url: std::env::var("SDKWORK_IM_REALTIME_ROUTE_STORE_URL")
                .ok()
                .or_else(|| std::env::var("SDKWORK_IM_REALTIME_CLUSTER_BUS_URL").ok())
                .map(|value| value.trim().to_owned())
                .filter(|value| !value.is_empty()),
            postgres_configured: std::env::var("SDKWORK_DATABASE_URL")
                .ok()
                .map(|value| !value.trim().is_empty())
                .unwrap_or(false),
            iam_auth_pool: None,
            draining: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Attaches the IAM auth pool so `/readyz` performs a real `SELECT 1`
    /// probe instead of trusting the `SDKWORK_DATABASE_URL` env flag alone.
    ///
    /// When the pool is present the probe is authoritative: a failed probe
    /// makes the gateway report `not_ready` (503). When no IAM pool is
    /// configured (dev/private deployments without an IAM database) the
    /// legacy env-based behavior is preserved because there is nothing to
    /// probe.
    pub fn with_iam_auth_pool(mut self, pool: Option<Arc<PgPool>>) -> Self {
        self.iam_auth_pool = pool;
        self
    }

    pub async fn is_ready(&self) -> bool {
        if self.is_draining() {
            return false;
        }
        let redis_ready = match self.redis_url.as_deref() {
            Some(redis_url) => ping_redis(redis_url).await,
            None => true,
        };
        let postgres_ready = match self.iam_auth_pool.as_deref() {
            Some(pool) => ping_postgres_pool(pool).await,
            // No IAM auth pool configured: keep the legacy env-flag behavior.
            None => true,
        };
        combine_dependency_readiness(redis_ready, postgres_ready)
    }

    pub fn mark_draining(&self) {
        self.draining.store(true, Ordering::Release);
    }

    pub fn is_draining(&self) -> bool {
        self.draining.load(Ordering::Acquire)
    }

    pub fn redis_url(&self) -> Option<&str> {
        self.redis_url.as_deref()
    }

    pub fn postgres_configured(&self) -> bool {
        self.postgres_configured || self.iam_auth_pool.is_some()
    }
}

/// Collapses the per-dependency probe results into one fail-closed readiness
/// decision. Kept as a pure function so the decision logic is unit-testable
/// without a live Redis or PostgreSQL.
fn combine_dependency_readiness(redis_ready: bool, postgres_ready: bool) -> bool {
    redis_ready && postgres_ready
}

async fn ping_postgres_pool(pool: &PgPool) -> bool {
    tokio::time::timeout(
        POSTGRES_READINESS_TIMEOUT,
        sqlx::query("SELECT 1").execute(pool),
    )
    .await
    .map(|result| result.is_ok())
    .unwrap_or(false)
}

async fn ping_redis(redis_url: &str) -> bool {
    let Ok(client) = redis::Client::open(redis_url) else {
        return false;
    };
    let Ok(Ok(mut connection)) = tokio::time::timeout(
        REDIS_READINESS_TIMEOUT,
        client.get_multiplexed_async_connection(),
    )
    .await
    else {
        return false;
    };
    tokio::time::timeout(
        REDIS_READINESS_TIMEOUT,
        redis::cmd("PING").query_async::<String>(&mut connection),
    )
    .await
    .ok()
    .and_then(Result::ok)
    .is_some_and(|response| response.eq_ignore_ascii_case("PONG"))
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HealthResponse {
    pub status: &'static str,
    pub service: &'static str,
}

pub async fn healthz() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok",
        service: "session-gateway",
    })
}

pub async fn readyz(State(state): State<AppState>) -> Result<Json<ReadinessResponse>, ApiError> {
    let ready = state.readiness.is_ready().await;
    let response = ReadinessResponse {
        status: if ready { "ready" } else { "not_ready" },
        service: "session-gateway",
        redis: state.readiness.redis_url().map(|_| "configured"),
        postgres: if state.readiness.postgres_configured() {
            Some("configured")
        } else {
            None
        },
    };
    if ready {
        Ok(Json(response))
    } else {
        Err(ApiError {
            status: axum::http::StatusCode::SERVICE_UNAVAILABLE,
            code: "not_ready",
            message: if state.readiness.is_draining() {
                "session-gateway is draining".to_owned()
            } else {
                "session-gateway dependencies are not ready".to_owned()
            },
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn draining_readiness_is_shared_and_fail_closed() {
        let readiness = ServiceReadiness::default();
        let observer = readiness.clone();
        assert!(observer.is_ready().await);

        readiness.mark_draining();

        assert!(observer.is_draining());
        assert!(!observer.is_ready().await);
    }

    #[tokio::test]
    async fn postgres_probe_failure_fails_closed_even_when_redis_is_healthy() {
        // A failed real PostgreSQL probe must make /readyz report not_ready
        // (503) regardless of the Redis outcome — this is the P1 fix for the
        // former env-flag-only fake check.
        assert!(!combine_dependency_readiness(true, false));
        assert!(!combine_dependency_readiness(false, false));
    }

    #[tokio::test]
    async fn healthy_probes_report_ready_and_redis_failure_is_fail_closed() {
        assert!(combine_dependency_readiness(true, true));
        assert!(!combine_dependency_readiness(false, true));
    }

    #[test]
    fn attaching_a_pool_marks_postgres_configured_without_pool_creation() {
        // with_iam_auth_pool(None) must not regress the env-flag behavior, and
        // postgres_configured() reflects the attached pool so the /readyz
        // response payload stays truthful about the wiring.
        let readiness = ServiceReadiness::from_env().with_iam_auth_pool(None);
        assert_eq!(
            readiness.postgres_configured(),
            std::env::var("SDKWORK_DATABASE_URL")
                .map(|value| !value.trim().is_empty())
                .unwrap_or(false)
        );
    }
}
