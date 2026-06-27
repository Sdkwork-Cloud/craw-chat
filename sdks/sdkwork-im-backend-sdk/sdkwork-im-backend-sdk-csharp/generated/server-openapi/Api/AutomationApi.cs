using System;
using System.Collections.Generic;
using System.Threading.Tasks;
using Sdkwork.Im.BackendApi.Generated.Models;
using SdkHttpClient = Sdkwork.Im.BackendApi.Generated.Http.HttpClient;

namespace Sdkwork.Im.BackendApi.Generated.Api
{
    public class AutomationApi
    {
        private readonly SdkHttpClient _client;

        public AutomationApi(SdkHttpClient client)
        {
            _client = client;
        }

        /// <summary>
        /// Retrieve automation governance
        /// </summary>
        public async Task<Dictionary<string, object>?> GovernanceRetrieveAsync()
        {
            return await _client.GetAsync<Dictionary<string, object>>(ApiPaths.BackendPath("/automation/governance"));
        }



    }
}
