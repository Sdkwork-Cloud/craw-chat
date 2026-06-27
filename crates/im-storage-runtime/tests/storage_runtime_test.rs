use im_storage_contracts::{
    StorageCatalog, StorageConfigUpsertBindingInput, StorageConfigUpsertConfigInput,
    StorageConfigUpsertInput, StorageConfigUpsertSecretInput, StorageCredentialMode,
    StorageDomainSnapshot, StorageDomainSnapshotStore, StorageValidationStage,
    StorageValidationStatus,
};
use im_storage_runtime::{
    StorageConfigUpsert, StorageRuntimeState, StoreBackedStorageRuntime,
    storage_config_upsert_from_input,
};
use sdkwork_im_contract_core::ContractError;
use serde_json::json;
use std::sync::{Arc, Mutex};

fn aws_access_key_payload() -> &'static str {
    "{\"accessKeyId\":\"global-access-key\",\"secretAccessKey\":\"global-secret-key\"}"
}

fn gcs_service_account_payload() -> &'static str {
    "{\"serviceAccountJson\":{\"client_email\":\"tenant@sdkwork.local\"}}"
}

#[derive(Clone, Default)]
struct MemoryStorageSnapshotStore {
    snapshot: Arc<Mutex<Option<StorageDomainSnapshot>>>,
}

impl StorageDomainSnapshotStore for MemoryStorageSnapshotStore {
    fn load_snapshot(&self, _domain: &str) -> Result<Option<StorageDomainSnapshot>, ContractError> {
        Ok(self
            .snapshot
            .lock()
            .expect("snapshot mutex should lock")
            .clone())
    }

    fn save_snapshot(&self, snapshot: StorageDomainSnapshot) -> Result<(), ContractError> {
        *self.snapshot.lock().expect("snapshot mutex should lock") = Some(snapshot);
        Ok(())
    }
}

#[test]
fn runtime_state_supports_global_and_tenant_storage_resolution() {
    let mut state = StorageRuntimeState::new_object_storage();

    state.save_global(
        StorageConfigUpsert::new("object-storage-aws")
            .with_bucket_or_container("global-assets")
            .with_region("us-east-1")
            .with_secret(
                StorageCredentialMode::AccessKeyPair,
                aws_access_key_payload(),
                "fp-global",
            ),
    );

    state.save_tenant(
        "tenant-northstar",
        StorageConfigUpsert::new("object-storage-google")
            .with_bucket_or_container("tenant-assets")
            .with_region("asia-east1")
            .with_secret(
                StorageCredentialMode::ServiceAccountJson,
                gcs_service_account_payload(),
                "fp-tenant",
            ),
    );

    let effective_tenant = state
        .effective_for_tenant("tenant-northstar")
        .expect("tenant override should resolve");
    assert_eq!(effective_tenant.resolved_scope.kind.as_str(), "tenant");
    assert_eq!(
        effective_tenant.binding.provider_plugin_id,
        "object-storage-google"
    );

    state.delete_tenant("tenant-northstar");
    let effective_global = state
        .effective_for_tenant("tenant-northstar")
        .expect("global fallback should resolve");
    assert_eq!(effective_global.resolved_scope.kind.as_str(), "global");
    assert_eq!(
        effective_global.binding.provider_plugin_id,
        "object-storage-aws"
    );
}

#[test]
fn runtime_state_validates_required_storage_fields() {
    let mut state = StorageRuntimeState::new_object_storage();
    state.save_global(StorageConfigUpsert::new("object-storage-aws"));

    let validation = state.validate_global();
    assert_eq!(validation.status, StorageValidationStatus::Invalid);
    assert_eq!(validation.stage, StorageValidationStage::Schema);

    state.save_global(
        StorageConfigUpsert::new("object-storage-aws")
            .with_bucket_or_container("global-assets")
            .with_region("us-east-1")
            .with_secret(
                StorageCredentialMode::AccessKeyPair,
                aws_access_key_payload(),
                "fp-global",
            ),
    );

    let healthy = state.validate_global();
    assert_eq!(healthy.status, StorageValidationStatus::Healthy);
    assert_eq!(healthy.stage, StorageValidationStage::Presign);
}

