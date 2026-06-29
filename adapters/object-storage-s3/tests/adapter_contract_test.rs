use im_adapter_object_storage_s3::{
    S3CompatibleObjectStorageProvider, S3CompatibleObjectStorageProviderConfig,
    GOOGLE_OBJECT_STORAGE_PLUGIN_ID, VOLCENGINE_OBJECT_STORAGE_PLUGIN_ID,
};
use im_platform_contracts::{
    ObjectStorageDownloadUrlRequest, ObjectStorageProvider, ObjectStoragePutRequest,
    ObjectStorageUploadUrlRequest, ProviderDomain,
};
use sdkwork_im_contract_core::ContractError;

/// Force the SDKWork environment to a non-production value so the S3 adapter's
/// P0-13 fail-closed gate allows the unsigned dev/test fallback path. Env vars
/// are process-global in Rust 2024, so `set_var` is wrapped in `unsafe`.
fn ensure_dev_environment() {
    unsafe {
        std::env::set_var("SDKWORK_IM_ENVIRONMENT", "test");
    }
}

#[test]
fn test_volcengine_s3_adapter_exposes_expected_contract_shape() {
    ensure_dev_environment();
    let provider = S3CompatibleObjectStorageProvider::volcengine_default();
    let descriptor = provider.descriptor();

    assert_eq!(descriptor.plugin_id, VOLCENGINE_OBJECT_STORAGE_PLUGIN_ID);
    assert_eq!(descriptor.domain, ProviderDomain::ObjectStorage);
    assert_eq!(descriptor.provider_kind, "volcengine");
    assert_eq!(
        descriptor.required_capabilities,
        vec!["s3", "presign", "multipart"]
    );

    let object = provider
        .put_object(ObjectStoragePutRequest {
            bucket: "media-demo".into(),
            object_key: "tenant/100001/demo.mp4".into(),
            content_length: 2048,
            content_type: Some("video/mp4".into()),
            storage_class: Some("standard".into()),
        })
        .expect("put_object should succeed");
    assert_eq!(object.bucket, "media-demo");
    assert!(
        object
            .etag
            .as_deref()
            .is_some_and(|etag| etag.contains("volcengine"))
    );

    // volcengine_default() carries no SigV4 credentials, so in dev/test the
    // adapter falls back to the unsigned URL format (`provider=`/`expires=`).
    // Production would fail-closed instead of producing these URLs.
    let url = provider
        .signed_download_url(ObjectStorageDownloadUrlRequest {
            bucket: "media-demo".into(),
            object_key: "tenant/100001/demo.mp4".into(),
            expires_in_seconds: 900,
        })
        .expect("signed_download_url should succeed");
    assert!(url.contains("provider=object-storage-volcengine"));
    assert!(url.contains("expires=900"));

    let upload = provider
        .signed_upload_url(ObjectStorageUploadUrlRequest {
            bucket: "media-demo".into(),
            object_key: "tenant/100001/demo.mp4".into(),
            content_length: Some(2048),
            content_type: Some("video/mp4".into()),
            expires_in_seconds: 900,
        })
        .expect("signed_upload_url should succeed");
    assert_eq!(upload.method, "PUT");
    assert!(upload.url.contains("provider=object-storage-volcengine"));
    assert!(upload.url.contains("upload=put"));
    assert_eq!(
        upload.headers.get("content-type"),
        Some(&"video/mp4".into())
    );
    // SSE-S3 default header is always attached (no KMS key configured).
    assert_eq!(
        upload.headers.get("X-Amz-Server-Side-Encryption"),
        Some(&"AES256".into())
    );
}

#[test]
fn test_google_s3_gateway_adapter_marks_gateway_capability() {
    let provider = S3CompatibleObjectStorageProvider::google_default();
    let descriptor = provider.descriptor();

    assert_eq!(descriptor.plugin_id, GOOGLE_OBJECT_STORAGE_PLUGIN_ID);
    assert_eq!(descriptor.provider_kind, "google");
    assert_eq!(
        descriptor.required_capabilities,
        vec!["s3-gateway", "presign"]
    );
}

