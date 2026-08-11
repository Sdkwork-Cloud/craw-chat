//! Streaming service runtime bootstrap from process environment.

use std::sync::{Arc, OnceLock};

use im_adapters_postgres_journal::{PostgresJournalConfig, PostgresStreamStateStore};
use im_app_context::{
    allows_header_only_app_context_fallback, resolve_web_environment_from_process_env,
};
use im_platform_contracts::StreamStateStore;
use sdkwork_database_config::{DatabaseConfig, DatabaseEngine};
use sdkwork_web_core::WebEnvironment;
use tracing::info;

use crate::state::{AppState, RuntimeMemoryStreamStateStore, StreamingRuntime};

const IM_DATABASE_URL_ENV: &str = "SDKWORK_DATABASE_URL";

static DEFAULT_STREAMING_RUNTIME: OnceLock<Arc<StreamingRuntime>> = OnceLock::new();

pub fn default_streaming_runtime() -> Arc<StreamingRuntime> {
    if should_use_ephemeral_streaming_runtime() {
        return build_runtime_from_env().unwrap_or_else(|_| Arc::new(StreamingRuntime::default()));
    }

    DEFAULT_STREAMING_RUNTIME
        .get_or_init(build_streaming_runtime_or_fallback)
        .clone()
}

fn should_use_ephemeral_streaming_runtime() -> bool {
    matches!(
        resolve_web_environment_from_process_env(),
        WebEnvironment::Dev | WebEnvironment::Test
    )
}

fn build_streaming_runtime_or_fallback() -> Arc<StreamingRuntime> {
    match build_runtime_from_env() {
        Ok(runtime) => runtime,
        Err(error) if allows_header_only_app_context_fallback() => {
            tracing::warn!(
                error = %error,
                "streaming-service bootstrap unavailable; using in-memory stream runtime fallback (development/test only)"
            );
            Arc::new(StreamingRuntime::default())
        }
        Err(error) => {
            panic!(
                "streaming-service durable bootstrap failed in production-like environment: {error}"
            );
        }
    }
}

pub fn build_runtime_from_env() -> Result<Arc<StreamingRuntime>, String> {
    let state_store = resolve_stream_state_store_from_env()?;
    Ok(Arc::new(StreamingRuntime::with_store(state_store)))
}

pub fn default_app_state() -> AppState {
    AppState {
        runtime: default_streaming_runtime(),
    }
}

fn resolve_stream_state_store_from_env() -> Result<Arc<dyn StreamStateStore>, String> {
    if let Ok(config) = DatabaseConfig::from_env("IM")
        && config.engine == DatabaseEngine::Postgres {
            match PostgresJournalConfig::from_database_config(&config).connect_pool() {
                Ok(pool) => {
                    info!("streaming-service using postgres stream state store");
                    return Ok(Arc::new(PostgresStreamStateStore::from_pool(pool)));
                }
                Err(error) if allows_header_only_app_context_fallback() => {
                    tracing::warn!(
                        error = ?error,
                        "postgres stream state store bootstrap failed; using in-memory fallback (development/test only)"
                    );
                }
                Err(error) => {
                    return Err(format!(
                        "postgres stream state store bootstrap failed: {error:?}"
                    ));
                }
            }
        }

    if let Some(database_url) = resolve_im_database_url_from_env() {
        match PostgresJournalConfig::new(database_url).connect_pool() {
            Ok(pool) => {
                info!("streaming-service using postgres stream state store");
                return Ok(Arc::new(PostgresStreamStateStore::from_pool(pool)));
            }
            Err(error) if allows_header_only_app_context_fallback() => {
                tracing::warn!(
                    error = ?error,
                    "postgres stream state store bootstrap failed; using in-memory fallback (development/test only)"
                );
            }
            Err(error) => {
                return Err(format!(
                    "postgres stream state store bootstrap failed: {error:?}"
                ));
            }
        }
    }

    let environment = resolve_web_environment_from_process_env();
    if matches!(environment, WebEnvironment::Dev | WebEnvironment::Test)
        || allows_header_only_app_context_fallback()
    {
        info!("streaming-service using in-memory stream state store (development only)");
        return Ok(Arc::new(RuntimeMemoryStreamStateStore::default()));
    }

    Err(format!(
        "postgres stream state store is required in production: set {IM_DATABASE_URL_ENV}"
    ))
}

fn resolve_im_database_url_from_env() -> Option<String> {
    std::env::var(IM_DATABASE_URL_ENV)
        .ok()
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn production_requires_database_url_for_stream_state_store() {
        let database_url = std::env::var(IM_DATABASE_URL_ENV).ok();
        let im_env = std::env::var("SDKWORK_IM_ENVIRONMENT").ok();
        unsafe {
            std::env::remove_var(IM_DATABASE_URL_ENV);
            std::env::set_var("SDKWORK_IM_ENVIRONMENT", "prod");
        }

        let result = resolve_stream_state_store_from_env();
        assert!(result.is_err());

        unsafe {
            if let Some(value) = database_url {
                std::env::set_var(IM_DATABASE_URL_ENV, value);
            } else {
                std::env::remove_var(IM_DATABASE_URL_ENV);
            }
            if let Some(value) = im_env {
                std::env::set_var("SDKWORK_IM_ENVIRONMENT", value);
            } else {
                std::env::remove_var("SDKWORK_IM_ENVIRONMENT");
            }
        }
    }
}
