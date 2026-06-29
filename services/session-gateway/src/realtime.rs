use im_platform_contracts::{
    RealtimeEventWindowDiagnosticsSnapshot, RealtimeEventWindowHighRiskRecord,
    RealtimeEventWindowRecord, RealtimeEventWindowStore,
};
use sdkwork_im_contract_control::{
    normalize_realtime_organization_id, RealtimeCheckpointRecord, RealtimeCheckpointStore,
    RealtimeMatchingSubscriptionQuery, RealtimeSubscriptionRecord, RealtimeSubscriptionStore,
};
use sdkwork_im_contract_core::ContractError;
use std::collections::hash_map::DefaultHasher;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::fmt;
use std::hash::{Hash, Hasher};
use std::ops::Bound::{Excluded, Unbounded};
use std::sync::{Arc, Mutex, MutexGuard};

use im_domain_core::realtime::{
    RealtimeAckState, RealtimeEvent, RealtimeEventWindow, RealtimeSubscription,
    RealtimeSubscriptionSnapshot,
};
use im_time::utc_now_rfc3339_millis;
use serde::Deserialize;
use tokio::sync::watch;

use crate::principal_scope::typed_client_route_scope_key;
use crate::realtime::storage::checkpoint_record_from_sequences;

pub mod postgres_sql;
mod storage;

pub use postgres_sql::{
    RealtimePostgresAdapterPlan, RealtimePostgresBindingError, RealtimePostgresBindingValue,
    RealtimePostgresBoundParameter, RealtimePostgresBoundStatement,
    RealtimePostgresBoundTransaction, RealtimePostgresCheckpointMutation,
    RealtimePostgresClientRouteEventMutation, RealtimePostgresMethodAtomicity,
    RealtimePostgresMethodPlan, RealtimePostgresMethodStep, RealtimePostgresParameterBinding,
    RealtimePostgresRowColumn, RealtimePostgresRowMapping, RealtimePostgresSqlContract,
    realtime_postgres_bind_ack_transaction, realtime_postgres_bind_checkpoint_upsert,
    realtime_postgres_bind_client_route_event_upsert, realtime_postgres_bind_publish_transaction,
    realtime_postgres_bind_save_subscription_transaction,
    realtime_postgres_bind_subscription_scope_clear,
    realtime_postgres_bind_subscription_scope_replacements,
    realtime_postgres_bind_subscription_upsert, realtime_postgres_bind_trim_client_route_events,
};
use storage::{
    RealtimeCheckpointRecordParts, RuntimeMemoryCheckpointStore, RuntimeMemoryEventWindowStore,
    RuntimeMemorySubscriptionStore,
};

