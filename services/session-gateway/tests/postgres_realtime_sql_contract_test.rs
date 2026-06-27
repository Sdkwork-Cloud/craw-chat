use session_gateway::{
    RealtimePostgresBindingValue, RealtimePostgresCheckpointMutation,
    RealtimePostgresClientRouteEventMutation, RealtimePostgresMethodAtomicity,
    realtime_postgres_adapter_plan, realtime_postgres_bind_ack_transaction,
    realtime_postgres_bind_checkpoint_upsert, realtime_postgres_bind_client_route_event_upsert,
    realtime_postgres_bind_publish_transaction,
    realtime_postgres_bind_save_subscription_transaction,
    realtime_postgres_bind_subscription_scope_clear,
    realtime_postgres_bind_subscription_scope_replacements,
    realtime_postgres_bind_subscription_upsert, realtime_postgres_bind_trim_client_route_events,
    realtime_postgres_sql_contract_specs,
};

use im_domain_core::realtime::{RealtimeEvent, RealtimeSubscription};
use sdkwork_im_contract_control::{RealtimeCheckpointRecord, RealtimeSubscriptionRecord};

fn postgres_realtime_sql_source() -> String {
    include_str!("../../../crates/im-postgres-realtime-contracts/src/lib.rs")
        .replace("\r\n", "\n")
        .to_lowercase()
}

#[test]
fn test_session_gateway_reexports_shared_postgres_realtime_contracts() {
    let source = include_str!("../src/realtime/postgres_sql.rs");

    assert!(
        source.contains("pub use im_postgres_realtime_contracts::*;"),
        "session-gateway must re-export the shared PostgreSQL realtime contract crate"
    );
    assert!(
        !source.contains("pub const LOAD_REALTIME_CHECKPOINT_SQL"),
        "session-gateway must not own duplicate PostgreSQL realtime SQL constants"
    );
}

fn constant_source<'a>(source: &'a str, constant_name: &str) -> &'a str {
    let marker = format!("pub const {constant_name}: &str");
    let start = source
        .find(marker.as_str())
        .unwrap_or_else(|| panic!("PostgreSQL realtime SQL contract is missing {constant_name}"));
    let tail = &source[start..];
    let next_start = tail[marker.len()..]
        .find("\npub const ")
        .map(|offset| marker.len() + offset)
        .unwrap_or(tail.len());
    &tail[..next_start]
}

fn assert_contains_all(source: &str, expected: &[&str]) {
    for item in expected {
        assert!(
            source.contains(item),
            "PostgreSQL realtime SQL contract is missing required fragment: {item}"
        );
    }
}

fn placeholder_numbers(source: &str) -> Vec<u32> {
    let bytes = source.as_bytes();
    let mut numbers = Vec::new();
    let mut index = 0;
    while index < bytes.len() {
        if bytes[index] != b'$' {
            index += 1;
            continue;
        }
        index += 1;
        let start = index;
        while index < bytes.len() && bytes[index].is_ascii_digit() {
            index += 1;
        }
        if start == index {
            continue;
        }
        let number = source[start..index]
            .parse::<u32>()
            .expect("PostgreSQL SQL placeholders should be numeric");
        numbers.push(number);
    }
    numbers.sort_unstable();
    numbers.dedup();
    numbers
}

fn assert_uses_contiguous_placeholders(source: &str, expected_max: u32) {
    let expected = if expected_max == 0 {
        Vec::new()
    } else {
        (1..=expected_max).collect::<Vec<_>>()
    };
    assert_eq!(
        placeholder_numbers(source),
        expected,
        "PostgreSQL SQL placeholders must be contiguous from $1 to ${expected_max}"
    );
}

fn assert_plan_contains_steps(source: &str, constant_name: &str, expected_steps: &[&str]) {
    let plan = constant_source(source, constant_name);
    assert_contains_all(plan, expected_steps);
}

fn sql_spec<'a>(
    specs: &'a [session_gateway::RealtimePostgresSqlContract],
    name: &str,
) -> &'a session_gateway::RealtimePostgresSqlContract {
    specs
        .iter()
        .find(|spec| spec.name == name)
        .unwrap_or_else(|| panic!("PostgreSQL realtime SQL spec is missing {name}"))
}

fn method_plan<'a>(
    plan: &'a session_gateway::RealtimePostgresAdapterPlan,
    name: &str,
) -> &'a session_gateway::RealtimePostgresMethodPlan {
    plan.method_plans
        .iter()
        .find(|method| method.name == name)
        .unwrap_or_else(|| panic!("PostgreSQL realtime adapter plan is missing {name}"))
}

fn binding_names(spec: &session_gateway::RealtimePostgresSqlContract) -> Vec<&'static str> {
    spec.parameter_bindings
        .iter()
        .map(|binding| binding.name)
        .collect()
}

fn row_columns(spec: &session_gateway::RealtimePostgresSqlContract) -> Vec<&'static str> {
    spec.row_mapping
        .expect("SQL spec should define a result row mapping")
        .columns
        .iter()
        .map(|column| column.column)
        .collect()
}

fn step_sql_names(method: &session_gateway::RealtimePostgresMethodPlan) -> Vec<&'static str> {
    method
        .steps
        .iter()
        .map(|step| step.sql_contract_name)
        .collect()
}

fn bound_values(
    statement: &session_gateway::RealtimePostgresBoundStatement,
) -> Vec<RealtimePostgresBindingValue> {
    statement
        .parameters
        .iter()
        .map(|parameter| parameter.value.clone())
        .collect()
}

fn bound_names(statement: &session_gateway::RealtimePostgresBoundStatement) -> Vec<&'static str> {
    statement
        .parameters
        .iter()
        .map(|parameter| parameter.name)
        .collect()
}

fn transaction_sql_names(
    transaction: &session_gateway::RealtimePostgresBoundTransaction,
) -> Vec<&'static str> {
    transaction
        .statements
        .iter()
        .map(|statement| statement.sql_name)
        .collect()
}

fn assert_sql_spec_bindings_match_placeholders(
    spec: &session_gateway::RealtimePostgresSqlContract,
) {
    let placeholders = placeholder_numbers(spec.sql);
    assert_eq!(
        spec.parameter_bindings.len(),
        placeholders.len(),
        "{name} must define one binding for every SQL placeholder",
        name = spec.name
    );
    for (index, binding) in spec.parameter_bindings.iter().enumerate() {
        assert_eq!(
            binding.position as usize,
            index + 1,
            "{name} binding positions must be contiguous and ordered",
            name = spec.name
        );
        assert!(
            placeholders.contains(&binding.position),
            "{name} binding ${position} is not present in SQL",
            name = spec.name,
            position = binding.position
        );
        assert!(
            !binding.name.trim().is_empty(),
            "{name} binding names must be explicit",
            name = spec.name
        );
        assert!(
            !binding.rust_type.trim().is_empty(),
            "{name} binding Rust types must be explicit",
            name = spec.name
        );
        assert!(
            !binding.postgres_type.trim().is_empty(),
            "{name} binding PostgreSQL types must be explicit",
            name = spec.name
        );
    }
}

