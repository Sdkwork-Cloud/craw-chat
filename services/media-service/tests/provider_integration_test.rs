use std::fs;
use std::path::Path;

#[test]
fn test_media_service_has_no_object_storage_provider_integration() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let cargo_toml = fs::read_to_string(manifest_dir.join("Cargo.toml"))
        .expect("media-service Cargo.toml should be readable");
    let source =
        fs::read_to_string(manifest_dir.join("src/lib.rs")).expect("media-service lib readable");

    for forbidden in [
        "im-adapter-object-storage-s3",
        "ObjectStorage",
        "signed_download_url",
        "presign",
        "bucket",
        "object_key",
        "objectKey",
    ] {
        assert!(
            !cargo_toml.contains(forbidden) && !source.contains(forbidden),
            "media-service must not keep app-local object storage provider lifecycle term `{forbidden}`"
        );
    }
}
