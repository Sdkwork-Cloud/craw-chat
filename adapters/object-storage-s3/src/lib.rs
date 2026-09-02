use std::collections::BTreeMap;
use std::time::{SystemTime, UNIX_EPOCH};

use hmac::{Hmac, Mac};
use im_platform_contracts::{
    ObjectStorageDownloadUrlRequest, ObjectStorageObjectDescriptor, ObjectStorageProvider,
    ObjectStoragePutRequest, ObjectStorageUploadSession, ObjectStorageUploadUrlRequest,
    ProviderDomain, ProviderHealthSnapshot, ProviderPluginDescriptor,
};
use im_time::{format_unix_timestamp_millis, utc_now_rfc3339_millis};
use sdkwork_im_contract_core::ContractError;
use sdkwork_utils_rust::{hex_encode, sha256_hash};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

pub const ALIYUN_OBJECT_STORAGE_PLUGIN_ID: &str = "object-storage-aliyun";
pub const TENCENT_OBJECT_STORAGE_PLUGIN_ID: &str = "object-storage-tencent";
pub const VOLCENGINE_OBJECT_STORAGE_PLUGIN_ID: &str = "object-storage-volcengine";
pub const AWS_OBJECT_STORAGE_PLUGIN_ID: &str = "object-storage-aws";
pub const GOOGLE_OBJECT_STORAGE_PLUGIN_ID: &str = "object-storage-google";
pub const MICROSOFT_OBJECT_STORAGE_PLUGIN_ID: &str = "object-storage-microsoft";