fn assert_method_references_known_sql(
    method: &session_gateway::RealtimePostgresMethodPlan,
    specs: &[session_gateway::RealtimePostgresSqlContract],
) {
    for step in method.steps {
        sql_spec(specs, step.sql_contract_name);
        assert!(
            !step.binding_source.trim().is_empty(),
            "{method} step {sql} must document its binding source",
            method = method.name,
            sql = step.sql_contract_name
        );
    }
}

#[test]
fn test_postgres_realtime_checkpoint_sql_preserves_monotonic_capacity_trim_metadata() {
    let source = postgres_realtime_sql_source();
    let upsert = constant_source(&source, "upsert_realtime_checkpoint_sql");
    let load = constant_source(&source, "load_realtime_checkpoint_sql");

    assert_contains_all(
        upsert,
        &[
            "pub const upsert_realtime_checkpoint_sql",
            "insert into im_realtime_checkpoints",
            "capacity_trimmed_event_count",
            "capacity_trimmed_through_seq",
            "last_capacity_trimmed_at",
            "on conflict (tenant_id, organization_id, client_route_scope_key) do update",
            "latest_realtime_seq = greatest(",
            "acked_through_seq = greatest(",
            "trimmed_through_seq = greatest(",
            "capacity_trimmed_event_count = greatest(",
            "capacity_trimmed_through_seq = least(",
            "last_capacity_trimmed_at = case",
            "when im_realtime_checkpoints.last_capacity_trimmed_at is null then excluded.last_capacity_trimmed_at",
            "when excluded.last_capacity_trimmed_at is null then im_realtime_checkpoints.last_capacity_trimmed_at",
            "else greatest(",
            "constraint-safe capacity trim sequence must never exceed trimmed_through_seq",
            "null-safe monotonic capacity trim timestamp",
        ],
    );
    assert_contains_all(
        load,
        &[
            "pub const load_realtime_checkpoint_sql",
            "where tenant_id = $1 and organization_id = $2 and client_route_scope_key = $3",
        ],
    );
}

#[test]
fn test_postgres_realtime_event_window_sql_supports_range_reads_trim_and_clear() {
    let source = postgres_realtime_sql_source();
    let list = constant_source(&source, "list_realtime_client_route_events_sql");
    let upsert = constant_source(&source, "upsert_realtime_client_route_event_sql");
    let trim = constant_source(&source, "trim_realtime_client_route_events_sql");
    let clear = constant_source(&source, "clear_realtime_client_route_events_sql");
    let diagnostics = constant_source(&source, "load_realtime_event_window_diagnostics_sql");
    let high_risk = constant_source(&source, "list_realtime_event_window_high_risk_windows_sql");
    let orphaned_events =
        constant_source(&source, "list_orphaned_realtime_client_route_events_sql");

    assert_contains_all(
        list,
        &[
            "pub const list_realtime_client_route_events_sql",
            "from im_realtime_device_events",
            "delivery_class",
            "realtime_seq > $4",
            "order by realtime_seq asc",
            "limit $5",
        ],
    );
    assert_contains_all(
        upsert,
        &[
            "pub const upsert_realtime_client_route_event_sql",
            "delivery_class",
            "payload_json",
            "payload_hash",
            "on conflict (tenant_id, organization_id, client_route_scope_key, realtime_seq) do nothing",
        ],
    );
    assert_contains_all(
        trim,
        &[
            "pub const trim_realtime_client_route_events_sql",
            "delete from im_realtime_device_events",
            "realtime_seq <= $4",
        ],
    );
    assert_contains_all(
        clear,
        &[
            "pub const clear_realtime_client_route_events_sql",
            "delete from im_realtime_device_events",
            "where tenant_id = $1 and organization_id = $2 and client_route_scope_key = $3",
        ],
    );
    assert_contains_all(
        diagnostics,
        &[
            "pub const load_realtime_event_window_diagnostics_sql",
            "from im_realtime_checkpoints c",
            "left join im_realtime_device_events e",
            "count(distinct c.tenant_id || ':' || c.client_route_scope_key) as client_route_window_count",
            "count(e.realtime_seq) as pending_event_count",
            "coalesce(max(window_counts.pending_event_count), 0) as max_client_route_window_event_count",
            "coalesce(max(c.trimmed_through_seq), 0) as max_trimmed_through_seq",
            "coalesce(sum(c.capacity_trimmed_event_count), 0) as capacity_trimmed_event_count",
            "coalesce(max(c.capacity_trimmed_through_seq), 0) as max_capacity_trimmed_through_seq",
            "max(c.last_capacity_trimmed_at) as last_capacity_trimmed_at",
            "min(e.occurred_at) as oldest_pending_occurred_at",
        ],
    );
    assert_contains_all(
        high_risk,
        &[
            "pub const list_realtime_event_window_high_risk_windows_sql",
            "from im_realtime_checkpoints c",
            "join im_realtime_device_events e",
            "group by\n    c.tenant_id,\n    c.principal_kind,\n    c.principal_id,\n    c.device_id,\n    c.client_route_scope_key,\n    c.trimmed_through_seq,\n    c.capacity_trimmed_event_count,\n    c.capacity_trimmed_through_seq,\n    c.last_capacity_trimmed_at",
            "count(e.realtime_seq) as pending_event_count",
            "min(e.occurred_at) as oldest_pending_occurred_at",
            "order by pending_event_count desc, oldest_pending_occurred_at asc, c.tenant_id asc, c.principal_kind asc, c.principal_id asc, c.device_id asc",
            "limit 5",
        ],
    );
    assert_contains_all(
        orphaned_events,
        &[
            "pub const list_orphaned_realtime_client_route_events_sql",
            "from im_realtime_device_events e",
            "left join im_realtime_checkpoints c",
            "c.client_route_scope_key = e.client_route_scope_key",
            "where c.client_route_scope_key is null",
            "group by e.tenant_id, e.client_route_scope_key",
            "count(*) as orphaned_event_count",
            "min(e.realtime_seq) as min_realtime_seq",
            "max(e.realtime_seq) as max_realtime_seq",
            "min(e.occurred_at) as oldest_orphaned_occurred_at",
            "order by orphaned_event_count desc, oldest_orphaned_occurred_at asc, e.tenant_id asc, e.client_route_scope_key asc",
            "limit $1",
        ],
    );
}

