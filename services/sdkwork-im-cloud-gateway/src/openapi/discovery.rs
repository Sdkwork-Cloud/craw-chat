//! Gateway-owned OpenAPI discovery schema components and the discovery path
//! injections merged into the aggregate document.

use serde_json::{Map, Value, json};
use sdkwork_im_api_registry::{RouteDescriptor, RouteVisibility};

pub(crate) fn merge_gateway_discovery_openapi(
    tags: &mut std::collections::BTreeMap<String, Value>,
    paths: &mut Map<String, Value>,
) {
    tags.entry("gatewayDiscovery".to_owned()).or_insert_with(|| {
        json!({
            "name": "gatewayDiscovery",
            "description": "Gateway discovery operations exposed directly by sdkwork-im-cloud-gateway."
        })
    });

    paths.insert(
        "/openapi/index.json".to_owned(),
        json!({
            "get": {
                "operationId": "getGatewayOpenapiIndex",
                "summary": "Get gateway service schema index",
                "tags": ["gatewayDiscovery"],
                "responses": {
                    "200": {
                        "description": "Gateway service schema index",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "$ref": "#/components/schemas/GatewayOpenapiIndex"
                                }
                            }
                        }
                    }
                },
                "x-sdkwork-service": "sdkwork-im-cloud-gateway"
            }
        }),
    );
    paths.insert(
        "/openapi/runtime-summary.json".to_owned(),
        json!({
            "get": {
                "operationId": "getGatewayRuntimeSummary",
                "summary": "Get gateway runtime discovery summary",
                "tags": ["gatewayDiscovery"],
                "responses": {
                    "200": {
                        "description": "Gateway runtime summary",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "$ref": "#/components/schemas/GatewayRuntimeSummary"
                                }
                            }
                        }
                    }
                },
                "x-sdkwork-service": "sdkwork-im-cloud-gateway"
            }
        }),
    );
    paths.insert(
        "/openapi/services/{serviceId}.openapi.json".to_owned(),
        json!({
            "get": {
                "operationId": "getGatewayServiceSchema",
                "summary": "Get gateway service OpenAPI schema",
                "tags": ["gatewayDiscovery"],
                "parameters": [
                    {
                        "name": "serviceId",
                        "in": "path",
                        "required": true,
                        "schema": { "type": "string" },
                        "description": "Gateway service identifier"
                    }
                ],
                "responses": {
                    "200": {
                        "description": "Service OpenAPI schema"
                    }
                },
                "x-sdkwork-service": "sdkwork-im-cloud-gateway"
            }
        }),
    );
}

