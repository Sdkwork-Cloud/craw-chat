use chrono::{DateTime, SecondsFormat, Utc};
use im_domain_core::{
    presence::{PresenceClientView, PresenceStatus},
    realtime::RealtimeEvent,
};
use im_platform_contracts::{
    ContractError, PresenceStateRecord, PresenceStateStore, RealtimeCheckpointRecord,
    RealtimeCheckpointStore, RealtimeDisconnectFenceRecord, RealtimeDisconnectFenceStore,
    RealtimeEventWindowDiagnosticsSnapshot, RealtimeEventWindowHighRiskRecord,
    RealtimeEventWindowRecord, RealtimeEventWindowStore, RealtimeMatchingSubscriptionQuery,
    RealtimeSubscriptionRecord, RealtimeSubscriptionStore,
};
use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;
pub use r2d2_postgres::postgres::NoTls;
use r2d2_postgres::postgres::{Row, Transaction};
use sdkwork_utils_rust::sha256_hash;
use tokio::runtime::Handle;
mod route_store;
pub use route_store::{PostgresBackedRouteStore, PostgresRoutePersistence};

const DEFAULT_POOL_MAX_SIZE: u32 = 16;
const DEFAULT_POOL_MIN_IDLE: u32 = 0;

const LOAD_PRESENCE_STATE_SQL: &str = r#"
select
    tenant_id,
    organization_id,
    principal_kind,
    principal_id,
    device_id,
    session_id,
    presence_status,
    last_sync_seq,
    last_resume_at,
    last_seen_at,
    resume_required,
    payload_json::text as payload_json,
    updated_at
from im_presence_states
where tenant_id = $1
  and organization_id = $2
  and principal_kind = $3
  and principal_id = $4
  and device_id = $5
"#;

const UPSERT_PRESENCE_STATE_SQL: &str = r#"
insert into im_presence_states (
    tenant_id,
    organization_id,
    principal_kind,
    principal_id,
    device_id,
    session_id,
    presence_status,
    last_sync_seq,
    last_resume_at,
    last_seen_at,
    payload_json,
    payload_hash,
    resume_required,
    created_at,
    updated_at
) values (
    $1, $2, $3, $4, $5, $6, $7, $8, $9, $10,
    $11::jsonb, $12, $13, $14, $15
)
on conflict (tenant_id, organization_id, principal_kind, principal_id, device_id) do update set
    session_id = excluded.session_id,
    presence_status = excluded.presence_status,
    last_sync_seq = excluded.last_sync_seq,
    last_resume_at = excluded.last_resume_at,
    last_seen_at = excluded.last_seen_at,
    payload_json = excluded.payload_json,
    payload_hash = excluded.payload_hash,
    resume_required = excluded.resume_required,
    updated_at = excluded.updated_at
where excluded.updated_at > im_presence_states.updated_at
   or (
      excluded.updated_at = im_presence_states.updated_at
      and excluded.last_sync_seq > im_presence_states.last_sync_seq
   )
   or (
      excluded.updated_at = im_presence_states.updated_at
      and excluded.last_sync_seq = im_presence_states.last_sync_seq
      and excluded.payload_hash = im_presence_states.payload_hash
   )
"#;

const LIST_PRESENCE_STATES_FOR_PRINCIPAL_SQL: &str = r#"
select
    tenant_id,
    organization_id,
    principal_kind,
    principal_id,
    device_id,
    session_id,
    presence_status,
    last_sync_seq,
    last_resume_at,
    last_seen_at,
    resume_required,
    payload_json::text as payload_json,
    updated_at
from im_presence_states
where tenant_id = $1
  and organization_id = $2
  and principal_kind = $3
  and principal_id = $4
order by device_id asc
"#;

const LIST_STALE_ONLINE_PRESENCE_STATES_SQL: &str = r#"
select
    tenant_id,
    organization_id,
    principal_kind,
    principal_id,
    device_id,
    session_id,
    presence_status,
    last_sync_seq,
    last_resume_at,
    last_seen_at,
    resume_required,
    payload_json::text as payload_json,
    updated_at
from im_presence_states
where presence_status = 'online'
  and last_seen_at is not null
  and last_seen_at <= $1
order by last_seen_at asc, tenant_id asc, organization_id asc, principal_kind asc, principal_id asc, device_id asc
limit $2
"#;

const EXPIRE_STALE_ONLINE_PRESENCE_STATE_SQL: &str = r#"
update im_presence_states
set
    session_id = null,
    presence_status = 'offline',
    last_seen_at = $7,
    payload_json = $8::jsonb,
    payload_hash = $9,
    resume_required = true,
    updated_at = $7
where tenant_id = $1
  and organization_id = $2
  and principal_kind = $3
  and principal_id = $4
  and device_id = $5
  and presence_status = 'online'
  and last_seen_at is not null
  and last_seen_at <= $6
returning
    tenant_id,
    organization_id,
    principal_kind,
    principal_id,
    device_id,
    session_id,
    presence_status,
    last_sync_seq,
    last_resume_at,
    last_seen_at,
    resume_required,
    payload_json::text as payload_json,
    updated_at
"#;

use im_postgres_realtime_contracts::{
    CLEAR_REALTIME_CLIENT_ROUTE_EVENTS_SQL,
    CLEAR_REALTIME_DISCONNECT_FENCE_DISCONNECTED_AT_OR_BEFORE_SQL,
    CLEAR_REALTIME_DISCONNECT_FENCE_IF_MATCHES_SQL, CLEAR_REALTIME_DISCONNECT_FENCE_SQL,
    CLEAR_REALTIME_SUBSCRIPTION_IF_SYNCED_AT_OR_BEFORE_SQL, CLEAR_REALTIME_SUBSCRIPTION_SCOPES_SQL,
    CLEAR_REALTIME_SUBSCRIPTION_SQL, LIST_REALTIME_CLIENT_ROUTE_EVENTS_SQL,
    LIST_REALTIME_EVENT_WINDOW_HIGH_RISK_WINDOWS_SQL, LOAD_MATCHING_REALTIME_SUBSCRIPTIONS_SQL,
    LOAD_REALTIME_CHECKPOINT_SQL, LOAD_REALTIME_DISCONNECT_FENCE_SQL,
    LOAD_REALTIME_EVENT_WINDOW_DIAGNOSTICS_SQL, LOAD_REALTIME_SUBSCRIPTION_SQL,
    REPLACE_REALTIME_SUBSCRIPTION_SCOPES_SQL, TRIM_REALTIME_CLIENT_ROUTE_EVENTS_SQL,
    UPSERT_REALTIME_CHECKPOINT_SQL, UPSERT_REALTIME_CLIENT_ROUTE_EVENT_SQL,
    UPSERT_REALTIME_DISCONNECT_FENCE_SQL, UPSERT_REALTIME_SUBSCRIPTION_SQL,
};
pub type PostgresRealtimeConnectionManager = PostgresConnectionManager<NoTls>;
pub type PostgresRealtimePool = Pool<PostgresRealtimeConnectionManager>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PostgresRealtimeConfig {
    database_url: String,
    pool_max_size: u32,
    pool_min_idle: Option<u32>,
}

impl PostgresRealtimeConfig {
    pub fn new(database_url: impl Into<String>) -> Self {
        Self {
            database_url: database_url.into(),
            pool_max_size: DEFAULT_POOL_MAX_SIZE,
            pool_min_idle: Some(DEFAULT_POOL_MIN_IDLE),
        }
    }

    /// Create config from sdkwork-pool DatabaseConfig
    pub fn from_pool_config(config: &sdkwork_database_config::DatabaseConfig) -> Self {
        Self {
            database_url: config.url.clone(),
            pool_max_size: config.max_connections,
            pool_min_idle: Some(config.min_connections),
        }
    }

    pub fn with_pool_max_size(mut self, pool_max_size: u32) -> Self {
        self.pool_max_size = pool_max_size.max(1);
        if let Some(pool_min_idle) = self.pool_min_idle {
            self.pool_min_idle = Some(pool_min_idle.min(self.pool_max_size));
        }
        self
    }

    pub fn with_pool_min_idle(mut self, pool_min_idle: u32) -> Self {
        self.pool_min_idle = Some(pool_min_idle.min(self.pool_max_size));
        self
    }

    pub fn database_url(&self) -> &str {
        self.database_url.as_str()
    }

    pub fn pool_max_size(&self) -> u32 {
        self.pool_max_size
    }

    pub fn pool_min_idle(&self) -> Option<u32> {
        self.pool_min_idle
    }

    pub fn connect_pool(&self) -> Result<PostgresRealtimePool, ContractError> {
        if Handle::try_current().is_ok() {
            return self.connect_pool_bridged();
        }
        build_realtime_pool(self)
    }

    /// Creates a pool on a dedicated OS thread when called from a Tokio runtime.
    pub fn connect_pool_bridged(&self) -> Result<PostgresRealtimePool, ContractError> {
        let config = self.clone();
        run_postgres_io(move || build_realtime_pool(&config))
    }

    pub fn connect(&self) -> Result<PostgresRealtimeCheckpointStore, ContractError> {
        PostgresRealtimeCheckpointStore::connect(self.clone())
    }

    pub fn connect_event_window(&self) -> Result<PostgresRealtimeEventWindowStore, ContractError> {
        PostgresRealtimeEventWindowStore::connect(self.clone())
    }
}

