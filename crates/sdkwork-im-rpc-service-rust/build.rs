use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use serde_json::Value;

fn main() {
    let crate_dir =
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR is set"));
    let repo_root = crate_dir
        .parent()
        .and_then(Path::parent)
        .expect("crate lives below repository crates directory");
    let manifest_path = repo_root
        .join("sdks")
        .join("sdkwork-im-rpc-sdk")
        .join("rpc")
        .join("sdkwork-im-rpc.manifest.json");
    let proto_root = repo_root
        .join("apis")
        .join("rpc")
        .join("sdkwork")
        .join("communication");

    println!("cargo:rerun-if-changed={}", manifest_path.display());
    for proto_file in collect_proto_files(&proto_root) {
        println!("cargo:rerun-if-changed={}", proto_file.display());
    }

    let manifest_source =
        fs::read_to_string(&manifest_path).expect("sdkwork-im-rpc RPC manifest must be readable");
    let manifest: Value =
        serde_json::from_str(&manifest_source).expect("sdkwork-im-rpc RPC manifest must be JSON");
    let services = manifest
        .get("services")
        .and_then(Value::as_array)
        .expect("sdkwork-im-rpc RPC manifest must declare services");
    let proto_catalog = parse_proto_catalog(&proto_root);

    let mut method_bindings = String::from("&[\n");
    let mut tonic_adapters = String::new();
    let mut method_index = 0usize;

    tonic_adapters.push_str("pub const GENERATED_TONIC_SERVICE_ADAPTER_COUNT: usize = ");
    tonic_adapters.push_str(&services.len().to_string());
    tonic_adapters.push_str(";\n\n");

    let mut router_services: Vec<(String, String)> = Vec::new();

    for service in services {
        let package = required_string(service, "package");
        let service_name = required_string(service, "service");
        let surface = required_string(service, "surface");
        let methods = service
            .get("methods")
            .and_then(Value::as_array)
            .expect("RPC manifest service must declare methods");
        let service_key = format!("{package}.{service_name}");
        let proto_service = proto_catalog
            .iter()
            .find(|candidate| candidate.service_key == service_key)
            .unwrap_or_else(|| panic!("proto catalog must declare RPC service {service_key}"));
        let adapter_name = format!("{service_name}Adapter");
        let service_module = format!("{}_server", pascal_to_snake(service_name));
        let sdk_module = package_to_sdk_module(package);

        tonic_adapters.push_str("#[derive(Clone)]\n");
        tonic_adapters.push_str(&format!("pub struct {adapter_name} {{\n"));
        tonic_adapters
            .push_str("    dispatcher: ::std::sync::Arc<dyn crate::ImRpcRuntimeDispatcher>,\n");
        tonic_adapters.push_str("}\n\n");
        tonic_adapters.push_str(&format!("impl {adapter_name} {{\n"));
        tonic_adapters.push_str("    pub fn new<D>(dispatcher: ::std::sync::Arc<D>) -> Self\n");
        tonic_adapters.push_str("    where\n");
        tonic_adapters.push_str("        D: crate::ImRpcRuntimeDispatcher,\n");
        tonic_adapters.push_str("    {\n");
        tonic_adapters.push_str(
            "        let dispatcher: ::std::sync::Arc<dyn crate::ImRpcRuntimeDispatcher> = dispatcher;\n",
        );
        tonic_adapters.push_str("        Self { dispatcher }\n");
        tonic_adapters.push_str("    }\n");
        tonic_adapters.push_str("}\n\n");
        tonic_adapters.push_str(&format!("impl ::std::fmt::Debug for {adapter_name} {{\n"));
        tonic_adapters.push_str(
            "    fn fmt(&self, formatter: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {\n",
        );
        tonic_adapters.push_str(&format!(
            "        formatter.debug_struct(\"{adapter_name}\").finish_non_exhaustive()\n"
        ));
        tonic_adapters.push_str("    }\n");
        tonic_adapters.push_str("}\n\n");
        tonic_adapters.push_str("#[tonic::async_trait]\n");
        tonic_adapters.push_str(&format!(
            "impl {sdk_module}::{service_module}::{service_name} for {adapter_name} {{\n"
        ));

        for method in methods {
            let method_name = required_string(method, "method");
            let operation_id = required_string(method, "operationId");
            let auth = required_string(method, "auth");
            let idempotency = required_string(method, "idempotency");
            let streaming = required_string(method, "streaming");
            let owner = required_string(method, "owner");
            let compatibility = required_string(method, "compatibility");
            let proto_method = proto_service
                .methods
                .iter()
                .find(|candidate| candidate.name == method_name)
                .unwrap_or_else(|| {
                    panic!("proto service {service_key} must declare RPC method {method_name}")
                });

            method_bindings.push_str("    RpcMethodBinding {\n");
            method_bindings.push_str(&format!(
                "        method_key: \"{package}.{service_name}/{method_name}\",\n"
            ));
            method_bindings.push_str(&format!("        package: \"{package}\",\n"));
            method_bindings.push_str(&format!("        service: \"{service_name}\",\n"));
            method_bindings.push_str(&format!("        method: \"{method_name}\",\n"));
            method_bindings.push_str(&format!("        surface: \"{surface}\",\n"));
            method_bindings.push_str(&format!("        operation_id: \"{operation_id}\",\n"));
            method_bindings.push_str(&format!("        auth: \"{auth}\",\n"));
            method_bindings.push_str(&format!("        idempotency: \"{idempotency}\",\n"));
            method_bindings.push_str(&format!("        streaming: \"{streaming}\",\n"));
            method_bindings.push_str(&format!("        owner: \"{owner}\",\n"));
            method_bindings.push_str(&format!("        compatibility: \"{compatibility}\",\n"));
            method_bindings.push_str("    },\n");

            if proto_method.request_streaming {
                panic!(
                    "client-streaming RPC method {package}.{service_name}/{method_name} is not supported by the current IM RPC adapter generator"
                );
            }

            let method_fn = pascal_to_snake(method_name);
            let request_type = &proto_method.request_type;
            let response_type = &proto_method.response_type;

            if proto_method.response_streaming {
                let stream_type_name = format!("{method_name}Stream");
                tonic_adapters.push_str(&format!(
                    "    type {stream_type_name} = crate::ImRpcResponseStream<{sdk_module}::{response_type}>;\n\n"
                ));
                tonic_adapters.push_str(&format!(
                    "    async fn {method_fn}(\n        &self,\n        request: tonic::Request<{sdk_module}::{request_type}>,\n    ) -> ::std::result::Result<tonic::Response<Self::{stream_type_name}>, tonic::Status> {{\n"
                ));
                tonic_adapters.push_str(&format!(
                    "        crate::dispatch_server_stream_rpc(self.dispatcher.as_ref(), &crate::RPC_METHOD_BINDINGS[{method_index}], request).await\n"
                ));
                tonic_adapters.push_str("    }\n\n");
            } else {
                tonic_adapters.push_str(&format!(
                    "    async fn {method_fn}(\n        &self,\n        request: tonic::Request<{sdk_module}::{request_type}>,\n    ) -> ::std::result::Result<tonic::Response<{sdk_module}::{response_type}>, tonic::Status> {{\n"
                ));
                tonic_adapters.push_str(&format!(
                    "        crate::dispatch_unary_rpc(self.dispatcher.as_ref(), &crate::RPC_METHOD_BINDINGS[{method_index}], request).await\n"
                ));
                tonic_adapters.push_str("    }\n\n");
            }

            method_index += 1;
        }
        tonic_adapters.push_str("}\n\n");

        router_services.push((
            service_key.clone(),
            format!(
                ".add_service({sdk_module}::{service_module}::{service_name}Server::new({adapter_name}::new(dispatcher.clone())))"
            ),
        ));
    }
    method_bindings.push_str("]\n");

    tonic_adapters.push_str("pub const IM_RPC_SERVICE_KEYS: &[&str] = &[\n");
    for (service_key, _) in &router_services {
        tonic_adapters.push_str(&format!("    \"{service_key}\",\n"));
    }
    tonic_adapters.push_str("];\n\n");

    tonic_adapters.push_str("pub fn build_im_rpc_service_router<D>(\n");
    tonic_adapters.push_str("    dispatcher: ::std::sync::Arc<D>,\n");
    tonic_adapters.push_str(") -> tonic::transport::server::Router\n");
    tonic_adapters.push_str("where\n");
    tonic_adapters.push_str("    D: crate::ImRpcRuntimeDispatcher,\n");
    tonic_adapters.push_str("{\n");
    tonic_adapters.push_str("    let config = crate::ImRpcServerConfig::local_default();\n");
    tonic_adapters.push_str("    build_im_rpc_service_router_with_config(&config, dispatcher)\n");
    tonic_adapters.push_str("}\n\n");

    tonic_adapters.push_str("pub fn build_im_rpc_service_router_with_config<D>(\n");
    tonic_adapters.push_str("    config: &crate::ImRpcServerConfig,\n");
    tonic_adapters.push_str("    dispatcher: ::std::sync::Arc<D>,\n");
    tonic_adapters.push_str(") -> tonic::transport::server::Router\n");
    tonic_adapters.push_str("where\n");
    tonic_adapters.push_str("    D: crate::ImRpcRuntimeDispatcher,\n");
    tonic_adapters.push_str("{\n");
    tonic_adapters.push_str(
        "    let mut server = tonic::transport::Server::builder().timeout(config.default_deadline.as_duration());\n",
    );
    tonic_adapters.push_str("    let router = add_all_im_rpc_services(&mut server, dispatcher);\n");
    tonic_adapters.push_str("    if config.enable_health {\n");
    tonic_adapters.push_str("        router.add_service(crate::build_im_rpc_health_server())\n");
    tonic_adapters.push_str("    } else {\n");
    tonic_adapters.push_str("        router\n");
    tonic_adapters.push_str("    }\n");
    tonic_adapters.push_str("}\n\n");

    tonic_adapters.push_str("pub fn build_im_rpc_service_router_with_config_for_services<D>(\n");
    tonic_adapters.push_str("    config: &crate::ImRpcServerConfig,\n");
    tonic_adapters.push_str("    dispatcher: ::std::sync::Arc<D>,\n");
    tonic_adapters.push_str("    service_keys: &[&str],\n");
    tonic_adapters.push_str(") -> tonic::transport::server::Router\n");
    tonic_adapters.push_str("where\n");
    tonic_adapters.push_str("    D: crate::ImRpcRuntimeDispatcher,\n");
    tonic_adapters.push_str("{\n");
    tonic_adapters.push_str(
        "    let mut server = tonic::transport::Server::builder().timeout(config.default_deadline.as_duration());\n",
    );
    tonic_adapters
        .push_str("    let router = add_im_rpc_services(&mut server, dispatcher, service_keys);\n");
    tonic_adapters.push_str("    if config.enable_health {\n");
    tonic_adapters.push_str("        router.add_service(crate::build_im_rpc_health_server())\n");
    tonic_adapters.push_str("    } else {\n");
    tonic_adapters.push_str("        router\n");
    tonic_adapters.push_str("    }\n");
    tonic_adapters.push_str("}\n\n");

    tonic_adapters.push_str("pub fn add_all_im_rpc_services<D>(\n");
    tonic_adapters.push_str("    server: &mut tonic::transport::Server,\n");
    tonic_adapters.push_str("    dispatcher: ::std::sync::Arc<D>,\n");
    tonic_adapters.push_str(") -> tonic::transport::server::Router\n");
    tonic_adapters.push_str("where\n");
    tonic_adapters.push_str("    D: crate::ImRpcRuntimeDispatcher,\n");
    tonic_adapters.push_str("{\n");
    tonic_adapters.push_str("    add_im_rpc_services(server, dispatcher, IM_RPC_SERVICE_KEYS)\n");
    tonic_adapters.push_str("}\n\n");

    tonic_adapters.push_str("pub fn add_im_rpc_services<D>(\n");
    tonic_adapters.push_str("    server: &mut tonic::transport::Server,\n");
    tonic_adapters.push_str("    dispatcher: ::std::sync::Arc<D>,\n");
    tonic_adapters.push_str("    service_keys: &[&str],\n");
    tonic_adapters.push_str(") -> tonic::transport::server::Router\n");
    tonic_adapters.push_str("where\n");
    tonic_adapters.push_str("    D: crate::ImRpcRuntimeDispatcher,\n");
    tonic_adapters.push_str("{\n");
    tonic_adapters.push_str("    use ::std::collections::HashSet;\n");
    tonic_adapters
        .push_str("    let selected: HashSet<&str> = service_keys.iter().copied().collect();\n");
    tonic_adapters.push_str("    assert!(!selected.is_empty(), \"at least one IM RPC service key must be selected\");\n");
    tonic_adapters
        .push_str("    let mut router: Option<tonic::transport::server::Router> = None;\n");
    for (service_key, service_registration) in &router_services {
        tonic_adapters.push_str(&format!("    if selected.contains(\"{service_key}\") {{\n"));
        tonic_adapters.push_str("        router = Some(match router {\n");
        tonic_adapters.push_str("            None => server");
        tonic_adapters.push_str(service_registration);
        tonic_adapters.push_str(",\n");
        tonic_adapters.push_str("            Some(existing) => existing");
        tonic_adapters.push_str(service_registration);
        tonic_adapters.push_str(",\n");
        tonic_adapters.push_str("        });\n");
        tonic_adapters.push_str("    }\n");
    }
    tonic_adapters.push_str("    router.expect(\"filtered IM RPC router must register at least one selected service\")\n");
    tonic_adapters.push_str("}\n");

    let output_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR is set"));
    fs::write(output_dir.join("rpc_method_bindings.rs"), method_bindings)
        .expect("generated RPC method bindings must be writable");
    fs::write(
        output_dir.join("rpc_tonic_service_adapters.rs"),
        tonic_adapters,
    )
    .expect("generated RPC tonic service adapters must be writable");
}