pub(crate) fn gateway_discovery_schema_components() -> Map<String, Value> {
    let mut schemas = Map::new();

    schemas.insert(
        "GatewayOpenapiIndex".to_owned(),
        json!({
            "type": "object",
            "required": ["sdkContracts", "services", "routes", "surfaceGroups"],
            "properties": {
                "sdkContracts": {
                    "type": "array",
                    "items": { "$ref": "#/components/schemas/GatewaySdkContractSummary" }
                },
                "services": {
                    "type": "array",
                    "items": { "$ref": "#/components/schemas/GatewayServiceSchemaIndexEntry" }
                },
                "routes": {
                    "type": "array",
                    "items": { "$ref": "#/components/schemas/GatewayRouteSummary" }
                },
                "surfaceGroups": {
                    "type": "array",
                    "items": { "$ref": "#/components/schemas/GatewaySurfaceGroupSummary" }
                }
            }
        }),
    );
    schemas.insert(
        "GatewayRuntimeSummary".to_owned(),
        json!({
            "type": "object",
            "required": [
                "bindAddr",
                "baseUrl",
                "aggregateOpenapiUrl",
                "openapiIndexUrl",
                "runtimeSummaryUrl",
                "docsUrl",
                "runtimeMode",
                "sdkContracts",
                "upstreams",
                "serviceContracts",
                "publicEndpoints",
                "surfaceGroups"
            ],
            "properties": {
                "bindAddr": { "type": "string" },
                "baseUrl": { "type": "string", "format": "uri" },
                "aggregateOpenapiUrl": { "type": "string", "format": "uri" },
                "openapiIndexUrl": { "type": "string", "format": "uri" },
                "runtimeSummaryUrl": { "type": "string", "format": "uri" },
                "docsUrl": { "type": "string", "format": "uri" },
                "runtimeMode": { "$ref": "#/components/schemas/GatewayRuntimeMode" },
                "sdkContracts": {
                    "type": "array",
                    "items": { "$ref": "#/components/schemas/GatewaySdkContractSummary" }
                },
                "upstreams": {
                    "type": "array",
                    "items": { "$ref": "#/components/schemas/GatewayUpstreamBinding" }
                },
                "serviceContracts": {
                    "type": "array",
                    "items": { "$ref": "#/components/schemas/GatewayServiceContractSummary" }
                },
                "publicEndpoints": {
                    "type": "array",
                    "items": { "$ref": "#/components/schemas/GatewayPublicEndpointSummary" }
                },
                "surfaceGroups": {
                    "type": "array",
                    "items": { "$ref": "#/components/schemas/GatewaySurfaceGroupSummary" }
                }
            }
        }),
    );
    schemas.insert(
        "GatewayServiceSchemaIndexEntry".to_owned(),
        json!({
            "type": "object",
            "required": [
                "serviceId",
                "contractKind",
                "schemaUrl",
                "docsUrl",
                "visibility",
                "routeCount",
                "operationGroups",
                "sdkTargets",
                "protocols"
            ],
            "properties": {
                "serviceId": { "type": "string" },
                "contractKind": { "$ref": "#/components/schemas/GatewayContractKind" },
                "schemaUrl": { "type": "string" },
                "docsUrl": { "type": "string" },
                "visibility": { "$ref": "#/components/schemas/GatewayRouteVisibility" },
                "routeCount": { "type": "integer", "minimum": 0 },
                "operationGroups": {
                    "type": "array",
                    "items": { "type": "string" }
                },
                "sdkTargets": {
                    "type": "array",
                    "items": { "$ref": "#/components/schemas/GatewaySdkTarget" }
                },
                "protocols": {
                    "type": "array",
                    "items": { "$ref": "#/components/schemas/GatewayRouteProtocol" }
                },
                "websocketSubprotocols": {
                    "type": "array",
                    "items": { "type": "string" }
                }
            }
        }),
    );
    schemas.insert(
        "GatewayRouteSummary".to_owned(),
        json!({
            "type": "object",
            "required": [
                "serviceId",
                "operationGroup",
                "visibility",
                "pathPattern",
                "methods",
                "protocol",
                "sdkTargets"
            ],
            "properties": {
                "serviceId": { "type": "string" },
                "operationGroup": { "type": "string" },
                "visibility": { "$ref": "#/components/schemas/GatewayRouteVisibility" },
                "pathPattern": { "type": "string" },
                "methods": {
                    "type": "array",
                    "items": { "$ref": "#/components/schemas/GatewayHttpMethod" }
                },
                "protocol": { "$ref": "#/components/schemas/GatewayRouteProtocol" },
                "sdkTargets": {
                    "type": "array",
                    "items": { "$ref": "#/components/schemas/GatewaySdkTarget" }
                },
                "websocketSubprotocols": {
                    "type": "array",
                    "items": { "type": "string" }
                }
            }
        }),
    );
    schemas.insert(
        "GatewaySurfaceGroupSummary".to_owned(),
        json!({
            "type": "object",
            "required": [
                "serviceId",
                "operationGroup",
                "visibility",
                "routeCount",
                "sdkTargets",
                "protocols"
            ],
            "properties": {
                "serviceId": { "type": "string" },
                "operationGroup": { "type": "string" },
                "visibility": { "$ref": "#/components/schemas/GatewayRouteVisibility" },
                "routeCount": { "type": "integer", "minimum": 0 },
                "sdkTargets": {
                    "type": "array",
                    "items": { "$ref": "#/components/schemas/GatewaySdkTarget" }
                },
                "protocols": {
                    "type": "array",
                    "items": { "$ref": "#/components/schemas/GatewayRouteProtocol" }
                },
                "websocketSubprotocols": {
                    "type": "array",
                    "items": { "type": "string" }
                }
            }
        }),
    );
    schemas.insert(
        "GatewayServiceContractSummary".to_owned(),
        json!({
            "type": "object",
            "required": ["serviceId", "contractKind", "schemaUrl", "docsUrl"],
            "properties": {
                "serviceId": { "type": "string" },
                "contractKind": { "$ref": "#/components/schemas/GatewayContractKind" },
                "schemaUrl": { "type": "string", "format": "uri" },
                "docsUrl": { "type": "string", "format": "uri" }
            }
        }),
    );
    schemas.insert(
        "GatewaySdkContractSummary".to_owned(),
        json!({
            "type": "object",
            "required": ["groupId", "contractKind", "schemaUrl", "apiPrefix", "sdkTarget"],
            "properties": {
                "groupId": { "type": "string", "enum": ["im-open-api", "im-app-api", "im-backend-api"] },
                "contractKind": { "$ref": "#/components/schemas/GatewayContractKind" },
                "schemaUrl": { "type": "string" },
                "apiPrefix": { "type": "string" },
                "sdkTarget": { "$ref": "#/components/schemas/GatewaySdkTarget" }
            }
        }),
    );
    schemas.insert(
        "GatewayPublicEndpointSummary".to_owned(),
        json!({
            "type": "object",
            "required": ["serviceId", "pathPattern", "protocol", "visibility", "methods"],
            "properties": {
                "serviceId": { "type": "string" },
                "pathPattern": { "type": "string" },
                "protocol": { "$ref": "#/components/schemas/GatewayRouteProtocol" },
                "visibility": { "$ref": "#/components/schemas/GatewayRouteVisibility" },
                "methods": {
                    "type": "array",
                    "items": { "$ref": "#/components/schemas/GatewayHttpMethod" }
                }
            }
        }),
    );
    schemas.insert(
        "GatewayRuntimeMode".to_owned(),
        json!({
            "type": "string",
            "enum": ["split", "unified"]
        }),
    );
    schemas.insert(
        "GatewayUpstreamBinding".to_owned(),
        json!({
            "type": "array",
            "prefixItems": [
                { "type": "string" },
                { "type": "string", "format": "uri" }
            ],
            "minItems": 2,
            "maxItems": 2
        }),
    );
    schemas.insert(
        "GatewayRouteVisibility".to_owned(),
        json!({
            "type": "string",
            "enum": ["public", "partner", "internal"]
        }),
    );
    schemas.insert(
        "GatewayRouteProtocol".to_owned(),
        json!({
            "type": "string",
            "enum": ["http", "websocket"]
        }),
    );
    schemas.insert(
        "GatewaySdkTarget".to_owned(),
        json!({
            "type": "string",
            "enum": ["sdkworkImSdk", "sdkworkImAppSdk", "sdkworkImBackendSdk", "sdkworkDriveAppSdk", "sdkworkNotaryAppSdk", "none"]
        }),
    );
    schemas.insert(
        "GatewayContractKind".to_owned(),
        json!({
            "type": "string",
            "enum": ["sdk", "upstreamOperational"]
        }),
    );
    schemas.insert(
        "GatewayHttpMethod".to_owned(),
        json!({
            "type": "string",
            "enum": ["delete", "get", "head", "options", "patch", "post", "put"]
        }),
    );

    schemas
}

pub(crate) fn service_visibility(service_routes: &[&RouteDescriptor]) -> Option<RouteVisibility> {
    if service_routes
        .iter()
        .any(|entry| entry.visibility == RouteVisibility::Public)
    {
        return Some(RouteVisibility::Public);
    }
    if service_routes
        .iter()
        .any(|entry| entry.visibility == RouteVisibility::Partner)
    {
        return Some(RouteVisibility::Partner);
    }
    if service_routes
        .iter()
        .any(|entry| entry.visibility == RouteVisibility::Internal)
    {
        return Some(RouteVisibility::Internal);
    }
    None
}

pub(crate) fn visibility_for_service(service_id: &str) -> RouteVisibility {
    match service_id {
        "governance-service" | "audit-service" | "ops-service" => RouteVisibility::Internal,
        _ => RouteVisibility::Public,
    }
}
