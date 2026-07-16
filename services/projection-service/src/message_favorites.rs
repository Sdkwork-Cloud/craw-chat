use std::cmp::Ordering;
use std::collections::{BTreeSet, HashMap};
use std::ops::Bound::{Excluded, Unbounded};

use im_time::utc_now_rfc3339_millis;
use sdkwork_utils_rust::SdkWorkPageData;

use super::model::{FavoriteMessageRequest, FavoriteMessagesListCursor, MessageFavoriteView};
use super::{TimelineProjectionService, lock_projection_mutex, scope::scope_key};

#[derive(Clone, Eq, PartialEq)]
pub(crate) struct MessageFavoriteIndexEntry {
    favorited_at: String,
    favorite_id: String,
}

pub(crate) struct MessageFavoritesWindowQuery<'a> {
    pub tenant_id: &'a str,
    pub organization_id: &'a str,
    pub principal_kind: &'a str,
    pub principal_id: &'a str,
    pub limit: usize,
    pub cursor: FavoriteMessagesListCursor,
    pub favorite_type: Option<&'a str>,
    pub search_query: Option<&'a str>,
}

impl Ord for MessageFavoriteIndexEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .favorited_at
            .cmp(&self.favorited_at)
            .then_with(|| other.favorite_id.cmp(&self.favorite_id))
    }
}

impl PartialOrd for MessageFavoriteIndexEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl MessageFavoriteIndexEntry {
    fn from_view(view: &MessageFavoriteView) -> Self {
        Self {
            favorited_at: view.favorited_at.clone(),
            favorite_id: view.favorite_id.clone(),
        }
    }
}

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
            .entry(key.clone())
            .or_default()
            .insert(favorite_id, view.clone());
        self.upsert_message_favorite_index(key.as_str(), &view);
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
        let removed = lock_projection_mutex(&self.message_favorites, "message favorites store")
            .get_mut(key.as_str())
            .is_some_and(|favorites| favorites.remove(favorite_id).is_some());
        if removed {
            self.remove_message_favorite_index_entry(key.as_str(), favorite_id);
        }
        removed
    }

    pub(crate) fn message_favorites_window_for_principal(
        &self,
        query: MessageFavoritesWindowQuery<'_>,
    ) -> Result<SdkWorkPageData<super::MessageFavoriteView>, crate::projection::ProjectionError>
    {
        let limit = query.limit;
        let mut items = self.collect_message_favorites_index_window(&query);
        let has_more = items.len() > limit;
        if has_more {
            items.truncate(limit);
        }
        let next_cursor = if has_more {
            items
                .last()
                .map(|favorite| {
                    let payload = serde_json::json!({
                        "favoritedAt": favorite.favorited_at,
                        "favoriteId": favorite.favorite_id,
                    });
                    crate::cursor_auth::encode_projection_list_cursor(&payload)
                })
                .transpose()?
        } else {
            None
        };
        Ok(super::list_page::cursor_page(
            items,
            limit,
            next_cursor,
            has_more,
        ))
    }

    fn collect_message_favorites_index_window(
        &self,
        query: &MessageFavoritesWindowQuery<'_>,
    ) -> Vec<MessageFavoriteView> {
        let key = message_favorites_scope_key(
            query.tenant_id,
            query.organization_id,
            query.principal_kind,
            query.principal_id,
        );
        let limit = query.limit.max(1);
        let mut window = Vec::with_capacity(limit.saturating_add(1));
        let favorites = lock_projection_mutex(&self.message_favorites, "message favorites store");
        let index = lock_projection_mutex(&self.message_favorites_index, "message favorites index");
        let Some(scope_index) = index.get(key.as_str()) else {
            return window;
        };
        let scope_favorites = favorites.get(key.as_str());
        let legacy_offset = matches!(&query.cursor, FavoriteMessagesListCursor::Offset(_));
        let offset = match &query.cursor {
            FavoriteMessagesListCursor::Offset(value) => *value,
            _ => 0,
        };
        let keyset_cursor = match &query.cursor {
            FavoriteMessagesListCursor::Keyset {
                favorited_at,
                favorite_id,
            } => Some((favorited_at.clone(), favorite_id.clone())),
            _ => None,
        };
        let index_iter: Box<dyn Iterator<Item = &MessageFavoriteIndexEntry>> =
            if let Some((favorited_at, favorite_id)) = keyset_cursor.as_ref() {
                let cursor_entry = MessageFavoriteIndexEntry {
                    favorited_at: favorited_at.clone(),
                    favorite_id: favorite_id.clone(),
                };
                Box::new(scope_index.range((Excluded(cursor_entry), Unbounded)))
            } else {
                Box::new(scope_index.iter())
            };
        let mut skipped = 0usize;
        for entry in index_iter {
            let Some(scope_favorites) = scope_favorites else {
                break;
            };
            let Some(favorite) = scope_favorites.get(entry.favorite_id.as_str()) else {
                continue;
            };
            if !favorite_matches_filters(favorite, query.favorite_type, query.search_query) {
                continue;
            }
            if legacy_offset && skipped < offset {
                skipped += 1;
                continue;
            }
            window.push(favorite.clone());
            if window.len() > limit {
                break;
            }
        }
        window
    }

    pub(crate) fn rebuild_message_favorites_index_for_scope(
        &self,
        scope_key: &str,
        favorites: &HashMap<String, MessageFavoriteView>,
    ) {
        let mut index = BTreeSet::new();
        for favorite in favorites.values() {
            index.insert(MessageFavoriteIndexEntry::from_view(favorite));
        }
        let mut store =
            lock_projection_mutex(&self.message_favorites_index, "message favorites index");
        if index.is_empty() {
            store.remove(scope_key);
        } else {
            store.insert(scope_key.to_owned(), index);
        }
    }

    fn upsert_message_favorite_index(&self, scope_key: &str, favorite: &MessageFavoriteView) {
        lock_projection_mutex(&self.message_favorites_index, "message favorites index")
            .entry(scope_key.to_owned())
            .or_default()
            .insert(MessageFavoriteIndexEntry::from_view(favorite));
    }

    fn remove_message_favorite_index_entry(&self, scope_key: &str, favorite_id: &str) {
        let mut store =
            lock_projection_mutex(&self.message_favorites_index, "message favorites index");
        if let Some(index) = store.get_mut(scope_key) {
            index.retain(|entry| entry.favorite_id != favorite_id);
            if index.is_empty() {
                store.remove(scope_key);
            }
        }
    }
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

