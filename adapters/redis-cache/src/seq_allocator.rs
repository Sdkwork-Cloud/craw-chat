//! Redis-backed [`ConversationSeqAllocator`] implementation.
//!
//! Eliminates the single-row hotspot in `im_conversation_seq_counters` by
//! batch-prefetching sequences via `INCRBY`. Each node fetches `batch_size`
//! sequences at once and allocates locally until exhausted.
//!
//! ## Key pattern
//! `seq:{length-prefixed tenant/org/conversation scope}` -> atomic counter (i64)

use std::collections::HashMap;
use std::sync::Mutex;

use sdkwork_im_contract_core::ContractError;

use crate::redis_blocking::{RedisBlockingTimeouts, run_bounded_redis_command};
use crate::redis_key::encode_redis_key_segments;

const DEFAULT_BATCH_SIZE: u32 = 1000;
/// Upper bound on locally cached sequence batches. Entries beyond this limit
/// are evicted in LRU order so a long-running node serving many conversations
/// cannot grow the cache without bound.
const MAX_CACHED_BATCHES: usize = 4096;

fn seq_key(tenant_id: &str, org_id: &str, conversation_id: &str) -> String {
    format!(
        "seq:{}",
        encode_redis_key_segments([tenant_id, org_id, conversation_id])
    )
}

/// One locally cached sequence batch.
struct SeqBatch {
    next_seq: u64,
    upper_bound: u64,
    /// Monotonic clock tick refreshed on every hit; drives LRU eviction.
    last_used: u64,
}

/// Bounded batch cache with LRU eviction.
#[derive(Default)]
struct BatchCache {
    batches: HashMap<String, SeqBatch>,
    clock: u64,
}

impl BatchCache {
    fn next_seq(&mut self, key: &str) -> Option<u64> {
        let batch = self.batches.get_mut(key)?;
        if batch.next_seq > batch.upper_bound {
            return None;
        }
        self.clock = self.clock.saturating_add(1);
        batch.last_used = self.clock;
        let seq = batch.next_seq;
        batch.next_seq = seq.saturating_add(1);
        Some(seq)
    }

    fn insert(&mut self, key: String, next_seq: u64, upper_bound: u64) {
        self.clock = self.clock.saturating_add(1);
        let is_new = !self.batches.contains_key(&key);
        self.batches.insert(
            key,
            SeqBatch {
                next_seq,
                upper_bound,
                last_used: self.clock,
            },
        );
        if is_new && self.batches.len() > MAX_CACHED_BATCHES {
            // Evict the least recently used entry.
            let victim_key = self
                .batches
                .iter()
                .min_by_key(|(_, batch)| batch.last_used)
                .map(|(key, _)| key.clone());
            if let Some(victim_key) = victim_key {
                self.batches.remove(&victim_key);
            }
        }
    }

    fn remove(&mut self, key: &str) {
        self.batches.remove(key);
    }
}

/// Redis-backed conversation sequence allocator with local batch caching.
pub struct RedisSeqAllocator {
    client: redis::Client,
    batch_size: u32,
    timeouts: RedisBlockingTimeouts,
    /// Local batch cache: key -> batch with LRU eviction.
    batches: Mutex<BatchCache>,
}

impl RedisSeqAllocator {
    pub fn new(client: redis::Client) -> Self {
        Self {
            client,
            batch_size: DEFAULT_BATCH_SIZE,
            timeouts: RedisBlockingTimeouts::from_env(),
            batches: Mutex::new(BatchCache::default()),
        }
    }

    pub fn with_batch_size(mut self, batch_size: u32) -> Self {
        self.batch_size = batch_size.max(1);
        self
    }
}

