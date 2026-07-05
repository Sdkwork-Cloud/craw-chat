//! SdkWorkApiResponse payload helpers (`API_SPEC.md` §14–16).

use sdkwork_utils_rust::{
    cursor_list_page_data, offset_window_page_info, SdkWorkPageData, SdkWorkResourceData,
};

pub fn resource_item<T>(item: T) -> SdkWorkResourceData<T> {
    SdkWorkResourceData { item }
}

/// SQL window already fetched with `LIMIT page_size + 1 OFFSET offset`.
pub fn bounded_sql_list_page<T>(
    records: Vec<T>,
    page_size: usize,
    offset: usize,
) -> SdkWorkPageData<T> {
    let has_more = records.len() > page_size;
    let mut items = records;
    if has_more {
        items.truncate(page_size);
    }
    let next_cursor = has_more.then(|| offset.saturating_add(items.len()).to_string());
    SdkWorkPageData {
        items,
        page_info: offset_window_page_info(Some(page_size), next_cursor, has_more),
    }
}

/// Full in-memory inventory dump (no continuation cursor).
pub fn full_inventory_page<T>(items: Vec<T>) -> SdkWorkPageData<T> {
    let len = items.len();
    cursor_list_page_data(items, len.max(1), None, false)
}
