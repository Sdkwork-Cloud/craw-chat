use axum::{
    body::Body,
    http::{header, HeaderMap, Method, StatusCode, Uri},
    response::{IntoResponse, Response},
    Json,
};
use bytes::Bytes;
use im_adapters_local_disk::FileStorageDomainSnapshotStore;
use im_storage_contracts::{
    ContractError, StorageCatalog, StorageConfigUpsertInput, StorageDomainSnapshot,
    StorageDomainSnapshotStore,
};
use im_storage_runtime::{storage_config_upsert_from_input, StoreBackedStorageRuntime};
use serde::Deserialize;
use serde_json::{json, Value};
use std::path::PathBuf;
use std::sync::{Arc, Mutex, MutexGuard};

const ADMIN_SANDBOX_SEED_JSON: &str = include_str!("admin_sandbox_seed.json");
const BACKEND_ADMIN_API_PREFIX: &str = "/backend/v3/api/admin";
pub const BACKEND_ADMIN_API_ROUTES: &[&str] = &[
    "/backend/v3/api/admin/api_key_groups",
    "/backend/v3/api/admin/api_key_groups/{groupId}",
    "/backend/v3/api/admin/api_key_groups/{groupId}/status",
    "/backend/v3/api/admin/api_keys",
    "/backend/v3/api/admin/api_keys/{hashedKey}",
    "/backend/v3/api/admin/api_keys/{hashedKey}/status",
    "/backend/v3/api/admin/billing/events",
    "/backend/v3/api/admin/billing/events/summary",
    "/backend/v3/api/admin/billing/summary",
    "/backend/v3/api/admin/channel_models",
    "/backend/v3/api/admin/channel_models/{channelId}/models/{modelId}",
    "/backend/v3/api/admin/channels",
    "/backend/v3/api/admin/channels/{channelId}",
    "/backend/v3/api/admin/credentials",
    "/backend/v3/api/admin/credentials/{tenantId}/providers/{providerId}/keys/{keyReference}",
    "/backend/v3/api/admin/extensions/runtime_reloads",
    "/backend/v3/api/admin/extensions/runtime_statuses",
    "/backend/v3/api/admin/gateway/rate_limit_policies",
    "/backend/v3/api/admin/gateway/rate_limit_windows",
    "/backend/v3/api/admin/marketing/campaigns",
    "/backend/v3/api/admin/marketing/campaigns/{marketingCampaignId}/status",
    "/backend/v3/api/admin/model_prices",
    "/backend/v3/api/admin/model_prices/{channelId}/models/{modelId}/providers/{proxyProviderId}",
    "/backend/v3/api/admin/models",
    "/backend/v3/api/admin/models/{externalName}/providers/{providerId}",
    "/backend/v3/api/admin/providers",
    "/backend/v3/api/admin/providers/{providerId}",
    "/backend/v3/api/admin/routing/decision_logs",
    "/backend/v3/api/admin/routing/health_snapshots",
    "/backend/v3/api/admin/routing/profiles",
    "/backend/v3/api/admin/routing/snapshots",
    "/backend/v3/api/admin/storage/audit",
    "/backend/v3/api/admin/storage/config",
    "/backend/v3/api/admin/storage/config/tenants/{tenantId}",
    "/backend/v3/api/admin/storage/effective/tenants/{tenantId}",
    "/backend/v3/api/admin/storage/providers",
    "/backend/v3/api/admin/storage/validate",
    "/backend/v3/api/admin/storage/validate/tenants/{tenantId}",
    "/backend/v3/api/admin/usage/records",
    "/backend/v3/api/admin/usage/summary",
];

#[derive(Clone)]
pub struct SharedAdminSandboxState {
    inner: Arc<Mutex<AdminSandboxState>>,
}

#[derive(Clone)]
struct AdminSandboxState {
    store: Value,
    storage_runtime: StoreBackedStorageRuntime<AdminSandboxStorageStore>,
    clock_ms: u64,
    sequence: u64,
}

#[derive(Clone, Debug)]
enum AdminSandboxStorageStore {
    Ephemeral,
    File(FileStorageDomainSnapshotStore),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AdminSandboxSeed {
    clock_ms: u64,
    #[serde(flatten)]
    store: std::collections::BTreeMap<String, Value>,
}

type SandboxErrorResponse = Box<Response>;

impl SharedAdminSandboxState {
    pub fn seeded() -> Self {
        Self::seeded_from_store(None)
    }

    pub fn seeded_with_storage_file(file_path: impl Into<PathBuf>) -> Self {
        Self::seeded_from_store(Some(file_path.into()))
    }

    fn seeded_from_store(storage_file_path: Option<PathBuf>) -> Self {
        Self {
            inner: Arc::new(Mutex::new(AdminSandboxState::seeded(storage_file_path))),
        }
    }

    fn lock(&self, operation: &'static str) -> MutexGuard<'_, AdminSandboxState> {
        match self.inner.lock() {
            Ok(guard) => guard,
            Err(poisoned) => {
                eprintln!(
                    "warning: recovering poisoned sdkwork admin sandbox state lock during {operation}"
                );
                poisoned.into_inner()
            }
        }
    }
}

impl AdminSandboxState {
    fn seeded(storage_file_path: Option<PathBuf>) -> Self {
        let seed: AdminSandboxSeed =
            serde_json::from_str(ADMIN_SANDBOX_SEED_JSON).expect("admin sandbox seed should parse");
        let mut store = Value::Object(seed.store.into_iter().collect());
        sync_provider_credential_readiness(&mut store);
        let storage_runtime = StoreBackedStorageRuntime::load(
            storage_file_path
                .map(FileStorageDomainSnapshotStore::new)
                .map(AdminSandboxStorageStore::File)
                .unwrap_or(AdminSandboxStorageStore::Ephemeral),
            StorageCatalog::object_storage(),
        )
        .expect("admin sandbox storage snapshot should load");

        Self {
            store,
            storage_runtime,
            clock_ms: seed.clock_ms,
            sequence: 0,
        }
    }

    fn next_timestamp(&mut self) -> u64 {
        self.clock_ms += 60_000;
        self.clock_ms
    }

    fn next_sequence(&mut self) -> String {
        self.sequence += 1;
        format!("{:04}", self.sequence)
    }

    fn next_id(&mut self, prefix: &str) -> String {
        format!("{prefix}_{}", self.next_sequence())
    }
}

impl StorageDomainSnapshotStore for AdminSandboxStorageStore {
    fn load_snapshot(&self, domain: &str) -> Result<Option<StorageDomainSnapshot>, ContractError> {
        match self {
            Self::Ephemeral => Ok(None),
            Self::File(store) => store.load_snapshot(domain),
        }
    }

    fn save_snapshot(&self, snapshot: StorageDomainSnapshot) -> Result<(), ContractError> {
        match self {
            Self::Ephemeral => Ok(()),
            Self::File(store) => store.save_snapshot(snapshot),
        }
    }
}

fn json_response(status: StatusCode, payload: Value) -> Response {
    (status, Json(payload)).into_response()
}

fn json_error_response(status: StatusCode, message: &str) -> Response {
    json_response(
        status,
        json!({
            "error": {
                "message": message,
            },
            "status": status.as_u16(),
        }),
    )
}

fn empty_response(status: StatusCode) -> Response {
    Response::builder()
        .status(status)
        .body(Body::empty())
        .expect("empty sandbox response should build")
}

fn admin_api_segments(uri: &Uri) -> Vec<String> {
    uri.path()
        .trim_start_matches(BACKEND_ADMIN_API_PREFIX)
        .trim_matches('/')
        .split('/')
        .filter(|segment| !segment.is_empty())
        .map(|segment| segment.to_owned())
        .collect()
}

fn parse_body(body: &Bytes) -> Result<Value, SandboxErrorResponse> {
    if body.is_empty() {
        return Ok(json!({}));
    }

    serde_json::from_slice(body).map_err(|_| {
        Box::new(json_error_response(
            StatusCode::BAD_REQUEST,
            "Admin sandbox request body must be valid JSON.",
        ))
    })
}

fn store_value<'a>(store: &'a Value, key: &str) -> &'a Value {
    store.get(key).unwrap_or(&Value::Null)
}

fn array_ref<'a>(store: &'a Value, key: &str) -> &'a Vec<Value> {
    store_value(store, key)
        .as_array()
        .expect("sandbox array should exist")
}

fn array_mut<'a>(store: &'a mut Value, key: &str) -> &'a mut Vec<Value> {
    store
        .get_mut(key)
        .and_then(Value::as_array_mut)
        .expect("sandbox mutable array should exist")
}

fn string_field<'a>(value: &'a Value, key: &str) -> Option<&'a str> {
    value.get(key).and_then(Value::as_str)
}

fn bool_field(value: &Value, key: &str) -> Option<bool> {
    value.get(key).and_then(Value::as_bool)
}

fn require_appbase_bearer(headers: &HeaderMap) -> Result<(), SandboxErrorResponse> {
    let header_value = headers
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .unwrap_or_default();

    let Some(token) = header_value.strip_prefix("Bearer ").map(str::trim) else {
        return Err(Box::new(json_error_response(
            StatusCode::UNAUTHORIZED,
            "Admin sandbox requires an sdkwork-appbase bearer token.",
        )));
    };

    if token.is_empty() {
        return Err(Box::new(json_error_response(
            StatusCode::UNAUTHORIZED,
            "Admin sandbox requires an sdkwork-appbase bearer token.",
        )));
    }

    Ok(())
}

fn find_index_by(records: &[Value], key: &str, needle: &str) -> Option<usize> {
    records
        .iter()
        .position(|record| string_field(record, key) == Some(needle))
}

