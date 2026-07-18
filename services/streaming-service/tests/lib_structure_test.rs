#[test]
fn stream_runtime_uses_scoped_incremental_store_contract() {
    let runtime_source = format!(
        "{}\n{}\n{}",
        include_str!("../src/state.rs"),
        include_str!("../src/handlers.rs"),
        include_str!("../src/helpers.rs"),
    )
    .replace("\r\n", "\n");
    let contract_source =
        include_str!("../../../crates/sdkwork-im-contract-stream/src/lib.rs").replace("\r\n", "\n");
    let postgres_source =
        include_str!("../../../adapters/postgres-journal/src/stream_state_store.rs")
            .replace("\r\n", "\n");

    assert!(!runtime_source.contains("sessions: Mutex<HashMap"));
    assert!(!runtime_source.contains("frames: Mutex<HashMap"));
    assert!(contract_source.contains("pub organization_id: String"));
    assert!(contract_source.contains("fn list_frames_after("));
    assert!(contract_source.contains("expected_version: u64"));
    assert!(postgres_source.contains("frame_seq > $4"));
    assert!(postgres_source.contains("limit $5"));
    assert!(postgres_source.contains("for update"));
    assert!(postgres_source.contains("and version = $17"));
    assert!(!postgres_source.contains("order by frame_seq asc\n\"#"));
    assert!(!runtime_source.contains("pageSize"));
    assert!(runtime_source.contains("auth.organization_id.as_str()"));
    assert!(!runtime_source.contains("format!(\"{tenant_id}:{stream_id}\")"));
}
