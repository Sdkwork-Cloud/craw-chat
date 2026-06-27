package api

import (
    "fmt"
    "net/url"
    "strings"
    sdktypes "github.com/sdkwork/im-backend-api-generated/types"
    sdkhttp "github.com/sdkwork/im-backend-api-generated/http"
)

type AdminApi struct {
    client *sdkhttp.Client
}

func NewAdminApi(client *sdkhttp.Client) *AdminApi {
    return &AdminApi{client: client}
}

// listApiKeyGroups
func (a *AdminApi) ApiKeyGroupsList() (sdktypes.LooseJsonValue, error) {
    raw, err := a.client.Get(BackendApiPath("/admin/api_key_groups"), nil, nil)
    if err != nil {
        var zero sdktypes.LooseJsonValue
        return zero, err
    }
    return decodeResult[sdktypes.LooseJsonValue](raw)
}

// createApiKeyGroup
func (a *AdminApi) ApiKeyGroupsCreate(body sdktypes.LooseJsonObject) (sdktypes.LooseJsonValue, error) {
    raw, err := a.client.Post(BackendApiPath("/admin/api_key_groups"), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.LooseJsonValue
        return zero, err
    }
    return decodeResult[sdktypes.LooseJsonValue](raw)
}

// updateApiKeyGroup
func (a *AdminApi) ApiKeyGroupsUpdate(groupId string, body sdktypes.LooseJsonObject) (sdktypes.LooseJsonValue, error) {
    raw, err := a.client.Patch(BackendApiPath(fmt.Sprintf("/admin/api_key_groups/%s", SerializePathParameter(groupId, PathParameterSpec{Name: "groupId", Style: "simple", Explode: false}))), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.LooseJsonValue
        return zero, err
    }
    return decodeResult[sdktypes.LooseJsonValue](raw)
}

// deleteApiKeyGroup
func (a *AdminApi) ApiKeyGroupsDelete(groupId string) (sdktypes.LooseJsonValue, error) {
    raw, err := a.client.Delete(BackendApiPath(fmt.Sprintf("/admin/api_key_groups/%s", SerializePathParameter(groupId, PathParameterSpec{Name: "groupId", Style: "simple", Explode: false}))), nil, nil)
    if err != nil {
        var zero sdktypes.LooseJsonValue
        return zero, err
    }
    return decodeResult[sdktypes.LooseJsonValue](raw)
}

// updateApiKeyGroupStatus
func (a *AdminApi) ApiKeyGroupsStatus(groupId string, body sdktypes.LooseJsonObject) (sdktypes.LooseJsonValue, error) {
    raw, err := a.client.Post(BackendApiPath(fmt.Sprintf("/admin/api_key_groups/%s/status", SerializePathParameter(groupId, PathParameterSpec{Name: "groupId", Style: "simple", Explode: false}))), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.LooseJsonValue
        return zero, err
    }
    return decodeResult[sdktypes.LooseJsonValue](raw)
}

// listApiKeys
func (a *AdminApi) ApiKeysList() (sdktypes.LooseJsonValue, error) {
    raw, err := a.client.Get(BackendApiPath("/admin/api_keys"), nil, nil)
    if err != nil {
        var zero sdktypes.LooseJsonValue
        return zero, err
    }
    return decodeResult[sdktypes.LooseJsonValue](raw)
}

// createApiKey
func (a *AdminApi) ApiKeysCreate(body sdktypes.LooseJsonObject) (sdktypes.LooseJsonValue, error) {
    raw, err := a.client.Post(BackendApiPath("/admin/api_keys"), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.LooseJsonValue
        return zero, err
    }
    return decodeResult[sdktypes.LooseJsonValue](raw)
}

// updateApiKey
func (a *AdminApi) ApiKeysUpdate(hashedKey string, body sdktypes.LooseJsonObject) (sdktypes.LooseJsonValue, error) {
    raw, err := a.client.Put(BackendApiPath(fmt.Sprintf("/admin/api_keys/%s", SerializePathParameter(hashedKey, PathParameterSpec{Name: "hashedKey", Style: "simple", Explode: false}))), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.LooseJsonValue
        return zero, err
    }
    return decodeResult[sdktypes.LooseJsonValue](raw)
}

