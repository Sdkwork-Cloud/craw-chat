//! Snowflake entity id generation for space-service handlers.

use std::sync::Arc;

use im_platform_contracts::IdGenerator;
use sdkwork_im_runtime_id::build_runtime_id_generator;
use sdkwork_routes_web_framework_backend_api::response::ApiProblem;

/// Snowflake ids are allocated through [`sdkwork_im_runtime_id::RuntimeSnowflakeIdGenerator`].

/// Build a runtime ID generator for space-service.
pub async fn build_runtime_id_generator_for_space() -> Arc<dyn IdGenerator> {
    build_runtime_id_generator("space-service").await
}

pub fn next_entity_id(id_generator: &Arc<dyn IdGenerator>) -> Result<i64, ApiProblem> {
    id_generator.next_id().map_err(|error| {
        tracing::error!(?error, "space-service snowflake id generation failed");
        ApiProblem::dependency_unavailable("space-service snowflake id generation failed")
    })
}
