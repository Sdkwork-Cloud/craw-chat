use std::sync::Arc;

use crate::api::paths::backend_path;
use crate::api::paths::append_query_string;
use crate::http::{SdkworkError, SdkworkHttpClient};
use crate::models::{SdkWorkPageData};

#[derive(Clone)]
pub struct AdminApi {
    client: Arc<SdkworkHttpClient>,
}

impl AdminApi {
    pub fn new(client: Arc<SdkworkHttpClient>) -> Self {
        Self { client }
    }

    /// listApiKeyGroups
    pub async fn api_key_groups_list(&self, page_size: Option<i64>, cursor: Option<&str>, page: Option<i64>, q: Option<&str>) -> Result<SdkWorkPageData, SdkworkError> {
        let query = build_query_string(&[
            QueryParameterSpec::new("page_size", page_size, "form", true, false, None),
            QueryParameterSpec::new("cursor", cursor, "form", true, false, None),
            QueryParameterSpec::new("page", page, "form", true, false, None),
            QueryParameterSpec::new("q", q, "form", true, false, None),
        ]);
        let path = append_query_string(backend_path(&"/admin/api_key_groups".to_string()), &query);
        self.client.get(&path, None, None).await
    }

    /// createApiKeyGroup
    pub async fn api_key_groups_create(&self, body: &std::collections::HashMap<String, serde_json::Value>) -> Result<serde_json::Value, SdkworkError> {
        let path = backend_path(&"/admin/api_key_groups".to_string());
        self.client.post(&path, Some(body), None, None, Some("application/json")).await
    }

    /// updateApiKeyGroup
    pub async fn api_key_groups_update(&self, group_id: &str, body: &std::collections::HashMap<String, serde_json::Value>) -> Result<serde_json::Value, SdkworkError> {
        let path = backend_path(&format!("/admin/api_key_groups/{}", serialize_path_parameter(group_id, PathParameterSpec::new("groupId", "simple", false))));
        self.client.patch(&path, Some(body), None, None, Some("application/json")).await
    }

    /// deleteApiKeyGroup
    pub async fn api_key_groups_delete(&self, group_id: &str) -> Result<(), SdkworkError> {
        let path = backend_path(&format!("/admin/api_key_groups/{}", serialize_path_parameter(group_id, PathParameterSpec::new("groupId", "simple", false))));
        self.client.delete(&path, None, None).await
    }

    /// updateApiKeyGroupStatus
    pub async fn api_key_groups_status(&self, group_id: &str, body: &std::collections::HashMap<String, serde_json::Value>) -> Result<serde_json::Value, SdkworkError> {
        let path = backend_path(&format!("/admin/api_key_groups/{}/status", serialize_path_parameter(group_id, PathParameterSpec::new("groupId", "simple", false))));
        self.client.post(&path, Some(body), None, None, Some("application/json")).await
    }

    /// listApiKeys
    pub async fn api_keys_list(&self, page_size: Option<i64>, cursor: Option<&str>, page: Option<i64>, q: Option<&str>) -> Result<SdkWorkPageData, SdkworkError> {
        let query = build_query_string(&[
            QueryParameterSpec::new("page_size", page_size, "form", true, false, None),
            QueryParameterSpec::new("cursor", cursor, "form", true, false, None),
            QueryParameterSpec::new("page", page, "form", true, false, None),
            QueryParameterSpec::new("q", q, "form", true, false, None),
        ]);
        let path = append_query_string(backend_path(&"/admin/api_keys".to_string()), &query);
        self.client.get(&path, None, None).await
    }

    /// createApiKey
    pub async fn api_keys_create(&self, body: &std::collections::HashMap<String, serde_json::Value>) -> Result<serde_json::Value, SdkworkError> {
        let path = backend_path(&"/admin/api_keys".to_string());
        self.client.post(&path, Some(body), None, None, Some("application/json")).await
    }

    /// updateApiKey
    pub async fn api_keys_update(&self, hashed_key: &str, body: &std::collections::HashMap<String, serde_json::Value>) -> Result<serde_json::Value, SdkworkError> {
        let path = backend_path(&format!("/admin/api_keys/{}", serialize_path_parameter(hashed_key, PathParameterSpec::new("hashedKey", "simple", false))));
        self.client.put(&path, Some(body), None, None, Some("application/json")).await
    }

    /// deleteApiKey
    pub async fn api_keys_delete(&self, hashed_key: &str) -> Result<(), SdkworkError> {
        let path = backend_path(&format!("/admin/api_keys/{}", serialize_path_parameter(hashed_key, PathParameterSpec::new("hashedKey", "simple", false))));
        self.client.delete(&path, None, None).await
    }