fn build_realtime_pool(config: &PostgresRealtimeConfig) -> Result<PostgresRealtimePool, ContractError> {
    let pg_config = config
        .database_url
        .parse()
        .map_err(|error| postgres_config_error(config.database_url.as_str(), error))?;
    let manager = PostgresConnectionManager::new(pg_config, NoTls);
    Pool::builder()
        .max_size(config.pool_max_size)
        .min_idle(config.pool_min_idle)
        .build(manager)
        .map_err(|error| postgres_unavailable("create realtime pool", error))
}

#[derive(Clone)]
pub struct PostgresRealtimeCheckpointStore {
    pool: PostgresRealtimePool,
}

#[derive(Clone)]
pub struct PostgresRealtimeEventWindowStore {
    pool: PostgresRealtimePool,
}

#[derive(Clone)]
pub struct PostgresRealtimePresenceStateStore {
    pool: PostgresRealtimePool,
}

impl PostgresRealtimePresenceStateStore {
    pub fn connect(config: PostgresRealtimeConfig) -> Result<Self, ContractError> {
        config.connect_pool().map(Self::from_pool)
    }

    pub fn from_pool(pool: PostgresRealtimePool) -> Self {
        Self { pool }
    }

    pub fn presence_load_sql() -> &'static str {
        LOAD_PRESENCE_STATE_SQL
    }

    pub fn presence_upsert_sql() -> &'static str {
        UPSERT_PRESENCE_STATE_SQL
    }

    pub fn presence_list_principal_sql() -> &'static str {
        LIST_PRESENCE_STATES_FOR_PRINCIPAL_SQL
    }

    pub fn presence_list_stale_online_sql() -> &'static str {
        LIST_STALE_ONLINE_PRESENCE_STATES_SQL
    }

    pub fn presence_expire_stale_online_sql() -> &'static str {
        EXPIRE_STALE_ONLINE_PRESENCE_STATE_SQL
    }
}

impl PostgresRealtimeEventWindowStore {
    pub fn connect(config: PostgresRealtimeConfig) -> Result<Self, ContractError> {
        config.connect_pool().map(Self::from_pool)
    }

    pub fn from_pool(pool: PostgresRealtimePool) -> Self {
        Self { pool }
    }

    pub fn checkpoint_load_sql() -> &'static str {
        LOAD_REALTIME_CHECKPOINT_SQL
    }

    pub fn checkpoint_upsert_sql() -> &'static str {
        UPSERT_REALTIME_CHECKPOINT_SQL
    }

    pub fn client_route_event_upsert_sql() -> &'static str {
        UPSERT_REALTIME_CLIENT_ROUTE_EVENT_SQL
    }

    pub fn client_route_events_list_sql() -> &'static str {
        LIST_REALTIME_CLIENT_ROUTE_EVENTS_SQL
    }

    pub fn client_route_events_trim_sql() -> &'static str {
        TRIM_REALTIME_CLIENT_ROUTE_EVENTS_SQL
    }

    pub fn client_route_events_clear_sql() -> &'static str {
        CLEAR_REALTIME_CLIENT_ROUTE_EVENTS_SQL
    }

    pub fn diagnostics_sql() -> &'static str {
        LOAD_REALTIME_EVENT_WINDOW_DIAGNOSTICS_SQL
    }

    pub fn high_risk_windows_sql() -> &'static str {
        LIST_REALTIME_EVENT_WINDOW_HIGH_RISK_WINDOWS_SQL
    }

    pub fn checkpoint_after_trim(
        current: Option<RealtimeCheckpointRecord>,
        acked_through_seq: u64,
        updated_at: &str,
    ) -> RealtimeCheckpointRecord {
        current
            .map(|mut record| {
                record.latest_realtime_seq = record.latest_realtime_seq.max(acked_through_seq);
                record.acked_through_seq = record.acked_through_seq.max(acked_through_seq);
                record.trimmed_through_seq = record.trimmed_through_seq.max(acked_through_seq);
                record.updated_at = updated_at.into();
                record.normalized()
            })
            .unwrap_or_else(|| RealtimeCheckpointRecord {
                tenant_id: String::new(),
                organization_id: String::new(),
                principal_kind: String::new(),
                principal_id: String::new(),
                device_id: String::new(),
                latest_realtime_seq: acked_through_seq,
                acked_through_seq,
                trimmed_through_seq: acked_through_seq,
                capacity_trimmed_event_count: 0,
                capacity_trimmed_through_seq: 0,
                last_capacity_trimmed_at: None,
                updated_at: updated_at.into(),
            })
    }
}

#[derive(Clone)]
pub struct PostgresRealtimeSubscriptionStore {
    pool: PostgresRealtimePool,
}

impl PostgresRealtimeSubscriptionStore {
    pub fn connect(config: PostgresRealtimeConfig) -> Result<Self, ContractError> {
        config.connect_pool().map(Self::from_pool)
    }

    pub fn from_pool(pool: PostgresRealtimePool) -> Self {
        Self { pool }
    }

    pub fn subscription_load_sql() -> &'static str {
        LOAD_REALTIME_SUBSCRIPTION_SQL
    }

    pub fn subscription_upsert_sql() -> &'static str {
        UPSERT_REALTIME_SUBSCRIPTION_SQL
    }

    pub fn subscription_clear_sql() -> &'static str {
        CLEAR_REALTIME_SUBSCRIPTION_SQL
    }

    pub fn subscription_clear_if_synced_sql() -> &'static str {
        CLEAR_REALTIME_SUBSCRIPTION_IF_SYNCED_AT_OR_BEFORE_SQL
    }

    pub fn subscription_scope_clear_sql() -> &'static str {
        CLEAR_REALTIME_SUBSCRIPTION_SCOPES_SQL
    }

    pub fn subscription_scope_replace_sql() -> &'static str {
        REPLACE_REALTIME_SUBSCRIPTION_SCOPES_SQL
    }

    pub fn matching_subscriptions_sql() -> &'static str {
        LOAD_MATCHING_REALTIME_SUBSCRIPTIONS_SQL
    }
}

#[derive(Clone)]
pub struct PostgresRealtimeDisconnectFenceStore {
    pool: PostgresRealtimePool,
}

impl PostgresRealtimeDisconnectFenceStore {
    pub fn connect(config: PostgresRealtimeConfig) -> Result<Self, ContractError> {
        config.connect_pool().map(Self::from_pool)
    }

    pub fn from_pool(pool: PostgresRealtimePool) -> Self {
        Self { pool }
    }

    pub fn fence_load_sql() -> &'static str {
        LOAD_REALTIME_DISCONNECT_FENCE_SQL
    }

    pub fn fence_upsert_sql() -> &'static str {
        UPSERT_REALTIME_DISCONNECT_FENCE_SQL
    }

    pub fn fence_clear_sql() -> &'static str {
        CLEAR_REALTIME_DISCONNECT_FENCE_SQL
    }

    pub fn fence_clear_if_matches_sql() -> &'static str {
        CLEAR_REALTIME_DISCONNECT_FENCE_IF_MATCHES_SQL
    }

    pub fn fence_clear_disconnected_at_or_before_sql() -> &'static str {
        CLEAR_REALTIME_DISCONNECT_FENCE_DISCONNECTED_AT_OR_BEFORE_SQL
    }
}

