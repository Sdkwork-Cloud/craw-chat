using System;
using System.Collections.Generic;
using System.Threading.Tasks;
using Sdkwork.Im.AppApi.Generated.Models;
using SdkHttpClient = Sdkwork.Im.AppApi.Generated.Http.HttpClient;

namespace Sdkwork.Im.AppApi.Generated.Api
{
    public class PortalApi
    {
        private readonly SdkHttpClient _client;

        public PortalApi(SdkHttpClient client)
        {
            _client = client;
        }

        /// <summary>
        /// Read the tenant portal sign-in snapshot
        /// </summary>
        public async Task<Dictionary<string, object>?> AccessRetrieveAsync()
        {
            return await _client.GetAsync<Dictionary<string, object>>(ApiPaths.AppPath("/portal/access"));
        }

        /// <summary>
        /// Read the tenant automation snapshot
        /// </summary>
        public async Task<Dictionary<string, object>?> AutomationRetrieveAsync()
        {
            return await _client.GetAsync<Dictionary<string, object>>(ApiPaths.AppPath("/portal/automation"));
        }

        /// <summary>
        /// Read the tenant conversations snapshot
        /// </summary>
        public async Task<Dictionary<string, object>?> ConversationSnapshotRetrieveAsync()
        {
            return await _client.GetAsync<Dictionary<string, object>>(ApiPaths.AppPath("/portal/conversations"));
        }

        /// <summary>
        /// Read the tenant dashboard snapshot
        /// </summary>
        public async Task<Dictionary<string, object>?> DashboardRetrieveAsync()
        {
            return await _client.GetAsync<Dictionary<string, object>>(ApiPaths.AppPath("/portal/dashboard"));
        }

        /// <summary>
        /// Read the tenant governance snapshot
        /// </summary>
        public async Task<Dictionary<string, object>?> GovernanceRetrieveAsync()
        {
            return await _client.GetAsync<Dictionary<string, object>>(ApiPaths.AppPath("/portal/governance"));
        }

        /// <summary>
        /// Read the tenant portal home snapshot
        /// </summary>
        public async Task<Dictionary<string, object>?> HomeRetrieveAsync()
        {
            return await _client.GetAsync<Dictionary<string, object>>(ApiPaths.AppPath("/portal/home"));
        }

        /// <summary>
        /// Read the tenant media snapshot
        /// </summary>
        public async Task<Dictionary<string, object>?> MediaRetrieveAsync()
        {
            return await _client.GetAsync<Dictionary<string, object>>(ApiPaths.AppPath("/portal/media"));
        }

        /// <summary>
        /// Read the tenant realtime snapshot
        /// </summary>
        public async Task<Dictionary<string, object>?> RealtimeRetrieveAsync()
        {
            return await _client.GetAsync<Dictionary<string, object>>(ApiPaths.AppPath("/portal/realtime"));
        }

        /// <summary>
        /// Read the current tenant workspace snapshot
        /// </summary>
        public async Task<Sdkwork.Im.AppApi.Generated.Models.PortalWorkspaceView?> WorkspaceRetrieveAsync()
        {
            return await _client.GetAsync<Sdkwork.Im.AppApi.Generated.Models.PortalWorkspaceView>(ApiPaths.AppPath("/portal/workspace"));
        }



    }
}
