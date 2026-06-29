#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ContractError {
    UnsupportedCapability(String),
    Conflict(String),
    Unavailable(String),
    Invalid(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LeaseGrant {
    pub scope_id: String,
    pub owner_node_id: String,
    pub epoch: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ObjectPutRequest {
    pub object_key: String,
    pub content_length: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ObjectDescriptor {
    pub object_key: String,
    pub content_length: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MetadataSnapshotRecord {
    pub scope: String,
    pub key: String,
    pub value: String,
}

pub trait MetadataStore {
    fn put_snapshot(&self, scope: &str, key: &str, value: &str) -> Result<(), ContractError>;

    fn load_snapshot(&self, scope: &str, key: &str) -> Result<Option<String>, ContractError>;

    fn put_snapshots(&self, snapshots: &[MetadataSnapshotRecord]) -> Result<(), ContractError> {
        for snapshot in snapshots {
            self.put_snapshot(
                snapshot.scope.as_str(),
                snapshot.key.as_str(),
                snapshot.value.as_str(),
            )?;
        }
        Ok(())
    }
}

pub trait LeaseStore {
    fn acquire(&self, grant: LeaseGrant) -> Result<LeaseGrant, ContractError>;
}

pub trait ObjectStore {
    fn put(&self, request: ObjectPutRequest) -> Result<ObjectDescriptor, ContractError>;
}
