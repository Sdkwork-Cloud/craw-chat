use std::collections::HashMap;

use im_domain_core::conversation::{
    ConversationMember, ConversationReadCursor, read_cursor_storage_key,
};

use crate::model::ConversationCatalogEntry;
use crate::{ConversationSummaryView, TimelineProjectionService, lock_projection_mutex, snapshot};

impl TimelineProjectionService {
    /// Read-through fallback: load conversation summary from the durable
    /// metadata store and write it back to the in-memory summary cache so
    /// subsequent reads hit memory. Returns `None` when the durable store is
    /// not configured, the snapshot is absent, or the load fails (warn-logged).
    pub(crate) fn load_summary_from_durable_store(
        &self,
        scope: &str,
    ) -> Option<ConversationSummaryView> {
        let store = self.durable_metadata_store()?;
        match snapshot::load_metadata_snapshot::<ConversationSummaryView>(
            store.as_ref(),
            scope,
            snapshot::CONVERSATION_SUMMARY_KEY,
        ) {
            Ok(Some(summary)) => {
                lock_projection_mutex(&self.summaries, "summary store")
                    .insert(scope.to_owned(), summary.clone());
                Some(summary)
            }
            Ok(None) => None,
            Err(error) => {
                tracing::warn!(
                    target: "sdkwork.im.projection.read_through",
                    event = "im.projection.summary_durable_read_failed",
                    scope = %scope,
                    error = %error,
                    "durable metadata read-through failed for conversation summary",
                );
                None
            }
        }
    }

    /// Read-through fallback: load conversation catalog entry (conversation
    /// type, history visibility) from the durable metadata store and hydrate
    /// the in-memory conversations cache. Returns `None` when the durable
    /// store is not configured, the snapshot is absent, or the load fails
    /// (warn-logged).
    pub(crate) fn load_conversation_catalog_from_durable_store(
        &self,
        scope: &str,
    ) -> Option<ConversationCatalogEntry> {
        let store = self.durable_metadata_store()?;
        match snapshot::load_metadata_snapshot::<ConversationCatalogEntry>(
            store.as_ref(),
            scope,
            snapshot::CONVERSATION_CATALOG_KEY,
        ) {
            Ok(Some(entry)) => {
                lock_projection_mutex(&self.conversations, "conversation store")
                    .insert(scope.to_owned(), entry.clone());
                Some(entry)
            }
            Ok(None) => None,
            Err(error) => {
                tracing::warn!(
                    target: "sdkwork.im.projection.read_through",
                    event = "im.projection.conversation_catalog_durable_read_failed",
                    scope = %scope,
                    error = %error,
                    "durable metadata read-through failed for conversation catalog",
                );
                None
            }
        }
    }

    /// Read-through fallback: load the full conversation read cursors
    /// snapshot from the durable metadata store, hydrate the in-memory cursor
    /// store, then return the hydrated map. One DB load serves all cursor
    /// lookups for the conversation. Returns `None` when the durable store is
    /// not configured, the snapshot is absent, or the load fails (warn-logged).
    pub(crate) fn load_read_cursors_from_durable_store(
        &self,
        scope: &str,
    ) -> Option<HashMap<String, ConversationReadCursor>> {
        let store = self.durable_metadata_store()?;
        let cursors = match snapshot::load_metadata_snapshot::<Vec<ConversationReadCursor>>(
            store.as_ref(),
            scope,
            snapshot::CONVERSATION_READ_CURSORS_KEY,
        ) {
            Ok(Some(cursors)) => cursors,
            Ok(None) => return None,
            Err(error) => {
                tracing::warn!(
                    target: "sdkwork.im.projection.read_through",
                    event = "im.projection.read_cursors_durable_read_failed",
                    scope = %scope,
                    error = %error,
                    "durable metadata read-through failed for conversation read cursors",
                );
                return None;
            }
        };
        let hydrated: HashMap<String, ConversationReadCursor> = cursors
            .into_iter()
            .map(|cursor| {
                let storage_key =
                    read_cursor_storage_key(cursor.member_id.as_str(), cursor.device_id.as_deref());
                (storage_key, cursor)
            })
            .collect();
        lock_projection_mutex(&self.read_cursors, "cursor store")
            .insert(scope.to_owned(), hydrated.clone());
        Some(hydrated)
    }

    /// Read-through fallback: load the full conversation members snapshot from
    /// the durable metadata store, hydrate the in-memory member store, then
    /// re-query. This avoids per-member DB round-trips: one load populates the
    /// cache for all members of the conversation. Returns `None` when the
    /// durable store is not configured, the snapshot is absent, or the load
    /// fails (warn-logged).
    pub(crate) fn load_member_from_durable_store(
        &self,
        scope: &str,
        principal_id: &str,
        principal_kind: &str,
    ) -> Option<ConversationMember> {
        let store = self.durable_metadata_store()?;
        let members = match snapshot::load_metadata_snapshot::<Vec<ConversationMember>>(
            store.as_ref(),
            scope,
            snapshot::CONVERSATION_MEMBERS_KEY,
        ) {
            Ok(Some(members)) => members,
            Ok(None) => return None,
            Err(error) => {
                tracing::warn!(
                    target: "sdkwork.im.projection.read_through",
                    event = "im.projection.members_durable_read_failed",
                    scope = %scope,
                    error = %error,
                    "durable metadata read-through failed for conversation members",
                );
                return None;
            }
        };
        {
            let mut member_store = lock_projection_mutex(&self.members, "member store");
            member_store.remove_conversation(scope);
            for member in &members {
                member_store.insert_member(scope.to_owned(), member.clone());
            }
        }
        lock_projection_mutex(&self.members, "member store")
            .member_for_principal_kind(scope, principal_id, principal_kind)
            .cloned()
    }
}