// deleteApiKey
func (a *AdminApi) ApiKeysDelete(hashedKey string) (sdktypes.LooseJsonValue, error) {
    raw, err := a.client.Delete(BackendApiPath(fmt.Sprintf("/admin/api_keys/%s", SerializePathParameter(hashedKey, PathParameterSpec{Name: "hashedKey", Style: "simple", Explode: false}))), nil, nil)
    if err != nil {
        var zero sdktypes.LooseJsonValue
        return zero, err
    }
    return decodeResult[sdktypes.LooseJsonValue](raw)
}

// updateApiKeyStatus
func (a *AdminApi) ApiKeysStatus(hashedKey string, body sdktypes.LooseJsonObject) (sdktypes.LooseJsonValue, error) {
    raw, err := a.client.Post(BackendApiPath(fmt.Sprintf("/admin/api_keys/%s/status", SerializePathParameter(hashedKey, PathParameterSpec{Name: "hashedKey", Style: "simple", Explode: false}))), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.LooseJsonValue
        return zero, err
    }
    return decodeResult[sdktypes.LooseJsonValue](raw)
}

// listBillingEvents
func (a *AdminApi) BillingEventsList() (sdktypes.LooseJsonValue, error) {
    raw, err := a.client.Get(BackendApiPath("/admin/billing/events"), nil, nil)
    if err != nil {
        var zero sdktypes.LooseJsonValue
        return zero, err
    }
    return decodeResult[sdktypes.LooseJsonValue](raw)
}

// getBillingEventSummary
func (a *AdminApi) BillingEventsSummaryRetrieve() (sdktypes.LooseJsonValue, error) {
    raw, err := a.client.Get(BackendApiPath("/admin/billing/events/summary"), nil, nil)
    if err != nil {
        var zero sdktypes.LooseJsonValue
        return zero, err
    }
    return decodeResult[sdktypes.LooseJsonValue](raw)
}

// getBillingSummary
func (a *AdminApi) BillingSummaryRetrieve() (sdktypes.LooseJsonValue, error) {
    raw, err := a.client.Get(BackendApiPath("/admin/billing/summary"), nil, nil)
    if err != nil {
        var zero sdktypes.LooseJsonValue
        return zero, err
    }
    return decodeResult[sdktypes.LooseJsonValue](raw)
}

// listChannelModels
func (a *AdminApi) ChannelModelsList() (sdktypes.LooseJsonValue, error) {
    raw, err := a.client.Get(BackendApiPath("/admin/channel_models"), nil, nil)
    if err != nil {
        var zero sdktypes.LooseJsonValue
        return zero, err
    }
    return decodeResult[sdktypes.LooseJsonValue](raw)
}

// saveChannelModel
func (a *AdminApi) ChannelModelsCreate(body sdktypes.LooseJsonObject) (sdktypes.LooseJsonValue, error) {
    raw, err := a.client.Post(BackendApiPath("/admin/channel_models"), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.LooseJsonValue
        return zero, err
    }
    return decodeResult[sdktypes.LooseJsonValue](raw)
}

// deleteChannelModel
func (a *AdminApi) ChannelModelsDelete(channelId string, modelId string) (sdktypes.LooseJsonValue, error) {
    raw, err := a.client.Delete(BackendApiPath(fmt.Sprintf("/admin/channel_models/%s/models/%s", SerializePathParameter(channelId, PathParameterSpec{Name: "channelId", Style: "simple", Explode: false}), SerializePathParameter(modelId, PathParameterSpec{Name: "modelId", Style: "simple", Explode: false}))), nil, nil)
    if err != nil {
        var zero sdktypes.LooseJsonValue
        return zero, err
    }
    return decodeResult[sdktypes.LooseJsonValue](raw)
}

// listChannels
func (a *AdminApi) ChannelsList() (sdktypes.LooseJsonValue, error) {
    raw, err := a.client.Get(BackendApiPath("/admin/channels"), nil, nil)
    if err != nil {
        var zero sdktypes.LooseJsonValue
        return zero, err
    }
    return decodeResult[sdktypes.LooseJsonValue](raw)
}

// saveChannel
func (a *AdminApi) ChannelsCreate(body sdktypes.LooseJsonObject) (sdktypes.LooseJsonValue, error) {
    raw, err := a.client.Post(BackendApiPath("/admin/channels"), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.LooseJsonValue
        return zero, err
    }
    return decodeResult[sdktypes.LooseJsonValue](raw)
}

