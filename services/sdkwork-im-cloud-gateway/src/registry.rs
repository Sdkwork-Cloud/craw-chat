//! Gateway route registry construction: declares every upstream route
//! descriptor and assembles the [`RouteRegistry`] used for proxy resolution.

use sdkwork_im_api_registry::{
    HttpMethod, RouteDescriptor, RouteProtocol, RouteRegistry, RouteVisibility, SdkTarget,
    build_registry,
};
use sdkwork_im_realtime_api_paths::REALTIME_WS;
use sdkwork_im_runtime_link::LINK_WEBSOCKET_SUBPROTOCOL;

use crate::constants::{COMMERCE_T1_ROUTE_GROUPS, COURSE_APP_API_SEGMENTS};

pub fn build_gateway_registry() -> Result<RouteRegistry, String> {
    build_registry(gateway_route_descriptors()).map_err(|error| error.message)
}

fn gateway_route_descriptors() -> Vec<RouteDescriptor> {
    let mut entries = Vec::new();

    entries.extend(prefix_routes(
        "session-gateway",
        vec![HttpMethod::Get, HttpMethod::Post],
        &["/im/v3/api/presence/{*path}"],
        RouteVisibility::Public,
        vec![SdkTarget::SdkworkImSdk],
        "presence",
    ));
    entries.extend(prefix_routes(
        "session-gateway",
        vec![HttpMethod::Get, HttpMethod::Post],
        &["/im/v3/api/realtime/{*path}"],
        RouteVisibility::Public,
        vec![SdkTarget::SdkworkImSdk],
        "realtime",
    ));
    entries.push(websocket_route(
        "session-gateway",
        REALTIME_WS,
        RouteVisibility::Public,
        vec![SdkTarget::SdkworkImSdk],
        "realtime",
        &[LINK_WEBSOCKET_SUBPROTOCOL],
    ));
    entries.extend(prefix_routes(
        "governance-service",
        all_http_methods(),
        &["/backend/v3/api/control/{*path}"],
        RouteVisibility::Internal,
        vec![SdkTarget::SdkworkImBackendSdk],
        "control",
    ));
    entries.extend(exact_routes(
        "comms-conversation-service",
        vec![HttpMethod::Post],
        &["/im/v3/api/chat/conversations", "/im/v3/api/chat/rooms"],
        RouteVisibility::Public,
        vec![SdkTarget::SdkworkImSdk],
        "conversations",
    ));
    entries.extend(prefix_routes(
        "comms-conversation-service",
        vec![HttpMethod::Post],
        &[
            "/im/v3/api/chat/conversations/{*path}",
            "/im/v3/api/chat/messages/{*path}",
            "/im/v3/api/chat/rooms/{*path}",
        ],
        RouteVisibility::Public,
        vec![SdkTarget::SdkworkImSdk],
        "conversations",
    ));
    entries.extend(exact_routes(
        "projection-service",
        vec![HttpMethod::Get],
        &["/im/v3/api/chat/contacts", "/im/v3/api/chat/inbox"],
        RouteVisibility::Public,
        vec![SdkTarget::SdkworkImSdk],
        "conversations",
    ));
    entries.extend(prefix_routes(
        "projection-service",
        vec![HttpMethod::Get],
        &["/im/v3/api/chat/conversations/{*path}"],
        RouteVisibility::Public,
        vec![SdkTarget::SdkworkImSdk],
        "conversations",
    ));
    entries.extend(prefix_routes(
        "comms-conversation-service",
        vec![HttpMethod::Get],
        &["/im/v3/api/chat/rooms/{*path}"],
        RouteVisibility::Public,
        vec![SdkTarget::SdkworkImSdk],
        "conversations",
    ));
    entries.extend(exact_routes(
        "streaming-service",
        vec![HttpMethod::Post],
        &["/im/v3/api/streams"],
        RouteVisibility::Public,
        vec![SdkTarget::SdkworkImSdk],
        "streams",
    ));
    entries.extend(prefix_routes(
        "streaming-service",
        vec![HttpMethod::Get, HttpMethod::Post],
        &["/im/v3/api/streams/{*path}"],
        RouteVisibility::Public,
        vec![SdkTarget::SdkworkImSdk],
        "streams",
    ));
    entries.extend(prefix_routes(
        "im-calls-service",
        vec![HttpMethod::Get, HttpMethod::Post],
        &["/im/v3/api/calls/{*path}"],
        RouteVisibility::Public,
        vec![SdkTarget::SdkworkImSdk],
        "calls",
    ));
    entries.extend(prefix_routes(
        "sdkwork-drive-app-api",
        all_http_methods(),
        &["/app/v3/api/drive/{*path}"],
        RouteVisibility::Public,
        vec![SdkTarget::SdkworkDriveAppSdk],
        "drive",
    ));
    entries.extend(prefix_routes(
        "sdkwork-notary-app-api",
        all_http_methods(),
        &["/app/v3/api/notary/{*path}"],
        RouteVisibility::Public,
        vec![SdkTarget::SdkworkNotaryAppSdk],
        "notary",
    ));
    for (service_id, segments) in COMMERCE_T1_ROUTE_GROUPS {
        let commerce_path_patterns: Vec<String> = segments
            .iter()
            .map(|segment| format!("/app/v3/api/{segment}/{{*path}}"))
            .collect();
        let commerce_paths: Vec<&str> = commerce_path_patterns
            .iter()
            .map(String::as_str)
            .collect();
        entries.extend(prefix_routes(
            service_id,
            all_http_methods(),
            &commerce_paths,
            RouteVisibility::Public,
            vec![SdkTarget::SdkworkCommerceAppSdk],
            "commerce",
        ));
    }
    entries.extend(prefix_routes(
        "sdkwork-mail-app-api",
        all_http_methods(),
        &["/app/v3/api/mail/{*path}"],
        RouteVisibility::Public,
        vec![SdkTarget::SdkworkMailAppSdk],
        "mail",
    ));
    entries.extend(prefix_routes(
        "sdkwork-community-app-api",
        all_http_methods(),
        &["/app/v3/api/community/{*path}"],
        RouteVisibility::Public,
        vec![SdkTarget::SdkworkCommunityAppSdk],
        "community",
    ));
    let course_path_patterns: Vec<String> = COURSE_APP_API_SEGMENTS
        .iter()
        .map(|segment| format!("/app/v3/api/{segment}/{{*path}}"))
        .collect();
    let course_paths: Vec<&str> = course_path_patterns
        .iter()
        .map(String::as_str)
        .collect();
    entries.extend(prefix_routes(
        "sdkwork-course-app-api",
        all_http_methods(),
        &course_paths,
        RouteVisibility::Public,
        vec![SdkTarget::SdkworkCourseAppSdk],
        "course",
    ));
    entries.extend(prefix_routes(
        "sdkwork-knowledgebase-app-api",
        all_http_methods(),
        &["/app/v3/api/knowledge/{*path}"],
        RouteVisibility::Public,
        vec![SdkTarget::SdkworkKnowledgebaseAppSdk],
        "knowledge",
    ));
    entries.extend(prefix_routes(
        "media-service",
        vec![HttpMethod::Get, HttpMethod::Post],
        &["/im/v3/api/media/{*path}"],
        RouteVisibility::Public,
        vec![SdkTarget::SdkworkImSdk],
        "media",
    ));
    entries.extend(prefix_routes(
        "sdkwork-iam-app-api",
        all_http_methods(),
        &[
            "/app/v3/api/auth/{*path}",
            "/app/v3/api/iam/{*path}",
            "/app/v3/api/oauth/{*path}",
            "/app/v3/api/system/iam/{*path}",
        ],
        RouteVisibility::Public,
        vec![SdkTarget::SdkworkImAppSdk],
        "iam",
    ));
    entries.extend(exact_routes(
        "notification-service",
        vec![HttpMethod::Get],
        &["/app/v3/api/notifications"],
        RouteVisibility::Public,
        vec![SdkTarget::SdkworkImAppSdk],
        "notifications",
    ));
    entries.extend(prefix_routes(
        "notification-service",
        vec![HttpMethod::Get, HttpMethod::Post],
        &["/app/v3/api/notifications/{*path}"],
        RouteVisibility::Public,
        vec![SdkTarget::SdkworkImAppSdk],
        "notifications",
    ));
    entries.extend(prefix_routes(
        "automation-service",
        vec![HttpMethod::Get, HttpMethod::Post],
        &["/app/v3/api/automation/{*path}"],
        RouteVisibility::Public,
        vec![SdkTarget::SdkworkImAppSdk],
        "automation",
    ));
    entries.extend(prefix_routes(
        "audit-service",
        vec![HttpMethod::Get, HttpMethod::Post],
        &["/backend/v3/api/audit/{*path}"],
        RouteVisibility::Internal,
        vec![SdkTarget::SdkworkImBackendSdk],
        "audit",
    ));
    entries.extend(prefix_routes(
        "ops-service",
        vec![HttpMethod::Get],
        &["/backend/v3/api/ops/{*path}"],
        RouteVisibility::Internal,
        vec![SdkTarget::SdkworkImBackendSdk],
        "ops",
    ));
    entries.extend(prefix_routes(
        "comms-social-service",
        vec![
            HttpMethod::Get,
            HttpMethod::Post,
            HttpMethod::Delete,
            HttpMethod::Patch,
        ],
        &["/im/v3/api/social/{*path}"],
        RouteVisibility::Public,
        vec![SdkTarget::SdkworkImSdk],
        "social",
    ));
    entries.extend(prefix_routes(
        "comms-space-service",
        vec![
            HttpMethod::Get,
            HttpMethod::Post,
            HttpMethod::Delete,
            HttpMethod::Patch,
        ],
        &["/im/v3/api/spaces/{*path}"],
        RouteVisibility::Public,
        vec![SdkTarget::SdkworkImSdk],
        "spaces",
    ));

    entries
}

