use im_adapter_object_storage_s3::{
    GOOGLE_OBJECT_STORAGE_PLUGIN_ID, S3CompatibleObjectStorageProvider,
    VOLCENGINE_OBJECT_STORAGE_PLUGIN_ID,
};
use im_platform_contracts::{
    ObjectStorageDownloadUrlRequest, ObjectStorageProvider, ObjectStoragePutRequest,
    ObjectStorageUploadUrlRequest, ProviderDomain,
};

#[test]
fn test_volcengine_s3_adapter_exposes_expected_contract_shape() {
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
