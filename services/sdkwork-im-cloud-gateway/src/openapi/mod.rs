//! OpenAPI surface assembly: handlers, document aggregation, discovery schemas,
//! and docs spec builders served by the gateway.

mod aggregate;
mod cache;
mod discovery;
mod handlers;
mod sdk_contract_documents;
mod spec;

pub(crate) use cache::OpenApiAggregateCache;
pub(crate) use handlers::{
    docs, im_app_sdk_openapi_json, im_backend_sdk_openapi_json, im_sdk_openapi_json,
    openapi_index_json, openapi_json, openapi_runtime_summary_json, service_docs,
    service_openapi_json,
};