// deleteChannel
func (a *AdminApi) ChannelsDelete(channelId string) (sdktypes.LooseJsonValue, error) {
    raw, err := a.client.Delete(BackendApiPath(fmt.Sprintf("/admin/channels/%s", SerializePathParameter(channelId, PathParameterSpec{Name: "channelId", Style: "simple", Explode: false}))), nil, nil)
    if err != nil {
        var zero sdktypes.LooseJsonValue
        return zero, err
    }
    return decodeResult[sdktypes.LooseJsonValue](raw)
}

// listCredentials
func (a *AdminApi) CredentialsList() (sdktypes.LooseJsonValue, error) {
    raw, err := a.client.Get(BackendApiPath("/admin/credentials"), nil, nil)
    if err != nil {
        var zero sdktypes.LooseJsonValue
        return zero, err
    }
    return decodeResult[sdktypes.LooseJsonValue](raw)
}

// saveCredential
func (a *AdminApi) CredentialsCreate(body sdktypes.LooseJsonObject) (sdktypes.LooseJsonValue, error) {
    raw, err := a.client.Post(BackendApiPath("/admin/credentials"), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.LooseJsonValue
        return zero, err
    }
    return decodeResult[sdktypes.LooseJsonValue](raw)
}

// deleteCredential
func (a *AdminApi) CredentialsProvidersKeysDelete(tenantId string, providerId string, keyReference string) (sdktypes.LooseJsonValue, error) {
    raw, err := a.client.Delete(BackendApiPath(fmt.Sprintf("/admin/credentials/%s/providers/%s/keys/%s", SerializePathParameter(tenantId, PathParameterSpec{Name: "tenantId", Style: "simple", Explode: false}), SerializePathParameter(providerId, PathParameterSpec{Name: "providerId", Style: "simple", Explode: false}), SerializePathParameter(keyReference, PathParameterSpec{Name: "keyReference", Style: "simple", Explode: false}))), nil, nil)
    if err != nil {
        var zero sdktypes.LooseJsonValue
        return zero, err
    }
    return decodeResult[sdktypes.LooseJsonValue](raw)
}

// reloadExtensionRuntimes
func (a *AdminApi) ExtensionsRuntimeReloadsCreate(body sdktypes.LooseJsonObject) (sdktypes.LooseJsonValue, error) {
    raw, err := a.client.Post(BackendApiPath("/admin/extensions/runtime_reloads"), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.LooseJsonValue
        return zero, err
    }
    return decodeResult[sdktypes.LooseJsonValue](raw)
}

// listRuntimeStatuses
func (a *AdminApi) ExtensionsRuntimeStatusesList() (sdktypes.LooseJsonValue, error) {
    raw, err := a.client.Get(BackendApiPath("/admin/extensions/runtime_statuses"), nil, nil)
    if err != nil {
        var zero sdktypes.LooseJsonValue
        return zero, err
    }
    return decodeResult[sdktypes.LooseJsonValue](raw)
}

// listRateLimitPolicies
func (a *AdminApi) GatewayRateLimitPoliciesList() (sdktypes.LooseJsonValue, error) {
    raw, err := a.client.Get(BackendApiPath("/admin/gateway/rate_limit_policies"), nil, nil)
    if err != nil {
        var zero sdktypes.LooseJsonValue
        return zero, err
    }
    return decodeResult[sdktypes.LooseJsonValue](raw)
}

// createRateLimitPolicy
func (a *AdminApi) GatewayRateLimitPoliciesCreate(body sdktypes.LooseJsonObject) (sdktypes.LooseJsonValue, error) {
    raw, err := a.client.Post(BackendApiPath("/admin/gateway/rate_limit_policies"), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.LooseJsonValue
        return zero, err
    }
    return decodeResult[sdktypes.LooseJsonValue](raw)
}

// listRateLimitWindows
func (a *AdminApi) GatewayRateLimitWindowsList() (sdktypes.LooseJsonValue, error) {
    raw, err := a.client.Get(BackendApiPath("/admin/gateway/rate_limit_windows"), nil, nil)
    if err != nil {
        var zero sdktypes.LooseJsonValue
        return zero, err
    }
    return decodeResult[sdktypes.LooseJsonValue](raw)
}

