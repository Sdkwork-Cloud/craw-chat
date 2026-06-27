package im

import (
    "github.com/sdkwork/im-sdk-generated/api"
    sdkhttp "github.com/sdkwork/im-sdk-generated/http"
)

type SdkworkImClient struct {
    http *sdkhttp.Client
    Presence *api.PresenceApi
    Realtime *api.RealtimeApi
    Calls *api.CallsApi
    Social *api.SocialApi
    Chat *api.ChatApi
    Streams *api.StreamsApi
    Spaces *api.SpacesApi
}

func NewSdkworkImClient(baseURL string) *SdkworkImClient {
    cfg := sdkhttp.NewDefaultConfig(baseURL)
    return NewSdkworkImClientWithConfig(cfg)
}

func NewSdkworkImClientWithConfig(config sdkhttp.Config) *SdkworkImClient {
    client := sdkhttp.NewClient(config)
    return &SdkworkImClient{
        http: client,
        Presence: api.NewPresenceApi(client),
        Realtime: api.NewRealtimeApi(client),
        Calls: api.NewCallsApi(client),
        Social: api.NewSocialApi(client),
        Chat: api.NewChatApi(client),
        Streams: api.NewStreamsApi(client),
        Spaces: api.NewSpacesApi(client),
    }
}

func (c *SdkworkImClient) SetAuthToken(token string) *SdkworkImClient {
    c.http.SetAuthToken(token)
    return c
}

func (c *SdkworkImClient) SetAccessToken(token string) *SdkworkImClient {
    c.http.SetAccessToken(token)
    return c
}

func (c *SdkworkImClient) SetHeader(key string, value string) *SdkworkImClient {
    c.http.SetHeader(key, value)
    return c
}

func (c *SdkworkImClient) Http() *sdkhttp.Client {
    return c.http
}