impl RealtimeDisconnectFenceStore for PostgresRealtimeDisconnectFenceStore {
    fn load_fence(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_kind: &str,
        principal_id: &str,
        device_id: &str,
    ) -> Result<Option<RealtimeDisconnectFenceRecord>, ContractError> {
        let pool = self.pool.clone();
        let tenant_id = tenant_id.to_owned();
        let organization_id = organization_id.to_owned();
        let principal_kind = principal_kind.to_owned();
        let principal_id = principal_id.to_owned();
        let device_id = device_id.to_owned();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "get disconnect fence connection")?;
            client
                .query_opt(
                    LOAD_REALTIME_DISCONNECT_FENCE_SQL,
                    &[&tenant_id, &organization_id, &principal_kind, &principal_id, &device_id],
                )
                .map_err(|error| postgres_unavailable("load disconnect fence", error))?
                .map(disconnect_fence_from_row)
                .transpose()
        })
    }

    fn save_fence(&self, record: RealtimeDisconnectFenceRecord) -> Result<(), ContractError> {
        validate_disconnect_fence_for_write(&record)?;
        let disconnected_at = parse_utc("disconnected_at", record.disconnected_at.as_str())?;
        let payload_json = disconnect_fence_payload_json(&record)?;
        let payload_value = postgres_jsonb_value("disconnect_fence.payload_json", &payload_json)?;
        let payload_hash = postgres_realtime_payload_hash(payload_json.as_str());
        let pool = self.pool.clone();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "get disconnect fence connection")?;
            let statement_timestamp = Utc::now();
            client
                .execute(
                    UPSERT_REALTIME_DISCONNECT_FENCE_SQL,
                    &[
                        &record.tenant_id,
                        &record.organization_id,
                        &record.principal_kind,
                        &record.principal_id,
                        &record.device_id,
                        &record.session_id,
                        &record.owner_node_id,
                        &disconnected_at,
                        &record.fence_token,
                        &payload_value,
                        &payload_hash,
                        &statement_timestamp,
                        &statement_timestamp,
                    ],
                )
                .map_err(|error| postgres_unavailable("upsert disconnect fence", error))?;
            Ok(())
        })
    }

    fn clear_fence(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_kind: &str,
        principal_id: &str,
        device_id: &str,
    ) -> Result<bool, ContractError> {
        let pool = self.pool.clone();
        let tenant_id = tenant_id.to_owned();
        let organization_id = organization_id.to_owned();
        let principal_kind = principal_kind.to_owned();
        let principal_id = principal_id.to_owned();
        let device_id = device_id.to_owned();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "get disconnect fence connection")?;
            let deleted = client
                .execute(
                    CLEAR_REALTIME_DISCONNECT_FENCE_SQL,
                    &[&tenant_id, &organization_id, &principal_kind, &principal_id, &device_id],
                )
                .map_err(|error| postgres_unavailable("clear disconnect fence", error))?;
            Ok(deleted > 0)
        })
    }

    fn clear_fence_disconnected_at_or_before(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_kind: &str,
        principal_id: &str,
        device_id: &str,
        cutoff_disconnected_at: &str,
    ) -> Result<bool, ContractError> {
        let cutoff_disconnected_at = parse_utc("cutoff_disconnected_at", cutoff_disconnected_at)?;
        let pool = self.pool.clone();
        let tenant_id = tenant_id.to_owned();
        let organization_id = organization_id.to_owned();
        let principal_kind = principal_kind.to_owned();
        let principal_id = principal_id.to_owned();
        let device_id = device_id.to_owned();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "get disconnect fence connection")?;
            let deleted = client
                .execute(
                    CLEAR_REALTIME_DISCONNECT_FENCE_DISCONNECTED_AT_OR_BEFORE_SQL,
                    &[
                        &tenant_id,
                        &organization_id,
                        &principal_kind,
                        &principal_id,
                        &device_id,
                        &cutoff_disconnected_at,
                    ],
                )
                .map_err(|error| {
                    postgres_unavailable("clear disconnect fence by timestamp", error)
                })?;
            Ok(deleted > 0)
        })
    }

    fn clear_fence_if_matches(
        &self,
        expected: &RealtimeDisconnectFenceRecord,
    ) -> Result<bool, ContractError> {
        let expected = expected.clone();
        let pool = self.pool.clone();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "get disconnect fence connection")?;
            let deleted = client
                .execute(
                    CLEAR_REALTIME_DISCONNECT_FENCE_IF_MATCHES_SQL,
                    &[
                        &expected.tenant_id,
                        &expected.organization_id,
                        &expected.principal_kind,
                        &expected.principal_id,
                        &expected.device_id,
                        &expected.fence_token,
                    ],
                )
                .map_err(|error| postgres_unavailable("clear matching disconnect fence", error))?;
            Ok(deleted > 0)
        })
    }
}

impl PresenceStateStore for PostgresRealtimePresenceStateStore {
    fn load_state(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_kind: &str,
        principal_id: &str,
        device_id: &str,
    ) -> Result<Option<PresenceStateRecord>, ContractError> {
        let pool = self.pool.clone();
        let tenant_id = tenant_id.to_owned();
        let organization_id = organization_id.to_owned();
        let principal_kind = principal_kind.to_owned();
        let principal_id = principal_id.to_owned();
        let device_id = device_id.to_owned();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "get presence connection")?;
            client
                .query_opt(
                    LOAD_PRESENCE_STATE_SQL,
                    &[
                        &tenant_id,
                        &organization_id,
                        &principal_kind,
                        &principal_id,
                        &device_id,
                    ],
                )
                .map_err(|error| postgres_unavailable("load presence state", error))?
                .map(presence_state_from_row)
                .transpose()
        })
    }

    fn save_state(&self, record: PresenceStateRecord) -> Result<(), ContractError> {
        validate_presence_state_for_write(&record)?;
        let last_sync_seq = u64_to_i64("last_sync_seq", record.presence.last_sync_seq)?;
        let last_resume_at =
            parse_optional_utc("last_resume_at", record.presence.last_resume_at.as_deref())?;
        let last_seen_at =
            parse_optional_utc("last_seen_at", record.presence.last_seen_at.as_deref())?;
        let updated_at = parse_utc("updated_at", record.updated_at.as_str())?;
        let payload_json = presence_payload_json(&record)?;
        let payload_value = postgres_jsonb_value("presence.payload_json", &payload_json)?;
        let payload_hash = postgres_realtime_payload_hash(payload_json.as_str());
        let pool = self.pool.clone();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "get presence connection")?;
            let created_at = Utc::now();
            client
                .execute(
                    UPSERT_PRESENCE_STATE_SQL,
                    &[
                        &record.tenant_id,
                        &record.organization_id,
                        &record.principal_kind,
                        &record.principal_id,
                        &record.device_id,
                        &record.presence.session_id,
                        &record.presence.status.as_str(),
                        &last_sync_seq,
                        &last_resume_at,
                        &last_seen_at,
                        &payload_value,
                        &payload_hash,
                        &record.resume_required,
                        &created_at,
                        &updated_at,
                    ],
                )
                .map_err(|error| postgres_unavailable("upsert presence state", error))?;
            Ok(())
        })
    }

    fn list_states_for_principal(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_kind: &str,
        principal_id: &str,
    ) -> Result<Vec<PresenceStateRecord>, ContractError> {
        let pool = self.pool.clone();
        let tenant_id = tenant_id.to_owned();
        let organization_id = organization_id.to_owned();
        let principal_kind = principal_kind.to_owned();
        let principal_id = principal_id.to_owned();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "get presence connection")?;
            client
                .query(
                    LIST_PRESENCE_STATES_FOR_PRINCIPAL_SQL,
                    &[
                        &tenant_id,
                        &organization_id,
                        &principal_kind,
                        &principal_id,
                    ],
                )
                .map_err(|error| postgres_unavailable("list presence states for principal", error))?
                .into_iter()
                .map(presence_state_from_row)
                .collect()
        })
    }

    fn list_online_states_seen_at_or_before(
        &self,
        cutoff_seen_at: &str,
        limit: usize,
    ) -> Result<Vec<PresenceStateRecord>, ContractError> {
        if limit == 0 {
            return Ok(Vec::new());
        }
        let cutoff_seen_at = parse_utc("cutoff_seen_at", cutoff_seen_at)?;
        let limit = i64::try_from(limit).map_err(|_| {
            ContractError::Conflict(format!(
                "postgres realtime presence limit {limit} exceeds PostgreSQL bigint maximum {}",
                i64::MAX
            ))
        })?;
        let pool = self.pool.clone();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "get presence connection")?;
            client
                .query(
                    LIST_STALE_ONLINE_PRESENCE_STATES_SQL,
                    &[&cutoff_seen_at, &limit],
                )
                .map_err(|error| postgres_unavailable("list stale online presence states", error))?
                .into_iter()
                .map(presence_state_from_row)
                .collect()
        })
    }

    fn expire_online_state_if_seen_at_or_before(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_kind: &str,
        principal_id: &str,
        device_id: &str,
        cutoff_seen_at: &str,
        expired_at: &str,
    ) -> Result<Option<PresenceStateRecord>, ContractError> {
        let Some(current) = self.load_state(
            tenant_id,
            organization_id,
            principal_kind,
            principal_id,
            device_id,
        )?
        else {
            return Ok(None);
        };
        if !current.is_online_seen_at_or_before(cutoff_seen_at) {
            return Ok(None);
        }
        let expired = current.into_expired_offline(expired_at);
        let cutoff_seen_at = parse_utc("cutoff_seen_at", cutoff_seen_at)?;
        let expired_at = parse_utc("expired_at", expired_at)?;
        let payload_json = presence_payload_json(&expired)?;
        let payload_value = postgres_jsonb_value("presence.expired_payload_json", &payload_json)?;
        let payload_hash = postgres_realtime_payload_hash(payload_json.as_str());
        let pool = self.pool.clone();
        let tenant_id = tenant_id.to_owned();
        let organization_id = organization_id.to_owned();
        let principal_kind = principal_kind.to_owned();
        let principal_id = principal_id.to_owned();
        let device_id = device_id.to_owned();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "get presence connection")?;
            client
                .query_opt(
                    EXPIRE_STALE_ONLINE_PRESENCE_STATE_SQL,
                    &[
                        &tenant_id,
                        &organization_id,
                        &principal_kind,
                        &principal_id,
                        &device_id,
                        &cutoff_seen_at,
                        &expired_at,
                        &payload_value,
                        &payload_hash,
                    ],
                )
                .map_err(|error| postgres_unavailable("expire stale online presence state", error))?
                .map(presence_state_from_row)
                .transpose()
        })
    }
}

