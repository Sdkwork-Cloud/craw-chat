using System;
using System.Collections.Generic;
using System.Threading.Tasks;
using Sdkwork.Im.BackendApi.Generated.Models;
using SdkHttpClient = Sdkwork.Im.BackendApi.Generated.Http.HttpClient;

namespace Sdkwork.Im.BackendApi.Generated.Api
{
    public class OpsApi
    {
        private readonly SdkHttpClient _client;

        public OpsApi(SdkHttpClient client)
        {
            _client = client;
        }

        /// <summary>
        /// Retrieve ops health
        /// </summary>
        public async Task<Dictionary<string, object>?> HealthRetrieveAsync()
        {
            return await _client.GetAsync<Dictionary<string, object>>(ApiPaths.BackendPath("/ops/health"));
        }

        /// <summary>
        /// Retrieve cluster state
        /// </summary>
        public async Task<Dictionary<string, object>?> ClusterRetrieveAsync()
        {
            return await _client.GetAsync<Dictionary<string, object>>(ApiPaths.BackendPath("/ops/cluster"));
        }

        /// <summary>
        /// Retrieve projection lag
        /// </summary>
        public async Task<Dictionary<string, object>?> LagRetrieveAsync()
        {
            return await _client.GetAsync<Dictionary<string, object>>(ApiPaths.BackendPath("/ops/lag"));
        }

        /// <summary>
        /// Retrieve replay status
        /// </summary>
        public async Task<Dictionary<string, object>?> ReplayStatusRetrieveAsync()
        {
            return await _client.GetAsync<Dictionary<string, object>>(ApiPaths.BackendPath("/ops/replay_status"));
        }

        /// <summary>
        /// Retrieve commercial readiness
        /// </summary>
        public async Task<Dictionary<string, object>?> CommercialReadinessRetrieveAsync()
        {
            return await _client.GetAsync<Dictionary<string, object>>(ApiPaths.BackendPath("/ops/commercial_readiness"));
        }

        /// <summary>
        /// Inspect runtime directory
        /// </summary>
        public async Task<Dictionary<string, object>?> RuntimeDirRetrieveAsync()
        {
            return await _client.GetAsync<Dictionary<string, object>>(ApiPaths.BackendPath("/ops/runtime_dir"));
        }

        /// <summary>
        /// List provider bindings
        /// </summary>
        public async Task<Dictionary<string, object>?> ProviderBindingsListAsync()
        {
            return await _client.GetAsync<Dictionary<string, object>>(ApiPaths.BackendPath("/ops/provider_bindings"));
        }

        /// <summary>
        /// Retrieve provider binding drift
        /// </summary>
        public async Task<Dictionary<string, object>?> ProviderBindingsDriftRetrieveAsync()
        {
            return await _client.GetAsync<Dictionary<string, object>>(ApiPaths.BackendPath("/ops/provider_bindings/drift"));
        }

        /// <summary>
        /// Retrieve diagnostics
        /// </summary>
        public async Task<Dictionary<string, object>?> DiagnosticsRetrieveAsync()
        {
            return await _client.GetAsync<Dictionary<string, object>>(ApiPaths.BackendPath("/ops/diagnostics"));
        }



    }
}