#[test]
fn runtime_state_exposes_catalog_and_scope_snapshots_for_consumers() {
    let mut state = StorageRuntimeState::new_object_storage();

    assert!(
        state
            .catalog()
            .provider_plugin_ids()
            .any(|provider_plugin_id| provider_plugin_id == "object-storage-aws")
    );

    let empty_global_snapshot = state.global_snapshot();
    assert_eq!(empty_global_snapshot.scope.kind.as_str(), "global");
    assert!(empty_global_snapshot.binding.is_none());
    assert!(empty_global_snapshot.config.is_none());
    assert!(empty_global_snapshot.secret.is_none());

    state.save_tenant(
        "tenant-northstar",
        StorageConfigUpsert::new("object-storage-google")
            .with_bucket_or_container("tenant-assets")
            .with_region("asia-east1")
            .with_public_base_url("https://cdn.tenant.example")
            .with_provider_config(serde_json::json!({
                "projectId": "tenant-project",
            }))
            .with_secret(
                StorageCredentialMode::ServiceAccountJson,
                gcs_service_account_payload(),
                "fp-tenant",
            ),
    );

    let tenant_snapshot = state.tenant_snapshot("tenant-northstar");
    assert_eq!(tenant_snapshot.scope.kind.as_str(), "tenant");
    assert_eq!(
        tenant_snapshot
            .binding
            .as_ref()
            .map(|binding| binding.provider_plugin_id.as_str()),
        Some("object-storage-google")
    );
    assert_eq!(
        tenant_snapshot
            .config
            .as_ref()
            .and_then(|config| config.public_base_url.as_deref()),
        Some("https://cdn.tenant.example")
    );
    assert_eq!(
        tenant_snapshot
            .config
            .as_ref()
            .map(|config| config.provider_config.clone()),
        Some(serde_json::json!({
            "projectId": "tenant-project",
        }))
    );
}

#[test]
fn runtime_state_validates_tenant_scope_with_global_fallback_and_non_empty_secrets() {
    let mut state = StorageRuntimeState::new_object_storage();
    state.save_global(
        StorageConfigUpsert::new("object-storage-aws")
            .with_bucket_or_container("global-assets")
            .with_region("us-east-1")
            .with_secret(
                StorageCredentialMode::AccessKeyPair,
                aws_access_key_payload(),
                "fp-global",
            ),
    );

    let fallback_validation = state.validate_tenant("tenant-missing");
    assert_eq!(fallback_validation.status, StorageValidationStatus::Healthy);
    assert_eq!(fallback_validation.stage, StorageValidationStage::Presign);
    assert_eq!(
        fallback_validation.provider_plugin_id.as_deref(),
        Some("object-storage-aws")
    );

    state.save_tenant(
        "tenant-empty-secret",
        StorageConfigUpsert::new("object-storage-google")
            .with_bucket_or_container("tenant-assets")
            .with_secret(StorageCredentialMode::ServiceAccountJson, "", "fp-empty"),
    );

    let invalid_tenant_validation = state.validate_tenant("tenant-empty-secret");
    assert_eq!(
        invalid_tenant_validation.status,
        StorageValidationStatus::Invalid
    );
    assert_eq!(
        invalid_tenant_validation.stage,
        StorageValidationStage::Credentials
    );
    assert_eq!(
        invalid_tenant_validation.provider_plugin_id.as_deref(),
        Some("object-storage-google")
    );
}

#[test]
fn runtime_state_records_storage_audit_entries() {
    let mut state = StorageRuntimeState::new_object_storage();
    state.save_global(
        StorageConfigUpsert::new("object-storage-aws")
            .with_bucket_or_container("global-assets")
            .with_secret(
                StorageCredentialMode::AccessKeyPair,
                aws_access_key_payload(),
                "fp-global",
            ),
    );
    state.save_tenant(
        "tenant-northstar",
        StorageConfigUpsert::new("object-storage-google")
            .with_bucket_or_container("tenant-assets")
            .with_secret(
                StorageCredentialMode::ServiceAccountJson,
                gcs_service_account_payload(),
                "fp-tenant",
            ),
    );
    state.delete_tenant("tenant-northstar");

    let audit = state.audit_trail();
    assert_eq!(audit.len(), 3);
    assert_eq!(audit[0].action, "delete");
    assert_eq!(audit[1].action, "upsert");
    assert_eq!(audit[2].action, "upsert");
}

