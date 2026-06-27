#[test]
fn test_streaming_runtime_frame_store_uses_sequence_index() {
    let source = format!(
        "{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}",
        include_str!("../src/lib.rs"),
        include_str!("../src/state.rs"),
        include_str!("../src/dto.rs"),
        include_str!("../src/error.rs"),
        include_str!("../src/openapi.rs"),
        include_str!("../src/handlers.rs"),
        include_str!("../src/app.rs"),
        include_str!("../src/helpers.rs"),
    )
    .replace("\r\n", "\n");

    assert!(
        !source.contains("frames: Mutex<HashMap<String, Vec<StreamFrame>>>"),
        "streaming runtime must not keep stream frames in Vec; cursor reads and idempotency checks need sequence lookup"
    );
    assert!(
        source.contains("frames: Mutex<HashMap<String, BTreeMap<u64, StreamFrame>>>"),
        "streaming runtime should index frames by frame_seq per stream"
    );
    assert!(
        source.contains(".range((Excluded(after_frame_seq), Unbounded))"),
        "stream frame listing should range-seek from afterFrameSeq"
    );
    assert!(
        source.contains(".get(&request.frame_seq)"),
        "stream append retry/conflict detection should perform direct frame_seq lookup"
    );
    assert!(
        source.contains("fn encode_stream_key_segments"),
        "streaming runtime keys need a single segment-safe encoder for sessions, frames, state, and idempotency"
    );
    assert!(
        source.contains("encode_stream_key_segments([tenant_id, stream_id])"),
        "stream scope keys must use segment-safe length-prefixed encoding"
    );
    assert!(
        source.contains("encode_stream_key_segments([\n        auth.tenant_id.as_str(),"),
        "stream idempotency keys must use the segment-safe stream key encoder"
    );
    assert!(
        !source.contains("format!(\"{tenant_id}:{stream_id}\")"),
        "streaming runtime scope keys must not use delimiter-composed tenant/stream ids"
    );
    for forbidden in [
        "{}:{}:{}:open:{}",
        "{}:{}:{}:complete:{}",
        "{}:{}:{}:checkpoint:{}:{}",
        "{}:{}:{}:abort:{}",
        "{}:{}:{}:append:{}:{}",
    ] {
        assert!(
            !source.contains(forbidden),
            "stream idempotency keys must not use delimiter-composed format string: {forbidden}"
        );
    }
}
