//! Conversation sequence allocator contract.
//!
//! Eliminates the single-row hotspot in `im_conversation_seq_counters` by
//! batch-prefetching sequences from a fast atomic counter (Redis `INCRBY`).
//! Each node fetches a batch of N sequences and allocates locally until
//! exhausted, then fetches another batch.

use sdkwork_im_contract_core::ContractError;

/// Batch-prefetching conversation sequence allocator.
///
/// Instead of hitting the DB on every message, the allocator fetches a
/// batch of sequences from a distributed atomic counter. The local node
/// allocates from the batch without coordination until depleted.
pub trait ConversationSeqAllocator: Send + Sync {
    /// Allocate the next message sequence for a conversation.
    ///
    /// Returns a strictly increasing u64 that is unique within the
    /// conversation scope across all nodes.
    fn allocate_seq(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
    ) -> Result<u64, ContractError>;

    /// Batch size for prefetch. Nodes fetch this many sequences at once.
    fn batch_size(&self) -> u32;
}
