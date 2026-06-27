pub const PREFIX: &str = "/backend/v3/api/admin";

pub const PROTOCOL_REGISTRY: &str = "/backend/v3/api/control/protocol_registry";
pub const PROTOCOL_GOVERNANCE: &str = "/backend/v3/api/control/protocol_governance";
pub const PROVIDER_REGISTRY: &str = "/backend/v3/api/control/provider_registry";
pub const PROVIDER_BINDINGS: &str = "/backend/v3/api/control/provider_bindings";
pub const PROVIDER_POLICIES: &str = "/backend/v3/api/control/provider_policies";
pub const PROVIDER_POLICIES_DIFF: &str = "/backend/v3/api/control/provider_policies/diff";
pub const PROVIDER_POLICIES_PREVIEW: &str = "/backend/v3/api/control/provider_policies/preview";
pub const PROVIDER_POLICIES_ROLLBACK: &str = "/backend/v3/api/control/provider_policies/rollback";
pub const NODE_DRAIN: &str = "/backend/v3/api/control/nodes/{node_id}/drain";
pub const NODE_ACTIVATE: &str = "/backend/v3/api/control/nodes/{node_id}/activate";
pub const NODE_ROUTES_MIGRATE: &str = "/backend/v3/api/control/nodes/{node_id}/routes/migrate";
