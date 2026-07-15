//! Process configuration boundary for the IM -> Knowledgebase RPC adapter.
//!
//! The adapter itself receives typed configuration and never reads process
//! state. This module is the only place in the conversation runtime that
//! resolves the outbound endpoint, mTLS material, and service signing secret.

use std::collections::BTreeMap;
use std::sync::Arc;
use std::time::Duration;

use sdkwork_rpc_framework_core::RpcCallerContextSigningKey;

use super::knowledgebase_rpc_adapter::{
    KnowledgebaseGroupLifecycleRpcAdapterError, KnowledgebaseGroupLifecycleRpcPort,
    KnowledgebaseGroupLifecycleRpcPortConfig,
};
use super::{GroupKnowledgebasePort, RuntimeError};

pub(crate) const KNOWLEDGEBASE_RPC_CLIENT_ENDPOINT_ENV: &str =
    "SDKWORK_IM_KNOWLEDGEBASE_RPC_CLIENT_ENDPOINT";
pub(crate) const KNOWLEDGEBASE_RPC_CLIENT_CA_CERT_PATH_ENV: &str =
    "SDKWORK_IM_KNOWLEDGEBASE_RPC_CLIENT_CA_CERT_PATH";
pub(crate) const KNOWLEDGEBASE_RPC_CLIENT_CERT_PATH_ENV: &str =
    "SDKWORK_IM_KNOWLEDGEBASE_RPC_CLIENT_CERT_PATH";
pub(crate) const KNOWLEDGEBASE_RPC_CLIENT_KEY_PATH_ENV: &str =
    "SDKWORK_IM_KNOWLEDGEBASE_RPC_CLIENT_KEY_PATH";
pub(crate) const KNOWLEDGEBASE_RPC_CLIENT_TLS_DOMAIN_ENV: &str =
    "SDKWORK_IM_KNOWLEDGEBASE_RPC_CLIENT_TLS_DOMAIN";
pub(crate) const KNOWLEDGEBASE_RPC_CLIENT_CALLER_CONTEXT_SIGNING_KEY_ENV: &str =
    "SDKWORK_IM_KNOWLEDGEBASE_RPC_CLIENT_CALLER_CONTEXT_SIGNING_KEY";
pub(crate) const KNOWLEDGEBASE_RPC_CLIENT_CALLER_CONTEXT_SIGNING_KEY_FILE_ENV: &str =
    "SDKWORK_IM_KNOWLEDGEBASE_RPC_CLIENT_CALLER_CONTEXT_SIGNING_KEY_FILE";
pub(crate) const KNOWLEDGEBASE_RPC_CLIENT_CREDENTIAL_TTL_SECONDS_ENV: &str =
    "SDKWORK_IM_KNOWLEDGEBASE_RPC_CLIENT_CREDENTIAL_TTL_SECONDS";
pub(crate) const KNOWLEDGEBASE_RPC_CLIENT_TIMEOUT_MS_ENV: &str =
    "SDKWORK_IM_KNOWLEDGEBASE_RPC_CLIENT_TIMEOUT_MS";

const NON_SECRET_CONFIG_KEYS: [&str; 7] = [
    KNOWLEDGEBASE_RPC_CLIENT_ENDPOINT_ENV,
    KNOWLEDGEBASE_RPC_CLIENT_CA_CERT_PATH_ENV,
    KNOWLEDGEBASE_RPC_CLIENT_CERT_PATH_ENV,
    KNOWLEDGEBASE_RPC_CLIENT_KEY_PATH_ENV,
    KNOWLEDGEBASE_RPC_CLIENT_TLS_DOMAIN_ENV,
    KNOWLEDGEBASE_RPC_CLIENT_CREDENTIAL_TTL_SECONDS_ENV,
    KNOWLEDGEBASE_RPC_CLIENT_TIMEOUT_MS_ENV,
];

const ALL_CONFIG_KEYS: [&str; 9] = [
    KNOWLEDGEBASE_RPC_CLIENT_ENDPOINT_ENV,
    KNOWLEDGEBASE_RPC_CLIENT_CA_CERT_PATH_ENV,
    KNOWLEDGEBASE_RPC_CLIENT_CERT_PATH_ENV,
    KNOWLEDGEBASE_RPC_CLIENT_KEY_PATH_ENV,
    KNOWLEDGEBASE_RPC_CLIENT_TLS_DOMAIN_ENV,
    KNOWLEDGEBASE_RPC_CLIENT_CALLER_CONTEXT_SIGNING_KEY_ENV,
    KNOWLEDGEBASE_RPC_CLIENT_CALLER_CONTEXT_SIGNING_KEY_FILE_ENV,
    KNOWLEDGEBASE_RPC_CLIENT_CREDENTIAL_TTL_SECONDS_ENV,
    KNOWLEDGEBASE_RPC_CLIENT_TIMEOUT_MS_ENV,
];