#[test]
fn test_postgres_realtime_subscription_sql_has_fanout_index_and_clear_contracts() {
    let source = postgres_realtime_sql_source();
    let upsert = constant_source(&source, "upsert_realtime_subscription_sql");
    let replace_scopes = constant_source(&source, "replace_realtime_subscription_scopes_sql");
    let load_matching = constant_source(&source, "load_matching_realtime_subscriptions_sql");
    let clear_subscription = constant_source(&source, "clear_realtime_subscription_sql");
    let clear_scopes = constant_source(&source, "clear_realtime_subscription_scopes_sql");

    assert_contains_all(
        upsert,
        &[
            "pub const upsert_realtime_subscription_sql",
            "insert into im_realtime_subscriptions",
            "subscriptions_json",
            "subscription_count",
            "on conflict (tenant_id, organization_id, client_route_scope_key) do update",
            "where excluded.synced_at >= im_realtime_subscriptions.synced_at",
        ],
    );
    assert_contains_all(
        replace_scopes,
        &[
            "pub const replace_realtime_subscription_scopes_sql",
            "insert into im_realtime_subscription_scopes",
            "created_at",
            "updated_at",
            "select\n    $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12\nfrom im_realtime_subscriptions current_subscription",
            "current_subscription.tenant_id = $1",
            "current_subscription.organization_id = $2",
            "current_subscription.client_route_scope_key = $8",
            "current_subscription.synced_at = $10",
            "on conflict (\n    tenant_id,\n    organization_id,\n    principal_kind,\n    principal_id,\n    scope_type,\n    scope_id,\n    event_type,\n    client_route_scope_key\n) do update set",
            "where excluded.synced_at >= im_realtime_subscription_scopes.synced_at",
        ],
    );
    assert!(
        !replace_scopes.contains("subscribed_at"),
        "scope fanout index SQL must not reference subscribed_at because the schema stores it in subscriptions_json, not im_realtime_subscription_scopes"
    );
    assert_contains_all(
        load_matching,
        &[
            "pub const load_matching_realtime_subscriptions_sql",
            "from im_realtime_subscription_scopes",
            "where fs.tenant_id = $1",
            "fs.organization_id = $2",
            "fs.principal_kind = $3",
            "fs.principal_id = $4",
            "fs.scope_type = $5",
            "fs.scope_id = $6",
            "fs.event_type in ($7, '*')",
            "fs.device_id = any($8)",
        ],
    );
    assert_contains_all(
        clear_subscription,
        &[
            "pub const clear_realtime_subscription_sql",
            "delete from im_realtime_subscriptions",
        ],
    );
    assert_contains_all(
        clear_scopes,
        &[
            "pub const clear_realtime_subscription_scopes_sql",
            "delete from im_realtime_subscription_scopes",
            "where tenant_id = $1",
            "organization_id = $2",
            "client_route_scope_key = $3",
            "and synced_at <= $4",
        ],
    );
}

#[test]
fn test_postgres_realtime_subscription_sql_documents_transactional_replace_order() {
    let source = postgres_realtime_sql_source();

    assert_contains_all(
        &source,
        &[
            "save realtime subscription transaction order",
            "1. upsert_realtime_subscription_sql",
            "2. clear_realtime_subscription_scopes_sql",
            "3. replace_realtime_subscription_scopes_sql",
            "the clear step must use the attempted synced_at cutoff",
            "the replace step must join the current subscription row by the same synced_at",
        ],
    );
}

#[test]
fn test_postgres_realtime_transaction_plans_define_atomic_adapter_boundaries() {
    let source = postgres_realtime_sql_source();

    assert_plan_contains_steps(
        &source,
        "publish_realtime_events_transaction_plan",
        &[
            "begin transaction",
            "1. upsert_realtime_client_route_event_sql",
            "2. upsert_realtime_checkpoint_sql",
            "commit transaction",
            "rollback transaction on any error",
            "event window rows and checkpoint rows must never be committed separately",
        ],
    );
    assert_plan_contains_steps(
        &source,
        "ack_realtime_events_transaction_plan",
        &[
            "begin transaction",
            "1. trim_realtime_client_route_events_sql",
            "2. upsert_realtime_checkpoint_sql",
            "commit transaction",
            "rollback transaction on any error",
            "trimmed event rows and checkpoint rows must never be committed separately",
        ],
    );
    assert_plan_contains_steps(
        &source,
        "restore_realtime_client_route_state_transaction_plan",
        &[
            "begin transaction",
            "1. upsert_realtime_subscription_sql or clear_realtime_subscription_sql",
            "2. clear_realtime_subscription_scopes_sql when replacing subscriptions",
            "3. replace_realtime_subscription_scopes_sql for each derived scope row",
            "4. upsert_realtime_checkpoint_sql",
            "5. clear_realtime_client_route_events_sql",
            "6. upsert_realtime_client_route_event_sql for each restored event",
            "commit transaction",
            "rollback transaction on any error",
        ],
    );
    assert_plan_contains_steps(
        &source,
        "take_realtime_client_route_state_transaction_plan",
        &[
            "begin transaction",
            "1. load_realtime_checkpoint_sql",
            "2. list_realtime_client_route_events_sql",
            "3. load_realtime_subscription_sql",
            "4. clear_realtime_client_route_events_sql",
            "5. clear_realtime_subscription_scopes_sql",
            "6. clear_realtime_subscription_sql",
            "commit transaction",
            "rollback transaction on any error",
        ],
    );
    assert_plan_contains_steps(
        &source,
        "save_realtime_subscriptions_transaction_plan",
        &[
            "begin transaction",
            "1. upsert_realtime_subscription_sql",
            "2. clear_realtime_subscription_scopes_sql",
            "3. replace_realtime_subscription_scopes_sql",
            "commit transaction",
            "rollback transaction on any error",
        ],
    );
    assert_plan_contains_steps(
        &source,
        "disconnect_fence_transaction_plan",
        &[
            "single-statement compare-and-set operations",
            "upsert_realtime_disconnect_fence_sql",
            "clear_realtime_disconnect_fence_if_matches_sql",
            "clear_realtime_disconnect_fence_disconnected_at_or_before_sql",
        ],
    );
}