#[test]
fn runtime_state_preserves_existing_secret_when_same_provider_updates_without_secret() {
    let mut state = StorageRuntimeState::new_object_storage();
    state.save_global(
        StorageConfigUpsert::new("object-storage-aws")
            .with_bucket_or_container("global-assets")
            .with_region("us-east-1")
            .with_secret(
                StorageCredentialMode::AccessKeyPair,
                aws_access_key_payload(),
                "fp-global",
            ),
    );

    state.save_global(
        StorageConfigUpsert::new("object-storage-aws")
            .with_bucket_or_container("global-assets-v2")
            .with_region("us-west-2")
            .with_public_base_url("https://cdn.global.example"),
    );

    let snapshot = state.global_snapshot();
    assert_eq!(
        snapshot
            .config
            .as_ref()
            .and_then(|config| config.bucket_or_container.as_deref()),
        Some("global-assets-v2")
    );
    assert_eq!(
        snapshot
            .secret
            .as_ref()
            .map(|secret| secret.secret_fingerprint.as_str()),
        Some("fp-global")
    );

    let validation = state.validate_global();
    assert_eq!(validation.status, StorageValidationStatus::Healthy);
    assert_eq!(validation.stage, StorageValidationStage::Presign);
}

#[test]
fn runtime_state_validates_mode_specific_secret_fields_from_stored_records() {
    let mut state = StorageRuntimeState::new_object_storage();
    state.save_global(
        StorageConfigUpsert::new("object-storage-google")
            .with_bucket_or_container("tenant-assets")
            .with_secret(
                StorageCredentialMode::InteroperabilityKey,
                "{\"interoperabilityAccessKey\":\"interop-access-key\"}",
                "fp-google-interop",
            ),
    );

    let validation = state.validate_global();
    assert_eq!(validation.status, StorageValidationStatus::Invalid);
    assert_eq!(validation.stage, StorageValidationStage::Credentials);
    assert_eq!(
        validation.message,
        "Interoperability Secret Key is required."
    );
}

#[test]
fn runtime_parses_storage_config_upsert_input_from_api_payloads() {
    let catalog = StorageCatalog::object_storage();
    let upsert = storage_config_upsert_from_input(
        &catalog,
        &StorageConfigUpsertInput {
            binding: StorageConfigUpsertBindingInput {
                provider_plugin_id: "object-storage-aws".into(),
                enabled: Some(false),
            },
            config: StorageConfigUpsertConfigInput {
                bucket_or_container: Some("global-assets".into()),
                region: Some("us-east-1".into()),
                endpoint: Some("https://s3.amazonaws.com".into()),
                public_base_url: None,
                upload_prefix: None,
                download_prefix: None,
                provider_config: Some(json!({
                    "pathStyle": true,
                })),
            },
            secret: Some(StorageConfigUpsertSecretInput {
                credential_mode: StorageCredentialMode::RoleAssumption,
                encrypted_secret_payload: "{\"roleArn\":\"arn:aws:iam::123456789012:role/sdkwork\",\"sessionName\":\"sdkwork-runtime\"}".into(),
                secret_fingerprint: Some("fp-role".into()),
            }),
        },
    )
    .expect("runtime should parse valid typed api payloads");

    let mut state = StorageRuntimeState::new_object_storage();
    state.save_global(upsert);

    let snapshot = state.global_snapshot();
    assert_eq!(
        snapshot
            .binding
            .as_ref()
            .map(|binding| binding.provider_plugin_id.as_str()),
        Some("object-storage-aws")
    );
    assert_eq!(
        snapshot.binding.as_ref().map(|binding| binding.enabled),
        Some(false)
    );
    assert_eq!(
        snapshot
            .config
            .as_ref()
            .and_then(|config| config.provider_config.get("pathStyle"))
            .and_then(|value| value.as_bool()),
        Some(true)
    );
    assert_eq!(
        snapshot
            .secret
            .as_ref()
            .map(|secret| secret.credential_mode),
        Some(StorageCredentialMode::RoleAssumption)
    );
}