fn prefix_routes(
    service_id: &str,
    methods: Vec<HttpMethod>,
    paths: &[&str],
    visibility: RouteVisibility,
    sdk_targets: Vec<SdkTarget>,
    operation_group: &str,
) -> Vec<RouteDescriptor> {
    paths
        .iter()
        .map(|path| {
            route_descriptor(
                service_id,
                methods.clone(),
                path,
                visibility,
                sdk_targets.clone(),
                operation_group,
            )
        })
        .collect()
}

fn exact_routes(
    service_id: &str,
    methods: Vec<HttpMethod>,
    paths: &[&str],
    visibility: RouteVisibility,
    sdk_targets: Vec<SdkTarget>,
    operation_group: &str,
) -> Vec<RouteDescriptor> {
    prefix_routes(
        service_id,
        methods,
        paths,
        visibility,
        sdk_targets,
        operation_group,
    )
}

fn route_descriptor(
    service_id: &str,
    methods: Vec<HttpMethod>,
    path_pattern: &str,
    visibility: RouteVisibility,
    sdk_targets: Vec<SdkTarget>,
    operation_group: &str,
) -> RouteDescriptor {
    route_descriptor_with_protocol(RouteDescriptorSpec {
        service_id,
        methods,
        path_pattern,
        visibility,
        sdk_targets,
        operation_group,
        protocol: RouteProtocol::Http,
        websocket_subprotocols: &[],
    })
}

