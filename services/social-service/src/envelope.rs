//! Shared SdkWorkApiResponse serialization for social HTTP handlers.

use axum::response::Response;
use sdkwork_routes_web_framework_backend_api::response::finish_api_json;
use sdkwork_web_core::WebRequestContext;
use serde::Serialize;

use crate::friendship::SocialServiceError;

pub(crate) fn finish_enveloped_json<T: Serialize>(
    ctx: &WebRequestContext,
    result: Result<T, SocialServiceError>,
) -> Response {
    finish_api_json(ctx, result.map_err(Into::into))
}
