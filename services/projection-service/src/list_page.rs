use sdkwork_utils_rust::{PageInfo, PageMode, SdkWorkPageData, cursor_list_page_data};

pub(crate) fn cursor_page<T>(
    items: Vec<T>,
    page_size: usize,
    next_cursor: Option<String>,
    has_more: bool,
) -> SdkWorkPageData<T> {
    cursor_list_page_data(items, page_size, next_cursor, has_more)
}

pub(crate) fn seq_cursor_page<T>(
    items: Vec<T>,
    page_size: usize,
    next_after_seq: Option<u64>,
    has_more: bool,
) -> SdkWorkPageData<T> {
    SdkWorkPageData {
        items,
        page_info: PageInfo {
            mode: PageMode::Cursor,
            page: None,
            page_size: Some(page_size as i32),
            total_items: None,
            total_pages: None,
            next_cursor: next_after_seq.map(|value| value.to_string()),
            has_more: Some(has_more),
        },
    }
}

pub(crate) fn cursor_page_with_total<T>(
    items: Vec<T>,
    page_size: usize,
    next_cursor: Option<String>,
    has_more: bool,
    total_count: u64,
) -> SdkWorkPageData<T> {
    SdkWorkPageData {
        items,
        page_info: PageInfo {
            mode: PageMode::Cursor,
            page: None,
            page_size: Some(page_size as i32),
            total_items: Some(total_count.to_string()),
            total_pages: None,
            next_cursor,
            has_more: Some(has_more),
        },
    }
}
