package api

import (
    "encoding/json"
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
func (a *AdminApi) ApiKeyGroupsList(pageSize *int, cursor *string, page *int, q *string) (sdktypes.SdkWorkListResponse, error) {
    query := BuildQueryString([]QueryParameterSpec{
        {Name: "page_size", Value: func() interface{} { if pageSize == nil { return nil }; return *pageSize }(), Style: "form", Explode: true, AllowReserved: false},
        {Name: "cursor", Value: func() interface{} { if cursor == nil { return nil }; return *cursor }(), Style: "form", Explode: true, AllowReserved: false},
        {Name: "page", Value: func() interface{} { if page == nil { return nil }; return *page }(), Style: "form", Explode: true, AllowReserved: false},
        {Name: "q", Value: func() interface{} { if q == nil { return nil }; return *q }(), Style: "form", Explode: true, AllowReserved: false},
    })
    raw, err := a.client.Get(AppendQueryString(BackendApiPath("/admin/api_key_groups"), query), nil, nil)
    if err != nil {
        var zero sdktypes.SdkWorkListResponse
        return zero, err
    }
    return decodeResult[sdktypes.SdkWorkListResponse](raw)
}

// createApiKeyGroup
func (a *AdminApi) ApiKeyGroupsCreate(body sdktypes.LooseJsonObject) (sdktypes.ApiKeyGroupsCreateResponse201, error) {
    raw, err := a.client.Post(BackendApiPath("/admin/api_key_groups"), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.ApiKeyGroupsCreateResponse201
        return zero, err
    }
    return decodeResult[sdktypes.ApiKeyGroupsCreateResponse201](raw)
}

// updateApiKeyGroup
func (a *AdminApi) ApiKeyGroupsUpdate(groupId string, body sdktypes.LooseJsonObject) (sdktypes.ApiKeyGroupsUpdateResponse, error) {
    raw, err := a.client.Patch(BackendApiPath(fmt.Sprintf("/admin/api_key_groups/%s", SerializePathParameter(groupId, PathParameterSpec{Name: "groupId", Style: "simple", Explode: false}))), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.ApiKeyGroupsUpdateResponse
        return zero, err
    }
    return decodeResult[sdktypes.ApiKeyGroupsUpdateResponse](raw)
}

// deleteApiKeyGroup
func (a *AdminApi) ApiKeyGroupsDelete(groupId string) (struct{}, error) {
    raw, err := a.client.Delete(BackendApiPath(fmt.Sprintf("/admin/api_key_groups/%s", SerializePathParameter(groupId, PathParameterSpec{Name: "groupId", Style: "simple", Explode: false}))), nil, nil)
    if err != nil {
        var zero struct{}
        return zero, err
    }
    return decodeResult[struct{}](raw)
}

// updateApiKeyGroupStatus
func (a *AdminApi) ApiKeyGroupsStatus(groupId string, body sdktypes.LooseJsonObject) (sdktypes.ApiKeyGroupsStatusResponse, error) {
    raw, err := a.client.Post(BackendApiPath(fmt.Sprintf("/admin/api_key_groups/%s/status", SerializePathParameter(groupId, PathParameterSpec{Name: "groupId", Style: "simple", Explode: false}))), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.ApiKeyGroupsStatusResponse
        return zero, err
    }
    return decodeResult[sdktypes.ApiKeyGroupsStatusResponse](raw)
}

// listApiKeys
func (a *AdminApi) ApiKeysList(pageSize *int, cursor *string, page *int, q *string) (sdktypes.SdkWorkListResponse, error) {
    query := BuildQueryString([]QueryParameterSpec{
        {Name: "page_size", Value: func() interface{} { if pageSize == nil { return nil }; return *pageSize }(), Style: "form", Explode: true, AllowReserved: false},
        {Name: "cursor", Value: func() interface{} { if cursor == nil { return nil }; return *cursor }(), Style: "form", Explode: true, AllowReserved: false},
        {Name: "page", Value: func() interface{} { if page == nil { return nil }; return *page }(), Style: "form", Explode: true, AllowReserved: false},
        {Name: "q", Value: func() interface{} { if q == nil { return nil }; return *q }(), Style: "form", Explode: true, AllowReserved: false},
    })
    raw, err := a.client.Get(AppendQueryString(BackendApiPath("/admin/api_keys"), query), nil, nil)
    if err != nil {
        var zero sdktypes.SdkWorkListResponse
        return zero, err
    }
    return decodeResult[sdktypes.SdkWorkListResponse](raw)
}

// createApiKey
func (a *AdminApi) ApiKeysCreate(body sdktypes.LooseJsonObject) (sdktypes.ApiKeysCreateResponse201, error) {
    raw, err := a.client.Post(BackendApiPath("/admin/api_keys"), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.ApiKeysCreateResponse201
        return zero, err
    }
    return decodeResult[sdktypes.ApiKeysCreateResponse201](raw)
}

// updateApiKey
func (a *AdminApi) ApiKeysUpdate(hashedKey string, body sdktypes.LooseJsonObject) (sdktypes.ApiKeysUpdateResponse, error) {
    raw, err := a.client.Put(BackendApiPath(fmt.Sprintf("/admin/api_keys/%s", SerializePathParameter(hashedKey, PathParameterSpec{Name: "hashedKey", Style: "simple", Explode: false}))), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.ApiKeysUpdateResponse
        return zero, err
    }
    return decodeResult[sdktypes.ApiKeysUpdateResponse](raw)
}

// deleteApiKey
func (a *AdminApi) ApiKeysDelete(hashedKey string) (struct{}, error) {
    raw, err := a.client.Delete(BackendApiPath(fmt.Sprintf("/admin/api_keys/%s", SerializePathParameter(hashedKey, PathParameterSpec{Name: "hashedKey", Style: "simple", Explode: false}))), nil, nil)
    if err != nil {
        var zero struct{}
        return zero, err
    }
    return decodeResult[struct{}](raw)
}

// updateApiKeyStatus
func (a *AdminApi) ApiKeysStatus(hashedKey string, body sdktypes.LooseJsonObject) (sdktypes.ApiKeysStatusResponse, error) {
    raw, err := a.client.Post(BackendApiPath(fmt.Sprintf("/admin/api_keys/%s/status", SerializePathParameter(hashedKey, PathParameterSpec{Name: "hashedKey", Style: "simple", Explode: false}))), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.ApiKeysStatusResponse
        return zero, err
    }
    return decodeResult[sdktypes.ApiKeysStatusResponse](raw)
}

// listBillingEvents
func (a *AdminApi) BillingEventsList(pageSize *int, cursor *string, page *int, q *string) (sdktypes.SdkWorkListResponse, error) {
    query := BuildQueryString([]QueryParameterSpec{
        {Name: "page_size", Value: func() interface{} { if pageSize == nil { return nil }; return *pageSize }(), Style: "form", Explode: true, AllowReserved: false},
        {Name: "cursor", Value: func() interface{} { if cursor == nil { return nil }; return *cursor }(), Style: "form", Explode: true, AllowReserved: false},
        {Name: "page", Value: func() interface{} { if page == nil { return nil }; return *page }(), Style: "form", Explode: true, AllowReserved: false},
        {Name: "q", Value: func() interface{} { if q == nil { return nil }; return *q }(), Style: "form", Explode: true, AllowReserved: false},
    })
    raw, err := a.client.Get(AppendQueryString(BackendApiPath("/admin/billing/events"), query), nil, nil)
    if err != nil {
        var zero sdktypes.SdkWorkListResponse
        return zero, err
    }
    return decodeResult[sdktypes.SdkWorkListResponse](raw)
}

// getBillingEventSummary
func (a *AdminApi) BillingEventsSummaryRetrieve() (sdktypes.BillingEventsSummaryRetrieveResponse, error) {
    raw, err := a.client.Get(BackendApiPath("/admin/billing/events/summary"), nil, nil)
    if err != nil {
        var zero sdktypes.BillingEventsSummaryRetrieveResponse
        return zero, err
    }
    return decodeResult[sdktypes.BillingEventsSummaryRetrieveResponse](raw)
}

// getBillingSummary
func (a *AdminApi) BillingSummaryRetrieve() (sdktypes.BillingSummaryRetrieveResponse, error) {
    raw, err := a.client.Get(BackendApiPath("/admin/billing/summary"), nil, nil)
    if err != nil {
        var zero sdktypes.BillingSummaryRetrieveResponse
        return zero, err
    }
    return decodeResult[sdktypes.BillingSummaryRetrieveResponse](raw)
}

// listChannelModels
func (a *AdminApi) ChannelModelsList(pageSize *int, cursor *string, page *int, q *string) (sdktypes.SdkWorkListResponse, error) {
    query := BuildQueryString([]QueryParameterSpec{
        {Name: "page_size", Value: func() interface{} { if pageSize == nil { return nil }; return *pageSize }(), Style: "form", Explode: true, AllowReserved: false},
        {Name: "cursor", Value: func() interface{} { if cursor == nil { return nil }; return *cursor }(), Style: "form", Explode: true, AllowReserved: false},
        {Name: "page", Value: func() interface{} { if page == nil { return nil }; return *page }(), Style: "form", Explode: true, AllowReserved: false},
        {Name: "q", Value: func() interface{} { if q == nil { return nil }; return *q }(), Style: "form", Explode: true, AllowReserved: false},
    })
    raw, err := a.client.Get(AppendQueryString(BackendApiPath("/admin/channel_models"), query), nil, nil)
    if err != nil {
        var zero sdktypes.SdkWorkListResponse
        return zero, err
    }
    return decodeResult[sdktypes.SdkWorkListResponse](raw)
}

// saveChannelModel
func (a *AdminApi) ChannelModelsCreate(body sdktypes.LooseJsonObject) (sdktypes.ChannelModelsCreateResponse201, error) {
    raw, err := a.client.Post(BackendApiPath("/admin/channel_models"), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.ChannelModelsCreateResponse201
        return zero, err
    }
    return decodeResult[sdktypes.ChannelModelsCreateResponse201](raw)
}

// deleteChannelModel
func (a *AdminApi) ChannelModelsDelete(channelId string, modelId string) (struct{}, error) {
    raw, err := a.client.Delete(BackendApiPath(fmt.Sprintf("/admin/channel_models/%s/models/%s", SerializePathParameter(channelId, PathParameterSpec{Name: "channelId", Style: "simple", Explode: false}), SerializePathParameter(modelId, PathParameterSpec{Name: "modelId", Style: "simple", Explode: false}))), nil, nil)
    if err != nil {
        var zero struct{}
        return zero, err
    }
    return decodeResult[struct{}](raw)
}

// listChannels
func (a *AdminApi) ChannelsList(pageSize *int, cursor *string, page *int, q *string) (sdktypes.SdkWorkListResponse, error) {
    query := BuildQueryString([]QueryParameterSpec{
        {Name: "page_size", Value: func() interface{} { if pageSize == nil { return nil }; return *pageSize }(), Style: "form", Explode: true, AllowReserved: false},
        {Name: "cursor", Value: func() interface{} { if cursor == nil { return nil }; return *cursor }(), Style: "form", Explode: true, AllowReserved: false},
        {Name: "page", Value: func() interface{} { if page == nil { return nil }; return *page }(), Style: "form", Explode: true, AllowReserved: false},
        {Name: "q", Value: func() interface{} { if q == nil { return nil }; return *q }(), Style: "form", Explode: true, AllowReserved: false},
    })
    raw, err := a.client.Get(AppendQueryString(BackendApiPath("/admin/channels"), query), nil, nil)
    if err != nil {
        var zero sdktypes.SdkWorkListResponse
        return zero, err
    }
    return decodeResult[sdktypes.SdkWorkListResponse](raw)
}

// saveChannel
func (a *AdminApi) ChannelsCreate(body sdktypes.LooseJsonObject) (sdktypes.ChannelsCreateResponse201, error) {
    raw, err := a.client.Post(BackendApiPath("/admin/channels"), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.ChannelsCreateResponse201
        return zero, err
    }
    return decodeResult[sdktypes.ChannelsCreateResponse201](raw)
}

// deleteChannel
func (a *AdminApi) ChannelsDelete(channelId string) (struct{}, error) {
    raw, err := a.client.Delete(BackendApiPath(fmt.Sprintf("/admin/channels/%s", SerializePathParameter(channelId, PathParameterSpec{Name: "channelId", Style: "simple", Explode: false}))), nil, nil)
    if err != nil {
        var zero struct{}
        return zero, err
    }
    return decodeResult[struct{}](raw)
}

// listCredentials
func (a *AdminApi) CredentialsList(pageSize *int, cursor *string, page *int, q *string) (sdktypes.SdkWorkListResponse, error) {
    query := BuildQueryString([]QueryParameterSpec{
        {Name: "page_size", Value: func() interface{} { if pageSize == nil { return nil }; return *pageSize }(), Style: "form", Explode: true, AllowReserved: false},
        {Name: "cursor", Value: func() interface{} { if cursor == nil { return nil }; return *cursor }(), Style: "form", Explode: true, AllowReserved: false},
        {Name: "page", Value: func() interface{} { if page == nil { return nil }; return *page }(), Style: "form", Explode: true, AllowReserved: false},
        {Name: "q", Value: func() interface{} { if q == nil { return nil }; return *q }(), Style: "form", Explode: true, AllowReserved: false},
    })
    raw, err := a.client.Get(AppendQueryString(BackendApiPath("/admin/credentials"), query), nil, nil)
    if err != nil {
        var zero sdktypes.SdkWorkListResponse
        return zero, err
    }
    return decodeResult[sdktypes.SdkWorkListResponse](raw)
}

// saveCredential
func (a *AdminApi) CredentialsCreate(body sdktypes.LooseJsonObject) (sdktypes.CredentialsCreateResponse201, error) {
    raw, err := a.client.Post(BackendApiPath("/admin/credentials"), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.CredentialsCreateResponse201
        return zero, err
    }
    return decodeResult[sdktypes.CredentialsCreateResponse201](raw)
}

// deleteCredential
func (a *AdminApi) CredentialsProvidersKeysDelete(tenantId string, providerId string, keyReference string) (struct{}, error) {
    raw, err := a.client.Delete(BackendApiPath(fmt.Sprintf("/admin/credentials/%s/providers/%s/keys/%s", SerializePathParameter(tenantId, PathParameterSpec{Name: "tenantId", Style: "simple", Explode: false}), SerializePathParameter(providerId, PathParameterSpec{Name: "providerId", Style: "simple", Explode: false}), SerializePathParameter(keyReference, PathParameterSpec{Name: "keyReference", Style: "simple", Explode: false}))), nil, nil)
    if err != nil {
        var zero struct{}
        return zero, err
    }
    return decodeResult[struct{}](raw)
}

// reloadExtensionRuntimes
func (a *AdminApi) ExtensionsRuntimeReloadsCreate(body sdktypes.LooseJsonObject) (sdktypes.ExtensionsRuntimeReloadsCreateResponse201, error) {
    raw, err := a.client.Post(BackendApiPath("/admin/extensions/runtime_reloads"), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.ExtensionsRuntimeReloadsCreateResponse201
        return zero, err
    }
    return decodeResult[sdktypes.ExtensionsRuntimeReloadsCreateResponse201](raw)
}

// listRuntimeStatuses
func (a *AdminApi) ExtensionsRuntimeStatusesList(pageSize *int, cursor *string, page *int, q *string) (sdktypes.SdkWorkListResponse, error) {
    query := BuildQueryString([]QueryParameterSpec{
        {Name: "page_size", Value: func() interface{} { if pageSize == nil { return nil }; return *pageSize }(), Style: "form", Explode: true, AllowReserved: false},
        {Name: "cursor", Value: func() interface{} { if cursor == nil { return nil }; return *cursor }(), Style: "form", Explode: true, AllowReserved: false},
        {Name: "page", Value: func() interface{} { if page == nil { return nil }; return *page }(), Style: "form", Explode: true, AllowReserved: false},
        {Name: "q", Value: func() interface{} { if q == nil { return nil }; return *q }(), Style: "form", Explode: true, AllowReserved: false},
    })
    raw, err := a.client.Get(AppendQueryString(BackendApiPath("/admin/extensions/runtime_statuses"), query), nil, nil)
    if err != nil {
        var zero sdktypes.SdkWorkListResponse
        return zero, err
    }
    return decodeResult[sdktypes.SdkWorkListResponse](raw)
}

// listRateLimitPolicies
func (a *AdminApi) GatewayRateLimitPoliciesList(pageSize *int, cursor *string, page *int, q *string) (sdktypes.SdkWorkListResponse, error) {
    query := BuildQueryString([]QueryParameterSpec{
        {Name: "page_size", Value: func() interface{} { if pageSize == nil { return nil }; return *pageSize }(), Style: "form", Explode: true, AllowReserved: false},
        {Name: "cursor", Value: func() interface{} { if cursor == nil { return nil }; return *cursor }(), Style: "form", Explode: true, AllowReserved: false},
        {Name: "page", Value: func() interface{} { if page == nil { return nil }; return *page }(), Style: "form", Explode: true, AllowReserved: false},
        {Name: "q", Value: func() interface{} { if q == nil { return nil }; return *q }(), Style: "form", Explode: true, AllowReserved: false},
    })
    raw, err := a.client.Get(AppendQueryString(BackendApiPath("/admin/gateway/rate_limit_policies"), query), nil, nil)
    if err != nil {
        var zero sdktypes.SdkWorkListResponse
        return zero, err
    }
    return decodeResult[sdktypes.SdkWorkListResponse](raw)
}

// createRateLimitPolicy
func (a *AdminApi) GatewayRateLimitPoliciesCreate(body sdktypes.LooseJsonObject) (sdktypes.GatewayRateLimitPoliciesCreateResponse201, error) {
    raw, err := a.client.Post(BackendApiPath("/admin/gateway/rate_limit_policies"), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.GatewayRateLimitPoliciesCreateResponse201
        return zero, err
    }
    return decodeResult[sdktypes.GatewayRateLimitPoliciesCreateResponse201](raw)
}

// listRateLimitWindows
func (a *AdminApi) GatewayRateLimitWindowsList(pageSize *int, cursor *string, page *int, q *string) (sdktypes.SdkWorkListResponse, error) {
    query := BuildQueryString([]QueryParameterSpec{
        {Name: "page_size", Value: func() interface{} { if pageSize == nil { return nil }; return *pageSize }(), Style: "form", Explode: true, AllowReserved: false},
        {Name: "cursor", Value: func() interface{} { if cursor == nil { return nil }; return *cursor }(), Style: "form", Explode: true, AllowReserved: false},
        {Name: "page", Value: func() interface{} { if page == nil { return nil }; return *page }(), Style: "form", Explode: true, AllowReserved: false},
        {Name: "q", Value: func() interface{} { if q == nil { return nil }; return *q }(), Style: "form", Explode: true, AllowReserved: false},
    })
    raw, err := a.client.Get(AppendQueryString(BackendApiPath("/admin/gateway/rate_limit_windows"), query), nil, nil)
    if err != nil {
        var zero sdktypes.SdkWorkListResponse
        return zero, err
    }
    return decodeResult[sdktypes.SdkWorkListResponse](raw)
}

// listMarketingCampaigns
func (a *AdminApi) MarketingCampaignsList(pageSize *int, cursor *string, page *int, q *string) (sdktypes.SdkWorkListResponse, error) {
    query := BuildQueryString([]QueryParameterSpec{
        {Name: "page_size", Value: func() interface{} { if pageSize == nil { return nil }; return *pageSize }(), Style: "form", Explode: true, AllowReserved: false},
        {Name: "cursor", Value: func() interface{} { if cursor == nil { return nil }; return *cursor }(), Style: "form", Explode: true, AllowReserved: false},
        {Name: "page", Value: func() interface{} { if page == nil { return nil }; return *page }(), Style: "form", Explode: true, AllowReserved: false},
        {Name: "q", Value: func() interface{} { if q == nil { return nil }; return *q }(), Style: "form", Explode: true, AllowReserved: false},
    })
    raw, err := a.client.Get(AppendQueryString(BackendApiPath("/admin/marketing/campaigns"), query), nil, nil)
    if err != nil {
        var zero sdktypes.SdkWorkListResponse
        return zero, err
    }
    return decodeResult[sdktypes.SdkWorkListResponse](raw)
}

// saveMarketingCampaign
func (a *AdminApi) MarketingCampaignsCreate(body sdktypes.LooseJsonObject) (sdktypes.MarketingCampaignsCreateResponse201, error) {
    raw, err := a.client.Post(BackendApiPath("/admin/marketing/campaigns"), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.MarketingCampaignsCreateResponse201
        return zero, err
    }
    return decodeResult[sdktypes.MarketingCampaignsCreateResponse201](raw)
}

// updateMarketingCampaignStatus
func (a *AdminApi) MarketingCampaignsStatus(marketingCampaignId string, body sdktypes.LooseJsonObject) (sdktypes.MarketingCampaignsStatusResponse, error) {
    raw, err := a.client.Post(BackendApiPath(fmt.Sprintf("/admin/marketing/campaigns/%s/status", SerializePathParameter(marketingCampaignId, PathParameterSpec{Name: "marketingCampaignId", Style: "simple", Explode: false}))), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.MarketingCampaignsStatusResponse
        return zero, err
    }
    return decodeResult[sdktypes.MarketingCampaignsStatusResponse](raw)
}

// listModelPrices
func (a *AdminApi) ModelPricesList(pageSize *int, cursor *string, page *int, q *string) (sdktypes.SdkWorkListResponse, error) {
    query := BuildQueryString([]QueryParameterSpec{
        {Name: "page_size", Value: func() interface{} { if pageSize == nil { return nil }; return *pageSize }(), Style: "form", Explode: true, AllowReserved: false},
        {Name: "cursor", Value: func() interface{} { if cursor == nil { return nil }; return *cursor }(), Style: "form", Explode: true, AllowReserved: false},
        {Name: "page", Value: func() interface{} { if page == nil { return nil }; return *page }(), Style: "form", Explode: true, AllowReserved: false},
        {Name: "q", Value: func() interface{} { if q == nil { return nil }; return *q }(), Style: "form", Explode: true, AllowReserved: false},
    })
    raw, err := a.client.Get(AppendQueryString(BackendApiPath("/admin/model_prices"), query), nil, nil)
    if err != nil {
        var zero sdktypes.SdkWorkListResponse
        return zero, err
    }
    return decodeResult[sdktypes.SdkWorkListResponse](raw)
}

// saveModelPrice
func (a *AdminApi) ModelPricesCreate(body sdktypes.LooseJsonObject) (sdktypes.ModelPricesCreateResponse201, error) {
    raw, err := a.client.Post(BackendApiPath("/admin/model_prices"), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.ModelPricesCreateResponse201
        return zero, err
    }
    return decodeResult[sdktypes.ModelPricesCreateResponse201](raw)
}

// deleteModelPrice
func (a *AdminApi) ModelPricesProvidersDelete(channelId string, modelId string, proxyProviderId string) (struct{}, error) {
    raw, err := a.client.Delete(BackendApiPath(fmt.Sprintf("/admin/model_prices/%s/models/%s/providers/%s", SerializePathParameter(channelId, PathParameterSpec{Name: "channelId", Style: "simple", Explode: false}), SerializePathParameter(modelId, PathParameterSpec{Name: "modelId", Style: "simple", Explode: false}), SerializePathParameter(proxyProviderId, PathParameterSpec{Name: "proxyProviderId", Style: "simple", Explode: false}))), nil, nil)
    if err != nil {
        var zero struct{}
        return zero, err
    }
    return decodeResult[struct{}](raw)
}

// listModels
func (a *AdminApi) ModelsList(pageSize *int, cursor *string, page *int, q *string) (sdktypes.SdkWorkListResponse, error) {
    query := BuildQueryString([]QueryParameterSpec{
        {Name: "page_size", Value: func() interface{} { if pageSize == nil { return nil }; return *pageSize }(), Style: "form", Explode: true, AllowReserved: false},
        {Name: "cursor", Value: func() interface{} { if cursor == nil { return nil }; return *cursor }(), Style: "form", Explode: true, AllowReserved: false},
        {Name: "page", Value: func() interface{} { if page == nil { return nil }; return *page }(), Style: "form", Explode: true, AllowReserved: false},
        {Name: "q", Value: func() interface{} { if q == nil { return nil }; return *q }(), Style: "form", Explode: true, AllowReserved: false},
    })
    raw, err := a.client.Get(AppendQueryString(BackendApiPath("/admin/models"), query), nil, nil)
    if err != nil {
        var zero sdktypes.SdkWorkListResponse
        return zero, err
    }
    return decodeResult[sdktypes.SdkWorkListResponse](raw)
}

// saveModel
func (a *AdminApi) ModelsCreate(body sdktypes.LooseJsonObject) (sdktypes.ModelsCreateResponse201, error) {
    raw, err := a.client.Post(BackendApiPath("/admin/models"), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.ModelsCreateResponse201
        return zero, err
    }
    return decodeResult[sdktypes.ModelsCreateResponse201](raw)
}

// deleteModel
func (a *AdminApi) ModelsProvidersDelete(externalName string, providerId string) (struct{}, error) {
    raw, err := a.client.Delete(BackendApiPath(fmt.Sprintf("/admin/models/%s/providers/%s", SerializePathParameter(externalName, PathParameterSpec{Name: "externalName", Style: "simple", Explode: false}), SerializePathParameter(providerId, PathParameterSpec{Name: "providerId", Style: "simple", Explode: false}))), nil, nil)
    if err != nil {
        var zero struct{}
        return zero, err
    }
    return decodeResult[struct{}](raw)
}

// listProviders
func (a *AdminApi) ProvidersList(pageSize *int, cursor *string, page *int, q *string) (sdktypes.SdkWorkListResponse, error) {
    query := BuildQueryString([]QueryParameterSpec{
        {Name: "page_size", Value: func() interface{} { if pageSize == nil { return nil }; return *pageSize }(), Style: "form", Explode: true, AllowReserved: false},
        {Name: "cursor", Value: func() interface{} { if cursor == nil { return nil }; return *cursor }(), Style: "form", Explode: true, AllowReserved: false},
        {Name: "page", Value: func() interface{} { if page == nil { return nil }; return *page }(), Style: "form", Explode: true, AllowReserved: false},
        {Name: "q", Value: func() interface{} { if q == nil { return nil }; return *q }(), Style: "form", Explode: true, AllowReserved: false},
    })
    raw, err := a.client.Get(AppendQueryString(BackendApiPath("/admin/providers"), query), nil, nil)
    if err != nil {
        var zero sdktypes.SdkWorkListResponse
        return zero, err
    }
    return decodeResult[sdktypes.SdkWorkListResponse](raw)
}

// saveProvider
func (a *AdminApi) ProvidersCreate(body sdktypes.LooseJsonObject) (sdktypes.ProvidersCreateResponse201, error) {
    raw, err := a.client.Post(BackendApiPath("/admin/providers"), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.ProvidersCreateResponse201
        return zero, err
    }
    return decodeResult[sdktypes.ProvidersCreateResponse201](raw)
}

// deleteProvider
func (a *AdminApi) ProvidersDelete(providerId string) (struct{}, error) {
    raw, err := a.client.Delete(BackendApiPath(fmt.Sprintf("/admin/providers/%s", SerializePathParameter(providerId, PathParameterSpec{Name: "providerId", Style: "simple", Explode: false}))), nil, nil)
    if err != nil {
        var zero struct{}
        return zero, err
    }
    return decodeResult[struct{}](raw)
}

// listRoutingDecisionLogs
func (a *AdminApi) RoutingDecisionLogsList(pageSize *int, cursor *string, page *int, q *string) (sdktypes.SdkWorkListResponse, error) {
    query := BuildQueryString([]QueryParameterSpec{
        {Name: "page_size", Value: func() interface{} { if pageSize == nil { return nil }; return *pageSize }(), Style: "form", Explode: true, AllowReserved: false},
        {Name: "cursor", Value: func() interface{} { if cursor == nil { return nil }; return *cursor }(), Style: "form", Explode: true, AllowReserved: false},
        {Name: "page", Value: func() interface{} { if page == nil { return nil }; return *page }(), Style: "form", Explode: true, AllowReserved: false},
        {Name: "q", Value: func() interface{} { if q == nil { return nil }; return *q }(), Style: "form", Explode: true, AllowReserved: false},
    })
    raw, err := a.client.Get(AppendQueryString(BackendApiPath("/admin/routing/decision_logs"), query), nil, nil)
    if err != nil {
        var zero sdktypes.SdkWorkListResponse
        return zero, err
    }
    return decodeResult[sdktypes.SdkWorkListResponse](raw)
}

// listProviderHealthSnapshots
func (a *AdminApi) RoutingHealthSnapshotsRetrieve() (sdktypes.RoutingHealthSnapshotsRetrieveResponse, error) {
    raw, err := a.client.Get(BackendApiPath("/admin/routing/health_snapshots"), nil, nil)
    if err != nil {
        var zero sdktypes.RoutingHealthSnapshotsRetrieveResponse
        return zero, err
    }
    return decodeResult[sdktypes.RoutingHealthSnapshotsRetrieveResponse](raw)
}

// listRoutingProfiles
func (a *AdminApi) RoutingProfilesList(pageSize *int, cursor *string, page *int, q *string) (sdktypes.SdkWorkListResponse, error) {
    query := BuildQueryString([]QueryParameterSpec{
        {Name: "page_size", Value: func() interface{} { if pageSize == nil { return nil }; return *pageSize }(), Style: "form", Explode: true, AllowReserved: false},
        {Name: "cursor", Value: func() interface{} { if cursor == nil { return nil }; return *cursor }(), Style: "form", Explode: true, AllowReserved: false},
        {Name: "page", Value: func() interface{} { if page == nil { return nil }; return *page }(), Style: "form", Explode: true, AllowReserved: false},
        {Name: "q", Value: func() interface{} { if q == nil { return nil }; return *q }(), Style: "form", Explode: true, AllowReserved: false},
    })
    raw, err := a.client.Get(AppendQueryString(BackendApiPath("/admin/routing/profiles"), query), nil, nil)
    if err != nil {
        var zero sdktypes.SdkWorkListResponse
        return zero, err
    }
    return decodeResult[sdktypes.SdkWorkListResponse](raw)
}

// createRoutingProfile
func (a *AdminApi) RoutingProfilesCreate(body sdktypes.LooseJsonObject) (sdktypes.RoutingProfilesCreateResponse201, error) {
    raw, err := a.client.Post(BackendApiPath("/admin/routing/profiles"), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.RoutingProfilesCreateResponse201
        return zero, err
    }
    return decodeResult[sdktypes.RoutingProfilesCreateResponse201](raw)
}

// listCompiledRoutingSnapshots
func (a *AdminApi) RoutingSnapshotsList(pageSize *int, cursor *string, page *int, q *string) (sdktypes.SdkWorkListResponse, error) {
    query := BuildQueryString([]QueryParameterSpec{
        {Name: "page_size", Value: func() interface{} { if pageSize == nil { return nil }; return *pageSize }(), Style: "form", Explode: true, AllowReserved: false},
        {Name: "cursor", Value: func() interface{} { if cursor == nil { return nil }; return *cursor }(), Style: "form", Explode: true, AllowReserved: false},
        {Name: "page", Value: func() interface{} { if page == nil { return nil }; return *page }(), Style: "form", Explode: true, AllowReserved: false},
        {Name: "q", Value: func() interface{} { if q == nil { return nil }; return *q }(), Style: "form", Explode: true, AllowReserved: false},
    })
    raw, err := a.client.Get(AppendQueryString(BackendApiPath("/admin/routing/snapshots"), query), nil, nil)
    if err != nil {
        var zero sdktypes.SdkWorkListResponse
        return zero, err
    }
    return decodeResult[sdktypes.SdkWorkListResponse](raw)
}

// listStorageAuditTrail
func (a *AdminApi) StorageAuditList(pageSize *int, cursor *string, page *int, q *string) (sdktypes.SdkWorkListResponse, error) {
    query := BuildQueryString([]QueryParameterSpec{
        {Name: "page_size", Value: func() interface{} { if pageSize == nil { return nil }; return *pageSize }(), Style: "form", Explode: true, AllowReserved: false},
        {Name: "cursor", Value: func() interface{} { if cursor == nil { return nil }; return *cursor }(), Style: "form", Explode: true, AllowReserved: false},
        {Name: "page", Value: func() interface{} { if page == nil { return nil }; return *page }(), Style: "form", Explode: true, AllowReserved: false},
        {Name: "q", Value: func() interface{} { if q == nil { return nil }; return *q }(), Style: "form", Explode: true, AllowReserved: false},
    })
    raw, err := a.client.Get(AppendQueryString(BackendApiPath("/admin/storage/audit"), query), nil, nil)
    if err != nil {
        var zero sdktypes.SdkWorkListResponse
        return zero, err
    }
    return decodeResult[sdktypes.SdkWorkListResponse](raw)
}

// getGlobalStorageConfig
func (a *AdminApi) StorageConfigRetrieve() (sdktypes.StorageConfigRetrieveResponse, error) {
    raw, err := a.client.Get(BackendApiPath("/admin/storage/config"), nil, nil)
    if err != nil {
        var zero sdktypes.StorageConfigRetrieveResponse
        return zero, err
    }
    return decodeResult[sdktypes.StorageConfigRetrieveResponse](raw)
}

// saveGlobalStorageConfig
func (a *AdminApi) StorageConfigCreate(body sdktypes.LooseJsonObject) (sdktypes.StorageConfigCreateResponse201, error) {
    raw, err := a.client.Post(BackendApiPath("/admin/storage/config"), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.StorageConfigCreateResponse201
        return zero, err
    }
    return decodeResult[sdktypes.StorageConfigCreateResponse201](raw)
}

// getTenantStorageConfig
func (a *AdminApi) StorageConfigTenantsRetrieve(tenantId string) (sdktypes.StorageConfigTenantsRetrieveResponse, error) {
    raw, err := a.client.Get(BackendApiPath(fmt.Sprintf("/admin/storage/config/tenants/%s", SerializePathParameter(tenantId, PathParameterSpec{Name: "tenantId", Style: "simple", Explode: false}))), nil, nil)
    if err != nil {
        var zero sdktypes.StorageConfigTenantsRetrieveResponse
        return zero, err
    }
    return decodeResult[sdktypes.StorageConfigTenantsRetrieveResponse](raw)
}

// saveTenantStorageConfig
func (a *AdminApi) StorageConfigTenantsCreate(tenantId string, body sdktypes.LooseJsonObject) (sdktypes.StorageConfigTenantsCreateResponse201, error) {
    raw, err := a.client.Post(BackendApiPath(fmt.Sprintf("/admin/storage/config/tenants/%s", SerializePathParameter(tenantId, PathParameterSpec{Name: "tenantId", Style: "simple", Explode: false}))), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.StorageConfigTenantsCreateResponse201
        return zero, err
    }
    return decodeResult[sdktypes.StorageConfigTenantsCreateResponse201](raw)
}

// deleteTenantStorageConfig
func (a *AdminApi) StorageConfigTenantsDelete(tenantId string) (struct{}, error) {
    raw, err := a.client.Delete(BackendApiPath(fmt.Sprintf("/admin/storage/config/tenants/%s", SerializePathParameter(tenantId, PathParameterSpec{Name: "tenantId", Style: "simple", Explode: false}))), nil, nil)
    if err != nil {
        var zero struct{}
        return zero, err
    }
    return decodeResult[struct{}](raw)
}

// getTenantEffectiveStorageConfig
func (a *AdminApi) StorageEffectiveTenantsRetrieve(tenantId string) (sdktypes.StorageEffectiveTenantsRetrieveResponse, error) {
    raw, err := a.client.Get(BackendApiPath(fmt.Sprintf("/admin/storage/effective/tenants/%s", SerializePathParameter(tenantId, PathParameterSpec{Name: "tenantId", Style: "simple", Explode: false}))), nil, nil)
    if err != nil {
        var zero sdktypes.StorageEffectiveTenantsRetrieveResponse
        return zero, err
    }
    return decodeResult[sdktypes.StorageEffectiveTenantsRetrieveResponse](raw)
}

// listStorageProviders
func (a *AdminApi) StorageProvidersList(pageSize *int, cursor *string, page *int, q *string) (sdktypes.SdkWorkListResponse, error) {
    query := BuildQueryString([]QueryParameterSpec{
        {Name: "page_size", Value: func() interface{} { if pageSize == nil { return nil }; return *pageSize }(), Style: "form", Explode: true, AllowReserved: false},
        {Name: "cursor", Value: func() interface{} { if cursor == nil { return nil }; return *cursor }(), Style: "form", Explode: true, AllowReserved: false},
        {Name: "page", Value: func() interface{} { if page == nil { return nil }; return *page }(), Style: "form", Explode: true, AllowReserved: false},
        {Name: "q", Value: func() interface{} { if q == nil { return nil }; return *q }(), Style: "form", Explode: true, AllowReserved: false},
    })
    raw, err := a.client.Get(AppendQueryString(BackendApiPath("/admin/storage/providers"), query), nil, nil)
    if err != nil {
        var zero sdktypes.SdkWorkListResponse
        return zero, err
    }
    return decodeResult[sdktypes.SdkWorkListResponse](raw)
}

// validateGlobalStorageConfig
func (a *AdminApi) StorageValidationCreate(body sdktypes.LooseJsonObject) (sdktypes.StorageValidationCreateResponse201, error) {
    raw, err := a.client.Post(BackendApiPath("/admin/storage/validate"), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.StorageValidationCreateResponse201
        return zero, err
    }
    return decodeResult[sdktypes.StorageValidationCreateResponse201](raw)
}

// validateTenantStorageConfig
func (a *AdminApi) StorageValidationTenantsCreate(tenantId string, body sdktypes.LooseJsonObject) (sdktypes.StorageValidationTenantsCreateResponse201, error) {
    raw, err := a.client.Post(BackendApiPath(fmt.Sprintf("/admin/storage/validate/tenants/%s", SerializePathParameter(tenantId, PathParameterSpec{Name: "tenantId", Style: "simple", Explode: false}))), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.StorageValidationTenantsCreateResponse201
        return zero, err
    }
    return decodeResult[sdktypes.StorageValidationTenantsCreateResponse201](raw)
}

// listUsageRecords
func (a *AdminApi) UsageRecordsList(pageSize *int, cursor *string, page *int, q *string) (sdktypes.SdkWorkListResponse, error) {
    query := BuildQueryString([]QueryParameterSpec{
        {Name: "page_size", Value: func() interface{} { if pageSize == nil { return nil }; return *pageSize }(), Style: "form", Explode: true, AllowReserved: false},
        {Name: "cursor", Value: func() interface{} { if cursor == nil { return nil }; return *cursor }(), Style: "form", Explode: true, AllowReserved: false},
        {Name: "page", Value: func() interface{} { if page == nil { return nil }; return *page }(), Style: "form", Explode: true, AllowReserved: false},
        {Name: "q", Value: func() interface{} { if q == nil { return nil }; return *q }(), Style: "form", Explode: true, AllowReserved: false},
    })
    raw, err := a.client.Get(AppendQueryString(BackendApiPath("/admin/usage/records"), query), nil, nil)
    if err != nil {
        var zero sdktypes.SdkWorkListResponse
        return zero, err
    }
    return decodeResult[sdktypes.SdkWorkListResponse](raw)
}

// getUsageSummary
func (a *AdminApi) UsageSummaryRetrieve() (sdktypes.UsageSummaryRetrieveResponse, error) {
    raw, err := a.client.Get(BackendApiPath("/admin/usage/summary"), nil, nil)
    if err != nil {
        var zero sdktypes.UsageSummaryRetrieveResponse
        return zero, err
    }
    return decodeResult[sdktypes.UsageSummaryRetrieveResponse](raw)
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
type QueryParameterSpec struct {
    Name          string
    Value         interface{}
    Style         string
    Explode       bool
    AllowReserved bool
    ContentType   string
}

func BuildQueryString(parameters []QueryParameterSpec) string {
    pairs := make([]string, 0)
    for _, parameter := range parameters {
        AppendSerializedParameter(&pairs, parameter)
    }
    return strings.Join(pairs, "&")
}

func AppendSerializedParameter(pairs *[]string, parameter QueryParameterSpec) {
    if parameter.Value == nil {
        return
    }

    if parameter.ContentType != "" {
        encoded, _ := json.Marshal(parameter.Value)
        *pairs = append(*pairs, url.QueryEscape(parameter.Name)+"="+EncodeQueryValue(string(encoded), parameter.AllowReserved))
        return
    }

    style := parameter.Style
    if style == "" {
        style = "form"
    }

    switch value := parameter.Value.(type) {
    case []string:
        AppendArrayParameter(pairs, parameter.Name, stringSliceToInterface(value), style, parameter.Explode, parameter.AllowReserved)
    case []int:
        AppendArrayParameter(pairs, parameter.Name, intSliceToInterface(value), style, parameter.Explode, parameter.AllowReserved)
    case []interface{}:
        AppendArrayParameter(pairs, parameter.Name, value, style, parameter.Explode, parameter.AllowReserved)
    case map[string]int:
        AppendObjectParameter(pairs, parameter.Name, intMapToInterface(value), style, parameter.Explode, parameter.AllowReserved)
    case map[string]string:
        AppendObjectParameter(pairs, parameter.Name, stringMapToInterface(value), style, parameter.Explode, parameter.AllowReserved)
    case map[string]interface{}:
        if style == "deepObject" {
            AppendDeepObjectParameter(pairs, parameter.Name, value, parameter.AllowReserved)
        } else {
            AppendObjectParameter(pairs, parameter.Name, value, style, parameter.Explode, parameter.AllowReserved)
        }
    default:
        *pairs = append(*pairs, url.QueryEscape(parameter.Name)+"="+EncodeQueryValue(fmt.Sprint(value), parameter.AllowReserved))
    }
}

func AppendArrayParameter(pairs *[]string, name string, value []interface{}, style string, explode bool, allowReserved bool) {
    values := make([]string, 0, len(value))
    for _, item := range value {
        if item != nil {
            values = append(values, fmt.Sprint(item))
        }
    }
    if len(values) == 0 {
        return
    }
    if style == "form" && explode {
        for _, item := range values {
            *pairs = append(*pairs, url.QueryEscape(name)+"="+EncodeQueryValue(item, allowReserved))
        }
        return
    }
    *pairs = append(*pairs, url.QueryEscape(name)+"="+EncodeQueryValue(strings.Join(values, ","), allowReserved))
}

func AppendObjectParameter(pairs *[]string, name string, value map[string]interface{}, style string, explode bool, allowReserved bool) {
    entries := make([]string, 0, len(value)*2)
    for key, item := range value {
        if item == nil {
            continue
        }
        if style == "form" && explode {
            *pairs = append(*pairs, url.QueryEscape(key)+"="+EncodeQueryValue(fmt.Sprint(item), allowReserved))
            continue
        }
        entries = append(entries, key, fmt.Sprint(item))
    }
    if len(entries) == 0 {
        return
    }
    if !(style == "form" && explode) {
        *pairs = append(*pairs, url.QueryEscape(name)+"="+EncodeQueryValue(strings.Join(entries, ","), allowReserved))
    }
}

func AppendDeepObjectParameter(pairs *[]string, name string, value map[string]interface{}, allowReserved bool) {
    for key, item := range value {
        if item == nil {
            continue
        }
        *pairs = append(*pairs, url.QueryEscape(fmt.Sprintf("%s[%s]", name, key))+"="+EncodeQueryValue(fmt.Sprint(item), allowReserved))
    }
}

func EncodeQueryValue(value string, allowReserved bool) string {
    encoded := url.QueryEscape(value)
    if !allowReserved {
        return encoded
    }
    replacements := map[string]string{
        "%3A": ":", "%2F": "/", "%3F": "?", "%23": "#",
        "%5B": "[", "%5D": "]", "%40": "@", "%21": "!",
        "%24": "$", "%26": "&", "%27": "'", "%28": "(",
        "%29": ")", "%2A": "*", "%2B": "+", "%2C": ",",
        "%3B": ";", "%3D": "=",
    }
    for escaped, reserved := range replacements {
        encoded = strings.ReplaceAll(encoded, escaped, reserved)
    }
    return encoded
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
