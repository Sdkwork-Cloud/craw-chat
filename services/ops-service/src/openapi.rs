use std::collections::{BTreeSet, VecDeque};
use std::sync::OnceLock;

use axum::Json;
use axum::response::Html;
use sdkwork_im_api_registry::HttpMethod;
use sdkwork_im_openapi::{
    OpenApiServiceSpec, build_openapi_document, extract_routes_from_function, render_docs_html,
};

use crate::error::OpsError;

pub(crate) async fn openapi_json() -> Result<Json<serde_json::Value>, OpsError> {
    Ok(Json(build_ops_service_openapi_document().map_err(
        |message| OpsError::internal("openapi_export_failed", message),
    )?))
}

pub(crate) async fn docs() -> Html<String> {
    Html(render_docs_html(&ops_service_openapi_spec()))
}

fn build_ops_service_openapi_document() -> Result<serde_json::Value, String> {
    static DOCUMENT: OnceLock<Result<serde_json::Value, String>> = OnceLock::new();

    DOCUMENT
        .get_or_init(build_ops_service_openapi_document_uncached)
        .clone()
}

fn build_ops_service_openapi_document_uncached() -> Result<serde_json::Value, String> {
    let source = include_str!("app.rs");
    let mut routes = extract_routes_from_function(
        source,
        "build_business_router",
        &[],
        &["/openapi.json", "/docs"],
    )?;
    routes.extend(extract_routes_from_function(
        source,
        "build_domain_api_router",
        &[],
        &[],
    )?);

    let mut document = build_openapi_document(
        &ops_service_openapi_spec(),
        &routes,
        ops_service_tag,
        ops_service_requires_app_context,
        ops_service_summary,
    );
    let authority = serde_yaml::from_str::<serde_json::Value>(include_str!(
        "../../../apis/backend-api/communication/sdkwork-im-backend-api.openapi.yaml"
    ))
    .map_err(|error| format!("failed to parse backend OpenAPI authority: {error}"))?;

    merge_authoritative_operations(&mut document, &authority)?;
    Ok(document)
}

fn merge_authoritative_operations(
    document: &mut serde_json::Value,
    authority: &serde_json::Value,
) -> Result<(), String> {
    let authority_paths = authority
        .get("paths")
        .and_then(serde_json::Value::as_object)
        .ok_or_else(|| "backend OpenAPI authority has no paths object".to_owned())?;
    let document_paths = document
        .get_mut("paths")
        .and_then(serde_json::Value::as_object_mut)
        .ok_or_else(|| "generated ops OpenAPI document has no paths object".to_owned())?;

    for (path, path_item) in document_paths {
        let Some(authority_path_item) = authority_paths
            .get(path)
            .and_then(serde_json::Value::as_object)
        else {
            continue;
        };
        let Some(path_operations) = path_item.as_object_mut() else {
            continue;
        };

        for (method, operation) in path_operations {
            if let Some(authoritative_operation) = authority_path_item.get(method) {
                *operation = authoritative_operation.clone();
            }
        }
    }

    merge_referenced_components(document, authority)
}

fn merge_referenced_components(
    document: &mut serde_json::Value,
    authority: &serde_json::Value,
) -> Result<(), String> {
    let authority_components = authority
        .get("components")
        .and_then(serde_json::Value::as_object)
        .ok_or_else(|| "backend OpenAPI authority has no components object".to_owned())?;
    let mut pending = VecDeque::new();
    collect_component_refs(document, &mut pending);
    let mut visited = BTreeSet::new();
    let mut selected = serde_json::Map::new();

    while let Some(reference) = pending.pop_front() {
        if !visited.insert(reference.clone()) {
            continue;
        }
        let Some(component_path) = reference.strip_prefix("#/components/") else {
            continue;
        };
        let Some((section, name)) = component_path.split_once('/') else {
            return Err(format!("invalid component reference `{reference}`"));
        };
        let component = authority_components
            .get(section)
            .and_then(serde_json::Value::as_object)
            .and_then(|components| components.get(name))
            .ok_or_else(|| format!("authority component `{reference}` does not exist"))?
            .clone();
        collect_component_refs(&component, &mut pending);
        selected
            .entry(section.to_owned())
            .or_insert_with(|| serde_json::Value::Object(serde_json::Map::new()))
            .as_object_mut()
            .expect("selected component section is always an object")
            .insert(name.to_owned(), component);
    }

    let document_components = document
        .as_object_mut()
        .ok_or_else(|| "generated ops OpenAPI document is not an object".to_owned())?
        .entry("components")
        .or_insert_with(|| serde_json::Value::Object(serde_json::Map::new()))
        .as_object_mut()
        .ok_or_else(|| "generated ops OpenAPI components is not an object".to_owned())?;
    for (section, components) in selected {
        let target = document_components
            .entry(section)
            .or_insert_with(|| serde_json::Value::Object(serde_json::Map::new()))
            .as_object_mut()
            .ok_or_else(|| "generated ops OpenAPI component section is not an object".to_owned())?;
        target.extend(
            components
                .as_object()
                .expect("selected component section is always an object")
                .clone(),
        );
    }
    Ok(())
}

fn collect_component_refs(value: &serde_json::Value, output: &mut VecDeque<String>) {
    match value {
        serde_json::Value::Array(values) => {
            for value in values {
                collect_component_refs(value, output);
            }
        }
        serde_json::Value::Object(object) => {
            if let Some(reference) = object.get("$ref").and_then(serde_json::Value::as_str)
                && reference.starts_with("#/components/")
            {
                output.push_back(reference.to_owned());
            }
            for value in object.values() {
                collect_component_refs(value, output);
            }
        }
        _ => {}
    }
}

fn ops_service_openapi_spec() -> OpenApiServiceSpec<'static> {
    OpenApiServiceSpec {
        title: "Sdkwork IM Ops Service API",
        version: env!("CARGO_PKG_VERSION"),
        description: "Live OpenAPI contract generated from the ops-service router for cluster, lag, diagnostics, replay_status, runtime_dir, retention, and provider binding inspections.",
        openapi_path: "/openapi.json",
        docs_path: "/docs",
    }
}

fn ops_service_tag(path: &str, _method: HttpMethod) -> String {
    match path {
        "/healthz" | "/readyz" | "/metrics" => "system".to_owned(),
        path if path.contains("provider_bindings") => "provider_bindings".to_owned(),
        path if path.contains("diagnostics") => "diagnostics".to_owned(),
        path if path.contains("retention") => "retention".to_owned(),
        _ => "ops".to_owned(),
    }
}

fn ops_service_requires_app_context(path: &str, _method: HttpMethod) -> bool {
    !matches!(path, "/healthz" | "/readyz" | "/metrics")
}

fn ops_service_summary(path: &str, method: HttpMethod) -> String {
    match (path, method) {
        ("/healthz", HttpMethod::Get) => "Check ops service health".to_owned(),
        ("/readyz", HttpMethod::Get) => "Check ops service readiness".to_owned(),
        _ => format!(
            "{} {}",
            ops_service_method_display(method),
            path.trim_matches('/').replace('/', " ")
        ),
    }
}

fn ops_service_method_display(method: HttpMethod) -> &'static str {
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
