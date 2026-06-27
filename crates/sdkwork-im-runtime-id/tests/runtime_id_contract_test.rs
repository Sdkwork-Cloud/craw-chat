use sdkwork_id::{SnowflakeIdError, default_snowflake_epoch_millis, max_snowflake_node_id};
use sdkwork_im_runtime_id::{
    RuntimeIdConfig, RuntimeIdError, RuntimeIdStrategy, RuntimeSnowflakeIdGenerator,
    SDKWORK_IM_ID_NODE_ID_ENV, runtime_id_strategy,
};

#[test]
fn runtime_id_strategy_declares_database_spec_failure_policies() {
    assert_eq!(
        runtime_id_strategy(),
        RuntimeIdStrategy {
            id_type: "snowflake",
            clock_rollback: "reject_and_alert",
            node_conflict: "database_backed_auto_allocation",
            sequence_overflow: "fail_closed",
            restart_recovery: "idempotent_lease_reclaim",
            failure_handling: "database_first_then_env_fallback",
            public_id: "uuid_or_business_id",
        }
    );
}

#[test]
fn runtime_snowflake_generator_uses_native_i64_and_preserves_monotonic_order() {
    let generator = RuntimeSnowflakeIdGenerator::with_node_id(42)
        .expect("node id inside appbase snowflake range should be accepted");
    let now_millis = default_snowflake_epoch_millis() + 12_345;

    let first = generator
        .next_id_at(now_millis)
        .expect("first id should be generated");
    let second = generator
        .next_id_at(now_millis)
        .expect("second id should be generated");

    assert!(first > 0);
    assert!(second > first);
    assert_eq!(generator.node_id(), 42);
}

#[test]
fn runtime_snowflake_generator_rejects_clock_rollback_without_fallback() {
    let generator = RuntimeSnowflakeIdGenerator::with_node_id(7)
        .expect("node id inside appbase snowflake range should be accepted");
    let baseline = default_snowflake_epoch_millis() + 10;
    generator
        .next_id_at(baseline)
        .expect("baseline id should be generated");

    let error = generator
        .next_id_at(baseline - 1)
        .expect_err("clock rollback must reject instead of falling back");

    assert!(matches!(
        error,
        RuntimeIdError::Snowflake(SnowflakeIdError::ClockMovedBackwards { .. })
    ));
}

#[test]
fn runtime_id_config_requires_explicit_distributed_node_id() {
    let missing = RuntimeIdConfig::from_env_pairs(Vec::<(&str, &str)>::new());
    assert!(matches!(missing, Err(RuntimeIdError::MissingNodeId)));

    let invalid = RuntimeIdConfig::from_env_pairs([(SDKWORK_IM_ID_NODE_ID_ENV, "not-a-number")]);
    assert!(matches!(
        invalid,
        Err(RuntimeIdError::InvalidNodeIdConfig { .. })
    ));

    let out_of_range = RuntimeIdConfig::from_env_pairs([(SDKWORK_IM_ID_NODE_ID_ENV, "1024")]);
    assert!(matches!(
        out_of_range,
        Err(RuntimeIdError::Snowflake(SnowflakeIdError::InvalidNodeId {
            node_id,
            max_node_id,
        })) if node_id == max_snowflake_node_id() + 1 && max_node_id == max_snowflake_node_id()
    ));

    let config = RuntimeIdConfig::from_env_pairs([(SDKWORK_IM_ID_NODE_ID_ENV, "42")])
        .expect("valid node id should resolve");
    assert_eq!(config.node_id, 42);
}