impl RealtimeSubscriptionStore for PostgresRealtimeSubscriptionStore {
    fn load_subscriptions(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_kind: &str,
        principal_id: &str,
        device_id: &str,
    ) -> Result<Option<RealtimeSubscriptionRecord>, ContractError> {
        let pool = self.pool.clone();
        let tenant_id = tenant_id.to_owned();
        let organization_id = organization_id.to_owned();
        let principal_kind = principal_kind.to_owned();
        let principal_id = principal_id.to_owned();
        let device_id = device_id.to_owned();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "get subscription connection")?;
            let client_route_scope_key = postgres_realtime_client_route_scope_key(
                tenant_id.as_str(),
                organization_id.as_str(),
                principal_kind.as_str(),
                principal_id.as_str(),
                device_id.as_str(),
            );
            client
                .query_opt(
                    LOAD_REALTIME_SUBSCRIPTION_SQL,
                    &[&tenant_id, &organization_id, &client_route_scope_key],
                )
                .map_err(|error| postgres_unavailable("load subscription", error))?
                .map(subscription_from_row)
                .transpose()
        })
    }

    fn load_matching_subscriptions(
        &self,
        query: RealtimeMatchingSubscriptionQuery<'_>,
    ) -> Result<Vec<RealtimeSubscriptionRecord>, ContractError> {
        if query.candidate_device_ids.is_empty() {
            return Ok(Vec::new());
        }
        let pool = self.pool.clone();
        let organization_id = query.organization_id.to_owned();
        let tenant_id = query.tenant_id.to_owned();
        let principal_kind = query.principal_kind.to_owned();
        let principal_id = query.principal_id.to_owned();
        let scope_type = query.scope_type.to_owned();
        let scope_id = query.scope_id.to_owned();
        let event_type = query.event_type.to_owned();
        let candidate_device_ids = query
            .candidate_device_ids
            .iter()
            .map(|item| (*item).to_owned())
            .collect::<Vec<_>>();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "get subscription connection")?;
            client
                .query(
                    LOAD_MATCHING_REALTIME_SUBSCRIPTIONS_SQL,
                    &[
                        &tenant_id,
                        &organization_id,
                        &principal_kind,
                        &principal_id,
                        &scope_type,
                        &scope_id,
                        &event_type,
                        &candidate_device_ids,
                    ],
                )
                .map_err(|error| postgres_unavailable("load matching subscriptions", error))?
                .into_iter()
                .map(subscription_from_row)
                .collect()
        })
    }

    fn save_subscriptions(&self, record: RealtimeSubscriptionRecord) -> Result<(), ContractError> {
        validate_subscription_for_write(&record)?;
        let pool = self.pool.clone();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "get subscription connection")?;
            let mut transaction = client.transaction().map_err(|error| {
                postgres_unavailable("begin subscription save transaction", error)
            })?;
            let client_route_scope_key = postgres_realtime_client_route_scope_key(
                record.tenant_id.as_str(),
                record.organization_id.as_str(),
                record.principal_kind.as_str(),
                record.principal_id.as_str(),
                record.device_id.as_str(),
            );
            let statement_timestamp = Utc::now();
            execute_subscription_upsert(
                &mut transaction,
                &record,
                client_route_scope_key.as_str(),
                &statement_timestamp,
            )?;
            let synced_at = parse_utc("synced_at", record.synced_at.as_str())?;
            transaction
                .execute(
                    CLEAR_REALTIME_SUBSCRIPTION_SCOPES_SQL,
                    &[&record.tenant_id, &record.organization_id, &client_route_scope_key, &synced_at],
                )
                .map_err(|error| postgres_unavailable("clear subscription scopes", error))?;
            for (scope_type, scope_id, event_type) in subscription_scope_rows(&record) {
                transaction
                    .execute(
                        REPLACE_REALTIME_SUBSCRIPTION_SCOPES_SQL,
                        &[
                            &record.tenant_id,
                            &record.organization_id,
                            &record.principal_kind,
                            &record.principal_id,
                            &scope_type,
                            &scope_id,
                            &event_type,
                            &client_route_scope_key,
                            &record.device_id,
                            &synced_at,
                            &statement_timestamp,
                            &statement_timestamp,
                        ],
                    )
                    .map_err(|error| postgres_unavailable("replace subscription scope", error))?;
            }
            transaction.commit().map_err(|error| {
                postgres_unavailable("commit subscription save transaction", error)
            })?;
            Ok(())
        })
    }

    fn clear_subscriptions(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_kind: &str,
        principal_id: &str,
        device_id: &str,
    ) -> Result<bool, ContractError> {
        let pool = self.pool.clone();
        let tenant_id = tenant_id.to_owned();
        let organization_id = organization_id.to_owned();
        let principal_kind = principal_kind.to_owned();
        let principal_id = principal_id.to_owned();
        let device_id = device_id.to_owned();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "get subscription connection")?;
            let client_route_scope_key = postgres_realtime_client_route_scope_key(
                tenant_id.as_str(),
                organization_id.as_str(),
                principal_kind.as_str(),
                principal_id.as_str(),
                device_id.as_str(),
            );
            let deleted = client
                .execute(
                    CLEAR_REALTIME_SUBSCRIPTION_SQL,
                    &[&tenant_id, &organization_id, &client_route_scope_key],
                )
                .map_err(|error| postgres_unavailable("clear subscription", error))?;
            Ok(deleted > 0)
        })
    }

    fn clear_subscriptions_synced_at_or_before(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_kind: &str,
        principal_id: &str,
        device_id: &str,
        cutoff_synced_at: &str,
    ) -> Result<bool, ContractError> {
        let cutoff_synced_at = parse_utc("cutoff_synced_at", cutoff_synced_at)?;
        let pool = self.pool.clone();
        let tenant_id = tenant_id.to_owned();
        let organization_id = organization_id.to_owned();
        let principal_kind = principal_kind.to_owned();
        let principal_id = principal_id.to_owned();
        let device_id = device_id.to_owned();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "get subscription connection")?;
            let client_route_scope_key = postgres_realtime_client_route_scope_key(
                tenant_id.as_str(),
                organization_id.as_str(),
                principal_kind.as_str(),
                principal_id.as_str(),
                device_id.as_str(),
            );
            let deleted = client
                .execute(
                    CLEAR_REALTIME_SUBSCRIPTION_IF_SYNCED_AT_OR_BEFORE_SQL,
                    &[&tenant_id, &organization_id, &client_route_scope_key, &cutoff_synced_at],
                )
                .map_err(|error| postgres_unavailable("clear subscription by synced_at", error))?;
            Ok(deleted > 0)
        })
    }
}

