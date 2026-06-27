using System;
using System.Collections.Generic;
using System.Threading.Tasks;
using Sdkwork.Im.AppApi.Generated.Models;
using SdkHttpClient = Sdkwork.Im.AppApi.Generated.Http.HttpClient;

namespace Sdkwork.Im.AppApi.Generated.Api
{
    public class ProviderApi
    {
        private readonly SdkHttpClient _client;

        public ProviderApi(SdkHttpClient client)
        {
            _client = client;
        }

        /// <summary>
        /// Retrieve media provider health
        /// </summary>
        public async Task<Dictionary<string, object>?> MediaHealthRetrieveAsync()
        {
            return await _client.GetAsync<Dictionary<string, object>>(ApiPaths.AppPath("/media/provider_health"));
        }

        /// <summary>
        /// Retrieve principal-profile provider health
        /// </summary>
        public async Task<Dictionary<string, object>?> PrincipalProfileHealthRetrieveAsync()
        {
            return await _client.GetAsync<Dictionary<string, object>>(ApiPaths.AppPath("/principal/profiles/provider_health"));
        }



    }
}
