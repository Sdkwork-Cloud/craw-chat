//! SdkWorkApiResponse payload helpers (`API_SPEC.md` §14–16).

use sdkwork_utils_rust::{
    offset_limit_page_from_iter, offset_window_page_info, SdkWorkPageData, SdkWorkResourceData,
};

pub fn resource_item<T>(item: T) -> SdkWorkResourceData<T> {
    SdkWorkResourceData { item }
}

/// In-memory ordered collection paging with offset continuation (`nextCursor` when `hasMore`).
pub fn limited_list_page<T>(items: Vec<T>, page_size: usize, offset: usize) -> SdkWorkPageData<T> {
    let page = offset_limit_page_from_iter(items.into_iter(), page_size, offset);
    SdkWorkPageData {
        items: page.items,
        page_info: offset_window_page_info(Some(page_size), page.next_cursor, page.has_more),
    }
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
