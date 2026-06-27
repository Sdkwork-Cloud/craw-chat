//! OpenAPI document generation and schema helpers for the automation service.

use sdkwork_im_api_registry::HttpMethod;
use sdkwork_im_openapi::{
    OpenApiServiceSpec, build_openapi_document, extract_routes_from_function, render_docs_html,
};

pub(crate) fn build_automation_service_openapi_document() -> Result<serde_json::Value, String> {
    let routes = extract_routes_from_function(
        include_str!("app.rs"),
        "build_business_router",
        &[],
        &["/openapi.json", "/docs"],
    )?;

    Ok(build_openapi_document(
        &automation_service_openapi_spec(),
        &routes,
        automation_service_tag,
        automation_service_requires_app_context,
        automation_service_summary,
    ))
}

pub(crate) fn automation_service_openapi_spec() -> OpenApiServiceSpec<'static> {
    OpenApiServiceSpec {
        title: "Sdkwork IM Automation Service API",
        version: env!("CARGO_PKG_VERSION"),
        description: "Live OpenAPI contract generated from the automation-service router for execution requests, governance inspection, agent response streams, tool call workflows, and execution lookup flows.",
        openapi_path: "/openapi.json",
        docs_path: "/docs",
    }
}

fn automation_service_tag(path: &str, _method: HttpMethod) -> String {
    match path {
        "/healthz" | "/readyz" => "system".to_owned(),
        path if path.contains("governance") => "governance".to_owned(),
        path if path.contains("agent_tool_calls") => "agent_tool_calls".to_owned(),
        path if path.contains("agent_responses") => "agent_responses".to_owned(),
        _ => "automation".to_owned(),
    }
}

fn automation_service_requires_app_context(path: &str, _method: HttpMethod) -> bool {
    !matches!(path, "/healthz" | "/readyz")
}

fn automation_service_summary(path: &str, method: HttpMethod) -> String {
    match (path, method) {
        ("/healthz", HttpMethod::Get) => "Check automation service health".to_owned(),
        ("/readyz", HttpMethod::Get) => "Check automation service readiness".to_owned(),
        _ => format!(
            "{} {}",
            automation_service_method_display(method),
            path.trim_matches('/').replace('/', " ")
        ),
    }
}

fn automation_service_method_display(method: HttpMethod) -> &'static str {
    match method {
        HttpMethod::Delete => "Delete",
        HttpMethod::Get => "Get",
        HttpMethod::Head => "Head",
        HttpMethod::Options => "Options",
        HttpMethod::Patch => "Patch",
        HttpMethod::Post => "Post",
        HttpMethod::Put => "Put",
    }
}

pub(crate) fn render_automation_docs_html() -> String {
    render_docs_html(&automation_service_openapi_spec())
}
