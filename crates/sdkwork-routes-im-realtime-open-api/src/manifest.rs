use sdkwork_web_contract::{HttpMethod, HttpRoute};
use sdkwork_web_core::HttpRouteManifest;

use crate::paths;

/// API surface: open-api
pub const API_SURFACE: &str = "open-api";

pub const ROUTES: &[HttpRoute] = &[
    HttpRoute::api_key_or_dual_token(
        HttpMethod::Get,
        paths::PREFIX,
        "realtime",
        "realtime.prefix",
    ),
    // HTTP upgrade is anonymous; credentials arrive in the first `auth.init` websocket frame
    // (see docs/架构/20-WebSocket实时传输绑定标准.md).
    HttpRoute::public(
        HttpMethod::Get,
        paths::REALTIME_WS,
        "realtime",
        "realtime.ws.retrieve",
    ),
    HttpRoute::api_key_or_dual_token(
        HttpMethod::Post,
        paths::REALTIME_SUBSCRIPTIONS_SYNC,
        "realtime",
        "realtime.subscriptions.sync",
    ),
    HttpRoute::api_key_or_dual_token(
        HttpMethod::Post,
        paths::REALTIME_EVENTS_ACK,
        "realtime",
        "realtime.events.ack",
    ),
    HttpRoute::api_key_or_dual_token(
        HttpMethod::Get,
        paths::REALTIME_EVENTS,
        "realtime",
        "realtime.events.list",
    ),
    HttpRoute::api_key_or_dual_token(
        HttpMethod::Post,
        paths::PRESENCE_HEARTBEAT,
        "presence",
        "presence.heartbeat",
    ),
    HttpRoute::api_key_or_dual_token(
        HttpMethod::Get,
        paths::PRESENCE_ME,
        "presence",
        "presence.me.retrieve",
    ),
];

pub fn route_manifest() -> HttpRouteManifest {
    HttpRouteManifest::new(ROUTES)
}