// listMarketingCampaigns
func (a *AdminApi) MarketingCampaignsList() (sdktypes.LooseJsonValue, error) {
    raw, err := a.client.Get(BackendApiPath("/admin/marketing/campaigns"), nil, nil)
    if err != nil {
        var zero sdktypes.LooseJsonValue
        return zero, err
    }
    return decodeResult[sdktypes.LooseJsonValue](raw)
}

// saveMarketingCampaign
func (a *AdminApi) MarketingCampaignsCreate(body sdktypes.LooseJsonObject) (sdktypes.LooseJsonValue, error) {
    raw, err := a.client.Post(BackendApiPath("/admin/marketing/campaigns"), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.LooseJsonValue
        return zero, err
    }
    return decodeResult[sdktypes.LooseJsonValue](raw)
}

// updateMarketingCampaignStatus
func (a *AdminApi) MarketingCampaignsStatus(marketingCampaignId string, body sdktypes.LooseJsonObject) (sdktypes.LooseJsonValue, error) {
    raw, err := a.client.Post(BackendApiPath(fmt.Sprintf("/admin/marketing/campaigns/%s/status", SerializePathParameter(marketingCampaignId, PathParameterSpec{Name: "marketingCampaignId", Style: "simple", Explode: false}))), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.LooseJsonValue
        return zero, err
    }
    return decodeResult[sdktypes.LooseJsonValue](raw)
}

// listModelPrices
func (a *AdminApi) ModelPricesList() (sdktypes.LooseJsonValue, error) {
    raw, err := a.client.Get(BackendApiPath("/admin/model_prices"), nil, nil)
    if err != nil {
        var zero sdktypes.LooseJsonValue
        return zero, err
    }
    return decodeResult[sdktypes.LooseJsonValue](raw)
}

// saveModelPrice
func (a *AdminApi) ModelPricesCreate(body sdktypes.LooseJsonObject) (sdktypes.LooseJsonValue, error) {
    raw, err := a.client.Post(BackendApiPath("/admin/model_prices"), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.LooseJsonValue
        return zero, err
    }
    return decodeResult[sdktypes.LooseJsonValue](raw)
}

// deleteModelPrice
func (a *AdminApi) ModelPricesProvidersDelete(channelId string, modelId string, proxyProviderId string) (sdktypes.LooseJsonValue, error) {
    raw, err := a.client.Delete(BackendApiPath(fmt.Sprintf("/admin/model_prices/%s/models/%s/providers/%s", SerializePathParameter(channelId, PathParameterSpec{Name: "channelId", Style: "simple", Explode: false}), SerializePathParameter(modelId, PathParameterSpec{Name: "modelId", Style: "simple", Explode: false}), SerializePathParameter(proxyProviderId, PathParameterSpec{Name: "proxyProviderId", Style: "simple", Explode: false}))), nil, nil)
    if err != nil {
        var zero sdktypes.LooseJsonValue
        return zero, err
    }
    return decodeResult[sdktypes.LooseJsonValue](raw)
}

// listModels
func (a *AdminApi) ModelsList() (sdktypes.LooseJsonValue, error) {
    raw, err := a.client.Get(BackendApiPath("/admin/models"), nil, nil)
    if err != nil {
        var zero sdktypes.LooseJsonValue
        return zero, err
    }
    return decodeResult[sdktypes.LooseJsonValue](raw)
}

// saveModel
func (a *AdminApi) ModelsCreate(body sdktypes.LooseJsonObject) (sdktypes.LooseJsonValue, error) {
    raw, err := a.client.Post(BackendApiPath("/admin/models"), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.LooseJsonValue
        return zero, err
    }
    return decodeResult[sdktypes.LooseJsonValue](raw)
}

// deleteModel
func (a *AdminApi) ModelsProvidersDelete(externalName string, providerId string) (sdktypes.LooseJsonValue, error) {
    raw, err := a.client.Delete(BackendApiPath(fmt.Sprintf("/admin/models/%s/providers/%s", SerializePathParameter(externalName, PathParameterSpec{Name: "externalName", Style: "simple", Explode: false}), SerializePathParameter(providerId, PathParameterSpec{Name: "providerId", Style: "simple", Explode: false}))), nil, nil)
    if err != nil {
        var zero sdktypes.LooseJsonValue
        return zero, err
    }
    return decodeResult[sdktypes.LooseJsonValue](raw)
}