fn required_string<'a>(value: &'a Value, field: &str) -> &'a str {
    value
        .get(field)
        .and_then(Value::as_str)
        .unwrap_or_else(|| panic!("RPC manifest entry must declare string field {field}"))
}

#[derive(Clone, Debug)]
struct ProtoService {
    service_key: String,
    methods: Vec<ProtoMethod>,
}

#[derive(Clone, Debug)]
struct ProtoMethod {
    name: String,
    request_type: String,
    response_type: String,
    request_streaming: bool,
    response_streaming: bool,
}

fn collect_proto_files(root: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    collect_proto_files_into(root, &mut files);
    files.sort();
    files
}

fn collect_proto_files_into(root: &Path, files: &mut Vec<PathBuf>) {
    for entry in fs::read_dir(root).unwrap_or_else(|_| panic!("proto root must exist: {root:?}")) {
        let path = entry
            .expect("proto directory entry must be readable")
            .path();
        if path.is_dir() {
            collect_proto_files_into(&path, files);
        } else if path
            .extension()
            .is_some_and(|extension| extension == "proto")
        {
            files.push(path);
        }
    }
}

fn parse_proto_catalog(proto_root: &Path) -> Vec<ProtoService> {
    let mut services = Vec::new();

    for proto_file in collect_proto_files(proto_root) {
        let source = fs::read_to_string(&proto_file)
            .unwrap_or_else(|_| panic!("proto file must be readable: {proto_file:?}"));
        let package = parse_proto_package(&source)
            .unwrap_or_else(|| panic!("proto file must declare a package: {proto_file:?}"));
        let mut current_service: Option<ProtoService> = None;
        let mut pending_rpc = String::new();

        for raw_line in source.lines() {
            let line = raw_line
                .split_once("//")
                .map_or(raw_line, |(prefix, _)| prefix)
                .trim();
            if line.is_empty() {
                continue;
            }

            if let Some(service_name) = line
                .strip_prefix("service ")
                .and_then(|rest| rest.split_whitespace().next())
            {
                if let Some(service) = current_service.take() {
                    services.push(service);
                }
                current_service = Some(ProtoService {
                    service_key: format!("{package}.{service_name}"),
                    methods: Vec::new(),
                });
                continue;
            }

            if current_service.is_none() {
                continue;
            }

            if line.starts_with("rpc ") || !pending_rpc.is_empty() {
                if !pending_rpc.is_empty() {
                    pending_rpc.push(' ');
                }
                pending_rpc.push_str(line);

                if pending_rpc.ends_with(';') {
                    let method = parse_rpc_statement(&pending_rpc).unwrap_or_else(|| {
                        panic!("RPC statement must be parseable: {pending_rpc}")
                    });
                    current_service
                        .as_mut()
                        .expect("current proto service exists")
                        .methods
                        .push(method);
                    pending_rpc.clear();
                }
            }

            if line == "}"
                && let Some(service) = current_service.take()
            {
                services.push(service);
            }
        }

        if let Some(service) = current_service.take() {
            services.push(service);
        }
    }

    services
}