fn remove_by(records: &mut Vec<Value>, key: &str, needle: &str) {
    records.retain(|record| string_field(record, key) != Some(needle));
}

fn sync_provider_credential_readiness(store: &mut Value) {
    let ready_provider_ids = array_ref(store, "credentials")
        .iter()
        .filter_map(|credential| string_field(credential, "provider_id"))
        .map(str::to_owned)
        .collect::<Vec<_>>();

    for provider in array_mut(store, "providers") {
        if let Some(provider_id) = string_field(provider, "id").map(str::to_owned) {
            provider["credential_readiness"] = json!({
                "ready": ready_provider_ids.iter().any(|candidate| candidate == &provider_id),
                "state": if ready_provider_ids.iter().any(|candidate| candidate == &provider_id) {
                    "ready"
                } else {
                    "missing"
                }
            });
        }
    }
}

fn object_response(value: Value) -> Response {
    json_response(StatusCode::OK, value)
}

fn list_response(store: &Value, key: &str) -> Response {
    json_response(StatusCode::OK, store_value(store, key).clone())
}

fn upsert_record<F>(records: &mut Vec<Value>, mut predicate: F, next_record: Value) -> Value
where
    F: FnMut(&Value) -> bool,
{
    if let Some(index) = records.iter().position(&mut predicate) {
        records[index] = next_record.clone();
        return records[index].clone();
    }

    records.push(next_record.clone());
    next_record
}

fn remove_where<F>(records: &mut Vec<Value>, mut predicate: F)
where
    F: FnMut(&Value) -> bool,
{
    records.retain(|record| !predicate(record));
}

fn value_or_null(value: &Value, key: &str) -> Value {
    value.get(key).cloned().unwrap_or(Value::Null)
}

fn trimmed_string_field(value: &Value, key: &str) -> Option<String> {
    string_field(value, key)
        .map(str::trim)
        .filter(|candidate| !candidate.is_empty())
        .map(str::to_owned)
}

fn string_or_null(value: &Value, key: &str) -> Value {
    trimmed_string_field(value, key)
        .map(Value::String)
        .unwrap_or(Value::Null)
}

fn bool_or_default(value: &Value, key: &str, default: bool) -> bool {
    bool_field(value, key).unwrap_or(default)
}

fn value_to_json<T>(value: &T) -> Value
where
    T: ?Sized + serde::Serialize,
{
    serde_json::to_value(value).expect("admin sandbox value should serialize")
}

fn slugify(value: &str) -> String {
    let mut slug = String::new();
    let mut last_was_dash = true;

    for ch in value.chars() {
        if ch.is_ascii_alphanumeric() {
            slug.push(ch.to_ascii_lowercase());
            last_was_dash = false;
        } else if !last_was_dash {
            slug.push('-');
            last_was_dash = true;
        }
    }

    while slug.ends_with('-') {
        slug.pop();
    }

    if slug.is_empty() {
        "sandbox".to_owned()
    } else {
        slug
    }
}

fn merge_objects(base: &Value, overlay: &Value) -> Value {
    match (base.as_object(), overlay.as_object()) {
        (Some(base_object), Some(overlay_object)) => {
            let mut merged = base_object.clone();
            for (key, value) in overlay_object {
                merged.insert(key.clone(), value.clone());
            }
            Value::Object(merged)
        }
        _ => overlay.clone(),
    }
}

fn existing_record_by_key(
    store: &Value,
    store_key: &str,
    field: &str,
    needle: &str,
) -> Option<Value> {
    array_ref(store, store_key)
        .iter()
        .find(|record| string_field(record, field) == Some(needle))
        .cloned()
}

fn save_marketing_campaign(state: &mut AdminSandboxState, input: &Value) -> Value {
    let campaign_id = string_field(input, "marketing_campaign_id")
        .unwrap_or_default()
        .to_owned();
    let existing = existing_record_by_key(
        &state.store,
        "marketingCampaigns",
        "marketing_campaign_id",
        &campaign_id,
    );
    let created_at_ms = existing
        .as_ref()
        .and_then(|record| record.get("created_at_ms").cloned())
        .unwrap_or_else(|| json!(state.next_timestamp()));
    let record = json!({
        "marketing_campaign_id": campaign_id,
        "display_name": string_field(input, "display_name").unwrap_or_default(),
        "status": string_field(input, "status").unwrap_or_default(),
        "start_at_ms": value_or_null(input, "start_at_ms"),
        "end_at_ms": value_or_null(input, "end_at_ms"),
        "created_at_ms": created_at_ms,
        "updated_at_ms": json!(state.next_timestamp()),
    });
    let campaign_id = string_field(&record, "marketing_campaign_id")
        .expect("marketing campaign should include an id")
        .to_owned();
    let records = array_mut(&mut state.store, "marketingCampaigns");
    upsert_record(
        records,
        |campaign| string_field(campaign, "marketing_campaign_id") == Some(campaign_id.as_str()),
        record,
    )
}

fn save_api_key_group(
    state: &mut AdminSandboxState,
    input: &Value,
    group_id_override: Option<&str>,
) -> Value {
    let group_id = group_id_override
        .map(str::to_owned)
        .or_else(|| trimmed_string_field(input, "group_id"))
        .unwrap_or_else(|| state.next_id("group"));
    let existing = existing_record_by_key(&state.store, "apiKeyGroups", "group_id", &group_id);
    let name = string_field(input, "name").unwrap_or_default();
    let slug = trimmed_string_field(input, "slug").unwrap_or_else(|| slugify(name));
    let created_at_ms = existing
        .as_ref()
        .and_then(|record| record.get("created_at_ms").cloned())
        .unwrap_or_else(|| json!(state.next_timestamp()));
    let active = bool_field(input, "active")
        .or_else(|| {
            existing
                .as_ref()
                .and_then(|record| bool_field(record, "active"))
        })
        .unwrap_or(true);
    let record = json!({
        "group_id": group_id,
        "tenant_id": string_field(input, "tenant_id").unwrap_or_default(),
        "project_id": string_field(input, "project_id").unwrap_or_default(),
        "environment": string_field(input, "environment").unwrap_or_default(),
        "name": name,
        "slug": slug,
        "description": string_or_null(input, "description"),
        "color": string_or_null(input, "color"),
        "default_capability_scope": string_or_null(input, "default_capability_scope"),
        "default_accounting_mode": string_or_null(input, "default_accounting_mode"),
        "default_routing_profile_id": string_or_null(input, "default_routing_profile_id"),
        "active": active,
        "created_at_ms": created_at_ms,
        "updated_at_ms": json!(state.next_timestamp()),
    });
    let group_id = string_field(&record, "group_id")
        .expect("api key group should include an id")
        .to_owned();
    let records = array_mut(&mut state.store, "apiKeyGroups");
    upsert_record(
        records,
        |group| string_field(group, "group_id") == Some(group_id.as_str()),
        record,
    )
}

fn save_routing_profile(state: &mut AdminSandboxState, input: &Value) -> Value {
    let profile_id =
        trimmed_string_field(input, "profile_id").unwrap_or_else(|| state.next_id("routing"));
    let existing =
        existing_record_by_key(&state.store, "routingProfiles", "profile_id", &profile_id);
    let name = string_field(input, "name").unwrap_or_default();
    let slug = trimmed_string_field(input, "slug").unwrap_or_else(|| slugify(name));
    let created_at_ms = existing
        .as_ref()
        .and_then(|record| record.get("created_at_ms").cloned())
        .unwrap_or_else(|| json!(state.next_timestamp()));
    let active = bool_field(input, "active")
        .or_else(|| {
            existing
                .as_ref()
                .and_then(|record| bool_field(record, "active"))
        })
        .unwrap_or(true);
    let require_healthy = bool_field(input, "require_healthy")
        .or_else(|| {
            existing
                .as_ref()
                .and_then(|record| bool_field(record, "require_healthy"))
        })
        .unwrap_or(true);
    let record = json!({
        "profile_id": profile_id,
        "tenant_id": string_field(input, "tenant_id").unwrap_or_default(),
        "project_id": string_field(input, "project_id").unwrap_or_default(),
        "name": name,
        "slug": slug,
        "description": string_or_null(input, "description"),
        "active": active,
        "strategy": trimmed_string_field(input, "strategy").unwrap_or_else(|| "priority".to_owned()),
        "ordered_provider_ids": input.get("ordered_provider_ids").cloned().unwrap_or_else(|| json!([])),
        "default_provider_id": string_or_null(input, "default_provider_id"),
        "max_cost": value_or_null(input, "max_cost"),
        "max_latency_ms": value_or_null(input, "max_latency_ms"),
        "require_healthy": require_healthy,
        "preferred_region": string_or_null(input, "preferred_region"),
        "created_at_ms": created_at_ms,
        "updated_at_ms": json!(state.next_timestamp()),
    });
    let profile_id = string_field(&record, "profile_id")
        .expect("routing profile should include an id")
        .to_owned();
    let records = array_mut(&mut state.store, "routingProfiles");
    upsert_record(
        records,
        |profile| string_field(profile, "profile_id") == Some(profile_id.as_str()),
        record,
    )
}