impl RealtimeEventWindowStore for PostgresRealtimeEventWindowStore {
    fn load_window(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_kind: &str,
        principal_id: &str,
        device_id: &str,
    ) -> Result<Option<RealtimeEventWindowRecord>, ContractError> {
        let pool = self.pool.clone();
        let tenant_id = tenant_id.to_owned();
        let organization_id = organization_id.to_owned();
        let principal_kind = principal_kind.to_owned();
        let principal_id = principal_id.to_owned();
        let device_id = device_id.to_owned();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "get event window connection")?;
            let client_route_scope_key = postgres_realtime_client_route_scope_key(
                tenant_id.as_str(),
                organization_id.as_str(),
                principal_kind.as_str(),
                principal_id.as_str(),
                device_id.as_str(),
            );
            let mut transaction = client.transaction().map_err(|error| {
                postgres_unavailable("begin event window load transaction", error)
            })?;
            let checkpoint_row = transaction
                .query_opt(
                    LOAD_REALTIME_CHECKPOINT_SQL,
                    &[&tenant_id, &organization_id, &client_route_scope_key],
                )
                .map_err(|error| postgres_unavailable("load event window checkpoint", error))?;
            let Some(checkpoint) = checkpoint_row.map(checkpoint_from_row).transpose()? else {
                transaction.commit().map_err(|error| {
                    postgres_unavailable("commit empty event window load", error)
                })?;
                return Ok(None);
            };
            let after_seq = u64_to_i64("trimmed_through_seq", checkpoint.trimmed_through_seq)?;
            let limit = i64::MAX;
            let events = transaction
                .query(
                    LIST_REALTIME_CLIENT_ROUTE_EVENTS_SQL,
                    &[&tenant_id, &organization_id, &client_route_scope_key, &after_seq, &limit],
                )
                .map_err(|error| postgres_unavailable("list event window events", error))?
                .into_iter()
                .map(realtime_event_from_row)
                .collect::<Result<Vec<_>, ContractError>>()?;
            transaction
                .commit()
                .map_err(|error| postgres_unavailable("commit event window load", error))?;
            Ok(Some(
                RealtimeEventWindowRecord {
                    tenant_id: checkpoint.tenant_id,
                    organization_id: checkpoint.organization_id,
                    principal_kind: checkpoint.principal_kind,
                    principal_id: checkpoint.principal_id,
                    device_id: checkpoint.device_id,
                    events,
                    trimmed_through_seq: checkpoint.trimmed_through_seq,
                    capacity_trimmed_event_count: checkpoint.capacity_trimmed_event_count,
                    capacity_trimmed_through_seq: checkpoint.capacity_trimmed_through_seq,
                    last_capacity_trimmed_at: checkpoint.last_capacity_trimmed_at,
                    updated_at: checkpoint.updated_at,
                }
                .normalized(),
            ))
        })
    }

    fn save_windows(&self, records: Vec<RealtimeEventWindowRecord>) -> Result<(), ContractError> {
        let records = records
            .into_iter()
            .map(RealtimeEventWindowRecord::normalized)
            .map(|record| {
                validate_event_window_for_write(&record)?;
                Ok(record)
            })
            .collect::<Result<Vec<_>, ContractError>>()?;
        if records.is_empty() {
            return Ok(());
        }

        let pool = self.pool.clone();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "get event window connection")?;
            let mut transaction = client.transaction().map_err(|error| {
                postgres_unavailable("begin event window save transaction", error)
            })?;
            let statement_timestamp = Utc::now();
            for record in records {
                let client_route_scope_key = postgres_realtime_client_route_scope_key(
                    record.tenant_id.as_str(),
                    record.organization_id.as_str(),
                    record.principal_kind.as_str(),
                    record.principal_id.as_str(),
                    record.device_id.as_str(),
                );
                transaction
                    .execute(
                        CLEAR_REALTIME_CLIENT_ROUTE_EVENTS_SQL,
                        &[&record.tenant_id, &record.organization_id, &client_route_scope_key],
                    )
                    .map_err(|error| {
                        postgres_unavailable("clear previous event window events", error)
                    })?;
                for event in &record.events {
                    execute_client_route_event_upsert(
                        &mut transaction,
                        event,
                        record.organization_id.as_str(),
                        record.principal_kind.as_str(),
                        client_route_scope_key.as_str(),
                        &statement_timestamp,
                    )?;
                }
                let checkpoint = checkpoint_from_window(&record);
                execute_checkpoint_upsert(
                    &mut transaction,
                    &checkpoint,
                    client_route_scope_key.as_str(),
                    &statement_timestamp,
                )?;
            }
            transaction.commit().map_err(|error| {
                postgres_unavailable("commit event window save transaction", error)
            })?;
            Ok(())
        })
    }

    fn clear_window(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_kind: &str,
        principal_id: &str,
        device_id: &str,
    ) -> Result<bool, ContractError> {
        let pool = self.pool.clone();
        let tenant_id = tenant_id.to_owned();
        let organization_id = organization_id.to_owned();
        let principal_kind = principal_kind.to_owned();
        let principal_id = principal_id.to_owned();
        let device_id = device_id.to_owned();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "get event window connection")?;
            let client_route_scope_key = postgres_realtime_client_route_scope_key(
                tenant_id.as_str(),
                organization_id.as_str(),
                principal_kind.as_str(),
                principal_id.as_str(),
                device_id.as_str(),
            );
            let deleted = client
                .execute(
                    CLEAR_REALTIME_CLIENT_ROUTE_EVENTS_SQL,
                    &[&tenant_id, &organization_id, &client_route_scope_key],
                )
                .map_err(|error| postgres_unavailable("clear event window events", error))?;
            Ok(deleted > 0)
        })
    }

    fn diagnostics_snapshot(
        &self,
    ) -> Result<RealtimeEventWindowDiagnosticsSnapshot, ContractError> {
        let pool = self.pool.clone();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "get event window connection")?;
            let diagnostics_row = client
                .query_one(LOAD_REALTIME_EVENT_WINDOW_DIAGNOSTICS_SQL, &[])
                .map_err(|error| postgres_unavailable("load event window diagnostics", error))?;
            let high_risk_windows = client
                .query(LIST_REALTIME_EVENT_WINDOW_HIGH_RISK_WINDOWS_SQL, &[])
                .map_err(|error| postgres_unavailable("list high risk event windows", error))?
                .into_iter()
                .map(high_risk_window_from_row)
                .collect::<Result<Vec<_>, ContractError>>()?;
            diagnostics_from_row(diagnostics_row, high_risk_windows)
        })
    }

    fn trim_window(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_kind: &str,
        principal_id: &str,
        device_id: &str,
        acked_through_seq: u64,
    ) -> Result<(), ContractError> {
        let acked_through_seq_i64 = u64_to_i64("acked_through_seq", acked_through_seq)?;
        let pool = self.pool.clone();
        let tenant_id = tenant_id.to_owned();
        let organization_id = organization_id.to_owned();
        let principal_kind = principal_kind.to_owned();
        let principal_id = principal_id.to_owned();
        let device_id = device_id.to_owned();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "get event window connection")?;
            let client_route_scope_key = postgres_realtime_client_route_scope_key(
                tenant_id.as_str(),
                organization_id.as_str(),
                principal_kind.as_str(),
                principal_id.as_str(),
                device_id.as_str(),
            );
            let mut transaction = client.transaction().map_err(|error| {
                postgres_unavailable("begin event window trim transaction", error)
            })?;
            transaction
                .execute(
                    TRIM_REALTIME_CLIENT_ROUTE_EVENTS_SQL,
                    &[&tenant_id, &organization_id, &client_route_scope_key, &acked_through_seq_i64],
                )
                .map_err(|error| postgres_unavailable("trim event window events", error))?;
            let existing = transaction
                .query_opt(
                    LOAD_REALTIME_CHECKPOINT_SQL,
                    &[&tenant_id, &organization_id, &client_route_scope_key],
                )
                .map_err(|error| postgres_unavailable("load checkpoint for trim", error))?
                .map(checkpoint_from_row)
                .transpose()?;
            let updated_at = Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true);
            let checkpoint = existing
                .map(|record| {
                    Self::checkpoint_after_trim(
                        Some(record),
                        acked_through_seq,
                        updated_at.as_str(),
                    )
                })
                .unwrap_or_else(|| RealtimeCheckpointRecord {
                    tenant_id: tenant_id.clone(),
                    organization_id: organization_id.clone(),
                    principal_kind: principal_kind.clone(),
                    principal_id: principal_id.clone(),
                    device_id: device_id.clone(),
                    ..Self::checkpoint_after_trim(None, acked_through_seq, updated_at.as_str())
                });
            execute_checkpoint_upsert(
                &mut transaction,
                &checkpoint,
                &client_route_scope_key,
                &Utc::now(),
            )?;
            transaction.commit().map_err(|error| {
                postgres_unavailable("commit event window trim transaction", error)
            })?;
            Ok(())
        })
    }
}

impl PostgresRealtimeCheckpointStore {
    pub fn connect(config: PostgresRealtimeConfig) -> Result<Self, ContractError> {
        config.connect_pool().map(Self::from_pool)
    }

    pub fn from_pool(pool: PostgresRealtimePool) -> Self {
        Self { pool }
    }

    pub fn checkpoint_load_sql() -> &'static str {
        LOAD_REALTIME_CHECKPOINT_SQL
    }

    pub fn checkpoint_upsert_sql() -> &'static str {
        UPSERT_REALTIME_CHECKPOINT_SQL
    }

    pub fn merge_checkpoint_for_write(
        current: Option<RealtimeCheckpointRecord>,
        next: RealtimeCheckpointRecord,
    ) -> RealtimeCheckpointRecord {
        current
            .map(|previous| previous.merge_monotonic(next.clone()))
            .unwrap_or_else(|| next.normalized())
    }

    pub fn validate_checkpoint_for_write(
        record: &RealtimeCheckpointRecord,
    ) -> Result<(), ContractError> {
        if record.acked_through_seq > record.latest_realtime_seq {
            return Err(checkpoint_conflict(format!(
                "acked_through_seq={} must be <= latest_realtime_seq={}",
                record.acked_through_seq, record.latest_realtime_seq
            )));
        }
        if record.trimmed_through_seq > record.latest_realtime_seq {
            return Err(checkpoint_conflict(format!(
                "trimmed_through_seq={} must be <= latest_realtime_seq={}",
                record.trimmed_through_seq, record.latest_realtime_seq
            )));
        }
        if record.capacity_trimmed_through_seq > record.trimmed_through_seq {
            return Err(checkpoint_conflict(format!(
                "capacity_trimmed_through_seq={} must be <= trimmed_through_seq={}",
                record.capacity_trimmed_through_seq, record.trimmed_through_seq
            )));
        }
        match (
            record.capacity_trimmed_event_count,
            record.capacity_trimmed_through_seq,
            record.last_capacity_trimmed_at.as_deref(),
        ) {
            (0, 0, None) => {}
            (count, through_seq, Some(last_trimmed_at)) if count > 0 && through_seq > 0 => {
                parse_utc("last_capacity_trimmed_at", last_trimmed_at)?;
            }
            _ => {
                return Err(checkpoint_conflict(
                    "capacity trim metadata must either be all empty or include positive count, positive through seq, and last_capacity_trimmed_at",
                ));
            }
        }
        parse_utc("updated_at", record.updated_at.as_str())?;
        u64_to_i64("latest_realtime_seq", record.latest_realtime_seq)?;
        u64_to_i64("acked_through_seq", record.acked_through_seq)?;
        u64_to_i64("trimmed_through_seq", record.trimmed_through_seq)?;
        u64_to_i64(
            "capacity_trimmed_event_count",
            record.capacity_trimmed_event_count,
        )?;
        u64_to_i64(
            "capacity_trimmed_through_seq",
            record.capacity_trimmed_through_seq,
        )?;
        Ok(())
    }
}