    /// updateApiKeyStatus
    pub async fn api_keys_status(&self, hashed_key: &str, body: &std::collections::HashMap<String, serde_json::Value>) -> Result<serde_json::Value, SdkworkError> {
        let path = backend_path(&format!("/admin/api_keys/{}/status", serialize_path_parameter(hashed_key, PathParameterSpec::new("hashedKey", "simple", false))));
        self.client.post(&path, Some(body), None, None, Some("application/json")).await
    }

    /// listBillingEvents
    pub async fn billing_events_list(&self, page_size: Option<i64>, cursor: Option<&str>, page: Option<i64>, q: Option<&str>) -> Result<SdkWorkPageData, SdkworkError> {
        let query = build_query_string(&[
            QueryParameterSpec::new("page_size", page_size, "form", true, false, None),
            QueryParameterSpec::new("cursor", cursor, "form", true, false, None),
            QueryParameterSpec::new("page", page, "form", true, false, None),
            QueryParameterSpec::new("q", q, "form", true, false, None),
        ]);
        let path = append_query_string(backend_path(&"/admin/billing/events".to_string()), &query);
        self.client.get(&path, None, None).await
    }

    /// getBillingEventSummary
    pub async fn billing_events_summary_retrieve(&self) -> Result<serde_json::Value, SdkworkError> {
        let path = backend_path(&"/admin/billing/events/summary".to_string());
        self.client.get(&path, None, None).await
    }

    /// getBillingSummary
    pub async fn billing_summary_retrieve(&self) -> Result<serde_json::Value, SdkworkError> {
        let path = backend_path(&"/admin/billing/summary".to_string());
        self.client.get(&path, None, None).await
    }

    /// listChannelModels
    pub async fn channel_models_list(&self, page_size: Option<i64>, cursor: Option<&str>, page: Option<i64>, q: Option<&str>) -> Result<SdkWorkPageData, SdkworkError> {
        let query = build_query_string(&[
            QueryParameterSpec::new("page_size", page_size, "form", true, false, None),
            QueryParameterSpec::new("cursor", cursor, "form", true, false, None),
            QueryParameterSpec::new("page", page, "form", true, false, None),
            QueryParameterSpec::new("q", q, "form", true, false, None),
        ]);
        let path = append_query_string(backend_path(&"/admin/channel_models".to_string()), &query);
        self.client.get(&path, None, None).await
    }

    /// saveChannelModel
    pub async fn channel_models_create(&self, body: &std::collections::HashMap<String, serde_json::Value>) -> Result<serde_json::Value, SdkworkError> {
        let path = backend_path(&"/admin/channel_models".to_string());
        self.client.post(&path, Some(body), None, None, Some("application/json")).await
    }

    /// deleteChannelModel
    pub async fn channel_models_delete(&self, channel_id: &str, model_id: &str) -> Result<(), SdkworkError> {
        let path = backend_path(&format!("/admin/channel_models/{}/models/{}", serialize_path_parameter(channel_id, PathParameterSpec::new("channelId", "simple", false)), serialize_path_parameter(model_id, PathParameterSpec::new("modelId", "simple", false))));
        self.client.delete(&path, None, None).await
    }

    /// listChannels
    pub async fn channels_list(&self, page_size: Option<i64>, cursor: Option<&str>, page: Option<i64>, q: Option<&str>) -> Result<SdkWorkPageData, SdkworkError> {
        let query = build_query_string(&[
            QueryParameterSpec::new("page_size", page_size, "form", true, false, None),
            QueryParameterSpec::new("cursor", cursor, "form", true, false, None),
            QueryParameterSpec::new("page", page, "form", true, false, None),
            QueryParameterSpec::new("q", q, "form", true, false, None),
        ]);
        let path = append_query_string(backend_path(&"/admin/channels".to_string()), &query);
        self.client.get(&path, None, None).await
    }

    /// saveChannel
    pub async fn channels_create(&self, body: &std::collections::HashMap<String, serde_json::Value>) -> Result<serde_json::Value, SdkworkError> {
        let path = backend_path(&"/admin/channels".to_string());
        self.client.post(&path, Some(body), None, None, Some("application/json")).await
    }

    /// deleteChannel
    pub async fn channels_delete(&self, channel_id: &str) -> Result<(), SdkworkError> {
        let path = backend_path(&format!("/admin/channels/{}", serialize_path_parameter(channel_id, PathParameterSpec::new("channelId", "simple", false))));
        self.client.delete(&path, None, None).await
    }

