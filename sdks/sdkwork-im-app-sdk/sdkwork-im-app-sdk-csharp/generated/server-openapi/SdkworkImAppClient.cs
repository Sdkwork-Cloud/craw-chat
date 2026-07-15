using System;
using SDKwork.Common.Core;
using SdkHttpClient = Sdkwork.Im.AppApi.Generated.Http.HttpClient;
using Sdkwork.Im.AppApi.Generated.Api;

namespace Sdkwork.Im.AppApi.Generated
{
    public class SdkworkImAppClient
    {
        private readonly SdkHttpClient _httpClient;

        public AutomationApi Automation { get; }
        public NotificationsApi Notifications { get; }
        public PortalApi Portal { get; }
        public ProviderApi Provider { get; }
        public ChatApi Chat { get; }

        public SdkworkImAppClient(string baseUrl)
        {
            _httpClient = new SdkHttpClient(baseUrl);
            Automation = new AutomationApi(_httpClient);
            Notifications = new NotificationsApi(_httpClient);
            Portal = new PortalApi(_httpClient);
            Provider = new ProviderApi(_httpClient);
            Chat = new ChatApi(_httpClient);
        }

        public SdkworkImAppClient(SdkConfig config)
        {
            _httpClient = new SdkHttpClient(config);
            Automation = new AutomationApi(_httpClient);
            Notifications = new NotificationsApi(_httpClient);
            Portal = new PortalApi(_httpClient);
            Provider = new ProviderApi(_httpClient);
            Chat = new ChatApi(_httpClient);
        }
        public SdkworkImAppClient SetAuthToken(string token)
        {
            _httpClient.SetAuthToken(token);
            return this;
        }

        public SdkworkImAppClient SetAccessToken(string token)
        {
            _httpClient.SetAccessToken(token);
            return this;
        }

        public SdkworkImAppClient SetHeader(string key, string value)
        {
            _httpClient.SetHeader(key, value);
            return this;
        }
    }
}
