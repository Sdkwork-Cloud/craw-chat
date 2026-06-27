use std::fs;
use std::path::PathBuf;

fn service_source() -> String {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    fs::read_to_string(manifest_dir.join("src/lib.rs")).expect("automation source should be read")
}

#[test]
fn test_automation_runtime_keys_use_segment_safe_encoding() {
    let source = service_source();

    assert!(
        source.contains("fn encode_automation_key_segments"),
        "automation-service should keep a dedicated segment-safe key encoder"
    );
    assert!(
        !source.contains("format!(\"{tenant_id}:{principal_kind}:{principal_id}:{execution_id}\")"),
        "execution and execution-index keys must not use delimiter-only concatenation"
    );
    assert!(
        !source.contains("format!(\"{tenant_id}:{principal_kind}:{principal_id}:{stream_id}\")"),
        "agent response stream keys must not use delimiter-only concatenation"
    );
    assert!(
        !source.contains(
            "format!(\"{tenant_id}:{principal_kind}:{principal_id}:{execution_id}:{tool_call_id}\")"
        ),
        "tool-call keys must not use delimiter-only concatenation"
    );
    assert!(
        !source.contains("{}:{event_type}"),
        "automation event idempotency keys must use segment-safe encoding"
    );
    assert!(
        !source.contains("{}:{event_type}:{ordering_seq}"),
        "automation ordered event idempotency keys must use segment-safe encoding"
    );
}