    /// listCredentials
    pub async fn credentials_list(&self, page_size: Option<i64>, cursor: Option<&str>, page: Option<i64>, q: Option<&str>) -> Result<SdkWorkPageData, SdkworkError> {
        let query = build_query_string(&[
            QueryParameterSpec::new("page_size", page_size, "form", true, false, None),
            QueryParameterSpec::new("cursor", cursor, "form", true, false, None),
            QueryParameterSpec::new("page", page, "form", true, false, None),
            QueryParameterSpec::new("q", q, "form", true, false, None),
        ]);
        let path = append_query_string(backend_path(&"/admin/credentials".to_string()), &query);
        self.client.get(&path, None, None).await
    }

    /// saveCredential
    pub async fn credentials_create(&self, body: &std::collections::HashMap<String, serde_json::Value>) -> Result<serde_json::Value, SdkworkError> {
        let path = backend_path(&"/admin/credentials".to_string());
        self.client.post(&path, Some(body), None, None, Some("application/json")).await
    }

    /// deleteCredential
    pub async fn credentials_providers_keys_delete(&self, tenant_id: &str, provider_id: &str, key_reference: &str) -> Result<(), SdkworkError> {
        let path = backend_path(&format!("/admin/credentials/{}/providers/{}/keys/{}", serialize_path_parameter(tenant_id, PathParameterSpec::new("tenantId", "simple", false)), serialize_path_parameter(provider_id, PathParameterSpec::new("providerId", "simple", false)), serialize_path_parameter(key_reference, PathParameterSpec::new("keyReference", "simple", false))));
        self.client.delete(&path, None, None).await
    }

    /// reloadExtensionRuntimes
    pub async fn extensions_runtime_reloads_create(&self, body: &std::collections::HashMap<String, serde_json::Value>) -> Result<serde_json::Value, SdkworkError> {
        let path = backend_path(&"/admin/extensions/runtime_reloads".to_string());
        self.client.post(&path, Some(body), None, None, Some("application/json")).await
    }

    /// listRuntimeStatuses
    pub async fn extensions_runtime_statuses_list(&self, page_size: Option<i64>, cursor: Option<&str>, page: Option<i64>, q: Option<&str>) -> Result<SdkWorkPageData, SdkworkError> {
        let query = build_query_string(&[
            QueryParameterSpec::new("page_size", page_size, "form", true, false, None),
            QueryParameterSpec::new("cursor", cursor, "form", true, false, None),
            QueryParameterSpec::new("page", page, "form", true, false, None),
            QueryParameterSpec::new("q", q, "form", true, false, None),
        ]);
        let path = append_query_string(backend_path(&"/admin/extensions/runtime_statuses".to_string()), &query);
        self.client.get(&path, None, None).await
    }

    /// listRateLimitPolicies
    pub async fn gateway_rate_limit_policies_list(&self, page_size: Option<i64>, cursor: Option<&str>, page: Option<i64>, q: Option<&str>) -> Result<SdkWorkPageData, SdkworkError> {
        let query = build_query_string(&[
            QueryParameterSpec::new("page_size", page_size, "form", true, false, None),
            QueryParameterSpec::new("cursor", cursor, "form", true, false, None),
            QueryParameterSpec::new("page", page, "form", true, false, None),
            QueryParameterSpec::new("q", q, "form", true, false, None),
        ]);
        let path = append_query_string(backend_path(&"/admin/gateway/rate_limit_policies".to_string()), &query);
        self.client.get(&path, None, None).await
    }

    /// createRateLimitPolicy
    pub async fn gateway_rate_limit_policies_create(&self, body: &std::collections::HashMap<String, serde_json::Value>) -> Result<serde_json::Value, SdkworkError> {
        let path = backend_path(&"/admin/gateway/rate_limit_policies".to_string());
        self.client.post(&path, Some(body), None, None, Some("application/json")).await
    }

    /// listRateLimitWindows
    pub async fn gateway_rate_limit_windows_list(&self, page_size: Option<i64>, cursor: Option<&str>, page: Option<i64>, q: Option<&str>) -> Result<SdkWorkPageData, SdkworkError> {
        let query = build_query_string(&[
            QueryParameterSpec::new("page_size", page_size, "form", true, false, None),
            QueryParameterSpec::new("cursor", cursor, "form", true, false, None),
            QueryParameterSpec::new("page", page, "form", true, false, None),
            QueryParameterSpec::new("q", q, "form", true, false, None),
        ]);
        let path = append_query_string(backend_path(&"/admin/gateway/rate_limit_windows".to_string()), &query);
        self.client.get(&path, None, None).await
    }

