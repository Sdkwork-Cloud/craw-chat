use im_domain_core::realtime::{RealtimeEvent, RealtimeEventWindow};
use sdkwork_utils_rust::{PageInfo, PageMode, SdkWorkPageData};
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RealtimeEventsListData {
    pub device_id: String,
    pub acked_through_seq: u64,
    pub trimmed_through_seq: u64,
    #[serde(flatten)]
    pub page: SdkWorkPageData<RealtimeEvent>,
}

pub fn realtime_events_list_from_window(
    window: RealtimeEventWindow,
    page_size: usize,
) -> RealtimeEventsListData {
    RealtimeEventsListData {
        device_id: window.device_id,
        acked_through_seq: window.acked_through_seq,
        trimmed_through_seq: window.trimmed_through_seq,
        page: SdkWorkPageData {
            items: window.items,
            page_info: PageInfo {
                mode: PageMode::Cursor,
                page: None,
                page_size: Some(page_size as i32),
                total_items: None,
                total_pages: None,
                next_cursor: window.next_after_seq.map(|value| value.to_string()),
                has_more: Some(window.has_more),
            },
        },
    }
}
