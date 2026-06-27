use im_platform_contracts::{
    CommitEnvelope, CommitJournal, CommitPosition, ContractError, LeaseGrant, LeaseStore, MetadataStore,
    ObjectDescriptor, ObjectPutRequest, ObjectStore, RealtimeDisconnectFenceRecord,
    RealtimeDisconnectFenceStore, TimelineProjectionStore,
};

struct NullJournal;
struct NullMetadata;
struct NullProjection;
struct NullLeaseStore;
struct NullObjectStore;
struct NullDisconnectFenceStore;

impl CommitJournal for NullJournal {
    fn append(
        &self,
        _envelope: CommitEnvelope,
    ) -> Result<CommitPosition, ContractError> {
        Ok(CommitPosition::new("p0", 1))
    }
}

impl MetadataStore for NullMetadata {
    fn put_snapshot(&self, _scope: &str, _key: &str, _value: &str) -> Result<(), ContractError> {
        Ok(())
    }

    fn load_snapshot(&self, _scope: &str, _key: &str) -> Result<Option<String>, ContractError> {
        Ok(None)
    }
}

impl TimelineProjectionStore for NullProjection {
    fn upsert_timeline_entry(
        &self,
        _tenant_id: &str,
        _timeline_scope: &str,
        _message_seq: u64,
        _payload: &str,
    ) -> Result<(), ContractError> {
        Ok(())
    }

    fn load_timeline(
        &self,
        _tenant_id: &str,
        _timeline_scope: &str,
    ) -> Result<Vec<(u64, String)>, ContractError> {
        Ok(Vec::new())
    }
}

impl RealtimeDisconnectFenceStore for NullDisconnectFenceStore {
    fn load_fence(
        &self,
        _tenant_id: &str,
        _principal_kind: &str,
        _principal_id: &str,
        _device_id: &str,
    ) -> Result<Option<RealtimeDisconnectFenceRecord>, ContractError> {
        Ok(None)
    }

    fn save_fence(&self, _record: RealtimeDisconnectFenceRecord) -> Result<(), ContractError> {
        Ok(())
    }

    fn clear_fence(
        &self,
        _tenant_id: &str,
        _principal_kind: &str,
        _principal_id: &str,
        _device_id: &str,
    ) -> Result<bool, ContractError> {
        Ok(false)
    }

    fn clear_fence_disconnected_at_or_before(
        &self,
        _tenant_id: &str,
        _principal_kind: &str,
        _principal_id: &str,
        _device_id: &str,
        _cutoff_disconnected_at: &str,
    ) -> Result<bool, ContractError> {
        Ok(false)
    }

    fn clear_fence_if_matches(
        &self,
        _expected: &RealtimeDisconnectFenceRecord,
    ) -> Result<bool, ContractError> {
        Ok(false)
    }
}

impl LeaseStore for NullLeaseStore {
    fn acquire(&self, grant: LeaseGrant) -> Result<LeaseGrant, ContractError> {
        Ok(grant)
    }
}

impl ObjectStore for NullObjectStore {
    fn put(&self, request: ObjectPutRequest) -> Result<ObjectDescriptor, ContractError> {
        Ok(ObjectDescriptor {
            object_key: request.object_key,
            content_length: request.content_length,
        })
    }
}

#[test]
fn test_contract_types_are_usable_without_binding_to_a_vendor() {
    let journal = NullJournal;
    let metadata = NullMetadata;
    let projection = NullProjection;
    let disconnect_fence_store = NullDisconnectFenceStore;
    let lease_store = NullLeaseStore;
    let position = journal.append(CommitEnvelope::minimal(
        "evt_demo",
        "100001",
        "message.posted",
        "conversation",
        "c_demo",
        1,
    ));

    assert_eq!(position.expect("append should succeed").cursor(), "p0:1");

    let object_store = NullObjectStore;
    let descriptor = object_store
        .put(ObjectPutRequest {
            object_key: "asset/demo.png".into(),
            content_length: 42,
        })
        .expect("put should succeed");

    metadata
        .put_snapshot("tenant", "demo", "value")
        .expect("metadata snapshot should succeed");
    metadata
        .load_snapshot("tenant", "demo")
        .expect("metadata load should succeed");
    projection
        .upsert_timeline_entry("100001", "c_demo", 1, "{}")
        .expect("projection upsert should succeed");
    projection
        .load_timeline("100001", "c_demo")
        .expect("projection load should succeed");
    disconnect_fence_store
        .save_fence(RealtimeDisconnectFenceRecord {
            tenant_id: "100001".into(),
            principal_kind: "user".into(),
            principal_id: "1".into(),
            device_id: "d_demo".into(),
            session_id: Some("s_demo".into()),
            owner_node_id: "node_a".into(),
            disconnected_at: "2026-04-06T00:00:00Z".into(),
            fence_token: "fence:100001:1:d_demo:s_demo:node_a:2026-04-06T00:00:00Z".into(),
        })
        .expect("disconnect fence save should succeed");
    lease_store
        .acquire(LeaseGrant {
            scope_id: "c_demo".into(),
            owner_node_id: "node_a".into(),
            epoch: 1,
        })
        .expect("lease acquire should succeed");

    assert_eq!(descriptor.object_key, "asset/demo.png");
}