fn create_api_key_record(state: &mut AdminSandboxState, input: &Value) -> Value {
    let sequence = state.next_sequence();
    let hashed_key = format!("sandbox_hash_{sequence}");
    let plaintext = trimmed_string_field(input, "plaintext_key")
        .unwrap_or_else(|| format!("sandbox_sk_{sequence}"));
    let created_at_ms = state.next_timestamp();
    let label =
        trimmed_string_field(input, "label").unwrap_or_else(|| format!("Sandbox key {sequence}"));
    let notes = trimmed_string_field(input, "notes")
        .map(Value::String)
        .unwrap_or(Value::Null);
    let record = json!({
        "tenant_id": string_field(input, "tenant_id").unwrap_or_default(),
        "project_id": string_field(input, "project_id").unwrap_or_default(),
        "environment": string_field(input, "environment").unwrap_or_default(),
        "hashed_key": hashed_key,
        "api_key_group_id": string_or_null(input, "api_key_group_id"),
        "label": label,
        "notes": notes,
        "created_at_ms": created_at_ms,
        "last_used_at_ms": Value::Null,
        "expires_at_ms": value_or_null(input, "expires_at_ms"),
        "active": true,
    });
    array_mut(&mut state.store, "apiKeys").push(record.clone());

    json!({
        "plaintext": plaintext,
        "hashed": string_field(&record, "hashed_key").unwrap_or_default(),
        "tenant_id": string_field(&record, "tenant_id").unwrap_or_default(),
        "project_id": string_field(&record, "project_id").unwrap_or_default(),
        "environment": string_field(&record, "environment").unwrap_or_default(),
        "api_key_group_id": value_or_null(&record, "api_key_group_id"),
        "label": string_field(&record, "label").unwrap_or_default(),
        "notes": value_or_null(&record, "notes"),
        "created_at_ms": created_at_ms,
        "expires_at_ms": value_or_null(&record, "expires_at_ms"),
    })
}

fn save_api_key_update(state: &mut AdminSandboxState, hashed_key: &str, input: &Value) -> Value {
    let existing = existing_record_by_key(&state.store, "apiKeys", "hashed_key", hashed_key);
    let created_at_ms = existing
        .as_ref()
        .and_then(|record| record.get("created_at_ms").cloned())
        .unwrap_or_else(|| json!(state.next_timestamp()));
    let last_used_at_ms = existing
        .as_ref()
        .and_then(|record| record.get("last_used_at_ms").cloned())
        .unwrap_or(Value::Null);
    let active = existing
        .as_ref()
        .and_then(|record| bool_field(record, "active"))
        .unwrap_or(true);
    let record = json!({
        "hashed_key": hashed_key,
        "tenant_id": string_field(input, "tenant_id").unwrap_or_default(),
        "project_id": string_field(input, "project_id").unwrap_or_default(),
        "environment": string_field(input, "environment").unwrap_or_default(),
        "label": string_field(input, "label").unwrap_or_default(),
        "notes": string_or_null(input, "notes"),
        "expires_at_ms": value_or_null(input, "expires_at_ms"),
        "api_key_group_id": string_or_null(input, "api_key_group_id"),
        "active": active,
        "created_at_ms": created_at_ms,
        "last_used_at_ms": last_used_at_ms,
    });
    let records = array_mut(&mut state.store, "apiKeys");
    upsert_record(
        records,
        |api_key| string_field(api_key, "hashed_key") == Some(hashed_key),
        record,
    )
}

fn save_channel(state: &mut AdminSandboxState, input: &Value) -> Value {
    let channel_id = string_field(input, "id").unwrap_or_default().to_owned();
    let record = json!({
        "id": channel_id,
        "name": string_field(input, "name").unwrap_or_default(),
    });
    let records = array_mut(&mut state.store, "channels");
    upsert_record(
        records,
        |channel| string_field(channel, "id") == Some(channel_id.as_str()),
        record,
    )
}

fn build_provider_catalog_record(existing: Option<&Value>, input: &Value) -> Value {
    let provider_id = string_field(input, "id").unwrap_or_default();
    let extension_id = trimmed_string_field(input, "extension_id").or_else(|| {
        existing.and_then(|record| string_field(record, "extension_id").map(str::to_owned))
    });
    let protocol_kind = trimmed_string_field(input, "protocol_kind")
        .or_else(|| {
            existing.and_then(|record| string_field(record, "protocol_kind").map(str::to_owned))
        })
        .unwrap_or_else(|| "openai".to_owned());
    let adapter_kind = trimmed_string_field(input, "adapter_kind")
        .or_else(|| {
            existing.and_then(|record| string_field(record, "adapter_kind").map(str::to_owned))
        })
        .unwrap_or_else(|| "openai-compatible".to_owned());
    let channel_bindings = input
        .get("channel_bindings")
        .cloned()
        .or_else(|| existing.and_then(|record| record.get("channel_bindings").cloned()))
        .unwrap_or_else(|| {
            json!([{
                "channel_id": string_field(input, "channel_id").unwrap_or_default(),
                "is_primary": true,
            }])
        });
    let integration = existing
        .and_then(|record| record.get("integration").cloned())
        .unwrap_or_else(|| {
            json!({
                "mode": "standard_passthrough",
                "default_plugin_family": protocol_kind.clone(),
            })
        });
    let execution = existing
        .and_then(|record| record.get("execution").cloned())
        .unwrap_or_else(|| {
            json!({
                "binding_kind": "provider",
                "runtime": "sandbox-runtime",
                "runtime_key": extension_id.clone().unwrap_or_else(|| provider_id.to_owned()),
                "passthrough_protocol": protocol_kind.clone(),
                "supports_provider_adapter": true,
                "supports_raw_plugin": true,
                "fail_closed": true,
                "route_readiness": {
                    "openai": {
                        "executable": true,
                        "supported": true,
                    },
                    "anthropic": {
                        "executable": false,
                        "supported": false,
                    },
                    "gemini": {
                        "executable": false,
                        "supported": false,
                    },
                },
            })
        });
    let credential_readiness = existing
        .and_then(|record| record.get("credential_readiness").cloned())
        .unwrap_or_else(|| {
            json!({
                "ready": false,
                "state": "missing",
            })
        });

    json!({
        "id": provider_id,
        "channel_id": string_field(input, "channel_id").unwrap_or_default(),
        "extension_id": extension_id.map(Value::String).unwrap_or(Value::Null),
        "adapter_kind": adapter_kind,
        "protocol_kind": protocol_kind,
        "base_url": trimmed_string_field(input, "base_url")
            .map(Value::String)
            .or_else(|| existing.and_then(|record| record.get("base_url").cloned()))
            .unwrap_or(Value::Null),
        "display_name": string_field(input, "display_name").unwrap_or_default(),
        "channel_bindings": channel_bindings,
        "integration": integration,
        "execution": execution,
        "credential_readiness": credential_readiness,
    })
}

fn save_provider(state: &mut AdminSandboxState, input: &Value) -> Value {
    let provider_id = string_field(input, "id").unwrap_or_default().to_owned();
    let existing = existing_record_by_key(&state.store, "providers", "id", &provider_id);
    let record = build_provider_catalog_record(existing.as_ref(), input);
    {
        let records = array_mut(&mut state.store, "providers");
        upsert_record(
            records,
            |provider| string_field(provider, "id") == Some(provider_id.as_str()),
            record,
        );
    }
    sync_provider_credential_readiness(&mut state.store);
    existing_record_by_key(&state.store, "providers", "id", &provider_id)
        .expect("saved provider should be present in the store")
}

fn build_credential_record(input: &Value) -> Value {
    json!({
        "tenant_id": string_field(input, "tenant_id").unwrap_or_default(),
        "provider_id": string_field(input, "provider_id").unwrap_or_default(),
        "key_reference": string_field(input, "key_reference").unwrap_or_default(),
        "secret_backend": "sandbox-inmemory",
        "secret_local_file": Value::Null,
        "secret_keyring_service": Value::Null,
        "secret_master_key_id": Value::Null,
    })
}

fn save_credential(state: &mut AdminSandboxState, input: &Value) -> Value {
    let record = build_credential_record(input);
    let tenant_id = string_field(&record, "tenant_id")
        .unwrap_or_default()
        .to_owned();
    let provider_id = string_field(&record, "provider_id")
        .unwrap_or_default()
        .to_owned();
    let key_reference = string_field(&record, "key_reference")
        .unwrap_or_default()
        .to_owned();
    {
        let records = array_mut(&mut state.store, "credentials");
        upsert_record(
            records,
            |credential| {
                string_field(credential, "tenant_id") == Some(tenant_id.as_str())
                    && string_field(credential, "provider_id") == Some(provider_id.as_str())
                    && string_field(credential, "key_reference") == Some(key_reference.as_str())
            },
            record.clone(),
        );
    }
    sync_provider_credential_readiness(&mut state.store);
    record
}

fn save_model(state: &mut AdminSandboxState, input: &Value) -> Value {
    let external_name = string_field(input, "external_name")
        .unwrap_or_default()
        .to_owned();
    let provider_id = string_field(input, "provider_id")
        .unwrap_or_default()
        .to_owned();
    let record = json!({
        "external_name": external_name,
        "provider_id": provider_id,
        "capabilities": input.get("capabilities").cloned().unwrap_or_else(|| json!([])),
        "streaming": bool_or_default(input, "streaming", false),
        "context_window": value_or_null(input, "context_window"),
    });
    let records = array_mut(&mut state.store, "models");
    upsert_record(
        records,
        |model| {
            string_field(model, "external_name") == Some(external_name.as_str())
                && string_field(model, "provider_id") == Some(provider_id.as_str())
        },
        record,
    )
}

