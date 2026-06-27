use im_storage_contracts::{
    StorageBindingRecord, StorageCatalog, StorageConfigRecord, StorageConfigUpsertBindingInput,
    StorageConfigUpsertConfigInput, StorageConfigUpsertInput, StorageConfigUpsertSecretInput,
    StorageCredentialMode, StorageDomainSnapshot, StorageScopeRef, StorageSecretRecord,
};
use serde_json::json;

#[test]
fn object_storage_catalog_lists_all_supported_provider_plugins() {
    let catalog = StorageCatalog::object_storage();
    let provider_ids = catalog
        .provider_plugin_ids()
        .map(str::to_owned)
        .collect::<Vec<_>>();

    assert_eq!(
        provider_ids,
        vec![
            "object-storage-aliyun",
            "object-storage-tencent",
            "object-storage-volcengine",
            "object-storage-aws",
            "object-storage-google",
            "object-storage-microsoft",
        ]
    );
}

#[test]
fn tenant_override_resolves_before_global_fallback() {
    let catalog = StorageCatalog::object_storage();
    let snapshot = StorageDomainSnapshot::new(catalog)
        .with_binding(StorageBindingRecord::new_global("object-storage-aws"))
        .with_config(StorageConfigRecord::new_global("object-storage-aws"))
        .with_secret(StorageSecretRecord::new_global(
            "object-storage-aws",
            StorageCredentialMode::AccessKeyPair,
            "encrypted-global-secret",
        ))
        .with_binding(StorageBindingRecord::new_tenant(
            "100001",
            "object-storage-google",
        ))
        .with_config(StorageConfigRecord::new_tenant(
            "100001",
            "object-storage-google",
        ))
        .with_secret(StorageSecretRecord::new_tenant(
            "100001",
            "object-storage-google",
            StorageCredentialMode::ServiceAccountJson,
            "encrypted-tenant-secret",
        ));

    let effective = snapshot
        .effective_config(StorageScopeRef::tenant("100001"))
        .expect("tenant storage config should resolve");

    assert_eq!(
        effective.resolved_scope,
        StorageScopeRef::tenant("100001")
    );
    assert_eq!(
        effective.binding.provider_plugin_id,
        "object-storage-google"
    );
    assert_eq!(
        effective
            .secret
            .as_ref()
            .expect("tenant secret should be present")
            .secret_fingerprint,
        "encrypted-tenant-secret"
    );
}

#[test]
fn storage_summaries_mask_secret_values() {
    let secret = StorageSecretRecord::new_global(
        "object-storage-aws",
        StorageCredentialMode::AccessKeyPair,
        "raw-secret-value",
    )
    .with_secret_fingerprint("fingerprint-123");

    let summary = secret.redacted_summary();
    let debug = format!("{summary:?}");

    assert!(!debug.contains("raw-secret-value"));
    assert!(debug.contains("fingerprint-123"));
}

#[test]
fn provider_schemas_expose_field_groups_for_the_ui_in_camel_case() {
    let catalog = StorageCatalog::object_storage();
    let schema = catalog
        .provider_schema("object-storage-microsoft")
        .expect("microsoft storage schema should exist");

    assert_eq!(schema.provider_family.as_str(), "azure-blob");
    assert!(
        schema
            .common_fields
            .iter()
            .any(|field| field.name == "bucketOrContainer")
    );
    assert!(
        schema
            .credential_fields
            .iter()
            .any(|field| field.name == "sasToken")
    );
}

#[test]
fn provider_schemas_scope_credential_fields_to_supported_modes() {
    let catalog = StorageCatalog::object_storage();
    let google = catalog
        .provider_schema("object-storage-google")
        .expect("google storage schema should exist");
    let aws = catalog
        .provider_schema("object-storage-aws")
        .expect("aws storage schema should exist");
    let azure = catalog
        .provider_schema("object-storage-microsoft")
        .expect("azure storage schema should exist");

    let google_service_account = google
        .credential_fields
        .iter()
        .find(|field| field.name == "serviceAccountJson")
        .expect("service account field should exist");
    assert_eq!(
        google_service_account.credential_modes.as_deref(),
        Some(&[StorageCredentialMode::ServiceAccountJson][..])
    );

    let google_interop_secret = google
        .credential_fields
        .iter()
        .find(|field| field.name == "interoperabilitySecretKey")
        .expect("interop secret field should exist");
    assert_eq!(
        google_interop_secret.credential_modes.as_deref(),
        Some(&[StorageCredentialMode::InteroperabilityKey][..])
    );
    assert!(google_interop_secret.required);

    let aws_role_arn = aws
        .credential_fields
        .iter()
        .find(|field| field.name == "roleArn")
        .expect("aws role assumption field should exist");
    assert_eq!(
        aws_role_arn.credential_modes.as_deref(),
        Some(&[StorageCredentialMode::RoleAssumption][..])
    );
    assert!(aws_role_arn.required);

    let azure_sas_token = azure
        .credential_fields
        .iter()
        .find(|field| field.name == "sasToken")
        .expect("azure sas token field should exist");
    assert_eq!(
        azure_sas_token.credential_modes.as_deref(),
        Some(&[StorageCredentialMode::SasToken][..])
    );
    assert!(azure_sas_token.required);
}

#[test]
fn storage_upsert_input_contract_serializes_in_camel_case_for_admin_api_consumers() {
    let input = StorageConfigUpsertInput {
        binding: StorageConfigUpsertBindingInput {
            provider_plugin_id: "object-storage-aws".into(),
            enabled: Some(true),
        },
        config: StorageConfigUpsertConfigInput {
            bucket_or_container: Some("global-assets".into()),
            region: Some("us-east-1".into()),
            endpoint: Some("https://s3.amazonaws.com".into()),
            public_base_url: Some("https://cdn.global.example".into()),
            upload_prefix: Some("chat/uploads".into()),
            download_prefix: Some("chat/public".into()),
            provider_config: Some(json!({
                "pathStyle": true,
            })),
        },
        secret: Some(StorageConfigUpsertSecretInput {
            credential_mode: StorageCredentialMode::AccessKeyPair,
            encrypted_secret_payload:
                "{\"accessKeyId\":\"global-access-key\",\"secretAccessKey\":\"global-secret-key\"}"
                    .into(),
            secret_fingerprint: Some("fp-global-aws".into()),
        }),
    };

    let value = serde_json::to_value(&input).expect("storage upsert input should serialize");
    assert_eq!(
        value,
        json!({
            "binding": {
                "providerPluginId": "object-storage-aws",
                "enabled": true,
            },
            "config": {
                "bucketOrContainer": "global-assets",
                "region": "us-east-1",
                "endpoint": "https://s3.amazonaws.com",
                "publicBaseUrl": "https://cdn.global.example",
                "uploadPrefix": "chat/uploads",
                "downloadPrefix": "chat/public",
                "providerConfig": {
                    "pathStyle": true,
                },
            },
            "secret": {
                "credentialMode": "access-key-pair",
                "encryptedSecretPayload": "{\"accessKeyId\":\"global-access-key\",\"secretAccessKey\":\"global-secret-key\"}",
                "secretFingerprint": "fp-global-aws",
            },
        })
    );

    let roundtrip: StorageConfigUpsertInput =
        serde_json::from_value(value).expect("storage upsert input should deserialize");
    assert_eq!(roundtrip, input);
}
