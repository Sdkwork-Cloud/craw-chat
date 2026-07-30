#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ContractError {
    UnsupportedCapability(String),
    Conflict(String),
    Unavailable(String),
    Invalid(String),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PrivilegedOperationActorKind {
    ServiceWorker,
    OpsAdministrator,
}

impl PrivilegedOperationActorKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ServiceWorker => "service-worker",
            Self::OpsAdministrator => "ops-administrator",
        }
    }
}

/// Verified caller evidence carried by an explicitly privileged operation.
///
/// Construct this only after worker composition or administrative
/// authorization has selected the privileged execution path. Repository
/// adapters consume it for audit evidence; it is not an authorization bypass.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PrivilegedOperationContext {
    actor_kind: PrivilegedOperationActorKind,
    actor_id: String,
    trace_id: String,
}

impl PrivilegedOperationContext {
    pub fn try_new(
        actor_kind: PrivilegedOperationActorKind,
        actor_id: impl Into<String>,
        trace_id: impl Into<String>,
    ) -> Result<Self, ContractError> {
        let actor_id = actor_id.into();
        let trace_id = trace_id.into();
        if actor_id.trim().is_empty() || actor_id.len() > 128 {
            return Err(ContractError::Invalid(
                "privileged operation actor id must contain 1 to 128 bytes".into(),
            ));
        }
        if trace_id.trim().is_empty() || trace_id.len() > 256 {
            return Err(ContractError::Invalid(
                "privileged operation trace id must contain 1 to 256 bytes".into(),
            ));
        }
        Ok(Self {
            actor_kind,
            actor_id,
            trace_id,
        })
    }

    pub const fn actor_kind(&self) -> PrivilegedOperationActorKind {
        self.actor_kind
    }

    pub fn actor_id(&self) -> &str {
        self.actor_id.as_str()
    }

    pub fn trace_id(&self) -> &str {
        self.trace_id.as_str()
    }
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