#[test]
fn runtime_rejects_unsupported_credential_modes_for_a_provider() {
    let catalog = StorageCatalog::object_storage();
    let error = storage_config_upsert_from_input(
        &catalog,
        &StorageConfigUpsertInput {
            binding: StorageConfigUpsertBindingInput {
                provider_plugin_id: "object-storage-google".into(),
                enabled: None,
            },
            config: StorageConfigUpsertConfigInput {
                bucket_or_container: Some("tenant-assets".into()),
                region: None,
                endpoint: None,
                public_base_url: None,
                upload_prefix: None,
                download_prefix: None,
                provider_config: None,
            },
            secret: Some(StorageConfigUpsertSecretInput {
                credential_mode: StorageCredentialMode::RoleAssumption,
                encrypted_secret_payload:
                    "{\"roleArn\":\"arn:aws:iam::123456789012:role/sdkwork\"}".into(),
                secret_fingerprint: None,
            }),
        },
    )
    .expect_err("provider-specific unsupported modes should be rejected");

    assert_eq!(
        error,
        "Storage provider object-storage-google does not support credential mode role-assumption."
    );
}

#[test]
fn runtime_state_roundtrips_through_domain_snapshot_exports() {
    let mut state = StorageRuntimeState::new_object_storage();
    state.save_global(
        StorageConfigUpsert::new("object-storage-aws")
            .with_bucket_or_container("global-assets")
            .with_region("us-east-1")
            .with_public_base_url("https://cdn.global.example")
            .with_provider_config(json!({
                "pathStyle": true,
            }))
            .with_secret(
                StorageCredentialMode::AccessKeyPair,
                aws_access_key_payload(),
                "fp-global",
            ),
    );
    state.save_tenant(
        "tenant-northstar",
        StorageConfigUpsert::new("object-storage-google")
            .with_bucket_or_container("tenant-assets")
            .with_region("asia-east1")
            .with_secret(
                StorageCredentialMode::ServiceAccountJson,
                gcs_service_account_payload(),
                "fp-tenant",
            ),
    );

    let snapshot = state.domain_snapshot();
    let restored = StorageRuntimeState::from_domain_snapshot(snapshot.clone());

    assert_eq!(restored.domain_snapshot(), snapshot);
    assert_eq!(
        restored
            .effective_for_tenant("tenant-northstar")
            .as_ref()
            .map(|effective| effective.binding.provider_plugin_id.as_str()),
        Some("object-storage-google")
    );
}

#[test]
fn runtime_state_restores_validation_and_fallback_from_domain_snapshot() {
    let snapshot = StorageDomainSnapshot::new(StorageCatalog::object_storage())
        .with_binding(im_storage_contracts::StorageBindingRecord::new_global(
            "object-storage-aws",
        ))
        .with_config(im_storage_contracts::StorageConfigRecord::new_global(
            "object-storage-aws",
        ))
        .with_secret(im_storage_contracts::StorageSecretRecord::new_global(
            "object-storage-aws",
            StorageCredentialMode::AccessKeyPair,
            aws_access_key_payload(),
        ))
        .with_binding(im_storage_contracts::StorageBindingRecord::new_tenant(
            "tenant-invalid",
            "object-storage-google",
        ))
        .with_config({
            let mut config = im_storage_contracts::StorageConfigRecord::new_tenant(
                "tenant-invalid",
                "object-storage-google",
            );
            config.bucket_or_container = Some("tenant-invalid-assets".into());
            config
        })
        .with_secret(im_storage_contracts::StorageSecretRecord::new_tenant(
            "tenant-invalid",
            "object-storage-google",
            StorageCredentialMode::InteroperabilityKey,
            "{\"interoperabilityAccessKey\":\"interop-access-key\"}",
        ));

    let restored = StorageRuntimeState::from_domain_snapshot(snapshot);

    let fallback = restored
        .effective_for_tenant("tenant-missing")
        .expect("global fallback should remain available");
    assert_eq!(fallback.binding.provider_plugin_id, "object-storage-aws");

    let invalid_validation = restored.validate_tenant("tenant-invalid");
    assert_eq!(invalid_validation.status, StorageValidationStatus::Invalid);
    assert_eq!(
        invalid_validation.stage,
        StorageValidationStage::Credentials
    );
    assert_eq!(
        invalid_validation.message,
        "Interoperability Secret Key is required."
    );
}

#[test]
fn runtime_state_loads_default_catalog_when_store_is_empty() {
    let store = MemoryStorageSnapshotStore::default();

    let state = StorageRuntimeState::load_from_store(&store, StorageCatalog::object_storage())
        .expect("empty store should still hydrate a runtime state");

    assert_eq!(state.catalog().domain, "object-storage");
    assert!(state.global_snapshot().binding.is_none());
    assert!(state.tenant_snapshot("tenant-missing").binding.is_none());
}