const S3_ACCESS_KEY_ID_ENV: &str = "SDKWORK_IM_S3_ACCESS_KEY_ID";
const S3_SECRET_ACCESS_KEY_ENV: &str = "SDKWORK_IM_S3_SECRET_ACCESS_KEY";
const S3_SECURITY_TOKEN_ENV: &str = "SDKWORK_IM_S3_SECURITY_TOKEN";
const S3_KMS_KEY_ID_ENV: &str = "SDKWORK_IM_S3_KMS_KEY_ID";
const ENVIRONMENT_ENV: &str = "SDKWORK_IM_ENVIRONMENT";

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct S3CompatibleObjectStorageProviderConfig {
    pub plugin_id: String,
    pub provider_kind: String,
    pub display_name: String,
    pub endpoint: String,
    pub region: String,
    pub gateway_mode: bool,
    // P0-13: credentials for SigV4 signing. When None, URLs are unsigned
    // (dev/test only). Production MUST configure credentials via env vars.
    pub access_key_id: Option<String>,
    pub secret_access_key: Option<String>,
    pub security_token: Option<String>,
    // P0-13: KMS key ARN for SSE-KMS encryption. When None, SSE-S3 is used.
    pub kms_key_id: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct S3CompatibleObjectStorageProvider {
    config: S3CompatibleObjectStorageProviderConfig,
}

impl S3CompatibleObjectStorageProvider {
    pub fn new(config: S3CompatibleObjectStorageProviderConfig) -> Self {
        Self { config }
    }

    pub fn aliyun_default() -> Self {
        Self::new(Self::base_config(
            ALIYUN_OBJECT_STORAGE_PLUGIN_ID,
            "aliyun",
            "Aliyun Object Storage",
            "https://oss.aliyun.local",
            "cn-hangzhou",
            false,
        ))
    }

    pub fn tencent_default() -> Self {
        Self::new(Self::base_config(
            TENCENT_OBJECT_STORAGE_PLUGIN_ID,
            "tencent",
            "Tencent Cloud Object Storage",
            "https://cos.tencent.local",
            "ap-guangzhou",
            false,
        ))
    }

    pub fn volcengine_default() -> Self {
        Self::new(Self::base_config(
            VOLCENGINE_OBJECT_STORAGE_PLUGIN_ID,
            "volcengine",
            "Volcengine Object Storage",
            "https://tos.volcengine.local",
            "cn-beijing",
            false,
        ))
    }

    pub fn aws_default() -> Self {
        Self::new(Self::base_config(
            AWS_OBJECT_STORAGE_PLUGIN_ID,
            "aws",
            "Amazon S3",
            "https://s3.aws.local",
            "us-east-1",
            false,
        ))
    }

    pub fn google_default() -> Self {
        Self::new(Self::base_config(
            GOOGLE_OBJECT_STORAGE_PLUGIN_ID,
            "google",
            "Google Cloud Storage S3 Gateway",
            "https://storage.googleapis.local",
            "us-central1",
            true,
        ))
    }

    pub fn microsoft_default() -> Self {
        Self::new(Self::base_config(
            MICROSOFT_OBJECT_STORAGE_PLUGIN_ID,
            "microsoft",
            "Azure Blob S3 Gateway",
            "https://blob.azure.local",
            "eastasia",
            true,
        ))
    }

    /// Build a provider whose credentials are read from the SDKWork S3
    /// environment variables (`SDKWORK_IM_S3_ACCESS_KEY_ID`,
    /// `SDKWORK_IM_S3_SECRET_ACCESS_KEY`, optional `SDKWORK_IM_S3_SECURITY_TOKEN`
    /// and `SDKWORK_IM_S3_KMS_KEY_ID`). Missing optional vars yield `None`.
    pub fn from_env(
        plugin_id: impl Into<String>,
        provider_kind: impl Into<String>,
        display_name: impl Into<String>,
        endpoint: impl Into<String>,
        region: impl Into<String>,
        gateway_mode: bool,
    ) -> Self {
        let config = S3CompatibleObjectStorageProviderConfig {
            plugin_id: plugin_id.into(),
            provider_kind: provider_kind.into(),
            display_name: display_name.into(),
            endpoint: endpoint.into(),
            region: region.into(),
            gateway_mode,
            access_key_id: std::env::var(S3_ACCESS_KEY_ID_ENV).ok(),
            secret_access_key: std::env::var(S3_SECRET_ACCESS_KEY_ENV).ok(),
            security_token: std::env::var(S3_SECURITY_TOKEN_ENV).ok(),
            kms_key_id: std::env::var(S3_KMS_KEY_ID_ENV).ok(),
        };
        Self::new(config)
    }

    fn base_config(
        plugin_id: &str,
        provider_kind: &str,
        display_name: &str,
        endpoint: &str,
        region: &str,
        gateway_mode: bool,
    ) -> S3CompatibleObjectStorageProviderConfig {
        S3CompatibleObjectStorageProviderConfig {
            plugin_id: plugin_id.into(),
            provider_kind: provider_kind.into(),
            display_name: display_name.into(),
            endpoint: endpoint.into(),
            region: region.into(),
            gateway_mode,
            access_key_id: None,
            secret_access_key: None,
            security_token: None,
            kms_key_id: None,
        }
    }

    fn descriptor_with_defaults(&self) -> ProviderPluginDescriptor {
        let descriptor = ProviderPluginDescriptor::new(
            self.config.plugin_id.clone(),
            ProviderDomain::ObjectStorage,
            self.config.provider_kind.clone(),
            self.config.display_name.clone(),
        );
        if self.config.gateway_mode {
            descriptor
                .with_required_capabilities(["s3-gateway", "presign"])
                .with_optional_capabilities(["retention"])
        } else {
            // `multipart` is intentionally not declared: no multipart upload
            // implementation exists in this adapter. Only presigned transfer
            // and retention metadata are real capabilities.
            descriptor
                .with_required_capabilities(["s3", "presign"])
                .with_optional_capabilities(["retention"])
        }
    }

    fn has_credentials(&self) -> bool {
        self.config.access_key_id.is_some() && self.config.secret_access_key.is_some()
    }

    /// P0-13 fail-closed: in production, SigV4 credentials MUST be configured.
    /// Returns `ContractError::Unavailable` when production is detected and
    /// credentials are missing. In dev/test, missing credentials are allowed
    /// (a warning is emitted and callers fall back to unsigned URLs).
    fn verify_production_credentials(&self) -> Result<(), ContractError> {
        let environment = std::env::var(ENVIRONMENT_ENV)
            .unwrap_or_default()
            .trim()
            .to_ascii_lowercase();
        let is_production = !matches!(
            environment.as_str(),
            "" | "dev" | "development" | "test" | "testing"
        );
        let has_creds = self.has_credentials();
        if is_production && !has_creds {
            return Err(ContractError::Unavailable(format!(
                "P0-13 production fail-closed: S3 SigV4 credentials are required for {} but \
                 {S3_ACCESS_KEY_ID_ENV}/{S3_SECRET_ACCESS_KEY_ENV} are not configured \
                 (environment={environment})",
                self.config.plugin_id
            )));
        }
        if !has_creds {
            tracing::warn!(
                plugin_id = %self.config.plugin_id,
                "P0-13 S3 credentials are not configured; falling back to unsigned presigned URLs (dev/test only)"
            );
        }
        Ok(())
    }

    fn unsigned_download_url(
        &self,
        bucket: &str,
        object_key: &str,
        expires_in_seconds: u32,
    ) -> String {
        format!(
            "{}/{}/{}?provider={}&expires={}",
            self.config.endpoint.trim_end_matches('/'),
            bucket,
            object_key,
            self.config.plugin_id,
            expires_in_seconds
        )
    }

    fn unsigned_upload_url(
        &self,
        bucket: &str,
        object_key: &str,
        expires_in_seconds: u32,
    ) -> String {
        format!(
            "{}/{}/{}?provider={}&expires={}&upload=put",
            self.config.endpoint.trim_end_matches('/'),
            bucket,
            object_key,
            self.config.plugin_id,
            expires_in_seconds
        )
    }

    /// Validate `content_type` against an allowlist of safe MIME types and
    /// cross-check it against the object key's extension when one is known.
    /// `None` is always accepted (the server infers the type).
    pub fn validate_content_type(
        content_type: &Option<String>,
        object_key: &str,
    ) -> Result<(), ContractError> {
        const ALLOWED: &[&str] = &[
            "image/jpeg",
            "image/png",
            "image/gif",
            "image/webp",
            "image/svg+xml",
            "video/mp4",
            "video/webm",
            "video/quicktime",
            "audio/mpeg",
            "audio/mp4",
            "audio/ogg",
            "audio/webm",
            "application/pdf",
            "application/json",
            "text/plain",
            "text/csv",
            "text/markdown",
            "application/octet-stream",
        ];
        let Some(ct) = content_type.as_ref() else {
            return Ok(());
        };
        let bare = ct
            .split(';')
            .next()
            .unwrap_or(ct)
            .trim()
            .to_ascii_lowercase();
        if !ALLOWED.contains(&bare.as_str()) {
            return Err(ContractError::Invalid(format!(
                "unsupported content_type: {ct}"
            )));
        }
        if let Some(expected) = expected_content_type_for_key(object_key)
            && bare != expected
        {
            return Err(ContractError::Invalid(format!(
                "content_type {bare} does not match extension of object key {object_key} (expected {expected})"
            )));
        }
        Ok(())
    }
}

impl ObjectStorageProvider for S3CompatibleObjectStorageProvider {
    fn descriptor(&self) -> ProviderPluginDescriptor {
        self.descriptor_with_defaults()
    }

    fn put_object(
        &self,
        request: ObjectStoragePutRequest,
    ) -> Result<ObjectStorageObjectDescriptor, ContractError> {
        Self::validate_content_type(&request.content_type, &request.object_key)?;
        // Server-side object PUT is not implemented. The production data path
        // uploads through SigV4-presigned URLs (`signed_upload_url`), which is
        // the only supported transfer mode; a direct `put_object` must fail
        // closed instead of returning a fabricated descriptor or ETag.
        Err(ContractError::UnsupportedCapability(format!(
            "{}: server-side put_object is not supported; upload through signed_upload_url instead (bucket={}, object_key={})",
            self.config.plugin_id, request.bucket, request.object_key
        )))
    }

    fn signed_upload_url(
        &self,
        request: ObjectStorageUploadUrlRequest,
    ) -> Result<ObjectStorageUploadSession, ContractError> {
        Self::validate_content_type(&request.content_type, &request.object_key)?;
        self.verify_production_credentials()?;

        let mut headers = BTreeMap::new();
        if let Some(content_type) = request.content_type.as_ref() {
            headers.insert("content-type".into(), content_type.clone());
        }
        // SSE headers: SSE-KMS when a KMS key ARN is configured, otherwise the
        // SSE-S3 default (AES256). These are sent with the PUT request.
        match &self.config.kms_key_id {
            Some(kms_key_id) => {
                headers.insert("X-Amz-Server-Side-Encryption".into(), "aws:kms".into());
                headers.insert(
                    "X-Amz-Server-Side-Encryption-Aws-Kms-Key-Id".into(),
                    kms_key_id.clone(),
                );
            }
            None => {
                headers.insert("X-Amz-Server-Side-Encryption".into(), "AES256".into());
            }
        }

        let expires_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis()
            + (request.expires_in_seconds as u128 * 1_000);

        let url = if self.has_credentials() {
            build_signed_url(
                &self.config.endpoint,
                &request.bucket,
                &request.object_key,
                &self.config.region,
                self.config.access_key_id.as_deref().unwrap_or_default(),
                self.config.secret_access_key.as_deref().unwrap_or_default(),
                self.config.security_token.as_deref(),
                request.expires_in_seconds,
                "PUT",
                &headers,
            )
        } else {
            self.unsigned_upload_url(
                &request.bucket,
                &request.object_key,
                request.expires_in_seconds,
            )
        };

        Ok(ObjectStorageUploadSession {
            method: "PUT".into(),
            url,
            headers,
            expires_at: format_unix_timestamp_millis(expires_at),
        })
    }

    fn signed_download_url(
        &self,
        request: ObjectStorageDownloadUrlRequest,
    ) -> Result<String, ContractError> {
        self.verify_production_credentials()?;
        if self.has_credentials() {
            Ok(build_signed_url(
                &self.config.endpoint,
                &request.bucket,
                &request.object_key,
                &self.config.region,
                self.config.access_key_id.as_deref().unwrap_or_default(),
                self.config.secret_access_key.as_deref().unwrap_or_default(),
                self.config.security_token.as_deref(),
                request.expires_in_seconds,
                "GET",
                &BTreeMap::new(),
            ))
        } else {
            Ok(self.unsigned_download_url(
                &request.bucket,
                &request.object_key,
                request.expires_in_seconds,
            ))
        }
    }

    fn provider_health_snapshot(&self) -> ProviderHealthSnapshot {
        let mut details = BTreeMap::new();
        details.insert("providerKind".into(), self.config.provider_kind.clone());
        details.insert("endpoint".into(), self.config.endpoint.clone());
        details.insert("region".into(), self.config.region.clone());
        details.insert("gatewayMode".into(), self.config.gateway_mode.to_string());
        details.insert(
            "credentialsConfigured".into(),
            self.has_credentials().to_string(),
        );
        details.insert(
            "sseKmsEnabled".into(),
            self.config.kms_key_id.is_some().to_string(),
        );
        let status = if self.has_credentials() {
            "healthy"
        } else {
            "unconfigured"
        };
        ProviderHealthSnapshot {
            plugin_id: self.config.plugin_id.clone(),
            status: status.into(),
            checked_at: utc_now_rfc3339_millis(),
            details,
        }
    }
}

/// Build an AWS SigV4 presigned URL for `method` on `{endpoint}/{bucket}/{key}`.
///
/// The URL carries `X-Amz-Algorithm`, `X-Amz-Credential`, `X-Amz-Date`,
/// `X-Amz-Expires`, `X-Amz-SignedHeaders` (and `X-Amz-Security-Token` when
/// STS temporary credentials are supplied), with `X-Amz-Signature` appended
/// last.
///
/// `extra_signed_headers` are the request headers the caller will send with
/// the request (e.g. `content-type` and `x-amz-server-side-encryption` on a
/// PUT upload). SigV4 requires every header the client sends to participate
/// in the signature, otherwise S3 rejects the request with
/// `SignatureDoesNotMatch`. GET downloads sign only `host`.
#[allow(clippy::too_many_arguments)]
fn build_signed_url(
    endpoint: &str,
    bucket: &str,
    object_key: &str,
    region: &str,
    access_key_id: &str,
    secret_access_key: &str,
    security_token: Option<&str>,
    expires_in_seconds: u32,
    method: &str,
    extra_signed_headers: &BTreeMap<String, String>,
) -> String {
    let host = extract_host(endpoint);
    let canonical_uri = build_canonical_uri(bucket, object_key);

    let (timestamp, date) = format_sigv4_timestamp(SystemTime::now());
    let scope = format!("{date}/{region}/s3/aws4_request");
    let expires_str = expires_in_seconds.to_string();

    // Query parameters that participate in signing (signature appended later).
    let mut params: Vec<(String, String)> = Vec::new();
    params.push(("X-Amz-Algorithm".into(), "AWS4-HMAC-SHA256".into()));
    params.push((
        "X-Amz-Credential".into(),
        format!("{access_key_id}/{scope}"),
    ));
    params.push(("X-Amz-Date".into(), timestamp.clone()));
    params.push(("X-Amz-Expires".into(), expires_str));
    if let Some(token) = security_token {
        params.push(("X-Amz-Security-Token".into(), token.into()));
    }

    // Headers that participate in signing, sorted by lowercase name per
    // SigV4: `host` is always signed; the extra headers the client will send
    // (content-type, SSE headers) are signed too so the upload is not
    // rejected with SignatureDoesNotMatch.
    let mut signed_header_entries: Vec<(String, String)> = extra_signed_headers
        .iter()
        .map(|(name, value)| (name.to_ascii_lowercase(), value.trim().to_owned()))
        .collect();
    signed_header_entries.push(("host".into(), host.clone()));
    signed_header_entries.sort_by(|left, right| left.0.cmp(&right.0));
    let canonical_headers = signed_header_entries
        .iter()
        .map(|(name, value)| format!("{name}:{value}\n"))
        .collect::<String>();
    let signed_headers = signed_header_entries
        .iter()
        .map(|(name, _)| name.as_str())
        .collect::<Vec<_>>()
        .join(";");
    params.push(("X-Amz-SignedHeaders".into(), signed_headers.clone()));
    params.sort_by(|a, b| a.0.cmp(&b.0));

    let canonical_query_string = params
        .iter()
        .map(|(k, v)| format!("{}={}", percent_encode(k), percent_encode(v)))
        .collect::<Vec<_>>()
        .join("&");

    let payload_hash = "UNSIGNED-PAYLOAD";

    let canonical_request = format!(
        "{method}\n{canonical_uri}\n{canonical_query_string}\n{canonical_headers}\n{signed_headers}\n{payload_hash}"
    );

    let string_to_sign = format!(
        "AWS4-HMAC-SHA256\n{timestamp}\n{scope}\n{}",
        sha256_hash(canonical_request.as_bytes())
    );

    // Signing key chain: k_date -> k_region -> k_service -> k_signing.
    let k_date = hmac_sha256(
        format!("AWS4{secret_access_key}").as_bytes(),
        date.as_bytes(),
    );
    let k_region = hmac_sha256(&k_date, region.as_bytes());
    let k_service = hmac_sha256(&k_region, b"s3");
    let k_signing = hmac_sha256(&k_service, b"aws4_request");
    let signature = hex_encode(&hmac_sha256(&k_signing, string_to_sign.as_bytes()));

    let base = endpoint.trim_end_matches('/');
    format!("{base}{canonical_uri}?{canonical_query_string}&X-Amz-Signature={signature}")
}

/// Path-style canonical URI: `/bucket/key` where each segment is percent-encoded
/// and `/` separators are preserved. Empty segments (e.g. a leading `/`) are
/// dropped so the path stays canonical.
fn build_canonical_uri(bucket: &str, object_key: &str) -> String {
    let mut parts: Vec<String> = Vec::new();
    if !bucket.is_empty() {
        parts.push(percent_encode(bucket));
    }
    for seg in object_key.split('/') {
        if !seg.is_empty() {
            parts.push(percent_encode(seg));
        }
    }
    format!("/{}", parts.join("/"))
}

/// Extract the host (optionally with port) from an endpoint URL by stripping
/// the `https://`/`http://` scheme and any trailing path.
fn extract_host(endpoint: &str) -> String {
    let stripped = endpoint
        .strip_prefix("https://")
        .or_else(|| endpoint.strip_prefix("http://"))
        .unwrap_or(endpoint);
    stripped
        .split('/')
        .next()
        .filter(|s| !s.is_empty())
        .unwrap_or(stripped)
        .to_string()
}

/// Percent-encode per RFC 3986: every byte except the unreserved set
/// (`A-Za-z0-9-._~`) is encoded as `%XX` with uppercase hex. Suitable for both
/// canonical URI segments and canonical query string keys/values.
fn percent_encode(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for &b in s.as_bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'.' | b'_' | b'~' => {
                out.push(b as char);
            }
            _ => out.push_str(&format!("%{b:02X}")),
        }
    }
    out
}

