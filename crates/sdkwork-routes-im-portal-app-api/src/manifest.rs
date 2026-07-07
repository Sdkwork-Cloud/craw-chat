use sdkwork_web_contract::{HttpMethod, HttpRoute};
use sdkwork_web_core::HttpRouteManifest;

use crate::paths;

/// API surface: app-api
pub const API_SURFACE: &str = "app-api";

pub const ROUTES: &[HttpRoute] = &[
    HttpRoute::dual_token(
        HttpMethod::Get,
        paths::PORTAL_ACCESS,
        "portal",
        "portal.access.retrieve",
    ),
    HttpRoute::dual_token(
        HttpMethod::Get,
        paths::PORTAL_WORKSPACE,
        "portal",
        "portal.workspace.retrieve",
    ),
    HttpRoute::dual_token(
        HttpMethod::Get,
        paths::PORTAL_SECTION,
        "portal",
        "portal.snapshot.retrieve",
    ),
];

pub fn route_manifest() -> HttpRouteManifest {
    HttpRouteManifest::new(ROUTES)
}