// listProviders
func (a *AdminApi) ProvidersList() (sdktypes.LooseJsonValue, error) {
    raw, err := a.client.Get(BackendApiPath("/admin/providers"), nil, nil)
    if err != nil {
        var zero sdktypes.LooseJsonValue
        return zero, err
    }
    return decodeResult[sdktypes.LooseJsonValue](raw)
}

// saveProvider
func (a *AdminApi) ProvidersCreate(body sdktypes.LooseJsonObject) (sdktypes.LooseJsonValue, error) {
    raw, err := a.client.Post(BackendApiPath("/admin/providers"), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.LooseJsonValue
        return zero, err
    }
    return decodeResult[sdktypes.LooseJsonValue](raw)
}

// deleteProvider
func (a *AdminApi) ProvidersDelete(providerId string) (sdktypes.LooseJsonValue, error) {
    raw, err := a.client.Delete(BackendApiPath(fmt.Sprintf("/admin/providers/%s", SerializePathParameter(providerId, PathParameterSpec{Name: "providerId", Style: "simple", Explode: false}))), nil, nil)
    if err != nil {
        var zero sdktypes.LooseJsonValue
        return zero, err
    }
    return decodeResult[sdktypes.LooseJsonValue](raw)
}

// listRoutingDecisionLogs
func (a *AdminApi) RoutingDecisionLogsList() (sdktypes.LooseJsonValue, error) {
    raw, err := a.client.Get(BackendApiPath("/admin/routing/decision_logs"), nil, nil)
    if err != nil {
        var zero sdktypes.LooseJsonValue
        return zero, err
    }
    return decodeResult[sdktypes.LooseJsonValue](raw)
}

// listProviderHealthSnapshots
func (a *AdminApi) RoutingHealthSnapshotsRetrieve() (sdktypes.LooseJsonValue, error) {
    raw, err := a.client.Get(BackendApiPath("/admin/routing/health_snapshots"), nil, nil)
    if err != nil {
        var zero sdktypes.LooseJsonValue
        return zero, err
    }
    return decodeResult[sdktypes.LooseJsonValue](raw)
}

// listRoutingProfiles
func (a *AdminApi) RoutingProfilesList() (sdktypes.LooseJsonValue, error) {
    raw, err := a.client.Get(BackendApiPath("/admin/routing/profiles"), nil, nil)
    if err != nil {
        var zero sdktypes.LooseJsonValue
        return zero, err
    }
    return decodeResult[sdktypes.LooseJsonValue](raw)
}

// createRoutingProfile
func (a *AdminApi) RoutingProfilesCreate(body sdktypes.LooseJsonObject) (sdktypes.LooseJsonValue, error) {
    raw, err := a.client.Post(BackendApiPath("/admin/routing/profiles"), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.LooseJsonValue
        return zero, err
    }
    return decodeResult[sdktypes.LooseJsonValue](raw)
}

// listCompiledRoutingSnapshots
func (a *AdminApi) RoutingSnapshotsList() (sdktypes.LooseJsonValue, error) {
    raw, err := a.client.Get(BackendApiPath("/admin/routing/snapshots"), nil, nil)
    if err != nil {
        var zero sdktypes.LooseJsonValue
        return zero, err
    }
    return decodeResult[sdktypes.LooseJsonValue](raw)
}

// listStorageAuditTrail
func (a *AdminApi) StorageAuditList() (sdktypes.LooseJsonValue, error) {
    raw, err := a.client.Get(BackendApiPath("/admin/storage/audit"), nil, nil)
    if err != nil {
        var zero sdktypes.LooseJsonValue
        return zero, err
    }
    return decodeResult[sdktypes.LooseJsonValue](raw)
}

// getGlobalStorageConfig
func (a *AdminApi) StorageConfigRetrieve() (sdktypes.LooseJsonValue, error) {
    raw, err := a.client.Get(BackendApiPath("/admin/storage/config"), nil, nil)
    if err != nil {
        var zero sdktypes.LooseJsonValue
        return zero, err
    }
    return decodeResult[sdktypes.LooseJsonValue](raw)
}