/// Format the current time as `(timestamp, date)` where timestamp is the
/// ISO 8601 basic form `YYYYMMDDTHHMMSSZ` and date is `YYYYMMDD` (UTC).
fn format_sigv4_timestamp(now: SystemTime) -> (String, String) {
    let secs = now
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let days = (secs / 86400) as i64;
    let rem = secs % 86400;
    let hour = (rem / 3600) as u32;
    let minute = ((rem % 3600) / 60) as u32;
    let second = (rem % 60) as u32;
    let (year, month, day) = civil_from_days(days);
    let timestamp = format!("{year:04}{month:02}{day:02}T{hour:02}{minute:02}{second:02}Z");
    let date = format!("{year:04}{month:02}{day:02}");
    (timestamp, date)
}

/// Convert a count of days since the Unix epoch (1970-01-01) to a proleptic
/// Gregorian `(year, month, day)` triple in UTC. Uses Howard Hinnant's
/// civil-from-days algorithm.
fn civil_from_days(z: i64) -> (i64, u32, u32) {
    let z = z + 719468;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let doe = z - era * 146097; // [0, 146096]
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365; // [0, 399]
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100); // [0, 365]
    let mp = (5 * doy + 2) / 153; // [0, 11]
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32; // [1, 31]
    let m = if mp < 10 { mp + 3 } else { mp - 9 } as u32; // [1, 12]
    (if m <= 2 { y + 1 } else { y }, m, d)
}