impl RealtimeCheckpointStore for PostgresRealtimeCheckpointStore {
    fn load_checkpoint(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_kind: &str,
        principal_id: &str,
        device_id: &str,
    ) -> Result<Option<RealtimeCheckpointRecord>, ContractError> {
        let pool = self.pool.clone();
        let tenant_id = tenant_id.to_owned();
        let organization_id = organization_id.to_owned();
        let principal_kind = principal_kind.to_owned();
        let principal_id = principal_id.to_owned();
        let device_id = device_id.to_owned();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "get checkpoint connection")?;
            let client_route_scope_key = postgres_realtime_client_route_scope_key(
                tenant_id.as_str(),
                organization_id.as_str(),
                principal_kind.as_str(),
                principal_id.as_str(),
                device_id.as_str(),
            );
            let row = client
                .query_opt(
                    LOAD_REALTIME_CHECKPOINT_SQL,
                    &[&tenant_id, &organization_id, &client_route_scope_key],
                )
                .map_err(|error| postgres_unavailable("load checkpoint", error))?;
            row.map(checkpoint_from_row).transpose()
        })
    }

    fn save_checkpoints(
        &self,
        records: Vec<RealtimeCheckpointRecord>,
    ) -> Result<(), ContractError> {
        let records = records
            .into_iter()
            .map(RealtimeCheckpointRecord::normalized)
            .map(|record| {
                Self::validate_checkpoint_for_write(&record)?;
                Ok(record)
            })
            .collect::<Result<Vec<_>, ContractError>>()?;

        if records.is_empty() {
            return Ok(());
        }

        let pool = self.pool.clone();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "get checkpoint connection")?;
            let mut transaction = client
                .transaction()
                .map_err(|error| postgres_unavailable("begin checkpoint transaction", error))?;
            let created_at = Utc::now();

            for record in records {
                let client_route_scope_key = postgres_realtime_client_route_scope_key(
                    record.tenant_id.as_str(),
                    record.organization_id.as_str(),
                    record.principal_kind.as_str(),
                    record.principal_id.as_str(),
                    record.device_id.as_str(),
                );
                let latest_realtime_seq =
                    u64_to_i64("latest_realtime_seq", record.latest_realtime_seq)?;
                let acked_through_seq = u64_to_i64("acked_through_seq", record.acked_through_seq)?;
                let trimmed_through_seq =
                    u64_to_i64("trimmed_through_seq", record.trimmed_through_seq)?;
                let capacity_trimmed_event_count = u64_to_i64(
                    "capacity_trimmed_event_count",
                    record.capacity_trimmed_event_count,
                )?;
                let capacity_trimmed_through_seq = u64_to_i64(
                    "capacity_trimmed_through_seq",
                    record.capacity_trimmed_through_seq,
                )?;
                let last_capacity_trimmed_at = parse_optional_utc(
                    "last_capacity_trimmed_at",
                    record.last_capacity_trimmed_at.as_deref(),
                )?;
                let updated_at = parse_utc("updated_at", record.updated_at.as_str())?;

                transaction
                    .execute(
                        UPSERT_REALTIME_CHECKPOINT_SQL,
                        &[
                            &record.tenant_id,
                            &record.organization_id,
                            &client_route_scope_key,
                            &record.principal_kind,
                            &record.principal_id,
                            &record.device_id,
                            &latest_realtime_seq,
                            &acked_through_seq,
                            &trimmed_through_seq,
                            &capacity_trimmed_event_count,
                            &capacity_trimmed_through_seq,
                            &last_capacity_trimmed_at,
                            &created_at,
                            &updated_at,
                        ],
                    )
                    .map_err(|error| postgres_unavailable("upsert checkpoint", error))?;
            }

            transaction
                .commit()
                .map_err(|error| postgres_unavailable("commit checkpoint transaction", error))?;
            Ok(())
        })
    }
}

pub fn postgres_realtime_client_route_scope_key(
    tenant_id: &str,
    organization_id: &str,
    principal_kind: &str,
    principal_id: &str,
    device_id: &str,
) -> String {
    im_platform_contracts::realtime_client_route_scope_key(
        tenant_id,
        organization_id,
        principal_kind,
        principal_id,
        device_id,
    )
}

fn checkpoint_from_row(row: Row) -> Result<RealtimeCheckpointRecord, ContractError> {
    Ok(RealtimeCheckpointRecord {
        tenant_id: row.get("tenant_id"),
        organization_id: row.get("organization_id"),
        principal_kind: row.get("principal_kind"),
        principal_id: row.get("principal_id"),
        device_id: row.get("device_id"),
        latest_realtime_seq: i64_to_u64("latest_realtime_seq", row.get("latest_realtime_seq"))?,
        acked_through_seq: i64_to_u64("acked_through_seq", row.get("acked_through_seq"))?,
        trimmed_through_seq: i64_to_u64("trimmed_through_seq", row.get("trimmed_through_seq"))?,
        capacity_trimmed_event_count: i64_to_u64(
            "capacity_trimmed_event_count",
            row.get("capacity_trimmed_event_count"),
        )?,
        capacity_trimmed_through_seq: i64_to_u64(
            "capacity_trimmed_through_seq",
            row.get("capacity_trimmed_through_seq"),
        )?,
        last_capacity_trimmed_at: format_optional_utc_millis(row.get("last_capacity_trimmed_at")),
        updated_at: format_utc_millis(row.get("updated_at")),
    }
    .normalized())
}

fn checkpoint_from_window(record: &RealtimeEventWindowRecord) -> RealtimeCheckpointRecord {
    let latest_realtime_seq = record
        .events
        .iter()
        .map(|event| event.realtime_seq)
        .max()
        .unwrap_or(record.trimmed_through_seq);
    RealtimeCheckpointRecord {
        tenant_id: record.tenant_id.clone(),
        organization_id: record.organization_id.clone(),
        principal_kind: record.principal_kind.clone(),
        principal_id: record.principal_id.clone(),
        device_id: record.device_id.clone(),
        latest_realtime_seq,
        acked_through_seq: record.trimmed_through_seq.min(latest_realtime_seq),
        trimmed_through_seq: record.trimmed_through_seq,
        capacity_trimmed_event_count: record.capacity_trimmed_event_count,
        capacity_trimmed_through_seq: record.capacity_trimmed_through_seq,
        last_capacity_trimmed_at: record.last_capacity_trimmed_at.clone(),
        updated_at: record.updated_at.clone(),
    }
    .normalized()
}

fn validate_event_window_for_write(
    record: &RealtimeEventWindowRecord,
) -> Result<(), ContractError> {
    parse_utc("updated_at", record.updated_at.as_str())?;
    let normalized = record.clone().normalized();
    if normalized != *record {
        return Err(ContractError::Conflict(
            "postgres realtime event window record must be normalized before write".into(),
        ));
    }
    let checkpoint = checkpoint_from_window(record);
    PostgresRealtimeCheckpointStore::validate_checkpoint_for_write(&checkpoint)?;
    for event in &record.events {
        validate_realtime_event_window_identity(record, event)?;
        validate_realtime_event_for_write(event)?;
    }
    Ok(())
}

fn validate_realtime_event_window_identity(
    record: &RealtimeEventWindowRecord,
    event: &RealtimeEvent,
) -> Result<(), ContractError> {
    if event.tenant_id != record.tenant_id {
        return Err(ContractError::Conflict(format!(
            "postgres realtime event window event.tenant_id={} must match record tenant_id={}",
            event.tenant_id, record.tenant_id
        )));
    }
    if event.principal_id != record.principal_id {
        return Err(ContractError::Conflict(format!(
            "postgres realtime event window event.principal_id={} must match record principal_id={}",
            event.principal_id, record.principal_id
        )));
    }
    if event.device_id != record.device_id {
        return Err(ContractError::Conflict(format!(
            "postgres realtime event window event.device_id={} must match record device_id={}",
            event.device_id, record.device_id
        )));
    }
    Ok(())
}

fn validate_realtime_event_for_write(event: &RealtimeEvent) -> Result<(), ContractError> {
    if event.realtime_seq == 0 {
        return Err(ContractError::Conflict(
            "postgres realtime event realtime_seq must be positive".into(),
        ));
    }
    serde_json::from_str::<serde_json::Value>(event.payload.as_str()).map_err(|error| {
        ContractError::Conflict(format!(
            "postgres realtime event payload must be valid JSON: {error}"
        ))
    })?;
    parse_utc("occurred_at", event.occurred_at.as_str())?;
    u64_to_i64("realtime_seq", event.realtime_seq)?;
    Ok(())
}

fn execute_checkpoint_upsert(
    transaction: &mut Transaction<'_>,
    record: &RealtimeCheckpointRecord,
    client_route_scope_key: &str,
    created_at: &DateTime<Utc>,
) -> Result<(), ContractError> {
    let record = record.clone().normalized();
    PostgresRealtimeCheckpointStore::validate_checkpoint_for_write(&record)?;
    let latest_realtime_seq = u64_to_i64("latest_realtime_seq", record.latest_realtime_seq)?;
    let acked_through_seq = u64_to_i64("acked_through_seq", record.acked_through_seq)?;
    let trimmed_through_seq = u64_to_i64("trimmed_through_seq", record.trimmed_through_seq)?;
    let capacity_trimmed_event_count = u64_to_i64(
        "capacity_trimmed_event_count",
        record.capacity_trimmed_event_count,
    )?;
    let capacity_trimmed_through_seq = u64_to_i64(
        "capacity_trimmed_through_seq",
        record.capacity_trimmed_through_seq,
    )?;
    let last_capacity_trimmed_at = parse_optional_utc(
        "last_capacity_trimmed_at",
        record.last_capacity_trimmed_at.as_deref(),
    )?;
    let updated_at = parse_utc("updated_at", record.updated_at.as_str())?;
    transaction
        .execute(
            UPSERT_REALTIME_CHECKPOINT_SQL,
            &[
                &record.tenant_id,
                &record.organization_id,
                &client_route_scope_key,
                &record.principal_kind,
                &record.principal_id,
                &record.device_id,
                &latest_realtime_seq,
                &acked_through_seq,
                &trimmed_through_seq,
                &capacity_trimmed_event_count,
                &capacity_trimmed_through_seq,
                &last_capacity_trimmed_at,
                created_at,
                &updated_at,
            ],
        )
        .map_err(|error| postgres_unavailable("upsert checkpoint", error))?;
    Ok(())
}

