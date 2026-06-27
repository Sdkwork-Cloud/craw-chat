package app

import (
    "github.com/sdkwork/im-app-api-generated/api"
    sdkhttp "github.com/sdkwork/im-app-api-generated/http"
)

type SdkworkImAppClient struct {
    http *sdkhttp.Client
    Automation *api.AutomationApi
    Notification *api.NotificationApi
    Portal *api.PortalApi
    Provider *api.ProviderApi
}

func NewSdkworkImAppClient(baseURL string) *SdkworkImAppClient {
    cfg := sdkhttp.NewDefaultConfig(baseURL)
    return NewSdkworkImAppClientWithConfig(cfg)
}

func NewSdkworkImAppClientWithConfig(config sdkhttp.Config) *SdkworkImAppClient {
    client := sdkhttp.NewClient(config)
    return &SdkworkImAppClient{
        http: client,
        Automation: api.NewAutomationApi(client),
        Notification: api.NewNotificationApi(client),
        Portal: api.NewPortalApi(client),
        Provider: api.NewProviderApi(client),
    }
}

func (c *SdkworkImAppClient) SetAuthToken(token string) *SdkworkImAppClient {
    c.http.SetAuthToken(token)
    return c
}

func (c *SdkworkImAppClient) SetAccessToken(token string) *SdkworkImAppClient {
    c.http.SetAccessToken(token)
    return c
}

func (c *SdkworkImAppClient) SetHeader(key string, value string) *SdkworkImAppClient {
    c.http.SetHeader(key, value)
    return c
}

func (c *SdkworkImAppClient) Http() *sdkhttp.Client {
    return c.http
}

type SdkworkAppClient = SdkworkImAppClient

func NewSdkworkAppClient(baseURL string) *SdkworkAppClient {
    return NewSdkworkImAppClient(baseURL)
}

func NewSdkworkAppClientWithConfig(config sdkhttp.Config) *SdkworkAppClient {
    return NewSdkworkImAppClientWithConfig(config)
}
