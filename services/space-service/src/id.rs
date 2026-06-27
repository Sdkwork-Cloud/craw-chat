//! Snowflake entity id generation for space-service handlers.

use std::sync::Arc;

use axum::http::StatusCode;
use im_platform_contracts::IdGenerator;
use sdkwork_im_runtime_id::RuntimeSnowflakeIdGenerator;

/// Build a runtime ID generator, preferring database-backed node_id allocation.
///
/// Falls back to `SDKWORK_IM_ID_NODE_ID` env var, then to node 0 for
/// dev/test environments without a database.
pub async fn build_runtime_id_generator() -> Arc<dyn IdGenerator> {
    match RuntimeSnowflakeIdGenerator::from_database_env("space-service").await {
        Ok(generator) => Arc::new(generator),
        Err(error) => {
            tracing::warn!(
                ?error,
                "database node_id allocation failed; falling back to env for space-service"
            );
            match RuntimeSnowflakeIdGenerator::from_env() {
                Ok(generator) => Arc::new(generator),
                Err(error) => {
                    tracing::warn!(
                        ?error,
                        "SDKWORK_IM_ID_NODE_ID missing; using snowflake node 0 for space-service bootstrap"
                    );
                    Arc::new(
                        RuntimeSnowflakeIdGenerator::with_node_id(0)
                            .expect("snowflake node 0 must initialize"),
                    )
                }
            }
        }
    }
}

pub fn next_entity_id(id_generator: &Arc<dyn IdGenerator>) -> Result<i64, StatusCode> {
    id_generator.next_id().map_err(|error| {
        tracing::error!(?error, "space-service snowflake id generation failed");
        StatusCode::SERVICE_UNAVAILABLE
    })
}