    /// listMarketingCampaigns
    pub async fn marketing_campaigns_list(&self, page_size: Option<i64>, cursor: Option<&str>, page: Option<i64>, q: Option<&str>) -> Result<SdkWorkPageData, SdkworkError> {
        let query = build_query_string(&[
            QueryParameterSpec::new("page_size", page_size, "form", true, false, None),
            QueryParameterSpec::new("cursor", cursor, "form", true, false, None),
            QueryParameterSpec::new("page", page, "form", true, false, None),
            QueryParameterSpec::new("q", q, "form", true, false, None),
        ]);
        let path = append_query_string(backend_path(&"/admin/marketing/campaigns".to_string()), &query);
        self.client.get(&path, None, None).await
    }

    /// saveMarketingCampaign
    pub async fn marketing_campaigns_create(&self, body: &std::collections::HashMap<String, serde_json::Value>) -> Result<serde_json::Value, SdkworkError> {
        let path = backend_path(&"/admin/marketing/campaigns".to_string());
        self.client.post(&path, Some(body), None, None, Some("application/json")).await
    }

    /// updateMarketingCampaignStatus
    pub async fn marketing_campaigns_status(&self, marketing_campaign_id: &str, body: &std::collections::HashMap<String, serde_json::Value>) -> Result<serde_json::Value, SdkworkError> {
        let path = backend_path(&format!("/admin/marketing/campaigns/{}/status", serialize_path_parameter(marketing_campaign_id, PathParameterSpec::new("marketingCampaignId", "simple", false))));
        self.client.post(&path, Some(body), None, None, Some("application/json")).await
    }

    /// listModelPrices
    pub async fn model_prices_list(&self, page_size: Option<i64>, cursor: Option<&str>, page: Option<i64>, q: Option<&str>) -> Result<SdkWorkPageData, SdkworkError> {
        let query = build_query_string(&[
            QueryParameterSpec::new("page_size", page_size, "form", true, false, None),
            QueryParameterSpec::new("cursor", cursor, "form", true, false, None),
            QueryParameterSpec::new("page", page, "form", true, false, None),
            QueryParameterSpec::new("q", q, "form", true, false, None),
        ]);
        let path = append_query_string(backend_path(&"/admin/model_prices".to_string()), &query);
        self.client.get(&path, None, None).await
    }

    /// saveModelPrice
    pub async fn model_prices_create(&self, body: &std::collections::HashMap<String, serde_json::Value>) -> Result<serde_json::Value, SdkworkError> {
        let path = backend_path(&"/admin/model_prices".to_string());
        self.client.post(&path, Some(body), None, None, Some("application/json")).await
    }

    /// deleteModelPrice
    pub async fn model_prices_providers_delete(&self, channel_id: &str, model_id: &str, proxy_provider_id: &str) -> Result<(), SdkworkError> {
        let path = backend_path(&format!("/admin/model_prices/{}/models/{}/providers/{}", serialize_path_parameter(channel_id, PathParameterSpec::new("channelId", "simple", false)), serialize_path_parameter(model_id, PathParameterSpec::new("modelId", "simple", false)), serialize_path_parameter(proxy_provider_id, PathParameterSpec::new("proxyProviderId", "simple", false))));
        self.client.delete(&path, None, None).await
    }

    /// listModels
    pub async fn models_list(&self, page_size: Option<i64>, cursor: Option<&str>, page: Option<i64>, q: Option<&str>) -> Result<SdkWorkPageData, SdkworkError> {
        let query = build_query_string(&[
            QueryParameterSpec::new("page_size", page_size, "form", true, false, None),
            QueryParameterSpec::new("cursor", cursor, "form", true, false, None),
            QueryParameterSpec::new("page", page, "form", true, false, None),
            QueryParameterSpec::new("q", q, "form", true, false, None),
        ]);
        let path = append_query_string(backend_path(&"/admin/models".to_string()), &query);
        self.client.get(&path, None, None).await
    }

    /// saveModel
    pub async fn models_create(&self, body: &std::collections::HashMap<String, serde_json::Value>) -> Result<serde_json::Value, SdkworkError> {
        let path = backend_path(&"/admin/models".to_string());
        self.client.post(&path, Some(body), None, None, Some("application/json")).await
    }

