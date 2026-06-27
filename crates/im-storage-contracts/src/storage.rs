use sdkwork_im_contract_core::ContractError;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum StorageScopeKind {
    Global,
    Tenant,
}

impl StorageScopeKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Global => "global",
            Self::Tenant => "tenant",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StorageScopeRef {
    pub kind: StorageScopeKind,
    pub scope_id: Option<String>,
}

impl StorageScopeRef {
    pub fn global() -> Self {
        Self {
            kind: StorageScopeKind::Global,
            scope_id: None,
        }
    }

    pub fn tenant(tenant_id: impl Into<String>) -> Self {
        Self {
            kind: StorageScopeKind::Tenant,
            scope_id: Some(tenant_id.into()),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum StorageProviderFamily {
    S3Compatible,
    GoogleCloudStorage,
    AzureBlob,
}

impl StorageProviderFamily {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::S3Compatible => "s3-compatible",
            Self::GoogleCloudStorage => "google-cloud-storage",
            Self::AzureBlob => "azure-blob",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum StorageFieldInputKind {
    Text,
    Url,
    Number,
    Boolean,
    Secret,
    Json,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StorageSchemaField {
    pub name: String,
    pub label: String,
    pub input_kind: StorageFieldInputKind,
    pub required: bool,
    pub help_text: Option<String>,
    pub credential_modes: Option<Vec<StorageCredentialMode>>,
}

impl StorageSchemaField {
    pub fn new(
        name: impl Into<String>,
        label: impl Into<String>,
        input_kind: StorageFieldInputKind,
        required: bool,
    ) -> Self {
        Self {
            name: name.into(),
            label: label.into(),
            input_kind,
            required,
            help_text: None,
            credential_modes: None,
        }
    }

    pub fn with_help_text(mut self, help_text: impl Into<String>) -> Self {
        self.help_text = Some(help_text.into());
        self
    }

    pub fn with_credential_modes<I>(mut self, credential_modes: I) -> Self
    where
        I: IntoIterator<Item = StorageCredentialMode>,
    {
        self.credential_modes = Some(credential_modes.into_iter().collect());
        self
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum StorageCredentialMode {
    AccessKeyPair,
    SessionAccessKeyPair,
    RoleAssumption,
    InteroperabilityKey,
    ServiceAccountJson,
    AccountKey,
    SasToken,
    ServicePrincipal,
}

impl StorageCredentialMode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AccessKeyPair => "access-key-pair",
            Self::SessionAccessKeyPair => "session-access-key-pair",
            Self::RoleAssumption => "role-assumption",
            Self::InteroperabilityKey => "interoperability-key",
            Self::ServiceAccountJson => "service-account-json",
            Self::AccountKey => "account-key",
            Self::SasToken => "sas-token",
            Self::ServicePrincipal => "service-principal",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StorageProviderSchema {
    pub provider_plugin_id: String,
    pub display_name: String,
    pub provider_family: StorageProviderFamily,
    pub common_fields: Vec<StorageSchemaField>,
    pub credential_fields: Vec<StorageSchemaField>,
    pub supported_credential_modes: Vec<StorageCredentialMode>,
    pub capabilities: Vec<String>,
}

impl StorageProviderSchema {
    pub fn new(
        provider_plugin_id: impl Into<String>,
        display_name: impl Into<String>,
        provider_family: StorageProviderFamily,
    ) -> Self {
        Self {
            provider_plugin_id: provider_plugin_id.into(),
            display_name: display_name.into(),
            provider_family,
            common_fields: Vec::new(),
            credential_fields: Vec::new(),
            supported_credential_modes: Vec::new(),
            capabilities: Vec::new(),
        }
    }

    pub fn with_common_fields<I>(mut self, fields: I) -> Self
    where
        I: IntoIterator<Item = StorageSchemaField>,
    {
        self.common_fields = fields.into_iter().collect();
        self
    }

    pub fn with_credential_fields<I>(mut self, fields: I) -> Self
    where
        I: IntoIterator<Item = StorageSchemaField>,
    {
        self.credential_fields = fields.into_iter().collect();
        self
    }

    pub fn with_supported_credential_modes<I>(mut self, modes: I) -> Self
    where
        I: IntoIterator<Item = StorageCredentialMode>,
    {
        self.supported_credential_modes = modes.into_iter().collect();
        self
    }

    pub fn with_capabilities<I, S>(mut self, capabilities: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.capabilities = capabilities.into_iter().map(Into::into).collect();
        self
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StorageCatalog {
    pub domain: String,
    pub provider_schemas: Vec<StorageProviderSchema>,
}

impl StorageCatalog {
    pub fn object_storage() -> Self {
        Self {
            domain: "object-storage".into(),
            provider_schemas: vec![
                StorageProviderSchema::new(
                    "object-storage-aliyun",
                    "Aliyun OSS",
                    StorageProviderFamily::S3Compatible,
                )
                .with_common_fields(default_s3_common_fields())
                .with_credential_fields(default_access_key_fields("accessKeyId", "accessKeySecret"))
                .with_supported_credential_modes([
                    StorageCredentialMode::AccessKeyPair,
                    StorageCredentialMode::SessionAccessKeyPair,
                ])
                .with_capabilities(["presign", "multipart", "bucket-probe"]),
                StorageProviderSchema::new(
                    "object-storage-tencent",
                    "Tencent COS",
                    StorageProviderFamily::S3Compatible,
                )
                .with_common_fields(default_s3_common_fields())
                .with_credential_fields(default_access_key_fields("secretId", "secretKey"))
                .with_supported_credential_modes([
                    StorageCredentialMode::AccessKeyPair,
                    StorageCredentialMode::SessionAccessKeyPair,
                ])
                .with_capabilities(["presign", "multipart", "bucket-probe"]),
                StorageProviderSchema::new(
                    "object-storage-volcengine",
                    "Volcengine TOS",
                    StorageProviderFamily::S3Compatible,
                )
                .with_common_fields(default_s3_common_fields())
                .with_credential_fields(default_access_key_fields("accessKeyId", "secretAccessKey"))
                .with_supported_credential_modes([
                    StorageCredentialMode::AccessKeyPair,
                    StorageCredentialMode::SessionAccessKeyPair,
                ])
                .with_capabilities(["presign", "multipart", "bucket-probe"]),
                StorageProviderSchema::new(
                    "object-storage-aws",
                    "Amazon S3",
                    StorageProviderFamily::S3Compatible,
                )
                .with_common_fields(default_s3_common_fields())
                .with_credential_fields(
                    default_access_key_fields("accessKeyId", "secretAccessKey")
                        .into_iter()
                        .chain(aws_role_assumption_fields())
                        .collect::<Vec<_>>(),
                )
                .with_supported_credential_modes([
                    StorageCredentialMode::AccessKeyPair,
                    StorageCredentialMode::SessionAccessKeyPair,
                    StorageCredentialMode::RoleAssumption,
                ])
                .with_capabilities(["presign", "multipart", "bucket-probe"]),
                StorageProviderSchema::new(
                    "object-storage-google",
                    "Google Cloud Storage",
                    StorageProviderFamily::GoogleCloudStorage,
                )
                .with_common_fields(default_gcs_common_fields())
                .with_credential_fields(vec![
                    StorageSchemaField::new(
                        "serviceAccountJson",
                        "Service Account JSON",
                        StorageFieldInputKind::Json,
                        true,
                    )
                    .with_credential_modes([StorageCredentialMode::ServiceAccountJson]),
                    StorageSchemaField::new(
                        "interoperabilityAccessKey",
                        "Interoperability Access Key",
                        StorageFieldInputKind::Text,
                        true,
                    )
                    .with_credential_modes([StorageCredentialMode::InteroperabilityKey]),
                    StorageSchemaField::new(
                        "interoperabilitySecretKey",
                        "Interoperability Secret Key",
                        StorageFieldInputKind::Secret,
                        true,
                    )
                    .with_credential_modes([StorageCredentialMode::InteroperabilityKey]),
                ])
                .with_supported_credential_modes([
                    StorageCredentialMode::ServiceAccountJson,
                    StorageCredentialMode::InteroperabilityKey,
                ])
                .with_capabilities(["presign", "bucket-probe"]),
                StorageProviderSchema::new(
                    "object-storage-microsoft",
                    "Azure Blob Storage",
                    StorageProviderFamily::AzureBlob,
                )
                .with_common_fields(default_azure_common_fields())
                .with_credential_fields(vec![
                    StorageSchemaField::new(
                        "accountName",
                        "Account Name",
                        StorageFieldInputKind::Text,
                        true,
                    ),
                    StorageSchemaField::new(
                        "accountKey",
                        "Account Key",
                        StorageFieldInputKind::Secret,
                        true,
                    )
                    .with_credential_modes([StorageCredentialMode::AccountKey]),
                    StorageSchemaField::new(
                        "sasToken",
                        "SAS Token",
                        StorageFieldInputKind::Secret,
                        true,
                    )
                    .with_credential_modes([StorageCredentialMode::SasToken]),
                    StorageSchemaField::new(
                        "tenantId",
                        "Tenant ID",
                        StorageFieldInputKind::Text,
                        true,
                    )
                    .with_credential_modes([StorageCredentialMode::ServicePrincipal]),
                    StorageSchemaField::new(
                        "clientId",
                        "Client ID",
                        StorageFieldInputKind::Text,
                        true,
                    )
                    .with_credential_modes([StorageCredentialMode::ServicePrincipal]),
                    StorageSchemaField::new(
                        "clientSecret",
                        "Client Secret",
                        StorageFieldInputKind::Secret,
                        true,
                    )
                    .with_credential_modes([StorageCredentialMode::ServicePrincipal]),
                ])
                .with_supported_credential_modes([
                    StorageCredentialMode::AccountKey,
                    StorageCredentialMode::SasToken,
                    StorageCredentialMode::ServicePrincipal,
                ])
                .with_capabilities(["presign", "bucket-probe"]),
            ],
        }
    }

    pub fn provider_plugin_ids(&self) -> impl Iterator<Item = &str> {
        self.provider_schemas
            .iter()
            .map(|schema| schema.provider_plugin_id.as_str())
    }

    pub fn provider_schema(&self, provider_plugin_id: &str) -> Option<&StorageProviderSchema> {
        self.provider_schemas
            .iter()
            .find(|schema| schema.provider_plugin_id == provider_plugin_id)
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StorageConfigUpsertInput {
    #[serde(default)]
    pub binding: StorageConfigUpsertBindingInput,
    #[serde(default)]
    pub config: StorageConfigUpsertConfigInput,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secret: Option<StorageConfigUpsertSecretInput>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StorageConfigUpsertBindingInput {
    pub provider_plugin_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StorageConfigUpsertConfigInput {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bucket_or_container: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub public_base_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub upload_prefix: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub download_prefix: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider_config: Option<Value>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StorageConfigUpsertSecretInput {
    pub credential_mode: StorageCredentialMode,
    pub encrypted_secret_payload: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secret_fingerprint: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StorageBindingRecord {
    pub scope: StorageScopeRef,
    pub provider_plugin_id: String,
    pub enabled: bool,
}

impl StorageBindingRecord {
    pub fn new_global(provider_plugin_id: impl Into<String>) -> Self {
        Self {
            scope: StorageScopeRef::global(),
            provider_plugin_id: provider_plugin_id.into(),
            enabled: true,
        }
    }

    pub fn new_tenant(tenant_id: impl Into<String>, provider_plugin_id: impl Into<String>) -> Self {
        Self {
            scope: StorageScopeRef::tenant(tenant_id),
            provider_plugin_id: provider_plugin_id.into(),
            enabled: true,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StorageConfigRecord {
    pub scope: StorageScopeRef,
    pub provider_plugin_id: String,
    pub bucket_or_container: Option<String>,
    pub region: Option<String>,
    pub endpoint: Option<String>,
    pub public_base_url: Option<String>,
    pub upload_prefix: Option<String>,
    pub download_prefix: Option<String>,
    pub provider_config: Value,
}

impl StorageConfigRecord {
    pub fn new_global(provider_plugin_id: impl Into<String>) -> Self {
        Self::new(StorageScopeRef::global(), provider_plugin_id)
    }

    pub fn new_tenant(tenant_id: impl Into<String>, provider_plugin_id: impl Into<String>) -> Self {
        Self::new(StorageScopeRef::tenant(tenant_id), provider_plugin_id)
    }

    fn new(scope: StorageScopeRef, provider_plugin_id: impl Into<String>) -> Self {
        Self {
            scope,
            provider_plugin_id: provider_plugin_id.into(),
            bucket_or_container: None,
            region: None,
            endpoint: None,
            public_base_url: None,
            upload_prefix: None,
            download_prefix: None,
            provider_config: Value::Object(Map::new()),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StorageSecretRecord {
    pub scope: StorageScopeRef,
    pub provider_plugin_id: String,
    pub credential_mode: StorageCredentialMode,
    pub encrypted_secret_payload: String,
    pub secret_fingerprint: String,
}

impl StorageSecretRecord {
    pub fn new_global(
        provider_plugin_id: impl Into<String>,
        credential_mode: StorageCredentialMode,
        encrypted_secret_payload: impl Into<String>,
    ) -> Self {
        Self::new(
            StorageScopeRef::global(),
            provider_plugin_id,
            credential_mode,
            encrypted_secret_payload,
        )
    }

    pub fn new_tenant(
        tenant_id: impl Into<String>,
        provider_plugin_id: impl Into<String>,
        credential_mode: StorageCredentialMode,
        encrypted_secret_payload: impl Into<String>,
    ) -> Self {
        Self::new(
            StorageScopeRef::tenant(tenant_id),
            provider_plugin_id,
            credential_mode,
            encrypted_secret_payload,
        )
    }

    fn new(
        scope: StorageScopeRef,
        provider_plugin_id: impl Into<String>,
        credential_mode: StorageCredentialMode,
        encrypted_secret_payload: impl Into<String>,
    ) -> Self {
        let encrypted_secret_payload = encrypted_secret_payload.into();
        Self {
            scope,
            provider_plugin_id: provider_plugin_id.into(),
            credential_mode,
            secret_fingerprint: encrypted_secret_payload.clone(),
            encrypted_secret_payload,
        }
    }

    pub fn with_secret_fingerprint(mut self, secret_fingerprint: impl Into<String>) -> Self {
        self.secret_fingerprint = secret_fingerprint.into();
        self
    }

    pub fn redacted_summary(&self) -> StorageSecretSummary {
        StorageSecretSummary {
            scope: self.scope.clone(),
            provider_plugin_id: self.provider_plugin_id.clone(),
            credential_mode: self.credential_mode,
            configured: true,
            secret_fingerprint: self.secret_fingerprint.clone(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StorageSecretSummary {
    pub scope: StorageScopeRef,
    pub provider_plugin_id: String,
    pub credential_mode: StorageCredentialMode,
    pub configured: bool,
    pub secret_fingerprint: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StorageConfigSnapshot {
    pub scope: StorageScopeRef,
    pub binding: Option<StorageBindingRecord>,
    pub config: Option<StorageConfigRecord>,
    pub secret: Option<StorageSecretSummary>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum StorageValidationStatus {
    Healthy,
    Degraded,
    Invalid,
    Unknown,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum StorageValidationStage {
    Schema,
    Credentials,
    Bucket,
    Presign,
    Readback,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StorageValidationResult {
    pub scope: StorageScopeRef,
    pub status: StorageValidationStatus,
    pub stage: StorageValidationStage,
    pub message: String,
    pub provider_plugin_id: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StorageAuditRecord {
    pub id: String,
    pub action: String,
    pub scope: StorageScopeRef,
    pub provider_plugin_id: String,
    pub created_at_ms: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StorageEffectiveConfig {
    pub requested_scope: StorageScopeRef,
    pub resolved_scope: StorageScopeRef,
    pub binding: StorageBindingRecord,
    pub config: StorageConfigRecord,
    pub secret: Option<StorageSecretSummary>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StorageDomainSnapshot {
    pub catalog: StorageCatalog,
    pub bindings: Vec<StorageBindingRecord>,
    pub configs: Vec<StorageConfigRecord>,
    pub secrets: Vec<StorageSecretRecord>,
}

impl StorageDomainSnapshot {
    pub fn new(catalog: StorageCatalog) -> Self {
        Self {
            catalog,
            bindings: Vec::new(),
            configs: Vec::new(),
            secrets: Vec::new(),
        }
    }

    pub fn with_binding(mut self, binding: StorageBindingRecord) -> Self {
        self.bindings.push(binding);
        self
    }

    pub fn with_config(mut self, config: StorageConfigRecord) -> Self {
        self.configs.push(config);
        self
    }

    pub fn with_secret(mut self, secret: StorageSecretRecord) -> Self {
        self.secrets.push(secret);
        self
    }

    pub fn effective_config(
        &self,
        requested_scope: StorageScopeRef,
    ) -> Option<StorageEffectiveConfig> {
        let candidates = match requested_scope.kind {
            StorageScopeKind::Tenant => vec![requested_scope.clone(), StorageScopeRef::global()],
            StorageScopeKind::Global => vec![StorageScopeRef::global()],
        };

        for scope in candidates {
            let Some(binding) = self.binding_for_scope(&scope).cloned() else {
                continue;
            };
            let Some(config) = self.config_for_scope(&scope).cloned() else {
                continue;
            };
            let secret = self
                .secret_for_scope(&scope)
                .map(StorageSecretRecord::redacted_summary);

            return Some(StorageEffectiveConfig {
                requested_scope,
                resolved_scope: scope,
                binding,
                config,
                secret,
            });
        }

        None
    }

    fn binding_for_scope(&self, scope: &StorageScopeRef) -> Option<&StorageBindingRecord> {
        self.bindings.iter().find(|binding| binding.scope == *scope)
    }

    fn config_for_scope(&self, scope: &StorageScopeRef) -> Option<&StorageConfigRecord> {
        self.configs.iter().find(|config| config.scope == *scope)
    }

    fn secret_for_scope(&self, scope: &StorageScopeRef) -> Option<&StorageSecretRecord> {
        self.secrets.iter().find(|secret| secret.scope == *scope)
    }
}

pub trait StorageDomainSnapshotStore: Send + Sync {
    fn load_snapshot(&self, domain: &str) -> Result<Option<StorageDomainSnapshot>, ContractError>;

    fn save_snapshot(&self, snapshot: StorageDomainSnapshot) -> Result<(), ContractError>;
}

fn default_s3_common_fields() -> Vec<StorageSchemaField> {
    vec![
        StorageSchemaField::new(
            "bucketOrContainer",
            "Bucket",
            StorageFieldInputKind::Text,
            true,
        ),
        StorageSchemaField::new("region", "Region", StorageFieldInputKind::Text, true),
        StorageSchemaField::new("endpoint", "Endpoint", StorageFieldInputKind::Url, false),
        StorageSchemaField::new(
            "publicBaseUrl",
            "Public Base URL",
            StorageFieldInputKind::Url,
            false,
        ),
        StorageSchemaField::new(
            "uploadPrefix",
            "Upload Prefix",
            StorageFieldInputKind::Text,
            false,
        ),
        StorageSchemaField::new(
            "downloadPrefix",
            "Download Prefix",
            StorageFieldInputKind::Text,
            false,
        ),
        StorageSchemaField::new(
            "pathStyle",
            "Path Style",
            StorageFieldInputKind::Boolean,
            false,
        ),
    ]
}

fn default_gcs_common_fields() -> Vec<StorageSchemaField> {
    vec![
        StorageSchemaField::new(
            "bucketOrContainer",
            "Bucket",
            StorageFieldInputKind::Text,
            true,
        ),
        StorageSchemaField::new("region", "Region", StorageFieldInputKind::Text, false),
        StorageSchemaField::new(
            "publicBaseUrl",
            "Public Base URL",
            StorageFieldInputKind::Url,
            false,
        ),
        StorageSchemaField::new(
            "uploadPrefix",
            "Upload Prefix",
            StorageFieldInputKind::Text,
            false,
        ),
        StorageSchemaField::new(
            "downloadPrefix",
            "Download Prefix",
            StorageFieldInputKind::Text,
            false,
        ),
    ]
}

fn default_azure_common_fields() -> Vec<StorageSchemaField> {
    vec![
        StorageSchemaField::new(
            "bucketOrContainer",
            "Container",
            StorageFieldInputKind::Text,
            true,
        ),
        StorageSchemaField::new("endpoint", "Endpoint", StorageFieldInputKind::Url, false),
        StorageSchemaField::new(
            "publicBaseUrl",
            "Public Base URL",
            StorageFieldInputKind::Url,
            false,
        ),
        StorageSchemaField::new(
            "uploadPrefix",
            "Upload Prefix",
            StorageFieldInputKind::Text,
            false,
        ),
        StorageSchemaField::new(
            "downloadPrefix",
            "Download Prefix",
            StorageFieldInputKind::Text,
            false,
        ),
    ]
}

fn default_access_key_fields(
    access_key_field: &'static str,
    secret_key_field: &'static str,
) -> Vec<StorageSchemaField> {
    vec![
        StorageSchemaField::new(
            access_key_field,
            "Access Key",
            StorageFieldInputKind::Text,
            true,
        )
        .with_credential_modes([
            StorageCredentialMode::AccessKeyPair,
            StorageCredentialMode::SessionAccessKeyPair,
        ]),
        StorageSchemaField::new(
            secret_key_field,
            "Secret Key",
            StorageFieldInputKind::Secret,
            true,
        )
        .with_credential_modes([
            StorageCredentialMode::AccessKeyPair,
            StorageCredentialMode::SessionAccessKeyPair,
        ]),
        StorageSchemaField::new(
            "sessionToken",
            "Session Token",
            StorageFieldInputKind::Secret,
            true,
        )
        .with_credential_modes([StorageCredentialMode::SessionAccessKeyPair]),
    ]
}

fn aws_role_assumption_fields() -> Vec<StorageSchemaField> {
    vec![
        StorageSchemaField::new("roleArn", "Role ARN", StorageFieldInputKind::Text, true)
            .with_credential_modes([StorageCredentialMode::RoleAssumption]),
        StorageSchemaField::new(
            "externalId",
            "External ID",
            StorageFieldInputKind::Text,
            false,
        )
        .with_credential_modes([StorageCredentialMode::RoleAssumption]),
        StorageSchemaField::new(
            "sessionName",
            "Session Name",
            StorageFieldInputKind::Text,
            false,
        )
        .with_credential_modes([StorageCredentialMode::RoleAssumption]),
    ]
}
