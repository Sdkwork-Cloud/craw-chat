//! Bootstrap helpers for conversation-bound authorization in call signaling.

use std::sync::Arc;

use im_adapters_postgres_journal::{
    PostgresAggregateStore, PostgresJournalPool, conversation_member_access_gate_from_pool,
};
use im_platform_contracts::{ConversationAggregateStore, ConversationMemberAccessGate};
use sdkwork_im_database_pool::clone_shared_im_postgres_r2d2_pool;

/// Resolve a member access gate from the shared IM process PostgreSQL pool.
pub fn build_conversation_member_access_gate_from_env()
-> Option<Arc<dyn ConversationMemberAccessGate>> {
    let pool = clone_shared_im_postgres_r2d2_pool()?;
    Some(conversation_member_access_gate_from_pool(
        PostgresJournalPool::from_pool(pool),
    ))
}

/// Resolve a conversation aggregate store from the shared IM process PostgreSQL pool.
pub fn build_conversation_aggregate_store_from_env() -> Option<Arc<dyn ConversationAggregateStore>>
{
    let pool = clone_shared_im_postgres_r2d2_pool()?;
    Some(Arc::new(PostgresAggregateStore::from_pool(
        PostgresJournalPool::from_pool(pool),
    )))
}
