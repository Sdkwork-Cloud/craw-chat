//! Shared list query types for social-service Postgres HTTP handlers.

use im_adapters_social_postgres::wire_id::parse_social_entity_id;
use sdkwork_routes_web_framework_backend_api::response::ApiProblem;

pub use sdkwork_utils_rust::SdkWorkCursorListQuery as ListQuery;

/// Keyset page parameters for social-service list queries.
///
/// Uses `(created_at DESC, entity_id DESC)` keyset cursor instead of OFFSET
/// to satisfy `PAGINATION_SPEC.md` §2 (no unbounded collect then skip/take).
#[derive(Clone, Debug)]
pub struct KeysetPageParams {
    pub page_size: usize,
    pub cursor_created_at: Option<String>,
    pub cursor_block_id: Option<i64>,
    pub cursor_direct_chat_id: Option<i64>,
}

impl KeysetPageParams {
    /// SQL LIMIT value: `page_size + 1` to detect `has_more` without a separate COUNT query.
    pub fn fetch_limit(&self) -> i64 {
        i64::try_from(self.page_size.saturating_add(1)).unwrap_or(i64::MAX)
    }
}

/// Resolve keyset page parameters from the standard `SdkWorkCursorListQuery`.
///
/// The cursor is expected to encode `created_at` and `entity_id` for keyset pagination.
/// When no cursor is provided, the first page is returned.
pub fn resolve_keyset_page(query: &ListQuery) -> Result<KeysetPageParams, ApiProblem> {
    let page_size = query
        .page_size
        .map(|v| v.clamp(1, 200) as usize)
        .unwrap_or(20);

    let cursor_raw = query.cursor.as_deref();

    let (cursor_created_at, cursor_entity_id) = match cursor_raw {
        Some(c) if !c.is_empty() => {
            let parts: Vec<&str> = c.splitn(2, '|').collect();
            if parts.len() != 2 {
                return Err(ApiProblem::bad_request(
                    "cursor must encode 'created_at|entity_id'",
                ));
            }
            (Some(parts[0].to_string()), Some(parts[1].to_string()))
        }
        _ => (None, None),
    };

    let cursor_entity_id = cursor_entity_id
        .as_deref()
        .map(parse_social_entity_id)
        .transpose()
        .map_err(|_| {
            ApiProblem::bad_request(
                "cursor entity_id must be a canonical positive signed int64 string",
            )
        })?;

    Ok(KeysetPageParams {
        page_size,
        cursor_created_at,
        cursor_block_id: cursor_entity_id,
        cursor_direct_chat_id: cursor_entity_id,
    })
}

#[cfg(test)]
mod tests {
    use super::{ListQuery, resolve_keyset_page};

    #[test]
    fn rejects_non_canonical_cursor_entity_ids() {
        for entity_id in ["0", "01", "-1", "block_1", "9223372036854775808"] {
            let query = ListQuery {
                cursor: Some(format!("2026-07-24T00:00:00Z|{entity_id}")),
                ..Default::default()
            };
            assert!(
                resolve_keyset_page(&query).is_err(),
                "cursor entity id {entity_id:?} must be rejected"
            );
        }
    }

    #[test]
    fn preserves_canonical_cursor_entity_id() {
        let query = ListQuery {
            cursor: Some("2026-07-24T00:00:00Z|330339707122622464".to_owned()),
            ..Default::default()
        };

        let paging = resolve_keyset_page(&query).expect("canonical cursor");

        assert_eq!(paging.cursor_block_id, Some(330339707122622464));
        assert_eq!(paging.cursor_direct_chat_id, Some(330339707122622464));
    }
}
