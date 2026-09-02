use sdkwork_web_contract::{HttpMethod, HttpRoute};
use sdkwork_web_core::HttpRouteManifest;

use crate::paths;

/// API surface: backend-api
pub const API_SURFACE: &str = "backend-api";

pub const ROUTES: &[HttpRoute] = &[
    HttpRoute::dual_token(HttpMethod::Get, paths::HEALTH, "ops", "health.retrieve"),
    HttpRoute::dual_token(HttpMethod::Get, paths::CLUSTER, "ops", "cluster.retrieve"),
    HttpRoute::dual_token(HttpMethod::Get, paths::LAG, "ops", "lag.retrieve"),
    HttpRoute::dual_token(
        HttpMethod::Get,
        paths::REPLAY_STATUS,
        "ops",
        "replayStatus.retrieve",
    ),
    HttpRoute::dual_token(
        HttpMethod::Get,
        paths::RUNTIME_DIR,
        "ops",
        "runtimeDir.retrieve",
    ),
    HttpRoute::dual_token(
        HttpMethod::Get,
        paths::PROVIDER_BINDINGS,
        "ops",
        "ops.providerBindings.list",
    ),
    HttpRoute::dual_token(
        HttpMethod::Get,
        paths::PROVIDER_BINDINGS_DRIFT,
        "ops",
        "ops.providerBindings.drift.retrieve",
    ),
    HttpRoute::dual_token(
        HttpMethod::Get,
        paths::DIAGNOSTICS,
        "ops",
        "diagnostics.retrieve",
    ),
    HttpRoute::dual_token(
        HttpMethod::Post,
        paths::RETENTION_PURGE,
        "ops",
        "retention.purge",
    ),
];

pub fn route_manifest() -> HttpRouteManifest {
    HttpRouteManifest::new(ROUTES)
}