fn favorite_matches_filters(
    favorite: &MessageFavoriteView,
    favorite_type: Option<&str>,
    query: Option<&str>,
) -> bool {
    favorite_type.is_none_or(|value| favorite.favorite_type == value)
        && query.is_none_or(|value| {
            let trimmed = value.trim();
            trimmed.is_empty() || favorite_matches_query(favorite, trimmed)
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_CURSOR_SECRET_ENV: &str = "SDKWORK_IM_PROJECTION_CURSOR_HS256_SECRET";

    struct TestEnvGuard {
        _lock: std::sync::MutexGuard<'static, ()>,
        name: &'static str,
        previous: Option<String>,
    }

    impl TestEnvGuard {
        fn set(name: &'static str, value: &str) -> Self {
            let lock = crate::lock_projection_test_environment();
            let previous = std::env::var(name).ok();
            unsafe {
                std::env::set_var(name, value);
            }
            Self {
                _lock: lock,
                name,
                previous,
            }
        }
    }

    impl Drop for TestEnvGuard {
        fn drop(&mut self) {
            unsafe {
                match &self.previous {
                    Some(value) => std::env::set_var(self.name, value),
                    None => std::env::remove_var(self.name),
                }
            }
        }
    }

    #[test]
    fn favorite_messages_indexed_window_paginates_without_full_collect() {
        let _cursor_secret = TestEnvGuard::set(
            TEST_CURSOR_SECRET_ENV,
            "projection-service-test-cursor-secret-32-bytes",
        );
        let service = super::TimelineProjectionService::default();
        service.create_message_favorite(
            "100001",
            "default",
            "user",
            "1",
            "m1",
            FavoriteMessageRequest {
                conversation_id: "c1".into(),
                favorite_type: "message".into(),
                title: String::new(),
                content_preview: String::new(),
                source_display_name: String::new(),
            },
        );
        service.create_message_favorite(
            "100001",
            "default",
            "user",
            "1",
            "m2",
            FavoriteMessageRequest {
                conversation_id: "c1".into(),
                favorite_type: "message".into(),
                title: String::new(),
                content_preview: String::new(),
                source_display_name: String::new(),
            },
        );
        service.create_message_favorite(
            "100001",
            "default",
            "user",
            "1",
            "m3",
            FavoriteMessageRequest {
                conversation_id: "c1".into(),
                favorite_type: "message".into(),
                title: String::new(),
                content_preview: String::new(),
                source_display_name: String::new(),
            },
        );

        let first_page = service
            .message_favorites_window_for_principal(MessageFavoritesWindowQuery {
                tenant_id: "100001",
                organization_id: "default",
                principal_kind: "user",
                principal_id: "1",
                limit: 2,
                cursor: FavoriteMessagesListCursor::Start,
                favorite_type: None,
                search_query: None,
            })
            .expect("first favorites page");
        assert_eq!(first_page.page_info.has_more, Some(true));
        assert_eq!(first_page.items.len(), 2);

        let cursor = FavoriteMessagesListCursor::Keyset {
            favorited_at: first_page.items[1].favorited_at.clone(),
            favorite_id: first_page.items[1].favorite_id.clone(),
        };
        let second_page = service
            .message_favorites_window_for_principal(MessageFavoritesWindowQuery {
                tenant_id: "100001",
                organization_id: "default",
                principal_kind: "user",
                principal_id: "1",
                limit: 2,
                cursor,
                favorite_type: None,
                search_query: None,
            })
            .expect("second favorites page");
        assert_ne!(second_page.page_info.has_more, Some(true));
        assert_eq!(second_page.items.len(), 1);
    }

    #[test]
    fn favorite_messages_filtered_index_window_avoids_full_principal_collect() {
        let service = TimelineProjectionService::default();
        for (message_id, favorite_type) in [("m1", "message"), ("m2", "link"), ("m3", "message")] {
            service.create_message_favorite(
                "100001",
                "default",
                "user",
                "1",
                message_id,
                FavoriteMessageRequest {
                    conversation_id: "c1".into(),
                    favorite_type: favorite_type.into(),
                    title: String::new(),
                    content_preview: String::new(),
                    source_display_name: String::new(),
                },
            );
        }

        let filtered = service
            .message_favorites_window_for_principal(MessageFavoritesWindowQuery {
                tenant_id: "100001",
                organization_id: "default",
                principal_kind: "user",
                principal_id: "1",
                limit: 10,
                cursor: FavoriteMessagesListCursor::Start,
                favorite_type: Some("message"),
                search_query: None,
            })
            .expect("filtered favorites page");
        assert_eq!(filtered.items.len(), 2);
        assert!(
            filtered
                .items
                .iter()
                .all(|favorite| favorite.favorite_type == "message")
        );
    }
}
