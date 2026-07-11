use std::sync::{Arc, OnceLock};

use im_platform_contracts::MetadataStore;

/// Durable metadata tier configuration for `TimelineProjectionService`.
///
/// Mirrors `TimelineTierConfig` but for metadata snapshots (conversation
/// summary, members, read cursors, interactions, profile, …). When the
/// in-memory projection misses, read paths consult the durable metadata
/// store via [`MetadataTierConfig::durable_metadata_store`] and write the
/// loaded snapshot back to memory so subsequent reads hit the hot cache.
///
/// The store is injected once during bootstrap (see
/// `TimelineProjectionService::configure_durable_metadata`) and remains
/// immutable for the lifetime of the service. A missing store (dev/test
/// in-memory backend) simply disables read-through fallback.
pub struct MetadataTierConfig {
    durable_metadata: OnceLock<Arc<dyn MetadataStore + Send + Sync>>,
}

impl Default for MetadataTierConfig {
    fn default() -> Self {
        Self {
            durable_metadata: OnceLock::new(),
        }
    }
}

impl MetadataTierConfig {
    pub fn configure_durable_metadata(&self, store: Arc<dyn MetadataStore + Send + Sync>) {
        let _ = self.durable_metadata.set(store);
    }

    pub fn durable_metadata_store(&self) -> Option<Arc<dyn MetadataStore + Send + Sync>> {
        self.durable_metadata.get().cloned()
    }
}