/// Resolves an outbound port only when all client inputs are supplied. In
/// development/test an entirely absent client is allowed, but partial setup is
/// always rejected. Production must receive a complete mTLS configuration.
pub(crate) fn resolve_group_knowledgebase_rpc_port_from_env()
-> Result<Option<Arc<dyn GroupKnowledgebasePort>>, RuntimeError> {
    let values = read_environment_values()?;
    let require_configuration = !im_app_context::allows_header_only_app_context_fallback();
    let config = build_port_config(&values, require_configuration)?;
    config
        .map(|config| {
            KnowledgebaseGroupLifecycleRpcPort::new(config)
                .map(|port| Arc::new(port) as Arc<dyn GroupKnowledgebasePort>)
                .map_err(map_adapter_error)
        })
        .transpose()
}

fn build_port_config(
    values: &BTreeMap<&'static str, Option<String>>,
    require_configuration: bool,
) -> Result<Option<KnowledgebaseGroupLifecycleRpcPortConfig>, RuntimeError> {
    let configured_non_secret_count = NON_SECRET_CONFIG_KEYS
        .iter()
        .filter(|key| configured_value(values, key).is_some())
        .count();
    let configured_secret_count = if configured_value(
        values,
        KNOWLEDGEBASE_RPC_CLIENT_CALLER_CONTEXT_SIGNING_KEY_ENV,
    )
    .is_some()
        || configured_value(
            values,
            KNOWLEDGEBASE_RPC_CLIENT_CALLER_CONTEXT_SIGNING_KEY_FILE_ENV,
        )
        .is_some()
    {
        1
    } else {
        0
    };

    if configured_non_secret_count == 0 && configured_secret_count == 0 {
        if require_configuration {
            return Err(configuration_error(
                "the production Knowledgebase RPC client is not configured",
            ));
        }
        return Ok(None);
    }
    if configured_non_secret_count != NON_SECRET_CONFIG_KEYS.len() || configured_secret_count != 1 {
        return Err(configuration_error(
            "the Knowledgebase RPC client must be fully configured or fully absent in development/test",
        ));
    }

    let signing_key = RpcCallerContextSigningKey::from_base64url(
        resolve_signing_key(values)?.as_str(),
    )
    .map_err(|_| {
        configuration_error(
            "the Knowledgebase RPC client caller-context signing key must be an unpadded base64url 32-byte key",
        )
    })?;
    let credential_ttl_seconds = parse_positive_u64(
        KNOWLEDGEBASE_RPC_CLIENT_CREDENTIAL_TTL_SECONDS_ENV,
        required_value(values, KNOWLEDGEBASE_RPC_CLIENT_CREDENTIAL_TTL_SECONDS_ENV)?,
    )?;
    let request_timeout_ms = parse_positive_u64(
        KNOWLEDGEBASE_RPC_CLIENT_TIMEOUT_MS_ENV,
        required_value(values, KNOWLEDGEBASE_RPC_CLIENT_TIMEOUT_MS_ENV)?,
    )?;

    KnowledgebaseGroupLifecycleRpcPortConfig::new(
        required_value(values, KNOWLEDGEBASE_RPC_CLIENT_ENDPOINT_ENV)?,
        required_value(values, KNOWLEDGEBASE_RPC_CLIENT_CA_CERT_PATH_ENV)?,
        required_value(values, KNOWLEDGEBASE_RPC_CLIENT_CERT_PATH_ENV)?,
        required_value(values, KNOWLEDGEBASE_RPC_CLIENT_KEY_PATH_ENV)?,
        required_value(values, KNOWLEDGEBASE_RPC_CLIENT_TLS_DOMAIN_ENV)?,
        signing_key,
        Duration::from_secs(credential_ttl_seconds),
        Duration::from_millis(request_timeout_ms),
    )
    .map(Some)
    .map_err(map_adapter_error)
}

fn read_environment_values() -> Result<BTreeMap<&'static str, Option<String>>, RuntimeError> {
    ALL_CONFIG_KEYS
        .iter()
        .map(|key| {
            let value = match std::env::var(key) {
                Ok(value) if !value.trim().is_empty() => Some(value.trim().to_owned()),
                Ok(_) | Err(std::env::VarError::NotPresent) => None,
                Err(std::env::VarError::NotUnicode(_)) => {
                    return Err(configuration_error(
                        "a Knowledgebase RPC client environment variable is not valid Unicode",
                    ));
                }
            };
            Ok((*key, value))
        })
        .collect()
}

