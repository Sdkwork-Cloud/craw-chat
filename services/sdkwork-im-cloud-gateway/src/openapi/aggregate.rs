//! Aggregation of upstream service OpenAPI documents into a single gateway
//! contract, plus the service schema index projection.

use std::collections::BTreeSet;
use std::net::{IpAddr, SocketAddr};
use std::time::Duration;

use axum::http::StatusCode;
use axum::response::Response;
use reqwest::Url;
use sdkwork_im_api_registry::{ContractKind, RouteRegistry, ServiceSchemaIndexEntry};
use sdkwork_im_cloud_gateway_config::WebGatewayConfig;
use serde_json::{Map, Value, json};

use super::discovery::{
    gateway_discovery_schema_components, merge_gateway_discovery_openapi, service_visibility,
    visibility_for_service,
};
use crate::response::json_error_response;
use crate::state::GatewayState;

pub(crate) struct ServiceOpenApiDocument {
    service_id: String,
    document: Value,
}

pub(crate) async fn aggregate_openapi_document(
    state: &GatewayState,
    gateway_base_url: &str,
) -> Result<Value, Response> {
    let cache_key = aggregate_openapi_cache_key(state, gateway_base_url);
    state
        .openapi_aggregate_cache
        .get_or_refresh(cache_key.as_str(), || async {
            let documents = fetch_service_openapi_documents(state, gateway_base_url).await?;
            Ok(build_aggregate_openapi_document(&documents))
        })
        .await
}

pub(crate) async fn fetch_service_openapi_documents(
    state: &GatewayState,
    gateway_base_url: &str,
) -> Result<Vec<ServiceOpenApiDocument>, Response> {
    let gateway_openapi_urls = gateway_openapi_url_candidates(&state.config, gateway_base_url);
    let fetches = state.config.upstreams.iter().filter_map(|upstream| {
        let service_id = upstream.service_id.clone();
        if is_self_referential_gateway_openapi_fetch(
            upstream.base_url.as_str(),
            gateway_openapi_urls.as_slice(),
        ) {
            tracing::warn!(
                target: "sdkwork.im.gateway",
                event = "im.gateway.openapi.self_referential_upstream_skipped",
                service_id = %service_id,
                upstream_base_url = %upstream.base_url,
                gateway_base_url,
                "skipping self-referential upstream OpenAPI fetch"
            );
            return None;
        }

        Some(async move {
            (
                service_id.clone(),
                fetch_service_openapi_document(state, service_id.as_str(), Some(gateway_base_url))
                    .await,
            )
        })
    });
    let mut documents = Vec::new();
    for (service_id, result) in futures_util::future::join_all(fetches).await {
        match result {
            Ok(document) => documents.push(ServiceOpenApiDocument {
                service_id,
                document,
            }),
            Err(error) if state.config.strict_startup => return Err(error),
            Err(_) => continue,
        }
    }
    Ok(documents)
}

pub(crate) async fn fetch_service_openapi_document(
    state: &GatewayState,
    service_id: &str,
    gateway_base_url: Option<&str>,
) -> Result<Value, Response> {
    let Some(base_url) = state.config.upstream_base_url(service_id) else {
        return Err(json_error_response(
            StatusCode::NOT_FOUND,
            format!("service schema upstream is not configured for {service_id}").as_str(),
        ));
    };
    if let Some(gateway_base_url) = gateway_base_url {
        let gateway_openapi_urls = gateway_openapi_url_candidates(&state.config, gateway_base_url);
        if is_self_referential_gateway_openapi_fetch(base_url, gateway_openapi_urls.as_slice()) {
            return Err(json_error_response(
                StatusCode::BAD_GATEWAY,
                format!(
                    "service schema upstream for {service_id} points back to this gateway aggregate OpenAPI endpoint"
                )
                .as_str(),
            ));
        }
    }
    let url = format!("{}/openapi.json", base_url.trim_end_matches('/'));
    let response = state
        .client
        .get(url)
        .timeout(Duration::from_secs(2))
        .send()
        .await
        .map_err(|error| {
            json_error_response(
                StatusCode::BAD_GATEWAY,
                format!("failed to fetch upstream schema for {service_id}: {error}").as_str(),
            )
        })?;
    let status = response.status();
    if !status.is_success() {
        return Err(json_error_response(
            StatusCode::BAD_GATEWAY,
            format!("upstream schema request for {service_id} returned {status}").as_str(),
        ));
    }
    response.json::<Value>().await.map_err(|error| {
        json_error_response(
            StatusCode::BAD_GATEWAY,
            format!("failed to decode upstream schema for {service_id}: {error}").as_str(),
        )
    })
}

