use im_time::utc_now_rfc3339_millis;

use super::model::MessageVisibilityMutationResult;
use super::{
    ConversationStateService, TimelineWindowView, lock_conversation_state_mutex,
    scope::{self, scope_key},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TimelineWindowForPrincipalQuery<'a> {
    pub tenant_id: &'a str,
    pub organization_id: &'a str,
    pub conversation_id: &'a str,
    pub principal_kind: &'a str,
    pub principal_id: &'a str,
    pub after_seq: Option<u64>,
    pub limit: usize,
}

/// Per-principal message visibility cache key.
fn message_visibility_scope_key(
    tenant_id: &str,
    organization_id: &str,
    principal_kind: &str,
    principal_id: &str,
) -> String {
    format!(
        "{}:{}:{}",
        scope_key(tenant_id, organization_id, "message-visibilities"),
        principal_kind,
        principal_id
    )
}

impl ConversationStateService {
    /// Resolve the per-principal visibility state for a message, returning the
    /// cached `MessageVisibilityMutationResult` if previously recorded.
    /// Returns `None` when the principal has not explicitly mutated visibility
    /// (defaults to visible: `is_deleted = false`).
    ///
    pub fn message_visibility_for_principal(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_kind: &str,
        principal_id: &str,
        message_id: &str,
    ) -> Option<MessageVisibilityMutationResult> {
        let key =
            message_visibility_scope_key(tenant_id, organization_id, principal_kind, principal_id);
        if let Some(result) =
            lock_conversation_state_mutex(&self.message_visibilities, "message visibility store")
                .get(key.as_str())
                .and_then(|messages| messages.get(message_id))
                .cloned()
        {
            return Some(result);
        }
        None
    }

    /// Resolve `message_seq` for a Conversation Message from the process cache.
    /// Returns 0 when the message is not currently cached (the
    /// OpenAPI schema declares `minimum: 0`, so 0 is a safe placeholder).
    pub(crate) fn message_seq_for_conversation_message(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
        message_id: &str,
    ) -> i32 {
        lock_conversation_state_mutex(&self.entries, "conversation_state store")
            .get(scope_key(tenant_id, organization_id, conversation_id).as_str())
            .and_then(|timeline| {
                timeline
                    .values()
                    .find(|entry| entry.message_id == message_id)
                    .map(|entry| entry.message_seq)
            })
            .map(|seq| seq.min(i32::MAX as u64) as i32)
            .unwrap_or(0)
    }

    /// Resolve `conversation_id` for a Message using the process cache index.
    /// Returns `None` when the Message is not currently cached.
    pub fn conversation_id_for_message(
        &self,
        tenant_id: &str,
        organization_id: &str,
        message_id: &str,
    ) -> Option<String> {
        lock_conversation_state_mutex(
            &self.message_conversation_index,
            "message conversation index",
        )
        .get(scope::message_lookup_scope_key(tenant_id, organization_id, message_id).as_str())
        .cloned()
    }

    /// Mark a message as soft-deleted (hidden) for the current principal.
    ///
    /// Idempotent: re-applying `delete` on an already-deleted record refreshes
    /// `updated_at` and returns the same `is_deleted = true` state. The caller
    /// MUST validate membership and message/conversation identifiers before
    /// invoking this method.
    pub fn delete_message_visibility(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_kind: &str,
        principal_id: &str,
        message_id: &str,
        conversation_id_hint: Option<&str>,
    ) -> MessageVisibilityMutationResult {
        let conversation_id = conversation_id_hint
            .map(str::to_owned)
            .or_else(|| self.conversation_id_for_message(tenant_id, organization_id, message_id))
            .unwrap_or_default();
        let message_seq = if !conversation_id.is_empty() {
            self.message_seq_for_conversation_message(
                tenant_id,
                organization_id,
                conversation_id.as_str(),
                message_id,
            )
        } else {
            0
        };
        let updated_at = utc_now_rfc3339_millis();
        let result = MessageVisibilityMutationResult {
            tenant_id: tenant_id.to_owned(),
            conversation_id: conversation_id.clone(),
            message_id: message_id.to_owned(),
            message_seq,
            principal_kind: principal_kind.to_owned(),
            principal_id: principal_id.to_owned(),
            is_deleted: true,
            updated_at: updated_at.clone(),
        };
        let key =
            message_visibility_scope_key(tenant_id, organization_id, principal_kind, principal_id);
        lock_conversation_state_mutex(&self.message_visibilities, "message visibility store")
            .entry(key)
            .or_default()
            .insert(message_id.to_owned(), result.clone());
        result
    }

