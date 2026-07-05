//! Shared list query types for space-service HTTP handlers.

use sdkwork_routes_web_framework_backend_api::response::ApiProblem;
use sdkwork_utils_rust::CursorListPageParams;

pub use sdkwork_utils_rust::SdkWorkCursorListQuery as ListQuery;

pub fn resolve_list_page(query: &ListQuery) -> Result<CursorListPageParams, ApiProblem> {
    query.resolve().map_err(|_| {
        ApiProblem::bad_request("cursor must encode a non-negative numeric offset")
    })
}

pub fn sql_fetch_limit(page: CursorListPageParams) -> i64 {
    i64::try_from(page.page_size.saturating_add(1))
        .unwrap_or(i64::MAX)
}

pub fn sql_fetch_offset(page: CursorListPageParams) -> i64 {
    i64::try_from(page.offset).unwrap_or(0)
}
