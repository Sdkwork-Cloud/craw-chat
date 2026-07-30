use sdkwork_web_contract::{HttpMethod, HttpRoute};
use sdkwork_web_core::HttpRouteManifest;

use crate::paths;

/// API surface: backend-api
pub const API_SURFACE: &str = "backend-api";

pub const ROUTES: &[HttpRoute] = &[
    HttpRoute::dual_token(
        HttpMethod::Get,
        paths::AUTOMATION_GOVERNANCE,
        "automation",
        "automation.governance.retrieve",
    ),
    HttpRoute::dual_token(
        HttpMethod::Get,
        paths::PROTOCOL_REGISTRY,
        "governance",
        "governance.protocolRegistry.retrieve",
    ),
    HttpRoute::dual_token(
        HttpMethod::Get,
        paths::PROTOCOL_GOVERNANCE,
        "governance",
        "governance.protocolGovernance.retrieve",
    ),
    HttpRoute::dual_token(
        HttpMethod::Get,
        paths::PROVIDER_REGISTRY,
        "governance",
        "governance.providerRegistry.retrieve",
    ),
    HttpRoute::dual_token(
        HttpMethod::Get,
        paths::PROVIDER_BINDINGS,
        "governance",
        "governance.providerBindings.list",
    ),
    HttpRoute::dual_token(
        HttpMethod::Post,
        paths::PROVIDER_BINDINGS,
        "governance",
        "governance.providerBindings.upsert",
    ),
    HttpRoute::dual_token(
        HttpMethod::Get,
        paths::PROVIDER_POLICIES,
        "governance",
        "governance.providerPolicies.list",
    ),
    HttpRoute::dual_token(
        HttpMethod::Get,
        paths::PROVIDER_POLICIES_DIFF,
        "governance",
        "governance.providerPolicies.diff.retrieve",
    ),
    HttpRoute::dual_token(
        HttpMethod::Post,
        paths::PROVIDER_POLICIES_PREVIEW,
        "governance",
        "governance.providerPolicies.preview.create",
    ),
    HttpRoute::dual_token(
        HttpMethod::Post,
        paths::PROVIDER_POLICIES_ROLLBACK,
        "governance",
        "governance.providerPolicies.rollback.create",
    ),
    HttpRoute::dual_token(
        HttpMethod::Post,
        paths::NODE_DRAIN,
        "governance",
        "governance.nodes.drain.create",
    ),
    HttpRoute::dual_token(
        HttpMethod::Post,
        paths::NODE_ACTIVATE,
        "governance",
        "governance.nodes.activate.create",
    ),
    HttpRoute::dual_token(
        HttpMethod::Post,
        paths::NODE_ROUTES_MIGRATE,
        "governance",
        "governance.nodes.routes.migrate.create",
    ),
];

pub fn route_manifest() -> HttpRouteManifest {
    HttpRouteManifest::new(ROUTES)
}