fn save_channel_model(state: &mut AdminSandboxState, input: &Value) -> Value {
    let channel_id = string_field(input, "channel_id")
        .unwrap_or_default()
        .to_owned();
    let model_id = string_field(input, "model_id")
        .unwrap_or_default()
        .to_owned();
    let record = json!({
        "channel_id": channel_id,
        "model_id": model_id,
        "model_display_name": string_field(input, "model_display_name").unwrap_or_default(),
        "capabilities": input.get("capabilities").cloned().unwrap_or_else(|| json!([])),
        "streaming": bool_or_default(input, "streaming", false),
        "context_window": value_or_null(input, "context_window"),
        "description": string_or_null(input, "description"),
    });
    let records = array_mut(&mut state.store, "channelModels");
    upsert_record(
        records,
        |channel_model| {
            string_field(channel_model, "channel_id") == Some(channel_id.as_str())
                && string_field(channel_model, "model_id") == Some(model_id.as_str())
        },
        record,
    )
}

fn save_model_price(state: &mut AdminSandboxState, input: &Value) -> Value {
    let channel_id = string_field(input, "channel_id")
        .unwrap_or_default()
        .to_owned();
    let model_id = string_field(input, "model_id")
        .unwrap_or_default()
        .to_owned();
    let proxy_provider_id = string_field(input, "proxy_provider_id")
        .unwrap_or_default()
        .to_owned();
    let record = json!({
        "channel_id": channel_id,
        "model_id": model_id,
        "proxy_provider_id": proxy_provider_id,
        "currency_code": string_field(input, "currency_code").unwrap_or_default(),
        "price_unit": string_field(input, "price_unit").unwrap_or_default(),
        "input_price": value_or_null(input, "input_price"),
        "output_price": value_or_null(input, "output_price"),
        "cache_read_price": value_or_null(input, "cache_read_price"),
        "cache_write_price": value_or_null(input, "cache_write_price"),
        "request_price": value_or_null(input, "request_price"),
        "is_active": bool_or_default(input, "is_active", true),
    });
    let records = array_mut(&mut state.store, "modelPrices");
    upsert_record(
        records,
        |price| {
            string_field(price, "channel_id") == Some(channel_id.as_str())
                && string_field(price, "model_id") == Some(model_id.as_str())
                && string_field(price, "proxy_provider_id") == Some(proxy_provider_id.as_str())
        },
        record,
    )
}

fn build_rate_limit_window(state: &mut AdminSandboxState, policy: &Value) -> Value {
    let window_seconds = policy
        .get("window_seconds")
        .and_then(Value::as_u64)
        .unwrap_or(60);
    let window_start_ms = state.next_timestamp();

    json!({
        "policy_id": string_field(policy, "policy_id").unwrap_or_default(),
        "project_id": string_or_null(policy, "project_id"),
        "api_key_hash": string_or_null(policy, "api_key_hash"),
        "route_key": string_or_null(policy, "route_key"),
        "model_name": string_or_null(policy, "model_name"),
        "requests_per_window": policy.get("requests_per_window").cloned().unwrap_or_else(|| json!(0)),
        "window_seconds": window_seconds,
        "burst_requests": policy.get("burst_requests").cloned().unwrap_or_else(|| json!(0)),
        "limit_requests": policy.get("limit_requests").cloned().unwrap_or_else(|| json!(0)),
        "request_count": 0,
        "remaining_requests": policy.get("limit_requests").cloned().unwrap_or_else(|| json!(0)),
        "window_start_ms": window_start_ms,
        "window_end_ms": window_start_ms + window_seconds * 1000,
        "updated_at_ms": window_start_ms,
        "enabled": policy.get("enabled").cloned().unwrap_or(Value::Bool(true)),
        "exceeded": false,
    })
}

fn save_rate_limit_policy(state: &mut AdminSandboxState, input: &Value) -> Value {
    let policy_id = string_field(input, "policy_id")
        .unwrap_or_default()
        .to_owned();
    let existing =
        existing_record_by_key(&state.store, "rateLimitPolicies", "policy_id", &policy_id);
    let created_at_ms = existing
        .as_ref()
        .and_then(|record| record.get("created_at_ms").cloned())
        .unwrap_or_else(|| json!(state.next_timestamp()));
    let record = json!({
        "policy_id": policy_id,
        "project_id": string_field(input, "project_id").unwrap_or_default(),
        "api_key_hash": string_or_null(input, "api_key_hash"),
        "route_key": string_or_null(input, "route_key"),
        "model_name": string_or_null(input, "model_name"),
        "requests_per_window": value_or_null(input, "requests_per_window"),
        "window_seconds": input.get("window_seconds").cloned().unwrap_or_else(|| json!(60)),
        "burst_requests": value_or_null(input, "burst_requests"),
        "limit_requests": input.get("requests_per_window").cloned().unwrap_or_else(|| json!(0)),
        "enabled": input.get("enabled").cloned().unwrap_or(Value::Bool(true)),
        "notes": string_or_null(input, "notes"),
        "created_at_ms": created_at_ms,
        "updated_at_ms": json!(state.next_timestamp()),
    });
    let saved = {
        let records = array_mut(&mut state.store, "rateLimitPolicies");
        upsert_record(
            records,
            |policy| string_field(policy, "policy_id") == Some(policy_id.as_str()),
            record,
        )
    };
    let window = build_rate_limit_window(state, &saved);
    {
        let windows = array_mut(&mut state.store, "rateLimitWindows");
        upsert_record(
            windows,
            |policy_window| string_field(policy_window, "policy_id") == Some(policy_id.as_str()),
            window,
        );
    }
    saved
}

fn save_runtime_reload(state: &mut AdminSandboxState, input: &Value) -> Value {
    let reloaded_at_ms = state.next_timestamp();
    let (runtime_count, active_runtime_count, runtime_statuses) = {
        let statuses = array_mut(&mut state.store, "runtimeStatuses");
        for status in statuses.iter_mut() {
            if bool_field(status, "healthy") == Some(true) {
                status["message"] = json!(format!("Sandbox reload completed at {reloaded_at_ms}."));
            }
        }
        let runtime_count = statuses.len();
        let active_runtime_count = statuses
            .iter()
            .filter(|status| bool_field(status, "running") == Some(true))
            .count();
        (runtime_count, active_runtime_count, statuses.clone())
    };

    json!({
        "scope": if input.get("extension_id").is_some() || input.get("instance_id").is_some() {
            "targeted"
        } else {
            "workspace"
        },
        "requested_extension_id": string_or_null(input, "extension_id"),
        "requested_instance_id": string_or_null(input, "instance_id"),
        "resolved_extension_id": string_or_null(input, "extension_id"),
        "discovered_package_count": runtime_count,
        "loadable_package_count": runtime_count,
        "active_runtime_count": active_runtime_count,
        "reloaded_at_ms": reloaded_at_ms,
        "runtime_statuses": runtime_statuses,
    })
}

fn update_marketing_campaign_status(
    state: &mut AdminSandboxState,
    campaign_id: &str,
    status: &str,
) -> Response {
    let updated_campaign = {
        let updated_at_ms = state.next_timestamp();
        let records = array_mut(&mut state.store, "marketingCampaigns");
        let Some(index) = find_index_by(records, "marketing_campaign_id", campaign_id) else {
            return json_error_response(StatusCode::NOT_FOUND, "Marketing campaign not found.");
        };
        records[index]["status"] = json!(status);
        records[index]["updated_at_ms"] = json!(updated_at_ms);
        records[index].clone()
    };

    object_response(updated_campaign)
}

fn update_api_key_group_status(
    state: &mut AdminSandboxState,
    group_id: &str,
    active: bool,
) -> Response {
    let updated_group = {
        let updated_at_ms = state.next_timestamp();
        let records = array_mut(&mut state.store, "apiKeyGroups");
        let Some(index) = find_index_by(records, "group_id", group_id) else {
            return json_error_response(StatusCode::NOT_FOUND, "API key group not found.");
        };
        records[index]["active"] = json!(active);
        records[index]["updated_at_ms"] = json!(updated_at_ms);
        records[index].clone()
    };

    object_response(updated_group)
}

fn update_api_key_status(
    state: &mut AdminSandboxState,
    hashed_key: &str,
    active: bool,
) -> Response {
    let updated_api_key = {
        let records = array_mut(&mut state.store, "apiKeys");
        let Some(index) = find_index_by(records, "hashed_key", hashed_key) else {
            return json_error_response(StatusCode::NOT_FOUND, "API key not found.");
        };
        records[index]["active"] = json!(active);
        records[index].clone()
    };

    object_response(updated_api_key)
}

fn cascade_delete_provider(store: &mut Value, provider_id: &str) {
    remove_by(array_mut(store, "providers"), "id", provider_id);
    remove_where(array_mut(store, "credentials"), |credential| {
        string_field(credential, "provider_id") == Some(provider_id)
    });
    remove_where(array_mut(store, "models"), |model| {
        string_field(model, "provider_id") == Some(provider_id)
    });
    remove_where(array_mut(store, "modelPrices"), |price| {
        string_field(price, "proxy_provider_id") == Some(provider_id)
    });
    remove_where(array_mut(store, "providerHealth"), |snapshot| {
        string_field(snapshot, "provider_id") == Some(provider_id)
    });
    sync_provider_credential_readiness(store);
}

fn cascade_delete_model(store: &mut Value, external_name: &str, provider_id: &str) {
    remove_where(array_mut(store, "models"), |model| {
        string_field(model, "external_name") == Some(external_name)
            && string_field(model, "provider_id") == Some(provider_id)
    });
    remove_where(array_mut(store, "channelModels"), |model| {
        string_field(model, "model_id") == Some(external_name)
    });
    remove_where(array_mut(store, "modelPrices"), |price| {
        string_field(price, "model_id") == Some(external_name)
    });
}