#[test]
fn test_postgres_realtime_adapter_plan_marks_store_adapter_implementation_boundary() {
    let adapter_plan = realtime_postgres_adapter_plan();

    assert_eq!(adapter_plan.runtime_status, "store_adapter_implemented");
    assert!(
        adapter_plan
            .runtime_status_reason
            .contains("individual store traits"),
        "PostgreSQL realtime adapter plan must distinguish implemented store traits from runtime-level transaction composition"
    );
    assert!(
        adapter_plan
            .runtime_status_reason
            .contains("runtime-level multi-store transactions"),
        "PostgreSQL realtime adapter plan must not overstate runtime transaction atomicity"
    );
}

#[test]
fn test_postgres_realtime_sql_placeholders_are_contiguous() {
    let source = postgres_realtime_sql_source();

    assert_uses_contiguous_placeholders(
        constant_source(&source, "load_realtime_checkpoint_sql"),
        3,
    );
    assert_uses_contiguous_placeholders(
        constant_source(&source, "upsert_realtime_checkpoint_sql"),
        14,
    );
    assert_uses_contiguous_placeholders(
        constant_source(&source, "upsert_realtime_client_route_event_sql"),
        16,
    );
    assert_uses_contiguous_placeholders(
        constant_source(&source, "list_realtime_client_route_events_sql"),
        5,
    );
    assert_uses_contiguous_placeholders(
        constant_source(&source, "trim_realtime_client_route_events_sql"),
        4,
    );
    assert_uses_contiguous_placeholders(
        constant_source(&source, "clear_realtime_client_route_events_sql"),
        3,
    );
    assert_uses_contiguous_placeholders(
        constant_source(&source, "load_realtime_event_window_diagnostics_sql"),
        0,
    );
    assert_uses_contiguous_placeholders(
        constant_source(&source, "list_realtime_event_window_high_risk_windows_sql"),
        0,
    );
    assert_uses_contiguous_placeholders(
        constant_source(&source, "list_orphaned_realtime_client_route_events_sql"),
        1,
    );
    assert_uses_contiguous_placeholders(
        constant_source(&source, "load_realtime_subscription_sql"),
        3,
    );
    assert_uses_contiguous_placeholders(
        constant_source(&source, "upsert_realtime_subscription_sql"),
        11,
    );
    assert_uses_contiguous_placeholders(
        constant_source(&source, "clear_realtime_subscription_sql"),
        3,
    );
    assert_uses_contiguous_placeholders(
        constant_source(
            &source,
            "clear_realtime_subscription_if_synced_at_or_before_sql",
        ),
        4,
    );
    assert_uses_contiguous_placeholders(
        constant_source(&source, "clear_realtime_subscription_scopes_sql"),
        4,
    );
    assert_uses_contiguous_placeholders(
        constant_source(&source, "replace_realtime_subscription_scopes_sql"),
        12,
    );
    assert_uses_contiguous_placeholders(
        constant_source(&source, "load_matching_realtime_subscriptions_sql"),
        8,
    );
    assert_uses_contiguous_placeholders(
        constant_source(&source, "load_realtime_disconnect_fence_sql"),
        5,
    );
    assert_uses_contiguous_placeholders(
        constant_source(&source, "upsert_realtime_disconnect_fence_sql"),
        13,
    );
    assert_uses_contiguous_placeholders(
        constant_source(&source, "clear_realtime_disconnect_fence_sql"),
        5,
    );
    assert_uses_contiguous_placeholders(
        constant_source(&source, "clear_realtime_disconnect_fence_if_matches_sql"),
        6,
    );
    assert_uses_contiguous_placeholders(
        constant_source(
            &source,
            "clear_realtime_disconnect_fence_disconnected_at_or_before_sql",
        ),
        6,
    );
}

