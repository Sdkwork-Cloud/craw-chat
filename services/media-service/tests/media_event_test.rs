use std::fs;
use std::path::Path;

#[test]
fn test_media_service_does_not_emit_media_asset_lifecycle_events() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let source =
        fs::read_to_string(manifest_dir.join("src/lib.rs")).expect("media-service lib readable");

    for forbidden in [
        "MediaAsset",
        "media_asset",
        "media.asset",
        "upload.created",
        "upload.completed",
    ] {
        assert!(
            !source.contains(forbidden),
            "media-service must not emit app-local media asset lifecycle event term `{forbidden}`"
        );
    }
}