fn websocket_route(
    service_id: &str,
    path_pattern: &str,
    visibility: RouteVisibility,
    sdk_targets: Vec<SdkTarget>,
    operation_group: &str,
    websocket_subprotocols: &[&str],
) -> RouteDescriptor {
    route_descriptor_with_protocol(RouteDescriptorSpec {
        service_id,
        methods: vec![HttpMethod::Get],
        path_pattern,
        visibility,
        sdk_targets,
        operation_group,
        protocol: RouteProtocol::Websocket,
        websocket_subprotocols,
    })
}

struct RouteDescriptorSpec<'a> {
    service_id: &'a str,
    methods: Vec<HttpMethod>,
    path_pattern: &'a str,
    visibility: RouteVisibility,
    sdk_targets: Vec<SdkTarget>,
    operation_group: &'a str,
    protocol: RouteProtocol,
    websocket_subprotocols: &'a [&'a str],
}

fn route_descriptor_with_protocol(spec: RouteDescriptorSpec<'_>) -> RouteDescriptor {
    RouteDescriptor {
        service_id: spec.service_id.to_owned(),
        methods: spec.methods,
        path_pattern: spec.path_pattern.to_owned(),
        visibility: spec.visibility,
        sdk_targets: spec.sdk_targets,
        operation_group: spec.operation_group.to_owned(),
        protocol: spec.protocol,
        websocket_subprotocols: spec
            .websocket_subprotocols
            .iter()
            .map(|value| (*value).to_owned())
            .collect(),
    }
}

fn all_http_methods() -> Vec<HttpMethod> {
    vec![
        HttpMethod::Delete,
        HttpMethod::Get,
        HttpMethod::Head,
        HttpMethod::Options,
        HttpMethod::Patch,
        HttpMethod::Post,
        HttpMethod::Put,
    ]
}
