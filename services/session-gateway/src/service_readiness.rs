use axum::Json;
use axum::extract::State;
use serde::Serialize;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use crate::AppState;
use crate::api_error::ApiError;

const REDIS_READINESS_TIMEOUT: Duration = Duration::from_secs(1);

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
            postgres_configured: std::env::var("SDKWORK_IM_DATABASE_URL")
                .ok()
                .map(|value| !value.trim().is_empty())
                .unwrap_or(false),
            draining: Arc::new(AtomicBool::new(false)),
        }
    }

    pub async fn is_ready(&self) -> bool {
        if self.is_draining() {
            return false;
        }
        if let Some(redis_url) = self.redis_url.as_deref() {
            return ping_redis(redis_url).await;
        }
        true
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
        self.postgres_configured
    }
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
    use super::ServiceReadiness;

    #[tokio::test]
    async fn draining_readiness_is_shared_and_fail_closed() {
        let readiness = ServiceReadiness::default();
        let observer = readiness.clone();
        assert!(observer.is_ready().await);

        readiness.mark_draining();

        assert!(observer.is_draining());
        assert!(!observer.is_ready().await);
    }
}