#[test]
fn test_signed_url_with_credentials() {
    ensure_dev_environment();
    let provider = S3CompatibleObjectStorageProvider::new(S3CompatibleObjectStorageProviderConfig {
        plugin_id: VOLCENGINE_OBJECT_STORAGE_PLUGIN_ID.into(),
        provider_kind: "volcengine".into(),
        display_name: "Volcengine Object Storage".into(),
        endpoint: "https://tos.volcengine.local".into(),
        region: "cn-beijing".into(),
        gateway_mode: false,
        access_key_id: Some("AKIDTEST1234567890".into()),
        secret_access_key: Some("SECRET01234567890abcdef".into()),
        security_token: None,
        kms_key_id: Some("arn:aws:kms:cn-beijing:111122223333:key/abcd1234-5678".into()),
    });

    let download_url = provider
        .signed_download_url(ObjectStorageDownloadUrlRequest {
            bucket: "media-demo".into(),
            object_key: "tenant/100001/demo.mp4".into(),
            expires_in_seconds: 900,
        })
        .expect("signed_download_url should succeed with credentials");
    assert!(download_url.contains("X-Amz-Algorithm=AWS4-HMAC-SHA256"));
    assert!(download_url.contains("X-Amz-Credential="));
    assert!(download_url.contains("X-Amz-Date="));
    assert!(download_url.contains("X-Amz-Expires=900"));
    assert!(download_url.contains("X-Amz-SignedHeaders=host"));
    assert!(download_url.contains("X-Amz-Signature="));
    // The credential scope encodes `/` as %2F in the query string.
    assert!(download_url.contains("%2Fcn-beijing%2Fs3%2Faws4_request"));
    // Signature must be the last query parameter.
    let signature_idx = download_url
        .find("X-Amz-Signature=")
        .expect("signature param present");
    assert!(
        !download_url[signature_idx..].contains('&'),
        "X-Amz-Signature must be the last query parameter"
    );

    let upload = provider
        .signed_upload_url(ObjectStorageUploadUrlRequest {
            bucket: "media-demo".into(),
            object_key: "tenant/100001/demo.mp4".into(),
            content_length: Some(2048),
            content_type: Some("video/mp4".into()),
            expires_in_seconds: 900,
        })
        .expect("signed_upload_url should succeed with credentials");
    assert_eq!(upload.method, "PUT");
    assert!(upload.url.contains("X-Amz-Algorithm=AWS4-HMAC-SHA256"));
    assert!(upload.url.contains("X-Amz-Signature="));
    // SSE-KMS headers are attached because kms_key_id is configured.
    assert_eq!(
        upload.headers.get("X-Amz-Server-Side-Encryption"),
        Some(&"aws:kms".into())
    );
    assert_eq!(
        upload.headers.get("X-Amz-Server-Side-Encryption-Aws-Kms-Key-Id"),
        Some(&"arn:aws:kms:cn-beijing:111122223333:key/abcd1234-5678".into())
    );

    // Health snapshot reports healthy because credentials are configured.
    let health = provider.provider_health_snapshot();
    assert_eq!(health.status, "healthy");
    assert_eq!(
        health.details.get("credentialsConfigured"),
        Some(&"true".into())
    );
}

#[test]
fn test_mime_validation_rejects_unsafe_type() {
    let provider = S3CompatibleObjectStorageProvider::volcengine_default();
    let result = provider.put_object(ObjectStoragePutRequest {
        bucket: "media-demo".into(),
        object_key: "tenant/100001/page.html".into(),
        content_length: 128,
        content_type: Some("text/html".into()),
        storage_class: Some("standard".into()),
    });
    assert!(
        matches!(result, Err(ContractError::Invalid(_))),
        "text/html is not in the allowlist and must be rejected, got {result:?}"
    );
}

#[test]
fn test_mime_validation_rejects_extension_mismatch() {
    let provider = S3CompatibleObjectStorageProvider::volcengine_default();
    let result = provider.put_object(ObjectStoragePutRequest {
        bucket: "media-demo".into(),
        object_key: "image.png".into(),
        content_length: 256,
        content_type: Some("video/mp4".into()),
        storage_class: Some("standard".into()),
    });
    assert!(
        matches!(result, Err(ContractError::Invalid(_))),
        "video/mp4 does not match the .png extension and must be rejected, got {result:?}"
    );
}

#[test]
fn test_health_snapshot_unconfigured_without_credentials() {
    ensure_dev_environment();
    let provider = S3CompatibleObjectStorageProvider::volcengine_default();
    let health = provider.provider_health_snapshot();
    assert_eq!(health.status, "unconfigured");
    assert_eq!(
        health.details.get("credentialsConfigured"),
        Some(&"false".into())
    );
}
