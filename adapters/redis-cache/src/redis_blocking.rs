use std::time::Duration;

use crate::redis_unavailable;
use redis::aio::{ConnectionManager, ConnectionManagerConfig};

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

    fn connection_manager_config(self) -> ConnectionManagerConfig {
        ConnectionManagerConfig::new()
            .set_connection_timeout(self.connect_timeout)
            .set_response_timeout(self.command_timeout)
    }

    pub(crate) fn connect_timeout(self) -> Duration {
        self.connect_timeout
    }

    pub(crate) fn command_timeout(self) -> Duration {
        self.command_timeout
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
    T: Send,
    F: FnOnce(redis::aio::MultiplexedConnection) -> Fut + Send,
    Fut: std::future::Future<Output = redis::RedisResult<T>> + Send,
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

pub(crate) async fn bounded_connection_manager(
    client: redis::Client,
    timeouts: RedisBlockingTimeouts,
) -> redis::RedisResult<ConnectionManager> {
    ConnectionManager::new_with_config(client, timeouts.connection_manager_config()).await
}

fn run_redis_future<T, Fut>(future: Fut) -> redis::RedisResult<T>
where
    T: Send,
    Fut: std::future::Future<Output = redis::RedisResult<T>> + Send,
{
    match tokio::runtime::Handle::try_current() {
        Ok(handle) if handle.runtime_flavor() == tokio::runtime::RuntimeFlavor::MultiThread => {
            tokio::task::block_in_place(|| handle.block_on(future))
        }
        Ok(_) => run_redis_future_on_dedicated_thread(future),
        Err(_) => build_standalone_redis_runtime()?.block_on(future),
    }
}

fn run_redis_future_on_dedicated_thread<T, Fut>(future: Fut) -> redis::RedisResult<T>
where
    T: Send,
    Fut: std::future::Future<Output = redis::RedisResult<T>> + Send,
{
    std::thread::scope(|scope| {
        scope
            .spawn(move || {
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
            })
            .join()
            .map_err(|_| {
                redis::RedisError::from((
                    redis::ErrorKind::IoError,
                    "bounded redis runtime worker panicked",
                ))
            })?
    })
}

fn build_standalone_redis_runtime() -> redis::RedisResult<tokio::runtime::Runtime> {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .map_err(|error| {
            redis::RedisError::from((
                redis::ErrorKind::IoError,
                "bounded redis runtime build failed",
                error.to_string(),
            ))
        })
}

#[cfg(test)]
mod tests {
    use super::run_redis_future;

    #[tokio::test(flavor = "current_thread")]
    async fn run_redis_future_does_not_panic_inside_current_thread_runtime() {
        let result = run_redis_future::<(), _>(async {
            Err(redis::RedisError::from((
                redis::ErrorKind::IoError,
                "expected test redis error",
            )))
        });

        assert!(result.is_err());
    }
}
