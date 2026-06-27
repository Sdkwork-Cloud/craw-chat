use tonic::metadata::MetadataMap;
use tonic::metadata::MetadataValue;

use crate::ImRpcError;

pub const METADATA_AUTHORIZATION: &str = "authorization";
pub const METADATA_ACCESS_TOKEN: &str = "access-token";
pub const METADATA_REQUEST_ID: &str = "x-request-id";
pub const METADATA_TRACEPARENT: &str = "traceparent";
pub const METADATA_IDEMPOTENCY_KEY: &str = "idempotency-key";
pub const METADATA_REQUEST_HASH: &str = "x-request-hash";
pub const METADATA_CLIENT_VERSION: &str = "x-sdkwork-client-version";
pub const METADATA_SERVICE_IDENTITY: &str = "x-sdkwork-service";

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct RpcMetadata {
    pub authorization: Option<String>,
    pub access_token: Option<String>,
    pub request_id: Option<String>,
    pub traceparent: Option<String>,
    pub idempotency_key: Option<String>,
    pub request_hash: Option<String>,
    pub client_version: Option<String>,
    pub service_identity: Option<String>,
}

impl RpcMetadata {
    pub fn from_metadata_map(metadata: &MetadataMap) -> Result<Self, ImRpcError> {
        Ok(Self {
            authorization: optional_ascii_metadata(metadata, METADATA_AUTHORIZATION)?,
            access_token: optional_ascii_metadata(metadata, METADATA_ACCESS_TOKEN)?,
            request_id: optional_ascii_metadata(metadata, METADATA_REQUEST_ID)?,
            traceparent: optional_ascii_metadata(metadata, METADATA_TRACEPARENT)?,
            idempotency_key: optional_ascii_metadata(metadata, METADATA_IDEMPOTENCY_KEY)?,
            request_hash: optional_ascii_metadata(metadata, METADATA_REQUEST_HASH)?,
            client_version: optional_ascii_metadata(metadata, METADATA_CLIENT_VERSION)?,
            service_identity: optional_ascii_metadata(metadata, METADATA_SERVICE_IDENTITY)?,
        })
    }

    pub fn to_header_map(&self) -> MetadataMap {
        let mut headers = MetadataMap::new();
        if let Some(value) = &self.authorization {
            if let Ok(parsed) = MetadataValue::try_from(value.as_str()) {
                headers.insert(METADATA_AUTHORIZATION, parsed);
            }
        }
        if let Some(value) = &self.access_token {
            if let Ok(parsed) = MetadataValue::try_from(value.as_str()) {
                headers.insert(METADATA_ACCESS_TOKEN, parsed);
            }
        }
        if let Some(value) = &self.request_id {
            if let Ok(parsed) = MetadataValue::try_from(value.as_str()) {
                headers.insert(METADATA_REQUEST_ID, parsed);
            }
        }
        if let Some(value) = &self.traceparent {
            if let Ok(parsed) = MetadataValue::try_from(value.as_str()) {
                headers.insert(METADATA_TRACEPARENT, parsed);
            }
        }
        if let Some(value) = &self.idempotency_key {
            if let Ok(parsed) = MetadataValue::try_from(value.as_str()) {
                headers.insert(METADATA_IDEMPOTENCY_KEY, parsed);
            }
        }
        if let Some(value) = &self.request_hash {
            if let Ok(parsed) = MetadataValue::try_from(value.as_str()) {
                headers.insert(METADATA_REQUEST_HASH, parsed);
            }
        }
        if let Some(value) = &self.client_version {
            if let Ok(parsed) = MetadataValue::try_from(value.as_str()) {
                headers.insert(METADATA_CLIENT_VERSION, parsed);
            }
        }
        if let Some(value) = &self.service_identity {
            if let Ok(parsed) = MetadataValue::try_from(value.as_str()) {
                headers.insert(METADATA_SERVICE_IDENTITY, parsed);
            }
        }
        headers
    }
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
