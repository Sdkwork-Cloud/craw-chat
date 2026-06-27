use axum::extract::State;
use axum::Json;
use serde::Serialize;

use crate::api_error::ApiError;
use crate::AppState;

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
        }
    }

    pub fn is_ready(&self) -> bool {
        if let Some(redis_url) = self.redis_url.as_deref() {
            return ping_redis(redis_url);
        }
        true
    }

    pub fn redis_url(&self) -> Option<&str> {
        self.redis_url.as_deref()
    }

    pub fn postgres_configured(&self) -> bool {
        self.postgres_configured
    }
}

fn ping_redis(redis_url: &str) -> bool {
    redis::Client::open(redis_url)
        .ok()
        .and_then(|client| client.get_connection().ok())
        .and_then(|mut connection| redis::cmd("PING").query::<String>(&mut connection).ok())
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
    let ready = state.readiness.is_ready();
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
            message: "session-gateway dependencies are not ready".to_owned(),
        })
    }
}
