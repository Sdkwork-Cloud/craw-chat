//! Search provider contract — pluggable full-text search backend.
//!
//! Follows the same provider-plugin pattern as PushProvider / RTC adapters.
//!
//! ## Built-in implementations
//! - **PostgreSQL**: Uses `search_vector tsvector` + GIN index (default)
//! - **Elasticsearch**: Optional, via `adapters/search-elasticsearch/`
//!
//! ## Switching backends
//! Set `ProviderDomain::Search` via ProviderRegistry. The service queries
//! through the trait without knowing which backend is active.

use sdkwork_im_contract_core::ContractError;
use serde::{Deserialize, Serialize};

/// A searchable message entry indexed by the search provider.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SearchableMessage {
    pub tenant_id: String,
    pub organization_id: String,
    pub conversation_id: String,
    pub message_id: i64,
    pub message_seq: u64,
    pub sender_principal_id: String,
    pub message_type: String,
    pub text_content: String,
    pub created_at: String,
}

/// Result of a search query.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SearchResult {
    /// Matching message IDs in relevance order.
    pub message_ids: Vec<i64>,
    /// Total number of matches (may exceed returned IDs).
    pub total_count: u64,
    /// Cursor for pagination.
    pub next_cursor: Option<String>,
}

/// Search provider trait — pluggable full-text search backend.
pub trait SearchProvider: Send + Sync {
    /// Index a message for future search queries.
    fn index_message(&self, message: &SearchableMessage) -> Result<(), ContractError>;

    /// Index multiple messages in batch.
    fn index_batch(&self, messages: &[SearchableMessage]) -> Result<(), ContractError> {
        for msg in messages {
            self.index_message(msg)?;
        }
        Ok(())
    }

    /// Search messages within a tenant scope.
    fn search(
        &self,
        tenant_id: &str,
        organization_id: &str,
        query: &str,
        conversation_id: Option<&str>,
        limit: usize,
        cursor: Option<&str>,
    ) -> Result<SearchResult, ContractError>;

    /// Remove a message from the search index (e.g. on recall/delete).
    fn remove_message(&self, tenant_id: &str, message_id: i64) -> Result<(), ContractError>;

    /// The provider plugin identifier.
    fn plugin_id(&self) -> &'static str;
}
