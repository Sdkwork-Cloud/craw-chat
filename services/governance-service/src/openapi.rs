use serde_json::{Map as JsonMap, Value as JsonValue};
use sdkwork_im_openapi::OpenApiServiceSpec;

fn openapi_string_schema() -> JsonValue {
    serde_json::json!({
        "type": "string"
    })
}

fn openapi_nullable_string_schema() -> JsonValue {
    serde_json::json!({
        "type": "string",
        "nullable": true
    })
}

fn openapi_integer_schema() -> JsonValue {
    serde_json::json!({
        "type": "integer",
        "format": "int64"
    })
}

fn openapi_nullable_integer_schema() -> JsonValue {
    serde_json::json!({
        "type": "integer",
        "format": "int64",
        "nullable": true
    })
}

fn openapi_boolean_schema() -> JsonValue {
    serde_json::json!({
        "type": "boolean"
    })
}

fn openapi_string_array_schema() -> JsonValue {
    serde_json::json!({
        "type": "array",
        "items": openapi_string_schema()
    })
}

fn openapi_array_schema(items: JsonValue) -> JsonValue {
    serde_json::json!({
        "type": "array",
        "items": items
    })
}

fn openapi_object_schema(
    required: &[&str],
    properties: Vec<(&str, JsonValue)>,
    additional_properties: bool,
) -> JsonValue {
    let mut property_map = JsonMap::new();
    for (name, schema) in properties {
        property_map.insert(name.to_owned(), schema);
    }

    let mut schema = JsonMap::new();
    schema.insert("type".to_owned(), serde_json::json!("object"));
    schema.insert("properties".to_owned(), JsonValue::Object(property_map));
    schema.insert(
        "additionalProperties".to_owned(),
        JsonValue::Bool(additional_properties),
    );
    if !required.is_empty() {
        schema.insert("required".to_owned(), serde_json::json!(required));
    }

    JsonValue::Object(schema)
}

fn openapi_describe(mut schema: JsonValue, description: &str) -> JsonValue {
    if !description.is_empty()
        && let JsonValue::Object(object) = &mut schema
    {
        object.insert(
            "description".to_owned(),
            JsonValue::String(description.to_owned()),
        );
    }
    schema
}

fn openapi_generic_object_schema(description: &str) -> JsonValue {
    openapi_describe(openapi_object_schema(&[], Vec::new(), true), description)
}

fn openapi_component_ref(name: &str) -> JsonValue {
    serde_json::json!({
        "$ref": format!("#/components/schemas/{name}")
    })
}

fn insert_openapi_schema(schemas: &mut JsonMap<String, JsonValue>, name: &str, schema: JsonValue) {
    schemas.insert(name.to_owned(), schema);
}

fn openapi_json_response(description: &str, schema_name: &str) -> JsonValue {
    serde_json::json!({
        "description": description,
        "content": {
            "application/json": {
                "schema": openapi_component_ref(schema_name)
            }
        }
    })
}

fn openapi_problem_response(description: &str, schema_name: &str) -> JsonValue {
    serde_json::json!({
        "description": description,
        "content": {
            "application/problem+json": {
                "schema": openapi_component_ref(schema_name)
            }
        }
    })
}

fn openapi_standard_responses(success_schema_name: &str) -> JsonValue {
    let mut responses = JsonMap::new();
    responses.insert(
        "200".to_owned(),
        openapi_json_response("Successful response.", success_schema_name),
    );

    for (status, description) in [
        ("400", "Invalid request."),
        ("401", "Authentication required."),
        ("403", "Permission denied."),
        ("404", "Resource not found."),
        ("409", "Request conflicts with current control-plane state."),
        ("503", "Control-plane dependency is unavailable."),
    ] {
        responses.insert(
            status.to_owned(),
            openapi_problem_response(description, "ControlPlaneErrorResponse"),
        );
    }

    JsonValue::Object(responses)
}

fn openapi_request_body(schema_name: &str, description: &str) -> JsonValue {
    serde_json::json!({
        "required": true,
        "description": description,
        "content": {
            "application/json": {
                "schema": openapi_component_ref(schema_name)
            }
        }
    })
}

fn openapi_query_parameter(
    name: &str,
    required: bool,
    schema: JsonValue,
    description: &str,
) -> JsonValue {
    serde_json::json!({
        "name": name,
        "in": "query",
        "required": required,
        "description": description,
        "schema": schema
    })
}

