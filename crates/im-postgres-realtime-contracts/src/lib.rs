//! PostgreSQL SQL contracts for a production realtime store implementation.
//!
//! These constants freeze the executable SQL shape against the checked-in
//! migration schema. PostgreSQL adapters must bind parameters exactly as each
//! constant documents and execute multi-statement mutations in one transaction.

use im_domain_core::realtime::{RealtimeEvent, RealtimeSubscription};
use sdkwork_im_contract_control::{RealtimeCheckpointRecord, RealtimeSubscriptionRecord};

pub const LOAD_REALTIME_CHECKPOINT_SQL: &str = r#"
select
    tenant_id,
    organization_id,
    principal_kind,
    principal_id,
    device_id,
    latest_realtime_seq,
    acked_through_seq,
    trimmed_through_seq,
    capacity_trimmed_event_count,
    capacity_trimmed_through_seq,
    last_capacity_trimmed_at,
    updated_at
from im_realtime_checkpoints
where tenant_id = $1 and organization_id = $2 and client_route_scope_key = $3
"#;

pub const UPSERT_REALTIME_CHECKPOINT_SQL: &str = r#"
insert into im_realtime_checkpoints (
    tenant_id,
    organization_id,
    client_route_scope_key,
    principal_kind,
    principal_id,
    device_id,
    latest_realtime_seq,
    acked_through_seq,
    trimmed_through_seq,
    capacity_trimmed_event_count,
    capacity_trimmed_through_seq,
    last_capacity_trimmed_at,
    created_at,
    updated_at
) values (
    $1, $2, $3, $4, $5, $6,
    $7, $8, $9, $10, $11, $12, $13, $14
)
on conflict (tenant_id, organization_id, client_route_scope_key) do update set
    principal_kind = excluded.principal_kind,
    principal_id = excluded.principal_id,
    device_id = excluded.device_id,
    latest_realtime_seq = greatest(
        im_realtime_checkpoints.latest_realtime_seq,
        excluded.latest_realtime_seq
    ),
    acked_through_seq = greatest(
        im_realtime_checkpoints.acked_through_seq,
        excluded.acked_through_seq
    ),
    trimmed_through_seq = greatest(
        im_realtime_checkpoints.trimmed_through_seq,
        excluded.trimmed_through_seq
    ),
    capacity_trimmed_event_count = greatest(
        im_realtime_checkpoints.capacity_trimmed_event_count,
        excluded.capacity_trimmed_event_count
    ),
    -- Constraint-safe capacity trim sequence must never exceed trimmed_through_seq.
    capacity_trimmed_through_seq = least(
        greatest(
            im_realtime_checkpoints.capacity_trimmed_through_seq,
            excluded.capacity_trimmed_through_seq
        ),
        greatest(
            im_realtime_checkpoints.trimmed_through_seq,
            excluded.trimmed_through_seq
        )
    ),
    -- Null-safe monotonic capacity trim timestamp.
    last_capacity_trimmed_at = case
        when im_realtime_checkpoints.last_capacity_trimmed_at is null then excluded.last_capacity_trimmed_at
        when excluded.last_capacity_trimmed_at is null then im_realtime_checkpoints.last_capacity_trimmed_at
        else greatest(
            im_realtime_checkpoints.last_capacity_trimmed_at,
            excluded.last_capacity_trimmed_at
        )
    end,
    updated_at = greatest(im_realtime_checkpoints.updated_at, excluded.updated_at)
"#;

pub const UPSERT_REALTIME_CLIENT_ROUTE_EVENT_SQL: &str = r#"
insert into im_realtime_device_events (
    tenant_id,
    organization_id,
    client_route_scope_key,
    realtime_seq,
    principal_kind,
    principal_id,
    device_id,
    scope_type,
    scope_id,
    event_type,
    delivery_class,
    payload_json,
    payload_hash,
    occurred_at,
    created_at,
    retention_until
) values (
    $1, $2, $3, $4, $5, $6, $7, $8,
    $9, $10, $11, $12::jsonb, $13, $14, $15, $16
)
on conflict (tenant_id, organization_id, client_route_scope_key, realtime_seq) do nothing
"#;

pub const LIST_REALTIME_CLIENT_ROUTE_EVENTS_SQL: &str = r#"
select
    tenant_id,
    organization_id,
    principal_kind,
    principal_id,
    device_id,
    realtime_seq,
    scope_type,
    scope_id,
    event_type,
    delivery_class,
    payload_json::text as payload_json,
    occurred_at
from im_realtime_device_events
where tenant_id = $1
  and organization_id = $2
  and client_route_scope_key = $3
  and realtime_seq > $4
order by realtime_seq asc
limit $5
"#;

pub const TRIM_REALTIME_CLIENT_ROUTE_EVENTS_SQL: &str = r#"
delete from im_realtime_device_events
where tenant_id = $1
  and organization_id = $2
  and client_route_scope_key = $3
  and realtime_seq <= $4
"#;

pub const CLEAR_REALTIME_CLIENT_ROUTE_EVENTS_SQL: &str = r#"
delete from im_realtime_device_events
where tenant_id = $1 and organization_id = $2 and client_route_scope_key = $3
"#;

pub const LOAD_REALTIME_EVENT_WINDOW_DIAGNOSTICS_SQL: &str = r#"
with window_counts as (
    select
        tenant_id,
        client_route_scope_key,
        count(*) as pending_event_count
    from im_realtime_device_events
    group by tenant_id, organization_id, client_route_scope_key
)
select
    count(distinct c.tenant_id || ':' || c.client_route_scope_key) as client_route_window_count,
    count(e.realtime_seq) as pending_event_count,
    coalesce(max(window_counts.pending_event_count), 0) as max_client_route_window_event_count,
    coalesce(max(c.trimmed_through_seq), 0) as max_trimmed_through_seq,
    coalesce(sum(c.capacity_trimmed_event_count), 0) as capacity_trimmed_event_count,
    coalesce(max(c.capacity_trimmed_through_seq), 0) as max_capacity_trimmed_through_seq,
    max(c.last_capacity_trimmed_at) as last_capacity_trimmed_at,
    min(e.occurred_at) as oldest_pending_occurred_at
from im_realtime_checkpoints c
left join im_realtime_device_events e
  on e.tenant_id = c.tenant_id
 and e.organization_id = c.organization_id
 and e.client_route_scope_key = c.client_route_scope_key
left join window_counts
  on window_counts.tenant_id = c.tenant_id
 and window_counts.organization_id = c.organization_id
 and window_counts.client_route_scope_key = c.client_route_scope_key
"#;

pub const LIST_REALTIME_EVENT_WINDOW_HIGH_RISK_WINDOWS_SQL: &str = r#"
select
    c.tenant_id,
    c.organization_id,
    c.principal_kind,
    c.principal_id,
    c.device_id,
    count(e.realtime_seq) as pending_event_count,
    c.trimmed_through_seq,
    c.capacity_trimmed_event_count,
    c.capacity_trimmed_through_seq,
    c.last_capacity_trimmed_at,
    min(e.occurred_at) as oldest_pending_occurred_at
from im_realtime_checkpoints c
join im_realtime_device_events e
  on e.tenant_id = c.tenant_id
 and e.organization_id = c.organization_id
 and e.client_route_scope_key = c.client_route_scope_key
group by
    c.tenant_id,
    c.principal_kind,
    c.principal_id,
    c.device_id,
    c.client_route_scope_key,
    c.trimmed_through_seq,
    c.capacity_trimmed_event_count,
    c.capacity_trimmed_through_seq,
    c.last_capacity_trimmed_at
order by pending_event_count desc, oldest_pending_occurred_at asc, c.tenant_id asc, c.principal_kind asc, c.principal_id asc, c.device_id asc
limit 5
"#;

pub const LIST_ORPHANED_REALTIME_CLIENT_ROUTE_EVENTS_SQL: &str = r#"
select
    e.tenant_id,
    e.client_route_scope_key,
    count(*) as orphaned_event_count,
    min(e.realtime_seq) as min_realtime_seq,
    max(e.realtime_seq) as max_realtime_seq,
    min(e.occurred_at) as oldest_orphaned_occurred_at
from im_realtime_device_events e
left join im_realtime_checkpoints c
  on c.tenant_id = e.tenant_id
 and c.organization_id = e.organization_id
 and c.client_route_scope_key = e.client_route_scope_key
where c.client_route_scope_key is null
group by e.tenant_id, e.client_route_scope_key
order by orphaned_event_count desc, oldest_orphaned_occurred_at asc, e.tenant_id asc, e.client_route_scope_key asc
limit $1
"#;

pub const LOAD_REALTIME_SUBSCRIPTION_SQL: &str = r#"
select
    tenant_id,
    organization_id,
    principal_kind,
    principal_id,
    device_id,
    subscriptions_json::text as subscriptions_json,
    synced_at
from im_realtime_subscriptions
where tenant_id = $1 and organization_id = $2 and client_route_scope_key = $3
"#;

pub const UPSERT_REALTIME_SUBSCRIPTION_SQL: &str = r#"
insert into im_realtime_subscriptions (
    tenant_id,
    organization_id,
    client_route_scope_key,
    principal_kind,
    principal_id,
    device_id,
    subscriptions_json,
    subscription_count,
    synced_at,
    created_at,
    updated_at
) values (
    $1, $2, $3, $4, $5, $6, $7::jsonb, $8, $9, $10, $11
)
on conflict (tenant_id, organization_id, client_route_scope_key) do update set
    principal_kind = excluded.principal_kind,
    principal_id = excluded.principal_id,
    device_id = excluded.device_id,
    subscriptions_json = excluded.subscriptions_json,
    subscription_count = excluded.subscription_count,
    synced_at = excluded.synced_at,
    updated_at = excluded.updated_at
where excluded.synced_at >= im_realtime_subscriptions.synced_at
"#;

pub const CLEAR_REALTIME_SUBSCRIPTION_SQL: &str = r#"
delete from im_realtime_subscriptions
where tenant_id = $1 and organization_id = $2 and client_route_scope_key = $3
"#;

pub const CLEAR_REALTIME_SUBSCRIPTION_IF_SYNCED_AT_OR_BEFORE_SQL: &str = r#"
delete from im_realtime_subscriptions
where tenant_id = $1
  and organization_id = $2
  and client_route_scope_key = $3
  and synced_at <= $4
"#;