pub(crate) fn build_aggregate_openapi_document(documents: &[ServiceOpenApiDocument]) -> Value {
    let mut tags = std::collections::BTreeMap::<String, Value>::new();
    let mut paths = Map::new();
    let mut security_schemes = Map::new();
    let mut schemas = gateway_discovery_schema_components();

    for document in documents {
        if let Some(service_tags) = document.document.get("tags").and_then(Value::as_array) {
            for tag in service_tags {
                if let Some(name) = tag.get("name").and_then(Value::as_str) {
                    tags.entry(name.to_owned()).or_insert_with(|| tag.clone());
                }
            }
        }

        if let Some(service_paths) = document.document.get("paths").and_then(Value::as_object) {
            for (path, operations) in service_paths {
                let path_item = paths
                    .entry(path.clone())
                    .or_insert_with(|| Value::Object(Map::new()));
                let path_object = path_item
                    .as_object_mut()
                    .expect("aggregate path entry should always be an object");
                if let Some(operations_object) = operations.as_object() {
                    for (method, operation) in operations_object {
                        let mut operation_value = operation.clone();
                        if let Some(operation_object) = operation_value.as_object_mut() {
                            operation_object
                                .entry("x-sdkwork-service".to_owned())
                                .or_insert(Value::String(document.service_id.clone()));
                        }
                        path_object.insert(method.clone(), operation_value);
                    }
                }
            }
        }

        if let Some(schemes) = document
            .document
            .get("components")
            .and_then(|value| value.get("securitySchemes"))
            .and_then(Value::as_object)
        {
            for (name, scheme) in schemes {
                security_schemes
                    .entry(name.clone())
                    .or_insert_with(|| scheme.clone());
            }
        }
    }

    merge_gateway_discovery_openapi(&mut tags, &mut paths);

    let mut document = Map::new();
    document.insert("openapi".to_owned(), Value::String("3.1.0".to_owned()));
    document.insert(
        "info".to_owned(),
        json!({
            "title": "Sdkwork IM Unified Gateway API",
            "version": env!("CARGO_PKG_VERSION"),
            "description": "Aggregate OpenAPI contract assembled by sdkwork-im-cloud-gateway from live upstream service schemas."
        }),
    );
    document.insert("servers".to_owned(), json!([{ "url": "/" }]));
    document.insert(
        "tags".to_owned(),
        Value::Array(tags.into_values().collect()),
    );
    document.insert("paths".to_owned(), Value::Object(paths));

    if !security_schemes.is_empty() || !schemas.is_empty() {
        let mut components = Map::new();
        if !security_schemes.is_empty() {
            components.insert(
                "securitySchemes".to_owned(),
                Value::Object(security_schemes),
            );
        }
        if !schemas.is_empty() {
            components.insert(
                "schemas".to_owned(),
                Value::Object(std::mem::take(&mut schemas)),
            );
        }
        document.insert("components".to_owned(), Value::Object(components));
    }

    Value::Object(document)
}

pub(crate) fn service_schema_index_entries(
    config: &WebGatewayConfig,
    registry: &RouteRegistry,
) -> Vec<ServiceSchemaIndexEntry> {
    config
        .upstreams
        .iter()
        .map(|upstream| {
            let service_routes = registry
                .entries()
                .iter()
                .filter(|entry| entry.service_id == upstream.service_id)
                .collect::<Vec<_>>();

            ServiceSchemaIndexEntry {
                service_id: upstream.service_id.clone(),
                contract_kind: ContractKind::UpstreamOperational,
                schema_url: format!("/openapi/services/{}.openapi.json", upstream.service_id),
                docs_url: format!("/docs/services/{}", upstream.service_id),
                visibility: service_visibility(service_routes.as_slice())
                    .unwrap_or_else(|| visibility_for_service(upstream.service_id.as_str())),
                route_count: service_routes.len(),
                operation_groups: service_routes
                    .iter()
                    .map(|entry| entry.operation_group.clone())
                    .collect::<BTreeSet<_>>()
                    .into_iter()
                    .collect(),
                sdk_targets: service_routes
                    .iter()
                    .flat_map(|entry| entry.sdk_targets.iter().copied())
                    .collect::<BTreeSet<_>>()
                    .into_iter()
                    .collect(),
                protocols: service_routes
                    .iter()
                    .map(|entry| entry.protocol)
                    .collect::<BTreeSet<_>>()
                    .into_iter()
                    .collect(),
                websocket_subprotocols: service_routes
                    .iter()
                    .flat_map(|entry| entry.websocket_subprotocols.iter().cloned())
                    .collect::<BTreeSet<_>>()
                    .into_iter()
                    .collect(),
            }
        })
        .collect()
}