fn openapi_path_parameter(name: &str, description: &str) -> JsonValue {
    serde_json::json!({
        "name": name,
        "in": "path",
        "required": true,
        "description": description,
        "schema": openapi_string_schema()
    })
}

fn openapi_operation(
    summary: &str,
    operation_id: &str,
    tag: &str,
    parameters: Vec<JsonValue>,
    request_body: Option<JsonValue>,
    response_schema_name: &str,
    secure: bool,
) -> JsonValue {
    let mut operation = JsonMap::new();
    operation.insert("summary".to_owned(), JsonValue::String(summary.to_owned()));
    operation.insert(
        "operationId".to_owned(),
        JsonValue::String(operation_id.to_owned()),
    );
    operation.insert("tags".to_owned(), serde_json::json!([tag]));
    operation.insert(
        "responses".to_owned(),
        openapi_standard_responses(response_schema_name),
    );
    if secure {
        operation.insert(
            "security".to_owned(),
            serde_json::json!([{ "AuthToken": [], "AccessToken": [] }]),
        );
    } else {
        operation.insert("security".to_owned(), serde_json::json!([]));
    }
    if !parameters.is_empty() {
        operation.insert("parameters".to_owned(), JsonValue::Array(parameters));
    }
    if let Some(request_body) = request_body {
        operation.insert("requestBody".to_owned(), request_body);
    }

    JsonValue::Object(operation)
}

