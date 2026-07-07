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

fn seq_key(tenant_id: &str, org_id: &str, conversation_id: &str) -> String {
    format!(
        "seq:{}",
        encode_redis_key_segments([tenant_id, org_id, conversation_id])
    )
}

/// Redis-backed conversation sequence allocator with local batch caching.
pub struct RedisSeqAllocator {
    client: redis::Client,
    batch_size: u32,
    timeouts: RedisBlockingTimeouts,
    /// Local batch cache: key -> (next_seq_in_batch, batch_upper_bound)
    batches: Mutex<HashMap<String, (u64, u64)>>,
}

impl RedisSeqAllocator {
    pub fn new(client: redis::Client) -> Self {
        Self {
            client,
            batch_size: DEFAULT_BATCH_SIZE,
            timeouts: RedisBlockingTimeouts::from_env(),
            batches: Mutex::new(HashMap::new()),
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
            if let Some((next_seq, upper_bound)) = batches.get_mut(&key)
                && *next_seq <= *upper_bound
            {
                let seq = *next_seq;
                *next_seq = seq.saturating_add(1);
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
        batches.insert(key, (next_seq, new_upper));

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
}
