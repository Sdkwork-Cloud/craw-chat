using System;
using System.Collections.Generic;
using System.Threading.Tasks;
using Sdkwork.Im.BackendApi.Generated.Models;
using SdkHttpClient = Sdkwork.Im.BackendApi.Generated.Http.HttpClient;

namespace Sdkwork.Im.BackendApi.Generated.Api
{
    public class AuditApi
    {
        private readonly SdkHttpClient _client;

        public AuditApi(SdkHttpClient client)
        {
            _client = client;
        }

        /// <summary>
        /// List audit records
        /// </summary>
        public async Task<Dictionary<string, object>?> RecordsListAsync()
        {
            return await _client.GetAsync<Dictionary<string, object>>(ApiPaths.BackendPath("/audit/records"));
        }

        /// <summary>
        /// Record audit anchor
        /// </summary>
        public async Task<Dictionary<string, object>?> RecordsCreateAsync()
        {
            return await _client.PostAsync<Dictionary<string, object>>(ApiPaths.BackendPath("/audit/records"), null);
        }

        /// <summary>
        /// Export audit bundle
        /// </summary>
        public async Task<Dictionary<string, object>?> ExportRetrieveAsync()
        {
            return await _client.GetAsync<Dictionary<string, object>>(ApiPaths.BackendPath("/audit/export"));
        }



    }
}