    /// deleteModel
    pub async fn models_providers_delete(&self, external_name: &str, provider_id: &str) -> Result<(), SdkworkError> {
        let path = backend_path(&format!("/admin/models/{}/providers/{}", serialize_path_parameter(external_name, PathParameterSpec::new("externalName", "simple", false)), serialize_path_parameter(provider_id, PathParameterSpec::new("providerId", "simple", false))));
        self.client.delete(&path, None, None).await
    }

    /// listProviders
    pub async fn providers_list(&self, page_size: Option<i64>, cursor: Option<&str>, page: Option<i64>, q: Option<&str>) -> Result<SdkWorkPageData, SdkworkError> {
        let query = build_query_string(&[
            QueryParameterSpec::new("page_size", page_size, "form", true, false, None),
            QueryParameterSpec::new("cursor", cursor, "form", true, false, None),
            QueryParameterSpec::new("page", page, "form", true, false, None),
            QueryParameterSpec::new("q", q, "form", true, false, None),
        ]);
        let path = append_query_string(backend_path(&"/admin/providers".to_string()), &query);
        self.client.get(&path, None, None).await
    }

    /// saveProvider
    pub async fn providers_create(&self, body: &std::collections::HashMap<String, serde_json::Value>) -> Result<serde_json::Value, SdkworkError> {
        let path = backend_path(&"/admin/providers".to_string());
        self.client.post(&path, Some(body), None, None, Some("application/json")).await
    }

    /// deleteProvider
    pub async fn providers_delete(&self, provider_id: &str) -> Result<(), SdkworkError> {
        let path = backend_path(&format!("/admin/providers/{}", serialize_path_parameter(provider_id, PathParameterSpec::new("providerId", "simple", false))));
        self.client.delete(&path, None, None).await
    }

    /// listRoutingDecisionLogs
    pub async fn routing_decision_logs_list(&self, page_size: Option<i64>, cursor: Option<&str>, page: Option<i64>, q: Option<&str>) -> Result<SdkWorkPageData, SdkworkError> {
        let query = build_query_string(&[
            QueryParameterSpec::new("page_size", page_size, "form", true, false, None),
            QueryParameterSpec::new("cursor", cursor, "form", true, false, None),
            QueryParameterSpec::new("page", page, "form", true, false, None),
            QueryParameterSpec::new("q", q, "form", true, false, None),
        ]);
        let path = append_query_string(backend_path(&"/admin/routing/decision_logs".to_string()), &query);
        self.client.get(&path, None, None).await
    }

    /// listProviderHealthSnapshots
    pub async fn routing_health_snapshots_retrieve(&self) -> Result<serde_json::Value, SdkworkError> {
        let path = backend_path(&"/admin/routing/health_snapshots".to_string());
        self.client.get(&path, None, None).await
    }

    /// listRoutingProfiles
    pub async fn routing_profiles_list(&self, page_size: Option<i64>, cursor: Option<&str>, page: Option<i64>, q: Option<&str>) -> Result<SdkWorkPageData, SdkworkError> {
        let query = build_query_string(&[
            QueryParameterSpec::new("page_size", page_size, "form", true, false, None),
            QueryParameterSpec::new("cursor", cursor, "form", true, false, None),
            QueryParameterSpec::new("page", page, "form", true, false, None),
            QueryParameterSpec::new("q", q, "form", true, false, None),
        ]);
        let path = append_query_string(backend_path(&"/admin/routing/profiles".to_string()), &query);
        self.client.get(&path, None, None).await
    }

    /// createRoutingProfile
    pub async fn routing_profiles_create(&self, body: &std::collections::HashMap<String, serde_json::Value>) -> Result<serde_json::Value, SdkworkError> {
        let path = backend_path(&"/admin/routing/profiles".to_string());
        self.client.post(&path, Some(body), None, None, Some("application/json")).await
    }

    /// listCompiledRoutingSnapshots
    pub async fn routing_snapshots_list(&self, page_size: Option<i64>, cursor: Option<&str>, page: Option<i64>, q: Option<&str>) -> Result<SdkWorkPageData, SdkworkError> {
        let query = build_query_string(&[
            QueryParameterSpec::new("page_size", page_size, "form", true, false, None),
            QueryParameterSpec::new("cursor", cursor, "form", true, false, None),
            QueryParameterSpec::new("page", page, "form", true, false, None),
            QueryParameterSpec::new("q", q, "form", true, false, None),
        ]);
        let path = append_query_string(backend_path(&"/admin/routing/snapshots".to_string()), &query);
        self.client.get(&path, None, None).await
    }