#[test]
fn test_postgres_realtime_sql_specs_define_bindings_rows_and_complete_store_method_coverage() {
    let specs = realtime_postgres_sql_contract_specs();
    assert_eq!(specs.len(), 21);

    for spec in specs {
        assert!(!spec.name.trim().is_empty());
        assert!(!spec.sql.trim().is_empty());
        assert_sql_spec_bindings_match_placeholders(spec);
    }

    assert_eq!(
        binding_names(sql_spec(specs, "LOAD_REALTIME_CHECKPOINT_SQL")),
        vec!["tenant_id", "organization_id", "client_route_scope_key"]
    );
    assert_eq!(
        row_columns(sql_spec(specs, "LOAD_REALTIME_CHECKPOINT_SQL")),
        vec![
            "tenant_id",
            "organization_id",
            "principal_kind",
            "principal_id",
            "device_id",
            "latest_realtime_seq",
            "acked_through_seq",
            "trimmed_through_seq",
            "capacity_trimmed_event_count",
            "capacity_trimmed_through_seq",
            "last_capacity_trimmed_at",
            "updated_at",
        ]
    );
    assert_eq!(
        row_columns(sql_spec(specs, "LIST_REALTIME_CLIENT_ROUTE_EVENTS_SQL")),
        vec![
            "tenant_id",
            "organization_id",
            "principal_kind",
            "principal_id",
            "device_id",
            "realtime_seq",
            "scope_type",
            "scope_id",
            "event_type",
            "delivery_class",
            "payload_json",
            "occurred_at",
        ]
    );
    assert_eq!(
        binding_names(sql_spec(specs, "LOAD_REALTIME_SUBSCRIPTION_SQL")),
        vec!["tenant_id", "organization_id", "client_route_scope_key"]
    );
    assert_eq!(
        row_columns(sql_spec(specs, "LOAD_REALTIME_SUBSCRIPTION_SQL")),
        vec![
            "tenant_id",
            "organization_id",
            "principal_kind",
            "principal_id",
            "device_id",
            "subscriptions_json",
            "synced_at",
        ]
    );
    assert_eq!(
        binding_names(sql_spec(
            specs,
            "CLEAR_REALTIME_SUBSCRIPTION_IF_SYNCED_AT_OR_BEFORE_SQL"
        )),
        vec![
            "tenant_id",
            "organization_id",
            "client_route_scope_key",
            "cutoff_synced_at",
        ]
    );
    assert_eq!(
        binding_names(sql_spec(specs, "CLEAR_REALTIME_DISCONNECT_FENCE_SQL")),
        vec![
            "tenant_id",
            "organization_id",
            "principal_kind",
            "principal_id",
            "device_id",
        ]
    );

    let adapter_plan = realtime_postgres_adapter_plan();
    assert_eq!(adapter_plan.sql_contracts.len(), specs.len());
    assert_eq!(adapter_plan.method_plans.len(), 21);

    let required_methods = [
        "RealtimeCheckpointStore::load_checkpoint",
        "RealtimeCheckpointStore::save_checkpoints",
        "RealtimeEventWindowStore::load_window",
        "RealtimeEventWindowStore::save_windows",
        "RealtimeEventWindowStore::clear_window",
        "RealtimeEventWindowStore::diagnostics_snapshot",
        "RealtimeEventWindowStore::trim_window",
        "RealtimeSubscriptionStore::load_subscriptions",
        "RealtimeSubscriptionStore::load_matching_subscriptions",
        "RealtimeSubscriptionStore::save_subscriptions",
        "RealtimeSubscriptionStore::clear_subscriptions",
        "RealtimeSubscriptionStore::clear_subscriptions_synced_at_or_before",
        "RealtimeDisconnectFenceStore::load_fence",
        "RealtimeDisconnectFenceStore::save_fence",
        "RealtimeDisconnectFenceStore::clear_fence",
        "RealtimeDisconnectFenceStore::clear_fence_disconnected_at_or_before",
        "RealtimeDisconnectFenceStore::clear_fence_if_matches",
        "RealtimeDeliveryRuntime::restore_client_route_state",
        "RealtimeDeliveryRuntime::take_client_route_state",
        "RealtimeDeliveryRuntime::publish_scope_event",
        "RealtimeDeliveryRuntime::ack_events",
    ];

    for name in required_methods {
        let method = method_plan(adapter_plan, name);
        assert_method_references_known_sql(method, specs);
    }

    let publish = method_plan(adapter_plan, "RealtimeDeliveryRuntime::publish_scope_event");
    assert_eq!(
        publish.atomicity,
        RealtimePostgresMethodAtomicity::Transaction
    );
    assert_eq!(
        publish.transaction_plan_name,
        Some("PUBLISH_REALTIME_EVENTS_TRANSACTION_PLAN")
    );
    assert!(
        publish
            .notes
            .iter()
            .any(|note| note.contains("must never be committed separately")),
        "publish must document that event rows and checkpoints share one transaction"
    );
    assert_eq!(
        step_sql_names(publish),
        vec![
            "UPSERT_REALTIME_CLIENT_ROUTE_EVENT_SQL",
            "UPSERT_REALTIME_CHECKPOINT_SQL",
        ]
    );

    let ack = method_plan(adapter_plan, "RealtimeDeliveryRuntime::ack_events");
    assert_eq!(ack.atomicity, RealtimePostgresMethodAtomicity::Transaction);
    assert_eq!(
        ack.transaction_plan_name,
        Some("ACK_REALTIME_EVENTS_TRANSACTION_PLAN")
    );
    assert_eq!(
        step_sql_names(ack),
        vec![
            "TRIM_REALTIME_CLIENT_ROUTE_EVENTS_SQL",
            "UPSERT_REALTIME_CHECKPOINT_SQL",
        ]
    );

    let restore = method_plan(
        adapter_plan,
        "RealtimeDeliveryRuntime::restore_client_route_state",
    );
    assert_eq!(
        restore.atomicity,
        RealtimePostgresMethodAtomicity::Transaction
    );
    assert_eq!(
        restore.transaction_plan_name,
        Some("RESTORE_REALTIME_CLIENT_ROUTE_STATE_TRANSACTION_PLAN")
    );
    assert_eq!(
        step_sql_names(restore),
        vec![
            "UPSERT_REALTIME_SUBSCRIPTION_SQL",
            "CLEAR_REALTIME_SUBSCRIPTION_SCOPES_SQL",
            "REPLACE_REALTIME_SUBSCRIPTION_SCOPES_SQL",
            "UPSERT_REALTIME_CHECKPOINT_SQL",
            "CLEAR_REALTIME_CLIENT_ROUTE_EVENTS_SQL",
            "UPSERT_REALTIME_CLIENT_ROUTE_EVENT_SQL",
        ]
    );

    let take = method_plan(
        adapter_plan,
        "RealtimeDeliveryRuntime::take_client_route_state",
    );
    assert_eq!(take.atomicity, RealtimePostgresMethodAtomicity::Transaction);
    assert_eq!(
        take.transaction_plan_name,
        Some("TAKE_REALTIME_CLIENT_ROUTE_STATE_TRANSACTION_PLAN")
    );
    assert_eq!(
        step_sql_names(take),
        vec![
            "LOAD_REALTIME_CHECKPOINT_SQL",
            "LIST_REALTIME_CLIENT_ROUTE_EVENTS_SQL",
            "LOAD_REALTIME_SUBSCRIPTION_SQL",
            "CLEAR_REALTIME_CLIENT_ROUTE_EVENTS_SQL",
            "CLEAR_REALTIME_SUBSCRIPTION_SCOPES_SQL",
            "CLEAR_REALTIME_SUBSCRIPTION_SQL",
        ]
    );
}

#[test]
fn test_postgres_realtime_disconnect_fence_sql_uses_latest_and_cas_clear() {
    let source = postgres_realtime_sql_source();
    let upsert = constant_source(&source, "upsert_realtime_disconnect_fence_sql");
    let clear_if_matches =
        constant_source(&source, "clear_realtime_disconnect_fence_if_matches_sql");
    let clear_at_or_before = constant_source(
        &source,
        "clear_realtime_disconnect_fence_disconnected_at_or_before_sql",
    );
    let load = constant_source(&source, "load_realtime_disconnect_fence_sql");

    assert_contains_all(
        upsert,
        &[
            "pub const upsert_realtime_disconnect_fence_sql",
            "insert into im_realtime_disconnect_fences",
            "payload_json",
            "payload_hash",
            "on conflict (tenant_id, organization_id, principal_kind, principal_id, device_id) do update",
            "payload_json = excluded.payload_json",
            "payload_hash = excluded.payload_hash",
            "where excluded.disconnected_at > im_realtime_disconnect_fences.disconnected_at",
        ],
    );
    assert_contains_all(
        clear_if_matches,
        &[
            "pub const clear_realtime_disconnect_fence_if_matches_sql",
            "fence_token = $6",
        ],
    );
    assert_contains_all(
        clear_at_or_before,
        &[
            "pub const clear_realtime_disconnect_fence_disconnected_at_or_before_sql",
            "disconnected_at <= $6",
        ],
    );
    assert_contains_all(
        load,
        &[
            "pub const load_realtime_disconnect_fence_sql",
            "where tenant_id = $1",
            "organization_id = $2",
            "principal_kind = $3",
            "principal_id = $4",
            "device_id = $5",
        ],
    );
}

