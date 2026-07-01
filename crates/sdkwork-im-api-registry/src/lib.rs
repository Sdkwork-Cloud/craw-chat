use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum HttpMethod {
    Delete,
    Get,
    Head,
    Options,
    Patch,
    Post,
    Put,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum RouteProtocol {
    Http,
    Websocket,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum RouteVisibility {
    Public,
    Partner,
    Internal,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SdkTarget {
    SdkworkImSdk,
    SdkworkImAppSdk,
    SdkworkImBackendSdk,
    SdkworkDriveAppSdk,
    SdkworkNotaryAppSdk,
    SdkworkCommerceAppSdk,
    SdkworkMailAppSdk,
    SdkworkCommunityAppSdk,
    SdkworkCourseAppSdk,
    SdkworkKnowledgebaseAppSdk,
    SdkworkVoiceAppSdk,
    None,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ContractKind {
    Sdk,
    UpstreamOperational,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SdkContractSummary {
    pub group_id: String,
    pub contract_kind: ContractKind,
    pub schema_url: String,
    pub api_prefix: String,
    pub sdk_target: SdkTarget,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceDescriptor {
    pub service_id: String,
    pub schema_url: String,
    pub docs_url: String,
    pub health_url: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RouteDescriptor {
    pub service_id: String,
    pub methods: Vec<HttpMethod>,
    pub path_pattern: String,
    pub visibility: RouteVisibility,
    pub sdk_targets: Vec<SdkTarget>,
    pub operation_group: String,
    pub protocol: RouteProtocol,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub websocket_subprotocols: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceSchemaIndexEntry {
    pub service_id: String,
    pub contract_kind: ContractKind,
    pub schema_url: String,
    pub docs_url: String,
    pub visibility: RouteVisibility,
    pub route_count: usize,
    pub operation_groups: Vec<String>,
    pub sdk_targets: Vec<SdkTarget>,
    pub protocols: Vec<RouteProtocol>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub websocket_subprotocols: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RegistryError {
    pub code: &'static str,
    pub message: String,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct RouteRegistry {
    entries: Vec<RouteDescriptor>,
}

impl RouteRegistry {
    pub fn entries(&self) -> &[RouteDescriptor] {
        self.entries.as_slice()
    }

    pub fn resolve(&self, method: HttpMethod, path: &str) -> Option<&RouteDescriptor> {
        let mut best_match = None;
        let mut best_score = 0usize;

        for entry in &self.entries {
            if !entry.methods.contains(&method) {
                continue;
            }

            let Some(score) = route_match_score(entry.path_pattern.as_str(), path) else {
                continue;
            };
            if best_match.is_none() || score > best_score {
                best_match = Some(entry);
                best_score = score;
            }
        }

        best_match
    }
}

pub fn build_registry(entries: Vec<RouteDescriptor>) -> Result<RouteRegistry, RegistryError> {
    let mut seen = BTreeMap::new();
    for entry in &entries {
        if entry.protocol == RouteProtocol::Websocket && entry.websocket_subprotocols.is_empty() {
            return Err(RegistryError {
                code: "missing_websocket_subprotocols",
                message: format!(
                    "websocket route {} owned by {} must declare at least one websocket subprotocol",
                    entry.path_pattern, entry.service_id
                ),
            });
        }

        for method in &entry.methods {
            let key = (*method, entry.path_pattern.as_str());
            if let Some(previous_owner) = seen.insert(key, entry.service_id.as_str()) {
                return Err(RegistryError {
                    code: "duplicate_route_owner",
                    message: format!(
                        "duplicate owner for method/path pair: {:?} {} ({previous_owner} vs {})",
                        method, entry.path_pattern, entry.service_id
                    ),
                });
            }
        }
    }

    Ok(RouteRegistry { entries })
}

pub fn sdk_contract_summaries(base_url: &str) -> Vec<SdkContractSummary> {
    let base_url = base_url.trim_end_matches('/');
    [
        (
            "im-open-api",
            "/im/v3/openapi.json",
            "/im/v3/api",
            SdkTarget::SdkworkImSdk,
        ),
        (
            "im-app-api",
            "/app/v3/openapi.json",
            "/app/v3/api",
            SdkTarget::SdkworkImAppSdk,
        ),
        (
            "im-backend-api",
            "/backend/v3/openapi.json",
            "/backend/v3/api",
            SdkTarget::SdkworkImBackendSdk,
        ),
    ]
    .into_iter()
    .map(
        |(group_id, schema_path, api_prefix, sdk_target)| SdkContractSummary {
            group_id: group_id.to_owned(),
            contract_kind: ContractKind::Sdk,
            schema_url: if base_url.is_empty() {
                schema_path.to_owned()
            } else {
                format!("{base_url}{schema_path}")
            },
            api_prefix: api_prefix.to_owned(),
            sdk_target,
        },
    )
    .collect()
}

fn route_match_score(pattern: &str, path: &str) -> Option<usize> {
    let pattern_segments = split_path_segments(pattern);
    let path_segments = split_path_segments(path);
    let mut score = 0usize;
    let mut index = 0usize;

    while index < pattern_segments.len() {
        let pattern_segment = pattern_segments[index];
        if is_catch_all_segment(pattern_segment) {
            return Some(score);
        }

        let path_segment = *path_segments.get(index)?;
        if pattern_segment == path_segment {
            score += 2;
        } else if is_param_segment(pattern_segment) {
            score += 1;
        } else {
            return None;
        }
        index += 1;
    }

    if index == path_segments.len() {
        Some(score)
    } else {
        None
    }
}

fn split_path_segments(path: &str) -> Vec<&str> {
    path.trim_matches('/')
        .split('/')
        .filter(|segment| !segment.is_empty())
        .collect()
}

fn is_param_segment(segment: &str) -> bool {
    segment.starts_with('{') && segment.ends_with('}')
}

fn is_catch_all_segment(segment: &str) -> bool {
    segment.starts_with("{*") && segment.ends_with('}')
}
