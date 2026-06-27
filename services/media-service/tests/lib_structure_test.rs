#[test]
fn test_media_service_is_not_an_app_local_storage_lifecycle_owner() {
    let source = include_str!("../src/lib.rs").replace("\r\n", "\n");

    for forbidden in [
        "HashMap<String, MediaAsset>",
        "CreateUploadRequest",
        "CompleteUploadRequest",
        "MediaUploadMutationResponse",
        "media_create_upload_request_key",
        "media_complete_upload_request_key",
        "encode_media_key_segments",
        "media_asset_id",
        "with_journal",
        "CommitJournal",
    ] {
        assert!(
            !source.contains(forbidden),
            "media-service must not own app-local storage lifecycle construct `{forbidden}`"
        );
    }
}