fn control_plane_openapi_components() -> JsonValue {
    let mut schemas = JsonMap::new();

    insert_openapi_schema(
        &mut schemas,
        "HealthResponse",
        openapi_object_schema(
            &["status", "service"],
            vec![
                ("status", openapi_string_schema()),
                ("service", openapi_string_schema()),
            ],
            false,
        ),
    );
    insert_openapi_schema(
        &mut schemas,
        "ControlPlaneErrorResponse",
        openapi_object_schema(
            &["type", "title", "status"],
            vec![
                (
                    "type",
                    serde_json::json!({
                        "type": "string",
                        "format": "uri-reference"
                    }),
                ),
                ("title", openapi_string_schema()),
                (
                    "status",
                    serde_json::json!({
                        "type": "integer",
                        "minimum": 100,
                        "maximum": 599
                    }),
                ),
                ("detail", openapi_string_schema()),
                ("instance", openapi_string_schema()),
                ("code", openapi_string_schema()),
                ("message", openapi_string_schema()),
                ("errorStatus", openapi_string_schema()),
                (
                    "details",
                    serde_json::json!({
                        "type": "object",
                        "nullable": true,
                        "additionalProperties": true
                    }),
                ),
            ],
            true,
        ),
    );
    insert_openapi_schema(
        &mut schemas,
        "ProtocolSchemaResponse",
        openapi_object_schema(
            &[
                "schema",
                "kind",
                "stage",
                "bindingProtocols",
                "requiredCapabilities",
                "supportedConsumers",
            ],
            vec![
                ("schema", openapi_string_schema()),
                ("kind", openapi_string_schema()),
                ("stage", openapi_string_schema()),
                ("bindingProtocols", openapi_string_array_schema()),
                ("requiredCapabilities", openapi_string_array_schema()),
                ("supportedConsumers", openapi_string_array_schema()),
            ],
            false,
        ),
    );
    insert_openapi_schema(
        &mut schemas,
        "ClientCompatibilityResponse",
        openapi_object_schema(
            &[
                "clientType",
                "minimumProtocolVersion",
                "supportedBindings",
                "supportedCodecs",
                "supportedCapabilities",
                "blockedExperimentalCapabilities",
            ],
            vec![
                ("clientType", openapi_string_schema()),
                ("minimumProtocolVersion", openapi_string_schema()),
                ("supportedBindings", openapi_string_array_schema()),
                ("supportedCodecs", openapi_string_array_schema()),
                ("supportedCapabilities", openapi_string_array_schema()),
                (
                    "blockedExperimentalCapabilities",
                    openapi_string_array_schema(),
                ),
            ],
            false,
        ),
    );
    insert_openapi_schema(
        &mut schemas,
        "ProtocolRegistryResponse",
        openapi_object_schema(
            &[
                "protocolVersion",
                "bindings",
                "codecs",
                "schemas",
                "compatibilityMatrix",
            ],
            vec![
                ("protocolVersion", openapi_string_schema()),
                ("bindings", openapi_string_array_schema()),
                ("codecs", openapi_string_array_schema()),
                (
                    "schemas",
                    openapi_array_schema(openapi_component_ref("ProtocolSchemaResponse")),
                ),
                (
                    "compatibilityMatrix",
                    openapi_array_schema(openapi_component_ref("ClientCompatibilityResponse")),
                ),
            ],
            false,
        ),
    );
    insert_openapi_schema(
        &mut schemas,
        "CapabilityProfileResponse",
        openapi_object_schema(
            &[
                "profileId",
                "releaseChannel",
                "enabledCapabilities",
                "experimentalCapabilities",
            ],
            vec![
                ("profileId", openapi_string_schema()),
                ("releaseChannel", openapi_string_schema()),
                ("enabledCapabilities", openapi_string_array_schema()),
                ("experimentalCapabilities", openapi_string_array_schema()),
            ],
            false,
        ),
    );
    insert_openapi_schema(
        &mut schemas,
        "QuotaProfileResponse",
        openapi_object_schema(
            &[
                "profileId",
                "maxConcurrentSessionsPerTenant",
                "maxSubscriptionsPerSession",
                "maxInflightMessages",
                "maxPayloadBytes",
            ],
            vec![
                ("profileId", openapi_string_schema()),
                ("maxConcurrentSessionsPerTenant", openapi_integer_schema()),
                ("maxSubscriptionsPerSession", openapi_integer_schema()),
                ("maxInflightMessages", openapi_integer_schema()),
                ("maxPayloadBytes", openapi_integer_schema()),
            ],
            false,
        ),
    );
    insert_openapi_schema(
        &mut schemas,
        "RolloutPolicyResponse",
        openapi_object_schema(
            &[
                "policyId",
                "releaseChannel",
                "trafficPercent",
                "cellSelector",
                "regionSelector",
                "operatorOverride",
                "tenantAllowlist",
            ],
            vec![
                ("policyId", openapi_string_schema()),
                ("releaseChannel", openapi_string_schema()),
                ("trafficPercent", openapi_integer_schema()),
                ("cellSelector", openapi_string_schema()),
                ("regionSelector", openapi_string_schema()),
                ("operatorOverride", openapi_boolean_schema()),
                ("tenantAllowlist", openapi_string_array_schema()),
            ],
            false,
        ),
    );
    insert_openapi_schema(
        &mut schemas,
        "KillSwitchResponse",
        openapi_object_schema(
            &[
                "ruleId",
                "active",
                "reason",
                "disabledCapabilities",
                "disabledBindings",
                "disabledCodecs",
            ],
            vec![
                ("ruleId", openapi_string_schema()),
                ("active", openapi_boolean_schema()),
                ("reason", openapi_string_schema()),
                ("disabledCapabilities", openapi_string_array_schema()),
                ("disabledBindings", openapi_string_array_schema()),
                ("disabledCodecs", openapi_string_array_schema()),
            ],
            false,
        ),
    );
    insert_openapi_schema(
        &mut schemas,
        "EffectiveProtocolSnapshotResponse",
        openapi_object_schema(
            &[
                "protocolVersion",
                "releaseChannel",
                "enabledCapabilities",
                "allowedBindings",
                "allowedCodecs",
                "quotaProfileId",
                "killSwitchActive",
                "precedence",
            ],
            vec![
                ("protocolVersion", openapi_string_schema()),
                ("releaseChannel", openapi_string_schema()),
                ("enabledCapabilities", openapi_string_array_schema()),
                ("allowedBindings", openapi_string_array_schema()),
                ("allowedCodecs", openapi_string_array_schema()),
                ("quotaProfileId", openapi_string_schema()),
                ("killSwitchActive", openapi_boolean_schema()),
                ("precedence", openapi_string_array_schema()),
            ],
            false,
        ),
    );
    insert_openapi_schema(
        &mut schemas,
        "BusinessPolicyVocabularyResponse",
        openapi_object_schema(
            &[
                "policyVersionField",
                "capabilityFlagsField",
                "historyVisibilityField",
                "historyVisibilityModes",
                "retentionPolicyRefField",
                "retentionPolicyScopes",
                "retentionClasses",
            ],
            vec![
                ("policyVersionField", openapi_string_schema()),
                ("capabilityFlagsField", openapi_string_schema()),
                ("historyVisibilityField", openapi_string_schema()),
                ("historyVisibilityModes", openapi_string_array_schema()),
                ("retentionPolicyRefField", openapi_string_schema()),
                ("retentionPolicyScopes", openapi_string_array_schema()),
                ("retentionClasses", openapi_string_array_schema()),
            ],
            false,
        ),
    );
    insert_openapi_schema(
        &mut schemas,
        "SdkCompatibilityBaselineResponse",
        openapi_object_schema(
            &[
                "imSdkFamily",
                "appSdkFamily",
                "backendSdkFamily",
                "rtcSdkFamily",
                "matrixClientTypes",
                "protocolRegistryPath",
                "protocolGovernancePath",
            ],
            vec![
                ("imSdkFamily", openapi_string_schema()),
                ("appSdkFamily", openapi_string_schema()),
                ("backendSdkFamily", openapi_string_schema()),
                ("rtcSdkFamily", openapi_string_schema()),
                ("matrixClientTypes", openapi_string_array_schema()),
                ("protocolRegistryPath", openapi_string_schema()),
                ("protocolGovernancePath", openapi_string_schema()),
            ],
            false,
        ),
    );
    insert_openapi_schema(
        &mut schemas,
        "ProtocolGovernanceResponse",
        openapi_object_schema(
            &[
                "capabilityProfile",
                "quotaProfile",
                "rolloutPolicy",
                "killSwitch",
                "effectiveSnapshot",
                "businessPolicyVocabulary",
                "sdkCompatibilityBaseline",
            ],
            vec![
                (
                    "capabilityProfile",
                    openapi_component_ref("CapabilityProfileResponse"),
                ),
                (
                    "quotaProfile",
                    openapi_component_ref("QuotaProfileResponse"),
                ),
                (
                    "rolloutPolicy",
                    openapi_component_ref("RolloutPolicyResponse"),
                ),
                ("killSwitch", openapi_component_ref("KillSwitchResponse")),
                (
                    "effectiveSnapshot",
                    openapi_component_ref("EffectiveProtocolSnapshotResponse"),
                ),
                (
                    "businessPolicyVocabulary",
                    openapi_component_ref("BusinessPolicyVocabularyResponse"),
                ),
                (
                    "sdkCompatibilityBaseline",
                    openapi_component_ref("SdkCompatibilityBaselineResponse"),
                ),
            ],
            false,
        ),
    );
    insert_openapi_schema(
        &mut schemas,
        "RouteNodeLifecycle",
        openapi_object_schema(
            &["nodeId", "drainStatus", "rebalanceState", "ownedRouteCount"],
            vec![
                ("nodeId", openapi_string_schema()),
                ("drainStatus", openapi_string_schema()),
                ("rebalanceState", openapi_string_schema()),
                ("ownedRouteCount", openapi_integer_schema()),
            ],
            false,
        ),
    );
    insert_openapi_schema(
        &mut schemas,
        "RouteMigrationResult",
        openapi_object_schema(
            &[
                "sourceNodeId",
                "targetNodeId",
                "migratedRouteCount",
                "sourceDrainStatus",
                "sourceRebalanceState",
                "targetDrainStatus",
                "targetRebalanceState",
            ],
            vec![
                ("sourceNodeId", openapi_string_schema()),
                ("targetNodeId", openapi_string_schema()),
                ("migratedRouteCount", openapi_integer_schema()),
                ("sourceDrainStatus", openapi_string_schema()),
                ("sourceRebalanceState", openapi_string_schema()),
                ("targetDrainStatus", openapi_string_schema()),
                ("targetRebalanceState", openapi_string_schema()),
            ],
            false,
        ),
    );

    for (name, description) in [
        (
            "ProviderRegistrySnapshotResponse",
            "Provider registry snapshot for the current control-plane view.",
        ),
        (
            "ProviderBindingsResponse",
            "Effective provider bindings resolved for the current tenant scope.",
        ),
        (
            "ProviderBindingCommitResponse",
            "Provider binding mutation result after applying a control-plane policy change.",
        ),
        (
            "ProviderPolicyHistoryResponse",
            "Provider policy history snapshot for the current tenant scope.",
        ),
        (
            "ProviderPolicyDiffResponse",
            "Provider policy diff between two committed versions.",
        ),
    ] {
        insert_openapi_schema(
            &mut schemas,
            name,
            openapi_generic_object_schema(description),
        );
    }

    for (name, schema) in [
        (
            "UpsertProviderBindingPolicyRequest",
            openapi_object_schema(
                &["domain", "pluginId"],
                vec![
                    ("tenantId", openapi_nullable_string_schema()),
                    ("domain", openapi_string_schema()),
                    ("pluginId", openapi_string_schema()),
                    ("expectedBaseVersion", openapi_nullable_integer_schema()),
                ],
                false,
            ),
        ),
        (
            "ProviderPolicyRollbackRequest",
            openapi_object_schema(
                &["targetVersion"],
                vec![("targetVersion", openapi_integer_schema())],
                false,
            ),
        ),
        (
            "MigrateRoutesRequest",
            openapi_object_schema(
                &["targetNodeId"],
                vec![("targetNodeId", openapi_string_schema())],
                false,
            ),
        ),
    ] {
        insert_openapi_schema(&mut schemas, name, schema);
    }

    serde_json::json!({
        "securitySchemes": {
            "AuthToken": {
                "type": "http",
                "scheme": "bearer",
                "bearerFormat": "JWT"
            },
            "AccessToken": {
                "type": "apiKey",
                "in": "header",
                "name": "Access-Token"
            }
        },
        "schemas": JsonValue::Object(schemas)
    })
}

