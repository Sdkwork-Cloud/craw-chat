use sdkwork_web_contract::{HttpMethod, HttpRoute};
use sdkwork_web_core::HttpRouteManifest;

/// API surface: backend-api
pub const API_SURFACE: &str = "backend-api";

const SOCIAL_BACKEND_ROUTES: &[HttpRoute] = &[
    HttpRoute::dual_token(
        HttpMethod::Get,
        "/backend/v3/api/control/social/friend_requests",
        "control",
        "control.social.friendRequests.list",
    ),
    HttpRoute::dual_token(
        HttpMethod::Post,
        "/backend/v3/api/control/social/friend_requests",
        "control",
        "control.social.friendRequests.submit",
    ),
];

pub fn backend_routes() -> Vec<HttpRoute> {
    SOCIAL_BACKEND_ROUTES.to_vec()
}

pub fn backend_route_manifest() -> HttpRouteManifest {
    HttpRouteManifest::new(SOCIAL_BACKEND_ROUTES)
}