#[test]
fn test_postgres_realtime_checkpoint_upsert_binding_plan_matches_sql_contract_order() {
    let checkpoint = RealtimeCheckpointRecord {
        tenant_id: "100001".into(),
            organization_id: "0".into(),
            principal_kind: "user".into(),
        principal_id: "1".into(),
        device_id: "d_pad".into(),
        latest_realtime_seq: 42,
        acked_through_seq: 40,
        trimmed_through_seq: 39,
        capacity_trimmed_event_count: 7,
        capacity_trimmed_through_seq: 38,
        last_capacity_trimmed_at: Some("2026-05-01T10:00:00.000Z".into()),
        updated_at: "2026-05-01T10:00:02.000Z".into(),
    };

    let statement = realtime_postgres_bind_checkpoint_upsert(
        &checkpoint,
        "6:100001|4:user|1:1|5:d_pad",
        "2026-05-01T10:00:01.000Z",
    )
    .expect("checkpoint binding should succeed");

    assert_eq!(statement.sql_name, "UPSERT_REALTIME_CHECKPOINT_SQL");
    assert_eq!(
        bound_names(&statement),
        vec![
            "tenant_id",
            "organization_id",
            "client_route_scope_key",
            "principal_kind",
            "principal_id",
            "device_id",
            "latest_realtime_seq",
            "acked_through_seq",
            "trimmed_through_seq",
            "capacity_trimmed_event_count",
            "capacity_trimmed_through_seq",
            "last_capacity_trimmed_at",
            "created_at",
            "updated_at",
        ]
    );
    assert_eq!(
        bound_values(&statement),
        vec![
            RealtimePostgresBindingValue::Text("100001".into()),
            RealtimePostgresBindingValue::Text("default".into()),
            RealtimePostgresBindingValue::Text("6:100001|4:user|1:1|5:d_pad".into()),
            RealtimePostgresBindingValue::Text("user".into()),
            RealtimePostgresBindingValue::Text("1".into()),
            RealtimePostgresBindingValue::Text("d_pad".into()),
            RealtimePostgresBindingValue::BigInt(42),
            RealtimePostgresBindingValue::BigInt(40),
            RealtimePostgresBindingValue::BigInt(39),
            RealtimePostgresBindingValue::BigInt(7),
            RealtimePostgresBindingValue::BigInt(38),
            RealtimePostgresBindingValue::NullableTimestamptz(Some(
                "2026-05-01T10:00:00.000Z".into()
            )),
            RealtimePostgresBindingValue::Timestamptz("2026-05-01T10:00:01.000Z".into()),
            RealtimePostgresBindingValue::Timestamptz("2026-05-01T10:00:02.000Z".into()),
        ]
    );
}

