//! OpenAPI surface assembly: handlers, document aggregation, discovery schemas,
//! and docs spec builders served by the gateway.

mod aggregate;
mod discovery;
mod handlers;
mod spec;

pub(crate) use handlers::{
    docs, openapi_index_json, openapi_json, openapi_runtime_summary_json, service_docs,
    service_openapi_json,
};