    /// Timeline window filtered by per-principal soft-delete visibility.
    pub fn timeline_window_for_principal(
        &self,
        query: TimelineWindowForPrincipalQuery<'_>,
    ) -> Result<TimelineWindowView, crate::conversation_state::event_apply::ConversationStateError> {
        let limit = query.limit.max(1);
        let mut visible_after_seq = query.after_seq;
        let mut visible_items = Vec::new();
        const VISIBILITY_FILTER_BATCH_MULTIPLIER: usize = 4;
        let fetch_batch = (limit * VISIBILITY_FILTER_BATCH_MULTIPLIER).clamp(limit, 200);
        let mut trailing_has_more;

        loop {
            let batch = self.timeline_window(
                query.tenant_id,
                query.organization_id,
                query.conversation_id,
                visible_after_seq,
                fetch_batch,
            )?;
            trailing_has_more = batch.page_info.has_more == Some(true);

            for entry in batch.items {
                let hidden = self
                    .message_visibility_for_principal(
                        query.tenant_id,
                        query.organization_id,
                        query.principal_kind,
                        query.principal_id,
                        entry.message_id.as_str(),
                    )
                    .is_some_and(|visibility| visibility.is_deleted);
                if hidden {
                    continue;
                }
                visible_items.push(entry);
                if visible_items.len() > limit {
                    break;
                }
            }

            if visible_items.len() > limit || batch.page_info.has_more != Some(true) {
                break;
            }
            visible_after_seq = batch
                .page_info
                .next_cursor
                .as_deref()
                .and_then(|value| value.parse::<u64>().ok());
            if visible_after_seq.is_none() {
                break;
            }
        }

        let has_more = visible_items.len() > limit || trailing_has_more;
        if visible_items.len() > limit {
            visible_items.truncate(limit);
        }
        let next_after_seq = visible_items.last().map(|entry| entry.message_seq);

        Ok(crate::conversation_state::list_page::seq_cursor_page(
            visible_items,
            limit,
            next_after_seq,
            has_more,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_principal() -> (&'static str, &'static str, &'static str, &'static str) {
        ("100001", "default", "user", "u_1")
    }

    #[test]
    fn test_message_visibility_for_principal_returns_none_when_not_mutated() {
        let conversation_state = ConversationStateService::default();
        let (tenant, org, kind, id) = sample_principal();
        assert!(
            conversation_state
                .message_visibility_for_principal(tenant, org, kind, id, "m_1")
                .is_none()
        );
    }

    #[test]
    fn test_delete_message_visibility_marks_deleted_and_persists() {
        let conversation_state = ConversationStateService::default();
        let (tenant, org, kind, id) = sample_principal();

        let result =
            conversation_state.delete_message_visibility(tenant, org, kind, id, "m_1", Some("c_demo"));

        assert!(result.is_deleted);
        assert_eq!(result.tenant_id, tenant);
        assert_eq!(result.conversation_id, "c_demo");
        assert_eq!(result.message_id, "m_1");
        assert_eq!(result.principal_kind, kind);
        assert_eq!(result.principal_id, id);
        assert_eq!(result.message_seq, 0); // unprojected message defaults to 0

        let stored = conversation_state
            .message_visibility_for_principal(tenant, org, kind, id, "m_1")
            .expect("visibility state should be persisted");
        assert_eq!(stored, result);
    }

    #[test]
    fn test_delete_message_visibility_is_idempotent_and_refreshes_updated_at() {
        let conversation_state = ConversationStateService::default();
        let (tenant, org, kind, id) = sample_principal();

        let first =
            conversation_state.delete_message_visibility(tenant, org, kind, id, "m_1", Some("c_demo"));
        // Simulate time passage by ensuring updated_at differs on re-apply.
        std::thread::sleep(std::time::Duration::from_millis(2));
        let second =
            conversation_state.delete_message_visibility(tenant, org, kind, id, "m_1", Some("c_demo"));

        assert!(first.is_deleted);
        assert!(second.is_deleted);
        assert_ne!(
            first.updated_at, second.updated_at,
            "re-applying delete should refresh updated_at"
        );
    }

    #[test]
    fn test_message_visibility_scope_isolates_principals() {
        let conversation_state = ConversationStateService::default();
        let (tenant, org, kind, id) = sample_principal();

        let _ = conversation_state.delete_message_visibility(tenant, org, kind, id, "m_1", Some("c_demo"));

        // A different principal must not see the first principal's mutation.
        assert!(
            conversation_state
                .message_visibility_for_principal(tenant, org, kind, "u_2", "m_1")
                .is_none()
        );
    }

    #[test]
    fn test_conversation_id_for_message_returns_none_for_unprojected_message() {
        let conversation_state = ConversationStateService::default();
        let (tenant, org, _kind, _id) = sample_principal();
        assert!(
            conversation_state
                .conversation_id_for_message(tenant, org, "m_unknown")
                .is_none()
        );
    }
}