fn resolve_signing_key(
    values: &BTreeMap<&'static str, Option<String>>,
) -> Result<String, RuntimeError> {
    let direct = configured_value(
        values,
        KNOWLEDGEBASE_RPC_CLIENT_CALLER_CONTEXT_SIGNING_KEY_ENV,
    );
    let file = configured_value(
        values,
        KNOWLEDGEBASE_RPC_CLIENT_CALLER_CONTEXT_SIGNING_KEY_FILE_ENV,
    );
    match (direct, file) {
        (Some(_), Some(_)) => Err(configuration_error(
            "the Knowledgebase RPC client signing key and signing-key file are mutually exclusive",
        )),
        (Some(value), None) => Ok(value.to_owned()),
        (None, Some(path)) => std::fs::read_to_string(path)
            .map_err(|_| {
                configuration_error(
                    "the Knowledgebase RPC client signing-key file could not be read",
                )
            })
            .map(|value| value.trim().to_owned())
            .and_then(|value| {
                if value.is_empty() {
                    Err(configuration_error(
                        "the Knowledgebase RPC client signing key must not be blank",
                    ))
                } else {
                    Ok(value)
                }
            }),
        (None, None) => Err(configuration_error(
            "the Knowledgebase RPC client caller-context signing key is required",
        )),
    }
}

fn configured_value<'a>(
    values: &'a BTreeMap<&'static str, Option<String>>,
    key: &str,
) -> Option<&'a str> {
    values
        .get(key)
        .and_then(Option::as_deref)
        .filter(|value| !value.trim().is_empty())
}

fn required_value<'a>(
    values: &'a BTreeMap<&'static str, Option<String>>,
    key: &'static str,
) -> Result<&'a str, RuntimeError> {
    configured_value(values, key).ok_or_else(|| {
        configuration_error("a required Knowledgebase RPC client setting is missing")
    })
}

fn parse_positive_u64(key: &str, value: &str) -> Result<u64, RuntimeError> {
    let parsed = value.parse::<u64>().map_err(|_| {
        configuration_error("a Knowledgebase RPC client numeric setting is invalid")
    })?;
    if parsed == 0 {
        return Err(configuration_error(
            "a Knowledgebase RPC client numeric setting must be positive",
        ));
    }
    let _ = key;
    Ok(parsed)
}

fn map_adapter_error(error: KnowledgebaseGroupLifecycleRpcAdapterError) -> RuntimeError {
    match error {
        KnowledgebaseGroupLifecycleRpcAdapterError::Configuration(_) => {
            configuration_error("the Knowledgebase RPC client configuration is invalid")
        }
        KnowledgebaseGroupLifecycleRpcAdapterError::RpcFramework => {
            configuration_error("the Knowledgebase RPC client transport configuration is invalid")
        }
    }
}

fn configuration_error(message: &str) -> RuntimeError {
    RuntimeError::Contract(im_platform_contracts::ContractError::Unavailable(
        message.to_owned(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn values() -> BTreeMap<&'static str, Option<String>> {
        ALL_CONFIG_KEYS.iter().map(|key| (*key, None)).collect()
    }

    #[test]
    fn development_allows_a_fully_absent_rpc_client() {
        assert!(
            build_port_config(&values(), false)
                .expect("development config")
                .is_none()
        );
    }

    #[test]
    fn production_requires_a_complete_rpc_client_configuration() {
        assert!(build_port_config(&values(), true).is_err());
    }

    #[test]
    fn partial_configuration_fails_closed_even_for_development() {
        let mut configured = values();
        configured.insert(
            KNOWLEDGEBASE_RPC_CLIENT_ENDPOINT_ENV,
            Some("grpcs://knowledgebase.internal:7443".to_owned()),
        );
        assert!(build_port_config(&configured, false).is_err());
    }

    #[test]
    fn inbound_and_outbound_signing_key_names_are_directionally_distinct() {
        assert_ne!(
            KNOWLEDGEBASE_RPC_CLIENT_CALLER_CONTEXT_SIGNING_KEY_ENV,
            "SDKWORK_IM_KNOWLEDGEBASE_RPC_CALLER_CONTEXT_SIGNING_KEY"
        );
        assert_ne!(
            KNOWLEDGEBASE_RPC_CLIENT_CALLER_CONTEXT_SIGNING_KEY_FILE_ENV,
            "SDKWORK_IM_KNOWLEDGEBASE_RPC_CALLER_CONTEXT_SIGNING_KEY_FILE"
        );
    }

    #[test]
    fn direct_and_file_signing_key_sources_are_mutually_exclusive() {
        let mut configured = values();
        configured.insert(
            KNOWLEDGEBASE_RPC_CLIENT_CALLER_CONTEXT_SIGNING_KEY_ENV,
            Some("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA".to_owned()),
        );
        configured.insert(
            KNOWLEDGEBASE_RPC_CLIENT_CALLER_CONTEXT_SIGNING_KEY_FILE_ENV,
            Some("C:/secrets/knowledgebase-rpc.key".to_owned()),
        );
        assert!(resolve_signing_key(&configured).is_err());
    }
}