impl im_platform_contracts::ConversationSeqAllocator for RedisSeqAllocator {
    fn allocate_seq(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
    ) -> Result<u64, ContractError> {
        let key = seq_key(tenant_id, organization_id, conversation_id);

        // Fast path: serve from local batch cache under lock. The lock is
        // released before any blocking Redis IO so other conversations can
        // allocate concurrently.
        {
            let mut batches = self
                .batches
                .lock()
                .map_err(|_| ContractError::Unavailable("seq_allocator lock poisoned".into()))?;
            if let Some(seq) = batches.next_seq(&key) {
                return Ok(seq);
            }
        }

        // Slow path: fetch a new batch from Redis. No lock is held during the
        // blocking INCRBY call. Redis INCRBY is atomic, so concurrent fetches
        // for the same key receive disjoint sequence ranges; at worst one
        // extra batch is fetched, which is harmless.
        let batch_size_u64 = self.batch_size as u64;
        let redis_key = key.clone();
        let new_upper: i64 = run_bounded_redis_command(
            &self.client,
            self.timeouts,
            "seq_allocator_incrby",
            move |mut connection| async move {
                redis::cmd("INCRBY")
                    .arg(redis_key)
                    .arg(batch_size_u64)
                    .query_async(&mut connection)
                    .await
            },
        )?;
        let new_upper = new_upper as u64;
        let first_seq = new_upper.saturating_sub(batch_size_u64).saturating_add(1);

        if batch_size_u64 == 1 {
            let mut batches = self
                .batches
                .lock()
                .map_err(|_| ContractError::Unavailable("seq_allocator lock poisoned".into()))?;
            batches.remove(&key);
            return Ok(first_seq);
        }

        let next_seq = first_seq.saturating_add(1);
        let mut batches = self
            .batches
            .lock()
            .map_err(|_| ContractError::Unavailable("seq_allocator lock poisoned".into()))?;
        batches.insert(key, next_seq, new_upper);

        Ok(first_seq)
    }

    fn batch_size(&self) -> u32 {
        self.batch_size
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seq_key_is_segment_safe() {
        let k1 = seq_key("tenant:a", "default", "conversation");
        let k2 = seq_key("tenant", "a:default", "conversation");
        assert_ne!(k1, k2, "segment-safe sequence keys must not collide");
    }

    #[test]
    fn test_default_batch_size_is_reasonable() {
        const _: () = assert!(DEFAULT_BATCH_SIZE >= 100);
        const _: () = assert!(DEFAULT_BATCH_SIZE <= 10000);
        assert_eq!(DEFAULT_BATCH_SIZE, 1000);
    }

    #[test]
    fn test_batch_cache_evicts_least_recently_used_entry() {
        let mut cache = BatchCache::default();
        for index in 0..MAX_CACHED_BATCHES {
            cache.insert(format!("k{index}"), 1, 10);
        }
        // Touch k0 so it becomes the most recently used entry; k1 is now the
        // least recently used one.
        assert_eq!(cache.next_seq("k0"), Some(1));
        cache.insert("overflow".into(), 1, 10);
        assert_eq!(
            cache.batches.len(),
            MAX_CACHED_BATCHES,
            "cache must never exceed its capacity"
        );
        assert!(
            cache.next_seq("k1").is_none(),
            "least recently used entry must be evicted"
        );
        assert_eq!(
            cache.next_seq("k0"),
            Some(2),
            "recently used entry must survive eviction"
        );
        assert_eq!(
            cache.next_seq("overflow"),
            Some(1),
            "newest entry must survive eviction"
        );
    }

    #[test]
    fn test_batch_cache_exhausted_batch_is_replaced_not_duplicated() {
        let mut cache = BatchCache::default();
        cache.insert("k".into(), 11, 10);
        assert!(
            cache.next_seq("k").is_none(),
            "an exhausted batch must not serve sequences"
        );
        cache.insert("k".into(), 21, 30);
        assert_eq!(cache.next_seq("k"), Some(21));
        assert_eq!(
            cache.batches.len(),
            1,
            "replacing a batch must not grow the cache"
        );
    }

    #[test]
    fn test_batch_cache_remove_drops_entry() {
        let mut cache = BatchCache::default();
        cache.insert("k".into(), 1, 10);
        cache.remove("k");
        assert!(cache.next_seq("k").is_none());
        assert_eq!(cache.batches.len(), 0);
    }
}
