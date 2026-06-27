use sdkwork_communication_rtc_service::RtcRecordingArtifact;
use serde_json::{Value, json};

#[test]
fn test_rtc_recording_artifact_is_drive_backed_media_resource() {
    let artifact_json = json!({
        "tenantId": "100001",
        "rtcSessionId": "rtc_recording_demo",
        "drive": {
            "driveUri": "drive://spaces/space_rtc_recordings/nodes/node_rtc_recording_demo",
            "spaceId": "space_rtc_recordings",
            "spaceType": "rtc",
            "nodeId": "node_rtc_recording_demo",
            "nodeVersion": "1"
        },
        "resource": {
            "id": "node_rtc_recording_demo",
            "kind": "video",
            "source": "provider_asset",
            "uri": "drive://spaces/space_rtc_recordings/nodes/node_rtc_recording_demo",
            "fileName": "rtc_recording_demo.mp4",
            "mimeType": "video/mp4"
        },
        "mediaRole": "rtc_recording"
    });

    let artifact = serde_json::from_value::<RtcRecordingArtifact>(artifact_json)
        .expect("RTC recording artifact should deserialize from Drive-backed media resource JSON");
    let serialized =
        serde_json::to_value(artifact).expect("RTC recording artifact should serialize as JSON");

    assert_eq!(
        serialized["drive"]["driveUri"],
        Value::String("drive://spaces/space_rtc_recordings/nodes/node_rtc_recording_demo".into())
    );
    assert_eq!(
        serialized["resource"]["uri"],
        Value::String("drive://spaces/space_rtc_recordings/nodes/node_rtc_recording_demo".into())
    );
    for forbidden in ["bucket", "objectKey", "storageProvider", "playbackUrl"] {
        assert!(
            serialized.get(forbidden).is_none(),
            "RTC recording artifact must not expose object-storage field {forbidden}"
        );
    }
}

#[test]
fn test_rtc_recording_artifact_rejects_object_storage_identity() {
    let legacy = json!({
        "tenantId": "100001",
        "rtcSessionId": "rtc_recording_demo",
        "bucket": "rtc-artifacts",
        "objectKey": "recordings/t_demo/rtc_recording_demo.mp4",
        "storageProvider": "object-storage-volcengine",
        "playbackUrl": "https://storage.example/rtc_recording_demo.mp4"
    });

    assert!(
        serde_json::from_value::<RtcRecordingArtifact>(legacy).is_err(),
        "RTC recording artifact must reject object-storage identity and signed playback fields"
    );
}
