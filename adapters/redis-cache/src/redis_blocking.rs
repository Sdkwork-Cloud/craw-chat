use std::time::Duration;

use crate::redis_unavailable;

const GATEWAY_RATE_LIMIT_REDIS_CONNECT_TIMEOUT_MS_ENV: &str =
    "SDKWORK_IM_GATEWAY_RATE_LIMIT_REDIS_CONNECT_TIMEOUT_MS";
const GATEWAY_RATE_LIMIT_REDIS_COMMAND_TIMEOUT_MS_ENV: &str =
    "SDKWORK_IM_GATEWAY_RATE_LIMIT_REDIS_COMMAND_TIMEOUT_MS";
const REDIS_CONNECT_TIMEOUT_MS_ENV: &str = "SDKWORK_IM_REDIS_CONNECT_TIMEOUT_MS";
const REDIS_COMMAND_TIMEOUT_MS_ENV: &str = "SDKWORK_IM_REDIS_COMMAND_TIMEOUT_MS";
const DEFAULT_REDIS_CONNECT_TIMEOUT_MS: u64 = 1_000;
const DEFAULT_REDIS_COMMAND_TIMEOUT_MS: u64 = 1_000;
const MIN_REDIS_TIMEOUT_MS: u64 = 10;
const MAX_REDIS_TIMEOUT_MS: u64 = 10_000;

/// Bounded timeout policy for synchronous Redis operations used on blocking pools.
#[derive(Clone, Copy, Debug)]
pub(crate) struct RedisBlockingTimeouts {
    connect_timeout: Duration,
    command_timeout: Duration,
}

impl RedisBlockingTimeouts {
    pub(crate) fn from_env() -> Self {
        Self {
            connect_timeout: resolve_timeout_from_env(
                REDIS_CONNECT_TIMEOUT_MS_ENV,
                DEFAULT_REDIS_CONNECT_TIMEOUT_MS,
            ),
            command_timeout: resolve_timeout_from_env(
                REDIS_COMMAND_TIMEOUT_MS_ENV,
                DEFAULT_REDIS_COMMAND_TIMEOUT_MS,
            ),
        }
    }

    pub(crate) fn gateway_rate_limit_from_env() -> Self {
        Self {
            connect_timeout: resolve_timeout_from_env_with_fallback(
                GATEWAY_RATE_LIMIT_REDIS_CONNECT_TIMEOUT_MS_ENV,
                REDIS_CONNECT_TIMEOUT_MS_ENV,
                DEFAULT_REDIS_CONNECT_TIMEOUT_MS,
            ),
            command_timeout: resolve_timeout_from_env_with_fallback(
                GATEWAY_RATE_LIMIT_REDIS_COMMAND_TIMEOUT_MS_ENV,
                REDIS_COMMAND_TIMEOUT_MS_ENV,
                DEFAULT_REDIS_COMMAND_TIMEOUT_MS,
            ),
        }
    }
}

fn resolve_timeout_from_env(key: &str, default_ms: u64) -> Duration {
    let millis = std::env::var(key)
        .ok()
        .and_then(|value| value.trim().parse::<u64>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(default_ms)
        .clamp(MIN_REDIS_TIMEOUT_MS, MAX_REDIS_TIMEOUT_MS);
    Duration::from_millis(millis)
}

fn resolve_timeout_from_env_with_fallback(
    primary_key: &str,
    fallback_key: &str,
    default_ms: u64,
) -> Duration {
    let millis = [primary_key, fallback_key]
        .into_iter()
        .find_map(|key| {
            std::env::var(key)
                .ok()
                .and_then(|value| value.trim().parse::<u64>().ok())
                .filter(|value| *value > 0)
        })
        .unwrap_or(default_ms)
        .clamp(MIN_REDIS_TIMEOUT_MS, MAX_REDIS_TIMEOUT_MS);
    Duration::from_millis(millis)
}

pub(crate) fn run_bounded_redis_command<T, F, Fut>(
    client: &redis::Client,
    timeouts: RedisBlockingTimeouts,
    operation: &str,
    command: F,
) -> Result<T, im_platform_contracts::ContractError>
where
    F: FnOnce(redis::aio::MultiplexedConnection) -> Fut,
    Fut: std::future::Future<Output = redis::RedisResult<T>>,
{
    let client = client.clone();
    let future = async move {
        let connection = client
            .get_multiplexed_tokio_connection_with_response_timeouts(
                timeouts.command_timeout,
                timeouts.connect_timeout,
            )
            .await?;
        command(connection).await
    };
    run_redis_future(future).map_err(|error| redis_unavailable(operation, error))
}

fn run_redis_future<T, Fut>(future: Fut) -> redis::RedisResult<T>
where
    Fut: std::future::Future<Output = redis::RedisResult<T>>,
{
    match tokio::runtime::Handle::try_current() {
        Ok(handle) => tokio::task::block_in_place(|| handle.block_on(future)),
        Err(_) => {
            let runtime = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .map_err(|error| {
                    redis::RedisError::from((
                        redis::ErrorKind::IoError,
                        "bounded redis runtime build failed",
                        error.to_string(),
                    ))
                })?;
            runtime.block_on(future)
        }
    }
}

pub(crate) fn blocking_subscription_connection(
    client: &redis::Client,
    timeouts: RedisBlockingTimeouts,
    operation: &str,
) -> Result<redis::Connection, im_platform_contracts::ContractError> {
    let connect_operation = format!("{operation}_connect");
    client
        .get_connection_with_timeout(timeouts.connect_timeout)
        .map_err(|error| redis_unavailable(connect_operation.as_str(), error))
}