    /// listStorageAuditTrail
    pub async fn storage_audit_list(&self, page_size: Option<i64>, cursor: Option<&str>, page: Option<i64>, q: Option<&str>) -> Result<SdkWorkPageData, SdkworkError> {
        let query = build_query_string(&[
            QueryParameterSpec::new("page_size", page_size, "form", true, false, None),
            QueryParameterSpec::new("cursor", cursor, "form", true, false, None),
            QueryParameterSpec::new("page", page, "form", true, false, None),
            QueryParameterSpec::new("q", q, "form", true, false, None),
        ]);
        let path = append_query_string(backend_path(&"/admin/storage/audit".to_string()), &query);
        self.client.get(&path, None, None).await
    }

    /// getGlobalStorageConfig
    pub async fn storage_config_retrieve(&self) -> Result<serde_json::Value, SdkworkError> {
        let path = backend_path(&"/admin/storage/config".to_string());
        self.client.get(&path, None, None).await
    }

    /// saveGlobalStorageConfig
    pub async fn storage_config_create(&self, body: &std::collections::HashMap<String, serde_json::Value>) -> Result<serde_json::Value, SdkworkError> {
        let path = backend_path(&"/admin/storage/config".to_string());
        self.client.post(&path, Some(body), None, None, Some("application/json")).await
    }

    /// getTenantStorageConfig
    pub async fn storage_config_tenants_retrieve(&self, tenant_id: &str) -> Result<serde_json::Value, SdkworkError> {
        let path = backend_path(&format!("/admin/storage/config/tenants/{}", serialize_path_parameter(tenant_id, PathParameterSpec::new("tenantId", "simple", false))));
        self.client.get(&path, None, None).await
    }

    /// saveTenantStorageConfig
    pub async fn storage_config_tenants_create(&self, tenant_id: &str, body: &std::collections::HashMap<String, serde_json::Value>) -> Result<serde_json::Value, SdkworkError> {
        let path = backend_path(&format!("/admin/storage/config/tenants/{}", serialize_path_parameter(tenant_id, PathParameterSpec::new("tenantId", "simple", false))));
        self.client.post(&path, Some(body), None, None, Some("application/json")).await
    }

    /// deleteTenantStorageConfig
    pub async fn storage_config_tenants_delete(&self, tenant_id: &str) -> Result<(), SdkworkError> {
        let path = backend_path(&format!("/admin/storage/config/tenants/{}", serialize_path_parameter(tenant_id, PathParameterSpec::new("tenantId", "simple", false))));
        self.client.delete(&path, None, None).await
    }

    /// getTenantEffectiveStorageConfig
    pub async fn storage_effective_tenants_retrieve(&self, tenant_id: &str) -> Result<serde_json::Value, SdkworkError> {
        let path = backend_path(&format!("/admin/storage/effective/tenants/{}", serialize_path_parameter(tenant_id, PathParameterSpec::new("tenantId", "simple", false))));
        self.client.get(&path, None, None).await
    }

    /// listStorageProviders
    pub async fn storage_providers_list(&self, page_size: Option<i64>, cursor: Option<&str>, page: Option<i64>, q: Option<&str>) -> Result<SdkWorkPageData, SdkworkError> {
        let query = build_query_string(&[
            QueryParameterSpec::new("page_size", page_size, "form", true, false, None),
            QueryParameterSpec::new("cursor", cursor, "form", true, false, None),
            QueryParameterSpec::new("page", page, "form", true, false, None),
            QueryParameterSpec::new("q", q, "form", true, false, None),
        ]);
        let path = append_query_string(backend_path(&"/admin/storage/providers".to_string()), &query);
        self.client.get(&path, None, None).await
    }

    /// validateGlobalStorageConfig
    pub async fn storage_validation_create(&self, body: &std::collections::HashMap<String, serde_json::Value>) -> Result<serde_json::Value, SdkworkError> {
        let path = backend_path(&"/admin/storage/validate".to_string());
        self.client.post(&path, Some(body), None, None, Some("application/json")).await
    }

    /// validateTenantStorageConfig
    pub async fn storage_validation_tenants_create(&self, tenant_id: &str, body: &std::collections::HashMap<String, serde_json::Value>) -> Result<serde_json::Value, SdkworkError> {
        let path = backend_path(&format!("/admin/storage/validate/tenants/{}", serialize_path_parameter(tenant_id, PathParameterSpec::new("tenantId", "simple", false))));
        self.client.post(&path, Some(body), None, None, Some("application/json")).await
    }

