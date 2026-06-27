use std::collections::BTreeMap;

use im_platform_contracts::{
    ObjectStorageDownloadUrlRequest, ObjectStorageObjectDescriptor, ObjectStorageProvider,
    ObjectStoragePutRequest, ObjectStorageUploadSession, ObjectStorageUploadUrlRequest,
    ProviderDomain, ProviderHealthSnapshot, ProviderPluginDescriptor,
};
use im_time::{format_unix_timestamp_millis, utc_now_rfc3339_millis};
use sdkwork_im_contract_core::ContractError;

pub const ALIYUN_OBJECT_STORAGE_PLUGIN_ID: &str = "object-storage-aliyun";
pub const TENCENT_OBJECT_STORAGE_PLUGIN_ID: &str = "object-storage-tencent";
pub const VOLCENGINE_OBJECT_STORAGE_PLUGIN_ID: &str = "object-storage-volcengine";
pub const AWS_OBJECT_STORAGE_PLUGIN_ID: &str = "object-storage-aws";
pub const GOOGLE_OBJECT_STORAGE_PLUGIN_ID: &str = "object-storage-google";
pub const MICROSOFT_OBJECT_STORAGE_PLUGIN_ID: &str = "object-storage-microsoft";

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct S3CompatibleObjectStorageProviderConfig {
    pub plugin_id: String,
    pub provider_kind: String,
    pub display_name: String,
    pub endpoint: String,
    pub region: String,
    pub gateway_mode: bool,
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
        Self::new(S3CompatibleObjectStorageProviderConfig {
            plugin_id: ALIYUN_OBJECT_STORAGE_PLUGIN_ID.into(),
            provider_kind: "aliyun".into(),
            display_name: "Aliyun Object Storage".into(),
            endpoint: "https://oss.aliyun.local".into(),
            region: "cn-hangzhou".into(),
            gateway_mode: false,
        })
    }

    pub fn tencent_default() -> Self {
        Self::new(S3CompatibleObjectStorageProviderConfig {
            plugin_id: TENCENT_OBJECT_STORAGE_PLUGIN_ID.into(),
            provider_kind: "tencent".into(),
            display_name: "Tencent Cloud Object Storage".into(),
            endpoint: "https://cos.tencent.local".into(),
            region: "ap-guangzhou".into(),
            gateway_mode: false,
        })
    }

    pub fn volcengine_default() -> Self {
        Self::new(S3CompatibleObjectStorageProviderConfig {
            plugin_id: VOLCENGINE_OBJECT_STORAGE_PLUGIN_ID.into(),
            provider_kind: "volcengine".into(),
            display_name: "Volcengine Object Storage".into(),
            endpoint: "https://tos.volcengine.local".into(),
            region: "cn-beijing".into(),
            gateway_mode: false,
        })
    }

    pub fn aws_default() -> Self {
        Self::new(S3CompatibleObjectStorageProviderConfig {
            plugin_id: AWS_OBJECT_STORAGE_PLUGIN_ID.into(),
            provider_kind: "aws".into(),
            display_name: "Amazon S3".into(),
            endpoint: "https://s3.aws.local".into(),
            region: "us-east-1".into(),
            gateway_mode: false,
        })
    }

    pub fn google_default() -> Self {
        Self::new(S3CompatibleObjectStorageProviderConfig {
            plugin_id: GOOGLE_OBJECT_STORAGE_PLUGIN_ID.into(),
            provider_kind: "google".into(),
            display_name: "Google Cloud Storage S3 Gateway".into(),
            endpoint: "https://storage.googleapis.local".into(),
            region: "us-central1".into(),
            gateway_mode: true,
        })
    }

    pub fn microsoft_default() -> Self {
        Self::new(S3CompatibleObjectStorageProviderConfig {
            plugin_id: MICROSOFT_OBJECT_STORAGE_PLUGIN_ID.into(),
            provider_kind: "microsoft".into(),
            display_name: "Azure Blob S3 Gateway".into(),
            endpoint: "https://blob.azure.local".into(),
            region: "eastasia".into(),
            gateway_mode: true,
        })
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
                .with_optional_capabilities(["multipart"])
        } else {
            descriptor
                .with_required_capabilities(["s3", "presign", "multipart"])
                .with_optional_capabilities(["retention"])
        }
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
        Ok(ObjectStorageObjectDescriptor {
            bucket: request.bucket,
            object_key: request.object_key.clone(),
            content_length: request.content_length,
            etag: Some(format!(
                "{}:{}:{}",
                self.config.provider_kind, request.object_key, request.content_length
            )),
        })
    }

    fn signed_upload_url(
        &self,
        request: ObjectStorageUploadUrlRequest,
    ) -> Result<ObjectStorageUploadSession, ContractError> {
        let mut headers = BTreeMap::new();
        if let Some(content_type) = request.content_type.as_ref() {
            headers.insert("content-type".into(), content_type.clone());
        }

        let expires_at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis()
            + (request.expires_in_seconds as u128 * 1_000);

        Ok(ObjectStorageUploadSession {
            method: "PUT".into(),
            url: format!(
                "{}/{}/{}?provider={}&expires={}&upload=put",
                self.config.endpoint.trim_end_matches('/'),
                request.bucket,
                request.object_key,
                self.config.plugin_id,
                request.expires_in_seconds
            ),
            headers,
            expires_at: format_unix_timestamp_millis(expires_at),
        })
    }

    fn signed_download_url(
        &self,
        request: ObjectStorageDownloadUrlRequest,
    ) -> Result<String, ContractError> {
        Ok(format!(
            "{}/{}/{}?provider={}&expires={}",
            self.config.endpoint.trim_end_matches('/'),
            request.bucket,
            request.object_key,
            self.config.plugin_id,
            request.expires_in_seconds
        ))
    }

    fn provider_health_snapshot(&self) -> ProviderHealthSnapshot {
        let mut details = BTreeMap::new();
        details.insert("providerKind".into(), self.config.provider_kind.clone());
        details.insert("endpoint".into(), self.config.endpoint.clone());
        details.insert("region".into(), self.config.region.clone());
        details.insert("gatewayMode".into(), self.config.gateway_mode.to_string());
        ProviderHealthSnapshot {
            plugin_id: self.config.plugin_id.clone(),
            status: "healthy".into(),
            checked_at: utc_now_rfc3339_millis(),
            details,
        }
    }
}
