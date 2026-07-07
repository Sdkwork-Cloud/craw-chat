//! SdkWorkApiResponse payload helpers (`API_SPEC.md` §14–16).

use sdkwork_utils_rust::{SdkWorkPageData, SdkWorkResourceData, cursor_list_page_data};

pub fn resource_item<T>(item: T) -> SdkWorkResourceData<T> {
    SdkWorkResourceData { item }
}

/// Keyset page response builder for SQL stores that fetch `LIMIT page_size + 1`.
///
/// Uses `(sort_value, entity_value)` keyset cursor instead of OFFSET to satisfy
/// `PAGINATION_SPEC.md` §2.
///
/// The `cursor_extractor` closure maps each item to `(sort_value, entity_value)`
/// used to build the `next_cursor` for the next page.
pub fn keyset_list_page<T, F>(
    records: Vec<T>,
    page_size: usize,
    cursor_extractor: F,
) -> SdkWorkPageData<T>
where
    F: Fn(&T) -> (String, String),
{
    let has_more = records.len() > page_size;
    let mut items = records;
    if has_more {
        items.truncate(page_size);
    }
    let next_cursor = if has_more {
        items.last().map(|item| {
            let (sort_value, entity_value) = cursor_extractor(item);
            format!("{sort_value}|{entity_value}")
        })
    } else {
        None
    };
    cursor_list_page_data(items, page_size, next_cursor, has_more)
}
