use im_postgres_realtime_contracts::{
    ALL_REALTIME_POSTGRES_SQL_CONTRACTS, ALL_REALTIME_POSTGRES_TRANSACTION_PLANS,
    LIST_REALTIME_CLIENT_ROUTE_EVENTS_SQL, LOAD_MATCHING_REALTIME_SUBSCRIPTIONS_SQL,
    REALTIME_POSTGRES_ADAPTER_PLAN, REALTIME_POSTGRES_SQL_CONTRACT_SPECS,
    UPSERT_REALTIME_CHECKPOINT_SQL, UPSERT_REALTIME_CLIENT_ROUTE_EVENT_SQL,
};

const POSTGRES_CORE_SCHEMA: &str = include_str!(
    "../../../database/ddl/baseline/postgres/0001_im_baseline.sql"
);

#[test]
fn test_postgres_realtime_contract_crate_exports_complete_executable_contract_set() {
    assert_eq!(ALL_REALTIME_POSTGRES_SQL_CONTRACTS.len(), 21);
    assert_eq!(REALTIME_POSTGRES_SQL_CONTRACT_SPECS.len(), 21);
    assert_eq!(ALL_REALTIME_POSTGRES_TRANSACTION_PLANS.len(), 6);
    assert_eq!(REALTIME_POSTGRES_ADAPTER_PLAN.method_plans.len(), 21);
    assert_eq!(
        REALTIME_POSTGRES_ADAPTER_PLAN.runtime_status,
        "store_adapter_implemented"
    );
    assert!(
        REALTIME_POSTGRES_SQL_CONTRACT_SPECS
            .iter()
            .all(|spec| !spec.name.trim().is_empty() && !spec.sql.trim().is_empty()),
        "every PostgreSQL realtime SQL contract must have a name and SQL body"
    );
}

#[test]
fn test_postgres_realtime_contract_crate_keeps_runtime_safe_sql_shape() {
    let checkpoint_upsert = UPSERT_REALTIME_CHECKPOINT_SQL.to_lowercase();
    let event_upsert = UPSERT_REALTIME_CLIENT_ROUTE_EVENT_SQL.to_lowercase();
    let event_list = LIST_REALTIME_CLIENT_ROUTE_EVENTS_SQL.to_lowercase();
    let matching_subscriptions = LOAD_MATCHING_REALTIME_SUBSCRIPTIONS_SQL.to_lowercase();

    assert!(
        checkpoint_upsert.contains("on conflict (tenant_id, organization_id, client_route_scope_key) do update")
    );
    assert!(checkpoint_upsert.contains("latest_realtime_seq = greatest("));
    assert!(checkpoint_upsert.contains("capacity_trimmed_through_seq = least("));
    assert!(event_upsert.contains("$12::jsonb"));
    assert!(event_list.contains("payload_json::text as payload_json"));
    assert!(matching_subscriptions.contains("from im_realtime_subscription_scopes fs"));
    assert!(matching_subscriptions.contains("fs.event_type in ($7, '*')"));
}

#[test]
fn test_postgres_realtime_schema_enforces_event_window_checkpoint_parentage() {
    let schema = POSTGRES_CORE_SCHEMA.to_lowercase();

    assert!(
        schema.contains("create table if not exists im_realtime_device_events"),
        "schema must create the durable realtime event-window table"
    );
    assert!(
        schema.contains("create table if not exists im_realtime_checkpoints"),
        "schema must create the durable realtime checkpoint table"
    );
    assert!(
        schema.contains("foreign key (tenant_id, client_route_scope_key)")
            && schema
                .contains("references im_realtime_checkpoints (tenant_id, client_route_scope_key)")
            && schema.contains("on delete cascade"),
        "event-window rows must be tied to checkpoint rows so orphaned client route windows cannot survive checkpoint cleanup"
    );
    assert!(
        schema.contains("deferrable initially deferred"),
        "event-window checkpoint foreign key must be deferrable so checkpoint/event transactions can insert in either contract order"
    );
}