// Save realtime subscription transaction order:
// 1. UPSERT_REALTIME_SUBSCRIPTION_SQL.
// 2. CLEAR_REALTIME_SUBSCRIPTION_SCOPES_SQL.
// 3. REPLACE_REALTIME_SUBSCRIPTION_SCOPES_SQL for each derived scope row.
//
// The clear step must use the attempted synced_at cutoff. This mirrors the
// RealtimeSubscriptionStore clear_subscriptions_synced_at_or_before contract so
// a stale retry cannot delete fanout rows written by a newer subscription sync.
// The replace step must join the current subscription row by the same synced_at
// so a stale retry cannot insert old fanout rows after a newer sync wins.
pub const CLEAR_REALTIME_SUBSCRIPTION_SCOPES_SQL: &str = r#"
delete from im_realtime_subscription_scopes
where tenant_id = $1
  and organization_id = $2
  and client_route_scope_key = $3
  and synced_at <= $4
"#;

// Scope rows are a fanout index derived from RealtimeSubscriptionRecord.items.
// Persist empty event_types as one '*' row per scope so wildcard subscriptions
// use the same indexed lookup path as event-specific subscriptions.
pub const REPLACE_REALTIME_SUBSCRIPTION_SCOPES_SQL: &str = r#"
insert into im_realtime_subscription_scopes (
    tenant_id,
    organization_id,
    principal_kind,
    principal_id,
    scope_type,
    scope_id,
    event_type,
    client_route_scope_key,
    device_id,
    synced_at,
    created_at,
    updated_at
)
select
    $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12
from im_realtime_subscriptions current_subscription
where current_subscription.tenant_id = $1
  and current_subscription.organization_id = $2
  and current_subscription.client_route_scope_key = $8
  and current_subscription.synced_at = $10
on conflict (
    tenant_id,
    organization_id,
    principal_kind,
    principal_id,
    scope_type,
    scope_id,
    event_type,
    client_route_scope_key
) do update set
    device_id = excluded.device_id,
    synced_at = excluded.synced_at,
    updated_at = excluded.updated_at
where excluded.synced_at >= im_realtime_subscription_scopes.synced_at
"#;

pub const LOAD_MATCHING_REALTIME_SUBSCRIPTIONS_SQL: &str = r#"
select distinct
    s.tenant_id,
    s.principal_kind,
    s.principal_id,
    s.device_id,
    s.subscriptions_json::text as subscriptions_json,
    s.synced_at
from im_realtime_subscription_scopes fs
join im_realtime_subscriptions s
  on s.tenant_id = fs.tenant_id
 and s.organization_id = fs.organization_id
 and s.client_route_scope_key = fs.client_route_scope_key
where fs.tenant_id = $1
  and fs.organization_id = $2
  and fs.principal_kind = $3
  and fs.principal_id = $4
  and fs.scope_type = $5
  and fs.scope_id = $6
  and fs.event_type in ($7, '*')
  and fs.device_id = any($8)
order by s.device_id asc
"#;

pub const LOAD_REALTIME_DISCONNECT_FENCE_SQL: &str = r#"
select
    tenant_id,
    organization_id,
    principal_kind,
    principal_id,
    device_id,
    session_id,
    owner_node_id,
    disconnected_at,
    fence_token
from im_realtime_disconnect_fences
where tenant_id = $1
  and organization_id = $2
  and principal_kind = $3
  and principal_id = $4
  and device_id = $5
"#;

pub const UPSERT_REALTIME_DISCONNECT_FENCE_SQL: &str = r#"
insert into im_realtime_disconnect_fences (
    tenant_id,
    organization_id,
    principal_kind,
    principal_id,
    device_id,
    session_id,
    owner_node_id,
    disconnected_at,
    fence_token,
    payload_json,
    payload_hash,
    created_at,
    updated_at
) values (
    $1, $2, $3, $4, $5, $6, $7, $8, $9, $10::jsonb, $11, $12, $13
)
on conflict (tenant_id, organization_id, principal_kind, principal_id, device_id) do update set
    session_id = excluded.session_id,
    owner_node_id = excluded.owner_node_id,
    disconnected_at = excluded.disconnected_at,
    fence_token = excluded.fence_token,
    payload_json = excluded.payload_json,
    payload_hash = excluded.payload_hash,
    updated_at = excluded.updated_at
where excluded.disconnected_at > im_realtime_disconnect_fences.disconnected_at
"#;

pub const CLEAR_REALTIME_DISCONNECT_FENCE_SQL: &str = r#"
delete from im_realtime_disconnect_fences
where tenant_id = $1
  and organization_id = $2
  and principal_kind = $3
  and principal_id = $4
  and device_id = $5
"#;

pub const CLEAR_REALTIME_DISCONNECT_FENCE_IF_MATCHES_SQL: &str = r#"
delete from im_realtime_disconnect_fences
where tenant_id = $1
  and organization_id = $2
  and principal_kind = $3
  and principal_id = $4
  and device_id = $5
  and fence_token = $6
"#;

pub const CLEAR_REALTIME_DISCONNECT_FENCE_DISCONNECTED_AT_OR_BEFORE_SQL: &str = r#"
delete from im_realtime_disconnect_fences
where tenant_id = $1
  and organization_id = $2
  and principal_kind = $3
  and principal_id = $4
  and device_id = $5
  and disconnected_at <= $6
"#;

pub const PUBLISH_REALTIME_EVENTS_TRANSACTION_PLAN: &str = r#"
Begin transaction.
1. UPSERT_REALTIME_CLIENT_ROUTE_EVENT_SQL for each delivered client route event.
2. UPSERT_REALTIME_CHECKPOINT_SQL for each affected client route checkpoint.
Commit transaction.
Rollback transaction on any error.
Event window rows and checkpoint rows must never be committed separately.
"#;

pub const ACK_REALTIME_EVENTS_TRANSACTION_PLAN: &str = r#"
Begin transaction.
1. TRIM_REALTIME_CLIENT_ROUTE_EVENTS_SQL.
2. UPSERT_REALTIME_CHECKPOINT_SQL.
Commit transaction.
Rollback transaction on any error.
Trimmed event rows and checkpoint rows must never be committed separately.
"#;

pub const RESTORE_REALTIME_CLIENT_ROUTE_STATE_TRANSACTION_PLAN: &str = r#"
Begin transaction.
1. UPSERT_REALTIME_SUBSCRIPTION_SQL or CLEAR_REALTIME_SUBSCRIPTION_SQL.
2. CLEAR_REALTIME_SUBSCRIPTION_SCOPES_SQL when replacing subscriptions.
3. REPLACE_REALTIME_SUBSCRIPTION_SCOPES_SQL for each derived scope row.
4. UPSERT_REALTIME_CHECKPOINT_SQL.
5. CLEAR_REALTIME_CLIENT_ROUTE_EVENTS_SQL.
6. UPSERT_REALTIME_CLIENT_ROUTE_EVENT_SQL for each restored event.
Commit transaction.
Rollback transaction on any error.
"#;

pub const TAKE_REALTIME_CLIENT_ROUTE_STATE_TRANSACTION_PLAN: &str = r#"
Begin transaction.
1. LOAD_REALTIME_CHECKPOINT_SQL.
2. LIST_REALTIME_CLIENT_ROUTE_EVENTS_SQL.
3. LOAD_REALTIME_SUBSCRIPTION_SQL.
4. CLEAR_REALTIME_CLIENT_ROUTE_EVENTS_SQL.
5. CLEAR_REALTIME_SUBSCRIPTION_SCOPES_SQL.
6. CLEAR_REALTIME_SUBSCRIPTION_SQL.
Commit transaction.
Rollback transaction on any error.
"#;

pub const SAVE_REALTIME_SUBSCRIPTIONS_TRANSACTION_PLAN: &str = r#"
Begin transaction.
1. UPSERT_REALTIME_SUBSCRIPTION_SQL.
2. CLEAR_REALTIME_SUBSCRIPTION_SCOPES_SQL.
3. REPLACE_REALTIME_SUBSCRIPTION_SCOPES_SQL.
Commit transaction.
Rollback transaction on any error.
"#;