// saveGlobalStorageConfig
func (a *AdminApi) StorageConfigCreate(body sdktypes.LooseJsonObject) (sdktypes.LooseJsonValue, error) {
    raw, err := a.client.Post(BackendApiPath("/admin/storage/config"), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.LooseJsonValue
        return zero, err
    }
    return decodeResult[sdktypes.LooseJsonValue](raw)
}

// getTenantStorageConfig
func (a *AdminApi) StorageConfigTenantsRetrieve(tenantId string) (sdktypes.LooseJsonValue, error) {
    raw, err := a.client.Get(BackendApiPath(fmt.Sprintf("/admin/storage/config/tenants/%s", SerializePathParameter(tenantId, PathParameterSpec{Name: "tenantId", Style: "simple", Explode: false}))), nil, nil)
    if err != nil {
        var zero sdktypes.LooseJsonValue
        return zero, err
    }
    return decodeResult[sdktypes.LooseJsonValue](raw)
}

// saveTenantStorageConfig
func (a *AdminApi) StorageConfigTenantsCreate(tenantId string, body sdktypes.LooseJsonObject) (sdktypes.LooseJsonValue, error) {
    raw, err := a.client.Post(BackendApiPath(fmt.Sprintf("/admin/storage/config/tenants/%s", SerializePathParameter(tenantId, PathParameterSpec{Name: "tenantId", Style: "simple", Explode: false}))), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.LooseJsonValue
        return zero, err
    }
    return decodeResult[sdktypes.LooseJsonValue](raw)
}

// deleteTenantStorageConfig
func (a *AdminApi) StorageConfigTenantsDelete(tenantId string) (sdktypes.LooseJsonValue, error) {
    raw, err := a.client.Delete(BackendApiPath(fmt.Sprintf("/admin/storage/config/tenants/%s", SerializePathParameter(tenantId, PathParameterSpec{Name: "tenantId", Style: "simple", Explode: false}))), nil, nil)
    if err != nil {
        var zero sdktypes.LooseJsonValue
        return zero, err
    }
    return decodeResult[sdktypes.LooseJsonValue](raw)
}

// getTenantEffectiveStorageConfig
func (a *AdminApi) StorageEffectiveTenantsRetrieve(tenantId string) (sdktypes.LooseJsonValue, error) {
    raw, err := a.client.Get(BackendApiPath(fmt.Sprintf("/admin/storage/effective/tenants/%s", SerializePathParameter(tenantId, PathParameterSpec{Name: "tenantId", Style: "simple", Explode: false}))), nil, nil)
    if err != nil {
        var zero sdktypes.LooseJsonValue
        return zero, err
    }
    return decodeResult[sdktypes.LooseJsonValue](raw)
}

// listStorageProviders
func (a *AdminApi) StorageProvidersList() (sdktypes.LooseJsonValue, error) {
    raw, err := a.client.Get(BackendApiPath("/admin/storage/providers"), nil, nil)
    if err != nil {
        var zero sdktypes.LooseJsonValue
        return zero, err
    }
    return decodeResult[sdktypes.LooseJsonValue](raw)
}

// validateGlobalStorageConfig
func (a *AdminApi) StorageValidationCreate(body sdktypes.LooseJsonObject) (sdktypes.LooseJsonValue, error) {
    raw, err := a.client.Post(BackendApiPath("/admin/storage/validate"), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.LooseJsonValue
        return zero, err
    }
    return decodeResult[sdktypes.LooseJsonValue](raw)
}

// validateTenantStorageConfig
func (a *AdminApi) StorageValidationTenantsCreate(tenantId string, body sdktypes.LooseJsonObject) (sdktypes.LooseJsonValue, error) {
    raw, err := a.client.Post(BackendApiPath(fmt.Sprintf("/admin/storage/validate/tenants/%s", SerializePathParameter(tenantId, PathParameterSpec{Name: "tenantId", Style: "simple", Explode: false}))), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.LooseJsonValue
        return zero, err
    }
    return decodeResult[sdktypes.LooseJsonValue](raw)
}

// listUsageRecords
func (a *AdminApi) UsageRecordsList() (sdktypes.LooseJsonValue, error) {
    raw, err := a.client.Get(BackendApiPath("/admin/usage/records"), nil, nil)
    if err != nil {
        var zero sdktypes.LooseJsonValue
        return zero, err
    }
    return decodeResult[sdktypes.LooseJsonValue](raw)
}

