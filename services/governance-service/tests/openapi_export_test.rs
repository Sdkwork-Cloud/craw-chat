#[test]
fn test_library_exports_control_plane_openapi_document() {
    let document =
        governance_service::export_openapi_document().expect("openapi document should export");

    assert_eq!(document["openapi"], "3.1.2");
    assert_eq!(document["info"]["title"], "Control Plane API");
    assert!(document["paths"]["/backend/v3/api/control/protocol_registry"].is_object());
}

#[test]
fn test_control_plane_openapi_uses_sdkwork_dual_token_security() {
    let document =
        governance_service::export_openapi_document().expect("openapi document should export");

    let schemes = document
        .pointer("/components/securitySchemes")
        .and_then(serde_json::Value::as_object)
        .expect("control plane OpenAPI should expose security schemes");

    assert!(
        schemes.get("bearerAuth").is_none(),
        "legacy bearerAuth must not be emitted by backend API OpenAPI"
    );
    assert_eq!(
        schemes
            .get("AuthToken")
            .and_then(|scheme| scheme.get("type"))
            .and_then(serde_json::Value::as_str),
        Some("http")
    );
    assert_eq!(
        schemes
            .get("AccessToken")
            .and_then(|scheme| scheme.get("name"))
            .and_then(serde_json::Value::as_str),
        Some("Access-Token")
    );

    let protected_operation = document
        .pointer("/paths/~1backend~1v3~1api~1control~1protocol_registry/get/security")
        .and_then(serde_json::Value::as_array)
        .expect("protected operation should define security");
    assert!(
        protected_operation
            .iter()
            .any(|entry| entry.get("AuthToken").is_some() && entry.get("AccessToken").is_some()),
        "backend API protected operations must require SDKWork AuthToken plus AccessToken"
    );
}

#[test]
fn test_control_plane_openapi_error_responses_use_problem_json() {
    let document =
        governance_service::export_openapi_document().expect("openapi document should export");

    let error_response = document
        .pointer(
            "/paths/~1backend~1v3~1api~1control~1provider_registry/get/responses/403/content/application~1problem+json/schema/$ref",
        )
        .and_then(serde_json::Value::as_str)
        .expect("403 response should expose application/problem+json schema");
    assert_eq!(
        error_response,
        "#/components/schemas/ControlPlaneErrorResponse"
    );

    let required = document
        .pointer("/components/schemas/ControlPlaneErrorResponse/required")
        .and_then(serde_json::Value::as_array)
        .expect("ControlPlaneErrorResponse should define required fields");
    assert!(
        required.iter().any(|value| value == "type")
            && required.iter().any(|value| value == "title")
            && required.iter().any(|value| value == "status"),
        "ControlPlaneErrorResponse should require ProblemDetail core fields"
    );

    assert_eq!(
        document
            .pointer("/components/schemas/ControlPlaneErrorResponse/properties/status/type")
            .and_then(serde_json::Value::as_str),
        Some("integer")
    );
    assert_eq!(
        document
            .pointer("/components/schemas/ControlPlaneErrorResponse/properties/errorStatus/type")
            .and_then(serde_json::Value::as_str),
        Some("string")
    );
}