fn execute_client_route_event_upsert(
    transaction: &mut Transaction<'_>,
    event: &RealtimeEvent,
    organization_id: &str,
    principal_kind: &str,
    client_route_scope_key: &str,
    statement_timestamp: &DateTime<Utc>,
) -> Result<(), ContractError> {
    validate_realtime_event_for_write(event)?;
    let realtime_seq = u64_to_i64("realtime_seq", event.realtime_seq)?;
    let occurred_at = parse_utc("occurred_at", event.occurred_at.as_str())?;
    let payload_hash = postgres_realtime_payload_hash(event.payload.as_str());
    let payload_value = postgres_jsonb_value("event.payload_json", event.payload.as_str())?;
    let retention_until = im_domain_core::retention::retention_until_from_class(
        "standard",
        event.occurred_at.as_str(),
    )
    .and_then(|value| parse_utc("retention_until", value.as_str()).ok());
    transaction
        .execute(
            UPSERT_REALTIME_CLIENT_ROUTE_EVENT_SQL,
            &[
                &event.tenant_id,
                &organization_id,
                &client_route_scope_key,
                &realtime_seq,
                &principal_kind,
                &event.principal_id,
                &event.device_id,
                &event.scope_type,
                &event.scope_id,
                &event.event_type,
                &event.delivery_class,
                &payload_value,
                &payload_hash,
                &occurred_at,
                statement_timestamp,
                &retention_until,
            ],
        )
        .map_err(|error| postgres_unavailable("upsert event window event", error))?;
    Ok(())
}

fn realtime_event_from_row(row: Row) -> Result<RealtimeEvent, ContractError> {
    Ok(RealtimeEvent {
        tenant_id: row.get("tenant_id"),
        principal_id: row.get("principal_id"),
        device_id: row.get("device_id"),
        realtime_seq: i64_to_u64("realtime_seq", row.get("realtime_seq"))?,
        scope_type: row.get("scope_type"),
        scope_id: row.get("scope_id"),
        event_type: row.get("event_type"),
        delivery_class: row.get("delivery_class"),
        payload: row.get("payload_json"),
        occurred_at: format_utc_millis(row.get("occurred_at")),
    })
}

fn diagnostics_from_row(
    row: Row,
    high_risk_windows: Vec<RealtimeEventWindowHighRiskRecord>,
) -> Result<RealtimeEventWindowDiagnosticsSnapshot, ContractError> {
    Ok(RealtimeEventWindowDiagnosticsSnapshot {
        client_route_window_count: i64_to_u64(
            "client_route_window_count",
            row.get("client_route_window_count"),
        )?,
        pending_event_count: i64_to_u64("pending_event_count", row.get("pending_event_count"))?,
        max_client_route_window_event_count: i64_to_u64(
            "max_client_route_window_event_count",
            row.get("max_client_route_window_event_count"),
        )?,
        max_trimmed_through_seq: i64_to_u64(
            "max_trimmed_through_seq",
            row.get("max_trimmed_through_seq"),
        )?,
        capacity_trimmed_event_count: i64_to_u64(
            "capacity_trimmed_event_count",
            row.get("capacity_trimmed_event_count"),
        )?,
        max_capacity_trimmed_through_seq: i64_to_u64(
            "max_capacity_trimmed_through_seq",
            row.get("max_capacity_trimmed_through_seq"),
        )?,
        last_capacity_trimmed_at: format_optional_utc_millis(row.get("last_capacity_trimmed_at")),
        oldest_pending_occurred_at: format_optional_utc_millis(
            row.get("oldest_pending_occurred_at"),
        ),
        high_risk_windows,
    })
}

fn high_risk_window_from_row(row: Row) -> Result<RealtimeEventWindowHighRiskRecord, ContractError> {
    Ok(RealtimeEventWindowHighRiskRecord {
        tenant_id: row.get("tenant_id"),
        organization_id: row.get("organization_id"),
        principal_kind: row.get("principal_kind"),
        principal_id: row.get("principal_id"),
        device_id: row.get("device_id"),
        pending_event_count: i64_to_u64("pending_event_count", row.get("pending_event_count"))?,
        trimmed_through_seq: i64_to_u64("trimmed_through_seq", row.get("trimmed_through_seq"))?,
        capacity_trimmed_event_count: i64_to_u64(
            "capacity_trimmed_event_count",
            row.get("capacity_trimmed_event_count"),
        )?,
        capacity_trimmed_through_seq: i64_to_u64(
            "capacity_trimmed_through_seq",
            row.get("capacity_trimmed_through_seq"),
        )?,
        last_capacity_trimmed_at: format_optional_utc_millis(row.get("last_capacity_trimmed_at")),
        oldest_pending_occurred_at: format_optional_utc_millis(
            row.get("oldest_pending_occurred_at"),
        ),
    })
}

pub fn postgres_realtime_payload_hash(payload: &str) -> String {
    format!("sha256:{}", sha256_hash(payload.as_bytes()))
}

fn postgres_jsonb_value(
    field: &'static str,
    payload: &str,
) -> Result<serde_json::Value, ContractError> {
    serde_json::from_str(payload).map_err(|error| {
        ContractError::Conflict(format!(
            "postgres realtime {field} must be valid JSONB payload: {error}"
        ))
    })
}

fn validate_subscription_for_write(
    record: &RealtimeSubscriptionRecord,
) -> Result<(), ContractError> {
    parse_utc("synced_at", record.synced_at.as_str())?;
    let _ = i32::try_from(record.items.len()).map_err(|_| {
        ContractError::Conflict(format!(
            "postgres realtime subscription item count {} exceeds PostgreSQL integer maximum {}",
            record.items.len(),
            i32::MAX
        ))
    })?;
    for item in &record.items {
        parse_utc("subscribed_at", item.subscribed_at.as_str())?;
    }
    Ok(())
}

fn execute_subscription_upsert(
    transaction: &mut Transaction<'_>,
    record: &RealtimeSubscriptionRecord,
    client_route_scope_key: &str,
    statement_timestamp: &DateTime<Utc>,
) -> Result<(), ContractError> {
    let subscriptions_json = serde_json::to_string(&record.items).map_err(|error| {
        ContractError::Conflict(format!(
            "postgres realtime subscription items must serialize to JSON: {error}"
        ))
    })?;
    let subscription_count = i32::try_from(record.items.len()).map_err(|_| {
        ContractError::Conflict(format!(
            "postgres realtime subscription item count {} exceeds PostgreSQL integer maximum {}",
            record.items.len(),
            i32::MAX
        ))
    })?;
    let synced_at = parse_utc("synced_at", record.synced_at.as_str())?;
    let subscriptions_value = postgres_jsonb_value(
        "subscription.subscriptions_json",
        subscriptions_json.as_str(),
    )?;
    transaction
        .execute(
            UPSERT_REALTIME_SUBSCRIPTION_SQL,
            &[
                &record.tenant_id.as_str(),
                &record.organization_id.as_str(),
                &client_route_scope_key,
                &record.principal_kind.as_str(),
                &record.principal_id.as_str(),
                &record.device_id.as_str(),
                &subscriptions_value,
                &subscription_count,
                &synced_at,
                statement_timestamp,
                statement_timestamp,
            ],
        )
        .map_err(|error| postgres_unavailable("upsert subscription", error))?;
    Ok(())
}

fn subscription_scope_rows(record: &RealtimeSubscriptionRecord) -> Vec<(String, String, String)> {
    record
        .items
        .iter()
        .flat_map(|subscription| {
            let event_types = if subscription.event_types.is_empty() {
                vec!["*".to_owned()]
            } else {
                subscription.event_types.clone()
            };
            event_types.into_iter().map(move |event_type| {
                (
                    subscription.scope_type.clone(),
                    subscription.scope_id.clone(),
                    event_type,
                )
            })
        })
        .collect()
}

fn subscription_from_row(row: Row) -> Result<RealtimeSubscriptionRecord, ContractError> {
    let subscriptions_json: String = row.get("subscriptions_json");
    let items = serde_json::from_str(subscriptions_json.as_str()).map_err(|error| {
        ContractError::Unavailable(format!(
            "postgres realtime subscription row has invalid subscriptions_json: {error}"
        ))
    })?;
    Ok(RealtimeSubscriptionRecord {
        tenant_id: row.get("tenant_id"),
        organization_id: row.get("organization_id"),
        principal_kind: row.get("principal_kind"),
        principal_id: row.get("principal_id"),
        device_id: row.get("device_id"),
        items,
        synced_at: format_utc_millis(row.get("synced_at")),
    })
}

fn validate_presence_state_for_write(record: &PresenceStateRecord) -> Result<(), ContractError> {
    if record.presence.tenant_id != record.tenant_id {
        return Err(ContractError::Conflict(
            "postgres realtime presence tenant_id must match nested presence tenant_id".into(),
        ));
    }
    if record.presence.principal_id != record.principal_id {
        return Err(ContractError::Conflict(
            "postgres realtime presence principal_id must match nested presence principal_id"
                .into(),
        ));
    }
    if record.presence.device_id != record.device_id {
        return Err(ContractError::Conflict(
            "postgres realtime presence device_id must match nested presence device_id".into(),
        ));
    }
    if matches!(record.presence.status, PresenceStatus::Online)
        && record.presence.last_seen_at.is_none()
    {
        return Err(ContractError::Conflict(
            "postgres realtime presence online state must include last_seen_at".into(),
        ));
    }
    u64_to_i64("last_sync_seq", record.presence.last_sync_seq)?;
    parse_optional_utc("last_resume_at", record.presence.last_resume_at.as_deref())?;
    parse_optional_utc("last_seen_at", record.presence.last_seen_at.as_deref())?;
    parse_utc("updated_at", record.updated_at.as_str())?;
    Ok(())
}

