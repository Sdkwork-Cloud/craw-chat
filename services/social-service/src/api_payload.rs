//! SdkWorkApiResponse payload helpers (`API_SPEC.md` §14–16).

use sdkwork_utils_rust::{SdkWorkPageData, SdkWorkResourceData, cursor_list_page_data};

pub fn resource_item<T>(item: T) -> SdkWorkResourceData<T> {
    SdkWorkResourceData { item }
}

/// Keyset page response builder for SQL stores that fetch `LIMIT page_size + 1`.
///
/// Uses `(created_at, entity_id)` keyset cursor instead of OFFSET to satisfy
/// `PAGINATION_SPEC.md` §2.
///
/// The `cursor_extractor` closure maps each item to `(created_at, entity_id)`
/// used to build the `next_cursor` for the next page.
pub fn keyset_list_page<T, F>(
    records: Vec<T>,
    page_size: usize,
    cursor_extractor: F,
) -> SdkWorkPageData<T>
where
    F: Fn(&T) -> (String, i64),
{
    let has_more = records.len() > page_size;
    let mut items = records;
    if has_more {
        items.truncate(page_size);
    }
    let next_cursor = if has_more {
        items.last().map(|item| {
            let (created_at, entity_id) = cursor_extractor(item);
            format!("{created_at}|{entity_id}")
        })
    } else {
        None
    };
    cursor_list_page_data(items, page_size, next_cursor, has_more)
}

/// Full in-memory inventory dump (no continuation cursor).
pub fn full_inventory_page<T>(items: Vec<T>) -> SdkWorkPageData<T> {
    let len = items.len();
    cursor_list_page_data(items, len.max(1), None, false)
}
