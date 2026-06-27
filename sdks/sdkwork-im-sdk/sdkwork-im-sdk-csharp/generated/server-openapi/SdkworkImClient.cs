using System;
using SDKwork.Common.Core;
using SdkHttpClient = Sdkwork.Im.Sdk.Generated.Http.HttpClient;
using Sdkwork.Im.Sdk.Generated.Api;

namespace Sdkwork.Im.Sdk.Generated
{
    public class SdkworkImClient
    {
        private readonly SdkHttpClient _httpClient;

        public PresenceApi Presence { get; }
        public RealtimeApi Realtime { get; }
        public CallsApi Calls { get; }
        public SocialApi Social { get; }
        public ChatApi Chat { get; }
        public StreamsApi Streams { get; }
        public SpacesApi Spaces { get; }

        public SdkworkImClient(string baseUrl)
        {
            _httpClient = new SdkHttpClient(baseUrl);
            Presence = new PresenceApi(_httpClient);
            Realtime = new RealtimeApi(_httpClient);
            Calls = new CallsApi(_httpClient);
            Social = new SocialApi(_httpClient);
            Chat = new ChatApi(_httpClient);
            Streams = new StreamsApi(_httpClient);
            Spaces = new SpacesApi(_httpClient);
        }

        public SdkworkImClient(SdkConfig config)
        {
            _httpClient = new SdkHttpClient(config);
            Presence = new PresenceApi(_httpClient);
            Realtime = new RealtimeApi(_httpClient);
            Calls = new CallsApi(_httpClient);
            Social = new SocialApi(_httpClient);
            Chat = new ChatApi(_httpClient);
            Streams = new StreamsApi(_httpClient);
            Spaces = new SpacesApi(_httpClient);
        }
        public SdkworkImClient SetAuthToken(string token)
        {
            _httpClient.SetAuthToken(token);
            return this;
        }

        public SdkworkImClient SetAccessToken(string token)
        {
            _httpClient.SetAccessToken(token);
            return this;
        }

        public SdkworkImClient SetHeader(string key, string value)
        {
            _httpClient.SetHeader(key, value);
            return this;
        }
    }
}