fn presence_state_from_row(row: Row) -> Result<PresenceStateRecord, ContractError> {
    let tenant_id: String = row.get("tenant_id");
    let organization_id: String = row.get("organization_id");
    let principal_kind: String = row.get("principal_kind");
    let principal_id: String = row.get("principal_id");
    let device_id: String = row.get("device_id");
    let status = presence_status_from_str(row.get::<_, String>("presence_status").as_str())?;
    let record = PresenceStateRecord {
        tenant_id: tenant_id.clone(),
        organization_id,
        principal_kind,
        principal_id: principal_id.clone(),
        device_id: device_id.clone(),
        presence: PresenceClientView {
            tenant_id,
            principal_id,
            device_id,
            platform: presence_payload_platform(row.get::<_, String>("payload_json").as_str())?,
            session_id: row.get("session_id"),
            status,
            last_sync_seq: i64_to_u64("last_sync_seq", row.get("last_sync_seq"))?,
            last_resume_at: format_optional_utc_millis(row.get("last_resume_at")),
            last_seen_at: format_optional_utc_millis(row.get("last_seen_at")),
        },
        resume_required: row.get("resume_required"),
        updated_at: format_utc_millis(row.get("updated_at")),
    };
    Ok(record)
}

fn presence_status_from_str(status: &str) -> Result<PresenceStatus, ContractError> {
    match status {
        "online" => Ok(PresenceStatus::Online),
        "offline" => Ok(PresenceStatus::Offline),
        other => Err(ContractError::Unavailable(format!(
            "postgres realtime presence row has unsupported presence_status `{other}`"
        ))),
    }
}

fn presence_payload_platform(payload_json: &str) -> Result<Option<String>, ContractError> {
    let payload: serde_json::Value = serde_json::from_str(payload_json).map_err(|error| {
        ContractError::Unavailable(format!(
            "postgres realtime presence row has invalid payload_json: {error}"
        ))
    })?;
    Ok(payload
        .get("platform")
        .and_then(|value| value.as_str())
        .map(str::to_owned))
}

fn presence_payload_json(record: &PresenceStateRecord) -> Result<String, ContractError> {
    serde_json::to_string(&serde_json::json!({
        "tenantId": record.presence.tenant_id,
        "principalKind": record.principal_kind,
        "principalId": record.presence.principal_id,
        "deviceId": record.presence.device_id,
        "platform": record.presence.platform,
        "sessionId": record.presence.session_id,
        "status": record.presence.status,
        "lastSyncSeq": record.presence.last_sync_seq,
        "lastResumeAt": record.presence.last_resume_at,
        "lastSeenAt": record.presence.last_seen_at,
        "resumeRequired": record.resume_required,
        "updatedAt": record.updated_at,
    }))
    .map_err(|error| {
        ContractError::Conflict(format!(
            "postgres realtime presence payload must serialize to JSON: {error}"
        ))
    })
}

fn validate_disconnect_fence_for_write(
    record: &RealtimeDisconnectFenceRecord,
) -> Result<(), ContractError> {
    parse_utc("disconnected_at", record.disconnected_at.as_str())?;
    if record.owner_node_id.trim().is_empty() {
        return Err(ContractError::Conflict(
            "postgres realtime disconnect fence owner_node_id must not be empty".into(),
        ));
    }
    if record.fence_token.trim().is_empty() {
        return Err(ContractError::Conflict(
            "postgres realtime disconnect fence fence_token must not be empty".into(),
        ));
    }
    Ok(())
}

fn disconnect_fence_from_row(row: Row) -> Result<RealtimeDisconnectFenceRecord, ContractError> {
    Ok(RealtimeDisconnectFenceRecord {
        tenant_id: row.get("tenant_id"),
        organization_id: row.get("organization_id"),
        principal_kind: row.get("principal_kind"),
        principal_id: row.get("principal_id"),
        device_id: row.get("device_id"),
        session_id: row.get("session_id"),
        owner_node_id: row.get("owner_node_id"),
        disconnected_at: format_utc_millis(row.get("disconnected_at")),
        fence_token: row.get("fence_token"),
    })
}

fn disconnect_fence_payload_json(
    record: &RealtimeDisconnectFenceRecord,
) -> Result<String, ContractError> {
    Ok(serde_json::json!({
        "tenantId": record.tenant_id,
        "principalKind": record.principal_kind,
        "principalId": record.principal_id,
        "deviceId": record.device_id,
        "sessionId": record.session_id,
        "ownerNodeId": record.owner_node_id,
        "disconnectedAt": record.disconnected_at,
        "fenceToken": record.fence_token,
    })
    .to_string())
}

fn parse_optional_utc(
    field: &'static str,
    value: Option<&str>,
) -> Result<Option<DateTime<Utc>>, ContractError> {
    value
        .map(|candidate| parse_utc(field, candidate))
        .transpose()
}

fn parse_utc(field: &'static str, value: &str) -> Result<DateTime<Utc>, ContractError> {
    DateTime::parse_from_rfc3339(value)
        .map(|datetime| datetime.with_timezone(&Utc))
        .map_err(|error| {
            ContractError::Conflict(format!(
                "postgres realtime checkpoint {field} must be RFC3339: {error}"
            ))
        })
}

fn format_optional_utc_millis(value: Option<DateTime<Utc>>) -> Option<String> {
    value.map(format_utc_millis)
}

fn format_utc_millis(value: DateTime<Utc>) -> String {
    value.to_rfc3339_opts(SecondsFormat::Millis, true)
}

fn u64_to_i64(field: &'static str, value: u64) -> Result<i64, ContractError> {
    i64::try_from(value).map_err(|_| {
        checkpoint_conflict(format!(
            "postgres realtime checkpoint {field}={value} exceeds PostgreSQL bigint maximum {}",
            i64::MAX
        ))
    })
}

fn i64_to_u64(field: &'static str, value: i64) -> Result<u64, ContractError> {
    u64::try_from(value).map_err(|_| {
        ContractError::Unavailable(format!(
            "postgres realtime checkpoint row has negative {field}={value}"
        ))
    })
}

pub(crate) fn run_postgres_io<T>(
    operation: impl FnOnce() -> Result<T, ContractError> + Send,
) -> Result<T, ContractError>
where
    T: Send,
{
    std::thread::scope(|scope| {
        scope
            .spawn(operation)
            .join()
            .map_err(|_| postgres_io_thread_panic())?
    })
}

fn postgres_pool_client(
    pool: &PostgresRealtimePool,
    action: &'static str,
) -> Result<r2d2::PooledConnection<PostgresRealtimeConnectionManager>, ContractError> {
    pool.get()
        .map_err(|error| postgres_unavailable(action, error))
}

fn postgres_config_error(
    database_url: &str,
    error: r2d2_postgres::postgres::Error,
) -> ContractError {
    let redacted = redact_postgres_url(database_url);
    ContractError::Unavailable(format!(
        "postgres realtime checkpoint database url is invalid ({redacted}): {error}"
    ))
}

fn postgres_unavailable(action: &'static str, error: impl std::fmt::Display) -> ContractError {
    ContractError::Unavailable(format!(
        "postgres realtime checkpoint store {action} failed: {error}"
    ))
}

fn postgres_io_thread_panic() -> ContractError {
    ContractError::Unavailable(
        "postgres realtime checkpoint store blocking IO worker panicked".into(),
    )
}

fn checkpoint_conflict(message: impl Into<String>) -> ContractError {
    ContractError::Conflict(format!(
        "postgres realtime checkpoint record is invalid: {}",
        message.into()
    ))
}

fn redact_postgres_url(database_url: &str) -> String {
    let Some(scheme_end) = database_url.find("://") else {
        return "<redacted>".into();
    };
    let after_scheme = scheme_end + 3;
    let Some(at_offset) = database_url[after_scheme..].find('@') else {
        return database_url.into();
    };
    format!(
        "{}<credentials>@{}",
        &database_url[..after_scheme],
        &database_url[after_scheme + at_offset + 1..]
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_window_validation_rejects_events_for_different_device_identity() {
        for (field, event) in [
            (
                "event.tenant_id",
                RealtimeEvent {
                    tenant_id: "t_other".into(),
                    ..realtime_event()
                },
            ),
            (
                "event.principal_id",
                RealtimeEvent {
                    principal_id: "u_other".into(),
                    ..realtime_event()
                },
            ),
            (
                "event.device_id",
                RealtimeEvent {
                    device_id: "d_other".into(),
                    ..realtime_event()
                },
            ),
        ] {
            let error = validate_event_window_for_write(&RealtimeEventWindowRecord {
                tenant_id: "100001".into(),
                organization_id: "0".into(),
                principal_kind: "user".into(),
                principal_id: "1".into(),
                device_id: "d_pad".into(),
                events: vec![event],
                trimmed_through_seq: 0,
                capacity_trimmed_event_count: 0,
                capacity_trimmed_through_seq: 0,
                last_capacity_trimmed_at: None,
                updated_at: "2026-05-09T10:00:01.000Z".into(),
            })
            .expect_err("event window validation must reject mismatched event identity");

            match error {
                ContractError::Conflict(message) => assert!(
                    message.contains(field),
                    "identity conflict should name {field}; got {message}"
                ),
                other => panic!("identity mismatch should be a conflict, got {other:?}"),
            }
        }
    }

    fn realtime_event() -> RealtimeEvent {
        RealtimeEvent {
            tenant_id: "100001".into(),
            principal_id: "1".into(),
            device_id: "d_pad".into(),
            realtime_seq: 1,
            scope_type: "conversation".into(),
            scope_id: "c_demo".into(),
            event_type: "message.posted".into(),
            delivery_class: "ephemeral".into(),
            payload: r#"{"messageId":"msg_1"}"#.into(),
            occurred_at: "2026-05-09T10:00:00.000Z".into(),
        }
    }
}
