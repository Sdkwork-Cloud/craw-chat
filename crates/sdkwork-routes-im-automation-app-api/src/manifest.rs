use sdkwork_web_contract::{HttpMethod, HttpRoute};
use sdkwork_web_core::HttpRouteManifest;

use crate::paths;

/// API surface: app-api
pub const API_SURFACE: &str = "app-api";

pub const ROUTES: &[HttpRoute] = &[
    HttpRoute::dual_token(
        HttpMethod::Post,
        paths::EXECUTIONS,
        "automation",
        "automation.executions.create",
    ),
    HttpRoute::dual_token(
        HttpMethod::Get,
        paths::EXECUTION,
        "automation",
        "automation.executions.retrieve",
    ),
    HttpRoute::dual_token(
        HttpMethod::Post,
        paths::AGENT_RESPONSES,
        "automation",
        "automation.agentResponses.create",
    ),
    HttpRoute::dual_token(
        HttpMethod::Post,
        paths::AGENT_RESPONSE_FRAMES,
        "automation",
        "automation.agentResponses.frames.create",
    ),
    HttpRoute::dual_token(
        HttpMethod::Post,
        paths::AGENT_RESPONSE_COMPLETE,
        "automation",
        "automation.agentResponses.complete.create",
    ),
    HttpRoute::dual_token(
        HttpMethod::Post,
        paths::AGENT_TOOL_CALLS,
        "automation",
        "automation.agentToolCalls.create",
    ),
    HttpRoute::dual_token(
        HttpMethod::Post,
        paths::EXECUTION_TOOL_CALL_COMPLETE,
        "automation",
        "automation.executions.agentToolCalls.complete.create",
    ),
    HttpRoute::dual_token(
        HttpMethod::Get,
        paths::GOVERNANCE,
        "automation",
        "automation.governance.retrieve",
    ),
];

pub fn route_manifest() -> HttpRouteManifest {
    HttpRouteManifest::new(ROUTES)
}
