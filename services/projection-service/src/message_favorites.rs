use im_time::utc_now_rfc3339_millis;
use im_time::rfc3339_cmp;

use super::model::{FavoriteMessageRequest, FavoriteMessagesListCursor, MessageFavoriteView};
use super::{TimelineProjectionService, lock_projection_mutex, scope::scope_key};

pub(super) fn message_favorites_scope_key(
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
        let key =
            message_favorites_scope_key(tenant_id, organization_id, principal_kind, principal_id);
        let mut favorites =
            lock_projection_mutex(&self.message_favorites, "message favorites store")
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
        let message_seq = self.message_seq_for_conversation_message(
            tenant_id,
            organization_id,
            request.conversation_id.as_str(),
            message_id,
        );
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
        let key =
            message_favorites_scope_key(tenant_id, organization_id, principal_kind, principal_id);
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
        let key =
            message_favorites_scope_key(tenant_id, organization_id, principal_kind, principal_id);
        lock_projection_mutex(&self.message_favorites, "message favorites store")
            .get_mut(key.as_str())
            .is_some_and(|favorites| favorites.remove(favorite_id).is_some())
    }

    pub(crate) fn message_favorites_window_for_principal(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_kind: &str,
        principal_id: &str,
        limit: usize,
        cursor: FavoriteMessagesListCursor,
        favorite_type: Option<&str>,
        query: Option<&str>,
    ) -> super::FavoriteMessagesWindowView {
        let items = filter_message_favorites(
            self.message_favorites_for_principal(
                tenant_id,
                organization_id,
                principal_kind,
                principal_id,
            ),
            favorite_type,
            query,
        );
        let list_cursor = match cursor {
            FavoriteMessagesListCursor::Start => None,
            other => Some(other),
        };
        let (items, has_more) =
            favorite_messages_window_slice(items, list_cursor, limit);
        let next_cursor = if has_more {
            items.last().and_then(|favorite| {
                let payload = serde_json::json!({
                    "favoritedAt": favorite.favorited_at,
                    "favoriteId": favorite.favorite_id,
                });
                crate::cursor_auth::encode_signed_projection_cursor(&payload).ok()
            })
        } else {
            None
        };
        super::FavoriteMessagesWindowView {
            items,
            next_cursor,
            has_more,
        }
    }
}

pub(super) fn favorite_messages_window_slice(
    items: Vec<MessageFavoriteView>,
    cursor: Option<FavoriteMessagesListCursor>,
    limit: usize,
) -> (Vec<MessageFavoriteView>, bool) {
    let mut window = Vec::with_capacity(limit.saturating_add(1));
    let legacy_offset = matches!(cursor, Some(FavoriteMessagesListCursor::Offset(_)));
    let offset = match cursor {
        Some(FavoriteMessagesListCursor::Offset(value)) => value,
        _ => 0,
    };
    let keyset_cursor = match cursor {
        Some(FavoriteMessagesListCursor::Keyset {
            favorited_at,
            favorite_id,
        }) => Some((favorited_at, favorite_id)),
        _ => None,
    };
    let mut skipped = 0usize;
    for favorite in items {
        if let Some((favorited_at, favorite_id)) = keyset_cursor.as_ref()
            && !favorite_after_keyset_cursor(&favorite, favorited_at, favorite_id)
        {
            continue;
        }
        if legacy_offset && skipped < offset {
            skipped += 1;
            continue;
        }
        window.push(favorite);
        if window.len() > limit {
            break;
        }
    }
    let has_more = window.len() > limit;
    if has_more {
        window.truncate(limit);
    }
    (window, has_more)
}

fn favorite_after_keyset_cursor(
    favorite: &MessageFavoriteView,
    favorited_at: &str,
    favorite_id: &str,
) -> bool {
    use std::cmp::Ordering;

    match rfc3339_cmp(favorite.favorited_at.as_str(), favorited_at) {
        Ordering::Less => true,
        Ordering::Greater => false,
        Ordering::Equal => favorite.favorite_id.as_str() > favorite_id,
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
    [
        favorite.title.as_str(),
        favorite.content_preview.as_str(),
        favorite.source_display_name.as_str(),
    ]
    .into_iter()
    .any(|value| value.to_ascii_lowercase().contains(needle.as_str()))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn favorite(favorite_id: &str, favorited_at: &str) -> MessageFavoriteView {
        MessageFavoriteView {
            tenant_id: "100001".into(),
            principal_kind: "user".into(),
            principal_id: "1".into(),
            favorite_id: favorite_id.into(),
            favorite_type: "message".into(),
            conversation_id: "c1".into(),
            message_id: format!("m_{favorite_id}"),
            message_seq: 1,
            title: "title".into(),
            content_preview: "preview".into(),
            source_display_name: "source".into(),
            favorited_at: favorited_at.into(),
        }
    }

    #[test]
    fn favorite_messages_keyset_window_paginates_without_offset_scan() {
        let items = vec![
            favorite("f3", "2026-05-06T00:00:00.200Z"),
            favorite("f2", "2026-05-06T00:00:00.100Z"),
            favorite("f1", "2026-05-06T00:00:00.000Z"),
        ];
        let (first_page, has_more) = favorite_messages_window_slice(items.clone(), None, 2);
        assert!(has_more);
        assert_eq!(first_page.len(), 2);
        assert_eq!(first_page[0].favorite_id, "f3");

        let cursor = Some(FavoriteMessagesListCursor::Keyset {
            favorited_at: first_page[1].favorited_at.clone(),
            favorite_id: first_page[1].favorite_id.clone(),
        });
        let (second_page, has_more) = favorite_messages_window_slice(items, cursor, 2);
        assert!(!has_more);
        assert_eq!(second_page.len(), 1);
        assert_eq!(second_page[0].favorite_id, "f1");
    }
}
