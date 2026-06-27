using System;
using SDKwork.Common.Core;
using SdkHttpClient = Sdkwork.Im.BackendApi.Generated.Http.HttpClient;
using Sdkwork.Im.BackendApi.Generated.Api;

namespace Sdkwork.Im.BackendApi.Generated
{
    public class SdkworkImBackendClient
    {
        private readonly SdkHttpClient _httpClient;

        public OpsApi Ops { get; }
        public AuditApi Audit { get; }
        public AutomationApi Automation { get; }
        public ControlApi Control { get; }
        public AdminApi Admin { get; }

        public SdkworkImBackendClient(string baseUrl)
        {
            _httpClient = new SdkHttpClient(baseUrl);
            Ops = new OpsApi(_httpClient);
            Audit = new AuditApi(_httpClient);
            Automation = new AutomationApi(_httpClient);
            Control = new ControlApi(_httpClient);
            Admin = new AdminApi(_httpClient);
        }

        public SdkworkImBackendClient(SdkConfig config)
        {
            _httpClient = new SdkHttpClient(config);
            Ops = new OpsApi(_httpClient);
            Audit = new AuditApi(_httpClient);
            Automation = new AutomationApi(_httpClient);
            Control = new ControlApi(_httpClient);
            Admin = new AdminApi(_httpClient);
        }
        public SdkworkImBackendClient SetAuthToken(string token)
        {
            _httpClient.SetAuthToken(token);
            return this;
        }

        public SdkworkImBackendClient SetAccessToken(string token)
        {
            _httpClient.SetAccessToken(token);
            return this;
        }

        public SdkworkImBackendClient SetHeader(string key, string value)
        {
            _httpClient.SetHeader(key, value);
            return this;
        }
    }
}