#[test]
fn runtime_state_persists_domain_snapshot_through_store_boundary() {
    let store = MemoryStorageSnapshotStore::default();
    let mut state = StorageRuntimeState::new_object_storage();
    state.save_global(
        StorageConfigUpsert::new("object-storage-aws")
            .with_bucket_or_container("global-assets")
            .with_region("us-east-1")
            .with_secret(
                StorageCredentialMode::AccessKeyPair,
                aws_access_key_payload(),
                "fp-global",
            ),
    );

    state
        .save_to_store(&store)
        .expect("runtime state should persist as a shared snapshot");

    let restored = StorageRuntimeState::load_from_store(&store, StorageCatalog::object_storage())
        .expect("persisted snapshot should hydrate back into runtime state");
    assert_eq!(restored.domain_snapshot(), state.domain_snapshot());
}

#[test]
fn store_backed_runtime_persists_global_and_tenant_mutations_through_shared_store() {
    let store = MemoryStorageSnapshotStore::default();
    let mut runtime = StoreBackedStorageRuntime::load(store, StorageCatalog::object_storage())
        .expect("store-backed runtime should load");

    runtime
        .save_global(
            StorageConfigUpsert::new("object-storage-aws")
                .with_bucket_or_container("global-assets")
                .with_region("us-east-1")
                .with_secret(
                    StorageCredentialMode::AccessKeyPair,
                    aws_access_key_payload(),
                    "fp-global",
                ),
        )
        .expect("global storage save should persist through the store");
    runtime
        .save_tenant(
            "tenant-northstar",
            StorageConfigUpsert::new("object-storage-google")
                .with_bucket_or_container("tenant-assets")
                .with_region("asia-east1")
                .with_secret(
                    StorageCredentialMode::ServiceAccountJson,
                    gcs_service_account_payload(),
                    "fp-tenant",
                ),
        )
        .expect("tenant storage save should persist through the store");

    let restored =
        StoreBackedStorageRuntime::load(runtime.store().clone(), StorageCatalog::object_storage())
            .expect("a new runtime should load the persisted snapshot");
    assert_eq!(
        restored
            .effective_for_tenant("tenant-northstar")
            .as_ref()
            .map(|effective| effective.binding.provider_plugin_id.as_str()),
        Some("object-storage-google")
    );
    assert_eq!(
        restored
            .global_snapshot()
            .secret
            .as_ref()
            .map(|secret| secret.secret_fingerprint.as_str()),
        Some("fp-global")
    );
}

#[test]
fn store_backed_runtime_persists_tenant_delete_and_keeps_in_memory_audit() {
    let store = MemoryStorageSnapshotStore::default();
    let mut runtime = StoreBackedStorageRuntime::load(store, StorageCatalog::object_storage())
        .expect("store-backed runtime should load");

    runtime
        .save_global(
            StorageConfigUpsert::new("object-storage-aws")
                .with_bucket_or_container("global-assets")
                .with_region("us-east-1")
                .with_secret(
                    StorageCredentialMode::AccessKeyPair,
                    aws_access_key_payload(),
                    "fp-global",
                ),
        )
        .expect("global storage save should persist through the store");
    runtime
        .save_tenant(
            "tenant-northstar",
            StorageConfigUpsert::new("object-storage-google")
                .with_bucket_or_container("tenant-assets")
                .with_region("asia-east1")
                .with_secret(
                    StorageCredentialMode::ServiceAccountJson,
                    gcs_service_account_payload(),
                    "fp-tenant",
                ),
        )
        .expect("tenant storage save should persist through the store");

    assert!(
        runtime
            .delete_tenant("tenant-northstar")
            .expect("tenant delete should persist through the store")
    );
    assert_eq!(runtime.audit_trail().len(), 3);

    let restored =
        StoreBackedStorageRuntime::load(runtime.store().clone(), StorageCatalog::object_storage())
            .expect("restored runtime should reload the persisted snapshot after delete");
    assert_eq!(
        restored
            .effective_for_tenant("tenant-northstar")
            .as_ref()
            .map(|effective| effective.resolved_scope.kind.as_str()),
        Some("global")
    );
    assert!(restored.audit_trail().is_empty());
}
