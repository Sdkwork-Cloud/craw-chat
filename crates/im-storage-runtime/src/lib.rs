use im_storage_contracts::{
    StorageAuditRecord, StorageBindingRecord, StorageCatalog, StorageConfigRecord,
    StorageConfigSnapshot, StorageConfigUpsertInput, StorageCredentialMode, StorageDomainSnapshot,
    StorageDomainSnapshotStore, StorageEffectiveConfig, StorageProviderSchema, StorageSchemaField,
    StorageScopeKind, StorageScopeRef, StorageSecretRecord, StorageValidationResult,
    StorageValidationStage, StorageValidationStatus,
};
use sdkwork_im_contract_core::ContractError;
use serde_json::{Map, Value};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StorageConfigUpsert {
    provider_plugin_id: String,
    enabled: bool,
    bucket_or_container: Option<String>,
    region: Option<String>,
    endpoint: Option<String>,
    public_base_url: Option<String>,
    upload_prefix: Option<String>,
    download_prefix: Option<String>,
    provider_config: Value,
    secret: Option<StorageSecretUpsert>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct StorageSecretUpsert {
    credential_mode: StorageCredentialMode,
    encrypted_secret_payload: String,
    secret_fingerprint: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct StoredScopeRecord {
    scope: StorageScopeRef,
    binding: StorageBindingRecord,
    config: StorageConfigRecord,
    secret: Option<StorageSecretRecord>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct StorageInputValidationError {
    stage: StorageValidationStage,
    message: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StorageRuntimeState {
    catalog: StorageCatalog,
    global_config: Option<StoredScopeRecord>,
    tenant_configs: Vec<StoredScopeRecord>,
    audit_trail: Vec<StorageAuditRecord>,
    next_audit_sequence: u64,
}

#[derive(Clone, Debug)]
pub struct StoreBackedStorageRuntime<S>
where
    S: StorageDomainSnapshotStore + Clone,
{
    store: S,
    state: StorageRuntimeState,
}

impl StorageConfigUpsert {
    pub fn new(provider_plugin_id: impl Into<String>) -> Self {
        Self {
            provider_plugin_id: provider_plugin_id.into(),
            enabled: true,
            bucket_or_container: None,
            region: None,
            endpoint: None,
            public_base_url: None,
            upload_prefix: None,
            download_prefix: None,
            provider_config: Value::Object(Map::new()),
            secret: None,
        }
    }

    pub fn with_enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    pub fn with_bucket_or_container(mut self, bucket_or_container: impl Into<String>) -> Self {
        self.bucket_or_container = Some(bucket_or_container.into());
        self
    }

    pub fn with_region(mut self, region: impl Into<String>) -> Self {
        self.region = Some(region.into());
        self
    }

    pub fn with_endpoint(mut self, endpoint: impl Into<String>) -> Self {
        self.endpoint = Some(endpoint.into());
        self
    }

    pub fn with_public_base_url(mut self, public_base_url: impl Into<String>) -> Self {
        self.public_base_url = Some(public_base_url.into());
        self
    }

    pub fn with_upload_prefix(mut self, upload_prefix: impl Into<String>) -> Self {
        self.upload_prefix = Some(upload_prefix.into());
        self
    }

    pub fn with_download_prefix(mut self, download_prefix: impl Into<String>) -> Self {
        self.download_prefix = Some(download_prefix.into());
        self
    }

    pub fn with_provider_config(mut self, provider_config: Value) -> Self {
        self.provider_config = provider_config;
        self
    }

    pub fn with_secret(
        mut self,
        credential_mode: StorageCredentialMode,
        encrypted_secret_payload: impl Into<String>,
        secret_fingerprint: impl Into<String>,
    ) -> Self {
        self.secret = Some(StorageSecretUpsert {
            credential_mode,
            encrypted_secret_payload: encrypted_secret_payload.into(),
            secret_fingerprint: secret_fingerprint.into(),
        });
        self
    }
}

impl StorageRuntimeState {
    pub fn new(catalog: StorageCatalog) -> Self {
        Self {
            catalog,
            global_config: None,
            tenant_configs: Vec::new(),
            audit_trail: Vec::new(),
            next_audit_sequence: 0,
        }
    }

    pub fn new_object_storage() -> Self {
        Self::new(StorageCatalog::object_storage())
    }

    pub fn catalog(&self) -> &StorageCatalog {
        &self.catalog
    }

    pub fn load_from_store(
        store: &impl StorageDomainSnapshotStore,
        catalog: StorageCatalog,
    ) -> Result<Self, ContractError> {
        let requested_domain = catalog.domain.clone();
        let Some(snapshot) = store.load_snapshot(requested_domain.as_str())? else {
            return Ok(Self::new(catalog));
        };

        if snapshot.catalog.domain != requested_domain {
            return Err(ContractError::Conflict(format!(
                "Loaded storage snapshot domain {} does not match requested domain {}.",
                snapshot.catalog.domain, requested_domain
            )));
        }

        Ok(Self::from_domain_snapshot(snapshot))
    }

    pub fn save_to_store(
        &self,
        store: &impl StorageDomainSnapshotStore,
    ) -> Result<(), ContractError> {
        store.save_snapshot(self.domain_snapshot())
    }

    pub fn from_domain_snapshot(snapshot: StorageDomainSnapshot) -> Self {
        let global_config = snapshot
            .bindings
            .iter()
            .find(|binding| binding.scope.kind == StorageScopeKind::Global)
            .and_then(|binding| stored_scope_record_from_snapshot(&snapshot, binding));

        let tenant_configs = snapshot
            .bindings
            .iter()
            .filter(|binding| binding.scope.kind == StorageScopeKind::Tenant)
            .filter_map(|binding| stored_scope_record_from_snapshot(&snapshot, binding))
            .collect();

        Self {
            catalog: snapshot.catalog,
            global_config,
            tenant_configs,
            audit_trail: Vec::new(),
            next_audit_sequence: 0,
        }
    }

    pub fn domain_snapshot(&self) -> StorageDomainSnapshot {
        let mut snapshot = StorageDomainSnapshot::new(self.catalog.clone());

        if let Some(record) = self.global_config.as_ref() {
            snapshot = snapshot
                .with_binding(record.binding.clone())
                .with_config(record.config.clone());
            if let Some(secret) = record.secret.as_ref() {
                snapshot = snapshot.with_secret(secret.clone());
            }
        }

        for record in &self.tenant_configs {
            snapshot = snapshot
                .with_binding(record.binding.clone())
                .with_config(record.config.clone());
            if let Some(secret) = record.secret.as_ref() {
                snapshot = snapshot.with_secret(secret.clone());
            }
        }

        snapshot
    }

    pub fn global_snapshot(&self) -> StorageConfigSnapshot {
        self.snapshot_for_scope(StorageScopeRef::global())
    }

    pub fn tenant_snapshot(&self, tenant_id: &str) -> StorageConfigSnapshot {
        self.snapshot_for_scope(StorageScopeRef::tenant(tenant_id))
    }

    pub fn save_global(&mut self, input: StorageConfigUpsert) -> StorageConfigSnapshot {
        let scope = StorageScopeRef::global();
        let existing_secret = self
            .global_config
            .as_ref()
            .and_then(|record| record.secret.clone());
        let stored = self.normalize_record(scope.clone(), input, existing_secret);
        self.global_config = Some(stored.clone());
        self.record_audit("upsert", &scope, stored.binding.provider_plugin_id.as_str());
        self.snapshot_for_scope(scope)
    }

    pub fn save_tenant(
        &mut self,
        tenant_id: impl Into<String>,
        input: StorageConfigUpsert,
    ) -> StorageConfigSnapshot {
        let scope = StorageScopeRef::tenant(tenant_id);
        let existing_secret = self
            .tenant_configs
            .iter()
            .find(|existing| existing.scope == scope)
            .and_then(|record| record.secret.clone());
        let stored = self.normalize_record(scope.clone(), input, existing_secret);
        if let Some(index) = self
            .tenant_configs
            .iter()
            .position(|existing| existing.scope == scope)
        {
            self.tenant_configs[index] = stored.clone();
        } else {
            self.tenant_configs.push(stored.clone());
        }
        self.record_audit("upsert", &scope, stored.binding.provider_plugin_id.as_str());
        self.snapshot_for_scope(scope)
    }

    pub fn delete_tenant(&mut self, tenant_id: &str) -> bool {
        let scope = StorageScopeRef::tenant(tenant_id);
        let Some(index) = self
            .tenant_configs
            .iter()
            .position(|existing| existing.scope == scope)
        else {
            return false;
        };
        let removed = self.tenant_configs.remove(index);
        self.record_audit(
            "delete",
            &scope,
            removed.binding.provider_plugin_id.as_str(),
        );
        true
    }

    pub fn effective_for_tenant(&self, tenant_id: &str) -> Option<StorageEffectiveConfig> {
        let requested_scope = StorageScopeRef::tenant(tenant_id);
        self.tenant_configs
            .iter()
            .find(|existing| existing.scope == requested_scope)
            .or(self.global_config.as_ref())
            .map(|record| StorageEffectiveConfig {
                requested_scope,
                resolved_scope: record.scope.clone(),
                binding: record.binding.clone(),
                config: record.config.clone(),
                secret: record
                    .secret
                    .as_ref()
                    .map(StorageSecretRecord::redacted_summary),
            })
    }

    pub fn validate_global(&self) -> StorageValidationResult {
        self.validate_scope_record(self.global_config.as_ref(), StorageScopeRef::global())
    }

    pub fn validate_tenant(&self, tenant_id: &str) -> StorageValidationResult {
        let requested_scope = StorageScopeRef::tenant(tenant_id);
        let record = self
            .tenant_configs
            .iter()
            .find(|existing| existing.scope == requested_scope)
            .or(self.global_config.as_ref());
        self.validate_scope_record(record, requested_scope)
    }

    pub fn audit_trail(&self) -> &[StorageAuditRecord] {
        self.audit_trail.as_slice()
    }

    fn snapshot_for_scope(&self, scope: StorageScopeRef) -> StorageConfigSnapshot {
        let record = match scope.kind {
            StorageScopeKind::Global => self.global_config.as_ref(),
            StorageScopeKind::Tenant => self
                .tenant_configs
                .iter()
                .find(|existing| existing.scope == scope),
        };

        StorageConfigSnapshot {
            scope,
            binding: record.map(|record| record.binding.clone()),
            config: record.map(|record| record.config.clone()),
            secret: record
                .and_then(|record| record.secret.as_ref())
                .map(StorageSecretRecord::redacted_summary),
        }
    }

    fn validate_scope_record(
        &self,
        record: Option<&StoredScopeRecord>,
        scope: StorageScopeRef,
    ) -> StorageValidationResult {
        let Some(record) = record else {
            return StorageValidationResult {
                scope,
                status: StorageValidationStatus::Invalid,
                stage: StorageValidationStage::Schema,
                message: "Storage binding and config must both be present.".into(),
                provider_plugin_id: None,
            };
        };

        if let Err(error) = validate_storage_scope_record(&self.catalog, record) {
            return StorageValidationResult {
                scope,
                status: StorageValidationStatus::Invalid,
                stage: error.stage,
                message: error.message,
                provider_plugin_id: Some(record.binding.provider_plugin_id.clone()),
            };
        }

        StorageValidationResult {
            scope,
            status: StorageValidationStatus::Healthy,
            stage: StorageValidationStage::Presign,
            message: "Runtime validation passed.".into(),
            provider_plugin_id: Some(record.binding.provider_plugin_id.clone()),
        }
    }

    fn normalize_record(
        &self,
        scope: StorageScopeRef,
        input: StorageConfigUpsert,
        existing_secret: Option<StorageSecretRecord>,
    ) -> StoredScopeRecord {
        let binding = match scope.kind {
            StorageScopeKind::Global => {
                StorageBindingRecord::new_global(input.provider_plugin_id.clone())
            }
            StorageScopeKind::Tenant => StorageBindingRecord::new_tenant(
                scope.scope_id.clone().unwrap_or_default(),
                input.provider_plugin_id.clone(),
            ),
        };

        let mut binding = binding;
        binding.enabled = input.enabled;

        let mut config = match scope.kind {
            StorageScopeKind::Global => {
                StorageConfigRecord::new_global(input.provider_plugin_id.clone())
            }
            StorageScopeKind::Tenant => StorageConfigRecord::new_tenant(
                scope.scope_id.clone().unwrap_or_default(),
                input.provider_plugin_id.clone(),
            ),
        };
        config.bucket_or_container = input.bucket_or_container;
        config.region = input.region;
        config.endpoint = input.endpoint;
        config.public_base_url = input.public_base_url;
        config.upload_prefix = input.upload_prefix;
        config.download_prefix = input.download_prefix;
        config.provider_config = input.provider_config;

        let secret = match input.secret {
            Some(secret) => {
                let base = match scope.kind {
                    StorageScopeKind::Global => StorageSecretRecord::new_global(
                        input.provider_plugin_id.clone(),
                        secret.credential_mode,
                        secret.encrypted_secret_payload,
                    ),
                    StorageScopeKind::Tenant => StorageSecretRecord::new_tenant(
                        scope.scope_id.clone().unwrap_or_default(),
                        input.provider_plugin_id.clone(),
                        secret.credential_mode,
                        secret.encrypted_secret_payload,
                    ),
                };
                Some(base.with_secret_fingerprint(secret.secret_fingerprint))
            }
            None => existing_secret
                .filter(|secret| secret.provider_plugin_id == input.provider_plugin_id),
        };

        StoredScopeRecord {
            scope,
            binding,
            config,
            secret,
        }
    }

    fn record_audit(&mut self, action: &str, scope: &StorageScopeRef, provider_plugin_id: &str) {
        self.next_audit_sequence += 1;
        self.audit_trail.insert(
            0,
            StorageAuditRecord {
                id: format!("storage_audit_{:04}", self.next_audit_sequence),
                action: action.into(),
                scope: scope.clone(),
                provider_plugin_id: provider_plugin_id.into(),
                created_at_ms: self.next_audit_sequence,
            },
        );
    }
}

impl<S> StoreBackedStorageRuntime<S>
where
    S: StorageDomainSnapshotStore + Clone,
{
    pub fn load(store: S, catalog: StorageCatalog) -> Result<Self, ContractError> {
        let state = StorageRuntimeState::load_from_store(&store, catalog)?;
        Ok(Self { store, state })
    }

    pub fn store(&self) -> &S {
        &self.store
    }

    pub fn state(&self) -> &StorageRuntimeState {
        &self.state
    }

    pub fn catalog(&self) -> &StorageCatalog {
        self.state.catalog()
    }

    pub fn global_snapshot(&self) -> StorageConfigSnapshot {
        self.state.global_snapshot()
    }

    pub fn tenant_snapshot(&self, tenant_id: &str) -> StorageConfigSnapshot {
        self.state.tenant_snapshot(tenant_id)
    }

    pub fn effective_for_tenant(&self, tenant_id: &str) -> Option<StorageEffectiveConfig> {
        self.state.effective_for_tenant(tenant_id)
    }

    pub fn validate_global(&self) -> StorageValidationResult {
        self.state.validate_global()
    }

    pub fn validate_tenant(&self, tenant_id: &str) -> StorageValidationResult {
        self.state.validate_tenant(tenant_id)
    }

    pub fn audit_trail(&self) -> &[StorageAuditRecord] {
        self.state.audit_trail()
    }

    pub fn save_global(
        &mut self,
        input: StorageConfigUpsert,
    ) -> Result<StorageConfigSnapshot, ContractError> {
        let snapshot = self.state.save_global(input);
        self.persist()?;
        Ok(snapshot)
    }

    pub fn save_tenant(
        &mut self,
        tenant_id: impl Into<String>,
        input: StorageConfigUpsert,
    ) -> Result<StorageConfigSnapshot, ContractError> {
        let snapshot = self.state.save_tenant(tenant_id, input);
        self.persist()?;
        Ok(snapshot)
    }

    pub fn delete_tenant(&mut self, tenant_id: &str) -> Result<bool, ContractError> {
        let deleted = self.state.delete_tenant(tenant_id);
        if deleted {
            self.persist()?;
        }
        Ok(deleted)
    }

    fn persist(&self) -> Result<(), ContractError> {
        self.state.save_to_store(&self.store)
    }
}

pub fn storage_config_upsert_from_input(
    catalog: &StorageCatalog,
    input: &StorageConfigUpsertInput,
) -> Result<StorageConfigUpsert, String> {
    let provider_plugin_id = input.binding.provider_plugin_id.trim();
    if provider_plugin_id.is_empty() {
        return Err("Storage config payload must include binding.providerPluginId.".into());
    }
    let provider_schema = catalog
        .provider_schema(provider_plugin_id)
        .ok_or_else(|| format!("Storage provider {provider_plugin_id} is not supported."))?;

    validate_required_common_fields(provider_schema, |field| {
        input_common_field_configured(input, field.name.as_str())
    })
    .map_err(|error| error.message)?;

    let mut upsert = StorageConfigUpsert::new(provider_plugin_id);

    if let Some(enabled) = input.binding.enabled {
        upsert = upsert.with_enabled(enabled);
    }
    if let Some(bucket_or_container) =
        trimmed_optional_string(input.config.bucket_or_container.as_deref())
    {
        upsert = upsert.with_bucket_or_container(bucket_or_container);
    }
    if let Some(region) = trimmed_optional_string(input.config.region.as_deref()) {
        upsert = upsert.with_region(region);
    }
    if let Some(endpoint) = trimmed_optional_string(input.config.endpoint.as_deref()) {
        upsert = upsert.with_endpoint(endpoint);
    }
    if let Some(public_base_url) = trimmed_optional_string(input.config.public_base_url.as_deref())
    {
        upsert = upsert.with_public_base_url(public_base_url);
    }
    if let Some(upload_prefix) = trimmed_optional_string(input.config.upload_prefix.as_deref()) {
        upsert = upsert.with_upload_prefix(upload_prefix);
    }
    if let Some(download_prefix) = trimmed_optional_string(input.config.download_prefix.as_deref())
    {
        upsert = upsert.with_download_prefix(download_prefix);
    }
    if let Some(provider_config) = input.config.provider_config.as_ref() {
        if !provider_config.is_object() {
            return Err("Storage providerConfig must be a JSON object.".into());
        }
        upsert = upsert.with_provider_config(provider_config.clone());
    }

    if let Some(secret) = input.secret.as_ref() {
        validate_secret_payload(
            provider_schema,
            secret.credential_mode,
            secret.encrypted_secret_payload.as_str(),
        )
        .map_err(|error| error.message.clone())?;
        let secret_fingerprint = trimmed_optional_string(secret.secret_fingerprint.as_deref())
            .unwrap_or_else(|| format!("fp-{provider_plugin_id}"));
        upsert = upsert.with_secret(
            secret.credential_mode,
            secret.encrypted_secret_payload.clone(),
            secret_fingerprint,
        );
    }

    Ok(upsert)
}

pub fn parse_storage_config_upsert_input(
    catalog: &StorageCatalog,
    input: &Value,
) -> Result<StorageConfigUpsert, String> {
    let typed_input: StorageConfigUpsertInput = serde_json::from_value(input.clone())
        .map_err(|_| "Storage config payload must be valid JSON.".to_owned())?;
    storage_config_upsert_from_input(catalog, &typed_input)
}

fn validate_storage_scope_record(
    catalog: &StorageCatalog,
    record: &StoredScopeRecord,
) -> Result<(), StorageInputValidationError> {
    let provider_schema = catalog
        .provider_schema(record.binding.provider_plugin_id.as_str())
        .ok_or_else(|| {
            StorageInputValidationError::schema(format!(
                "Storage provider {} is not supported.",
                record.binding.provider_plugin_id
            ))
        })?;

    validate_required_common_fields(provider_schema, |field| {
        record_common_field_configured(&record.config, field.name.as_str())
    })?;

    let Some(secret) = record.secret.as_ref() else {
        return Err(StorageInputValidationError::credentials(
            "Credential payload is required.",
        ));
    };

    validate_secret_payload(
        provider_schema,
        secret.credential_mode,
        secret.encrypted_secret_payload.as_str(),
    )
}

fn stored_scope_record_from_snapshot(
    snapshot: &StorageDomainSnapshot,
    binding: &StorageBindingRecord,
) -> Option<StoredScopeRecord> {
    let config = snapshot
        .configs
        .iter()
        .find(|config| config.scope == binding.scope)?;
    let secret = snapshot
        .secrets
        .iter()
        .find(|secret| secret.scope == binding.scope)
        .cloned();

    Some(StoredScopeRecord {
        scope: binding.scope.clone(),
        binding: binding.clone(),
        config: config.clone(),
        secret,
    })
}

impl StorageInputValidationError {
    fn schema(message: impl Into<String>) -> Self {
        Self {
            stage: StorageValidationStage::Schema,
            message: message.into(),
        }
    }

    fn credentials(message: impl Into<String>) -> Self {
        Self {
            stage: StorageValidationStage::Credentials,
            message: message.into(),
        }
    }
}

fn value_configured(value: Option<&Value>) -> bool {
    match value {
        None | Some(Value::Null) => false,
        Some(Value::String(value)) => !value.trim().is_empty(),
        Some(Value::Array(values)) => !values.is_empty(),
        Some(Value::Object(values)) => !values.is_empty(),
        Some(_) => true,
    }
}

fn string_configured(value: Option<&str>) -> bool {
    value.is_some_and(|value| !value.trim().is_empty())
}

fn field_applies_to_credential_mode(
    field: &StorageSchemaField,
    credential_mode: StorageCredentialMode,
) -> bool {
    field
        .credential_modes
        .as_ref()
        .is_none_or(|modes| modes.contains(&credential_mode))
}

fn input_common_field_configured(input: &StorageConfigUpsertInput, field_name: &str) -> bool {
    match field_name {
        "bucketOrContainer" => string_configured(input.config.bucket_or_container.as_deref()),
        "region" => string_configured(input.config.region.as_deref()),
        "endpoint" => string_configured(input.config.endpoint.as_deref()),
        "publicBaseUrl" => string_configured(input.config.public_base_url.as_deref()),
        "uploadPrefix" => string_configured(input.config.upload_prefix.as_deref()),
        "downloadPrefix" => string_configured(input.config.download_prefix.as_deref()),
        _ => value_configured(
            input
                .config
                .provider_config
                .as_ref()
                .and_then(|provider_config| provider_config.get(field_name)),
        ),
    }
}

fn record_common_field_configured(config: &StorageConfigRecord, field_name: &str) -> bool {
    match field_name {
        "bucketOrContainer" => string_configured(config.bucket_or_container.as_deref()),
        "region" => string_configured(config.region.as_deref()),
        "endpoint" => string_configured(config.endpoint.as_deref()),
        "publicBaseUrl" => string_configured(config.public_base_url.as_deref()),
        "uploadPrefix" => string_configured(config.upload_prefix.as_deref()),
        "downloadPrefix" => string_configured(config.download_prefix.as_deref()),
        _ => value_configured(config.provider_config.get(field_name)),
    }
}

fn validate_required_common_fields<F>(
    provider_schema: &StorageProviderSchema,
    mut field_configured: F,
) -> Result<(), StorageInputValidationError>
where
    F: FnMut(&StorageSchemaField) -> bool,
{
    for field in provider_schema
        .common_fields
        .iter()
        .filter(|field| field.required)
    {
        if !field_configured(field) {
            return Err(StorageInputValidationError::schema(format!(
                "{} is required.",
                field.label
            )));
        }
    }

    Ok(())
}

fn validate_secret_payload(
    provider_schema: &StorageProviderSchema,
    credential_mode: StorageCredentialMode,
    encrypted_secret_payload: &str,
) -> Result<(), StorageInputValidationError> {
    if !provider_schema
        .supported_credential_modes
        .contains(&credential_mode)
    {
        return Err(StorageInputValidationError::credentials(format!(
            "Storage provider {} does not support credential mode {}.",
            provider_schema.provider_plugin_id,
            credential_mode.as_str()
        )));
    }

    let payload: Value = serde_json::from_str(encrypted_secret_payload).map_err(|_| {
        StorageInputValidationError::credentials(
            "Storage secret payload must be a valid JSON object.",
        )
    })?;

    if !payload.is_object() {
        return Err(StorageInputValidationError::credentials(
            "Storage secret payload must be a valid JSON object.",
        ));
    }

    for field in provider_schema
        .credential_fields
        .iter()
        .filter(|field| field.required && field_applies_to_credential_mode(field, credential_mode))
    {
        if !value_configured(payload.get(field.name.as_str())) {
            return Err(StorageInputValidationError::credentials(format!(
                "{} is required.",
                field.label
            )));
        }
    }

    Ok(())
}

fn trimmed_optional_string(value: Option<&str>) -> Option<String> {
    value
        .map(str::trim)
        .filter(|candidate| !candidate.is_empty())
        .map(str::to_owned)
}
