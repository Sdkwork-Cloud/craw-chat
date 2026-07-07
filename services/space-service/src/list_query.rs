//! Shared list query types for space-service HTTP handlers.
//!
//! All space-service list endpoints use keyset pagination (`(sort_value, entity_id)`
//! cursor) instead of OFFSET, per `PAGINATION_SPEC.md` §2 (no unbounded collect then
//! skip/take in process memory).

use sdkwork_routes_web_framework_backend_api::response::ApiProblem;

pub use sdkwork_utils_rust::SdkWorkCursorListQuery as ListQuery;

/// Keyset page parameters for space-service list queries.
///
/// Uses `(sort_value, entity_id)` keyset cursor instead of OFFSET to satisfy
/// `PAGINATION_SPEC.md` §2. `sort_value` is typically `created_at` or `joined_at`,
/// while `entity_id` is the primary key of the listed entity.
#[derive(Clone, Debug)]
pub struct KeysetPageParams {
    pub page_size: usize,
    pub cursor_sort_value: Option<String>,
    pub cursor_entity: Option<String>,
}

impl KeysetPageParams {
    /// SQL LIMIT value: `page_size + 1` to detect `has_more` without a separate COUNT query.
    pub fn fetch_limit(&self) -> i64 {
        i64::try_from(self.page_size.saturating_add(1)).unwrap_or(i64::MAX)
    }
}

/// Resolve keyset page parameters from the standard `SdkWorkCursorListQuery`.
///
/// The cursor is expected to encode `sort_value|entity_id` for keyset pagination.
/// When no cursor is provided, the first page is returned.
pub fn resolve_keyset_page(query: &ListQuery) -> Result<KeysetPageParams, ApiProblem> {
    let page_size = query
        .page_size
        .map(|v| v.clamp(1, 200) as usize)
        .unwrap_or(20);

    let cursor_raw = query.cursor.as_deref();

    let (cursor_sort_value, cursor_entity) = match cursor_raw {
        Some(c) if !c.is_empty() => {
            let parts: Vec<&str> = c.splitn(2, '|').collect();
            if parts.len() != 2 {
                return Err(ApiProblem::bad_request(
                    "cursor must encode 'sort_value|entity_id'",
                ));
            }
            (Some(parts[0].to_string()), Some(parts[1].to_string()))
        }
        _ => (None, None),
    };

    Ok(KeysetPageParams {
        page_size,
        cursor_sort_value,
        cursor_entity,
    })
}
