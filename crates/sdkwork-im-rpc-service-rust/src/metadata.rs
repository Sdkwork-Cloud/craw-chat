use axum::http::{HeaderMap, HeaderName, HeaderValue};
use im_app_context::{
    app_context_signature_header_name, signed_app_context_projection_header_names,
};
use tonic::metadata::MetadataMap;
use tonic::metadata::MetadataValue;

use crate::ImRpcError;

pub const METADATA_AUTHORIZATION: &str = "authorization";
pub const METADATA_ACCESS_TOKEN: &str = "access-token";
pub const METADATA_TRACE_ID: &str = "x-sdkwork-trace-id";
pub const METADATA_TRACEPARENT: &str = "traceparent";
pub const METADATA_IDEMPOTENCY_KEY: &str = "idempotency-key";
pub const METADATA_REQUEST_HASH: &str = "x-request-hash";
pub const METADATA_CLIENT_VERSION: &str = "x-sdkwork-client-version";
pub const METADATA_SERVICE_IDENTITY: &str = "x-sdkwork-service";

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct RpcMetadata {
    pub authorization: Option<String>,
    pub access_token: Option<String>,
    pub trace_id: Option<String>,
    pub traceparent: Option<String>,
    pub idempotency_key: Option<String>,
    pub request_hash: Option<String>,
    pub client_version: Option<String>,
    pub service_identity: Option<String>,
    pub orchestration_headers: Vec<(String, String)>,
}

impl RpcMetadata {
    pub fn from_metadata_map(metadata: &MetadataMap) -> Result<Self, ImRpcError> {
        Ok(Self {
            authorization: optional_ascii_metadata(metadata, METADATA_AUTHORIZATION)?,
            access_token: optional_ascii_metadata(metadata, METADATA_ACCESS_TOKEN)?,
            trace_id: optional_ascii_metadata(metadata, METADATA_TRACE_ID)?,
            traceparent: optional_ascii_metadata(metadata, METADATA_TRACEPARENT)?,
            idempotency_key: optional_ascii_metadata(metadata, METADATA_IDEMPOTENCY_KEY)?,
            request_hash: optional_ascii_metadata(metadata, METADATA_REQUEST_HASH)?,
            client_version: optional_ascii_metadata(metadata, METADATA_CLIENT_VERSION)?,
            service_identity: optional_ascii_metadata(metadata, METADATA_SERVICE_IDENTITY)?,
            orchestration_headers: signed_orchestration_headers_from_metadata(metadata)?,
        })
    }

    pub fn to_header_map(&self) -> MetadataMap {
        let mut headers = MetadataMap::new();

        // Helper macro to insert optional metadata values
        macro_rules! insert_if_valid {
            ($field:expr, $key:expr) => {
                if let Some(value) = $field {
                    if let Ok(parsed) = MetadataValue::try_from(value.as_str()) {
                        headers.insert($key, parsed);
                    }
                }
            };
        }

        insert_if_valid!(&self.authorization, METADATA_AUTHORIZATION);
        insert_if_valid!(&self.access_token, METADATA_ACCESS_TOKEN);
        insert_if_valid!(&self.trace_id, METADATA_TRACE_ID);
        insert_if_valid!(&self.traceparent, METADATA_TRACEPARENT);
        insert_if_valid!(&self.idempotency_key, METADATA_IDEMPOTENCY_KEY);
        insert_if_valid!(&self.request_hash, METADATA_REQUEST_HASH);
        insert_if_valid!(&self.client_version, METADATA_CLIENT_VERSION);
        insert_if_valid!(&self.service_identity, METADATA_SERVICE_IDENTITY);
        for (name, value) in &self.orchestration_headers {
            if let Ok(parsed) = MetadataValue::try_from(value.as_str())
                && let Ok(key) =
                    name.parse::<tonic::metadata::MetadataKey<tonic::metadata::Ascii>>()
            {
                headers.insert(key, parsed);
            }
        }

        headers
    }

    pub fn from_orchestration_http_headers(
        headers: &HeaderMap,
        service_identity: Option<String>,
        idempotency_key: Option<String>,
        trace_id: Option<String>,
    ) -> Self {
        let mut orchestration_headers = Vec::new();
        for name in signed_app_context_projection_header_names()
            .iter()
            .copied()
            .chain(std::iter::once(app_context_signature_header_name()))
        {
            if let Some(value) = header_value(headers, name) {
                orchestration_headers.push((name.to_owned(), value));
            }
        }
        Self {
            service_identity,
            idempotency_key,
            trace_id,
            orchestration_headers,
            ..Self::default()
        }
    }

    pub fn to_orchestration_header_map(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        insert_axum_header(
            &mut headers,
            METADATA_SERVICE_IDENTITY,
            &self.service_identity,
        );
        insert_axum_header(
            &mut headers,
            METADATA_IDEMPOTENCY_KEY,
            &self.idempotency_key,
        );
        insert_axum_header(&mut headers, METADATA_TRACE_ID, &self.trace_id);
        insert_axum_header(&mut headers, METADATA_TRACEPARENT, &self.traceparent);
        for (name, value) in &self.orchestration_headers {
            if let Ok(name) = HeaderName::from_bytes(name.as_bytes())
                && let Ok(value) = HeaderValue::from_str(value)
            {
                headers.insert(name, value);
            }
        }
        headers
    }
}

fn signed_orchestration_headers_from_metadata(
    metadata: &MetadataMap,
) -> Result<Vec<(String, String)>, ImRpcError> {
    let mut headers = Vec::new();
    for name in signed_app_context_projection_header_names()
        .iter()
        .copied()
        .chain(std::iter::once(app_context_signature_header_name()))
    {
        if let Some(value) = optional_ascii_metadata(metadata, name)? {
            headers.push((name.to_owned(), value));
        }
    }
    Ok(headers)
}

fn optional_ascii_metadata(
    metadata: &MetadataMap,
    key: &'static str,
) -> Result<Option<String>, ImRpcError> {
    metadata
        .get(key)
        .map(|value| {
            value
                .to_str()
                .map(str::to_owned)
                .map_err(|_| ImRpcError::invalid_argument(format!("metadata {key} is not ASCII")))
        })
        .transpose()
}

fn header_value(headers: &HeaderMap, key: &'static str) -> Option<String> {
    headers
        .get(key)
        .and_then(|value| value.to_str().ok())
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_owned)
}

fn insert_axum_header(headers: &mut HeaderMap, key: &'static str, value: &Option<String>) {
    if let Some(value) = value
        && let Ok(parsed) = HeaderValue::from_str(value)
    {
        headers.insert(key, parsed);
    }
}
