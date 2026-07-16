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
        "access.retrieve",
    )
    .with_required_permission("audit.read"),
    HttpRoute::dual_token(
        HttpMethod::Get,
        paths::PORTAL_AUTOMATION,
        "portal",
        "automation.retrieve",
    ),
    HttpRoute::dual_token(
        HttpMethod::Get,
        paths::PORTAL_CONVERSATIONS,
        "portal",
        "conversationSnapshot.retrieve",
    ),
    HttpRoute::dual_token(
        HttpMethod::Get,
        paths::PORTAL_DASHBOARD,
        "portal",
        "dashboard.retrieve",
    ),
    HttpRoute::dual_token(
        HttpMethod::Get,
        paths::PORTAL_GOVERNANCE,
        "portal",
        "governance.retrieve",
    )
    .with_required_permission("audit.read"),
    HttpRoute::dual_token(
        HttpMethod::Get,
        paths::PORTAL_HOME,
        "portal",
        "home.retrieve",
    ),
    HttpRoute::dual_token(
        HttpMethod::Get,
        paths::PORTAL_MEDIA,
        "portal",
        "media.retrieve",
    ),
    HttpRoute::dual_token(
        HttpMethod::Get,
        paths::PORTAL_REALTIME,
        "portal",
        "realtime.retrieve",
    ),
    HttpRoute::dual_token(
        HttpMethod::Get,
        paths::PORTAL_WORKSPACE,
        "portal",
        "workspace.retrieve",
    ),
];

pub fn route_manifest() -> HttpRouteManifest {
    HttpRouteManifest::new(ROUTES)
}