pub async fn handle_admin_sandbox_request(
    state: &SharedAdminSandboxState,
    method: Method,
    headers: HeaderMap,
    uri: Uri,
    body: Bytes,
) -> Response {
    debug_assert!(
        BACKEND_ADMIN_API_ROUTES
            .iter()
            .all(|route| route.starts_with(BACKEND_ADMIN_API_PREFIX)),
        "admin sandbox route registry must stay under the backend admin API prefix"
    );

    let segments = admin_api_segments(&uri);
    let segment_refs = segments.iter().map(String::as_str).collect::<Vec<_>>();

    let mut guard = state.lock("handle_admin_sandbox_request");

    if let Err(response) = require_appbase_bearer(&headers) {
        return *response;
    }

    if method == Method::GET {
        match segment_refs.as_slice() {
            ["marketing", "campaigns"] => return list_response(&guard.store, "marketingCampaigns"),
            ["api_keys"] => return list_response(&guard.store, "apiKeys"),
            ["api_key_groups"] => return list_response(&guard.store, "apiKeyGroups"),
            ["routing", "profiles"] => return list_response(&guard.store, "routingProfiles"),
            ["routing", "snapshots"] => {
                return list_response(&guard.store, "compiledRoutingSnapshots");
            }
            ["channels"] => return list_response(&guard.store, "channels"),
            ["providers"] => return list_response(&guard.store, "providers"),
            ["credentials"] => return list_response(&guard.store, "credentials"),
            ["models"] => return list_response(&guard.store, "models"),
            ["channel_models"] => return list_response(&guard.store, "channelModels"),
            ["model_prices"] => return list_response(&guard.store, "modelPrices"),
            ["usage", "records"] => return list_response(&guard.store, "usageRecords"),
            ["usage", "summary"] => {
                return object_response(store_value(&guard.store, "usageSummary").clone());
            }
            ["billing", "summary"] => {
                return object_response(store_value(&guard.store, "billingSummary").clone());
            }
            ["billing", "events"] => return list_response(&guard.store, "billingEvents"),
            ["billing", "events", "summary"] => {
                return object_response(store_value(&guard.store, "billingEventSummary").clone());
            }
            ["routing", "decision_logs"] => return list_response(&guard.store, "routingLogs"),
            ["gateway", "rate_limit_policies"] => {
                return list_response(&guard.store, "rateLimitPolicies");
            }
            ["gateway", "rate_limit_windows"] => {
                return list_response(&guard.store, "rateLimitWindows");
            }
            ["routing", "health_snapshots"] => {
                return list_response(&guard.store, "providerHealth");
            }
            ["extensions", "runtime_statuses"] => {
                return list_response(&guard.store, "runtimeStatuses");
            }
            ["storage", "providers"] => {
                return object_response(value_to_json(
                    &guard.storage_runtime.catalog().provider_schemas,
                ));
            }
            ["storage", "config"] => {
                return object_response(value_to_json(&guard.storage_runtime.global_snapshot()));
            }
            ["storage", "audit"] => {
                return object_response(value_to_json(guard.storage_runtime.audit_trail()));
            }
            ["storage", "config", "tenants", tenant_id] => {
                return object_response(value_to_json(
                    &guard.storage_runtime.tenant_snapshot(tenant_id),
                ));
            }
            ["storage", "effective", "tenants", tenant_id] => {
                let Some(effective) = guard.storage_runtime.effective_for_tenant(tenant_id) else {
                    return json_error_response(
                        StatusCode::NOT_FOUND,
                        &format!("Storage config not found for tenant {tenant_id}."),
                    );
                };
                return object_response(value_to_json(&effective));
            }
            _ => {}
        }
    }

    let input = match parse_body(&body) {
        Ok(input) => input,
        Err(response) => return *response,
    };

    if method == Method::POST {
        match segment_refs.as_slice() {
            ["marketing", "campaigns"] => {
                return object_response(save_marketing_campaign(&mut guard, &input));
            }
            ["api_key_groups"] => {
                return object_response(save_api_key_group(&mut guard, &input, None));
            }
            ["routing", "profiles"] => {
                return object_response(save_routing_profile(&mut guard, &input));
            }
            ["api_keys"] => return object_response(create_api_key_record(&mut guard, &input)),
            ["channels"] => return object_response(save_channel(&mut guard, &input)),
            ["providers"] => return object_response(save_provider(&mut guard, &input)),
            ["credentials"] => return object_response(save_credential(&mut guard, &input)),
            ["models"] => return object_response(save_model(&mut guard, &input)),
            ["channel_models"] => return object_response(save_channel_model(&mut guard, &input)),
            ["model_prices"] => return object_response(save_model_price(&mut guard, &input)),
            ["gateway", "rate_limit_policies"] => {
                return object_response(save_rate_limit_policy(&mut guard, &input));
            }
            ["extensions", "runtime_reloads"] => {
                return object_response(save_runtime_reload(&mut guard, &input));
            }
            ["storage", "config"] => {
                match serde_json::from_value::<StorageConfigUpsertInput>(input.clone()) {
                    Ok(storage_input) => match storage_config_upsert_from_input(
                        guard.storage_runtime.catalog(),
                        &storage_input,
                    ) {
                        Ok(storage_upsert) => {
                            match guard.storage_runtime.save_global(storage_upsert) {
                                Ok(saved_snapshot) => {
                                    return object_response(value_to_json(&saved_snapshot));
                                }
                                Err(error) => {
                                    return json_error_response(
                                        StatusCode::INTERNAL_SERVER_ERROR,
                                        &format!(
                                            "Failed to persist admin sandbox storage snapshot: {error:?}"
                                        ),
                                    );
                                }
                            }
                        }
                        Err(message) => {
                            return json_error_response(StatusCode::BAD_REQUEST, &message);
                        }
                    },
                    Err(_) => {
                        return json_error_response(
                            StatusCode::BAD_REQUEST,
                            "Storage config payload must be valid JSON.",
                        );
                    }
                }
            }
            ["storage", "validate"] => {
                return object_response(value_to_json(&guard.storage_runtime.validate_global()));
            }
            ["storage", "config", "tenants", tenant_id] => {
                match serde_json::from_value::<StorageConfigUpsertInput>(input.clone()) {
                    Ok(storage_input) => match storage_config_upsert_from_input(
                        guard.storage_runtime.catalog(),
                        &storage_input,
                    ) {
                        Ok(storage_upsert) => {
                            match guard
                                .storage_runtime
                                .save_tenant(*tenant_id, storage_upsert)
                            {
                                Ok(saved_snapshot) => {
                                    return object_response(value_to_json(&saved_snapshot));
                                }
                                Err(error) => {
                                    return json_error_response(
                                        StatusCode::INTERNAL_SERVER_ERROR,
                                        &format!(
                                            "Failed to persist admin sandbox storage snapshot: {error:?}"
                                        ),
                                    );
                                }
                            }
                        }
                        Err(message) => {
                            return json_error_response(StatusCode::BAD_REQUEST, &message);
                        }
                    },
                    Err(_) => {
                        return json_error_response(
                            StatusCode::BAD_REQUEST,
                            "Storage config payload must be valid JSON.",
                        );
                    }
                }
            }
            ["storage", "validate", "tenants", tenant_id] => {
                return object_response(value_to_json(
                    &guard.storage_runtime.validate_tenant(tenant_id),
                ));
            }
            _ => {}
        }

        match segment_refs.as_slice() {
            ["marketing", "campaigns", campaign_id, "status"] => {
                return update_marketing_campaign_status(
                    &mut guard,
                    campaign_id,
                    string_field(&input, "status").unwrap_or_default(),
                );
            }
            ["api_key_groups", group_id, "status"] => {
                return update_api_key_group_status(
                    &mut guard,
                    group_id,
                    bool_or_default(&input, "active", false),
                );
            }
            ["api_keys", hashed_key, "status"] => {
                return update_api_key_status(
                    &mut guard,
                    hashed_key,
                    bool_or_default(&input, "active", false),
                );
            }
            _ => {}
        }
    }

    if method == Method::PATCH {
        if let ["api_key_groups", group_id] = segment_refs.as_slice() {
            let Some(existing_group) =
                existing_record_by_key(&guard.store, "apiKeyGroups", "group_id", group_id)
            else {
                return json_error_response(StatusCode::NOT_FOUND, "API key group not found.");
            };
            let merged_input = merge_objects(&existing_group, &input);
            return object_response(save_api_key_group(
                &mut guard,
                &merged_input,
                Some(group_id),
            ));
        }
    }

    if method == Method::PUT {
        if let ["api_keys", hashed_key] = segment_refs.as_slice() {
            return object_response(save_api_key_update(&mut guard, hashed_key, &input));
        }
    }

    if method == Method::DELETE {
        match segment_refs.as_slice() {
            ["storage", "config", "tenants", tenant_id] => {
                if let Err(error) = guard.storage_runtime.delete_tenant(tenant_id) {
                    return json_error_response(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        &format!("Failed to persist admin sandbox storage snapshot: {error:?}"),
                    );
                }
                return empty_response(StatusCode::NO_CONTENT);
            }
            ["api_key_groups", group_id] => {
                remove_by(
                    array_mut(&mut guard.store, "apiKeyGroups"),
                    "group_id",
                    group_id,
                );
                return empty_response(StatusCode::NO_CONTENT);
            }
            ["api_keys", hashed_key] => {
                remove_by(
                    array_mut(&mut guard.store, "apiKeys"),
                    "hashed_key",
                    hashed_key,
                );
                return empty_response(StatusCode::NO_CONTENT);
            }
            ["channels", channel_id] => {
                remove_by(array_mut(&mut guard.store, "channels"), "id", channel_id);
                remove_where(
                    array_mut(&mut guard.store, "channelModels"),
                    |channel_model| string_field(channel_model, "channel_id") == Some(channel_id),
                );
                remove_where(array_mut(&mut guard.store, "modelPrices"), |model_price| {
                    string_field(model_price, "channel_id") == Some(channel_id)
                });
                return empty_response(StatusCode::NO_CONTENT);
            }
            ["providers", provider_id] => {
                cascade_delete_provider(&mut guard.store, provider_id);
                return empty_response(StatusCode::NO_CONTENT);
            }
            ["credentials", tenant_id, "providers", provider_id, "keys", key_reference] => {
                remove_where(array_mut(&mut guard.store, "credentials"), |credential| {
                    string_field(credential, "tenant_id") == Some(tenant_id)
                        && string_field(credential, "provider_id") == Some(provider_id)
                        && string_field(credential, "key_reference") == Some(key_reference)
                });
                sync_provider_credential_readiness(&mut guard.store);
                return empty_response(StatusCode::NO_CONTENT);
            }
            ["models", external_name, "providers", provider_id] => {
                cascade_delete_model(&mut guard.store, external_name, provider_id);
                return empty_response(StatusCode::NO_CONTENT);
            }
            ["channel_models", channel_id, "models", model_id] => {
                remove_where(
                    array_mut(&mut guard.store, "channelModels"),
                    |channel_model| {
                        string_field(channel_model, "channel_id") == Some(channel_id)
                            && string_field(channel_model, "model_id") == Some(model_id)
                    },
                );
                return empty_response(StatusCode::NO_CONTENT);
            }
            ["model_prices", channel_id, "models", model_id, "providers", provider_id] => {
                remove_where(array_mut(&mut guard.store, "modelPrices"), |model_price| {
                    string_field(model_price, "channel_id") == Some(channel_id)
                        && string_field(model_price, "model_id") == Some(model_id)
                        && string_field(model_price, "proxy_provider_id") == Some(provider_id)
                });
                return empty_response(StatusCode::NO_CONTENT);
            }
            _ => {}
        }
    }

    json_error_response(
        StatusCode::NOT_FOUND,
        &format!(
            "Admin sandbox route not implemented for {} {}.",
            method,
            uri.path()
        ),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::to_bytes;
    use serde_json::{json, Value};
    use std::fs;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn unique_storage_snapshot_file(prefix: &str) -> PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time should be after epoch")
            .as_nanos();
        std::env::temp_dir().join(format!("sdkwork_admin_sandbox_{prefix}_{unique}.json"))
    }

    fn authorization_headers(token: Option<&str>) -> HeaderMap {
        let mut headers = HeaderMap::new();
        if let Some(token) = token {
            headers.insert(
                header::AUTHORIZATION,
                format!("Bearer {token}")
                    .parse()
                    .expect("authorization header should parse"),
            );
        }
        headers
    }

    async fn send_request(
        state: &SharedAdminSandboxState,
        method: Method,
        path: &str,
        token: Option<&str>,
        body: Option<Value>,
    ) -> (StatusCode, Option<Value>) {
        let response = handle_admin_sandbox_request(
            state,
            method,
            authorization_headers(token),
            Uri::from_maybe_shared(Bytes::copy_from_slice(path.as_bytes()))
                .expect("sandbox request uri should parse"),
            body.map_or_else(Bytes::new, |payload| Bytes::from(payload.to_string())),
        )
        .await;

        let status = response.status();
        let bytes = to_bytes(response.into_body(), usize::MAX)
            .await
            .expect("sandbox response body should be readable");
        let payload = if bytes.is_empty() {
            None
        } else {
            Some(
                serde_json::from_slice::<Value>(&bytes)
                    .expect("sandbox response should be valid json"),
            )
        };

        (status, payload)
    }

    const TEST_APPBASE_BEARER: &str = "appbase-issued-admin-token";

    #[tokio::test]
    async fn sandbox_requires_appbase_bearer_token() {
        let state = SharedAdminSandboxState::seeded();

        let (missing_status, missing_payload) = send_request(
            &state,
            Method::GET,
            "/backend/v3/api/admin/storage/providers",
            None,
            None,
        )
        .await;
        assert_eq!(
            missing_status,
            StatusCode::UNAUTHORIZED,
            "sandbox must not mint or infer local appbase account sessions"
        );
        let missing_payload =
            missing_payload.expect("missing bearer response should return a payload");
        assert_eq!(
            missing_payload
                .get("error")
                .and_then(|error| string_field(error, "message")),
            Some("Admin sandbox requires an sdkwork-appbase bearer token.")
        );

        let (status, payload) = send_request(
            &state,
            Method::GET,
            "/backend/v3/api/admin/storage/providers",
            Some(TEST_APPBASE_BEARER),
            None,
        )
        .await;
        assert_eq!(status, StatusCode::OK);
        assert!(payload
            .as_ref()
            .and_then(Value::as_array)
            .is_some_and(|providers| !providers.is_empty()));
    }

    #[tokio::test]
    async fn sandbox_manages_storage_providers_configs_validation_and_fallback() {
        let state = SharedAdminSandboxState::seeded();
        let token = TEST_APPBASE_BEARER;

        let (status, providers_payload) = send_request(
            &state,
            Method::GET,
            "/backend/v3/api/admin/storage/providers",
            Some(token),
            None,
        )
        .await;
        assert_eq!(status, StatusCode::OK);
        let providers = providers_payload
            .as_ref()
            .and_then(Value::as_array)
            .cloned()
            .expect("storage providers should be an array");
        assert!(providers.iter().any(|provider| {
            string_field(provider, "providerPluginId") == Some("object-storage-aws")
        }));
        let aws_provider = providers
            .iter()
            .find(|provider| {
                string_field(provider, "providerPluginId") == Some("object-storage-aws")
            })
            .expect("aws provider schema should be present");
        let aws_common_fields = aws_provider
            .get("commonFields")
            .and_then(Value::as_array)
            .cloned()
            .expect("aws provider schema should expose common fields");
        assert!(aws_common_fields
            .iter()
            .any(|field| { string_field(field, "name") == Some("bucketOrContainer") }));

        let (status, global_payload) = send_request(
            &state,
            Method::POST,
            "/backend/v3/api/admin/storage/config",
            Some(token),
            Some(json!({
                "binding": {
                    "providerPluginId": "object-storage-aws",
                    "enabled": true,
                },
                "config": {
                    "bucketOrContainer": "global-assets",
                    "region": "us-east-1",
                    "endpoint": "https://s3.amazonaws.com",
                    "publicBaseUrl": "https://cdn.global.example",
                },
                "secret": {
                    "credentialMode": "access-key-pair",
                    "encryptedSecretPayload": "{\"accessKeyId\":\"global-access-key\",\"secretAccessKey\":\"global-secret-key\"}",
                    "secretFingerprint": "fp-global-aws",
                },
            })),
        )
        .await;
        assert_eq!(status, StatusCode::OK);
        let saved_global = global_payload.expect("global storage save should return a payload");
        assert_eq!(
            saved_global
                .get("binding")
                .and_then(|binding| string_field(binding, "providerPluginId")),
            Some("object-storage-aws")
        );
        assert!(!saved_global.to_string().contains("global-secret-key"));

        let (status, global_read_payload) = send_request(
            &state,
            Method::GET,
            "/backend/v3/api/admin/storage/config",
            Some(token),
            None,
        )
        .await;
        assert_eq!(status, StatusCode::OK);
        let global_read = global_read_payload.expect("global storage read should return a payload");
        assert_eq!(
            global_read
                .get("secret")
                .and_then(|secret| string_field(secret, "secretFingerprint")),
            Some("fp-global-aws")
        );

        let (status, global_validation_payload) = send_request(
            &state,
            Method::POST,
            "/backend/v3/api/admin/storage/validate",
            Some(token),
            Some(json!({})),
        )
        .await;
        assert_eq!(status, StatusCode::OK);
        let global_validation =
            global_validation_payload.expect("global storage validation should return a payload");
        assert_eq!(string_field(&global_validation, "status"), Some("healthy"));
        assert_eq!(string_field(&global_validation, "stage"), Some("presign"));

        let (status, tenant_payload) = send_request(
            &state,
            Method::POST,
            "/backend/v3/api/admin/storage/config/tenants/tenant_northstar",
            Some(token),
            Some(json!({
                "binding": {
                    "providerPluginId": "object-storage-google",
                    "enabled": true,
                },
                "config": {
                    "bucketOrContainer": "tenant-northstar-assets",
                    "region": "asia-east1",
                    "publicBaseUrl": "https://cdn.tenant.example",
                },
                "secret": {
                    "credentialMode": "service-account-json",
                    "encryptedSecretPayload": "{\"serviceAccountJson\":{\"client_email\":\"tenant@sdkwork.local\"}}",
                    "secretFingerprint": "fp-tenant-google",
                },
            })),
        )
        .await;
        assert_eq!(status, StatusCode::OK);
        let saved_tenant = tenant_payload.expect("tenant storage save should return a payload");
        assert_eq!(
            saved_tenant
                .get("scope")
                .and_then(|scope| string_field(scope, "kind")),
            Some("tenant")
        );
        assert_eq!(
            saved_tenant
                .get("scope")
                .and_then(|scope| string_field(scope, "scopeId")),
            Some("tenant_northstar")
        );

        let (status, tenant_read_payload) = send_request(
            &state,
            Method::GET,
            "/backend/v3/api/admin/storage/config/tenants/tenant_northstar",
            Some(token),
            None,
        )
        .await;
        assert_eq!(status, StatusCode::OK);
        let tenant_read = tenant_read_payload.expect("tenant storage read should return a payload");
        assert_eq!(
            tenant_read
                .get("binding")
                .and_then(|binding| string_field(binding, "providerPluginId")),
            Some("object-storage-google")
        );

        let (status, tenant_validation_payload) = send_request(
            &state,
            Method::POST,
            "/backend/v3/api/admin/storage/validate/tenants/tenant_northstar",
            Some(token),
            Some(json!({})),
        )
        .await;
        assert_eq!(status, StatusCode::OK);
        let tenant_validation =
            tenant_validation_payload.expect("tenant storage validation should return a payload");
        assert_eq!(string_field(&tenant_validation, "status"), Some("healthy"));
        assert_eq!(
            string_field(&tenant_validation, "providerPluginId"),
            Some("object-storage-google")
        );

        let (status, effective_payload) = send_request(
            &state,
            Method::GET,
            "/backend/v3/api/admin/storage/effective/tenants/tenant_northstar",
            Some(token),
            None,
        )
        .await;
        assert_eq!(status, StatusCode::OK);
        let effective = effective_payload.expect("effective storage should return a payload");
        assert_eq!(
            effective
                .get("resolvedScope")
                .and_then(|scope| string_field(scope, "kind")),
            Some("tenant")
        );
        assert_eq!(
            effective
                .get("secret")
                .and_then(|secret| string_field(secret, "secretFingerprint")),
            Some("fp-tenant-google")
        );

        let (status, audit_payload) = send_request(
            &state,
            Method::GET,
            "/backend/v3/api/admin/storage/audit",
            Some(token),
            None,
        )
        .await;
        assert_eq!(status, StatusCode::OK);
        let audit = audit_payload
            .as_ref()
            .and_then(Value::as_array)
            .cloned()
            .expect("storage audit should be an array");
        assert!(audit.len() >= 2);

        let (status, payload) = send_request(
            &state,
            Method::DELETE,
            "/backend/v3/api/admin/storage/config/tenants/tenant_northstar",
            Some(token),
            None,
        )
        .await;
        assert_eq!(status, StatusCode::NO_CONTENT);
        assert!(payload.is_none());

        let (status, fallback_payload) = send_request(
            &state,
            Method::GET,
            "/backend/v3/api/admin/storage/effective/tenants/tenant_northstar",
            Some(token),
            None,
        )
        .await;
        assert_eq!(status, StatusCode::OK);
        let fallback = fallback_payload.expect("fallback storage should return a payload");
        assert_eq!(
            fallback
                .get("resolvedScope")
                .and_then(|scope| string_field(scope, "kind")),
            Some("global")
        );
        assert_eq!(
            fallback
                .get("binding")
                .and_then(|binding| string_field(binding, "providerPluginId")),
            Some("object-storage-aws")
        );
    }

    #[tokio::test]
    async fn sandbox_storage_state_persists_across_seeded_instances_when_file_store_is_configured()
    {
        let file_path = unique_storage_snapshot_file("storage_state");

        let state = SharedAdminSandboxState::seeded_with_storage_file(&file_path);
        let token = TEST_APPBASE_BEARER;

        let (status, _) = send_request(
            &state,
            Method::POST,
            "/backend/v3/api/admin/storage/config",
            Some(token),
            Some(json!({
                "binding": {
                    "providerPluginId": "object-storage-aws",
                    "enabled": true,
                },
                "config": {
                    "bucketOrContainer": "global-assets",
                    "region": "us-east-1",
                    "endpoint": "https://s3.amazonaws.com",
                    "publicBaseUrl": "https://cdn.global.example",
                },
                "secret": {
                    "credentialMode": "access-key-pair",
                    "encryptedSecretPayload": "{\"accessKeyId\":\"global-access-key\",\"secretAccessKey\":\"global-secret-key\"}",
                    "secretFingerprint": "fp-global-aws",
                },
            })),
        )
        .await;
        assert_eq!(status, StatusCode::OK);

        let (status, _) = send_request(
            &state,
            Method::POST,
            "/backend/v3/api/admin/storage/config/tenants/tenant_northstar",
            Some(token),
            Some(json!({
                "binding": {
                    "providerPluginId": "object-storage-google",
                    "enabled": true,
                },
                "config": {
                    "bucketOrContainer": "tenant-northstar-assets",
                    "region": "asia-east1",
                    "publicBaseUrl": "https://cdn.tenant.example",
                },
                "secret": {
                    "credentialMode": "service-account-json",
                    "encryptedSecretPayload": "{\"serviceAccountJson\":{\"client_email\":\"tenant@sdkwork.local\"}}",
                    "secretFingerprint": "fp-tenant-google",
                },
            })),
        )
        .await;
        assert_eq!(status, StatusCode::OK);

        let reopened = SharedAdminSandboxState::seeded_with_storage_file(&file_path);
        let reopened_token = TEST_APPBASE_BEARER;

        let (status, global_payload) = send_request(
            &reopened,
            Method::GET,
            "/backend/v3/api/admin/storage/config",
            Some(reopened_token),
            None,
        )
        .await;
        assert_eq!(status, StatusCode::OK);
        let global_payload = global_payload.expect("global storage should persist across reopen");
        assert_eq!(
            global_payload
                .get("secret")
                .and_then(|secret| string_field(secret, "secretFingerprint")),
            Some("fp-global-aws")
        );

        let (status, tenant_payload) = send_request(
            &reopened,
            Method::GET,
            "/backend/v3/api/admin/storage/config/tenants/tenant_northstar",
            Some(reopened_token),
            None,
        )
        .await;
        assert_eq!(status, StatusCode::OK);
        let tenant_payload = tenant_payload.expect("tenant storage should persist across reopen");
        assert_eq!(
            tenant_payload
                .get("binding")
                .and_then(|binding| string_field(binding, "providerPluginId")),
            Some("object-storage-google")
        );

        let (status, _) = send_request(
            &reopened,
            Method::DELETE,
            "/backend/v3/api/admin/storage/config/tenants/tenant_northstar",
            Some(reopened_token),
            None,
        )
        .await;
        assert_eq!(status, StatusCode::NO_CONTENT);

        let reopened_again = SharedAdminSandboxState::seeded_with_storage_file(&file_path);
        let reopened_again_token = TEST_APPBASE_BEARER;
        let (status, effective_payload) = send_request(
            &reopened_again,
            Method::GET,
            "/backend/v3/api/admin/storage/effective/tenants/tenant_northstar",
            Some(reopened_again_token),
            None,
        )
        .await;
        assert_eq!(status, StatusCode::OK);
        let effective_payload =
            effective_payload.expect("effective storage should load from persisted snapshot");
        assert_eq!(
            effective_payload
                .get("resolvedScope")
                .and_then(|scope| string_field(scope, "kind")),
            Some("global")
        );
        assert_eq!(
            effective_payload
                .get("binding")
                .and_then(|binding| string_field(binding, "providerPluginId")),
            Some("object-storage-aws")
        );

        let _ = fs::remove_file(&file_path);
        let _ = fs::remove_file(file_path.with_extension("json.tmp"));
    }

    #[tokio::test]
    async fn sandbox_supports_workbench_write_contract() {
        let state = SharedAdminSandboxState::seeded();
        let token = TEST_APPBASE_BEARER;

        let (status, campaign_payload) = send_request(
            &state,
            Method::POST,
            "/backend/v3/api/admin/marketing/campaigns",
            Some(token),
            Some(json!({
                "marketing_campaign_id": "campaign_contract",
                "display_name": "Contract Campaign",
                "status": "draft",
                "start_at_ms": 1762752000000u64,
                "end_at_ms": 1762838400000u64,
            })),
        )
        .await;
        assert_eq!(status, StatusCode::OK);
        let campaign_id = campaign_payload
            .as_ref()
            .and_then(|value| value.get("marketing_campaign_id"))
            .and_then(Value::as_str)
            .expect("campaign save should return an id")
            .to_owned();

        let (status, _) = send_request(
            &state,
            Method::POST,
            &format!("/backend/v3/api/admin/marketing/campaigns/{campaign_id}/status"),
            Some(token),
            Some(json!({ "status": "active" })),
        )
        .await;
        assert_eq!(status, StatusCode::OK);

        let (status, api_key_group_payload) = send_request(
            &state,
            Method::POST,
            "/backend/v3/api/admin/api_key_groups",
            Some(token),
            Some(json!({
                "tenant_id": "tenant_contract",
                "project_id": "project_contract",
                "environment": "staging",
                "name": "Contract Group",
            })),
        )
        .await;
        assert_eq!(status, StatusCode::OK);
        let api_key_group_id = api_key_group_payload
            .as_ref()
            .and_then(|value| value.get("group_id"))
            .and_then(Value::as_str)
            .expect("api key group should return an id")
            .to_owned();

        let (status, _) = send_request(
            &state,
            Method::PATCH,
            &format!("/backend/v3/api/admin/api_key_groups/{api_key_group_id}"),
            Some(token),
            Some(json!({
                "description": "patched contract group",
            })),
        )
        .await;
        assert_eq!(status, StatusCode::OK);

        let (status, _) = send_request(
            &state,
            Method::POST,
            &format!("/backend/v3/api/admin/api_key_groups/{api_key_group_id}/status"),
            Some(token),
            Some(json!({ "active": false })),
        )
        .await;
        assert_eq!(status, StatusCode::OK);

        let (status, _) = send_request(
            &state,
            Method::POST,
            "/backend/v3/api/admin/routing/profiles",
            Some(token),
            Some(json!({
                "tenant_id": "tenant_contract",
                "project_id": "project_contract",
                "name": "Contract Routing",
            })),
        )
        .await;
        assert_eq!(status, StatusCode::OK);

        let (status, api_key_payload) = send_request(
            &state,
            Method::POST,
            "/backend/v3/api/admin/api_keys",
            Some(token),
            Some(json!({
                "tenant_id": "tenant_contract",
                "project_id": "project_contract",
                "environment": "staging",
                "label": "Contract key",
                "api_key_group_id": api_key_group_id.clone(),
            })),
        )
        .await;
        assert_eq!(status, StatusCode::OK);
        let hashed_key = api_key_payload
            .as_ref()
            .and_then(|value| value.get("hashed"))
            .and_then(Value::as_str)
            .expect("api key create should return the hashed key")
            .to_owned();

        let (status, _) = send_request(
            &state,
            Method::PUT,
            &format!("/backend/v3/api/admin/api_keys/{hashed_key}"),
            Some(token),
            Some(json!({
                "tenant_id": "tenant_contract",
                "project_id": "project_contract",
                "environment": "staging",
                "label": "Contract key updated",
                "notes": "updated through rust sandbox",
                "api_key_group_id": null,
                "expires_at_ms": 1762924800000u64,
            })),
        )
        .await;
        assert_eq!(status, StatusCode::OK);

        let (status, _) = send_request(
            &state,
            Method::POST,
            &format!("/backend/v3/api/admin/api_keys/{hashed_key}/status"),
            Some(token),
            Some(json!({ "active": false })),
        )
        .await;
        assert_eq!(status, StatusCode::OK);

        let (status, _) = send_request(
            &state,
            Method::POST,
            "/backend/v3/api/admin/channels",
            Some(token),
            Some(json!({
                "id": "channel_contract",
                "name": "Contract Channel",
            })),
        )
        .await;
        assert_eq!(status, StatusCode::OK);

        let (status, _) = send_request(
            &state,
            Method::POST,
            "/backend/v3/api/admin/providers",
            Some(token),
            Some(json!({
                "id": "provider_contract",
                "channel_id": "channel_contract",
                "extension_id": "runtime_contract",
                "adapter_kind": "openai-compatible",
                "protocol_kind": "openai",
                "base_url": "https://sandbox.provider.contract",
                "display_name": "Contract Provider",
                "channel_bindings": [
                    {
                        "channel_id": "channel_contract",
                        "is_primary": true,
                    }
                ],
            })),
        )
        .await;
        assert_eq!(status, StatusCode::OK);

        let (status, providers_payload) = send_request(
            &state,
            Method::GET,
            "/backend/v3/api/admin/providers",
            Some(token),
            None,
        )
        .await;
        assert_eq!(status, StatusCode::OK);
        let providers = providers_payload
            .as_ref()
            .and_then(|value| value.as_array())
            .cloned()
            .expect("providers list should be an array");
        let provider = providers
            .iter()
            .find(|record| string_field(record, "id") == Some("provider_contract"))
            .expect("provider should be present after save");
        assert_eq!(
            provider
                .get("credential_readiness")
                .and_then(|value| value.get("ready"))
                .and_then(Value::as_bool),
            Some(false)
        );

        let (status, _) = send_request(
            &state,
            Method::POST,
            "/backend/v3/api/admin/credentials",
            Some(token),
            Some(json!({
                "tenant_id": "tenant_contract",
                "provider_id": "provider_contract",
                "key_reference": "contract-key-ref",
                "secret_value": "super-secret",
            })),
        )
        .await;
        assert_eq!(status, StatusCode::OK);

        let (status, providers_payload) = send_request(
            &state,
            Method::GET,
            "/backend/v3/api/admin/providers",
            Some(token),
            None,
        )
        .await;
        assert_eq!(status, StatusCode::OK);
        let providers = providers_payload
            .as_ref()
            .and_then(|value| value.as_array())
            .cloned()
            .expect("providers list should be an array");
        let provider = providers
            .iter()
            .find(|record| string_field(record, "id") == Some("provider_contract"))
            .expect("provider should stay present after credential save");
        assert_eq!(
            provider
                .get("credential_readiness")
                .and_then(|value| value.get("ready"))
                .and_then(Value::as_bool),
            Some(true)
        );

        let (status, _) = send_request(
            &state,
            Method::POST,
            "/backend/v3/api/admin/models",
            Some(token),
            Some(json!({
                "external_name": "model-contract",
                "provider_id": "provider_contract",
                "capabilities": ["chat.reply"],
                "streaming": true,
                "context_window": 16384,
            })),
        )
        .await;
        assert_eq!(status, StatusCode::OK);

        let (status, _) = send_request(
            &state,
            Method::POST,
            "/backend/v3/api/admin/channel_models",
            Some(token),
            Some(json!({
                "channel_id": "channel_contract",
                "model_id": "model-contract",
                "model_display_name": "Contract Model",
                "capabilities": ["chat.reply"],
                "streaming": true,
                "context_window": 16384,
            })),
        )
        .await;
        assert_eq!(status, StatusCode::OK);

        let (status, _) = send_request(
            &state,
            Method::POST,
            "/backend/v3/api/admin/model_prices",
            Some(token),
            Some(json!({
                "channel_id": "channel_contract",
                "model_id": "model-contract",
                "proxy_provider_id": "provider_contract",
                "currency_code": "USD",
                "price_unit": "1k_tokens",
                "input_price": 0.2,
                "output_price": 0.4,
                "cache_read_price": 0.05,
                "cache_write_price": 0.1,
                "request_price": 0.01,
                "is_active": true,
            })),
        )
        .await;
        assert_eq!(status, StatusCode::OK);

        let (status, _) = send_request(
            &state,
            Method::POST,
            "/backend/v3/api/admin/gateway/rate_limit_policies",
            Some(token),
            Some(json!({
                "policy_id": "policy_contract",
                "project_id": "project_contract",
                "api_key_hash": hashed_key.clone(),
                "route_key": "contract-route",
                "model_name": "model-contract",
                "requests_per_window": 600,
                "window_seconds": 120,
                "burst_requests": 60,
                "enabled": true,
                "notes": "contract policy",
            })),
        )
        .await;
        assert_eq!(status, StatusCode::OK);

        let (status, windows_payload) = send_request(
            &state,
            Method::GET,
            "/backend/v3/api/admin/gateway/rate_limit_windows",
            Some(token),
            None,
        )
        .await;
        assert_eq!(status, StatusCode::OK);
        let windows = windows_payload
            .as_ref()
            .and_then(|value| value.as_array())
            .cloned()
            .expect("rate limit windows should be an array");
        assert!(windows.iter().any(|record| {
            string_field(record, "policy_id") == Some("policy_contract")
                && record.get("window_seconds").and_then(Value::as_u64) == Some(120)
        }));

        let (status, reload_payload) = send_request(
            &state,
            Method::POST,
            "/backend/v3/api/admin/extensions/runtime_reloads",
            Some(token),
            Some(json!({
                "extension_id": "runtime_contract",
            })),
        )
        .await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(
            reload_payload
                .as_ref()
                .and_then(|value| value.get("scope"))
                .and_then(Value::as_str),
            Some("targeted")
        );

        for (path, expected_status) in [
            (
                "/backend/v3/api/admin/channel_models/channel_contract/models/model-contract".to_owned(),
                StatusCode::NO_CONTENT,
            ),
            (
                "/backend/v3/api/admin/model_prices/channel_contract/models/model-contract/providers/provider_contract"
                    .to_owned(),
                StatusCode::NO_CONTENT,
            ),
            (
                "/backend/v3/api/admin/credentials/tenant_contract/providers/provider_contract/keys/contract-key-ref"
                    .to_owned(),
                StatusCode::NO_CONTENT,
            ),
            (
                "/backend/v3/api/admin/models/model-contract/providers/provider_contract".to_owned(),
                StatusCode::NO_CONTENT,
            ),
            (
                "/backend/v3/api/admin/providers/provider_contract".to_owned(),
                StatusCode::NO_CONTENT,
            ),
            (
                "/backend/v3/api/admin/channels/channel_contract".to_owned(),
                StatusCode::NO_CONTENT,
            ),
            (
                format!("/backend/v3/api/admin/api_keys/{hashed_key}"),
                StatusCode::NO_CONTENT,
            ),
            (
                format!("/backend/v3/api/admin/api_key_groups/{api_key_group_id}"),
                StatusCode::NO_CONTENT,
            ),
        ] {
            let (status, payload) = send_request(
                &state,
                Method::DELETE,
                &path,
                Some(token),
                None,
            )
            .await;
            assert_eq!(status, expected_status, "delete route should succeed for {path}");
            assert!(payload.is_none(), "delete routes should not return a payload");
        }
    }

    #[tokio::test]
    async fn sandbox_rejects_storage_credentials_missing_required_mode_fields() {
        let state = SharedAdminSandboxState::seeded();
        let token = TEST_APPBASE_BEARER;

        let (status, payload) = send_request(
            &state,
            Method::POST,
            "/backend/v3/api/admin/storage/config",
            Some(token),
            Some(json!({
                "binding": {
                    "providerPluginId": "object-storage-google",
                    "enabled": true,
                },
                "config": {
                    "bucketOrContainer": "tenant-assets",
                },
                "secret": {
                    "credentialMode": "interoperability-key",
                    "encryptedSecretPayload": "{\"interoperabilityAccessKey\":\"interop-access-key\"}",
                }
            })),
        )
        .await;

        assert_eq!(status, StatusCode::BAD_REQUEST);
        let payload =
            payload.expect("invalid storage credential save should return an error payload");
        assert_eq!(
            payload
                .get("error")
                .and_then(|error| string_field(error, "message")),
            Some("Interoperability Secret Key is required.")
        );
    }
}
