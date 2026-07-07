//! Short-lived cache for the aggregate OpenAPI document.

use std::collections::BTreeMap;
use std::future::Future;
use std::sync::Arc;
use std::time::{Duration, Instant};

use axum::response::Response;
use serde_json::Value;
use tokio::sync::{Mutex, oneshot};

const DEFAULT_OPENAPI_AGGREGATE_CACHE_TTL: Duration = Duration::from_secs(60);
const OPENAPI_AGGREGATE_CACHE_TTL_ENV: &str = "SDKWORK_IM_GATEWAY_OPENAPI_CACHE_TTL_SECS";

#[derive(Clone)]
pub(crate) struct OpenApiAggregateCache {
    entries: Arc<Mutex<BTreeMap<String, OpenApiAggregateCacheEntry>>>,
    ttl: Duration,
}

enum OpenApiAggregateCacheEntry {
    Ready {
        document: Value,
        expires_at: Instant,
    },
    Refreshing {
        waiters: Vec<oneshot::Sender<()>>,
    },
}

enum OpenApiAggregateCacheLookup {
    Hit(Value),
    Wait(oneshot::Receiver<()>),
    Refresh,
}

impl OpenApiAggregateCache {
    pub(crate) fn from_env() -> Self {
        Self::new(resolve_openapi_aggregate_cache_ttl())
    }

    fn new(ttl: Duration) -> Self {
        Self {
            entries: Arc::new(Mutex::new(BTreeMap::new())),
            ttl,
        }
    }

    pub(crate) async fn get_or_refresh<F, Fut>(
        &self,
        cache_key: &str,
        load: F,
    ) -> Result<Value, Response>
    where
        F: FnOnce() -> Fut,
        Fut: Future<Output = Result<Value, Response>>,
    {
        let mut load = Some(load);
        loop {
            match self.begin_read_or_refresh(cache_key).await {
                OpenApiAggregateCacheLookup::Hit(document) => return Ok(document),
                OpenApiAggregateCacheLookup::Wait(wait) => {
                    let _ = wait.await;
                    continue;
                }
                OpenApiAggregateCacheLookup::Refresh => {}
            }

            let result = load
                .take()
                .expect("OpenAPI aggregate cache refresh should start exactly once")(
            )
            .await;
            let cached_document = result.as_ref().ok().cloned();
            self.finish_refresh(cache_key, cached_document).await;
            return result;
        }
    }

    async fn begin_read_or_refresh(&self, cache_key: &str) -> OpenApiAggregateCacheLookup {
        let now = Instant::now();
        let mut entries = self.entries.lock().await;
        if let Some(entry) = entries.get_mut(cache_key) {
            match entry {
                OpenApiAggregateCacheEntry::Ready {
                    document,
                    expires_at,
                } if *expires_at > now => {
                    return OpenApiAggregateCacheLookup::Hit(document.clone());
                }
                OpenApiAggregateCacheEntry::Ready { .. } => {
                    *entry = OpenApiAggregateCacheEntry::Refreshing {
                        waiters: Vec::new(),
                    };
                    return OpenApiAggregateCacheLookup::Refresh;
                }
                OpenApiAggregateCacheEntry::Refreshing { waiters } => {
                    let (sender, receiver) = oneshot::channel();
                    waiters.push(sender);
                    return OpenApiAggregateCacheLookup::Wait(receiver);
                }
            }
        }

        entries.insert(
            cache_key.to_owned(),
            OpenApiAggregateCacheEntry::Refreshing {
                waiters: Vec::new(),
            },
        );
        OpenApiAggregateCacheLookup::Refresh
    }

    async fn finish_refresh(&self, cache_key: &str, cached_document: Option<Value>) {
        let mut entries = self.entries.lock().await;
        let waiters = match entries.remove(cache_key) {
            Some(OpenApiAggregateCacheEntry::Refreshing { waiters }) => waiters,
            _ => Vec::new(),
        };

        if let Some(document) = cached_document {
            entries.insert(
                cache_key.to_owned(),
                OpenApiAggregateCacheEntry::Ready {
                    document,
                    expires_at: Instant::now() + self.ttl,
                },
            );
        }

        drop(entries);
        for waiter in waiters {
            let _ = waiter.send(());
        }
    }
}

fn resolve_openapi_aggregate_cache_ttl() -> Duration {
    std::env::var(OPENAPI_AGGREGATE_CACHE_TTL_ENV)
        .ok()
        .and_then(|value| value.parse::<u64>().ok())
        .filter(|seconds| *seconds > 0)
        .map(Duration::from_secs)
        .unwrap_or(DEFAULT_OPENAPI_AGGREGATE_CACHE_TTL)
}
