use im_time::utc_now_rfc3339_millis;

use super::model::MessageVisibilityMutationResult;
use super::{TimelineProjectionService, TimelineWindowView, lock_projection_mutex, scope::{self, scope_key}};

/// Per-principal message visibility scope key for durable store adapters.
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

impl TimelineProjectionService {
    /// Resolve the per-principal visibility state for a message, returning the
    /// stored `MessageVisibilityMutationResult` snapshot if previously recorded.
    /// Returns `None` when the principal has not explicitly mutated visibility
    /// (defaults to visible: `is_deleted = false`).
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
        lock_projection_mutex(&self.message_visibilities, "message visibility store")
            .get(key.as_str())
            .and_then(|messages| messages.get(message_id))
            .cloned()
    }

    /// Resolve `message_seq` for a conversation message from the projection
    /// timeline. Returns 0 when the message is not currently projected (the
    /// OpenAPI schema declares `minimum: 0`, so 0 is a safe placeholder).
    pub(crate) fn message_seq_for_conversation_message(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
        message_id: &str,
    ) -> i32 {
        lock_projection_mutex(&self.entries, "projection store")
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

    /// Resolve `conversation_id` for a message using the projection index.
    /// Returns `None` when the message is not currently projected.
    pub fn conversation_id_for_message(
        &self,
        tenant_id: &str,
        organization_id: &str,
        message_id: &str,
    ) -> Option<String> {
        lock_projection_mutex(&self.message_conversation_index, "message conversation index")
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
        lock_projection_mutex(&self.message_visibilities, "message visibility store")
            .entry(key)
            .or_default()
            .insert(message_id.to_owned(), result.clone());
        result
    }

    /// Timeline window filtered by per-principal soft-delete visibility.
    pub fn timeline_window_for_principal(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
        principal_kind: &str,
        principal_id: &str,
        after_seq: Option<u64>,
        limit: usize,
    ) -> Result<TimelineWindowView, crate::projection::ProjectionError> {
        let limit = limit.max(1);
        let mut visible_after_seq = after_seq;
        let mut visible_items = Vec::new();
        const VISIBILITY_FILTER_BATCH_MULTIPLIER: usize = 4;
        let fetch_batch = (limit * VISIBILITY_FILTER_BATCH_MULTIPLIER).clamp(limit, 200);
        let mut trailing_has_more;

        loop {
            let batch = self.timeline_window(
                tenant_id,
                organization_id,
                conversation_id,
                visible_after_seq,
                fetch_batch,
            )?;
            trailing_has_more = batch.has_more;

            for entry in batch.items {
                let hidden = self
                    .message_visibility_for_principal(
                        tenant_id,
                        organization_id,
                        principal_kind,
                        principal_id,
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

            if visible_items.len() > limit || !batch.has_more {
                break;
            }
            visible_after_seq = batch.next_after_seq;
            if visible_after_seq.is_none() {
                break;
            }
        }

        let has_more = visible_items.len() > limit || trailing_has_more;
        if visible_items.len() > limit {
            visible_items.truncate(limit);
        }
        let next_after_seq = visible_items.last().map(|entry| entry.message_seq);

        Ok(TimelineWindowView {
            items: visible_items,
            next_after_seq,
            has_more,
        })
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
        let projection = TimelineProjectionService::default();
        let (tenant, org, kind, id) = sample_principal();
        assert!(
            projection
                .message_visibility_for_principal(tenant, org, kind, id, "m_1")
                .is_none()
        );
    }

    #[test]
    fn test_delete_message_visibility_marks_deleted_and_persists() {
        let projection = TimelineProjectionService::default();
        let (tenant, org, kind, id) = sample_principal();

        let result =
            projection.delete_message_visibility(tenant, org, kind, id, "m_1", Some("c_demo"));

        assert!(result.is_deleted);
        assert_eq!(result.tenant_id, tenant);
        assert_eq!(result.conversation_id, "c_demo");
        assert_eq!(result.message_id, "m_1");
        assert_eq!(result.principal_kind, kind);
        assert_eq!(result.principal_id, id);
        assert_eq!(result.message_seq, 0); // unprojected message defaults to 0

        let stored = projection
            .message_visibility_for_principal(tenant, org, kind, id, "m_1")
            .expect("visibility state should be persisted");
        assert_eq!(stored, result);
    }

    #[test]
    fn test_delete_message_visibility_is_idempotent_and_refreshes_updated_at() {
        let projection = TimelineProjectionService::default();
        let (tenant, org, kind, id) = sample_principal();

        let first =
            projection.delete_message_visibility(tenant, org, kind, id, "m_1", Some("c_demo"));
        // Simulate time passage by ensuring updated_at differs on re-apply.
        std::thread::sleep(std::time::Duration::from_millis(2));
        let second =
            projection.delete_message_visibility(tenant, org, kind, id, "m_1", Some("c_demo"));

        assert!(first.is_deleted);
        assert!(second.is_deleted);
        assert_ne!(
            first.updated_at, second.updated_at,
            "re-applying delete should refresh updated_at"
        );
    }

    #[test]
    fn test_message_visibility_scope_isolates_principals() {
        let projection = TimelineProjectionService::default();
        let (tenant, org, kind, id) = sample_principal();

        let _ = projection.delete_message_visibility(tenant, org, kind, id, "m_1", Some("c_demo"));

        // A different principal must not see the first principal's mutation.
        assert!(
            projection
                .message_visibility_for_principal(tenant, org, kind, "u_2", "m_1")
                .is_none()
        );
    }

    #[test]
    fn test_conversation_id_for_message_returns_none_for_unprojected_message() {
        let projection = TimelineProjectionService::default();
        let (tenant, org, _kind, _id) = sample_principal();
        assert!(
            projection
                .conversation_id_for_message(tenant, org, "m_unknown")
                .is_none()
        );
    }
}
