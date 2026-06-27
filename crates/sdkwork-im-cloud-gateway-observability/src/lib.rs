use std::collections::{BTreeMap, BTreeSet};

use sdkwork_im_api_registry::{
    ContractKind, HttpMethod, RouteProtocol, RouteRegistry, RouteVisibility, SdkContractSummary,
    SdkTarget, sdk_contract_summaries,
};
use sdkwork_im_cloud_gateway_config::{GatewayRuntimeMode, WebGatewayConfig};
use serde::Serialize;

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GatewayServiceContractSummary {
    pub service_id: String,
    pub contract_kind: ContractKind,
    pub schema_url: String,
    pub docs_url: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GatewayPublicEndpointSummary {
    pub service_id: String,
    pub path_pattern: String,
    pub protocol: RouteProtocol,
    pub visibility: RouteVisibility,
    pub methods: Vec<HttpMethod>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GatewayRouteSummary {
    pub service_id: String,
    pub operation_group: String,
    pub visibility: RouteVisibility,
    pub path_pattern: String,
    pub methods: Vec<HttpMethod>,
    pub protocol: RouteProtocol,
    pub sdk_targets: Vec<SdkTarget>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub websocket_subprotocols: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GatewaySurfaceGroupSummary {
    pub service_id: String,
    pub operation_group: String,
    pub visibility: RouteVisibility,
    pub route_count: usize,
    pub sdk_targets: Vec<SdkTarget>,
    pub protocols: Vec<RouteProtocol>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub websocket_subprotocols: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GatewayStartupSummary {
    pub bind_addr: String,
    pub base_url: String,
    pub portal_url: String,
    pub admin_url: String,
    pub aggregate_openapi_url: String,
    pub openapi_index_url: String,
    pub runtime_summary_url: String,
    pub docs_url: String,
    pub runtime_mode: GatewayRuntimeMode,
    pub sdk_contracts: Vec<SdkContractSummary>,
    pub upstreams: Vec<(String, String)>,
    pub service_contracts: Vec<GatewayServiceContractSummary>,
    pub public_endpoints: Vec<GatewayPublicEndpointSummary>,
    pub surface_groups: Vec<GatewaySurfaceGroupSummary>,
}

pub fn build_startup_summary(
    config: &WebGatewayConfig,
    base_url: impl Into<String>,
) -> GatewayStartupSummary {
    build_startup_summary_with_registry(config, &RouteRegistry::default(), base_url)
}

pub fn build_startup_summary_with_registry(
    config: &WebGatewayConfig,
    registry: &RouteRegistry,
    base_url: impl Into<String>,
) -> GatewayStartupSummary {
    let base_url = base_url.into();
    GatewayStartupSummary {
        bind_addr: config.bind_addr.clone(),
        base_url: base_url.clone(),
        portal_url: format!("{}/", base_url),
        admin_url: format!("{}/admin/", base_url),
        aggregate_openapi_url: format!("{}/openapi.json", base_url),
        openapi_index_url: format!("{}/openapi/index.json", base_url),
        runtime_summary_url: format!("{}/openapi/runtime-summary.json", base_url),
        docs_url: format!("{}/docs", base_url),
        runtime_mode: config.runtime_mode.clone(),
        sdk_contracts: sdk_contract_summaries(base_url.as_str()),
        upstreams: config
            .upstreams
            .iter()
            .map(|item| (item.service_id.clone(), item.base_url.clone()))
            .collect(),
        service_contracts: config
            .upstreams
            .iter()
            .map(|item| GatewayServiceContractSummary {
                service_id: item.service_id.clone(),
                contract_kind: ContractKind::UpstreamOperational,
                schema_url: format!(
                    "{}/openapi/services/{}.openapi.json",
                    base_url, item.service_id
                ),
                docs_url: format!("{}/docs/services/{}", base_url, item.service_id),
            })
            .collect(),
        public_endpoints: public_endpoint_summaries(registry),
        surface_groups: surface_group_summaries(registry),
    }
}

pub fn format_startup_summary(summary: &GatewayStartupSummary) -> String {
    let mode = match summary.runtime_mode {
        GatewayRuntimeMode::Split => "split",
        GatewayRuntimeMode::Unified => "unified",
    };

    let mut lines = vec![
        "Mode".to_owned(),
        format!("  runtime: {mode}"),
        "Bind Summary".to_owned(),
        format!("  bind: {}", summary.bind_addr),
        "Unified Access".to_owned(),
        format!("  base: {}", summary.base_url),
        format!("  portal: {}", summary.portal_url),
        format!("  admin: {}", summary.admin_url),
        format!("  healthz: {}/healthz", summary.base_url),
        format!("  readyz: {}/readyz", summary.base_url),
        "OpenAPI 3.1 Schemas".to_owned(),
        format!("  aggregate: {}", summary.aggregate_openapi_url),
        format!("  index: {}", summary.openapi_index_url),
        format!("  runtime summary: {}", summary.runtime_summary_url),
        format!("  docs: {}", summary.docs_url),
    ];

    if !summary.sdk_contracts.is_empty() {
        lines.push("SDK Contracts".to_owned());
        for contract in &summary.sdk_contracts {
            lines.push(format!(
                "  {} schema: {} [sdk:{}] [prefix:{}]",
                contract.group_id,
                contract.schema_url,
                format_sdk_target(contract.sdk_target),
                contract.api_prefix
            ));
        }
    }

    lines.push("Upstream Status".to_owned());

    for (service_id, base_url) in &summary.upstreams {
        lines.push(format!("  {service_id}: {base_url}"));
    }

    if !summary.public_endpoints.is_empty() {
        lines.push("Gateway Endpoints".to_owned());
        for endpoint in &summary.public_endpoints {
            lines.push(format!(
                "  {} {} {} [{}]: {}",
                format_visibility(endpoint.visibility),
                format_protocol(endpoint.protocol),
                endpoint.service_id,
                format_methods(endpoint.methods.as_slice()),
                endpoint.path_pattern
            ));
        }
    }

    if !summary.surface_groups.is_empty() {
        lines.push("Gateway Surface Groups".to_owned());
        for group in &summary.surface_groups {
            lines.push(format!(
                "  {} {} {} [sdk:{}] [protocols:{}]: {} routes",
                format_visibility(group.visibility),
                group.service_id,
                group.operation_group,
                format_sdk_targets(group.sdk_targets.as_slice()),
                format_protocols(group.protocols.as_slice()),
                group.route_count
            ));
        }
    }

    lines.join("\n")
}

pub fn route_summaries(registry: &RouteRegistry) -> Vec<GatewayRouteSummary> {
    let mut routes = registry
        .entries()
        .iter()
        .map(|entry| GatewayRouteSummary {
            service_id: entry.service_id.clone(),
            operation_group: entry.operation_group.clone(),
            visibility: entry.visibility,
            path_pattern: entry.path_pattern.clone(),
            methods: entry.methods.clone(),
            protocol: entry.protocol,
            sdk_targets: entry.sdk_targets.clone(),
            websocket_subprotocols: entry.websocket_subprotocols.clone(),
        })
        .collect::<Vec<_>>();

    routes.sort_by(|left, right| {
        left.path_pattern
            .cmp(&right.path_pattern)
            .then(left.protocol.cmp(&right.protocol))
            .then(left.service_id.cmp(&right.service_id))
            .then(left.operation_group.cmp(&right.operation_group))
    });

    routes
}

fn public_endpoint_summaries(registry: &RouteRegistry) -> Vec<GatewayPublicEndpointSummary> {
    let mut endpoints = registry
        .entries()
        .iter()
        .filter(|entry| entry.visibility != RouteVisibility::Internal)
        .map(|entry| GatewayPublicEndpointSummary {
            service_id: entry.service_id.clone(),
            path_pattern: entry.path_pattern.clone(),
            protocol: entry.protocol,
            visibility: entry.visibility,
            methods: entry.methods.clone(),
        })
        .collect::<Vec<_>>();

    endpoints.sort_by(|left, right| {
        left.path_pattern
            .cmp(&right.path_pattern)
            .then(left.protocol.cmp(&right.protocol))
            .then(left.service_id.cmp(&right.service_id))
    });

    endpoints
}

pub fn surface_group_summaries(registry: &RouteRegistry) -> Vec<GatewaySurfaceGroupSummary> {
    let mut grouped = BTreeMap::<
        (RouteVisibility, String, String),
        (
            usize,
            BTreeSet<SdkTarget>,
            BTreeSet<RouteProtocol>,
            BTreeSet<String>,
        ),
    >::new();

    for entry in registry.entries() {
        let (route_count, sdk_targets, protocols, websocket_subprotocols) = grouped
            .entry((
                entry.visibility,
                entry.service_id.clone(),
                entry.operation_group.clone(),
            ))
            .or_insert_with(|| (0, BTreeSet::new(), BTreeSet::new(), BTreeSet::new()));
        *route_count += 1;
        sdk_targets.extend(entry.sdk_targets.iter().copied());
        protocols.insert(entry.protocol);
        websocket_subprotocols.extend(entry.websocket_subprotocols.iter().cloned());
    }

    grouped
        .into_iter()
        .map(
            |(
                (visibility, service_id, operation_group),
                (route_count, sdk_targets, protocols, websocket_subprotocols),
            )| {
                GatewaySurfaceGroupSummary {
                    service_id,
                    operation_group,
                    visibility,
                    route_count,
                    sdk_targets: sdk_targets.into_iter().collect(),
                    protocols: protocols.into_iter().collect(),
                    websocket_subprotocols: websocket_subprotocols.into_iter().collect(),
                }
            },
        )
        .collect()
}

fn format_visibility(visibility: RouteVisibility) -> &'static str {
    match visibility {
        RouteVisibility::Public => "public",
        RouteVisibility::Partner => "partner",
        RouteVisibility::Internal => "internal",
    }
}

fn format_protocol(protocol: RouteProtocol) -> &'static str {
    match protocol {
        RouteProtocol::Http => "http",
        RouteProtocol::Websocket => "websocket",
    }
}

fn format_methods(methods: &[HttpMethod]) -> String {
    methods
        .iter()
        .map(|method| match method {
            HttpMethod::Delete => "DELETE",
            HttpMethod::Get => "GET",
            HttpMethod::Head => "HEAD",
            HttpMethod::Options => "OPTIONS",
            HttpMethod::Patch => "PATCH",
            HttpMethod::Post => "POST",
            HttpMethod::Put => "PUT",
        })
        .collect::<Vec<_>>()
        .join(",")
}

fn format_sdk_targets(sdk_targets: &[SdkTarget]) -> String {
    sdk_targets
        .iter()
        .map(|sdk_target| format_sdk_target(*sdk_target))
        .collect::<Vec<_>>()
        .join(",")
}

fn format_sdk_target(sdk_target: SdkTarget) -> &'static str {
    match sdk_target {
        SdkTarget::SdkworkImSdk => "sdkworkImSdk",
        SdkTarget::SdkworkImAppSdk => "sdkworkImAppSdk",
        SdkTarget::SdkworkImBackendSdk => "sdkworkImBackendSdk",
        SdkTarget::SdkworkDriveAppSdk => "sdkworkDriveAppSdk",
        SdkTarget::SdkworkNotaryAppSdk => "sdkworkNotaryAppSdk",
        SdkTarget::SdkworkCommerceAppSdk => "sdkworkCommerceAppSdk",
        SdkTarget::SdkworkMailAppSdk => "sdkworkMailAppSdk",
        SdkTarget::SdkworkCommunityAppSdk => "sdkworkCommunityAppSdk",
        SdkTarget::SdkworkCourseAppSdk => "sdkworkCourseAppSdk",
        SdkTarget::SdkworkKnowledgebaseAppSdk => "sdkworkKnowledgebaseAppSdk",
        SdkTarget::None => "none",
    }
}

fn format_protocols(protocols: &[RouteProtocol]) -> String {
    protocols
        .iter()
        .map(|protocol| match protocol {
            RouteProtocol::Http => "http",
            RouteProtocol::Websocket => "websocket",
        })
        .collect::<Vec<_>>()
        .join(",")
}