fn aggregate_openapi_cache_key(state: &GatewayState, gateway_base_url: &str) -> String {
    let gateway_openapi_urls = gateway_openapi_url_candidates(&state.config, gateway_base_url);
    let skipped_service_ids = state
        .config
        .upstreams
        .iter()
        .filter(|upstream| {
            is_self_referential_gateway_openapi_fetch(
                upstream.base_url.as_str(),
                gateway_openapi_urls.as_slice(),
            )
        })
        .map(|upstream| upstream.service_id.as_str())
        .collect::<Vec<_>>()
        .join(",");
    format!("self_referential_skips={skipped_service_ids}")
}

fn gateway_openapi_url_candidates(
    config: &WebGatewayConfig,
    request_base_url: &str,
) -> Vec<String> {
    let mut candidates = Vec::new();
    push_openapi_url_candidate(&mut candidates, request_base_url);
    if let Some(bind_base_url) = gateway_bind_base_url(config.bind_addr.as_str()) {
        push_openapi_url_candidate(&mut candidates, bind_base_url.as_str());
    }
    candidates
}

fn push_openapi_url_candidate(candidates: &mut Vec<String>, base_url: &str) {
    let candidate = service_openapi_url(base_url);
    if !candidates
        .iter()
        .any(|existing| same_http_endpoint(existing, &candidate))
    {
        candidates.push(candidate);
    }
}

fn service_openapi_url(base_url: &str) -> String {
    format!("{}/openapi.json", base_url.trim_end_matches('/'))
}

fn gateway_bind_base_url(bind_addr: &str) -> Option<String> {
    let bind_addr = bind_addr.trim().trim_end_matches('/');
    if bind_addr.is_empty() {
        return None;
    }
    if bind_addr.starts_with("http://") || bind_addr.starts_with("https://") {
        return Some(bind_addr.to_owned());
    }
    if let Ok(addr) = bind_addr.parse::<SocketAddr>() {
        return Some(format!("http://{}", display_socket_addr_for_base_url(addr)));
    }
    Some(format!("http://{bind_addr}"))
}

fn display_socket_addr_for_base_url(addr: SocketAddr) -> String {
    match addr.ip() {
        IpAddr::V4(ip) if ip.is_unspecified() => format!("127.0.0.1:{}", addr.port()),
        IpAddr::V6(ip) if ip.is_unspecified() => format!("[::1]:{}", addr.port()),
        _ => addr.to_string(),
    }
}

fn is_self_referential_gateway_openapi_fetch(
    upstream_base_url: &str,
    gateway_openapi_urls: &[String],
) -> bool {
    let upstream_openapi_url = service_openapi_url(upstream_base_url);
    gateway_openapi_urls.iter().any(|gateway_openapi_url| {
        same_http_endpoint(upstream_openapi_url.as_str(), gateway_openapi_url)
    })
}

fn same_http_endpoint(left: &str, right: &str) -> bool {
    let Ok(left) = Url::parse(left) else {
        return false;
    };
    let Ok(right) = Url::parse(right) else {
        return false;
    };
    match (http_endpoint_key(&left), http_endpoint_key(&right)) {
        (Some(left), Some(right)) => left == right,
        _ => false,
    }
}

fn http_endpoint_key(url: &Url) -> Option<(String, u16, String)> {
    Some((
        normalize_endpoint_host(url.host_str()?),
        url.port_or_known_default()?,
        normalize_endpoint_path(url.path()),
    ))
}

fn normalize_endpoint_host(host: &str) -> String {
    let host = host
        .trim_matches(|ch| ch == '[' || ch == ']')
        .to_ascii_lowercase();
    match host.as_str() {
        "localhost" | "127.0.0.1" | "::1" => "loopback".to_owned(),
        other => other.to_owned(),
    }
}

fn normalize_endpoint_path(path: &str) -> String {
    if path.is_empty() {
        return "/".to_owned();
    }
    path.to_owned()
}
