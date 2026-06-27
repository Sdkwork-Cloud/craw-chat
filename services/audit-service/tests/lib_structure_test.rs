#[test]
fn test_audit_runtime_uses_record_id_index_and_read_write_guards() {
    let source = include_str!("../src/lib.rs").replace("\r\n", "\n");

    assert!(
        !source.contains("records: Mutex<HashMap<String, Vec<AuditRecord>>>"),
        "audit runtime must not keep tenant records as Vec<AuditRecord>; idempotency checks need record_id lookup"
    );
    assert!(
        source.contains("records: RwLock<HashMap<String, TenantAuditRecords>>"),
        "audit runtime should keep tenant records behind an RwLock-owned indexed store"
    );
    assert!(
        source.contains("struct TenantAuditRecords"),
        "audit runtime should isolate per-tenant audit storage details"
    );
    assert!(
        source.contains("by_record_id: HashMap<String, AuditRecord>"),
        "audit runtime should index audit records by record_id"
    );
    assert!(
        source.contains("by_audit_seq: BTreeMap<u64, String>"),
        "audit runtime should index audit records by tenant-local audit_seq for cursor reads"
    );
    assert!(
        source.contains("record_order: Vec<String>"),
        "audit runtime should keep append order separately from record payload storage"
    );
    assert!(
        source.contains(".get(request.record_id.as_str())"),
        "record_anchor should use direct record_id lookup for idempotency"
    );
    assert!(
        source.contains(".range((Excluded(after_audit_seq), Unbounded))"),
        "audit record listing should range-seek from afterAuditSeq"
    );
    assert!(
        source.contains("audit_seq: next_audit_seq"),
        "audit records should receive a server-assigned tenant-local audit_seq"
    );
    assert!(
        source.contains("fn read_records(") && source.contains("fn write_records("),
        "audit runtime should expose explicit read/write guard helpers"
    );
}