    /// listUsageRecords
    pub async fn usage_records_list(&self, page_size: Option<i64>, cursor: Option<&str>, page: Option<i64>, q: Option<&str>) -> Result<SdkWorkPageData, SdkworkError> {
        let query = build_query_string(&[
            QueryParameterSpec::new("page_size", page_size, "form", true, false, None),
            QueryParameterSpec::new("cursor", cursor, "form", true, false, None),
            QueryParameterSpec::new("page", page, "form", true, false, None),
            QueryParameterSpec::new("q", q, "form", true, false, None),
        ]);
        let path = append_query_string(backend_path(&"/admin/usage/records".to_string()), &query);
        self.client.get(&path, None, None).await
    }

    /// getUsageSummary
    pub async fn usage_summary_retrieve(&self) -> Result<serde_json::Value, SdkworkError> {
        let path = backend_path(&"/admin/usage/summary".to_string());
        self.client.get(&path, None, None).await
    }

}

struct PathParameterSpec<'a> {
    name: &'a str,
    style: &'a str,
    explode: bool,
}

impl<'a> PathParameterSpec<'a> {
    fn new(name: &'a str, style: &'a str, explode: bool) -> Self {
        Self { name, style, explode }
    }
}

fn serialize_path_parameter<T: serde::Serialize>(value: T, spec: PathParameterSpec<'_>) -> String {
    let value = serde_json::to_value(value).unwrap_or(serde_json::Value::Null);
    if value.is_null() {
        return String::new();
    }
    let style = if spec.style.is_empty() { "simple" } else { spec.style };
    match value {
        serde_json::Value::Array(values) => serialize_path_array(spec.name, &values, style, spec.explode),
        serde_json::Value::Object(values) => serialize_path_object(spec.name, &values, style, spec.explode),
        value => format!("{}{}", path_primitive_prefix(spec.name, style), percent_encode(&primitive_to_string(&value))),
    }
}

fn serialize_path_array(name: &str, values: &[serde_json::Value], style: &str, explode: bool) -> String {
    let serialized = values
        .iter()
        .filter(|value| !value.is_null())
        .map(|value| percent_encode(&primitive_to_string(value)))
        .collect::<Vec<_>>();
    if serialized.is_empty() {
        return path_prefix(name, style);
    }
    if style == "matrix" {
        if explode {
            return serialized.iter().map(|item| format!(";{}={}", name, item)).collect::<Vec<_>>().join("");
        }
        return format!(";{}={}", name, serialized.join(","));
    }
    let separator = if explode { "." } else { "," };
    format!("{}{}", path_prefix(name, style), serialized.join(separator))
}

fn serialize_path_object(
    name: &str,
    values: &serde_json::Map<String, serde_json::Value>,
    style: &str,
    explode: bool,
) -> String {
    let mut entries = Vec::new();
    let mut exploded = Vec::new();
    for (key, value) in values {
        if value.is_null() {
            continue;
        }
        let escaped_key = percent_encode(key);
        let escaped_value = percent_encode(&primitive_to_string(value));
        if explode {
            if style == "matrix" {
                exploded.push(format!(";{}={}", escaped_key, escaped_value));
            } else {
                exploded.push(format!("{}={}", escaped_key, escaped_value));
            }
        } else {
            entries.push(escaped_key);
            entries.push(escaped_value);
        }
    }
    if style == "matrix" {
        if explode {
            return exploded.join("");
        }
        return format!(";{}={}", name, entries.join(","));
    }
    if explode {
        let separator = if style == "label" { "." } else { "," };
        return format!("{}{}", path_prefix(name, style), exploded.join(separator));
    }
    format!("{}{}", path_prefix(name, style), entries.join(","))
}

fn path_prefix(name: &str, style: &str) -> String {
    match style {
        "label" => ".".to_string(),
        "matrix" => format!(";{}", name),
        _ => String::new(),
    }
}

fn path_primitive_prefix(name: &str, style: &str) -> String {
    if style == "matrix" {
        format!(";{}=", name)
    } else {
        path_prefix(name, style)
    }
}


struct QueryParameterSpec<'a> {
    name: &'a str,
    value: serde_json::Value,
    style: &'a str,
    explode: bool,
    allow_reserved: bool,
    content_type: Option<&'a str>,
}

impl<'a> QueryParameterSpec<'a> {
    fn new<T: serde::Serialize>(
        name: &'a str,
        value: T,
        style: &'a str,
        explode: bool,
        allow_reserved: bool,
        content_type: Option<&'a str>,
    ) -> Self {
        Self {
            name,
            value: serde_json::to_value(value).unwrap_or(serde_json::Value::Null),
            style,
            explode,
            allow_reserved,
            content_type,
        }
    }
}