// getUsageSummary
func (a *AdminApi) UsageSummaryRetrieve() (sdktypes.LooseJsonValue, error) {
    raw, err := a.client.Get(BackendApiPath("/admin/usage/summary"), nil, nil)
    if err != nil {
        var zero sdktypes.LooseJsonValue
        return zero, err
    }
    return decodeResult[sdktypes.LooseJsonValue](raw)
}

type PathParameterSpec struct {
    Name    string
    Style   string
    Explode bool
}

func SerializePathParameter(value interface{}, spec PathParameterSpec) string {
    if value == nil {
        return ""
    }
    style := spec.Style
    if style == "" {
        style = "simple"
    }

    switch typed := value.(type) {
    case []string:
        return SerializePathArray(spec.Name, stringSliceToInterface(typed), style, spec.Explode)
    case []int:
        return SerializePathArray(spec.Name, intSliceToInterface(typed), style, spec.Explode)
    case []interface{}:
        return SerializePathArray(spec.Name, typed, style, spec.Explode)
    case map[string]string:
        return SerializePathObject(spec.Name, stringMapToInterface(typed), style, spec.Explode)
    case map[string]int:
        return SerializePathObject(spec.Name, intMapToInterface(typed), style, spec.Explode)
    case map[string]interface{}:
        return SerializePathObject(spec.Name, typed, style, spec.Explode)
    default:
        return PathPrefix(spec.Name, style) + url.PathEscape(fmt.Sprint(value))
    }
}

func SerializePathArray(name string, values []interface{}, style string, explode bool) string {
    serialized := make([]string, 0, len(values))
    for _, item := range values {
        if item != nil {
            serialized = append(serialized, url.PathEscape(fmt.Sprint(item)))
        }
    }
    if len(serialized) == 0 {
        return PathPrefix(name, style)
    }
    if style == "matrix" {
        if explode {
            parts := make([]string, 0, len(serialized))
            for _, item := range serialized {
                parts = append(parts, ";"+name+"="+item)
            }
            return strings.Join(parts, "")
        }
        return ";" + name + "=" + strings.Join(serialized, ",")
    }
    separator := ","
    if explode {
        separator = "."
    }
    return PathPrefix(name, style) + strings.Join(serialized, separator)
}

func SerializePathObject(name string, values map[string]interface{}, style string, explode bool) string {
    entries := make([]string, 0, len(values)*2)
    exploded := make([]string, 0, len(values))
    for key, value := range values {
        if value == nil {
            continue
        }
        escapedKey := url.PathEscape(key)
        escapedValue := url.PathEscape(fmt.Sprint(value))
        if explode {
            if style == "matrix" {
                exploded = append(exploded, ";"+escapedKey+"="+escapedValue)
            } else {
                exploded = append(exploded, escapedKey+"="+escapedValue)
            }
        } else {
            entries = append(entries, escapedKey, escapedValue)
        }
    }
    if style == "matrix" {
        if explode {
            return strings.Join(exploded, "")
        }
        return ";" + name + "=" + strings.Join(entries, ",")
    }
    if explode {
        separator := ","
        if style == "label" {
            separator = "."
        }
        return PathPrefix(name, style) + strings.Join(exploded, separator)
    }
    return PathPrefix(name, style) + strings.Join(entries, ",")
}

func PathPrefix(name string, style string) string {
    if style == "label" {
        return "."
    }
    if style == "matrix" {
        return ";" + name
    }
    return ""
}


func stringSliceToInterface(values []string) []interface{} {
    result := make([]interface{}, 0, len(values))
    for _, value := range values {
        result = append(result, value)
    }
    return result
}

func intSliceToInterface(values []int) []interface{} {
    result := make([]interface{}, 0, len(values))
    for _, value := range values {
        result = append(result, value)
    }
    return result
}

func stringMapToInterface(values map[string]string) map[string]interface{} {
    result := make(map[string]interface{}, len(values))
    for key, value := range values {
        result[key] = value
    }
    return result
}

func intMapToInterface(values map[string]int) map[string]interface{} {
    result := make(map[string]interface{}, len(values))
    for key, value := range values {
        result[key] = value
    }
    return result
}