pub const DISCONNECT_FENCE_TRANSACTION_PLAN: &str = r#"
Single-statement compare-and-set operations.
UPSERT_REALTIME_DISCONNECT_FENCE_SQL.
CLEAR_REALTIME_DISCONNECT_FENCE_SQL.
CLEAR_REALTIME_DISCONNECT_FENCE_IF_MATCHES_SQL.
CLEAR_REALTIME_DISCONNECT_FENCE_DISCONNECTED_AT_OR_BEFORE_SQL.
"#;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RealtimePostgresParameterBinding {
    pub position: u32,
    pub name: &'static str,
    pub rust_type: &'static str,
    pub postgres_type: &'static str,
    pub notes: &'static str,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RealtimePostgresRowColumn {
    pub column: &'static str,
    pub rust_field: &'static str,
    pub rust_type: &'static str,
    pub notes: &'static str,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RealtimePostgresRowMapping {
    pub target: &'static str,
    pub columns: &'static [RealtimePostgresRowColumn],
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RealtimePostgresSqlContract {
    pub name: &'static str,
    pub sql: &'static str,
    pub parameter_bindings: &'static [RealtimePostgresParameterBinding],
    pub row_mapping: Option<RealtimePostgresRowMapping>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RealtimePostgresMethodAtomicity {
    ReadOnly,
    SingleStatement,
    Transaction,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RealtimePostgresMethodStep {
    pub sql_contract_name: &'static str,
    pub binding_source: &'static str,
    pub result_mapping: &'static str,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RealtimePostgresMethodPlan {
    pub name: &'static str,
    pub atomicity: RealtimePostgresMethodAtomicity,
    pub transaction_plan_name: Option<&'static str>,
    pub steps: &'static [RealtimePostgresMethodStep],
    pub notes: &'static [&'static str],
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RealtimePostgresAdapterPlan {
    pub runtime_status: &'static str,
    pub runtime_status_reason: &'static str,
    pub sql_contracts: &'static [RealtimePostgresSqlContract],
    pub transaction_plans: &'static [&'static str],
    pub method_plans: &'static [RealtimePostgresMethodPlan],
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RealtimePostgresBindingValue {
    Text(String),
    BigInt(i64),
    Integer(i32),
    Json(String),
    Timestamptz(String),
    NullableTimestamptz(Option<String>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RealtimePostgresBoundParameter {
    pub position: u32,
    pub name: &'static str,
    pub postgres_type: &'static str,
    pub value: RealtimePostgresBindingValue,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RealtimePostgresBoundStatement {
    pub sql_name: &'static str,
    pub sql: &'static str,
    pub parameters: Vec<RealtimePostgresBoundParameter>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RealtimePostgresBoundTransaction {
    pub transaction_plan_name: &'static str,
    pub transaction_plan: &'static str,
    pub statements: Vec<RealtimePostgresBoundStatement>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RealtimePostgresClientRouteEventMutation {
    pub event: RealtimeEvent,
    pub organization_id: String,
    pub principal_kind: String,
    pub client_route_scope_key: String,
    pub payload_hash: String,
    pub retention_until: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RealtimePostgresCheckpointMutation {
    pub checkpoint: RealtimeCheckpointRecord,
    pub client_route_scope_key: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RealtimePostgresBindingError {
    pub code: &'static str,
    pub message: String,
}

impl RealtimePostgresBindingError {
    fn postgres_bigint_out_of_range(field: &'static str, value: u64) -> Self {
        Self {
            code: "postgres_bigint_out_of_range",
            message: format!(
                "{field}={value} exceeds PostgreSQL bigint maximum {}",
                i64::MAX
            ),
        }
    }

    fn postgres_integer_out_of_range(field: &'static str, value: usize) -> Self {
        Self {
            code: "postgres_integer_out_of_range",
            message: format!(
                "{field}={value} exceeds PostgreSQL integer maximum {}",
                i32::MAX
            ),
        }
    }

    fn invalid_jsonb_payload(field: &'static str, message: impl Into<String>) -> Self {
        Self {
            code: "invalid_jsonb_payload",
            message: format!(
                "{field} must be valid JSON for PostgreSQL jsonb: {}",
                message.into()
            ),
        }
    }

    fn internal_binding_mismatch(sql_name: &'static str, expected: usize, actual: usize) -> Self {
        Self {
            code: "postgres_binding_contract_mismatch",
            message: format!(
                "{sql_name} expects {expected} bindings but builder produced {actual}"
            ),
        }
    }
}

pub fn realtime_postgres_bind_checkpoint_upsert(
    checkpoint: &RealtimeCheckpointRecord,
    client_route_scope_key: &str,
    created_at: &str,
) -> Result<RealtimePostgresBoundStatement, RealtimePostgresBindingError> {
    build_bound_statement(
        "UPSERT_REALTIME_CHECKPOINT_SQL",
        UPSERT_REALTIME_CHECKPOINT_SQL,
        UPSERT_REALTIME_CHECKPOINT_BINDINGS,
        vec![
            text(checkpoint.tenant_id.as_str()),
            text(checkpoint.organization_id.as_str()),
            text(client_route_scope_key),
            text(checkpoint.principal_kind.as_str()),
            text(checkpoint.principal_id.as_str()),
            text(checkpoint.device_id.as_str()),
            bigint("latest_realtime_seq", checkpoint.latest_realtime_seq)?,
            bigint("acked_through_seq", checkpoint.acked_through_seq)?,
            bigint("trimmed_through_seq", checkpoint.trimmed_through_seq)?,
            bigint(
                "capacity_trimmed_event_count",
                checkpoint.capacity_trimmed_event_count,
            )?,
            bigint(
                "capacity_trimmed_through_seq",
                checkpoint.capacity_trimmed_through_seq,
            )?,
            nullable_timestamptz(checkpoint.last_capacity_trimmed_at.as_deref()),
            timestamptz(created_at),
            timestamptz(checkpoint.updated_at.as_str()),
        ],
    )
}

pub fn realtime_postgres_bind_client_route_event_upsert<'a>(
    event: &RealtimeEvent,
    organization_id: &str,
    principal_kind: &str,
    client_route_scope_key: &str,
    payload_hash: &str,
    created_at: &str,
    retention_until: impl Into<Option<&'a str>>,
) -> Result<RealtimePostgresBoundStatement, RealtimePostgresBindingError> {
    serde_json::from_str::<serde_json::Value>(event.payload.as_str()).map_err(|error| {
        RealtimePostgresBindingError::invalid_jsonb_payload("payload_json", error.to_string())
    })?;

    build_bound_statement(
        "UPSERT_REALTIME_CLIENT_ROUTE_EVENT_SQL",
        UPSERT_REALTIME_CLIENT_ROUTE_EVENT_SQL,
        UPSERT_REALTIME_CLIENT_ROUTE_EVENT_BINDINGS,
        vec![
            text(event.tenant_id.as_str()),
            text(organization_id),
            text(client_route_scope_key),
            bigint("realtime_seq", event.realtime_seq)?,
            text(principal_kind),
            text(event.principal_id.as_str()),
            text(event.device_id.as_str()),
            text(event.scope_type.as_str()),
            text(event.scope_id.as_str()),
            text(event.event_type.as_str()),
            text(event.delivery_class.as_str()),
            json(event.payload.as_str()),
            text(payload_hash),
            timestamptz(event.occurred_at.as_str()),
            timestamptz(created_at),
            nullable_timestamptz(retention_until.into()),
        ],
    )
}

pub fn realtime_postgres_bind_trim_client_route_events(
    tenant_id: &str,
    organization_id: &str,
    client_route_scope_key: &str,
    acked_through_seq: u64,
) -> Result<RealtimePostgresBoundStatement, RealtimePostgresBindingError> {
    build_bound_statement(
        "TRIM_REALTIME_CLIENT_ROUTE_EVENTS_SQL",
        TRIM_REALTIME_CLIENT_ROUTE_EVENTS_SQL,
        TRIM_REALTIME_CLIENT_ROUTE_EVENTS_BINDINGS,
        vec![
            text(tenant_id),
            text(organization_id),
            text(client_route_scope_key),
            bigint("acked_through_seq", acked_through_seq)?,
        ],
    )
}

pub fn realtime_postgres_bind_subscription_upsert(
    record: &RealtimeSubscriptionRecord,
    client_route_scope_key: &str,
    statement_timestamp: &str,
) -> Result<RealtimePostgresBoundStatement, RealtimePostgresBindingError> {
    let subscriptions_json = serde_json::to_string(&record.items).map_err(|error| {
        RealtimePostgresBindingError::invalid_jsonb_payload("subscriptions_json", error.to_string())
    })?;

    build_bound_statement(
        "UPSERT_REALTIME_SUBSCRIPTION_SQL",
        UPSERT_REALTIME_SUBSCRIPTION_SQL,
        UPSERT_REALTIME_SUBSCRIPTION_BINDINGS,
        vec![
            text(record.tenant_id.as_str()),
            text(record.organization_id.as_str()),
            text(client_route_scope_key),
            text(record.principal_kind.as_str()),
            text(record.principal_id.as_str()),
            text(record.device_id.as_str()),
            RealtimePostgresBindingValue::Json(subscriptions_json),
            integer("subscription_count", record.items.len())?,
            timestamptz(record.synced_at.as_str()),
            timestamptz(statement_timestamp),
            timestamptz(statement_timestamp),
        ],
    )
}

pub fn realtime_postgres_bind_subscription_scope_clear(
    tenant_id: &str,
    organization_id: &str,
    client_route_scope_key: &str,
    cutoff_synced_at: &str,
) -> RealtimePostgresBoundStatement {
    build_bound_statement(
        "CLEAR_REALTIME_SUBSCRIPTION_SCOPES_SQL",
        CLEAR_REALTIME_SUBSCRIPTION_SCOPES_SQL,
        CLEAR_REALTIME_SUBSCRIPTION_IF_SYNCED_BINDINGS,
        vec![
            text(tenant_id),
            text(organization_id),
            text(client_route_scope_key),
            timestamptz(cutoff_synced_at),
        ],
    )
    .expect("static PostgreSQL subscription scope clear bindings should match SQL contract")
}

pub fn realtime_postgres_bind_subscription_scope_replacements(
    record: &RealtimeSubscriptionRecord,
    client_route_scope_key: &str,
    statement_timestamp: &str,
) -> Vec<RealtimePostgresBoundStatement> {
    record
        .items
        .iter()
        .flat_map(|subscription| {
            subscription_fanout_event_types(subscription)
                .into_iter()
                .map(move |event_type| {
                    build_bound_statement(
                        "REPLACE_REALTIME_SUBSCRIPTION_SCOPES_SQL",
                        REPLACE_REALTIME_SUBSCRIPTION_SCOPES_SQL,
                        REPLACE_REALTIME_SUBSCRIPTION_SCOPES_BINDINGS,
                        vec![
                            text(record.tenant_id.as_str()),
                            text(record.organization_id.as_str()),
                            text(record.principal_kind.as_str()),
                            text(record.principal_id.as_str()),
                            text(subscription.scope_type.as_str()),
                            text(subscription.scope_id.as_str()),
                            text(event_type.as_str()),
                            text(client_route_scope_key),
                            text(record.device_id.as_str()),
                            timestamptz(record.synced_at.as_str()),
                            timestamptz(statement_timestamp),
                            timestamptz(statement_timestamp),
                        ],
                    )
                    .expect(
                        "static PostgreSQL subscription scope replacement bindings should match SQL contract",
                    )
                })
        })
        .collect()
}

pub fn realtime_postgres_bind_publish_transaction(
    events: Vec<RealtimePostgresClientRouteEventMutation>,
    checkpoints: Vec<RealtimePostgresCheckpointMutation>,
    statement_timestamp: &str,
) -> Result<RealtimePostgresBoundTransaction, RealtimePostgresBindingError> {
    let mut statements = Vec::with_capacity(events.len() + checkpoints.len());
    for event in events {
        statements.push(realtime_postgres_bind_client_route_event_upsert(
            &event.event,
            event.organization_id.as_str(),
            event.principal_kind.as_str(),
            event.client_route_scope_key.as_str(),
            event.payload_hash.as_str(),
            statement_timestamp,
            event.retention_until.as_deref(),
        )?);
    }
    for checkpoint in checkpoints {
        statements.push(realtime_postgres_bind_checkpoint_upsert(
            &checkpoint.checkpoint,
            checkpoint.client_route_scope_key.as_str(),
            statement_timestamp,
        )?);
    }

    Ok(RealtimePostgresBoundTransaction {
        transaction_plan_name: "PUBLISH_REALTIME_EVENTS_TRANSACTION_PLAN",
        transaction_plan: PUBLISH_REALTIME_EVENTS_TRANSACTION_PLAN,
        statements,
    })
}

pub fn realtime_postgres_bind_ack_transaction(
    tenant_id: &str,
    organization_id: &str,
    client_route_scope_key: &str,
    acked_through_seq: u64,
    checkpoint: &RealtimeCheckpointRecord,
    statement_timestamp: &str,
) -> Result<RealtimePostgresBoundTransaction, RealtimePostgresBindingError> {
    Ok(RealtimePostgresBoundTransaction {
        transaction_plan_name: "ACK_REALTIME_EVENTS_TRANSACTION_PLAN",
        transaction_plan: ACK_REALTIME_EVENTS_TRANSACTION_PLAN,
        statements: vec![
            realtime_postgres_bind_trim_client_route_events(
                tenant_id,
                organization_id,
                client_route_scope_key,
                acked_through_seq,
            )?,
            realtime_postgres_bind_checkpoint_upsert(
                checkpoint,
                client_route_scope_key,
                statement_timestamp,
            )?,
        ],
    })
}

pub fn realtime_postgres_bind_save_subscription_transaction(
    record: &RealtimeSubscriptionRecord,
    client_route_scope_key: &str,
    statement_timestamp: &str,
) -> Result<RealtimePostgresBoundTransaction, RealtimePostgresBindingError> {
    let mut statements =
        Vec::with_capacity(2 + subscription_scope_replacement_count(record.items.as_slice()));
    statements.push(realtime_postgres_bind_subscription_upsert(
        record,
        client_route_scope_key,
        statement_timestamp,
    )?);
    statements.push(realtime_postgres_bind_subscription_scope_clear(
        record.tenant_id.as_str(),
        record.organization_id.as_str(),
        client_route_scope_key,
        record.synced_at.as_str(),
    ));
    statements.extend(realtime_postgres_bind_subscription_scope_replacements(
        record,
        client_route_scope_key,
        statement_timestamp,
    ));

    Ok(RealtimePostgresBoundTransaction {
        transaction_plan_name: "SAVE_REALTIME_SUBSCRIPTIONS_TRANSACTION_PLAN",
        transaction_plan: SAVE_REALTIME_SUBSCRIPTIONS_TRANSACTION_PLAN,
        statements,
    })
}

const fn binding(
    position: u32,
    name: &'static str,
    rust_type: &'static str,
    postgres_type: &'static str,
    notes: &'static str,
) -> RealtimePostgresParameterBinding {
    RealtimePostgresParameterBinding {
        position,
        name,
        rust_type,
        postgres_type,
        notes,
    }
}

const fn row_column(
    column: &'static str,
    rust_field: &'static str,
    rust_type: &'static str,
    notes: &'static str,
) -> RealtimePostgresRowColumn {
    RealtimePostgresRowColumn {
        column,
        rust_field,
        rust_type,
        notes,
    }
}

const fn step(
    sql_contract_name: &'static str,
    binding_source: &'static str,
    result_mapping: &'static str,
) -> RealtimePostgresMethodStep {
    RealtimePostgresMethodStep {
        sql_contract_name,
        binding_source,
        result_mapping,
    }
}

fn build_bound_statement(
    sql_name: &'static str,
    sql: &'static str,
    bindings: &'static [RealtimePostgresParameterBinding],
    values: Vec<RealtimePostgresBindingValue>,
) -> Result<RealtimePostgresBoundStatement, RealtimePostgresBindingError> {
    if bindings.len() != values.len() {
        return Err(RealtimePostgresBindingError::internal_binding_mismatch(
            sql_name,
            bindings.len(),
            values.len(),
        ));
    }

    Ok(RealtimePostgresBoundStatement {
        sql_name,
        sql,
        parameters: bindings
            .iter()
            .zip(values)
            .map(|(binding, value)| RealtimePostgresBoundParameter {
                position: binding.position,
                name: binding.name,
                postgres_type: binding.postgres_type,
                value,
            })
            .collect(),
    })
}

fn text(value: &str) -> RealtimePostgresBindingValue {
    RealtimePostgresBindingValue::Text(value.into())
}

fn json(value: &str) -> RealtimePostgresBindingValue {
    RealtimePostgresBindingValue::Json(value.into())
}

fn timestamptz(value: &str) -> RealtimePostgresBindingValue {
    RealtimePostgresBindingValue::Timestamptz(value.into())
}

fn nullable_timestamptz(value: Option<&str>) -> RealtimePostgresBindingValue {
    RealtimePostgresBindingValue::NullableTimestamptz(value.map(str::to_owned))
}

fn bigint(
    field: &'static str,
    value: u64,
) -> Result<RealtimePostgresBindingValue, RealtimePostgresBindingError> {
    let value = i64::try_from(value)
        .map_err(|_| RealtimePostgresBindingError::postgres_bigint_out_of_range(field, value))?;
    Ok(RealtimePostgresBindingValue::BigInt(value))
}

fn integer(
    field: &'static str,
    value: usize,
) -> Result<RealtimePostgresBindingValue, RealtimePostgresBindingError> {
    let value = i32::try_from(value)
        .map_err(|_| RealtimePostgresBindingError::postgres_integer_out_of_range(field, value))?;
    Ok(RealtimePostgresBindingValue::Integer(value))
}

fn subscription_fanout_event_types(subscription: &RealtimeSubscription) -> Vec<String> {
    if subscription.event_types.is_empty() {
        vec!["*".into()]
    } else {
        subscription.event_types.clone()
    }
}

fn subscription_scope_replacement_count(items: &[RealtimeSubscription]) -> usize {
    items.iter().map(|item| item.event_types.len().max(1)).sum()
}

const DEVICE_SCOPE_BINDINGS: &[RealtimePostgresParameterBinding] = &[
    binding(1, "tenant_id", "&str", "text", "Tenant partition key."),
    binding(2, "organization_id", "&str", "text", "Organization partition key."),
    binding(
        3,
        "client_route_scope_key",
        "String",
        "text",
        "Encoded tenant/principal/device scope key.",
    ),
];

const LOAD_REALTIME_CHECKPOINT_ROW_COLUMNS: &[RealtimePostgresRowColumn] = &[
    row_column("tenant_id", "tenant_id", "String", ""),
    row_column("organization_id", "organization_id", "String", ""),
    row_column("principal_kind", "principal_kind", "String", ""),
    row_column("principal_id", "principal_id", "String", ""),
    row_column("device_id", "device_id", "String", ""),
    row_column("latest_realtime_seq", "latest_realtime_seq", "u64", ""),
    row_column("acked_through_seq", "acked_through_seq", "u64", ""),
    row_column("trimmed_through_seq", "trimmed_through_seq", "u64", ""),
    row_column(
        "capacity_trimmed_event_count",
        "capacity_trimmed_event_count",
        "u64",
        "",
    ),
    row_column(
        "capacity_trimmed_through_seq",
        "capacity_trimmed_through_seq",
        "u64",
        "",
    ),
    row_column(
        "last_capacity_trimmed_at",
        "last_capacity_trimmed_at",
        "Option<String>",
        "Map timestamptz to RFC3339 UTC string.",
    ),
    row_column(
        "updated_at",
        "updated_at",
        "String",
        "Map timestamptz to RFC3339 UTC string.",
    ),
];

const UPSERT_REALTIME_CHECKPOINT_BINDINGS: &[RealtimePostgresParameterBinding] = &[
    binding(1, "tenant_id", "&str", "text", ""),
    binding(2, "organization_id", "&str", "text", ""),
    binding(3, "client_route_scope_key", "String", "text", ""),
    binding(4, "principal_kind", "&str", "text", ""),
    binding(5, "principal_id", "&str", "text", ""),
    binding(6, "device_id", "&str", "text", ""),
    binding(7, "latest_realtime_seq", "u64", "bigint", ""),
    binding(8, "acked_through_seq", "u64", "bigint", ""),
    binding(9, "trimmed_through_seq", "u64", "bigint", ""),
    binding(10, "capacity_trimmed_event_count", "u64", "bigint", ""),
    binding(11, "capacity_trimmed_through_seq", "u64", "bigint", ""),
    binding(
        12,
        "last_capacity_trimmed_at",
        "Option<&str>",
        "timestamptz",
        "Bind RFC3339 UTC timestamp when capacity trim metadata is present.",
    ),
    binding(13, "created_at", "&str", "timestamptz", ""),
    binding(14, "updated_at", "&str", "timestamptz", ""),
];

const UPSERT_REALTIME_CLIENT_ROUTE_EVENT_BINDINGS: &[RealtimePostgresParameterBinding] = &[
    binding(1, "tenant_id", "&str", "text", ""),
    binding(2, "organization_id", "&str", "text", ""),
    binding(3, "client_route_scope_key", "String", "text", ""),
    binding(4, "realtime_seq", "u64", "bigint", ""),
    binding(5, "principal_kind", "&str", "text", ""),
    binding(6, "principal_id", "&str", "text", ""),
    binding(7, "device_id", "&str", "text", ""),
    binding(8, "scope_type", "&str", "text", ""),
    binding(9, "scope_id", "&str", "text", ""),
    binding(10, "event_type", "&str", "text", ""),
    binding(11, "delivery_class", "&str", "text", ""),
    binding(
        12,
        "payload_json",
        "&str",
        "jsonb",
        "Serialized RealtimeEvent.payload JSON.",
    ),
    binding(13, "payload_hash", "&str", "text", ""),
    binding(14, "occurred_at", "&str", "timestamptz", ""),
    binding(15, "created_at", "&str", "timestamptz", ""),
    binding(16, "retention_until", "Option<&str>", "timestamptz", ""),
];

const LIST_REALTIME_CLIENT_ROUTE_EVENTS_BINDINGS: &[RealtimePostgresParameterBinding] = &[
    binding(1, "tenant_id", "&str", "text", ""),
    binding(2, "organization_id", "&str", "text", ""),
    binding(3, "client_route_scope_key", "String", "text", ""),
    binding(4, "after_seq", "u64", "bigint", ""),
    binding(5, "limit", "usize", "bigint", ""),
];

const LIST_REALTIME_CLIENT_ROUTE_EVENTS_ROW_COLUMNS: &[RealtimePostgresRowColumn] = &[
    row_column("tenant_id", "tenant_id", "String", ""),
    row_column("organization_id", "organization_id", "String", ""),
    row_column("principal_kind", "principal_kind", "String", ""),
    row_column("principal_id", "principal_id", "String", ""),
    row_column("device_id", "device_id", "String", ""),
    row_column("realtime_seq", "realtime_seq", "u64", ""),
    row_column("scope_type", "scope_type", "String", ""),
    row_column("scope_id", "scope_id", "String", ""),
    row_column("event_type", "event_type", "String", ""),
    row_column("delivery_class", "delivery_class", "String", ""),
    row_column("payload_json", "payload", "String", ""),
    row_column(
        "occurred_at",
        "occurred_at",
        "String",
        "Map timestamptz to RFC3339 UTC string.",
    ),
];

const TRIM_REALTIME_CLIENT_ROUTE_EVENTS_BINDINGS: &[RealtimePostgresParameterBinding] = &[
    binding(1, "tenant_id", "&str", "text", ""),
    binding(2, "organization_id", "&str", "text", ""),
    binding(3, "client_route_scope_key", "String", "text", ""),
    binding(4, "acked_through_seq", "u64", "bigint", ""),
];

const ORPHANED_LIMIT_BINDINGS: &[RealtimePostgresParameterBinding] =
    &[binding(1, "limit", "usize", "bigint", "")];

const REALTIME_EVENT_WINDOW_DIAGNOSTICS_ROW_COLUMNS: &[RealtimePostgresRowColumn] = &[
    row_column(
        "client_route_window_count",
        "client_route_window_count",
        "u64",
        "",
    ),
    row_column("pending_event_count", "pending_event_count", "u64", ""),
    row_column(
        "max_client_route_window_event_count",
        "max_client_route_window_event_count",
        "u64",
        "",
    ),
    row_column(
        "max_trimmed_through_seq",
        "max_trimmed_through_seq",
        "u64",
        "",
    ),
    row_column(
        "capacity_trimmed_event_count",
        "capacity_trimmed_event_count",
        "u64",
        "",
    ),
    row_column(
        "max_capacity_trimmed_through_seq",
        "max_capacity_trimmed_through_seq",
        "u64",
        "",
    ),
    row_column(
        "last_capacity_trimmed_at",
        "last_capacity_trimmed_at",
        "Option<String>",
        "",
    ),
    row_column(
        "oldest_pending_occurred_at",
        "oldest_pending_occurred_at",
        "Option<String>",
        "",
    ),
];

const REALTIME_EVENT_WINDOW_HIGH_RISK_ROW_COLUMNS: &[RealtimePostgresRowColumn] = &[
    row_column("tenant_id", "tenant_id", "String", ""),
    row_column("principal_kind", "principal_kind", "String", ""),
    row_column("principal_id", "principal_id", "String", ""),
    row_column("device_id", "device_id", "String", ""),
    row_column("pending_event_count", "pending_event_count", "u64", ""),
    row_column("trimmed_through_seq", "trimmed_through_seq", "u64", ""),
    row_column(
        "capacity_trimmed_event_count",
        "capacity_trimmed_event_count",
        "u64",
        "",
    ),
    row_column(
        "capacity_trimmed_through_seq",
        "capacity_trimmed_through_seq",
        "u64",
        "",
    ),
    row_column(
        "last_capacity_trimmed_at",
        "last_capacity_trimmed_at",
        "Option<String>",
        "",
    ),
    row_column(
        "oldest_pending_occurred_at",
        "oldest_pending_occurred_at",
        "Option<String>",
        "",
    ),
];

const ORPHANED_REALTIME_CLIENT_ROUTE_EVENTS_ROW_COLUMNS: &[RealtimePostgresRowColumn] = &[
    row_column("tenant_id", "tenant_id", "String", ""),
    row_column(
        "client_route_scope_key",
        "client_route_scope_key",
        "String",
        "",
    ),
    row_column("orphaned_event_count", "orphaned_event_count", "u64", ""),
    row_column("min_realtime_seq", "min_realtime_seq", "u64", ""),
    row_column("max_realtime_seq", "max_realtime_seq", "u64", ""),
    row_column(
        "oldest_orphaned_occurred_at",
        "oldest_orphaned_occurred_at",
        "Option<String>",
        "",
    ),
];

const LOAD_REALTIME_SUBSCRIPTION_ROW_COLUMNS: &[RealtimePostgresRowColumn] = &[
    row_column("tenant_id", "tenant_id", "String", ""),
    row_column("organization_id", "organization_id", "String", ""),
    row_column("principal_kind", "principal_kind", "String", ""),
    row_column("principal_id", "principal_id", "String", ""),
    row_column("device_id", "device_id", "String", ""),
    row_column(
        "subscriptions_json",
        "items",
        "Vec<RealtimeSubscription>",
        "Deserialize JSON array into RealtimeSubscriptionRecord.items.",
    ),
    row_column(
        "synced_at",
        "synced_at",
        "String",
        "Map timestamptz to RFC3339 UTC string.",
    ),
];

const UPSERT_REALTIME_SUBSCRIPTION_BINDINGS: &[RealtimePostgresParameterBinding] = &[
    binding(1, "tenant_id", "&str", "text", ""),
    binding(2, "organization_id", "&str", "text", ""),
    binding(3, "client_route_scope_key", "String", "text", ""),
    binding(4, "principal_kind", "&str", "text", ""),
    binding(5, "principal_id", "&str", "text", ""),
    binding(6, "device_id", "&str", "text", ""),
    binding(
        7,
        "subscriptions_json",
        "&str",
        "jsonb",
        "Serialize RealtimeSubscriptionRecord.items.",
    ),
    binding(8, "subscription_count", "usize", "integer", ""),
    binding(9, "synced_at", "&str", "timestamptz", ""),
    binding(10, "created_at", "&str", "timestamptz", ""),
    binding(11, "updated_at", "&str", "timestamptz", ""),
];

const CLEAR_REALTIME_SUBSCRIPTION_IF_SYNCED_BINDINGS: &[RealtimePostgresParameterBinding] = &[
    binding(1, "tenant_id", "&str", "text", ""),
    binding(2, "organization_id", "&str", "text", ""),
    binding(3, "client_route_scope_key", "String", "text", ""),
    binding(4, "cutoff_synced_at", "&str", "timestamptz", ""),
];

const REPLACE_REALTIME_SUBSCRIPTION_SCOPES_BINDINGS: &[RealtimePostgresParameterBinding] = &[
    binding(1, "tenant_id", "&str", "text", ""),
    binding(2, "organization_id", "&str", "text", ""),
    binding(3, "principal_kind", "&str", "text", ""),
    binding(4, "principal_id", "&str", "text", ""),
    binding(5, "scope_type", "&str", "text", ""),
    binding(6, "scope_id", "&str", "text", ""),
    binding(
        7,
        "event_type",
        "&str",
        "text",
        "Use '*' when RealtimeSubscription.event_types is empty.",
    ),
    binding(8, "client_route_scope_key", "String", "text", ""),
    binding(9, "device_id", "&str", "text", ""),
    binding(10, "synced_at", "&str", "timestamptz", ""),
    binding(11, "created_at", "&str", "timestamptz", ""),
    binding(12, "updated_at", "&str", "timestamptz", ""),
];

const LOAD_MATCHING_REALTIME_SUBSCRIPTIONS_BINDINGS: &[RealtimePostgresParameterBinding] = &[
    binding(1, "tenant_id", "&str", "text", ""),
    binding(2, "organization_id", "&str", "text", ""),
    binding(3, "principal_kind", "&str", "text", ""),
    binding(4, "principal_id", "&str", "text", ""),
    binding(5, "scope_type", "&str", "text", ""),
    binding(6, "scope_id", "&str", "text", ""),
    binding(7, "event_type", "&str", "text", ""),
    binding(8, "candidate_device_ids", "&[String]", "text[]", ""),
];

const LOAD_REALTIME_DISCONNECT_FENCE_BINDINGS: &[RealtimePostgresParameterBinding] = &[
    binding(1, "tenant_id", "&str", "text", ""),
    binding(2, "organization_id", "&str", "text", ""),
    binding(3, "principal_kind", "&str", "text", ""),
    binding(4, "principal_id", "&str", "text", ""),
    binding(5, "device_id", "&str", "text", ""),
];

const LOAD_REALTIME_DISCONNECT_FENCE_ROW_COLUMNS: &[RealtimePostgresRowColumn] = &[
    row_column("tenant_id", "tenant_id", "String", ""),
    row_column("organization_id", "organization_id", "String", ""),
    row_column("principal_kind", "principal_kind", "String", ""),
    row_column("principal_id", "principal_id", "String", ""),
    row_column("device_id", "device_id", "String", ""),
    row_column("session_id", "session_id", "Option<String>", ""),
    row_column("owner_node_id", "owner_node_id", "String", ""),
    row_column(
        "disconnected_at",
        "disconnected_at",
        "String",
        "Map timestamptz to RFC3339 UTC string.",
    ),
    row_column("fence_token", "fence_token", "String", ""),
];

const UPSERT_REALTIME_DISCONNECT_FENCE_BINDINGS: &[RealtimePostgresParameterBinding] = &[
    binding(1, "tenant_id", "&str", "text", ""),
    binding(2, "organization_id", "&str", "text", ""),
    binding(3, "principal_kind", "&str", "text", ""),
    binding(4, "principal_id", "&str", "text", ""),
    binding(5, "device_id", "&str", "text", ""),
    binding(6, "session_id", "Option<&str>", "text", ""),
    binding(7, "owner_node_id", "&str", "text", ""),
    binding(8, "disconnected_at", "&str", "timestamptz", ""),
    binding(9, "fence_token", "&str", "text", ""),
    binding(
        10,
        "payload_json",
        "&str",
        "jsonb",
        "Serialize full RealtimeDisconnectFenceRecord for auditability.",
    ),
    binding(11, "payload_hash", "&str", "text", ""),
    binding(12, "created_at", "&str", "timestamptz", ""),
    binding(13, "updated_at", "&str", "timestamptz", ""),
];

const CLEAR_REALTIME_DISCONNECT_FENCE_BINDINGS: &[RealtimePostgresParameterBinding] = &[
    binding(1, "tenant_id", "&str", "text", ""),
    binding(2, "organization_id", "&str", "text", ""),
    binding(3, "principal_kind", "&str", "text", ""),
    binding(4, "principal_id", "&str", "text", ""),
    binding(5, "device_id", "&str", "text", ""),
];

const CLEAR_REALTIME_DISCONNECT_FENCE_IF_MATCHES_BINDINGS: &[RealtimePostgresParameterBinding] = &[
    binding(1, "tenant_id", "&str", "text", ""),
    binding(2, "organization_id", "&str", "text", ""),
    binding(3, "principal_kind", "&str", "text", ""),
    binding(4, "principal_id", "&str", "text", ""),
    binding(5, "device_id", "&str", "text", ""),
    binding(6, "fence_token", "&str", "text", ""),
];

const CLEAR_REALTIME_DISCONNECT_FENCE_AT_OR_BEFORE_BINDINGS: &[RealtimePostgresParameterBinding] =
    &[
        binding(1, "tenant_id", "&str", "text", ""),
        binding(2, "organization_id", "&str", "text", ""),
        binding(3, "principal_kind", "&str", "text", ""),
        binding(4, "principal_id", "&str", "text", ""),
        binding(5, "device_id", "&str", "text", ""),
        binding(6, "cutoff_disconnected_at", "&str", "timestamptz", ""),
    ];

pub const ALL_REALTIME_POSTGRES_SQL_CONTRACTS: &[&str] = &[
    LOAD_REALTIME_CHECKPOINT_SQL,
    UPSERT_REALTIME_CHECKPOINT_SQL,
    UPSERT_REALTIME_CLIENT_ROUTE_EVENT_SQL,
    LIST_REALTIME_CLIENT_ROUTE_EVENTS_SQL,
    TRIM_REALTIME_CLIENT_ROUTE_EVENTS_SQL,
    CLEAR_REALTIME_CLIENT_ROUTE_EVENTS_SQL,
    LOAD_REALTIME_EVENT_WINDOW_DIAGNOSTICS_SQL,
    LIST_REALTIME_EVENT_WINDOW_HIGH_RISK_WINDOWS_SQL,
    LIST_ORPHANED_REALTIME_CLIENT_ROUTE_EVENTS_SQL,
    LOAD_REALTIME_SUBSCRIPTION_SQL,
    UPSERT_REALTIME_SUBSCRIPTION_SQL,
    CLEAR_REALTIME_SUBSCRIPTION_SQL,
    CLEAR_REALTIME_SUBSCRIPTION_IF_SYNCED_AT_OR_BEFORE_SQL,
    CLEAR_REALTIME_SUBSCRIPTION_SCOPES_SQL,
    REPLACE_REALTIME_SUBSCRIPTION_SCOPES_SQL,
    LOAD_MATCHING_REALTIME_SUBSCRIPTIONS_SQL,
    LOAD_REALTIME_DISCONNECT_FENCE_SQL,
    UPSERT_REALTIME_DISCONNECT_FENCE_SQL,
    CLEAR_REALTIME_DISCONNECT_FENCE_SQL,
    CLEAR_REALTIME_DISCONNECT_FENCE_IF_MATCHES_SQL,
    CLEAR_REALTIME_DISCONNECT_FENCE_DISCONNECTED_AT_OR_BEFORE_SQL,
];

pub const ALL_REALTIME_POSTGRES_TRANSACTION_PLANS: &[&str] = &[
    PUBLISH_REALTIME_EVENTS_TRANSACTION_PLAN,
    ACK_REALTIME_EVENTS_TRANSACTION_PLAN,
    RESTORE_REALTIME_CLIENT_ROUTE_STATE_TRANSACTION_PLAN,
    TAKE_REALTIME_CLIENT_ROUTE_STATE_TRANSACTION_PLAN,
    SAVE_REALTIME_SUBSCRIPTIONS_TRANSACTION_PLAN,
    DISCONNECT_FENCE_TRANSACTION_PLAN,
];

pub const REALTIME_POSTGRES_SQL_CONTRACT_SPECS: &[RealtimePostgresSqlContract] = &[
    RealtimePostgresSqlContract {
        name: "LOAD_REALTIME_CHECKPOINT_SQL",
        sql: LOAD_REALTIME_CHECKPOINT_SQL,
        parameter_bindings: DEVICE_SCOPE_BINDINGS,
        row_mapping: Some(RealtimePostgresRowMapping {
            target: "RealtimeCheckpointRecord",
            columns: LOAD_REALTIME_CHECKPOINT_ROW_COLUMNS,
        }),
    },
    RealtimePostgresSqlContract {
        name: "UPSERT_REALTIME_CHECKPOINT_SQL",
        sql: UPSERT_REALTIME_CHECKPOINT_SQL,
        parameter_bindings: UPSERT_REALTIME_CHECKPOINT_BINDINGS,
        row_mapping: None,
    },
    RealtimePostgresSqlContract {
        name: "UPSERT_REALTIME_CLIENT_ROUTE_EVENT_SQL",
        sql: UPSERT_REALTIME_CLIENT_ROUTE_EVENT_SQL,
        parameter_bindings: UPSERT_REALTIME_CLIENT_ROUTE_EVENT_BINDINGS,
        row_mapping: None,
    },
    RealtimePostgresSqlContract {
        name: "LIST_REALTIME_CLIENT_ROUTE_EVENTS_SQL",
        sql: LIST_REALTIME_CLIENT_ROUTE_EVENTS_SQL,
        parameter_bindings: LIST_REALTIME_CLIENT_ROUTE_EVENTS_BINDINGS,
        row_mapping: Some(RealtimePostgresRowMapping {
            target: "RealtimeEvent",
            columns: LIST_REALTIME_CLIENT_ROUTE_EVENTS_ROW_COLUMNS,
        }),
    },
    RealtimePostgresSqlContract {
        name: "TRIM_REALTIME_CLIENT_ROUTE_EVENTS_SQL",
        sql: TRIM_REALTIME_CLIENT_ROUTE_EVENTS_SQL,
        parameter_bindings: TRIM_REALTIME_CLIENT_ROUTE_EVENTS_BINDINGS,
        row_mapping: None,
    },
    RealtimePostgresSqlContract {
        name: "CLEAR_REALTIME_CLIENT_ROUTE_EVENTS_SQL",
        sql: CLEAR_REALTIME_CLIENT_ROUTE_EVENTS_SQL,
        parameter_bindings: DEVICE_SCOPE_BINDINGS,
        row_mapping: None,
    },
    RealtimePostgresSqlContract {
        name: "LOAD_REALTIME_EVENT_WINDOW_DIAGNOSTICS_SQL",
        sql: LOAD_REALTIME_EVENT_WINDOW_DIAGNOSTICS_SQL,
        parameter_bindings: &[],
        row_mapping: Some(RealtimePostgresRowMapping {
            target: "RealtimeEventWindowDiagnosticsSnapshot",
            columns: REALTIME_EVENT_WINDOW_DIAGNOSTICS_ROW_COLUMNS,
        }),
    },
    RealtimePostgresSqlContract {
        name: "LIST_REALTIME_EVENT_WINDOW_HIGH_RISK_WINDOWS_SQL",
        sql: LIST_REALTIME_EVENT_WINDOW_HIGH_RISK_WINDOWS_SQL,
        parameter_bindings: &[],
        row_mapping: Some(RealtimePostgresRowMapping {
            target: "RealtimeEventWindowHighRiskRecord",
            columns: REALTIME_EVENT_WINDOW_HIGH_RISK_ROW_COLUMNS,
        }),
    },
    RealtimePostgresSqlContract {
        name: "LIST_ORPHANED_REALTIME_CLIENT_ROUTE_EVENTS_SQL",
        sql: LIST_ORPHANED_REALTIME_CLIENT_ROUTE_EVENTS_SQL,
        parameter_bindings: ORPHANED_LIMIT_BINDINGS,
        row_mapping: Some(RealtimePostgresRowMapping {
            target: "RealtimePostgresOrphanedClientRouteEventsDiagnostic",
            columns: ORPHANED_REALTIME_CLIENT_ROUTE_EVENTS_ROW_COLUMNS,
        }),
    },
    RealtimePostgresSqlContract {
        name: "LOAD_REALTIME_SUBSCRIPTION_SQL",
        sql: LOAD_REALTIME_SUBSCRIPTION_SQL,
        parameter_bindings: DEVICE_SCOPE_BINDINGS,
        row_mapping: Some(RealtimePostgresRowMapping {
            target: "RealtimeSubscriptionRecord",
            columns: LOAD_REALTIME_SUBSCRIPTION_ROW_COLUMNS,
        }),
    },
    RealtimePostgresSqlContract {
        name: "UPSERT_REALTIME_SUBSCRIPTION_SQL",
        sql: UPSERT_REALTIME_SUBSCRIPTION_SQL,
        parameter_bindings: UPSERT_REALTIME_SUBSCRIPTION_BINDINGS,
        row_mapping: None,
    },
    RealtimePostgresSqlContract {
        name: "CLEAR_REALTIME_SUBSCRIPTION_SQL",
        sql: CLEAR_REALTIME_SUBSCRIPTION_SQL,
        parameter_bindings: DEVICE_SCOPE_BINDINGS,
        row_mapping: None,
    },
    RealtimePostgresSqlContract {
        name: "CLEAR_REALTIME_SUBSCRIPTION_IF_SYNCED_AT_OR_BEFORE_SQL",
        sql: CLEAR_REALTIME_SUBSCRIPTION_IF_SYNCED_AT_OR_BEFORE_SQL,
        parameter_bindings: CLEAR_REALTIME_SUBSCRIPTION_IF_SYNCED_BINDINGS,
        row_mapping: None,
    },
    RealtimePostgresSqlContract {
        name: "CLEAR_REALTIME_SUBSCRIPTION_SCOPES_SQL",
        sql: CLEAR_REALTIME_SUBSCRIPTION_SCOPES_SQL,
        parameter_bindings: CLEAR_REALTIME_SUBSCRIPTION_IF_SYNCED_BINDINGS,
        row_mapping: None,
    },
    RealtimePostgresSqlContract {
        name: "REPLACE_REALTIME_SUBSCRIPTION_SCOPES_SQL",
        sql: REPLACE_REALTIME_SUBSCRIPTION_SCOPES_SQL,
        parameter_bindings: REPLACE_REALTIME_SUBSCRIPTION_SCOPES_BINDINGS,
        row_mapping: None,
    },
    RealtimePostgresSqlContract {
        name: "LOAD_MATCHING_REALTIME_SUBSCRIPTIONS_SQL",
        sql: LOAD_MATCHING_REALTIME_SUBSCRIPTIONS_SQL,
        parameter_bindings: LOAD_MATCHING_REALTIME_SUBSCRIPTIONS_BINDINGS,
        row_mapping: Some(RealtimePostgresRowMapping {
            target: "RealtimeSubscriptionRecord",
            columns: LOAD_REALTIME_SUBSCRIPTION_ROW_COLUMNS,
        }),
    },
    RealtimePostgresSqlContract {
        name: "LOAD_REALTIME_DISCONNECT_FENCE_SQL",
        sql: LOAD_REALTIME_DISCONNECT_FENCE_SQL,
        parameter_bindings: LOAD_REALTIME_DISCONNECT_FENCE_BINDINGS,
        row_mapping: Some(RealtimePostgresRowMapping {
            target: "RealtimeDisconnectFenceRecord",
            columns: LOAD_REALTIME_DISCONNECT_FENCE_ROW_COLUMNS,
        }),
    },
    RealtimePostgresSqlContract {
        name: "UPSERT_REALTIME_DISCONNECT_FENCE_SQL",
        sql: UPSERT_REALTIME_DISCONNECT_FENCE_SQL,
        parameter_bindings: UPSERT_REALTIME_DISCONNECT_FENCE_BINDINGS,
        row_mapping: None,
    },
    RealtimePostgresSqlContract {
        name: "CLEAR_REALTIME_DISCONNECT_FENCE_SQL",
        sql: CLEAR_REALTIME_DISCONNECT_FENCE_SQL,
        parameter_bindings: CLEAR_REALTIME_DISCONNECT_FENCE_BINDINGS,
        row_mapping: None,
    },
    RealtimePostgresSqlContract {
        name: "CLEAR_REALTIME_DISCONNECT_FENCE_IF_MATCHES_SQL",
        sql: CLEAR_REALTIME_DISCONNECT_FENCE_IF_MATCHES_SQL,
        parameter_bindings: CLEAR_REALTIME_DISCONNECT_FENCE_IF_MATCHES_BINDINGS,
        row_mapping: None,
    },
    RealtimePostgresSqlContract {
        name: "CLEAR_REALTIME_DISCONNECT_FENCE_DISCONNECTED_AT_OR_BEFORE_SQL",
        sql: CLEAR_REALTIME_DISCONNECT_FENCE_DISCONNECTED_AT_OR_BEFORE_SQL,
        parameter_bindings: CLEAR_REALTIME_DISCONNECT_FENCE_AT_OR_BEFORE_BINDINGS,
        row_mapping: None,
    },
];

const SAVE_CHECKPOINT_STEPS: &[RealtimePostgresMethodStep] = &[step(
    "UPSERT_REALTIME_CHECKPOINT_SQL",
    "Each RealtimeCheckpointRecord plus derived client_route_scope_key, created_at, and updated_at.",
    "No rows returned; use affected-row result only for diagnostics.",
)];

const LOAD_CHECKPOINT_STEPS: &[RealtimePostgresMethodStep] = &[step(
    "LOAD_REALTIME_CHECKPOINT_SQL",
    "tenant_id, organization_id, and derived client_route_scope_key.",
    "Map optional row to RealtimeCheckpointRecord and normalize sequence metadata.",
)];

const LOAD_WINDOW_STEPS: &[RealtimePostgresMethodStep] = &[
    step(
        "LOAD_REALTIME_CHECKPOINT_SQL",
        "tenant_id, organization_id, and derived client_route_scope_key.",
        "Provides principal/device identity and trim metadata.",
    ),
    step(
        "LIST_REALTIME_CLIENT_ROUTE_EVENTS_SQL",
        "tenant_id, derived client_route_scope_key, checkpoint.trimmed_through_seq, and bounded limit.",
        "Map rows to RealtimeEvent values ordered by realtime_seq.",
    ),
];

const SAVE_WINDOW_STEPS: &[RealtimePostgresMethodStep] = &[
    step(
        "CLEAR_REALTIME_CLIENT_ROUTE_EVENTS_SQL",
        "RealtimeEventWindowRecord tenant_id and derived client_route_scope_key.",
        "Clears previous durable window before inserting replacement rows in the same transaction.",
    ),
    step(
        "UPSERT_REALTIME_CLIENT_ROUTE_EVENT_SQL",
        "Each RealtimeEventWindowRecord event plus payload_hash, created_at, and retention_until.",
        "No rows returned.",
    ),
    step(
        "UPSERT_REALTIME_CHECKPOINT_SQL",
        "RealtimeEventWindowRecord trim metadata converted to RealtimeCheckpointRecord-compatible fields.",
        "No rows returned.",
    ),
];

const CLEAR_WINDOW_STEPS: &[RealtimePostgresMethodStep] = &[step(
    "CLEAR_REALTIME_CLIENT_ROUTE_EVENTS_SQL",
    "tenant_id and derived client_route_scope_key.",
    "Return true when at least one event row was deleted.",
)];

const DIAGNOSTICS_STEPS: &[RealtimePostgresMethodStep] = &[
    step(
        "LOAD_REALTIME_EVENT_WINDOW_DIAGNOSTICS_SQL",
        "No parameters.",
        "Map single aggregate row to RealtimeEventWindowDiagnosticsSnapshot.",
    ),
    step(
        "LIST_REALTIME_EVENT_WINDOW_HIGH_RISK_WINDOWS_SQL",
        "No parameters.",
        "Map rows to high_risk_windows.",
    ),
];

const TRIM_WINDOW_STEPS: &[RealtimePostgresMethodStep] = &[
    step(
        "TRIM_REALTIME_CLIENT_ROUTE_EVENTS_SQL",
        "tenant_id, derived client_route_scope_key, and acked_through_seq.",
        "No rows returned.",
    ),
    step(
        "UPSERT_REALTIME_CHECKPOINT_SQL",
        "Existing checkpoint merged with acked/trimmed sequence metadata.",
        "No rows returned.",
    ),
];

const LOAD_SUBSCRIPTION_STEPS: &[RealtimePostgresMethodStep] = &[step(
    "LOAD_REALTIME_SUBSCRIPTION_SQL",
    "tenant_id and derived client_route_scope_key.",
    "Map optional row to RealtimeSubscriptionRecord.",
)];

const LOAD_MATCHING_SUBSCRIPTION_STEPS: &[RealtimePostgresMethodStep] = &[step(
    "LOAD_MATCHING_REALTIME_SUBSCRIPTIONS_SQL",
    "tenant/principal/scope/event tuple and candidate_device_ids text array.",
    "Map distinct rows to RealtimeSubscriptionRecord.",
)];

const SAVE_SUBSCRIPTION_STEPS: &[RealtimePostgresMethodStep] = &[
    step(
        "UPSERT_REALTIME_SUBSCRIPTION_SQL",
        "RealtimeSubscriptionRecord plus serialized subscriptions_json.",
        "No rows returned.",
    ),
    step(
        "CLEAR_REALTIME_SUBSCRIPTION_SCOPES_SQL",
        "tenant_id, derived client_route_scope_key, and attempted synced_at.",
        "Deletes only fanout rows no newer than this sync attempt.",
    ),
    step(
        "REPLACE_REALTIME_SUBSCRIPTION_SCOPES_SQL",
        "One derived row per scope/event type fanout item.",
        "No rows returned.",
    ),
];

const CLEAR_SUBSCRIPTION_STEPS: &[RealtimePostgresMethodStep] = &[step(
    "CLEAR_REALTIME_SUBSCRIPTION_SQL",
    "tenant_id and derived client_route_scope_key.",
    "Cascade deletes fanout rows through schema foreign key.",
)];

const CLEAR_SUBSCRIPTION_IF_SYNCED_STEPS: &[RealtimePostgresMethodStep] = &[step(
    "CLEAR_REALTIME_SUBSCRIPTION_IF_SYNCED_AT_OR_BEFORE_SQL",
    "tenant_id, derived client_route_scope_key, and cutoff_synced_at.",
    "Cascade deletes fanout rows only when the persisted subscription is not newer.",
)];

const LOAD_FENCE_STEPS: &[RealtimePostgresMethodStep] = &[step(
    "LOAD_REALTIME_DISCONNECT_FENCE_SQL",
    "tenant_id, principal_kind, principal_id, and device_id.",
    "Map optional row to RealtimeDisconnectFenceRecord.",
)];

const SAVE_FENCE_STEPS: &[RealtimePostgresMethodStep] = &[step(
    "UPSERT_REALTIME_DISCONNECT_FENCE_SQL",
    "RealtimeDisconnectFenceRecord plus payload_json, payload_hash, created_at, and updated_at.",
    "No rows returned; stale disconnected_at writes are ignored by SQL.",
)];

const CLEAR_FENCE_STEPS: &[RealtimePostgresMethodStep] = &[step(
    "CLEAR_REALTIME_DISCONNECT_FENCE_SQL",
    "tenant_id, principal_kind, principal_id, and device_id.",
    "Return true when one row was deleted.",
)];

const CLEAR_FENCE_AT_OR_BEFORE_STEPS: &[RealtimePostgresMethodStep] = &[step(
    "CLEAR_REALTIME_DISCONNECT_FENCE_DISCONNECTED_AT_OR_BEFORE_SQL",
    "tenant_id, principal_kind, principal_id, device_id, and cutoff_disconnected_at.",
    "Return true when one row was deleted.",
)];

const CLEAR_FENCE_IF_MATCHES_STEPS: &[RealtimePostgresMethodStep] = &[step(
    "CLEAR_REALTIME_DISCONNECT_FENCE_IF_MATCHES_SQL",
    "Expected RealtimeDisconnectFenceRecord identity and fence_token.",
    "Return true when the expected fence was deleted.",
)];

const RESTORE_CLIENT_ROUTE_STATE_STEPS: &[RealtimePostgresMethodStep] = &[
    step(
        "UPSERT_REALTIME_SUBSCRIPTION_SQL",
        "Optional restored RealtimeSubscriptionRecord.",
        "Skipped when restored subscription list is empty; CLEAR_REALTIME_SUBSCRIPTION_SQL is used instead.",
    ),
    step(
        "CLEAR_REALTIME_SUBSCRIPTION_SCOPES_SQL",
        "Restored subscription synced_at cutoff.",
        "Clears stale fanout rows before replacement in the same transaction.",
    ),
    step(
        "REPLACE_REALTIME_SUBSCRIPTION_SCOPES_SQL",
        "Derived subscription fanout rows.",
        "No rows returned.",
    ),
    step(
        "UPSERT_REALTIME_CHECKPOINT_SQL",
        "Restored normalized checkpoint metadata.",
        "No rows returned.",
    ),
    step(
        "CLEAR_REALTIME_CLIENT_ROUTE_EVENTS_SQL",
        "tenant_id and derived client_route_scope_key.",
        "Clears previous event window before restored rows are inserted.",
    ),
    step(
        "UPSERT_REALTIME_CLIENT_ROUTE_EVENT_SQL",
        "Each restored event.",
        "No rows returned.",
    ),
];

const TAKE_CLIENT_ROUTE_STATE_STEPS: &[RealtimePostgresMethodStep] = &[
    step(
        "LOAD_REALTIME_CHECKPOINT_SQL",
        "tenant_id and derived client_route_scope_key.",
        "Maps checkpoint portion of RealtimeClientRouteStateSnapshot.",
    ),
    step(
        "LIST_REALTIME_CLIENT_ROUTE_EVENTS_SQL",
        "tenant_id, derived client_route_scope_key, after_seq=0, and bounded limit.",
        "Maps events portion of RealtimeClientRouteStateSnapshot.",
    ),
    step(
        "LOAD_REALTIME_SUBSCRIPTION_SQL",
        "tenant_id and derived client_route_scope_key.",
        "Maps subscriptions portion of RealtimeClientRouteStateSnapshot.",
    ),
    step(
        "CLEAR_REALTIME_CLIENT_ROUTE_EVENTS_SQL",
        "tenant_id and derived client_route_scope_key.",
        "Deletes source event window after snapshot rows are read.",
    ),
    step(
        "CLEAR_REALTIME_SUBSCRIPTION_SCOPES_SQL",
        "tenant_id, derived client_route_scope_key, and snapshot subscription synced_at or transaction timestamp.",
        "Clears source subscription fanout rows.",
    ),
    step(
        "CLEAR_REALTIME_SUBSCRIPTION_SQL",
        "tenant_id and derived client_route_scope_key.",
        "Deletes source subscription row after snapshot rows are read.",
    ),
];

const PUBLISH_STEPS: &[RealtimePostgresMethodStep] = &[
    step(
        "UPSERT_REALTIME_CLIENT_ROUTE_EVENT_SQL",
        "Each delivered client route event mutation.",
        "No rows returned.",
    ),
    step(
        "UPSERT_REALTIME_CHECKPOINT_SQL",
        "Each affected device checkpoint mutation.",
        "No rows returned.",
    ),
];

const ACK_STEPS: &[RealtimePostgresMethodStep] = &[
    step(
        "TRIM_REALTIME_CLIENT_ROUTE_EVENTS_SQL",
        "tenant_id, derived client_route_scope_key, and effective acked_through_seq.",
        "No rows returned.",
    ),
    step(
        "UPSERT_REALTIME_CHECKPOINT_SQL",
        "Checkpoint with monotonic acked/trimmed sequence metadata.",
        "No rows returned.",
    ),
];

const ATOMIC_EVENT_WINDOW_NOTES: &[&str] = &[
    "Event window rows and checkpoint rows must never be committed separately.",
    "The adapter must rollback the whole transaction on any statement or serialization error.",
];

const ATOMIC_SUBSCRIPTION_NOTES: &[&str] = &[
    "Subscription row and fanout scope rows must be committed in one transaction.",
    "Stale retries must not delete or reinsert fanout rows after a newer synced_at wins.",
];

const EMPTY_NOTES: &[&str] = &[];

pub const REALTIME_POSTGRES_METHOD_PLANS: &[RealtimePostgresMethodPlan] = &[
    RealtimePostgresMethodPlan {
        name: "RealtimeCheckpointStore::load_checkpoint",
        atomicity: RealtimePostgresMethodAtomicity::ReadOnly,
        transaction_plan_name: None,
        steps: LOAD_CHECKPOINT_STEPS,
        notes: EMPTY_NOTES,
    },
    RealtimePostgresMethodPlan {
        name: "RealtimeCheckpointStore::save_checkpoints",
        atomicity: RealtimePostgresMethodAtomicity::Transaction,
        transaction_plan_name: None,
        steps: SAVE_CHECKPOINT_STEPS,
        notes: &["Batch checkpoint saves should execute in one transaction."],
    },
    RealtimePostgresMethodPlan {
        name: "RealtimeEventWindowStore::load_window",
        atomicity: RealtimePostgresMethodAtomicity::ReadOnly,
        transaction_plan_name: None,
        steps: LOAD_WINDOW_STEPS,
        notes: &["Checkpoint and event rows should be read from one consistent snapshot."],
    },
    RealtimePostgresMethodPlan {
        name: "RealtimeEventWindowStore::save_windows",
        atomicity: RealtimePostgresMethodAtomicity::Transaction,
        transaction_plan_name: None,
        steps: SAVE_WINDOW_STEPS,
        notes: ATOMIC_EVENT_WINDOW_NOTES,
    },
    RealtimePostgresMethodPlan {
        name: "RealtimeEventWindowStore::clear_window",
        atomicity: RealtimePostgresMethodAtomicity::SingleStatement,
        transaction_plan_name: None,
        steps: CLEAR_WINDOW_STEPS,
        notes: EMPTY_NOTES,
    },
    RealtimePostgresMethodPlan {
        name: "RealtimeEventWindowStore::diagnostics_snapshot",
        atomicity: RealtimePostgresMethodAtomicity::ReadOnly,
        transaction_plan_name: None,
        steps: DIAGNOSTICS_STEPS,
        notes: &["Diagnostic queries must never select payload_json."],
    },
    RealtimePostgresMethodPlan {
        name: "RealtimeEventWindowStore::trim_window",
        atomicity: RealtimePostgresMethodAtomicity::Transaction,
        transaction_plan_name: Some("ACK_REALTIME_EVENTS_TRANSACTION_PLAN"),
        steps: TRIM_WINDOW_STEPS,
        notes: ATOMIC_EVENT_WINDOW_NOTES,
    },
    RealtimePostgresMethodPlan {
        name: "RealtimeSubscriptionStore::load_subscriptions",
        atomicity: RealtimePostgresMethodAtomicity::ReadOnly,
        transaction_plan_name: None,
        steps: LOAD_SUBSCRIPTION_STEPS,
        notes: EMPTY_NOTES,
    },
    RealtimePostgresMethodPlan {
        name: "RealtimeSubscriptionStore::load_matching_subscriptions",
        atomicity: RealtimePostgresMethodAtomicity::ReadOnly,
        transaction_plan_name: None,
        steps: LOAD_MATCHING_SUBSCRIPTION_STEPS,
        notes: &["Fanout lookup must use the indexed scope table, including '*' wildcard rows."],
    },
    RealtimePostgresMethodPlan {
        name: "RealtimeSubscriptionStore::save_subscriptions",
        atomicity: RealtimePostgresMethodAtomicity::Transaction,
        transaction_plan_name: Some("SAVE_REALTIME_SUBSCRIPTIONS_TRANSACTION_PLAN"),
        steps: SAVE_SUBSCRIPTION_STEPS,
        notes: ATOMIC_SUBSCRIPTION_NOTES,
    },
    RealtimePostgresMethodPlan {
        name: "RealtimeSubscriptionStore::clear_subscriptions",
        atomicity: RealtimePostgresMethodAtomicity::SingleStatement,
        transaction_plan_name: None,
        steps: CLEAR_SUBSCRIPTION_STEPS,
        notes: &["Schema cascade deletes subscription scope fanout rows."],
    },
    RealtimePostgresMethodPlan {
        name: "RealtimeSubscriptionStore::clear_subscriptions_synced_at_or_before",
        atomicity: RealtimePostgresMethodAtomicity::SingleStatement,
        transaction_plan_name: None,
        steps: CLEAR_SUBSCRIPTION_IF_SYNCED_STEPS,
        notes: &[
            "Used by restore compensation and disconnect cleanup to avoid deleting newer subscriptions.",
        ],
    },
    RealtimePostgresMethodPlan {
        name: "RealtimeDisconnectFenceStore::load_fence",
        atomicity: RealtimePostgresMethodAtomicity::ReadOnly,
        transaction_plan_name: None,
        steps: LOAD_FENCE_STEPS,
        notes: EMPTY_NOTES,
    },
    RealtimePostgresMethodPlan {
        name: "RealtimeDisconnectFenceStore::save_fence",
        atomicity: RealtimePostgresMethodAtomicity::SingleStatement,
        transaction_plan_name: Some("DISCONNECT_FENCE_TRANSACTION_PLAN"),
        steps: SAVE_FENCE_STEPS,
        notes: &["Upsert is monotonic by disconnected_at."],
    },
    RealtimePostgresMethodPlan {
        name: "RealtimeDisconnectFenceStore::clear_fence",
        atomicity: RealtimePostgresMethodAtomicity::SingleStatement,
        transaction_plan_name: Some("DISCONNECT_FENCE_TRANSACTION_PLAN"),
        steps: CLEAR_FENCE_STEPS,
        notes: EMPTY_NOTES,
    },
    RealtimePostgresMethodPlan {
        name: "RealtimeDisconnectFenceStore::clear_fence_disconnected_at_or_before",
        atomicity: RealtimePostgresMethodAtomicity::SingleStatement,
        transaction_plan_name: Some("DISCONNECT_FENCE_TRANSACTION_PLAN"),
        steps: CLEAR_FENCE_AT_OR_BEFORE_STEPS,
        notes: EMPTY_NOTES,
    },
    RealtimePostgresMethodPlan {
        name: "RealtimeDisconnectFenceStore::clear_fence_if_matches",
        atomicity: RealtimePostgresMethodAtomicity::SingleStatement,
        transaction_plan_name: Some("DISCONNECT_FENCE_TRANSACTION_PLAN"),
        steps: CLEAR_FENCE_IF_MATCHES_STEPS,
        notes: &["Compare-and-delete by fence_token to avoid clearing a newer fence."],
    },
    RealtimePostgresMethodPlan {
        name: "RealtimeDeliveryRuntime::restore_client_route_state",
        atomicity: RealtimePostgresMethodAtomicity::Transaction,
        transaction_plan_name: Some("RESTORE_REALTIME_CLIENT_ROUTE_STATE_TRANSACTION_PLAN"),
        steps: RESTORE_CLIENT_ROUTE_STATE_STEPS,
        notes: &["Restore must not expose partial subscription/checkpoint/event-window state."],
    },
    RealtimePostgresMethodPlan {
        name: "RealtimeDeliveryRuntime::take_client_route_state",
        atomicity: RealtimePostgresMethodAtomicity::Transaction,
        transaction_plan_name: Some("TAKE_REALTIME_CLIENT_ROUTE_STATE_TRANSACTION_PLAN"),
        steps: TAKE_CLIENT_ROUTE_STATE_STEPS,
        notes: &[
            "Take must read source state and delete source durable windows in one transaction.",
        ],
    },
    RealtimePostgresMethodPlan {
        name: "RealtimeDeliveryRuntime::publish_scope_event",
        atomicity: RealtimePostgresMethodAtomicity::Transaction,
        transaction_plan_name: Some("PUBLISH_REALTIME_EVENTS_TRANSACTION_PLAN"),
        steps: PUBLISH_STEPS,
        notes: ATOMIC_EVENT_WINDOW_NOTES,
    },
    RealtimePostgresMethodPlan {
        name: "RealtimeDeliveryRuntime::ack_events",
        atomicity: RealtimePostgresMethodAtomicity::Transaction,
        transaction_plan_name: Some("ACK_REALTIME_EVENTS_TRANSACTION_PLAN"),
        steps: ACK_STEPS,
        notes: ATOMIC_EVENT_WINDOW_NOTES,
    },
];

pub const REALTIME_POSTGRES_ADAPTER_PLAN: RealtimePostgresAdapterPlan =
    RealtimePostgresAdapterPlan {
        runtime_status: "store_adapter_implemented",
        runtime_status_reason: "PostgreSQL driver-backed realtime storage now implements the individual store traits; runtime-level multi-store transactions remain adapter contracts until a composed runtime uses one shared transaction boundary.",
        sql_contracts: REALTIME_POSTGRES_SQL_CONTRACT_SPECS,
        transaction_plans: ALL_REALTIME_POSTGRES_TRANSACTION_PLANS,
        method_plans: REALTIME_POSTGRES_METHOD_PLANS,
    };