fn control_plane_openapi_paths() -> JsonValue {
    let mut paths = JsonMap::new();

    paths.insert(
        "/healthz".to_owned(),
        serde_json::json!({
            "get": openapi_operation(
                "Check control-plane process health.",
                "getHealthz",
                "meta",
                Vec::new(),
                None,
                "HealthResponse",
                false
            )
        }),
    );
    paths.insert(
        "/backend/v3/api/control/protocol_registry".to_owned(),
        serde_json::json!({
            "get": openapi_operation(
                "Read the control-plane protocol registry snapshot.",
                "getProtocolRegistry",
                "protocol",
                Vec::new(),
                None,
                "ProtocolRegistryResponse",
                true
            )
        }),
    );
    paths.insert(
        "/backend/v3/api/control/protocol_governance".to_owned(),
        serde_json::json!({
            "get": openapi_operation(
                "Read the control-plane protocol governance snapshot.",
                "getProtocolGovernance",
                "protocol",
                Vec::new(),
                None,
                "ProtocolGovernanceResponse",
                true
            )
        }),
    );
    paths.insert(
        "/backend/v3/api/control/provider_registry".to_owned(),
        serde_json::json!({
            "get": openapi_operation(
                "Read the provider registry snapshot.",
                "getProviderRegistry",
                "providers",
                Vec::new(),
                None,
                "ProviderRegistrySnapshotResponse",
                true
            )
        }),
    );
    paths.insert(
        "/backend/v3/api/control/provider_bindings".to_owned(),
        serde_json::json!({
            "get": openapi_operation(
                "Read effective provider bindings.",
                "getProviderBindings",
                "providers",
                vec![
                    openapi_query_parameter(
                        "tenantId",
                        false,
                        openapi_string_schema(),
                        "Optional tenant scope for effective provider bindings."
                    )
                ],
                None,
                "ProviderBindingsResponse",
                true
            ),
            "post": openapi_operation(
                "Upsert a provider binding policy.",
                "upsertProviderBindingPolicy",
                "providers",
                Vec::new(),
                Some(openapi_request_body(
                    "UpsertProviderBindingPolicyRequest",
                    "Provider binding mutation payload."
                )),
                "ProviderBindingCommitResponse",
                true
            )
        }),
    );
    paths.insert(
        "/backend/v3/api/control/provider_policies".to_owned(),
        serde_json::json!({
            "get": openapi_operation(
                "Read provider policy history.",
                "getProviderPolicyHistory",
                "providers",
                Vec::new(),
                None,
                "ProviderPolicyHistoryResponse",
                true
            )
        }),
    );
    paths.insert(
        "/backend/v3/api/control/provider_policies/diff".to_owned(),
        serde_json::json!({
            "get": openapi_operation(
                "Read provider policy diff between two versions.",
                "getProviderPolicyDiff",
                "providers",
                vec![
                    openapi_query_parameter(
                        "fromVersion",
                        true,
                        openapi_integer_schema(),
                        "Base provider policy version."
                    ),
                    openapi_query_parameter(
                        "toVersion",
                        true,
                        openapi_integer_schema(),
                        "Target provider policy version."
                    )
                ],
                None,
                "ProviderPolicyDiffResponse",
                true
            )
        }),
    );
    paths.insert(
        "/backend/v3/api/control/provider_policies/preview".to_owned(),
        serde_json::json!({
            "post": openapi_operation(
                "Preview the effective provider policy result before commit.",
                "previewProviderPolicy",
                "providers",
                Vec::new(),
                Some(openapi_request_body(
                    "UpsertProviderBindingPolicyRequest",
                    "Provider binding preview payload."
                )),
                "ProviderBindingCommitResponse",
                true
            )
        }),
    );
    paths.insert(
        "/backend/v3/api/control/provider_policies/rollback".to_owned(),
        serde_json::json!({
            "post": openapi_operation(
                "Rollback provider policy history to a target version.",
                "rollbackProviderPolicy",
                "providers",
                Vec::new(),
                Some(openapi_request_body(
                    "ProviderPolicyRollbackRequest",
                    "Provider policy rollback payload."
                )),
                "ProviderBindingCommitResponse",
                true
            )
        }),
    );

    for (path, operation_id, summary) in [
        (
            "/backend/v3/api/control/nodes/{node_id}/drain",
            "drainNode",
            "Mark a realtime node as draining.",
        ),
        (
            "/backend/v3/api/control/nodes/{node_id}/activate",
            "activateNode",
            "Activate a realtime node and clear drain state.",
        ),
    ] {
        paths.insert(
            path.to_owned(),
            serde_json::json!({
                "post": openapi_operation(
                    summary,
                    operation_id,
                    "nodes",
                    vec![openapi_path_parameter("node_id", "Realtime node identifier.")],
                    None,
                    "RouteNodeLifecycle",
                    true
                )
            }),
        );
    }

    paths.insert(
        "/backend/v3/api/control/nodes/{node_id}/routes/migrate".to_owned(),
        serde_json::json!({
            "post": openapi_operation(
                "Migrate owned routes from the source node to the target node.",
                "migrateNodeRoutes",
                "nodes",
                vec![openapi_path_parameter("node_id", "Source realtime node identifier.")],
                Some(openapi_request_body(
                    "MigrateRoutesRequest",
                    "Route migration target payload."
                )),
                "RouteMigrationResult",
                true
            )
        }),
    );

    JsonValue::Object(paths)
}

