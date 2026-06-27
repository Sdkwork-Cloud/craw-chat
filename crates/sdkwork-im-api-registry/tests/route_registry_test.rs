use sdkwork_im_api_registry::{
    HttpMethod, RouteDescriptor, RouteProtocol, RouteVisibility, SdkTarget, build_registry,
};

fn http_route(service_id: &str, method: HttpMethod, path_pattern: &str) -> RouteDescriptor {
    RouteDescriptor {
        service_id: service_id.to_owned(),
        methods: vec![method],
        path_pattern: path_pattern.to_owned(),
        visibility: RouteVisibility::Internal,
        sdk_targets: vec![SdkTarget::None],
        operation_group: "system".to_owned(),
        protocol: RouteProtocol::Http,
        websocket_subprotocols: Vec::new(),
    }
}

#[test]
fn registry_rejects_duplicate_method_path_owner() {
    let result = build_registry(vec![
        http_route(
            "projection-service",
            HttpMethod::Get,
            "/im/v3/api/chat/conversations/{conversationId}/messages",
        ),
        http_route(
            "conversation-runtime",
            HttpMethod::Get,
            "/im/v3/api/chat/conversations/{conversationId}/messages",
        ),
    ]);

    let error = result.expect_err("duplicate owner should be rejected");
    assert_eq!(error.code, "duplicate_route_owner");
    assert!(
        error
            .message
            .contains("/im/v3/api/chat/conversations/{conversationId}/messages"),
        "unexpected error message: {}",
        error.message
    );
}

#[test]
fn registry_allows_method_level_split_ownership_on_same_path() {
    let registry = build_registry(vec![
        http_route(
            "projection-service",
            HttpMethod::Get,
            "/im/v3/api/chat/conversations/{conversationId}/messages",
        ),
        http_route(
            "conversation-runtime",
            HttpMethod::Post,
            "/im/v3/api/chat/conversations/{conversationId}/messages",
        ),
    ])
    .expect("method-level split ownership should be valid");

    let read_owner = registry
        .resolve(
            HttpMethod::Get,
            "/im/v3/api/chat/conversations/{conversationId}/messages",
        )
        .expect("read owner should exist");
    assert_eq!(read_owner.service_id, "projection-service");

    let write_owner = registry
        .resolve(
            HttpMethod::Post,
            "/im/v3/api/chat/conversations/{conversationId}/messages",
        )
        .expect("write owner should exist");
    assert_eq!(write_owner.service_id, "conversation-runtime");
}

#[test]
fn registry_resolves_templated_paths_against_runtime_paths() {
    let registry = build_registry(vec![http_route(
        "projection-service",
        HttpMethod::Get,
        "/im/v3/api/chat/conversations/{conversationId}/messages",
    )])
    .expect("templated path route should be valid");

    let route = registry
        .resolve(
            HttpMethod::Get,
            "/im/v3/api/chat/conversations/c_1/messages",
        )
        .expect("templated route should resolve for runtime path");
    assert_eq!(route.service_id, "projection-service");
}

#[test]
fn registry_preserves_visibility_and_sdk_targets() {
    let registry = build_registry(vec![RouteDescriptor {
        service_id: "session-gateway".to_owned(),
        methods: vec![HttpMethod::Post],
        path_pattern: "/im/v3/api/presence/heartbeat".to_owned(),
        visibility: RouteVisibility::Public,
        sdk_targets: vec![SdkTarget::SdkworkImSdk],
        operation_group: "presence".to_owned(),
        protocol: RouteProtocol::Http,
        websocket_subprotocols: Vec::new(),
    }])
    .expect("public route should build");

    let route = registry
        .resolve(HttpMethod::Post, "/im/v3/api/presence/heartbeat")
        .expect("presence route should exist");
    assert_eq!(route.visibility, RouteVisibility::Public);
    assert_eq!(route.sdk_targets, vec![SdkTarget::SdkworkImSdk]);
    assert_eq!(route.operation_group, "presence");
}

#[test]
fn registry_keeps_websocket_protocol_metadata() {
    let registry = build_registry(vec![RouteDescriptor {
        service_id: "session-gateway".to_owned(),
        methods: vec![HttpMethod::Get],
        path_pattern: "/im/v3/api/realtime/ws".to_owned(),
        visibility: RouteVisibility::Public,
        sdk_targets: vec![SdkTarget::SdkworkImSdk],
        operation_group: "realtime".to_owned(),
        protocol: RouteProtocol::Websocket,
        websocket_subprotocols: vec!["ccp.v1".to_owned()],
    }])
    .expect("websocket route should build");

    let route = registry
        .resolve(HttpMethod::Get, "/im/v3/api/realtime/ws")
        .expect("websocket route should exist");
    assert_eq!(route.protocol, RouteProtocol::Websocket);
    assert_eq!(route.websocket_subprotocols, vec!["ccp.v1".to_owned()]);
}

#[test]
fn registry_rejects_websocket_route_without_subprotocol_metadata() {
    let result = build_registry(vec![RouteDescriptor {
        service_id: "session-gateway".to_owned(),
        methods: vec![HttpMethod::Get],
        path_pattern: "/im/v3/api/realtime/ws".to_owned(),
        visibility: RouteVisibility::Public,
        sdk_targets: vec![SdkTarget::SdkworkImSdk],
        operation_group: "realtime".to_owned(),
        protocol: RouteProtocol::Websocket,
        websocket_subprotocols: Vec::new(),
    }]);

    let error = result.expect_err("websocket route should require subprotocol metadata");
    assert_eq!(error.code, "missing_websocket_subprotocols");
}
