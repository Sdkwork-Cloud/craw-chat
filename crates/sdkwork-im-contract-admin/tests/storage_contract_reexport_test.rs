use sdkwork_im_contract_admin::storage::{
    StorageCatalog, StorageConfigUpsertInput, StorageCredentialMode, StorageDomainSnapshot,
    StorageDomainSnapshotStore, StorageScopeKind,
};
use sdkwork_im_contract_core::ContractError;

struct NullStorageSnapshotStore;

impl StorageDomainSnapshotStore for NullStorageSnapshotStore {
    fn load_snapshot(&self, _domain: &str) -> Result<Option<StorageDomainSnapshot>, ContractError> {
        Ok(None)
    }

    fn save_snapshot(&self, _snapshot: StorageDomainSnapshot) -> Result<(), ContractError> {
        Ok(())
    }
}

fn accepts_snapshot_store(_store: &dyn StorageDomainSnapshotStore) {}

#[test]
fn admin_contract_reexports_generic_storage_contracts() {
    let catalog = StorageCatalog::object_storage();
    let provider_ids = catalog.provider_plugin_ids().collect::<Vec<_>>();

    assert!(provider_ids.contains(&"object-storage-aws"));
    assert_eq!(StorageScopeKind::Tenant, StorageScopeKind::Tenant);
    assert_eq!(
        StorageCredentialMode::ServiceAccountJson.as_str(),
        "service-account-json"
    );
    assert_eq!(
        StorageConfigUpsertInput::default()
            .binding
            .provider_plugin_id,
        ""
    );
    accepts_snapshot_store(&NullStorageSnapshotStore);
}