fn hmac_sha256(key: &[u8], msg: &[u8]) -> Vec<u8> {
    let mut mac = HmacSha256::new_from_slice(key).expect("HMAC-SHA256 accepts any key length");
    mac.update(msg);
    mac.finalize().into_bytes().to_vec()
}

/// Map a known file extension (lowercased, without the dot) to its canonical
/// MIME type. Returns `None` for unknown extensions (no cross-check applied).
fn expected_content_type_for_key(object_key: &str) -> Option<&'static str> {
    let ext = extension_of(object_key)?.to_ascii_lowercase();
    Some(match ext.as_str() {
        "jpg" | "jpeg" => "image/jpeg",
        "png" => "image/png",
        "gif" => "image/gif",
        "webp" => "image/webp",
        "svg" => "image/svg+xml",
        "mp4" => "video/mp4",
        "webm" => "video/webm",
        "mov" => "video/quicktime",
        "mp3" => "audio/mpeg",
        "m4a" => "audio/mp4",
        "ogg" => "audio/ogg",
        "pdf" => "application/pdf",
        "json" => "application/json",
        "txt" => "text/plain",
        "csv" => "text/csv",
        "md" | "markdown" => "text/markdown",
        _ => return None,
    })
}

/// Extract the file extension from an object key: the substring after the last
/// `.` only when no `/` follows it (so the dot is part of the filename, not a
/// path segment). Returns `None` when there is no extension.
fn extension_of(object_key: &str) -> Option<&str> {
    let last_dot = object_key.rfind('.')?;
    let after_dot = &object_key[last_dot + 1..];
    if after_dot.is_empty() || after_dot.contains('/') {
        return None;
    }
    Some(after_dot)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn civil_from_days_matches_known_dates() {
        // Day 0 is the Unix epoch.
        assert_eq!(civil_from_days(0), (1970, 1, 1));
        // 1970 is a non-leap year (365 days), so day 365 is 1971-01-01.
        assert_eq!(civil_from_days(365), (1971, 1, 1));
        // 1972 is a leap year: 730 days after epoch is 1972-01-01.
        assert_eq!(civil_from_days(730), (1972, 1, 1));
    }

    #[test]
    fn percent_encode_preserves_unreserved_and_encodes_rest() {
        assert_eq!(percent_encode("abcXYZ09-._~"), "abcXYZ09-._~");
        assert_eq!(percent_encode("/"), "%2F");
        assert_eq!(percent_encode(":"), "%3A");
        assert_eq!(percent_encode(" "), "%20");
    }

    #[test]
    fn extract_host_strips_scheme_and_path() {
        assert_eq!(
            extract_host("https://tos.volcengine.local"),
            "tos.volcengine.local"
        );
        assert_eq!(
            extract_host("http://s3.example.com:9000/path"),
            "s3.example.com:9000"
        );
        assert_eq!(extract_host("example.com"), "example.com");
    }

    #[test]
    fn extension_of_handles_paths_and_trailing_dot() {
        assert_eq!(extension_of("tenant/100001/demo.mp4"), Some("mp4"));
        assert_eq!(extension_of("image.png"), Some("png"));
        assert_eq!(extension_of("tenant/v2.0/demo"), None);
        assert_eq!(extension_of("noext"), None);
        assert_eq!(extension_of("trailing."), None);
    }

    #[test]
    fn validate_content_type_accepts_allowed_and_none() {
        assert!(
            S3CompatibleObjectStorageProvider::validate_content_type(&None, "demo.mp4").is_ok()
        );
        assert!(
            S3CompatibleObjectStorageProvider::validate_content_type(
                &Some("video/mp4".into()),
                "demo.mp4"
            )
            .is_ok()
        );
        assert!(
            S3CompatibleObjectStorageProvider::validate_content_type(
                &Some("application/octet-stream".into()),
                "blob.bin"
            )
            .is_ok()
        );
    }

    #[test]
    fn put_object_fails_closed_with_unsupported_capability() {
        // Server-side object PUT is not implemented; it must fail closed with
        // UnsupportedCapability instead of fabricating a descriptor/ETag.
        let provider = S3CompatibleObjectStorageProvider::aws_default();
        let error = provider
            .put_object(ObjectStoragePutRequest {
                bucket: "media".into(),
                object_key: "rtc/demo.mp4".into(),
                content_length: 128,
                content_type: Some("video/mp4".into()),
                storage_class: Some("standard".into()),
            })
            .expect_err("put_object must be unsupported");
        assert!(
            matches!(error, ContractError::UnsupportedCapability(message) if message.contains("signed_upload_url"))
        );
    }

    #[test]
    fn descriptor_does_not_declare_multipart_without_implementation() {
        let provider = S3CompatibleObjectStorageProvider::aws_default();
        let descriptor = provider.descriptor();
        assert_eq!(descriptor.required_capabilities, vec!["s3", "presign"]);
        assert!(
            !descriptor
                .optional_capabilities
                .iter()
                .any(|capability| capability == "multipart"),
            "multipart must not be declared without an implementation"
        );
    }

    #[test]
    fn signed_upload_url_is_signed_with_put_method() {
        // Regression: the canonical request used to be hardcoded to GET while
        // the session declared method PUT, so every production upload would be
        // rejected by S3 with SignatureDoesNotMatch.
        let provider =
            S3CompatibleObjectStorageProvider::new(S3CompatibleObjectStorageProviderConfig {
                plugin_id: "object-storage-aws".into(),
                provider_kind: "aws".into(),
                display_name: "AWS".into(),
                endpoint: "https://s3.example.test".into(),
                region: "us-east-1".into(),
                gateway_mode: false,
                access_key_id: Some("AKIDEXAMPLE".into()),
                secret_access_key: Some("SECRETKEY".into()),
                security_token: None,
                kms_key_id: None,
            });
        let session = provider
            .signed_upload_url(ObjectStorageUploadUrlRequest {
                bucket: "media".into(),
                object_key: "demo.mp4".into(),
                expires_in_seconds: 900,
                content_type: Some("video/mp4".into()),
                content_length: None,
            })
            .expect("signed upload url must be generated");
        assert_eq!(session.method, "PUT");
        assert!(
            session
                .headers
                .keys()
                .any(|name| name.eq_ignore_ascii_case("x-amz-server-side-encryption")),
            "upload session must carry SSE headers the client will send"
        );

        let params = parse_signed_url_query(&session.url);
        // The headers the client sends with the PUT (content-type plus the
        // x-amz-server-side-encryption headers) must participate in the
        // signature, otherwise S3 rejects the upload with SignatureDoesNotMatch.
        let signed_headers = &params["X-Amz-SignedHeaders"];
        assert!(
            signed_headers.split(';').any(|name| name == "content-type"),
            "PUT signing must include content-type in the signed headers: {signed_headers}"
        );
        assert!(
            signed_headers
                .split(';')
                .any(|name| name == "x-amz-server-side-encryption"),
            "PUT signing must include SSE headers in the signed headers: {signed_headers}"
        );

        // Recompute the signature from the URL's visible inputs: the PUT
        // canonical request must produce the URL's X-Amz-Signature and a GET
        // signing must not.
        let recomputed_put =
            recompute_sigv4_signature(&session.url, &params, "PUT", &session.headers);
        let recomputed_get =
            recompute_sigv4_signature(&session.url, &params, "GET", &session.headers);
        assert_eq!(
            recomputed_put, params["X-Amz-Signature"],
            "upload URL signature must be computed with the PUT method"
        );
        assert_ne!(
            recomputed_get, params["X-Amz-Signature"],
            "a GET-signed URL must not validate the PUT upload"
        );
    }

    #[test]
    fn signed_download_url_keeps_get_signing() {
        let provider =
            S3CompatibleObjectStorageProvider::new(S3CompatibleObjectStorageProviderConfig {
                plugin_id: "object-storage-aws".into(),
                provider_kind: "aws".into(),
                display_name: "AWS".into(),
                endpoint: "https://s3.example.test".into(),
                region: "us-east-1".into(),
                gateway_mode: false,
                access_key_id: Some("AKIDEXAMPLE".into()),
                secret_access_key: Some("SECRETKEY".into()),
                security_token: None,
                kms_key_id: None,
            });
        let url = provider
            .signed_download_url(ObjectStorageDownloadUrlRequest {
                bucket: "media".into(),
                object_key: "demo.mp4".into(),
                expires_in_seconds: 900,
            })
            .expect("signed download url must be generated");
        let params = parse_signed_url_query(&url);
        assert_eq!(
            params["X-Amz-SignedHeaders"], "host",
            "GET signing must keep the minimal host-only signed headers"
        );
        let recomputed_get = recompute_sigv4_signature(&url, &params, "GET", &BTreeMap::new());
        assert_eq!(
            recomputed_get, params["X-Amz-Signature"],
            "download URL signature must be computed with the GET method"
        );
    }

    /// Parse the query string of a presigned URL into decoded key/value pairs.
    fn parse_signed_url_query(url: &str) -> BTreeMap<String, String> {
        url.split('?')
            .nth(1)
            .expect("signed url must carry a query string")
            .split('&')
            .map(|pair| {
                let (key, value) = pair.split_once('=').expect("query pair must be key=value");
                (
                    percent_decode(key).to_owned(),
                    percent_decode(value).to_owned(),
                )
            })
            .collect()
    }

    /// Independently recompute the SigV4 signature for `method` from inputs
    /// visible in the signed URL plus the headers the client will send, and
    /// compare against the URL's `X-Amz-Signature`.
    fn recompute_sigv4_signature(
        url: &str,
        params: &BTreeMap<String, String>,
        method: &str,
        extra_headers: &BTreeMap<String, String>,
    ) -> String {
        let timestamp = &params["X-Amz-Date"];
        let credential = &params["X-Amz-Credential"];
        let scope = credential
            .splitn(2, '/')
            .nth(1)
            .expect("credential must contain the scope")
            .to_owned();
        let date = &timestamp[..8];
        let region = scope
            .split('/')
            .nth(1)
            .expect("scope must contain the region");

        let mut query_entries = params
            .iter()
            .filter(|(key, _)| key.as_str() != "X-Amz-Signature")
            .map(|(key, value)| (key.clone(), value.clone()))
            .collect::<Vec<_>>();
        query_entries.sort_by(|left, right| left.0.cmp(&right.0));
        let canonical_query_string = query_entries
            .iter()
            .map(|(key, value)| format!("{}={}", percent_encode(key), percent_encode(value)))
            .collect::<Vec<_>>()
            .join("&");

        let mut header_entries = extra_headers
            .iter()
            .map(|(name, value)| (name.to_ascii_lowercase(), value.trim().to_owned()))
            .collect::<Vec<_>>();
        header_entries.push(("host".into(), extract_host(url)));
        header_entries.sort_by(|left, right| left.0.cmp(&right.0));
        let canonical_headers = header_entries
            .iter()
            .map(|(name, value)| format!("{name}:{value}\n"))
            .collect::<String>();
        let signed_headers = header_entries
            .iter()
            .map(|(name, _)| name.as_str())
            .collect::<Vec<_>>()
            .join(";");

        let canonical_uri = canonical_uri_from_url(url);
        let canonical_request = format!(
            "{method}\n{canonical_uri}\n{canonical_query_string}\n{canonical_headers}\n{signed_headers}\nUNSIGNED-PAYLOAD"
        );
        let string_to_sign = format!(
            "AWS4-HMAC-SHA256\n{timestamp}\n{scope}\n{}",
            sha256_hash(canonical_request.as_bytes())
        );
        let k_date = hmac_sha256(format!("AWS4SECRETKEY").as_bytes(), date.as_bytes());
        let k_region = hmac_sha256(&k_date, region.as_bytes());
        let k_service = hmac_sha256(&k_region, b"s3");
        let k_signing = hmac_sha256(&k_service, b"aws4_request");
        hex_encode(&hmac_sha256(&k_signing, string_to_sign.as_bytes()))
    }

    /// Extract the canonical URI (`/bucket/key`, already percent-encoded)
    /// from a signed URL.
    fn canonical_uri_from_url(url: &str) -> String {
        let path_and_query = url.split('?').next().expect("signed url must have a path");
        let after_scheme = path_and_query
            .splitn(2, "://")
            .nth(1)
            .unwrap_or(path_and_query);
        let path_start = after_scheme.find('/').unwrap_or(after_scheme.len());
        let path = &after_scheme[path_start..];
        if path.is_empty() {
            "/".to_owned()
        } else {
            path.to_owned()
        }
    }

    fn percent_decode(value: &str) -> String {
        let bytes = value.as_bytes();
        let mut out = Vec::with_capacity(bytes.len());
        let mut index = 0;
        while index < bytes.len() {
            if bytes[index] == b'%' && index + 2 < bytes.len() {
                let hi = (bytes[index + 1] as char).to_digit(16);
                let lo = (bytes[index + 2] as char).to_digit(16);
                if let (Some(hi), Some(lo)) = (hi, lo) {
                    out.push((hi * 16 + lo) as u8);
                    index += 3;
                    continue;
                }
            }
            out.push(bytes[index]);
            index += 1;
        }
        String::from_utf8(out).expect("signed url query must be valid utf-8")
    }
}