#[test]
fn test_postgres_realtime_event_and_ack_binding_plans_match_sql_contract_order() {
    let event = RealtimeEvent {
        tenant_id: "100001".into(),
        principal_id: "1".into(),
        device_id: "d_pad".into(),
        realtime_seq: 42,
        scope_type: "conversation".into(),
        scope_id: "c_demo".into(),
        event_type: "message.posted".into(),
        delivery_class: "durable".into(),
        payload: r#"{"messageId":"m_demo"}"#.into(),
        occurred_at: "2026-05-01T10:00:00.000Z".into(),
    };

    let event_statement = realtime_postgres_bind_client_route_event_upsert(
        &event,
        "default",
        "user",
        "6:100001|4:user|1:1|5:d_pad",
        "sha256:demo",
        "2026-05-01T10:00:01.000Z",
        "2026-06-01T10:00:00.000Z",
    )
    .expect("event binding should succeed");

    assert_eq!(
        bound_names(&event_statement),
        vec![
            "tenant_id",
            "organization_id",
            "client_route_scope_key",
            "realtime_seq",
            "principal_kind",
            "principal_id",
            "device_id",
            "scope_type",
            "scope_id",
            "event_type",
            "delivery_class",
            "payload_json",
            "payload_hash",
            "occurred_at",
            "created_at",
            "retention_until",
        ]
    );
    assert_eq!(
        bound_values(&event_statement),
        vec![
            RealtimePostgresBindingValue::Text("100001".into()),
            RealtimePostgresBindingValue::Text("default".into()),
            RealtimePostgresBindingValue::Text("6:100001|4:user|1:1|5:d_pad".into()),
            RealtimePostgresBindingValue::BigInt(42),
            RealtimePostgresBindingValue::Text("user".into()),
            RealtimePostgresBindingValue::Text("1".into()),
            RealtimePostgresBindingValue::Text("d_pad".into()),
            RealtimePostgresBindingValue::Text("conversation".into()),
            RealtimePostgresBindingValue::Text("c_demo".into()),
            RealtimePostgresBindingValue::Text("message.posted".into()),
            RealtimePostgresBindingValue::Text("durable".into()),
            RealtimePostgresBindingValue::Json(r#"{"messageId":"m_demo"}"#.into()),
            RealtimePostgresBindingValue::Text("sha256:demo".into()),
            RealtimePostgresBindingValue::Timestamptz("2026-05-01T10:00:00.000Z".into()),
            RealtimePostgresBindingValue::Timestamptz("2026-05-01T10:00:01.000Z".into()),
            RealtimePostgresBindingValue::NullableTimestamptz(Some(
                "2026-06-01T10:00:00.000Z".into()
            )),
        ]
    );

    let trim_statement = realtime_postgres_bind_trim_client_route_events(
        "100001",
        "default",
        "6:100001|4:user|1:1|5:d_pad",
        40,
    )
    .expect("trim binding should succeed");

    assert_eq!(
        trim_statement.sql_name,
        "TRIM_REALTIME_CLIENT_ROUTE_EVENTS_SQL"
    );
    assert_eq!(
        bound_values(&trim_statement),
        vec![
            RealtimePostgresBindingValue::Text("100001".into()),
            RealtimePostgresBindingValue::Text("default".into()),
            RealtimePostgresBindingValue::Text("6:100001|4:user|1:1|5:d_pad".into()),
            RealtimePostgresBindingValue::BigInt(40),
        ]
    );
}

#[test]
fn test_postgres_realtime_subscription_binding_plan_serializes_items_and_fanout_rows() {
    let record = RealtimeSubscriptionRecord {
        tenant_id: "100001".into(),
            organization_id: "0".into(),
            principal_kind: "user".into(),
        principal_id: "1".into(),
        device_id: "d_pad".into(),
        synced_at: "2026-05-01T10:00:02.000Z".into(),
        items: vec![
            RealtimeSubscription {
                scope_type: "conversation".into(),
                scope_id: "c_demo".into(),
                event_types: vec!["message.posted".into(), "message.edited".into()],
                subscribed_at: "2026-05-01T10:00:00.000Z".into(),
            },
            RealtimeSubscription {
                scope_type: "project".into(),
                scope_id: "p_demo".into(),
                event_types: Vec::new(),
                subscribed_at: "2026-05-01T10:00:01.000Z".into(),
            },
        ],
    };

    let subscription_statement = realtime_postgres_bind_subscription_upsert(
        &record,
        "6:100001|4:user|1:1|5:d_pad",
        "2026-05-01T10:00:03.000Z",
    )
    .expect("subscription binding should succeed");

    assert_eq!(
        bound_names(&subscription_statement),
        vec![
            "tenant_id",
            "organization_id",
            "client_route_scope_key",
            "principal_kind",
            "principal_id",
            "device_id",
            "subscriptions_json",
            "subscription_count",
            "synced_at",
            "created_at",
            "updated_at",
        ]
    );
    assert_eq!(
        bound_values(&subscription_statement),
        vec![
            RealtimePostgresBindingValue::Text("100001".into()),
            RealtimePostgresBindingValue::Text("default".into()),
            RealtimePostgresBindingValue::Text("6:100001|4:user|1:1|5:d_pad".into()),
            RealtimePostgresBindingValue::Text("user".into()),
            RealtimePostgresBindingValue::Text("1".into()),
            RealtimePostgresBindingValue::Text("d_pad".into()),
            RealtimePostgresBindingValue::Json(
                r#"[{"scopeType":"conversation","scopeId":"c_demo","eventTypes":["message.posted","message.edited"],"subscribedAt":"2026-05-01T10:00:00.000Z"},{"scopeType":"project","scopeId":"p_demo","eventTypes":[],"subscribedAt":"2026-05-01T10:00:01.000Z"}]"#
                    .into()
            ),
            RealtimePostgresBindingValue::Integer(2),
            RealtimePostgresBindingValue::Timestamptz("2026-05-01T10:00:02.000Z".into()),
            RealtimePostgresBindingValue::Timestamptz("2026-05-01T10:00:03.000Z".into()),
            RealtimePostgresBindingValue::Timestamptz("2026-05-01T10:00:03.000Z".into()),
        ]
    );

    let clear_statement = realtime_postgres_bind_subscription_scope_clear(
        "100001",
        "default",
        "6:100001|4:user|1:1|5:d_pad",
        "2026-05-01T10:00:02.000Z",
    );
    assert_eq!(
        bound_values(&clear_statement),
        vec![
            RealtimePostgresBindingValue::Text("100001".into()),
            RealtimePostgresBindingValue::Text("default".into()),
            RealtimePostgresBindingValue::Text("6:100001|4:user|1:1|5:d_pad".into()),
            RealtimePostgresBindingValue::Timestamptz("2026-05-01T10:00:02.000Z".into()),
        ]
    );

    let fanout_statements = realtime_postgres_bind_subscription_scope_replacements(
        &record,
        "6:100001|4:user|1:1|5:d_pad",
        "2026-05-01T10:00:03.000Z",
    );

    assert_eq!(fanout_statements.len(), 3);
    assert_eq!(
        fanout_statements
            .iter()
            .map(bound_values)
            .collect::<Vec<_>>(),
        vec![
            vec![
                RealtimePostgresBindingValue::Text("100001".into()),
                RealtimePostgresBindingValue::Text("default".into()),
                RealtimePostgresBindingValue::Text("user".into()),
                RealtimePostgresBindingValue::Text("1".into()),
                RealtimePostgresBindingValue::Text("conversation".into()),
                RealtimePostgresBindingValue::Text("c_demo".into()),
                RealtimePostgresBindingValue::Text("message.posted".into()),
                RealtimePostgresBindingValue::Text("6:100001|4:user|1:1|5:d_pad".into()),
                RealtimePostgresBindingValue::Text("d_pad".into()),
                RealtimePostgresBindingValue::Timestamptz("2026-05-01T10:00:02.000Z".into()),
                RealtimePostgresBindingValue::Timestamptz("2026-05-01T10:00:03.000Z".into()),
                RealtimePostgresBindingValue::Timestamptz("2026-05-01T10:00:03.000Z".into()),
            ],
            vec![
                RealtimePostgresBindingValue::Text("100001".into()),
                RealtimePostgresBindingValue::Text("default".into()),
                RealtimePostgresBindingValue::Text("user".into()),
                RealtimePostgresBindingValue::Text("1".into()),
                RealtimePostgresBindingValue::Text("conversation".into()),
                RealtimePostgresBindingValue::Text("c_demo".into()),
                RealtimePostgresBindingValue::Text("message.edited".into()),
                RealtimePostgresBindingValue::Text("6:100001|4:user|1:1|5:d_pad".into()),
                RealtimePostgresBindingValue::Text("d_pad".into()),
                RealtimePostgresBindingValue::Timestamptz("2026-05-01T10:00:02.000Z".into()),
                RealtimePostgresBindingValue::Timestamptz("2026-05-01T10:00:03.000Z".into()),
                RealtimePostgresBindingValue::Timestamptz("2026-05-01T10:00:03.000Z".into()),
            ],
            vec![
                RealtimePostgresBindingValue::Text("100001".into()),
                RealtimePostgresBindingValue::Text("default".into()),
                RealtimePostgresBindingValue::Text("user".into()),
                RealtimePostgresBindingValue::Text("1".into()),
                RealtimePostgresBindingValue::Text("project".into()),
                RealtimePostgresBindingValue::Text("p_demo".into()),
                RealtimePostgresBindingValue::Text("*".into()),
                RealtimePostgresBindingValue::Text("6:100001|4:user|1:1|5:d_pad".into()),
                RealtimePostgresBindingValue::Text("d_pad".into()),
                RealtimePostgresBindingValue::Timestamptz("2026-05-01T10:00:02.000Z".into()),
                RealtimePostgresBindingValue::Timestamptz("2026-05-01T10:00:03.000Z".into()),
                RealtimePostgresBindingValue::Timestamptz("2026-05-01T10:00:03.000Z".into()),
            ],
        ]
    );
}

#[test]
fn test_postgres_realtime_binding_plan_rejects_values_postgres_cannot_store() {
    let checkpoint = RealtimeCheckpointRecord {
        tenant_id: "100001".into(),
            organization_id: "0".into(),
            principal_kind: "user".into(),
        principal_id: "1".into(),
        device_id: "d_pad".into(),
        latest_realtime_seq: i64::MAX as u64 + 1,
        acked_through_seq: 0,
        trimmed_through_seq: 0,
        capacity_trimmed_event_count: 0,
        capacity_trimmed_through_seq: 0,
        last_capacity_trimmed_at: None,
        updated_at: "2026-05-01T10:00:02.000Z".into(),
    };

    let error = realtime_postgres_bind_checkpoint_upsert(
        &checkpoint,
        "6:100001|4:user|1:1|5:d_pad",
        "2026-05-01T10:00:01.000Z",
    )
    .expect_err("u64 values beyond PostgreSQL bigint range must fail before execution");

    assert_eq!(error.code, "postgres_bigint_out_of_range");
    assert!(
        error.message.contains("latest_realtime_seq"),
        "error should name the rejected field: {}",
        error.message
    );

    let invalid_payload_event = RealtimeEvent {
        tenant_id: "100001".into(),
        principal_id: "1".into(),
        device_id: "d_pad".into(),
        realtime_seq: 1,
        scope_type: "conversation".into(),
        scope_id: "c_demo".into(),
        event_type: "message.posted".into(),
        delivery_class: "durable".into(),
        payload: "{not-json}".into(),
        occurred_at: "2026-05-01T10:00:00.000Z".into(),
    };

    let error = realtime_postgres_bind_client_route_event_upsert(
        &invalid_payload_event,
        "default",
        "user",
        "6:100001|4:user|1:1|5:d_pad",
        "sha256:demo",
        "2026-05-01T10:00:01.000Z",
        "2026-06-01T10:00:00.000Z",
    )
    .expect_err("invalid jsonb payload must fail before execution");

    assert_eq!(error.code, "invalid_jsonb_payload");
}

#[test]
fn test_postgres_realtime_publish_and_ack_transactions_preserve_atomic_statement_order() {
    let event = RealtimeEvent {
        tenant_id: "100001".into(),
        principal_id: "1".into(),
        device_id: "d_pad".into(),
        realtime_seq: 42,
        scope_type: "conversation".into(),
        scope_id: "c_demo".into(),
        event_type: "message.posted".into(),
        delivery_class: "durable".into(),
        payload: r#"{"messageId":"m_demo"}"#.into(),
        occurred_at: "2026-05-01T10:00:00.000Z".into(),
    };
    let checkpoint = RealtimeCheckpointRecord {
        tenant_id: "100001".into(),
            organization_id: "0".into(),
            principal_kind: "user".into(),
        principal_id: "1".into(),
        device_id: "d_pad".into(),
        latest_realtime_seq: 42,
        acked_through_seq: 40,
        trimmed_through_seq: 40,
        capacity_trimmed_event_count: 0,
        capacity_trimmed_through_seq: 0,
        last_capacity_trimmed_at: None,
        updated_at: "2026-05-01T10:00:02.000Z".into(),
    };

    let publish = realtime_postgres_bind_publish_transaction(
        vec![RealtimePostgresClientRouteEventMutation {
            event: event.clone(),
            organization_id: "0".into(),
            principal_kind: "user".into(),
            client_route_scope_key: "6:100001|4:user|1:1|5:d_pad".into(),
            payload_hash: "sha256:demo".into(),
            retention_until: Some("2026-06-01T10:00:00.000Z".into()),
        }],
        vec![RealtimePostgresCheckpointMutation {
            checkpoint: checkpoint.clone(),
            client_route_scope_key: "6:100001|4:user|1:1|5:d_pad".into(),
        }],
        "2026-05-01T10:00:01.000Z",
    )
    .expect("publish transaction binding should succeed");

    assert_eq!(
        publish.transaction_plan_name,
        "PUBLISH_REALTIME_EVENTS_TRANSACTION_PLAN"
    );
    assert_eq!(
        transaction_sql_names(&publish),
        vec![
            "UPSERT_REALTIME_CLIENT_ROUTE_EVENT_SQL",
            "UPSERT_REALTIME_CHECKPOINT_SQL",
        ]
    );

    let ack = realtime_postgres_bind_ack_transaction(
        "100001",
        "default",
        "6:100001|4:user|1:1|5:d_pad",
        40,
        &checkpoint,
        "2026-05-01T10:00:01.000Z",
    )
    .expect("ack transaction binding should succeed");

    assert_eq!(
        ack.transaction_plan_name,
        "ACK_REALTIME_EVENTS_TRANSACTION_PLAN"
    );
    assert_eq!(
        transaction_sql_names(&ack),
        vec![
            "TRIM_REALTIME_CLIENT_ROUTE_EVENTS_SQL",
            "UPSERT_REALTIME_CHECKPOINT_SQL",
        ]
    );
}

#[test]
fn test_postgres_realtime_subscription_save_transaction_preserves_replace_order() {
    let record = RealtimeSubscriptionRecord {
        tenant_id: "100001".into(),
            organization_id: "0".into(),
            principal_kind: "user".into(),
        principal_id: "1".into(),
        device_id: "d_pad".into(),
        synced_at: "2026-05-01T10:00:02.000Z".into(),
        items: vec![
            RealtimeSubscription {
                scope_type: "conversation".into(),
                scope_id: "c_demo".into(),
                event_types: vec!["message.posted".into(), "message.edited".into()],
                subscribed_at: "2026-05-01T10:00:00.000Z".into(),
            },
            RealtimeSubscription {
                scope_type: "project".into(),
                scope_id: "p_demo".into(),
                event_types: Vec::new(),
                subscribed_at: "2026-05-01T10:00:01.000Z".into(),
            },
        ],
    };

    let transaction = realtime_postgres_bind_save_subscription_transaction(
        &record,
        "6:100001|4:user|1:1|5:d_pad",
        "2026-05-01T10:00:03.000Z",
    )
    .expect("subscription save transaction binding should succeed");

    assert_eq!(
        transaction.transaction_plan_name,
        "SAVE_REALTIME_SUBSCRIPTIONS_TRANSACTION_PLAN"
    );
    assert_eq!(
        transaction_sql_names(&transaction),
        vec![
            "UPSERT_REALTIME_SUBSCRIPTION_SQL",
            "CLEAR_REALTIME_SUBSCRIPTION_SCOPES_SQL",
            "REPLACE_REALTIME_SUBSCRIPTION_SCOPES_SQL",
            "REPLACE_REALTIME_SUBSCRIPTION_SCOPES_SQL",
            "REPLACE_REALTIME_SUBSCRIPTION_SCOPES_SQL",
        ]
    );
}
