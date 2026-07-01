use sdkwork_web_contract::{HttpMethod, HttpRoute};
use sdkwork_web_core::HttpRouteManifest;

use crate::paths;

/// API surface: open-api
pub const API_SURFACE: &str = "open-api";

pub const ROUTES: &[HttpRoute] = &[
    HttpRoute::dual_token(
        HttpMethod::Get,
        paths::PREFIX,
        "projection",
        "projection.prefix",
    ),
    HttpRoute::dual_token(
        HttpMethod::Get,
        paths::MESSAGE_FAVORITES,
        "chat",
        "messages.favorites.list",
    ),
    HttpRoute::dual_token(
        HttpMethod::Post,
        paths::MESSAGE_FAVORITE_CREATE,
        "chat",
        "messages.favorites.create",
    )
        .with_idempotent(true),
    HttpRoute::dual_token(
        HttpMethod::Delete,
        paths::MESSAGE_FAVORITE,
        "chat",
        "messages.favorites.delete",
    )
        .with_idempotent(true),
];

pub fn route_manifest() -> HttpRouteManifest {
    HttpRouteManifest::new(ROUTES)
}