pub(crate) fn control_plane_openapi_document() -> JsonValue {
    serde_json::json!({
        "openapi": "3.1.2",
        "info": {
            "title": "Control Plane API",
            "version": env!("CARGO_PKG_VERSION"),
            "description": "Live OpenAPI contract for the control-plane runtime. This document is emitted by the running service and is intended to be captured into the admin SDK workspace before generation."
        },
        "servers": [
            {
                "url": "/"
            }
        ],
        "tags": [
            {
                "name": "meta",
                "description": "Service metadata and runtime health endpoints."
            },
            {
                "name": "protocol",
                "description": "Protocol registry and protocol governance surfaces."
            },
            {
                "name": "providers",
                "description": "Provider registry, binding, history, diff, preview, and rollback surfaces."
            },
            {
                "name": "nodes",
                "description": "Realtime node lifecycle and route migration control surfaces."
            }
        ],
        "components": control_plane_openapi_components(),
        "paths": control_plane_openapi_paths()
    })
}

pub fn render_openapi_document() -> serde_json::Value {
    control_plane_openapi_document()
}

pub(crate) fn control_plane_openapi_spec() -> OpenApiServiceSpec<'static> {
    OpenApiServiceSpec {
        title: "Control Plane API",
        version: env!("CARGO_PKG_VERSION"),
        description: "Detailed OpenAPI contract for the Sdkwork IM control-plane runtime, including protocol governance, provider policy, and node lifecycle operations.",
        openapi_path: "/openapi.json",
        docs_path: "/docs",
    }
}

pub fn export_openapi_document() -> Result<serde_json::Value, String> {
    Ok(control_plane_openapi_document())
}

pub fn export_openapi_spec() -> OpenApiServiceSpec<'static> {
    control_plane_openapi_spec()
}
