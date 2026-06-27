//! Aggregation of upstream service OpenAPI documents into a single gateway
//! contract, plus the service schema index projection.

use std::collections::BTreeSet;
use std::time::Duration;

use axum::response::Response;
use serde_json::{Map, Value, json};
use sdkwork_im_api_registry::{ContractKind, RouteRegistry, ServiceSchemaIndexEntry};
use sdkwork_im_cloud_gateway_config::WebGatewayConfig;

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

pub(crate) async fn fetch_service_openapi_documents(
    state: &GatewayState,
) -> Result<Vec<ServiceOpenApiDocument>, Response> {
    let fetches = state.config.upstreams.iter().map(|upstream| {
        let service_id = upstream.service_id.clone();
        async move {
            (
                service_id.clone(),
                fetch_service_openapi_document(state, service_id.as_str()).await,
            )
        }
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
) -> Result<Value, Response> {
    let Some(base_url) = state.config.upstream_base_url(service_id) else {
        return Err(json_error_response(
            axum::http::StatusCode::NOT_FOUND,
            format!("service schema upstream is not configured for {service_id}").as_str(),
        ));
    };
    let url = format!("{}/openapi.json", base_url.trim_end_matches('/'));
    let response = state
        .client
        .get(url)
        .timeout(Duration::from_secs(2))
        .send()
        .await
        .map_err(|error| {
            json_error_response(
                axum::http::StatusCode::BAD_GATEWAY,
                format!("failed to fetch upstream schema for {service_id}: {error}").as_str(),
            )
        })?;
    let status = response.status();
    if !status.is_success() {
        return Err(json_error_response(
            axum::http::StatusCode::BAD_GATEWAY,
            format!("upstream schema request for {service_id} returned {status}").as_str(),
        ));
    }
    response.json::<Value>().await.map_err(|error| {
        json_error_response(
            axum::http::StatusCode::BAD_GATEWAY,
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
