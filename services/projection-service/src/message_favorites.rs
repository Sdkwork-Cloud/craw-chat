use im_time::utc_now_rfc3339_millis;

use super::model::{FavoriteMessageRequest, MessageFavoriteView};
use super::{lock_projection_mutex, scope::scope_key, TimelineProjectionService};

fn message_favorites_scope_key(
    tenant_id: &str,
    organization_id: &str,
    principal_kind: &str,
    principal_id: &str,
) -> String {
    format!(
        "{}:{}:{}",
        scope_key(tenant_id, organization_id, "message-favorites"),
        principal_kind,
        principal_id
    )
}

fn favorite_id_for_message(principal_id: &str, message_id: &str) -> String {
    format!("fav_{principal_id}_{message_id}")
}

impl TimelineProjectionService {
    pub fn message_favorites_for_principal(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_kind: &str,
        principal_id: &str,
    ) -> Vec<MessageFavoriteView> {
        let key = message_favorites_scope_key(
            tenant_id,
            organization_id,
            principal_kind,
            principal_id,
        );
        let mut favorites = lock_projection_mutex(&self.message_favorites, "message favorites store")
            .get(key.as_str())
            .cloned()
            .unwrap_or_default()
            .into_values()
            .collect::<Vec<_>>();
        favorites.sort_by(|left, right| right.favorited_at.cmp(&left.favorited_at));
        favorites
    }

    pub fn create_message_favorite(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_kind: &str,
        principal_id: &str,
        message_id: &str,
        request: FavoriteMessageRequest,
    ) -> MessageFavoriteView {
        let favorite_id = favorite_id_for_message(principal_id, message_id);
        let message_seq = self
            .message_seq_for_conversation_message(
                tenant_id,
                organization_id,
                request.conversation_id.as_str(),
                message_id,
            )
            .unwrap_or(0) as i32;
        let view = MessageFavoriteView {
            tenant_id: tenant_id.to_owned(),
            principal_kind: principal_kind.to_owned(),
            principal_id: principal_id.to_owned(),
            favorite_id: favorite_id.clone(),
            favorite_type: request.favorite_type,
            conversation_id: request.conversation_id,
            message_id: message_id.to_owned(),
            message_seq,
            title: request.title,
            content_preview: request.content_preview,
            source_display_name: request.source_display_name,
            favorited_at: utc_now_rfc3339_millis(),
        };
        let key = message_favorites_scope_key(
            tenant_id,
            organization_id,
            principal_kind,
            principal_id,
        );
        lock_projection_mutex(&self.message_favorites, "message favorites store")
            .entry(key)
            .or_default()
            .insert(favorite_id, view.clone());
        view
    }

    pub fn delete_message_favorite(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_kind: &str,
        principal_id: &str,
        favorite_id: &str,
    ) -> bool {
        let key = message_favorites_scope_key(
            tenant_id,
            organization_id,
            principal_kind,
            principal_id,
        );
        lock_projection_mutex(&self.message_favorites, "message favorites store")
            .get_mut(key.as_str())
            .is_some_and(|favorites| favorites.remove(favorite_id).is_some())
    }

    fn message_seq_for_conversation_message(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
        message_id: &str,
    ) -> Option<u64> {
        lock_projection_mutex(&self.entries, "projection store")
            .get(scope_key(tenant_id, organization_id, conversation_id).as_str())?
            .values()
            .find(|entry| entry.message_id == message_id)
            .map(|entry| entry.message_seq)
    }
}

pub fn filter_message_favorites(
    favorites: Vec<MessageFavoriteView>,
    favorite_type: Option<&str>,
    query: Option<&str>,
) -> Vec<MessageFavoriteView> {
    favorites
        .into_iter()
        .filter(|favorite| {
            favorite_type.is_none_or(|value| favorite.favorite_type == value)
                && query.is_none_or(|value| favorite_matches_query(favorite, value))
        })
        .collect()
}

fn favorite_matches_query(favorite: &MessageFavoriteView, query: &str) -> bool {
    let needle = query.trim().to_ascii_lowercase();
    if needle.is_empty() {
        return true;
    }
    [favorite.title.as_str(), favorite.content_preview.as_str(), favorite.source_display_name.as_str()]
        .into_iter()
        .any(|value| value.to_ascii_lowercase().contains(needle.as_str()))
}
