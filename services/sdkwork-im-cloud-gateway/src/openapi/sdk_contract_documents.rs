//! Compiled SDK contract documents served by the gateway's public schema URLs.
//!
//! The source OpenAPI authorities are embedded at compile time so standalone
//! deployments do not depend on source-checkout files at runtime.

use std::fmt;
use std::sync::{Arc, OnceLock};

use serde_json::Value;

const IM_OPEN_API_SOURCE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../apis/open-api/im/sdkwork-im-im.openapi.yaml"
));
const IM_APP_API_SOURCE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../apis/app-api/communication/sdkwork-im-app-api.openapi.yaml"
));
const IM_BACKEND_API_SOURCE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../apis/backend-api/communication/sdkwork-im-backend-api.openapi.yaml"
));

static IM_OPEN_API_DOCUMENT: OnceLock<Result<Arc<Value>, SdkContractDocumentError>> =
    OnceLock::new();
static IM_APP_API_DOCUMENT: OnceLock<Result<Arc<Value>, SdkContractDocumentError>> =
    OnceLock::new();
static IM_BACKEND_API_DOCUMENT: OnceLock<Result<Arc<Value>, SdkContractDocumentError>> =
    OnceLock::new();

#[derive(Clone, Copy, Debug)]
pub(crate) enum SdkContractDocument {
    Im,
    App,
    Backend,
}

impl SdkContractDocument {
    pub(crate) const fn identifier(self) -> &'static str {
        match self {
            Self::Im => "sdkwork-im-open-api",
            Self::App => "sdkwork-im-app-api",
            Self::Backend => "sdkwork-im-backend-api",
        }
    }

    const fn source(self) -> &'static str {
        match self {
            Self::Im => IM_OPEN_API_SOURCE,
            Self::App => IM_APP_API_SOURCE,
            Self::Backend => IM_BACKEND_API_SOURCE,
        }
    }

    const fn cache(self) -> &'static OnceLock<Result<Arc<Value>, SdkContractDocumentError>> {
        match self {
            Self::Im => &IM_OPEN_API_DOCUMENT,
            Self::App => &IM_APP_API_DOCUMENT,
            Self::Backend => &IM_BACKEND_API_DOCUMENT,
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) struct SdkContractDocumentError {
    contract: SdkContractDocument,
    detail: String,
}

impl fmt::Display for SdkContractDocumentError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "failed to parse embedded {} document: {}",
            self.contract.identifier(),
            self.detail
        )
    }
}

impl std::error::Error for SdkContractDocumentError {}

impl SdkContractDocumentError {
    pub(crate) const fn contract_identifier(&self) -> &'static str {
        self.contract.identifier()
    }
}

pub(crate) fn sdk_contract_document(
    contract: SdkContractDocument,
) -> Result<Arc<Value>, SdkContractDocumentError> {
    contract
        .cache()
        .get_or_init(|| parse_sdk_contract_document(contract))
        .clone()
}

fn parse_sdk_contract_document(
    contract: SdkContractDocument,
) -> Result<Arc<Value>, SdkContractDocumentError> {
    let document: Value =
        serde_yaml::from_str(contract.source()).map_err(|error| SdkContractDocumentError {
            contract,
            detail: error.to_string(),
        })?;
    let version = document.get("openapi").and_then(Value::as_str);
    if !version.is_some_and(|value| value.starts_with("3.")) {
        return Err(SdkContractDocumentError {
            contract,
            detail: "OpenAPI document is missing a supported openapi version".to_owned(),
        });
    }
    Ok(Arc::new(document))
}