const REALTIME_MAX_SCOPE_TYPE_BYTES: usize = 64;
const REALTIME_MAX_SCOPE_ID_BYTES: usize = 512;
const REALTIME_MAX_EVENT_TYPE_BYTES: usize = 128;
const REALTIME_MAX_EVENT_TYPES_TOTAL_BYTES: usize = 16 * 1024;
const REALTIME_MAX_SUBSCRIPTION_ITEMS: usize = 256;
const REALTIME_MAX_SUBSCRIPTION_ITEMS_TOTAL_BYTES: usize = 256 * 1024;
pub(crate) const REALTIME_EVENT_WINDOW_MAX_LIMIT: usize = 1000;
const REALTIME_CLIENT_ROUTE_WINDOW_MAX_RETAINED_EVENTS: usize = REALTIME_EVENT_WINDOW_MAX_LIMIT;
const REALTIME_CLIENT_ROUTE_WINDOW_CRITICAL_USAGE_PERMILLE: u64 = 950;
const REALTIME_MUTATION_LOCK_SHARDS: usize = 256;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RealtimeSubscriptionItemInput {
    pub scope_type: String,
    pub scope_id: String,
    #[serde(default)]
    pub event_types: Vec<String>,
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SyncRealtimeSubscriptionsRequest {
    pub device_id: Option<String>,
    #[serde(default)]
    pub items: Vec<RealtimeSubscriptionItemInput>,
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ListRealtimeEventsQuery {
    pub after_seq: Option<u64>,
    pub limit: Option<usize>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AckRealtimeEventsRequest {
    pub device_id: Option<String>,
    pub acked_seq: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RealtimeRuntimeError {
    pub code: &'static str,
    pub message: String,
}

impl From<ContractError> for RealtimeRuntimeError {
    fn from(value: ContractError) -> Self {
        Self::checkpoint_store(value)
    }
}

impl RealtimeRuntimeError {
    fn payload_too_large(field: &'static str, max_bytes: usize, actual_bytes: usize) -> Self {
        Self {
            code: "payload_too_large",
            message: format!(
                "payload too large for {field}: max={max_bytes} bytes, actual={actual_bytes} bytes"
            ),
        }
    }

    fn collection_too_large(field: &'static str, max_items: usize, actual_items: usize) -> Self {
        Self {
            code: "payload_too_large",
            message: format!(
                "payload too large for {field}: max={max_items} items, actual={actual_items} items"
            ),
        }
    }

    fn limit_invalid(limit: usize) -> Self {
        Self {
            code: "limit_invalid",
            message: format!(
                "limit must be between 1 and {REALTIME_EVENT_WINDOW_MAX_LIMIT}; actual={limit}"
            ),
        }
    }

    fn checkpoint_store(value: ContractError) -> Self {
        match value {
            ContractError::Unavailable(message) => Self {
                code: "checkpoint_store_unavailable",
                message,
            },
            ContractError::Conflict(message) => Self {
                code: "checkpoint_store_conflict",
                message,
            },
            ContractError::UnsupportedCapability(message) => Self {
                code: "checkpoint_store_unsupported",
                message,
            },
            ContractError::Invalid(message) => Self {
                code: "checkpoint_store_invalid",
                message,
            },
        }
    }

    fn subscription_store(value: ContractError) -> Self {
        match value {
            ContractError::Unavailable(message) => Self {
                code: "subscription_store_unavailable",
                message,
            },
            ContractError::Conflict(message) => Self {
                code: "subscription_store_conflict",
                message,
            },
            ContractError::UnsupportedCapability(message) => Self {
                code: "subscription_store_unsupported",
                message,
            },
            ContractError::Invalid(message) => Self {
                code: "subscription_store_invalid",
                message,
            },
        }
    }

    fn event_window_store(value: ContractError) -> Self {
        match value {
            ContractError::Unavailable(message) => Self {
                code: "event_window_store_unavailable",
                message,
            },
            ContractError::Conflict(message) => Self {
                code: "event_window_store_conflict",
                message,
            },
            ContractError::UnsupportedCapability(message) => Self {
                code: "event_window_store_unsupported",
                message,
            },
            ContractError::Invalid(message) => Self {
                code: "event_window_store_invalid",
                message,
            },
        }
    }
}

pub trait RealtimeScopeAccessPolicy: Send + Sync {
    fn validate_subscription_scope(
        &self,
        _tenant_id: &str,
        _principal_id: &str,
        _principal_kind: &str,
        _scope_type: &str,
        _scope_id: &str,
    ) -> Result<(), RealtimeRuntimeError> {
        Err(RealtimeRuntimeError {
            code: "realtime_scope_access_denied",
            message: "realtime scope access policy is not configured".into(),
        })
    }

    fn is_event_visible(
        &self,
        _tenant_id: &str,
        _principal_id: &str,
        _principal_kind: &str,
        _event: &RealtimeEvent,
    ) -> bool {
        false
    }
}

#[derive(Default)]
struct DenyAllRealtimeScopeAccessPolicy;

impl RealtimeScopeAccessPolicy for DenyAllRealtimeScopeAccessPolicy {}

#[derive(Default)]
pub struct StandaloneRealtimeScopeAccessPolicy;

impl RealtimeScopeAccessPolicy for StandaloneRealtimeScopeAccessPolicy {
    fn validate_subscription_scope(
        &self,
        _tenant_id: &str,
        _principal_id: &str,
        _principal_kind: &str,
        _scope_type: &str,
        _scope_id: &str,
    ) -> Result<(), RealtimeRuntimeError> {
        Ok(())
    }

    fn is_event_visible(
        &self,
        _tenant_id: &str,
        _principal_id: &str,
        _principal_kind: &str,
        _event: &RealtimeEvent,
    ) -> bool {
        true
    }
}

#[derive(Clone)]
pub struct RealtimeDeliveryRuntime {
    subscriptions: Arc<Mutex<HashMap<String, RealtimeClientRouteSubscriptions>>>,
    subscription_scope_index:
        Arc<Mutex<HashMap<RealtimePrincipalScopeKey, BTreeMap<String, RealtimeSubscription>>>>,
    windows: Arc<Mutex<HashMap<String, BTreeMap<u64, RealtimeEvent>>>>,
    latest_sequences: Arc<Mutex<HashMap<String, u64>>>,
    acked_sequences: Arc<Mutex<HashMap<String, u64>>>,
    trimmed_sequences: Arc<Mutex<HashMap<String, u64>>>,
    capacity_trimmed_event_counts: Arc<Mutex<HashMap<String, u64>>>,
    capacity_trimmed_sequences: Arc<Mutex<HashMap<String, u64>>>,
    last_capacity_trimmed_at: Arc<Mutex<HashMap<String, String>>>,
    notifiers: Arc<Mutex<HashMap<String, watch::Sender<u64>>>>,
    disconnect_generations: Arc<Mutex<HashMap<String, u64>>>,
    disconnect_notifiers: Arc<Mutex<HashMap<String, watch::Sender<u64>>>>,
    migrated_out_client_route_scopes: Arc<Mutex<HashSet<String>>>,
    mutation_locks: Arc<Vec<Mutex<()>>>,
    checkpoint_store: Arc<dyn RealtimeCheckpointStore>,
    subscription_store: Arc<dyn RealtimeSubscriptionStore>,
    event_window_store: Arc<dyn RealtimeEventWindowStore>,
    scope_access_policy: Arc<dyn RealtimeScopeAccessPolicy>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct RealtimeSubscriptionScopeKey {
    scope_type: String,
    scope_id: String,
}

impl RealtimeSubscriptionScopeKey {
    fn new(scope_type: &str, scope_id: &str) -> Self {
        Self {
            scope_type: scope_type.into(),
            scope_id: scope_id.into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct RealtimePrincipalScopeKey {
    tenant_id: String,
    organization_id: String,
    principal_kind: String,
    principal_id: String,
    scope: RealtimeSubscriptionScopeKey,
}

impl RealtimePrincipalScopeKey {
    fn new(
        tenant_id: &str,
        organization_id: &str,
        principal_kind: &str,
        principal_id: &str,
        scope_type: &str,
        scope_id: &str,
    ) -> Self {
        Self {
            tenant_id: tenant_id.into(),
            organization_id: normalize_realtime_organization_id(organization_id).into(),
            principal_kind: principal_kind.into(),
            principal_id: principal_id.into(),
            scope: RealtimeSubscriptionScopeKey::new(scope_type, scope_id),
        }
    }
}

#[derive(Clone, Debug, Default)]
struct RealtimeClientRouteSubscriptions {
    by_scope: HashMap<RealtimeSubscriptionScopeKey, RealtimeSubscription>,
    scope_order: Vec<RealtimeSubscriptionScopeKey>,
}

impl RealtimeClientRouteSubscriptions {
    fn from_items(items: Vec<RealtimeSubscription>) -> Self {
        let mut indexed = Self::default();
        for item in items {
            indexed.push(item);
        }
        indexed
    }

    fn push(&mut self, subscription: RealtimeSubscription) {
        let scope_key = RealtimeSubscriptionScopeKey::new(
            subscription.scope_type.as_str(),
            subscription.scope_id.as_str(),
        );
        if !self.by_scope.contains_key(&scope_key) {
            self.scope_order.push(scope_key.clone());
        }
        self.by_scope.insert(scope_key, subscription);
    }

    fn ordered_items(&self) -> Vec<RealtimeSubscription> {
        self.scope_order
            .iter()
            .filter_map(|scope_key| self.by_scope.get(scope_key).cloned())
            .collect()
    }
}

#[derive(Clone, Debug)]
struct RealtimePublishClientRouteMutation {
    scope_key: String,
    next_seq: u64,
    next_window: BTreeMap<u64, RealtimeEvent>,
    latest_realtime_seq: u64,
    trimmed_through_seq: u64,
    capacity_trimmed_event_count: u64,
    capacity_trimmed_through_seq: u64,
    last_capacity_trimmed_at: Option<String>,
    checkpoint: RealtimeCheckpointRecord,
    event_window: RealtimeEventWindowRecord,
}

/// Immutable snapshot of a single scope's realtime state, captured under the
/// global sequence/window locks. The snapshot is taken in a tight critical
/// section and the mutation (window insert + trim + checkpoint build) is
/// computed outside the lock so concurrent publishes targeting different
/// scopes do not serialize on the global HashMap mutexes during the
/// expensive clone/trim work.
#[derive(Clone, Debug)]
struct RealtimeScopeSnapshot {
    scope_key: String,
    device_id: String,
    latest_realtime_seq: u64,
    acked_through_seq: u64,
    trimmed_through_seq: u64,
    capacity_trimmed_event_count: u64,
    capacity_trimmed_through_seq: u64,
    last_capacity_trimmed_at: Option<String>,
    current_window: BTreeMap<u64, RealtimeEvent>,
}

struct RealtimeMutationScopeGuards {
    lock_indexes: Vec<usize>,
    locks: Arc<Vec<Mutex<()>>>,
}

impl RealtimeMutationScopeGuards {
    fn new(runtime: &RealtimeDeliveryRuntime, scope_keys: &[String]) -> Self {
        let mut lock_indexes = scope_keys
            .iter()
            .map(|scope_key| realtime_mutation_lock_index(scope_key.as_str()))
            .collect::<Vec<_>>();
        lock_indexes.sort_unstable();
        lock_indexes.dedup();
        Self {
            lock_indexes,
            locks: runtime.mutation_locks.clone(),
        }
    }

    fn lock(&self) -> Vec<MutexGuard<'_, ()>> {
        self.lock_indexes
            .iter()
            .map(|lock_index| {
                lock_realtime_mutex(
                    &self.locks[*lock_index],
                    "realtime principal mutation shard lock",
                )
            })
            .collect()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RealtimeWindowCheckpoint {
    pub latest_realtime_seq: u64,
    pub acked_through_seq: u64,
    pub trimmed_through_seq: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RealtimeClientRouteStateSnapshot {
    pub tenant_id: String,
    pub organization_id: String,
    pub principal_kind: String,
    pub principal_id: String,
    pub device_id: String,
    pub subscriptions: Vec<RealtimeSubscription>,
    pub events: Vec<RealtimeEvent>,
    pub latest_realtime_seq: u64,
    pub acked_through_seq: u64,
    pub trimmed_through_seq: u64,
    pub capacity_trimmed_event_count: u64,
    pub capacity_trimmed_through_seq: u64,
    pub last_capacity_trimmed_at: Option<String>,
    pub disconnect_generation: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RealtimeInboxDiagnosticsSnapshot {
    pub status: String,
    pub client_route_window_count: u64,
    pub pending_event_count: u64,
    pub max_client_route_window_event_count: u64,
    pub client_route_window_capacity: u64,
    pub max_client_route_window_usage_permille: u64,
    pub max_trimmed_through_seq: u64,
    pub capacity_trimmed_event_count: u64,
    pub max_capacity_trimmed_through_seq: u64,
    pub last_capacity_trimmed_at: Option<String>,
    pub oldest_pending_occurred_at: Option<String>,
    pub high_risk_windows: Vec<RealtimeInboxHighRiskWindow>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RealtimeInboxHighRiskWindow {
    pub tenant_id: String,
    pub principal_kind: String,
    pub principal_id: String,
    pub device_id: String,
    pub pending_event_count: u64,
    pub trimmed_through_seq: u64,
    pub capacity_trimmed_event_count: u64,
    pub capacity_trimmed_through_seq: u64,
    pub last_capacity_trimmed_at: Option<String>,
    pub usage_permille: u64,
    pub oldest_pending_occurred_at: Option<String>,
}

impl From<RealtimeEventWindowDiagnosticsSnapshot> for RealtimeInboxDiagnosticsSnapshot {
    fn from(value: RealtimeEventWindowDiagnosticsSnapshot) -> Self {
        let client_route_window_capacity = REALTIME_CLIENT_ROUTE_WINDOW_MAX_RETAINED_EVENTS as u64;
        let max_client_route_window_usage_permille = value
            .max_client_route_window_event_count
            .saturating_mul(1000)
            / client_route_window_capacity;
        let status = if max_client_route_window_usage_permille
            >= REALTIME_CLIENT_ROUTE_WINDOW_CRITICAL_USAGE_PERMILLE
        {
            "critical"
        } else if value.pending_event_count > 0 {
            "degraded"
        } else {
            "ok"
        };
        Self {
            status: status.into(),
            client_route_window_count: value.client_route_window_count,
            pending_event_count: value.pending_event_count,
            max_client_route_window_event_count: value.max_client_route_window_event_count,
            client_route_window_capacity,
            max_client_route_window_usage_permille,
            max_trimmed_through_seq: value.max_trimmed_through_seq,
            capacity_trimmed_event_count: value.capacity_trimmed_event_count,
            max_capacity_trimmed_through_seq: value.max_capacity_trimmed_through_seq,
            last_capacity_trimmed_at: value.last_capacity_trimmed_at,
            oldest_pending_occurred_at: value.oldest_pending_occurred_at,
            high_risk_windows: value
                .high_risk_windows
                .into_iter()
                .map(|window| {
                    RealtimeInboxHighRiskWindow::from_event_window_record(
                        window,
                        client_route_window_capacity,
                    )
                })
                .collect(),
        }
    }
}

impl RealtimeInboxHighRiskWindow {
    fn from_event_window_record(
        value: RealtimeEventWindowHighRiskRecord,
        client_route_window_capacity: u64,
    ) -> Self {
        Self {
            tenant_id: value.tenant_id,
            principal_kind: value.principal_kind,
            principal_id: value.principal_id,
            device_id: value.device_id,
            pending_event_count: value.pending_event_count,
            trimmed_through_seq: value.trimmed_through_seq,
            capacity_trimmed_event_count: value.capacity_trimmed_event_count,
            capacity_trimmed_through_seq: value.capacity_trimmed_through_seq,
            last_capacity_trimmed_at: value.last_capacity_trimmed_at,
            usage_permille: value.pending_event_count.saturating_mul(1000)
                / client_route_window_capacity,
            oldest_pending_occurred_at: value.oldest_pending_occurred_at,
        }
    }
}

impl Default for RealtimeDeliveryRuntime {
    fn default() -> Self {
        Self::with_checkpoint_store(Arc::new(RuntimeMemoryCheckpointStore::default()))
    }
}

impl fmt::Debug for RealtimeDeliveryRuntime {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("RealtimeDeliveryRuntime")
            .finish_non_exhaustive()
    }
}

impl RealtimeDeliveryRuntime {
    pub fn with_checkpoint_store(checkpoint_store: Arc<dyn RealtimeCheckpointStore>) -> Self {
        Self::with_stores_and_scope_access_policy(
            checkpoint_store,
            Arc::new(RuntimeMemorySubscriptionStore::default()),
            Arc::new(RuntimeMemoryEventWindowStore::default()),
            Arc::new(DenyAllRealtimeScopeAccessPolicy),
        )
    }

    pub fn with_stores(
        checkpoint_store: Arc<dyn RealtimeCheckpointStore>,
        subscription_store: Arc<dyn RealtimeSubscriptionStore>,
    ) -> Self {
        Self::with_stores_and_scope_access_policy(
            checkpoint_store,
            subscription_store,
            Arc::new(RuntimeMemoryEventWindowStore::default()),
            Arc::new(DenyAllRealtimeScopeAccessPolicy),
        )
    }

    pub fn standalone_gateway() -> Self {
        Self::with_stores_and_scope_access_policy(
            Arc::new(RuntimeMemoryCheckpointStore::default()),
            Arc::new(RuntimeMemorySubscriptionStore::default()),
            Arc::new(RuntimeMemoryEventWindowStore::default()),
            Arc::new(StandaloneRealtimeScopeAccessPolicy),
        )
    }

    pub fn with_checkpoint_store_for_standalone_gateway(
        checkpoint_store: Arc<dyn RealtimeCheckpointStore>,
    ) -> Self {
        Self::with_stores_and_scope_access_policy(
            checkpoint_store,
            Arc::new(RuntimeMemorySubscriptionStore::default()),
            Arc::new(RuntimeMemoryEventWindowStore::default()),
            Arc::new(StandaloneRealtimeScopeAccessPolicy),
        )
    }

    pub fn with_stores_for_standalone_gateway(
        checkpoint_store: Arc<dyn RealtimeCheckpointStore>,
        subscription_store: Arc<dyn RealtimeSubscriptionStore>,
    ) -> Self {
        Self::with_stores_and_scope_access_policy(
            checkpoint_store,
            subscription_store,
            Arc::new(RuntimeMemoryEventWindowStore::default()),
            Arc::new(StandaloneRealtimeScopeAccessPolicy),
        )
    }

    pub fn with_durable_stores_for_standalone_gateway(
        checkpoint_store: Arc<dyn RealtimeCheckpointStore>,
        subscription_store: Arc<dyn RealtimeSubscriptionStore>,
        event_window_store: Arc<dyn RealtimeEventWindowStore>,
    ) -> Self {
        Self::with_stores_and_scope_access_policy(
            checkpoint_store,
            subscription_store,
            event_window_store,
            Arc::new(StandaloneRealtimeScopeAccessPolicy),
        )
    }

    pub fn permissive_for_tests() -> Self {
        Self::standalone_gateway()
    }

    pub fn with_checkpoint_store_permissive_for_tests(
        checkpoint_store: Arc<dyn RealtimeCheckpointStore>,
    ) -> Self {
        Self::with_checkpoint_store_for_standalone_gateway(checkpoint_store)
    }

    pub fn with_stores_permissive_for_tests(
        checkpoint_store: Arc<dyn RealtimeCheckpointStore>,
        subscription_store: Arc<dyn RealtimeSubscriptionStore>,
    ) -> Self {
        Self::with_stores_for_standalone_gateway(checkpoint_store, subscription_store)
    }

    pub fn with_checkpoint_store_and_scope_access_policy(
        checkpoint_store: Arc<dyn RealtimeCheckpointStore>,
        scope_access_policy: Arc<dyn RealtimeScopeAccessPolicy>,
    ) -> Self {
        Self::with_stores_and_scope_access_policy(
            checkpoint_store,
            Arc::new(RuntimeMemorySubscriptionStore::default()),
            Arc::new(RuntimeMemoryEventWindowStore::default()),
            scope_access_policy,
        )
    }

    pub fn with_durable_stores_and_scope_access_policy(
        checkpoint_store: Arc<dyn RealtimeCheckpointStore>,
        subscription_store: Arc<dyn RealtimeSubscriptionStore>,
        event_window_store: Arc<dyn RealtimeEventWindowStore>,
        scope_access_policy: Arc<dyn RealtimeScopeAccessPolicy>,
    ) -> Self {
        Self::with_stores_and_scope_access_policy(
            checkpoint_store,
            subscription_store,
            event_window_store,
            scope_access_policy,
        )
    }

    pub fn with_stores_and_scope_access_policy(
        checkpoint_store: Arc<dyn RealtimeCheckpointStore>,
        subscription_store: Arc<dyn RealtimeSubscriptionStore>,
        event_window_store: Arc<dyn RealtimeEventWindowStore>,
        scope_access_policy: Arc<dyn RealtimeScopeAccessPolicy>,
    ) -> Self {
        Self {
            subscriptions: Arc::new(Mutex::new(HashMap::new())),
            subscription_scope_index: Arc::new(Mutex::new(HashMap::new())),
            windows: Arc::new(Mutex::new(HashMap::new())),
            latest_sequences: Arc::new(Mutex::new(HashMap::new())),
            acked_sequences: Arc::new(Mutex::new(HashMap::new())),
            trimmed_sequences: Arc::new(Mutex::new(HashMap::new())),
            capacity_trimmed_event_counts: Arc::new(Mutex::new(HashMap::new())),
            capacity_trimmed_sequences: Arc::new(Mutex::new(HashMap::new())),
            last_capacity_trimmed_at: Arc::new(Mutex::new(HashMap::new())),
            notifiers: Arc::new(Mutex::new(HashMap::new())),
            disconnect_generations: Arc::new(Mutex::new(HashMap::new())),
            disconnect_notifiers: Arc::new(Mutex::new(HashMap::new())),
            migrated_out_client_route_scopes: Arc::new(Mutex::new(HashSet::new())),
            mutation_locks: Arc::new(
                (0..REALTIME_MUTATION_LOCK_SHARDS)
                    .map(|_| Mutex::new(()))
                    .collect(),
            ),
            checkpoint_store,
            subscription_store,
            event_window_store,
            scope_access_policy,
        }
    }

    pub fn ensure_client_route_state_for_principal_kind(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_id: &str,
        principal_kind: &str,
        device_id: &str,
    ) -> Result<(), RealtimeRuntimeError> {
        self.ensure_client_route_state_internal(
            tenant_id,
            organization_id,
            principal_id,
            principal_kind,
            device_id,
        )
    }

    pub fn realtime_inbox_diagnostics(
        &self,
    ) -> Result<RealtimeInboxDiagnosticsSnapshot, RealtimeRuntimeError> {
        self.event_window_store
            .diagnostics_snapshot()
            .map(RealtimeInboxDiagnosticsSnapshot::from)
            .map_err(RealtimeRuntimeError::event_window_store)
    }

    fn ensure_client_route_state_internal(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_id: &str,
        principal_kind: &str,
        device_id: &str,
    ) -> Result<(), RealtimeRuntimeError> {
        let scope_key = client_route_scope_key(tenant_id, organization_id, principal_id, principal_kind, device_id);
        if lock_realtime_mutex(
            &self.migrated_out_client_route_scopes,
            "realtime migrated-out device scope store",
        )
        .contains(scope_key.as_str())
        {
            return Ok(());
        }
        let needs_restore = !lock_realtime_mutex(&self.latest_sequences, "realtime sequence store")
            .contains_key(scope_key.as_str());
        let restored = if needs_restore {
            self.checkpoint_store
                .load_checkpoint(tenant_id, organization_id, principal_kind, principal_id, device_id)
                .map_err(RealtimeRuntimeError::checkpoint_store)?
        } else {
            None
        };
        let restored_subscriptions = if needs_restore {
            self.subscription_store
                .load_subscriptions(tenant_id, organization_id, principal_kind, principal_id, device_id)
                .map_err(RealtimeRuntimeError::subscription_store)?
        } else {
            None
        };
        let restored_window = if needs_restore {
            self.event_window_store
                .load_window(tenant_id, organization_id, principal_kind, principal_id, device_id)
                .map_err(RealtimeRuntimeError::event_window_store)?
        } else {
            None
        };
        let restored_window_metadata = restored_window.as_ref().map(|record| {
            let record = record.clone().normalized();
            (
                record.capacity_trimmed_event_count,
                record.capacity_trimmed_through_seq,
                record.last_capacity_trimmed_at,
            )
        });
        let restored_capacity_trim_metadata = restored.as_ref().map(|record| {
            (
                record.capacity_trimmed_event_count,
                record.capacity_trimmed_through_seq,
                record.last_capacity_trimmed_at.clone(),
            )
        });
        let normalized_restored = restored.as_ref().map(|record| {
            normalize_checkpoint_fields(
                record.latest_realtime_seq,
                record.acked_through_seq,
                record.trimmed_through_seq,
            )
        });
        let checkpoint_needs_normalization = restored.as_ref().is_some_and(|record| {
            normalized_restored
                .map(|(latest, acked, trimmed)| {
                    latest != record.latest_realtime_seq
                        || acked != record.acked_through_seq
                        || trimmed != record.trimmed_through_seq
                })
                .unwrap_or(false)
        });

        if checkpoint_needs_normalization
            && let Some((latest_realtime_seq, acked_through_seq, trimmed_through_seq)) =
                normalized_restored
        {
            self.persist_checkpoint_records(vec![checkpoint_record_from_sequences(
                RealtimeCheckpointRecordParts {
                    tenant_id,
                    organization_id,
                    principal_id,
                    principal_kind,
                    device_id,
                    latest_realtime_seq,
                    acked_through_seq,
                    trimmed_through_seq,
                    capacity_trimmed_event_count: restored_capacity_trim_metadata
                        .as_ref()
                        .map(|item| item.0)
                        .unwrap_or_else(|| {
                            restored_window_metadata
                                .as_ref()
                                .map(|item| item.0)
                                .unwrap_or(0)
                        }),
                    capacity_trimmed_through_seq: restored_capacity_trim_metadata
                        .as_ref()
                        .map(|item| item.1)
                        .unwrap_or_else(|| {
                            restored_window_metadata
                                .as_ref()
                                .map(|item| item.1)
                                .unwrap_or(0)
                        }),
                    last_capacity_trimmed_at: restored_capacity_trim_metadata
                        .as_ref()
                        .and_then(|item| item.2.clone())
                        .or_else(|| {
                            restored_window_metadata
                                .as_ref()
                                .and_then(|item| item.2.clone())
                        }),
                },
            )])?;
        }

        let restored_window_events = restored_window
            .map(|record| {
                record
                    .normalized()
                    .events
                    .into_iter()
                    .map(|event| (event.realtime_seq, event))
                    .collect::<BTreeMap<_, _>>()
            })
            .unwrap_or_default();
        lock_realtime_mutex(&self.windows, "realtime window store")
            .entry(scope_key.clone())
            .or_insert(restored_window_events);
        if let Some(restored_subscriptions) =
            restored_subscriptions.filter(|record| !record.items.is_empty())
        {
            let restored_subscriptions =
                RealtimeClientRouteSubscriptions::from_items(restored_subscriptions.items);
            let inserted = {
                let mut subscriptions =
                    lock_realtime_mutex(&self.subscriptions, "realtime subscription store");
                if subscriptions.contains_key(scope_key.as_str()) {
                    false
                } else {
                    subscriptions.insert(scope_key.clone(), restored_subscriptions.clone());
                    true
                }
            };
            if inserted {
                self.index_client_route_subscriptions(
                    tenant_id,
                    organization_id,
                    principal_kind,
                    principal_id,
                    device_id,
                    &restored_subscriptions,
                );
            }
        }
        let latest_seq = {
            let mut latest_sequences =
                lock_realtime_mutex(&self.latest_sequences, "realtime sequence store");
            *latest_sequences
                .entry(scope_key.clone())
                .or_insert_with(|| normalized_restored.map(|item| item.0).unwrap_or(0))
        };
        lock_realtime_mutex(&self.acked_sequences, "realtime ack store")
            .entry(scope_key.clone())
            .or_insert_with(|| normalized_restored.map(|item| item.1).unwrap_or(0));
        lock_realtime_mutex(&self.trimmed_sequences, "realtime trim store")
            .entry(scope_key.clone())
            .or_insert_with(|| normalized_restored.map(|item| item.2).unwrap_or(0));
        lock_realtime_mutex(
            &self.capacity_trimmed_event_counts,
            "realtime capacity trim count store",
        )
        .entry(scope_key.clone())
        .or_insert_with(|| {
            restored_window_metadata
                .as_ref()
                .map(|item| item.0)
                .unwrap_or(0)
        });
        lock_realtime_mutex(
            &self.capacity_trimmed_sequences,
            "realtime capacity trim sequence store",
        )
        .entry(scope_key.clone())
        .or_insert_with(|| {
            restored_window_metadata
                .as_ref()
                .map(|item| item.1)
                .unwrap_or(0)
        });
        if let Some(last_trimmed_at) =
            restored_window_metadata.and_then(|(_, _, last_trimmed_at)| last_trimmed_at)
        {
            lock_realtime_mutex(
                &self.last_capacity_trimmed_at,
                "realtime capacity trim timestamp store",
            )
            .entry(scope_key.clone())
            .or_insert(last_trimmed_at);
        }
        lock_realtime_mutex(&self.notifiers, "realtime notifier store")
            .entry(scope_key.clone())
            .or_insert_with(|| {
                let (sender, _) = watch::channel(latest_seq);
                sender
            });
        let disconnect_generation = {
            let mut disconnect_generations = lock_realtime_mutex(
                &self.disconnect_generations,
                "realtime disconnect generation store",
            );
            *disconnect_generations.entry(scope_key.clone()).or_insert(0)
        };
        lock_realtime_mutex(
            &self.disconnect_notifiers,
            "realtime disconnect notifier store",
        )
        .entry(scope_key)
        .or_insert_with(|| {
            let (sender, _) = watch::channel(disconnect_generation);
            sender
        });

        Ok(())
    }

    pub fn subscribe_client_route_for_principal_kind(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_id: &str,
        principal_kind: &str,
        device_id: &str,
    ) -> Result<watch::Receiver<u64>, RealtimeRuntimeError> {
        self.subscribe_device_internal(tenant_id, organization_id, principal_id, principal_kind, device_id)
    }

    fn subscribe_device_internal(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_id: &str,
        principal_kind: &str,
        device_id: &str,
    ) -> Result<watch::Receiver<u64>, RealtimeRuntimeError> {
        self.ensure_client_route_state_internal(
            tenant_id,
            organization_id,
            principal_id,
            principal_kind,
            device_id,
        )?;
        let scope_key = client_route_scope_key(tenant_id, organization_id, principal_id, principal_kind, device_id);
        let sender = lock_realtime_mutex(&self.notifiers, "realtime notifier store")
            .entry(scope_key)
            .or_insert_with(|| {
                tracing::warn!("realtime notifier missing after ensure; reconstructing sender");
                let (sender, _) = watch::channel(0);
                sender
            })
            .clone();
        Ok(sender.subscribe())
    }

    pub fn subscribe_disconnect_signal_for_principal_kind(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_id: &str,
        principal_kind: &str,
        device_id: &str,
    ) -> Result<watch::Receiver<u64>, RealtimeRuntimeError> {
        self.subscribe_disconnect_signal_internal(
            tenant_id,
            organization_id,
            principal_id,
            principal_kind,
            device_id,
        )
    }

    fn subscribe_disconnect_signal_internal(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_id: &str,
        principal_kind: &str,
        device_id: &str,
    ) -> Result<watch::Receiver<u64>, RealtimeRuntimeError> {
        self.ensure_client_route_state_internal(
            tenant_id,
            organization_id,
            principal_id,
            principal_kind,
            device_id,
        )?;
        let scope_key = client_route_scope_key(tenant_id, organization_id, principal_id, principal_kind, device_id);
        let sender = lock_realtime_mutex(
            &self.disconnect_notifiers,
            "realtime disconnect notifier store",
        )
        .entry(scope_key)
        .or_insert_with(|| {
            tracing::warn!(
                "realtime disconnect notifier missing after ensure; reconstructing sender"
            );
            let (sender, _) = watch::channel(0);
            sender
        })
        .clone();
        Ok(sender.subscribe())
    }

    pub fn disconnect_generation_for_principal_kind(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_id: &str,
        principal_kind: &str,
        device_id: &str,
    ) -> Result<u64, RealtimeRuntimeError> {
        self.disconnect_generation_internal(tenant_id, organization_id, principal_id, principal_kind, device_id)
    }

    fn disconnect_generation_internal(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_id: &str,
        principal_kind: &str,
        device_id: &str,
    ) -> Result<u64, RealtimeRuntimeError> {
        self.ensure_client_route_state_internal(
            tenant_id,
            organization_id,
            principal_id,
            principal_kind,
            device_id,
        )?;
        Ok(lock_realtime_mutex(
            &self.disconnect_generations,
            "realtime disconnect generation store",
        )
        .get(client_route_scope_key(tenant_id, organization_id, principal_id, principal_kind, device_id).as_str())
        .copied()
        .unwrap_or(0))
    }

    pub fn signal_device_disconnect_for_principal_kind(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_id: &str,
        principal_kind: &str,
        device_id: &str,
    ) -> Result<(), RealtimeRuntimeError> {
        self.signal_device_disconnect_internal(tenant_id, organization_id, principal_id, principal_kind, device_id)
    }

    fn signal_device_disconnect_internal(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_id: &str,
        principal_kind: &str,
        device_id: &str,
    ) -> Result<(), RealtimeRuntimeError> {
        self.ensure_client_route_state_internal(
            tenant_id,
            organization_id,
            principal_id,
            principal_kind,
            device_id,
        )?;
        let scope_key = client_route_scope_key(tenant_id, organization_id, principal_id, principal_kind, device_id);
        let next_generation = {
            let mut disconnect_generations = lock_realtime_mutex(
                &self.disconnect_generations,
                "realtime disconnect generation store",
            );
            let entry = disconnect_generations.entry(scope_key.clone()).or_insert(0);
            *entry += 1;
            *entry
        };
        if let Some(sender) = lock_realtime_mutex(
            &self.disconnect_notifiers,
            "realtime disconnect notifier store",
        )
        .get(scope_key.as_str())
        .cloned()
        {
            let _ = sender.send(next_generation);
        }

        Ok(())
    }

    pub fn window_checkpoint_for_principal_kind(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_id: &str,
        principal_kind: &str,
        device_id: &str,
    ) -> Result<RealtimeWindowCheckpoint, RealtimeRuntimeError> {
        self.window_checkpoint_internal(tenant_id, organization_id, principal_id, principal_kind, device_id)
    }

    fn window_checkpoint_internal(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_id: &str,
        principal_kind: &str,
        device_id: &str,
    ) -> Result<RealtimeWindowCheckpoint, RealtimeRuntimeError> {
        self.ensure_client_route_state_internal(
            tenant_id,
            organization_id,
            principal_id,
            principal_kind,
            device_id,
        )?;
        let scope_key = client_route_scope_key(tenant_id, organization_id, principal_id, principal_kind, device_id);
        let latest_realtime_seq =
            lock_realtime_mutex(&self.latest_sequences, "realtime sequence store")
                .get(scope_key.as_str())
                .copied()
                .unwrap_or(0);
        let acked_through_seq = lock_realtime_mutex(&self.acked_sequences, "realtime ack store")
            .get(scope_key.as_str())
            .copied()
            .unwrap_or(0);
        let trimmed_through_seq =
            lock_realtime_mutex(&self.trimmed_sequences, "realtime trim store")
                .get(scope_key.as_str())
                .copied()
                .unwrap_or(0);
        let (latest_realtime_seq, acked_through_seq, trimmed_through_seq) =
            normalize_checkpoint_fields(
                latest_realtime_seq,
                acked_through_seq,
                trimmed_through_seq,
            );
        Ok(RealtimeWindowCheckpoint {
            latest_realtime_seq,
            acked_through_seq,
            trimmed_through_seq,
        })
    }

    pub fn sync_subscriptions_for_principal_kind(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_id: &str,
        principal_kind: &str,
        device_id: &str,
        items: Vec<RealtimeSubscriptionItemInput>,
    ) -> Result<RealtimeSubscriptionSnapshot, RealtimeRuntimeError> {
        self.sync_subscriptions_internal(
            tenant_id,
            organization_id,
            principal_id,
            principal_kind,
            device_id,
            items,
        )
    }

    pub fn validate_subscriptions_for_principal_kind(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_id: &str,
        principal_kind: &str,
        items: &[RealtimeSubscriptionItemInput],
    ) -> Result<(), RealtimeRuntimeError> {
        self.validate_subscriptions_internal(
            tenant_id,
            organization_id,
            principal_id,
            principal_kind,
            items,
        )
    }

    fn sync_subscriptions_internal(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_id: &str,
        principal_kind: &str,
        device_id: &str,
        items: Vec<RealtimeSubscriptionItemInput>,
    ) -> Result<RealtimeSubscriptionSnapshot, RealtimeRuntimeError> {
        let mutation_keys = [realtime_mutation_principal_key(
            tenant_id,
            principal_kind,
            principal_id,
        )];
        let mutation_scope = RealtimeMutationScopeGuards::new(self, &mutation_keys);
        let _mutation_guards = mutation_scope.lock();
        self.validate_subscriptions_internal(tenant_id, organization_id, principal_id, principal_kind, &items)?;
        let synced_at = realtime_timestamp();
        lock_realtime_mutex(
            &self.migrated_out_client_route_scopes,
            "realtime migrated-out device scope store",
        )
        .remove(
            client_route_scope_key(tenant_id, organization_id, principal_id, principal_kind, device_id).as_str(),
        );
        self.ensure_client_route_state_internal(
            tenant_id,
            organization_id,
            principal_id,
            principal_kind,
            device_id,
        )?;
        let subscriptions = items
            .into_iter()
            .map(|item| RealtimeSubscription {
                scope_type: item.scope_type,
                scope_id: item.scope_id,
                event_types: item.event_types,
                subscribed_at: synced_at.clone(),
            })
            .collect::<Vec<_>>();
        let scope_key = client_route_scope_key(tenant_id, organization_id, principal_id, principal_kind, device_id);
        let previous_subscriptions =
            lock_realtime_mutex(&self.subscriptions, "realtime subscription store")
                .get(scope_key.as_str())
                .cloned();

        let client_route_subscriptions =
            RealtimeClientRouteSubscriptions::from_items(subscriptions.clone());
        self.remove_device_subscription_index(
            tenant_id,
            organization_id,
            principal_kind,
            principal_id,
            device_id,
        );
        lock_realtime_mutex(&self.subscriptions, "realtime subscription store")
            .insert(scope_key.clone(), client_route_subscriptions.clone());
        self.index_client_route_subscriptions(
            tenant_id,
            organization_id,
            principal_kind,
            principal_id,
            device_id,
            &client_route_subscriptions,
        );
        if let Err(error) =
            self.persist_subscriptions_internal(tenant_id, organization_id, principal_id, principal_kind, device_id)
        {
            self.remove_device_subscription_index(
                tenant_id,
                organization_id,
                principal_kind,
                principal_id,
                device_id,
            );
            if let Some(previous_subscriptions) = previous_subscriptions {
                lock_realtime_mutex(&self.subscriptions, "realtime subscription store")
                    .insert(scope_key, previous_subscriptions.clone());
                self.index_client_route_subscriptions(
                    tenant_id,
                    organization_id,
                    principal_kind,
                    principal_id,
                    device_id,
                    &previous_subscriptions,
                );
            } else {
                lock_realtime_mutex(&self.subscriptions, "realtime subscription store")
                    .remove(scope_key.as_str());
            }
            return Err(error);
        }

        Ok(RealtimeSubscriptionSnapshot {
            tenant_id: tenant_id.into(),
            principal_id: principal_id.into(),
            device_id: device_id.into(),
            items: subscriptions,
            synced_at,
        })
    }

    fn validate_subscriptions_internal(
        &self,
        tenant_id: &str,
        _organization_id: &str,
        principal_id: &str,
        principal_kind: &str,
        items: &[RealtimeSubscriptionItemInput],
    ) -> Result<(), RealtimeRuntimeError> {
        validate_realtime_subscription_items(items)?;
        for item in items {
            self.scope_access_policy.validate_subscription_scope(
                tenant_id,
                principal_id,
                principal_kind,
                item.scope_type.as_str(),
                item.scope_id.as_str(),
            )?;
        }
        Ok(())
    }

    pub fn clear_client_route_subscriptions_for_principal_kind(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_id: &str,
        principal_kind: &str,
        device_id: &str,
    ) -> Result<(), RealtimeRuntimeError> {
        self.clear_client_route_subscriptions_internal(
            tenant_id,
            organization_id,
            principal_id,
            principal_kind,
            device_id,
        )
    }

    fn clear_client_route_subscriptions_internal(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_id: &str,
        principal_kind: &str,
        device_id: &str,
    ) -> Result<(), RealtimeRuntimeError> {
        let mutation_keys = [realtime_mutation_principal_key(
            tenant_id,
            principal_kind,
            principal_id,
        )];
        let mutation_scope = RealtimeMutationScopeGuards::new(self, &mutation_keys);
        let _mutation_guards = mutation_scope.lock();
        let scope_key = client_route_scope_key(tenant_id, organization_id, principal_id, principal_kind, device_id);
        let removed = lock_realtime_mutex(&self.subscriptions, "realtime subscription store")
            .remove(scope_key.as_str());
        if removed.is_some() {
            self.remove_device_subscription_index(
                tenant_id,
                organization_id,
                principal_kind,
                principal_id,
                device_id,
            );
        }
        let cutoff_synced_at = removed
            .as_ref()
            .map(|subscriptions| subscriptions_synced_at(subscriptions.ordered_items().as_slice()))
            .unwrap_or_else(realtime_timestamp);
        match self.clear_persisted_subscriptions_synced_at_or_before(
            tenant_id,
            organization_id,
            principal_id,
            principal_kind,
            device_id,
            cutoff_synced_at.as_str(),
        ) {
            Ok(_) => Ok(()),
            Err(error) => {
                if let Some(removed) = removed.as_ref() {
                    lock_realtime_mutex(&self.subscriptions, "realtime subscription store")
                        .insert(scope_key, removed.clone());
                    self.index_client_route_subscriptions(
                        tenant_id,
                        organization_id,
                        principal_kind,
                        principal_id,
                        device_id,
                        removed,
                    );
                }
                Err(error)
            }
        }
    }

    pub fn list_events_for_principal_kind(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_id: &str,
        principal_kind: &str,
        device_id: &str,
        after_seq: u64,
        limit: usize,
    ) -> Result<RealtimeEventWindow, RealtimeRuntimeError> {
        self.list_events_internal(
            tenant_id,
            organization_id,
            principal_id,
            principal_kind,
            device_id,
            after_seq,
            limit,
        )
    }

    fn list_events_internal(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_id: &str,
        principal_kind: &str,
        device_id: &str,
        after_seq: u64,
        limit: usize,
    ) -> Result<RealtimeEventWindow, RealtimeRuntimeError> {
        validate_realtime_event_limit(limit)?;
        self.ensure_client_route_state_internal(
            tenant_id,
            organization_id,
            principal_id,
            principal_kind,
            device_id,
        )?;
        let scope_key = client_route_scope_key(tenant_id, organization_id, principal_id, principal_kind, device_id);
        let acked_through_seq = lock_realtime_mutex(&self.acked_sequences, "realtime ack store")
            .get(scope_key.as_str())
            .copied()
            .unwrap_or(0);
        let trimmed_through_seq =
            lock_realtime_mutex(&self.trimmed_sequences, "realtime trim store")
                .get(scope_key.as_str())
                .copied()
                .unwrap_or(0);
        let windows = lock_realtime_mutex(&self.windows, "realtime window store");
        let mut has_more = false;
        let mut last_examined_seq = None;
        let effective_after_seq = after_seq.max(trimmed_through_seq);
        let items = windows
            .get(scope_key.as_str())
            .map(|scope_events| {
                let mut visible = Vec::new();
                for event in scope_events
                    .range((Excluded(effective_after_seq), Unbounded))
                    .map(|(_, event)| event)
                {
                    last_examined_seq = Some(event.realtime_seq);
                    if !self.scope_access_policy.is_event_visible(
                        tenant_id,
                        principal_id,
                        principal_kind,
                        event,
                    ) {
                        continue;
                    }
                    if visible.len() < limit {
                        visible.push(event.clone());
                        continue;
                    }
                    has_more = true;
                    break;
                }
                visible
            })
            .unwrap_or_default();
        let next_after_seq = if has_more {
            items.last().map(|item| item.realtime_seq)
        } else {
            last_examined_seq
        };

        Ok(RealtimeEventWindow {
            device_id: device_id.into(),
            items,
            next_after_seq,
            has_more,
            acked_through_seq,
            trimmed_through_seq,
        })
    }

    pub fn ack_events_for_principal_kind(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_id: &str,
        principal_kind: &str,
        device_id: &str,
        acked_seq: u64,
    ) -> Result<RealtimeAckState, RealtimeRuntimeError> {
        self.ack_events_internal(
            tenant_id,
            organization_id,
            principal_id,
            principal_kind,
            device_id,
            acked_seq,
        )
    }

    fn ack_events_internal(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_id: &str,
        principal_kind: &str,
        device_id: &str,
        acked_seq: u64,
    ) -> Result<RealtimeAckState, RealtimeRuntimeError> {
        let mutation_keys = [realtime_mutation_principal_key(
            tenant_id,
            principal_kind,
            principal_id,
        )];
        let mutation_scope = RealtimeMutationScopeGuards::new(self, &mutation_keys);
        let _mutation_guards = mutation_scope.lock();
        self.ensure_client_route_state_internal(
            tenant_id,
            organization_id,
            principal_id,
            principal_kind,
            device_id,
        )?;
        let scope_key = client_route_scope_key(tenant_id, organization_id, principal_id, principal_kind, device_id);
        let latest_seq = lock_realtime_mutex(&self.latest_sequences, "realtime sequence store")
            .get(scope_key.as_str())
            .copied()
            .unwrap_or(0);
        let effective_ack = acked_seq.min(latest_seq);
        let previous_acked_through_seq =
            lock_realtime_mutex(&self.acked_sequences, "realtime ack store")
                .get(scope_key.as_str())
                .copied()
                .unwrap_or(0);
        let acked_through_seq = previous_acked_through_seq.max(effective_ack);
        let previous_trimmed_through_seq =
            lock_realtime_mutex(&self.trimmed_sequences, "realtime trim store")
                .get(scope_key.as_str())
                .copied()
                .unwrap_or(0);
        let trimmed_through_seq = previous_trimmed_through_seq.max(acked_through_seq);
        let capacity_trimmed_event_count = lock_realtime_mutex(
            &self.capacity_trimmed_event_counts,
            "realtime capacity trim count store",
        )
        .get(scope_key.as_str())
        .copied()
        .unwrap_or(0);
        let capacity_trimmed_through_seq = lock_realtime_mutex(
            &self.capacity_trimmed_sequences,
            "realtime capacity trim sequence store",
        )
        .get(scope_key.as_str())
        .copied()
        .unwrap_or(0);
        let last_capacity_trimmed_at = lock_realtime_mutex(
            &self.last_capacity_trimmed_at,
            "realtime capacity trim timestamp store",
        )
        .get(scope_key.as_str())
        .cloned();
        let (next_window, retained_event_count) = {
            let windows = lock_realtime_mutex(&self.windows, "realtime window store");
            let mut next_window = windows.get(scope_key.as_str()).cloned().unwrap_or_default();
            next_window.retain(|seq, _| *seq > acked_through_seq);
            let retained_event_count = next_window.len();
            (next_window, retained_event_count)
        };
        let previous_event_window = self
            .event_window_store
            .load_window(tenant_id, organization_id, principal_kind, principal_id, device_id)
            .map_err(RealtimeRuntimeError::event_window_store)?;
        let rollback_event_window = event_window_record_or_empty(
            previous_event_window,
            tenant_id,
            organization_id,
            principal_id,
            principal_kind,
            device_id,
        );
        self.event_window_store
            .trim_window(
                tenant_id,
                organization_id,
                principal_kind,
                principal_id,
                device_id,
                acked_through_seq,
            )
            .map_err(RealtimeRuntimeError::event_window_store)?;
        if let Err(error) = self.persist_checkpoint_records(vec![checkpoint_record_from_sequences(
            RealtimeCheckpointRecordParts {
                tenant_id,
                organization_id,
                principal_id,
                principal_kind,
                device_id,
                latest_realtime_seq: latest_seq,
                acked_through_seq,
                trimmed_through_seq,
                capacity_trimmed_event_count,
                capacity_trimmed_through_seq,
                last_capacity_trimmed_at,
            },
        )]) {
            return self.fail_with_event_window_rollback(
                vec![rollback_event_window],
                error,
                "ack checkpoint persist failed",
            );
        }
        lock_realtime_mutex(&self.acked_sequences, "realtime ack store")
            .insert(scope_key.clone(), acked_through_seq);
        lock_realtime_mutex(&self.trimmed_sequences, "realtime trim store")
            .insert(scope_key.clone(), trimmed_through_seq);
        lock_realtime_mutex(&self.windows, "realtime window store").insert(scope_key, next_window);

        Ok(RealtimeAckState {
            tenant_id: tenant_id.into(),
            principal_id: principal_id.into(),
            device_id: device_id.into(),
            acked_through_seq,
            trimmed_through_seq,
            retained_event_count,
            acked_at: realtime_timestamp(),
        })
    }

    pub fn take_client_route_state_for_principal_kind(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_id: &str,
        principal_kind: &str,
        device_id: &str,
    ) -> Result<RealtimeClientRouteStateSnapshot, RealtimeRuntimeError> {
        self.take_client_route_state_internal(tenant_id, organization_id, principal_id, principal_kind, device_id)
    }

    /// Drop in-memory realtime state for a disconnected client route.
    ///
    /// Unlike `take_client_route_state_for_principal_kind` (used for cross-node migration),
    /// this method does not return a snapshot and does not mark the scope as migrated-out.
    /// It is called when a WebSocket disconnects to prevent unbounded memory growth from
    /// stale per-client HashMap entries.
    pub fn drop_client_route_state(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_id: &str,
        principal_kind: &str,
        device_id: &str,
    ) -> Result<(), RealtimeRuntimeError> {
        let mutation_keys = [realtime_mutation_principal_key(
            tenant_id,
            principal_kind,
            principal_id,
        )];
        let mutation_scope = RealtimeMutationScopeGuards::new(self, &mutation_keys);
        let _mutation_guards = mutation_scope.lock();

        let scope_key = client_route_scope_key(
            tenant_id,
            organization_id,
            principal_id,
            principal_kind,
            device_id,
        );

        // Clear in-memory subscription state.
        lock_realtime_mutex(&self.subscriptions, "realtime subscription store")
            .remove(scope_key.as_str());
        self.remove_device_subscription_index(
            tenant_id,
            organization_id,
            principal_kind,
            principal_id,
            device_id,
        );

        // Clear in-memory event windows and sequence trackers.
        lock_realtime_mutex(&self.windows, "realtime window store").remove(scope_key.as_str());
        lock_realtime_mutex(&self.latest_sequences, "realtime sequence store")
            .remove(scope_key.as_str());
        lock_realtime_mutex(&self.acked_sequences, "realtime ack store")
            .remove(scope_key.as_str());
        lock_realtime_mutex(&self.trimmed_sequences, "realtime trim store")
            .remove(scope_key.as_str());
        lock_realtime_mutex(
            &self.capacity_trimmed_event_counts,
            "realtime capacity trim count store",
        )
        .remove(scope_key.as_str());
        lock_realtime_mutex(
            &self.capacity_trimmed_sequences,
            "realtime capacity trim sequence store",
        )
        .remove(scope_key.as_str());
        lock_realtime_mutex(
            &self.last_capacity_trimmed_at,
            "realtime capacity trim timestamp store",
        )
        .remove(scope_key.as_str());

        // Drop watchers so receivers can observe termination.
        lock_realtime_mutex(&self.notifiers, "realtime notifier store").remove(scope_key.as_str());
        lock_realtime_mutex(
            &self.disconnect_notifiers,
            "realtime disconnect notifier store",
        )
        .remove(scope_key.as_str());
        lock_realtime_mutex(
            &self.disconnect_generations,
            "realtime disconnect generation store",
        )
        .remove(scope_key.as_str());

        // Clear persisted event window and subscription state for this route.
        self.event_window_store
            .clear_window(tenant_id, organization_id, principal_kind, principal_id, device_id)
            .map_err(RealtimeRuntimeError::event_window_store)?;
        self.subscription_store
            .clear_subscriptions(tenant_id, organization_id, principal_kind, principal_id, device_id)
            .map_err(RealtimeRuntimeError::subscription_store)?;

        Ok(())
    }

    fn take_client_route_state_internal(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_id: &str,
        principal_kind: &str,
        device_id: &str,
    ) -> Result<RealtimeClientRouteStateSnapshot, RealtimeRuntimeError> {
        let mutation_keys = [realtime_mutation_principal_key(
            tenant_id,
            principal_kind,
            principal_id,
        )];
        let mutation_scope = RealtimeMutationScopeGuards::new(self, &mutation_keys);
        let _mutation_guards = mutation_scope.lock();
        self.ensure_client_route_state_internal(
            tenant_id,
            organization_id,
            principal_id,
            principal_kind,
            device_id,
        )?;
        let scope_key = client_route_scope_key(tenant_id, organization_id, principal_id, principal_kind, device_id);
        let subscriptions = lock_realtime_mutex(&self.subscriptions, "realtime subscription store")
            .remove(scope_key.as_str())
            .map(|subscriptions| subscriptions.ordered_items())
            .unwrap_or_default();
        self.remove_device_subscription_index(
            tenant_id,
            organization_id,
            principal_kind,
            principal_id,
            device_id,
        );
        let events = lock_realtime_mutex(&self.windows, "realtime window store")
            .remove(scope_key.as_str())
            .unwrap_or_default()
            .into_values()
            .collect();
        self.event_window_store
            .clear_window(tenant_id, organization_id, principal_kind, principal_id, device_id)
            .map_err(RealtimeRuntimeError::event_window_store)?;
        self.subscription_store
            .clear_subscriptions(tenant_id, organization_id, principal_kind, principal_id, device_id)
            .map_err(RealtimeRuntimeError::subscription_store)?;
        let latest_realtime_seq =
            lock_realtime_mutex(&self.latest_sequences, "realtime sequence store")
                .remove(scope_key.as_str())
                .unwrap_or(0);
        let acked_through_seq = lock_realtime_mutex(&self.acked_sequences, "realtime ack store")
            .remove(scope_key.as_str())
            .unwrap_or(0);
        let trimmed_through_seq =
            lock_realtime_mutex(&self.trimmed_sequences, "realtime trim store")
                .remove(scope_key.as_str())
                .unwrap_or(0);
        let capacity_trimmed_event_count = lock_realtime_mutex(
            &self.capacity_trimmed_event_counts,
            "realtime capacity trim count store",
        )
        .remove(scope_key.as_str())
        .unwrap_or(0);
        let capacity_trimmed_through_seq = lock_realtime_mutex(
            &self.capacity_trimmed_sequences,
            "realtime capacity trim sequence store",
        )
        .remove(scope_key.as_str())
        .unwrap_or(0);
        let last_capacity_trimmed_at = lock_realtime_mutex(
            &self.last_capacity_trimmed_at,
            "realtime capacity trim timestamp store",
        )
        .remove(scope_key.as_str());
        lock_realtime_mutex(&self.notifiers, "realtime notifier store").remove(scope_key.as_str());
        let disconnect_generation = lock_realtime_mutex(
            &self.disconnect_generations,
            "realtime disconnect generation store",
        )
        .remove(scope_key.as_str())
        .unwrap_or(0);
        lock_realtime_mutex(
            &self.disconnect_notifiers,
            "realtime disconnect notifier store",
        )
        .remove(scope_key.as_str());
        lock_realtime_mutex(
            &self.migrated_out_client_route_scopes,
            "realtime migrated-out device scope store",
        )
        .insert(scope_key.clone());

        Ok(RealtimeClientRouteStateSnapshot {
            tenant_id: tenant_id.into(),
            organization_id: organization_id.into(),
            principal_kind: principal_kind.into(),
            principal_id: principal_id.into(),
            device_id: device_id.into(),
            subscriptions,
            events,
            latest_realtime_seq,
            acked_through_seq,
            trimmed_through_seq,
            capacity_trimmed_event_count,
            capacity_trimmed_through_seq,
            last_capacity_trimmed_at,
            disconnect_generation,
        })
    }

    pub fn restore_client_route_state(
        &self,
        snapshot: RealtimeClientRouteStateSnapshot,
    ) -> Result<(), RealtimeRuntimeError> {
        let mutation_keys = [realtime_mutation_principal_key(
            snapshot.tenant_id.as_str(),
            snapshot.principal_kind.as_str(),
            snapshot.principal_id.as_str(),
        )];
        let mutation_scope = RealtimeMutationScopeGuards::new(self, &mutation_keys);
        let _mutation_guards = mutation_scope.lock();
        let latest_realtime_seq = snapshot
            .events
            .iter()
            .map(|event| event.realtime_seq)
            .max()
            .unwrap_or(snapshot.latest_realtime_seq)
            .max(snapshot.latest_realtime_seq);
        let (latest_realtime_seq, acked_through_seq, trimmed_through_seq) =
            normalize_checkpoint_fields(
                latest_realtime_seq,
                snapshot.acked_through_seq,
                snapshot.trimmed_through_seq,
            );
        let normalized_events = normalize_window_events(snapshot.events, trimmed_through_seq);
        let (
            normalized_events,
            trimmed_through_seq,
            capacity_trimmed_event_count,
            capacity_trimmed_through_seq,
            last_capacity_trimmed_at,
        ) = retain_bounded_window_events(
            normalized_events,
            trimmed_through_seq,
            snapshot.capacity_trimmed_event_count,
            snapshot.capacity_trimmed_through_seq,
            snapshot.last_capacity_trimmed_at,
        );
        let normalized_events_for_store = normalized_events.values().cloned().collect::<Vec<_>>();
        let scope_key = client_route_scope_key(
            snapshot.tenant_id.as_str(),
            snapshot.organization_id.as_str(),
            snapshot.principal_id.as_str(),
            snapshot.principal_kind.as_str(),
            snapshot.device_id.as_str(),
        );
        lock_realtime_mutex(
            &self.migrated_out_client_route_scopes,
            "realtime migrated-out device scope store",
        )
        .remove(scope_key.as_str());
        let client_route_subscriptions =
            RealtimeClientRouteSubscriptions::from_items(snapshot.subscriptions);
        let subscription_items = client_route_subscriptions.ordered_items();
        let subscription_record = subscription_record_from_items(
            snapshot.tenant_id.as_str(),
            snapshot.organization_id.as_str(),
            snapshot.principal_id.as_str(),
            snapshot.principal_kind.as_str(),
            snapshot.device_id.as_str(),
            subscription_items.clone(),
        );
        let previous_subscription_record = self
            .subscription_store
            .load_subscriptions(
                snapshot.tenant_id.as_str(),
                snapshot.organization_id.as_str(),
                snapshot.principal_kind.as_str(),
                snapshot.principal_id.as_str(),
                snapshot.device_id.as_str(),
            )
            .map_err(RealtimeRuntimeError::subscription_store)?;
        match subscription_record.as_ref() {
            Some(record) => self
                .subscription_store
                .save_subscriptions(record.clone())
                .map_err(RealtimeRuntimeError::subscription_store)?,
            None => {
                self.subscription_store
                    .clear_subscriptions(
                        snapshot.tenant_id.as_str(),
                        snapshot.organization_id.as_str(),
                        snapshot.principal_kind.as_str(),
                        snapshot.principal_id.as_str(),
                        snapshot.device_id.as_str(),
                    )
                    .map_err(RealtimeRuntimeError::subscription_store)?;
            }
        }
        let checkpoint = checkpoint_record_from_sequences(RealtimeCheckpointRecordParts {
            tenant_id: snapshot.tenant_id.as_str(),
            organization_id: snapshot.organization_id.as_str(),
            principal_id: snapshot.principal_id.as_str(),
            principal_kind: snapshot.principal_kind.as_str(),
            device_id: snapshot.device_id.as_str(),
            latest_realtime_seq,
            acked_through_seq,
            trimmed_through_seq,
            capacity_trimmed_event_count,
            capacity_trimmed_through_seq,
            last_capacity_trimmed_at: last_capacity_trimmed_at.clone(),
        });
        let previous_event_window = self
            .event_window_store
            .load_window(
                snapshot.tenant_id.as_str(),
                snapshot.organization_id.as_str(),
                snapshot.principal_kind.as_str(),
                snapshot.principal_id.as_str(),
                snapshot.device_id.as_str(),
            )
            .map_err(RealtimeRuntimeError::event_window_store)?;
        let rollback_event_window = event_window_record_or_empty(
            previous_event_window,
            snapshot.tenant_id.as_str(),
            snapshot.organization_id.as_str(),
            snapshot.principal_id.as_str(),
            snapshot.principal_kind.as_str(),
            snapshot.device_id.as_str(),
        );
        if let Err(error) = self.persist_checkpoint_records(vec![checkpoint]) {
            if let Err(compensation_error) = self
                .restore_persisted_subscriptions_after_failed_restore(
                    previous_subscription_record,
                    subscription_record.as_ref(),
                    snapshot.tenant_id.as_str(),
                    snapshot.organization_id.as_str(),
                    snapshot.principal_id.as_str(),
                    snapshot.principal_kind.as_str(),
                    snapshot.device_id.as_str(),
                )
            {
                return Err(RealtimeRuntimeError {
                    code: "realtime_state_compensation_failed",
                    message: format!(
                        "checkpoint persist failed: {}; subscription compensation failed: {}",
                        error.message, compensation_error.message
                    ),
                });
            }
            return Err(error);
        }
        if let Err(error) = self
            .event_window_store
            .save_window(realtime_event_window_record_from_events(
                RealtimeEventWindowRecordParts {
                    tenant_id: snapshot.tenant_id.as_str(),
                    organization_id: snapshot.organization_id.as_str(),
                    principal_id: snapshot.principal_id.as_str(),
                    principal_kind: snapshot.principal_kind.as_str(),
                    device_id: snapshot.device_id.as_str(),
                    events: normalized_events_for_store,
                    trimmed_through_seq,
                    capacity_trimmed_event_count,
                    capacity_trimmed_through_seq,
                    last_capacity_trimmed_at: last_capacity_trimmed_at.clone(),
                },
            ))
            .map_err(RealtimeRuntimeError::event_window_store)
        {
            return self.fail_with_event_window_rollback(
                vec![rollback_event_window],
                error,
                "restore event window persist failed",
            );
        }

        self.remove_device_subscription_index(
            snapshot.tenant_id.as_str(),
            snapshot.organization_id.as_str(),
            snapshot.principal_kind.as_str(),
            snapshot.principal_id.as_str(),
            snapshot.device_id.as_str(),
        );
        lock_realtime_mutex(&self.subscriptions, "realtime subscription store")
            .insert(scope_key.clone(), client_route_subscriptions.clone());
        self.index_client_route_subscriptions(
            snapshot.tenant_id.as_str(),
            snapshot.organization_id.as_str(),
            snapshot.principal_kind.as_str(),
            snapshot.principal_id.as_str(),
            snapshot.device_id.as_str(),
            &client_route_subscriptions,
        );
        lock_realtime_mutex(&self.windows, "realtime window store")
            .insert(scope_key.clone(), normalized_events);
        lock_realtime_mutex(&self.latest_sequences, "realtime sequence store")
            .insert(scope_key.clone(), latest_realtime_seq);
        lock_realtime_mutex(&self.acked_sequences, "realtime ack store")
            .insert(scope_key.clone(), acked_through_seq);
        lock_realtime_mutex(&self.trimmed_sequences, "realtime trim store")
            .insert(scope_key.clone(), trimmed_through_seq);
        lock_realtime_mutex(
            &self.capacity_trimmed_event_counts,
            "realtime capacity trim count store",
        )
        .insert(scope_key.clone(), capacity_trimmed_event_count);
        lock_realtime_mutex(
            &self.capacity_trimmed_sequences,
            "realtime capacity trim sequence store",
        )
        .insert(scope_key.clone(), capacity_trimmed_through_seq);
        {
            let mut last_capacity_trimmed = lock_realtime_mutex(
                &self.last_capacity_trimmed_at,
                "realtime capacity trim timestamp store",
            );
            if let Some(last_capacity_trimmed_at) = last_capacity_trimmed_at {
                last_capacity_trimmed.insert(scope_key.clone(), last_capacity_trimmed_at);
            } else {
                last_capacity_trimmed.remove(scope_key.as_str());
            }
        }
        lock_realtime_mutex(&self.notifiers, "realtime notifier store")
            .insert(scope_key, watch::channel(latest_realtime_seq).0);
        let scope_key = client_route_scope_key(
            snapshot.tenant_id.as_str(),
            snapshot.organization_id.as_str(),
            snapshot.principal_id.as_str(),
            snapshot.principal_kind.as_str(),
            snapshot.device_id.as_str(),
        );
        lock_realtime_mutex(
            &self.disconnect_generations,
            "realtime disconnect generation store",
        )
        .insert(scope_key.clone(), snapshot.disconnect_generation);
        lock_realtime_mutex(
            &self.disconnect_notifiers,
            "realtime disconnect notifier store",
        )
        .insert(scope_key, watch::channel(snapshot.disconnect_generation).0);

        Ok(())
    }

    fn restore_persisted_subscriptions_after_failed_restore(
        &self,
        previous_record: Option<RealtimeSubscriptionRecord>,
        attempted_record: Option<&RealtimeSubscriptionRecord>,
        tenant_id: &str,
        organization_id: &str,
        principal_id: &str,
        principal_kind: &str,
        device_id: &str,
    ) -> Result<(), RealtimeRuntimeError> {
        if let Some(previous_record) = previous_record {
            self.subscription_store
                .save_subscriptions(previous_record)
                .map_err(RealtimeRuntimeError::subscription_store)?;
            return Ok(());
        }

        if let Some(attempted_record) = attempted_record {
            self.subscription_store
                .clear_subscriptions_synced_at_or_before(
                    tenant_id,
                    organization_id,
                    principal_kind,
                    principal_id,
                    device_id,
                    attempted_record.synced_at.as_str(),
                )
                .map_err(RealtimeRuntimeError::subscription_store)?;
        }
        Ok(())
    }

    fn fail_with_event_window_rollback<T>(
        &self,
        rollback_records: Vec<RealtimeEventWindowRecord>,
        primary_error: RealtimeRuntimeError,
        context: &str,
    ) -> Result<T, RealtimeRuntimeError> {
        if rollback_records.is_empty() {
            return Err(primary_error);
        }
        if let Err(rollback_error) = self
            .event_window_store
            .save_windows(rollback_records)
            .map_err(RealtimeRuntimeError::event_window_store)
        {
            return Err(RealtimeRuntimeError {
                code: "realtime_state_compensation_failed",
                message: format!(
                    "{context}: {}; event window rollback failed: {}",
                    primary_error.message, rollback_error.message
                ),
            });
        }
        Err(primary_error)
    }

    // Scope fanout takes explicit addressing and payload fields because this is
    // the runtime's main delivery boundary and call sites benefit from keeping
    // the event identity fully visible.
    #[allow(clippy::too_many_arguments)]
    pub fn publish_scope_event_for_principal_kind(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_id: &str,
        principal_kind: &str,
        scope_type: &str,
        scope_id: &str,
        event_type: &str,
        payload: String,
        registered_client_routes: Vec<String>,
    ) -> Result<usize, RealtimeRuntimeError> {
        self.publish_scope_event_internal(
            tenant_id,
            organization_id,
            principal_id,
            principal_kind,
            scope_type,
            scope_id,
            event_type,
            payload,
            registered_client_routes,
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn publish_scope_event_internal(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_id: &str,
        principal_kind: &str,
        scope_type: &str,
        scope_id: &str,
        event_type: &str,
        payload: String,
        registered_client_routes: Vec<String>,
    ) -> Result<usize, RealtimeRuntimeError> {
        let mutation_keys = [realtime_mutation_principal_key(
            tenant_id,
            principal_kind,
            principal_id,
        )];
        let mutation_scope = RealtimeMutationScopeGuards::new(self, &mutation_keys);
        let _mutation_guards = mutation_scope.lock();
        let mut registered_client_routes = registered_client_routes;
        registered_client_routes.sort_unstable();
        registered_client_routes.dedup();
        let mut matched_targets = {
            let subscription_scope_index = lock_realtime_mutex(
                &self.subscription_scope_index,
                "realtime scope fanout index",
            );
            collect_matched_delivery_targets(
                &subscription_scope_index,
                tenant_id,
                organization_id,
                principal_id,
                principal_kind,
                scope_type,
                scope_id,
                event_type,
                registered_client_routes.clone(),
            )
        };
        let matched_device_ids = matched_targets
            .iter()
            .map(|(_, device_id)| device_id.as_str())
            .collect::<BTreeSet<_>>();
        let unmatched_registered_client_routes = registered_client_routes
            .iter()
            .filter(|device_id| !matched_device_ids.contains(device_id.as_str()))
            .cloned()
            .collect::<Vec<_>>();
        if !unmatched_registered_client_routes.is_empty() {
            let mut durable_matched_devices = self
                .subscription_store
                .load_matching_subscriptions(RealtimeMatchingSubscriptionQuery {
                    tenant_id,
                    organization_id,
                    principal_kind,
                    principal_id,
                    scope_type,
                    scope_id,
                    event_type,
                    candidate_device_ids: &unmatched_registered_client_routes,
                })
                .map_err(RealtimeRuntimeError::subscription_store)?
                .into_iter()
                .map(|record| record.device_id)
                .collect::<Vec<_>>();
            durable_matched_devices.sort_unstable();
            durable_matched_devices.dedup();
            for device_id in &durable_matched_devices {
                self.ensure_client_route_state_internal(
                    tenant_id,
                    organization_id,
                    principal_id,
                    principal_kind,
                    device_id,
                )?;
            }
            if !durable_matched_devices.is_empty() {
                let subscription_scope_index = lock_realtime_mutex(
                    &self.subscription_scope_index,
                    "realtime scope fanout index",
                );
                matched_targets = collect_matched_delivery_targets(
                    &subscription_scope_index,
                    tenant_id,
                    organization_id,
                    principal_id,
                    principal_kind,
                    scope_type,
                    scope_id,
                    event_type,
                    registered_client_routes,
                );
            }
        }
        if matched_targets.is_empty() {
            return Ok(0);
        }

        let mutations = {
            // Capture per-scope snapshots under the global sequence/window
            // locks. The critical section only performs HashMap lookups and
            // a single BTreeMap clone per target — the expensive mutation
            // computation (window insert, capacity trim, checkpoint and
            // event-window record construction) runs outside the lock so
            // concurrent publishes to different scopes do not serialize on
            // these global mutexes during the heavy work.
            let snapshots = {
                let windows = lock_realtime_mutex(&self.windows, "realtime window store");
                let latest_sequences =
                    lock_realtime_mutex(&self.latest_sequences, "realtime sequence store");
                let acked_sequences =
                    lock_realtime_mutex(&self.acked_sequences, "realtime ack store");
                let trimmed_sequences =
                    lock_realtime_mutex(&self.trimmed_sequences, "realtime trim store");
                let capacity_trimmed_event_counts = lock_realtime_mutex(
                    &self.capacity_trimmed_event_counts,
                    "realtime capacity trim count store",
                );
                let capacity_trimmed_sequences = lock_realtime_mutex(
                    &self.capacity_trimmed_sequences,
                    "realtime capacity trim sequence store",
                );
                let last_capacity_trimmed_timestamps = lock_realtime_mutex(
                    &self.last_capacity_trimmed_at,
                    "realtime capacity trim timestamp store",
                );
                matched_targets
                    .iter()
                    .map(|(scope_key, device_id)| RealtimeScopeSnapshot {
                        scope_key: scope_key.clone(),
                        device_id: device_id.clone(),
                        latest_realtime_seq: latest_sequences
                            .get(scope_key.as_str())
                            .copied()
                            .unwrap_or(0),
                        acked_through_seq: acked_sequences
                            .get(scope_key.as_str())
                            .copied()
                            .unwrap_or(0),
                        trimmed_through_seq: trimmed_sequences
                            .get(scope_key.as_str())
                            .copied()
                            .unwrap_or(0),
                        capacity_trimmed_event_count: capacity_trimmed_event_counts
                            .get(scope_key.as_str())
                            .copied()
                            .unwrap_or(0),
                        capacity_trimmed_through_seq: capacity_trimmed_sequences
                            .get(scope_key.as_str())
                            .copied()
                            .unwrap_or(0),
                        last_capacity_trimmed_at: last_capacity_trimmed_timestamps
                            .get(scope_key.as_str())
                            .cloned(),
                        current_window: windows
                            .get(scope_key.as_str())
                            .cloned()
                            .unwrap_or_default(),
                    })
                    .collect::<Vec<_>>()
            };
            // Compute mutations outside the global locks. The per-scope
            // mutation_lock shard acquired above still serializes
            // overlapping publishes to the same scope, so the window
            // insert + trim here is safe even though the global HashMaps
            // are not held.
            snapshots
                .into_iter()
                .map(|snapshot| {
                    let RealtimeScopeSnapshot {
                        scope_key,
                        device_id,
                        latest_realtime_seq: current_latest_seq,
                        acked_through_seq,
                        mut trimmed_through_seq,
                        mut capacity_trimmed_event_count,
                        mut capacity_trimmed_through_seq,
                        mut last_capacity_trimmed_at,
                        mut current_window,
                    } = snapshot;
                    let latest_realtime_seq = current_latest_seq.saturating_add(1);
                    current_window.insert(
                        latest_realtime_seq,
                        RealtimeEvent {
                            tenant_id: tenant_id.into(),
                            principal_id: principal_id.into(),
                            device_id: device_id.clone(),
                            realtime_seq: latest_realtime_seq,
                            scope_type: scope_type.into(),
                            scope_id: scope_id.into(),
                            event_type: event_type.into(),
                            delivery_class: "ephemeral".into(),
                            payload: payload.clone(),
                            occurred_at: realtime_timestamp(),
                        },
                    );
                    trim_window_to_capacity(
                        &mut current_window,
                        &mut trimmed_through_seq,
                        &mut capacity_trimmed_event_count,
                        &mut capacity_trimmed_through_seq,
                        &mut last_capacity_trimmed_at,
                    );
                    let checkpoint =
                        checkpoint_record_from_sequences(RealtimeCheckpointRecordParts {
                            tenant_id,
                            organization_id,
                            principal_id,
                            principal_kind,
                            device_id: device_id.as_str(),
                            latest_realtime_seq,
                            acked_through_seq,
                            trimmed_through_seq,
                            capacity_trimmed_event_count,
                            capacity_trimmed_through_seq,
                            last_capacity_trimmed_at: last_capacity_trimmed_at.clone(),
                        });
                    let event_window =
                        realtime_event_window_record_from_events(RealtimeEventWindowRecordParts {
                            tenant_id,
                            organization_id,
                            principal_id,
                            principal_kind,
                            device_id: device_id.as_str(),
                            events: current_window.values().cloned().collect(),
                            trimmed_through_seq,
                            capacity_trimmed_event_count,
                            capacity_trimmed_through_seq,
                            last_capacity_trimmed_at: last_capacity_trimmed_at.clone(),
                        });
                    RealtimePublishClientRouteMutation {
                        scope_key,
                        next_seq: latest_realtime_seq,
                        next_window: current_window,
                        latest_realtime_seq,
                        trimmed_through_seq,
                        capacity_trimmed_event_count,
                        capacity_trimmed_through_seq,
                        last_capacity_trimmed_at,
                        checkpoint,
                        event_window,
                    }
                })
                .collect::<Vec<_>>()
        };
        let delivered = mutations.len();
        let rollback_event_windows = mutations
            .iter()
            .map(|mutation| {
                self.event_window_store
                    .load_window(
                        mutation.checkpoint.tenant_id.as_str(),
                        mutation.checkpoint.organization_id.as_str(),
                        mutation.checkpoint.principal_kind.as_str(),
                        mutation.checkpoint.principal_id.as_str(),
                        mutation.checkpoint.device_id.as_str(),
                    )
                    .map(|record| {
                        event_window_record_or_empty(
                            record,
                            mutation.checkpoint.tenant_id.as_str(),
                            mutation.checkpoint.organization_id.as_str(),
                            mutation.checkpoint.principal_id.as_str(),
                            mutation.checkpoint.principal_kind.as_str(),
                            mutation.checkpoint.device_id.as_str(),
                        )
                    })
            })
            .collect::<Result<Vec<_>, _>>()
            .map_err(RealtimeRuntimeError::event_window_store)?;
        self.event_window_store
            .save_windows(
                mutations
                    .iter()
                    .map(|mutation| mutation.event_window.clone())
                    .collect(),
            )
            .map_err(RealtimeRuntimeError::event_window_store)?;
        if let Err(error) = self.persist_checkpoint_records(
            mutations
                .iter()
                .map(|mutation| mutation.checkpoint.clone())
                .collect(),
        ) {
            return self.fail_with_event_window_rollback(
                rollback_event_windows,
                error,
                "publish checkpoint persist failed",
            );
        }

        {
            let mut windows = lock_realtime_mutex(&self.windows, "realtime window store");
            let mut latest_sequences =
                lock_realtime_mutex(&self.latest_sequences, "realtime sequence store");
            let mut trimmed_sequences =
                lock_realtime_mutex(&self.trimmed_sequences, "realtime trim store");
            let mut capacity_trimmed_event_counts = lock_realtime_mutex(
                &self.capacity_trimmed_event_counts,
                "realtime capacity trim count store",
            );
            let mut capacity_trimmed_sequences = lock_realtime_mutex(
                &self.capacity_trimmed_sequences,
                "realtime capacity trim sequence store",
            );
            let mut last_capacity_trimmed = lock_realtime_mutex(
                &self.last_capacity_trimmed_at,
                "realtime capacity trim timestamp store",
            );
            for mutation in &mutations {
                windows.insert(mutation.scope_key.clone(), mutation.next_window.clone());
                latest_sequences.insert(mutation.scope_key.clone(), mutation.latest_realtime_seq);
                trimmed_sequences.insert(mutation.scope_key.clone(), mutation.trimmed_through_seq);
                capacity_trimmed_event_counts.insert(
                    mutation.scope_key.clone(),
                    mutation.capacity_trimmed_event_count,
                );
                capacity_trimmed_sequences.insert(
                    mutation.scope_key.clone(),
                    mutation.capacity_trimmed_through_seq,
                );
                if let Some(last_capacity_trimmed_at) = &mutation.last_capacity_trimmed_at {
                    last_capacity_trimmed
                        .insert(mutation.scope_key.clone(), last_capacity_trimmed_at.clone());
                } else {
                    last_capacity_trimmed.remove(mutation.scope_key.as_str());
                }
            }
        }

        let mut notifiers = lock_realtime_mutex(&self.notifiers, "realtime notifier store");
        for mutation in mutations {
            let sender = notifiers.entry(mutation.scope_key).or_insert_with(|| {
                let (sender, _) = watch::channel(0);
                sender
            });
            let _ = sender.send(mutation.next_seq);
        }

        Ok(delivered)
    }

    fn index_client_route_subscriptions(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_kind: &str,
        principal_id: &str,
        device_id: &str,
        subscriptions: &RealtimeClientRouteSubscriptions,
    ) {
        let mut subscription_scope_index = lock_realtime_mutex(
            &self.subscription_scope_index,
            "realtime scope fanout index",
        );
        for scope_key in &subscriptions.scope_order {
            let Some(subscription) = subscriptions.by_scope.get(scope_key) else {
                continue;
            };
            subscription_scope_index
                .entry(RealtimePrincipalScopeKey::new(
                    tenant_id,
                    organization_id,
                    principal_kind,
                    principal_id,
                    scope_key.scope_type.as_str(),
                    scope_key.scope_id.as_str(),
                ))
                .or_default()
                .insert(device_id.into(), subscription.clone());
        }
    }

    fn remove_device_subscription_index(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_kind: &str,
        principal_id: &str,
        device_id: &str,
    ) {
        let organization_id = normalize_realtime_organization_id(organization_id);
        let mut subscription_scope_index = lock_realtime_mutex(
            &self.subscription_scope_index,
            "realtime scope fanout index",
        );
        subscription_scope_index.retain(|scope_key, device_ids| {
            if scope_key.tenant_id == tenant_id
                && scope_key.organization_id == organization_id
                && scope_key.principal_kind == principal_kind
                && scope_key.principal_id == principal_id
            {
                device_ids.remove(device_id);
            }
            !device_ids.is_empty()
        });
    }
}

pub(super) fn lock_realtime_mutex<'a, T>(
    mutex: &'a Mutex<T>,
    lock_name: &'static str,
) -> MutexGuard<'a, T> {
    match mutex.lock() {
        Ok(guard) => guard,
        Err(poisoned) => {
            tracing::warn!("recovered poisoned realtime runtime mutex lock={lock_name}");
            poisoned.into_inner()
        }
    }
}

fn client_route_scope_key(
    tenant_id: &str,
    organization_id: &str,
    principal_id: &str,
    principal_kind: &str,
    device_id: &str,
) -> String {
    typed_client_route_scope_key(
        tenant_id,
        organization_id,
        principal_id,
        principal_kind,
        device_id,
    )
}

fn validate_payload_size(
    field: &'static str,
    payload: &str,
    max_bytes: usize,
) -> Result<(), RealtimeRuntimeError> {
    let actual_bytes = payload.len();
    if actual_bytes > max_bytes {
        return Err(RealtimeRuntimeError::payload_too_large(
            field,
            max_bytes,
            actual_bytes,
        ));
    }
    Ok(())
}

fn validate_realtime_subscription_items(
    items: &[RealtimeSubscriptionItemInput],
) -> Result<(), RealtimeRuntimeError> {
    if items.len() > REALTIME_MAX_SUBSCRIPTION_ITEMS {
        return Err(RealtimeRuntimeError::collection_too_large(
            "items",
            REALTIME_MAX_SUBSCRIPTION_ITEMS,
            items.len(),
        ));
    }
    let mut seen_scopes = BTreeSet::new();
    for item in items {
        let scope_key =
            realtime_subscription_scope_key(item.scope_type.as_str(), item.scope_id.as_str());
        if !seen_scopes.insert(scope_key.clone()) {
            return Err(RealtimeRuntimeError {
                code: "subscription_scope_duplicate",
                message: format!("duplicate realtime subscription scope: {scope_key}"),
            });
        }
    }
    let total_item_bytes = items.iter().fold(0usize, |total, item| {
        total
            .saturating_add(item.scope_type.len())
            .saturating_add(item.scope_id.len())
            .saturating_add(item.event_types.iter().map(String::len).sum::<usize>())
    });
    if total_item_bytes > REALTIME_MAX_SUBSCRIPTION_ITEMS_TOTAL_BYTES {
        return Err(RealtimeRuntimeError::payload_too_large(
            "items",
            REALTIME_MAX_SUBSCRIPTION_ITEMS_TOTAL_BYTES,
            total_item_bytes,
        ));
    }
    for item in items {
        validate_payload_size(
            "scopeType",
            item.scope_type.as_str(),
            REALTIME_MAX_SCOPE_TYPE_BYTES,
        )?;
        validate_payload_size(
            "scopeId",
            item.scope_id.as_str(),
            REALTIME_MAX_SCOPE_ID_BYTES,
        )?;
        let event_types_total_bytes = item.event_types.iter().fold(0usize, |total, event_type| {
            total.saturating_add(event_type.len())
        });
        if event_types_total_bytes > REALTIME_MAX_EVENT_TYPES_TOTAL_BYTES {
            return Err(RealtimeRuntimeError::payload_too_large(
                "eventTypes",
                REALTIME_MAX_EVENT_TYPES_TOTAL_BYTES,
                event_types_total_bytes,
            ));
        }
        for event_type in &item.event_types {
            validate_payload_size(
                "eventTypes",
                event_type.as_str(),
                REALTIME_MAX_EVENT_TYPE_BYTES,
            )?;
        }
    }
    Ok(())
}

fn realtime_subscription_scope_key(scope_type: &str, scope_id: &str) -> String {
    encode_realtime_key_segments([scope_type, scope_id])
}

fn realtime_mutation_principal_key(
    tenant_id: &str,
    principal_kind: &str,
    principal_id: &str,
) -> String {
    encode_realtime_key_segments([tenant_id, principal_kind, principal_id])
}

fn realtime_mutation_lock_index(scope_key: &str) -> usize {
    let mut hasher = DefaultHasher::new();
    scope_key.hash(&mut hasher);
    (hasher.finish() as usize) % REALTIME_MUTATION_LOCK_SHARDS
}

fn encode_realtime_key_segments<const N: usize>(segments: [&str; N]) -> String {
    segments
        .into_iter()
        .map(|segment| format!("{}#{segment}", segment.len()))
        .collect::<String>()
}

pub(crate) fn validate_realtime_event_limit(limit: usize) -> Result<(), RealtimeRuntimeError> {
    if limit == 0 || limit > REALTIME_EVENT_WINDOW_MAX_LIMIT {
        return Err(RealtimeRuntimeError::limit_invalid(limit));
    }
    Ok(())
}

pub fn realtime_postgres_sql_contracts() -> &'static [&'static str] {
    postgres_sql::ALL_REALTIME_POSTGRES_SQL_CONTRACTS
}

pub fn realtime_postgres_sql_contract_specs() -> &'static [RealtimePostgresSqlContract] {
    postgres_sql::REALTIME_POSTGRES_SQL_CONTRACT_SPECS
}

pub fn realtime_postgres_transaction_plans() -> &'static [&'static str] {
    postgres_sql::ALL_REALTIME_POSTGRES_TRANSACTION_PLANS
}

pub fn realtime_postgres_adapter_plan() -> &'static RealtimePostgresAdapterPlan {
    &postgres_sql::REALTIME_POSTGRES_ADAPTER_PLAN
}

fn normalize_checkpoint_fields(
    latest_realtime_seq: u64,
    acked_through_seq: u64,
    trimmed_through_seq: u64,
) -> (u64, u64, u64) {
    let acked_through_seq = acked_through_seq.min(latest_realtime_seq);
    let trimmed_through_seq = trimmed_through_seq.min(latest_realtime_seq);
    (latest_realtime_seq, acked_through_seq, trimmed_through_seq)
}

fn normalize_window_events(
    events: Vec<RealtimeEvent>,
    trimmed_through_seq: u64,
) -> BTreeMap<u64, RealtimeEvent> {
    events
        .into_iter()
        .filter(|event| event.realtime_seq > trimmed_through_seq)
        .fold(BTreeMap::new(), |mut deduped, event| {
            deduped.insert(event.realtime_seq, event);
            deduped
        })
}

fn retain_bounded_window_events(
    mut events: BTreeMap<u64, RealtimeEvent>,
    mut trimmed_through_seq: u64,
    mut capacity_trimmed_event_count: u64,
    mut capacity_trimmed_through_seq: u64,
    mut last_capacity_trimmed_at: Option<String>,
) -> (BTreeMap<u64, RealtimeEvent>, u64, u64, u64, Option<String>) {
    trim_window_to_capacity(
        &mut events,
        &mut trimmed_through_seq,
        &mut capacity_trimmed_event_count,
        &mut capacity_trimmed_through_seq,
        &mut last_capacity_trimmed_at,
    );
    (
        events,
        trimmed_through_seq,
        capacity_trimmed_event_count,
        capacity_trimmed_through_seq,
        last_capacity_trimmed_at,
    )
}

fn trim_window_to_capacity(
    events: &mut BTreeMap<u64, RealtimeEvent>,
    trimmed_through_seq: &mut u64,
    capacity_trimmed_event_count: &mut u64,
    capacity_trimmed_through_seq: &mut u64,
    last_capacity_trimmed_at: &mut Option<String>,
) {
    let trim_started_at = if events.len() > REALTIME_CLIENT_ROUTE_WINDOW_MAX_RETAINED_EVENTS {
        Some(realtime_timestamp())
    } else {
        None
    };
    while events.len() > REALTIME_CLIENT_ROUTE_WINDOW_MAX_RETAINED_EVENTS {
        if let Some((trimmed_seq, _)) = events.pop_first() {
            *trimmed_through_seq = (*trimmed_through_seq).max(trimmed_seq);
            *capacity_trimmed_event_count = capacity_trimmed_event_count.saturating_add(1);
            *capacity_trimmed_through_seq = (*capacity_trimmed_through_seq).max(trimmed_seq);
        } else {
            break;
        }
    }
    if let Some(trim_started_at) = trim_started_at {
        *last_capacity_trimmed_at = Some(trim_started_at);
    }
}

struct RealtimeEventWindowRecordParts<'a> {
    tenant_id: &'a str,
    organization_id: &'a str,
    principal_id: &'a str,
    principal_kind: &'a str,
    device_id: &'a str,
    events: Vec<RealtimeEvent>,
    trimmed_through_seq: u64,
    capacity_trimmed_event_count: u64,
    capacity_trimmed_through_seq: u64,
    last_capacity_trimmed_at: Option<String>,
}

fn realtime_event_window_record_from_events(
    parts: RealtimeEventWindowRecordParts<'_>,
) -> RealtimeEventWindowRecord {
    RealtimeEventWindowRecord {
        tenant_id: parts.tenant_id.into(),
        organization_id: parts.organization_id.into(),
        principal_kind: parts.principal_kind.into(),
        principal_id: parts.principal_id.into(),
        device_id: parts.device_id.into(),
        events: parts.events,
        trimmed_through_seq: parts.trimmed_through_seq,
        capacity_trimmed_event_count: parts.capacity_trimmed_event_count,
        capacity_trimmed_through_seq: parts.capacity_trimmed_through_seq,
        last_capacity_trimmed_at: parts.last_capacity_trimmed_at,
        updated_at: realtime_timestamp(),
    }
    .normalized()
}

fn event_window_record_or_empty(
    record: Option<RealtimeEventWindowRecord>,
    tenant_id: &str,
    organization_id: &str,
    principal_id: &str,
    principal_kind: &str,
    device_id: &str,
) -> RealtimeEventWindowRecord {
    record
        .map(RealtimeEventWindowRecord::normalized)
        .unwrap_or_else(|| {
            realtime_event_window_record_from_events(RealtimeEventWindowRecordParts {
                tenant_id,
                organization_id,
                principal_id,
                principal_kind,
                device_id,
                events: Vec::new(),
                trimmed_through_seq: 0,
                capacity_trimmed_event_count: 0,
                capacity_trimmed_through_seq: 0,
                last_capacity_trimmed_at: None,
            })
        })
}

fn subscription_record_from_items(
    tenant_id: &str,
    organization_id: &str,
    principal_id: &str,
    principal_kind: &str,
    device_id: &str,
    items: Vec<RealtimeSubscription>,
) -> Option<RealtimeSubscriptionRecord> {
    if items.is_empty() {
        return None;
    }

    Some(RealtimeSubscriptionRecord {
        tenant_id: tenant_id.into(),
        organization_id: organization_id.into(),
        principal_kind: principal_kind.into(),
        principal_id: principal_id.into(),
        device_id: device_id.into(),
        synced_at: subscriptions_synced_at(items.as_slice()),
        items,
    })
}

#[allow(clippy::too_many_arguments)]
fn collect_matched_delivery_targets(
    subscription_scope_index: &HashMap<
        RealtimePrincipalScopeKey,
        BTreeMap<String, RealtimeSubscription>,
    >,
    tenant_id: &str,
    organization_id: &str,
    principal_id: &str,
    principal_kind: &str,
    scope_type: &str,
    scope_id: &str,
    event_type: &str,
    registered_client_routes: Vec<String>,
) -> Vec<(String, String)> {
    let registered_client_routes = registered_client_routes
        .into_iter()
        .collect::<BTreeSet<_>>();
    let candidate_subscriptions = subscription_scope_index
        .get(&RealtimePrincipalScopeKey::new(
            tenant_id,
            organization_id,
            principal_kind,
            principal_id,
            scope_type,
            scope_id,
        ))
        .cloned()
        .unwrap_or_default();

    candidate_subscriptions
        .into_iter()
        .filter_map(|(device_id, subscription)| {
            if !registered_client_routes.contains(device_id.as_str()) {
                return None;
            }
            if !subscription_matches_event(&subscription, event_type) {
                return None;
            }
            let scope_key = client_route_scope_key(
                tenant_id,
                organization_id,
                principal_id,
                principal_kind,
                device_id.as_str(),
            );
            Some((scope_key, device_id))
        })
        .collect()
}

fn subscription_matches_event(subscription: &RealtimeSubscription, event_type: &str) -> bool {
    subscription.event_types.is_empty()
        || subscription
            .event_types
            .iter()
            .any(|item| item == event_type)
}

fn realtime_timestamp() -> String {
    utc_now_rfc3339_millis()
}

fn realtime_checkpoint_timestamp() -> String {
    utc_now_rfc3339_millis()
}

fn subscriptions_synced_at(items: &[RealtimeSubscription]) -> String {
    items
        .iter()
        .map(|item| item.subscribed_at.as_str())
        .max()
        .map(str::to_owned)
        .unwrap_or_else(realtime_timestamp)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    use im_adapters_local_memory::MemoryRealtimeCheckpointStore;

    #[test]
    fn test_postgres_realtime_sql_contracts_are_compiled_with_runtime_module() {
        assert_eq!(realtime_postgres_sql_contracts().len(), 21);
        assert_eq!(realtime_postgres_sql_contract_specs().len(), 21);
        assert_eq!(realtime_postgres_transaction_plans().len(), 6);
        assert_eq!(realtime_postgres_adapter_plan().method_plans.len(), 21);
        assert!(
            realtime_postgres_sql_contracts()
                .iter()
                .all(|sql| !sql.trim().is_empty())
        );
        assert!(
            realtime_postgres_sql_contract_specs()
                .iter()
                .all(|spec| !spec.name.trim().is_empty() && !spec.sql.trim().is_empty())
        );
        assert!(
            realtime_postgres_transaction_plans()
                .iter()
                .all(|plan| !plan.trim().is_empty())
        );
    }

    #[test]
    fn test_collect_matched_delivery_targets_filters_to_registered_matching_devices() {
        let mut subscriptions = HashMap::new();
        subscriptions.insert(
            client_route_scope_key("100001", "default", "1", "user", "d_match"),
            RealtimeClientRouteSubscriptions::from_items(vec![subscription(
                "conversation",
                "c_demo",
                vec!["message.posted"],
            )]),
        );
        subscriptions.insert(
            client_route_scope_key("100001", "default", "1", "user", "d_other_scope"),
            RealtimeClientRouteSubscriptions::from_items(vec![subscription(
                "conversation",
                "c_other",
                vec!["message.posted"],
            )]),
        );
        subscriptions.insert(
            client_route_scope_key("100001", "default", "1", "user", "d_other_event"),
            RealtimeClientRouteSubscriptions::from_items(vec![subscription(
                "conversation",
                "c_demo",
                vec!["message.read"],
            )]),
        );
        let subscription_scope_index = subscription_scope_index_from_subscriptions(&[
            (
                "d_match",
                subscriptions
                    .get(client_route_scope_key("100001", "default", "1", "user", "d_match").as_str())
                    .expect("matching subscription should exist"),
            ),
            (
                "d_other_scope",
                subscriptions
                    .get(
                        client_route_scope_key("100001", "default", "1", "user", "d_other_scope")
                            .as_str(),
                    )
                    .expect("other scope subscription should exist"),
            ),
            (
                "d_other_event",
                subscriptions
                    .get(
                        client_route_scope_key("100001", "default", "1", "user", "d_other_event")
                            .as_str(),
                    )
                    .expect("other event subscription should exist"),
            ),
        ]);

        let matched = collect_matched_delivery_targets(
            &subscription_scope_index,
            "100001",
            "default",
            "1",
            "user",
            "conversation",
            "c_demo",
            "message.posted",
            vec![
                "d_other_scope".into(),
                "d_match".into(),
                "d_other_event".into(),
                "d_match".into(),
                "d_missing".into(),
            ],
        );

        assert_eq!(
            matched,
            vec![(
                client_route_scope_key("100001", "default", "1", "user", "d_match"),
                "d_match".into()
            )]
        );
    }

    #[test]
    fn test_collect_matched_delivery_targets_accepts_wildcard_event_subscriptions() {
        let mut subscriptions = HashMap::new();
        subscriptions.insert(
            client_route_scope_key("100001", "default", "1", "user", "d_wildcard"),
            RealtimeClientRouteSubscriptions::from_items(vec![subscription(
                "conversation",
                "c_demo",
                vec![],
            )]),
        );
        let subscription_scope_index = subscription_scope_index_from_subscriptions(&[(
            "d_wildcard",
            subscriptions
                .get(client_route_scope_key("100001", "default", "1", "user", "d_wildcard").as_str())
                .expect("wildcard subscription should exist"),
        )]);

        let matched = collect_matched_delivery_targets(
            &subscription_scope_index,
            "100001",
            "default",
            "1",
            "user",
            "conversation",
            "c_demo",
            "message.edited",
            vec!["d_wildcard".into()],
        );

        assert_eq!(
            matched,
            vec![(
                client_route_scope_key("100001", "default", "1", "user", "d_wildcard"),
                "d_wildcard".into()
            )]
        );
    }

    #[test]
    fn test_persist_checkpoint_normalizes_transient_inconsistent_sequence_state() {
        let checkpoint_store = Arc::new(MemoryRealtimeCheckpointStore::default());
        let runtime = RealtimeDeliveryRuntime::with_checkpoint_store(checkpoint_store.clone());
        let scope_key = client_route_scope_key("100001", "default", "1", "user", "d_pad");

        runtime
            .latest_sequences
            .lock()
            .expect("realtime sequence store should lock")
            .insert(scope_key.clone(), 3);
        runtime
            .acked_sequences
            .lock()
            .expect("realtime ack store should lock")
            .insert(scope_key.clone(), 9);
        runtime
            .trimmed_sequences
            .lock()
            .expect("realtime trim store should lock")
            .insert(scope_key, 11);

        runtime
            .persist_checkpoint_for_principal_kind("100001", "default", "1", "user", "d_pad")
            .expect("checkpoint persist should succeed");

        let persisted = checkpoint_store
            .checkpoint("100001", "default", "user", "1", "d_pad")
            .expect("checkpoint should be persisted");
        assert_eq!(persisted.latest_realtime_seq, 3);
        assert_eq!(persisted.acked_through_seq, 3);
        assert_eq!(persisted.trimmed_through_seq, 3);
    }

    #[test]
    fn test_window_checkpoint_normalizes_transient_inconsistent_sequence_state() {
        let runtime = RealtimeDeliveryRuntime::permissive_for_tests();
        let scope_key = client_route_scope_key("100001", "default", "1", "user", "d_pad");
        runtime
            .latest_sequences
            .lock()
            .expect("realtime sequence store should lock")
            .insert(scope_key.clone(), 3);
        runtime
            .acked_sequences
            .lock()
            .expect("realtime ack store should lock")
            .insert(scope_key.clone(), 9);
        runtime
            .trimmed_sequences
            .lock()
            .expect("realtime trim store should lock")
            .insert(scope_key, 11);

        let checkpoint = runtime
            .window_checkpoint_for_principal_kind("100001", "default", "1", "user", "d_pad")
            .expect("window checkpoint should be readable");

        assert_eq!(checkpoint.latest_realtime_seq, 3);
        assert_eq!(checkpoint.acked_through_seq, 3);
        assert_eq!(checkpoint.trimmed_through_seq, 3);
    }

    #[test]
    fn test_ensure_client_route_state_recovers_from_poisoned_sequence_store_lock() {
        let runtime = RealtimeDeliveryRuntime::default();
        let _ = std::panic::catch_unwind({
            let latest_sequences = runtime.latest_sequences.clone();
            move || {
                let _guard = latest_sequences
                    .lock()
                    .expect("realtime sequence store should lock");
                panic!("poison realtime sequence store lock");
            }
        });

        runtime
            .ensure_client_route_state_for_principal_kind("100001", "default", "1", "user", "d_poison")
            .expect("poisoned lock should be recovered");
        let checkpoint = runtime
            .window_checkpoint_for_principal_kind("100001", "default", "1", "user", "d_poison")
            .expect("window checkpoint should still be available");
        assert_eq!(checkpoint.latest_realtime_seq, 0);
        assert_eq!(checkpoint.acked_through_seq, 0);
        assert_eq!(checkpoint.trimmed_through_seq, 0);
    }

    #[test]
    fn test_sync_subscriptions_rejects_archived_conversation_scope_when_policy_denies() {
        let runtime = RealtimeDeliveryRuntime::with_checkpoint_store_and_scope_access_policy(
            Arc::new(MemoryRealtimeCheckpointStore::default()),
            Arc::new(ArchivedConversationPolicy),
        );

        let error = runtime
            .sync_subscriptions_for_principal_kind(
                "100001",
                "default",
                "1",
                "user",
                "d_pad",
                vec![RealtimeSubscriptionItemInput {
                    scope_type: "conversation".into(),
                    scope_id: "c_archived".into(),
                    event_types: vec!["message.posted".into()],
                }],
            )
            .expect_err("archived conversation subscription should be rejected");

        assert_eq!(error.code, "conversation_archived");
    }

    #[test]
    fn test_list_events_filters_hidden_conversation_scopes_and_advances_cursor() {
        let runtime = RealtimeDeliveryRuntime::with_checkpoint_store_and_scope_access_policy(
            Arc::new(MemoryRealtimeCheckpointStore::default()),
            Arc::new(ArchivedConversationPolicy),
        );
        let scope_key = client_route_scope_key("100001", "default", "1", "user", "d_pad");
        lock_realtime_mutex(&runtime.windows, "realtime window store").insert(
            scope_key.clone(),
            [
                (
                    1,
                    RealtimeEvent {
                        tenant_id: "100001".into(),
                        principal_id: "1".into(),
                        device_id: "d_pad".into(),
                        realtime_seq: 1,
                        scope_type: "conversation".into(),
                        scope_id: "c_visible".into(),
                        event_type: "message.posted".into(),
                        delivery_class: "ephemeral".into(),
                        payload: "{}".into(),
                        occurred_at: "2026-04-15T10:00:00Z".into(),
                    },
                ),
                (
                    2,
                    RealtimeEvent {
                        tenant_id: "100001".into(),
                        principal_id: "1".into(),
                        device_id: "d_pad".into(),
                        realtime_seq: 2,
                        scope_type: "conversation".into(),
                        scope_id: "c_archived".into(),
                        event_type: "message.posted".into(),
                        delivery_class: "ephemeral".into(),
                        payload: "{}".into(),
                        occurred_at: "2026-04-15T10:00:01Z".into(),
                    },
                ),
            ]
            .into_iter()
            .collect(),
        );
        lock_realtime_mutex(&runtime.latest_sequences, "realtime sequence store")
            .insert(scope_key, 2);

        let window = runtime
            .list_events_for_principal_kind("100001", "default", "1", "user", "d_pad", 0, 10)
            .expect("filtered realtime window should be readable");

        assert_eq!(window.items.len(), 1);
        assert_eq!(window.items[0].scope_id, "c_visible");
        assert_eq!(window.next_after_seq, Some(2));
        assert!(!window.has_more);
    }

    #[test]
    fn test_list_events_never_returns_events_at_or_below_trim_boundary() {
        let runtime = RealtimeDeliveryRuntime::with_checkpoint_store_and_scope_access_policy(
            Arc::new(MemoryRealtimeCheckpointStore::default()),
            Arc::new(StandaloneRealtimeScopeAccessPolicy),
        );
        let scope_key = client_route_scope_key("100001", "default", "1", "user", "d_pad");
        lock_realtime_mutex(&runtime.windows, "realtime window store").insert(
            scope_key.clone(),
            [
                (
                    1,
                    RealtimeEvent {
                        tenant_id: "100001".into(),
                        principal_id: "1".into(),
                        device_id: "d_pad".into(),
                        realtime_seq: 1,
                        scope_type: "conversation".into(),
                        scope_id: "c_demo".into(),
                        event_type: "message.posted".into(),
                        delivery_class: "ephemeral".into(),
                        payload: r#"{"messageId":"trimmed"}"#.into(),
                        occurred_at: "2026-04-15T10:00:00Z".into(),
                    },
                ),
                (
                    2,
                    RealtimeEvent {
                        tenant_id: "100001".into(),
                        principal_id: "1".into(),
                        device_id: "d_pad".into(),
                        realtime_seq: 2,
                        scope_type: "conversation".into(),
                        scope_id: "c_demo".into(),
                        event_type: "message.posted".into(),
                        delivery_class: "ephemeral".into(),
                        payload: r#"{"messageId":"visible"}"#.into(),
                        occurred_at: "2026-04-15T10:00:01Z".into(),
                    },
                ),
            ]
            .into_iter()
            .collect(),
        );
        lock_realtime_mutex(&runtime.latest_sequences, "realtime sequence store")
            .insert(scope_key.clone(), 2);
        lock_realtime_mutex(&runtime.trimmed_sequences, "realtime trim store").insert(scope_key, 1);

        let window = runtime
            .list_events_for_principal_kind("100001", "default", "1", "user", "d_pad", 0, 10)
            .expect("trimmed realtime window should be readable");

        assert_eq!(window.items.len(), 1);
        assert_eq!(window.items[0].realtime_seq, 2);
        assert_eq!(window.items[0].payload, r#"{"messageId":"visible"}"#);
        assert_eq!(window.next_after_seq, Some(2));
        assert_eq!(window.trimmed_through_seq, 1);
    }

    struct ArchivedConversationPolicy;

    impl RealtimeScopeAccessPolicy for ArchivedConversationPolicy {
        fn validate_subscription_scope(
            &self,
            _tenant_id: &str,
            _principal_id: &str,
            _principal_kind: &str,
            scope_type: &str,
            scope_id: &str,
        ) -> Result<(), RealtimeRuntimeError> {
            if scope_type == "conversation" && scope_id == "c_archived" {
                return Err(RealtimeRuntimeError {
                    code: "conversation_archived",
                    message: format!("direct chat conversation is archived: {scope_id}"),
                });
            }

            Ok(())
        }

        fn is_event_visible(
            &self,
            _tenant_id: &str,
            _principal_id: &str,
            _principal_kind: &str,
            event: &RealtimeEvent,
        ) -> bool {
            event.scope_type != "conversation" || event.scope_id != "c_archived"
        }
    }

    fn subscription(
        scope_type: &str,
        scope_id: &str,
        event_types: Vec<&str>,
    ) -> RealtimeSubscription {
        RealtimeSubscription {
            scope_type: scope_type.into(),
            scope_id: scope_id.into(),
            event_types: event_types.into_iter().map(|item| item.into()).collect(),
            subscribed_at: "2026-04-05T10:10:00Z".into(),
        }
    }

    fn subscription_scope_index_from_subscriptions(
        subscriptions: &[(&str, &RealtimeClientRouteSubscriptions)],
    ) -> HashMap<RealtimePrincipalScopeKey, BTreeMap<String, RealtimeSubscription>> {
        let mut index: HashMap<RealtimePrincipalScopeKey, BTreeMap<String, RealtimeSubscription>> =
            HashMap::new();
        for (device_id, client_route_subscriptions) in subscriptions {
            for subscription_scope in &client_route_subscriptions.scope_order {
                let subscription = client_route_subscriptions
                    .by_scope
                    .get(subscription_scope)
                    .expect("test subscription should exist for scope");
                index
                    .entry(RealtimePrincipalScopeKey::new(
                        "100001",
                        "default",
                        "user",
                        "1",
                        subscription_scope.scope_type.as_str(),
                        subscription_scope.scope_id.as_str(),
                    ))
                    .or_default()
                    .insert((*device_id).into(), subscription.clone());
            }
        }
        index
    }
}