fn parse_proto_package(source: &str) -> Option<String> {
    source.lines().find_map(|line| {
        let trimmed = line.trim();
        trimmed
            .strip_prefix("package ")
            .and_then(|rest| rest.strip_suffix(';'))
            .map(str::trim)
            .map(str::to_owned)
    })
}

fn parse_rpc_statement(statement: &str) -> Option<ProtoMethod> {
    let statement = statement.trim().trim_end_matches(';').trim();
    let after_rpc = statement.strip_prefix("rpc ")?;
    let (method_name, after_method) = after_rpc.split_once('(')?;
    let (request_segment, after_request) = after_method.split_once(')')?;
    let after_returns = after_request.trim().strip_prefix("returns")?.trim();
    let after_response_start = after_returns.strip_prefix('(')?;
    let (response_segment, _) = after_response_start.split_once(')')?;

    let (request_streaming, request_type) = parse_rpc_type_segment(request_segment);
    let (response_streaming, response_type) = parse_rpc_type_segment(response_segment);

    Some(ProtoMethod {
        name: method_name.trim().to_owned(),
        request_type,
        response_type,
        request_streaming,
        response_streaming,
    })
}

fn parse_rpc_type_segment(segment: &str) -> (bool, String) {
    let trimmed = segment.trim();
    if let Some(rest) = trimmed.strip_prefix("stream ") {
        (true, rest.trim().to_owned())
    } else {
        (false, trimmed.to_owned())
    }
}

fn package_to_sdk_module(package: &str) -> String {
    let mut segments = package.split('.');
    let root = segments.next().expect("proto package has root segment");
    if root != "sdkwork" {
        panic!("IM RPC proto package must start with sdkwork: {package}");
    }

    let mut module_path = String::from("::sdkwork_im_rpc_sdk_rust::sdkwork");
    for segment in segments {
        module_path.push_str("::");
        module_path.push_str(segment);
    }
    module_path
}

fn pascal_to_snake(value: &str) -> String {
    let chars: Vec<char> = value.chars().collect();
    let mut output = String::new();

    for (index, character) in chars.iter().copied().enumerate() {
        if character.is_ascii_uppercase() {
            let previous = index
                .checked_sub(1)
                .and_then(|previous| chars.get(previous));
            let next = chars.get(index + 1);
            let needs_separator = previous.is_some_and(|previous| {
                previous.is_ascii_lowercase()
                    || previous.is_ascii_digit()
                    || (previous.is_ascii_uppercase()
                        && next.is_some_and(|next| next.is_ascii_lowercase()))
            });

            if needs_separator {
                output.push('_');
            }
            output.push(character.to_ascii_lowercase());
        } else {
            output.push(character);
        }
    }

    output
}
