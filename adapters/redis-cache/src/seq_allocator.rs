//! Redis-backed [`ConversationSeqAllocator`] implementation.
//!
//! Eliminates the single-row hotspot in `im_conversation_seq_counters` by
//! batch-prefetching sequences via `INCRBY`. Each node fetches `batch_size`
//! sequences at once and allocates locally until exhausted.
//!
//! ## Key pattern
//! `seq:{tenant_id}:{org_id}:{conversation_id}` → atomic counter (i64)

use std::collections::HashMap;
use std::sync::Mutex;

use redis::Commands;
use sdkwork_im_contract_core::ContractError;

use crate::redis_unavailable;

const DEFAULT_BATCH_SIZE: u32 = 1000;

fn seq_key(tenant_id: &str, org_id: &str, conversation_id: &str) -> String {
    format!("seq:{tenant_id}:{org_id}:{conversation_id}")
}

/// Redis-backed conversation sequence allocator with local batch caching.
pub struct RedisSeqAllocator {
    client: redis::Client,
    batch_size: u32,
    /// Local batch cache: key → (next_seq_in_batch, batch_upper_bound)
    batches: Mutex<HashMap<String, (u64, u64)>>,
}

impl RedisSeqAllocator {
    pub fn new(client: redis::Client) -> Self {
        Self {
            client,
            batch_size: DEFAULT_BATCH_SIZE,
            batches: Mutex::new(HashMap::new()),
        }
    }

    pub fn with_batch_size(mut self, batch_size: u32) -> Self {
        self.batch_size = batch_size.max(1);
        self
    }

    fn connection(&self) -> Result<redis::Connection, ContractError> {
        self.client
            .get_connection()
            .map_err(|e| redis_unavailable("seq_allocator_connect", e))
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
        let mut conn = self.connection()?;
        let new_upper: i64 = conn
            .incr(&key, batch_size_u64)
            .map_err(|e| redis_unavailable("seq_allocator_incrby", e))?;
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

        // Store the remaining batch locally
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
        let k1 = seq_key("t1", "org1", "c1");
        let k2 = seq_key("t2", "org1", "c1");
        assert_ne!(k1, k2, "different tenant should produce different key");
    }

    #[test]
    fn test_default_batch_size_is_reasonable() {
        const _: () = assert!(DEFAULT_BATCH_SIZE >= 100);
        const _: () = assert!(DEFAULT_BATCH_SIZE <= 10000);
        assert_eq!(DEFAULT_BATCH_SIZE, 1000);
    }
}