fn build_query_string(parameters: &[QueryParameterSpec<'_>]) -> String {
    let mut pairs = Vec::new();
    for parameter in parameters {
        append_serialized_parameter(&mut pairs, parameter);
    }
    pairs.join("&")
}

fn append_serialized_parameter(pairs: &mut Vec<String>, parameter: &QueryParameterSpec<'_>) {
    if parameter.value.is_null() {
        return;
    }
    if parameter.content_type.is_some() {
        pairs.push(format!(
            "{}={}",
            percent_encode(parameter.name),
            encode_query_value(&parameter.value.to_string(), parameter.allow_reserved)
        ));
        return;
    }

    let style = if parameter.style.is_empty() { "form" } else { parameter.style };
    match &parameter.value {
        serde_json::Value::Array(values) => append_array_parameter(pairs, parameter.name, values, style, parameter.explode, parameter.allow_reserved),
        serde_json::Value::Object(values) if style == "deepObject" => append_deep_object_parameter(pairs, parameter.name, values, parameter.allow_reserved),
        serde_json::Value::Object(values) => append_object_parameter(pairs, parameter.name, values, style, parameter.explode, parameter.allow_reserved),
        value => pairs.push(format!("{}={}", percent_encode(parameter.name), encode_query_value(&primitive_to_string(value), parameter.allow_reserved))),
    }
}

fn append_array_parameter(
    pairs: &mut Vec<String>,
    name: &str,
    values: &[serde_json::Value],
    style: &str,
    explode: bool,
    allow_reserved: bool,
) {
    let serialized = values.iter().filter(|value| !value.is_null()).map(primitive_to_string).collect::<Vec<_>>();
    if serialized.is_empty() {
        return;
    }
    if style == "form" && explode {
        for item in serialized {
            pairs.push(format!("{}={}", percent_encode(name), encode_query_value(&item, allow_reserved)));
        }
        return;
    }
    pairs.push(format!("{}={}", percent_encode(name), encode_query_value(&serialized.join(","), allow_reserved)));
}

fn append_object_parameter(
    pairs: &mut Vec<String>,
    name: &str,
    values: &serde_json::Map<String, serde_json::Value>,
    style: &str,
    explode: bool,
    allow_reserved: bool,
) {
    let mut serialized = Vec::new();
    for (key, value) in values {
        if value.is_null() {
            continue;
        }
        if style == "form" && explode {
            pairs.push(format!("{}={}", percent_encode(key), encode_query_value(&primitive_to_string(value), allow_reserved)));
        } else {
            serialized.push(key.clone());
            serialized.push(primitive_to_string(value));
        }
    }
    if !serialized.is_empty() {
        pairs.push(format!("{}={}", percent_encode(name), encode_query_value(&serialized.join(","), allow_reserved)));
    }
}

fn append_deep_object_parameter(
    pairs: &mut Vec<String>,
    name: &str,
    values: &serde_json::Map<String, serde_json::Value>,
    allow_reserved: bool,
) {
    for (key, value) in values {
        if !value.is_null() {
            pairs.push(format!("{}={}", percent_encode(&format!("{}[{}]", name, key)), encode_query_value(&primitive_to_string(value), allow_reserved)));
        }
    }
}

fn encode_query_value(value: &str, allow_reserved: bool) -> String {
    let mut encoded = percent_encode(value);
    if !allow_reserved {
        return encoded;
    }
    for (escaped, reserved) in [
        ("%3A", ":"), ("%2F", "/"), ("%3F", "?"), ("%23", "#"),
        ("%5B", "["), ("%5D", "]"), ("%40", "@"), ("%21", "!"),
        ("%24", "$"), ("%26", "&"), ("%27", "'"), ("%28", "("),
        ("%29", ")"), ("%2A", "*"), ("%2B", "+"), ("%2C", ","),
        ("%3B", ";"), ("%3D", "="),
    ] {
        encoded = encoded.replace(escaped, reserved);
    }
    encoded
}

fn primitive_to_string(value: &serde_json::Value) -> String {
    match value {
        serde_json::Value::String(value) => value.clone(),
        serde_json::Value::Number(value) => value.to_string(),
        serde_json::Value::Bool(value) => value.to_string(),
        other => other.to_string(),
    }
}

fn percent_encode(value: &str) -> String {
    value
        .bytes()
        .flat_map(|byte| match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                vec![byte as char]
            }
            _ => format!("%{:02X}", byte).chars().collect(),
        })
        .collect()
}
