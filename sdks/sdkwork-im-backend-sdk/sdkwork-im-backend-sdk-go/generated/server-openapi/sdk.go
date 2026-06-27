package backend

import (
    "github.com/sdkwork/im-backend-api-generated/api"
    sdkhttp "github.com/sdkwork/im-backend-api-generated/http"
)

type SdkworkImBackendClient struct {
    http *sdkhttp.Client
    Ops *api.OpsApi
    Audit *api.AuditApi
    Automation *api.AutomationApi
    Control *api.ControlApi
    Admin *api.AdminApi
}

func NewSdkworkImBackendClient(baseURL string) *SdkworkImBackendClient {
    cfg := sdkhttp.NewDefaultConfig(baseURL)
    return NewSdkworkImBackendClientWithConfig(cfg)
}

func NewSdkworkImBackendClientWithConfig(config sdkhttp.Config) *SdkworkImBackendClient {
    client := sdkhttp.NewClient(config)
    return &SdkworkImBackendClient{
        http: client,
        Ops: api.NewOpsApi(client),
        Audit: api.NewAuditApi(client),
        Automation: api.NewAutomationApi(client),
        Control: api.NewControlApi(client),
        Admin: api.NewAdminApi(client),
    }
}

func (c *SdkworkImBackendClient) SetAuthToken(token string) *SdkworkImBackendClient {
    c.http.SetAuthToken(token)
    return c
}

func (c *SdkworkImBackendClient) SetAccessToken(token string) *SdkworkImBackendClient {
    c.http.SetAccessToken(token)
    return c
}

func (c *SdkworkImBackendClient) SetHeader(key string, value string) *SdkworkImBackendClient {
    c.http.SetHeader(key, value)
    return c
}

func (c *SdkworkImBackendClient) Http() *sdkhttp.Client {
    return c.http
}

type SdkworkBackendClient = SdkworkImBackendClient

func NewSdkworkBackendClient(baseURL string) *SdkworkBackendClient {
    return NewSdkworkImBackendClient(baseURL)
}

func NewSdkworkBackendClientWithConfig(config sdkhttp.Config) *SdkworkBackendClient {
    return NewSdkworkImBackendClientWithConfig(config)
}
