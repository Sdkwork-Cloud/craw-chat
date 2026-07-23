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
        /// Read the tenant portal access snapshot
        /// </summary>
        public async Task<Sdkwork.Im.AppApi.Generated.Models.AccessRetrieveResponse?> AccessRetrieveAsync()
        {
            return await _client.GetAsync<Sdkwork.Im.AppApi.Generated.Models.AccessRetrieveResponse>(ApiPaths.AppPath("/portal/access"));
        }

        /// <summary>
        /// Read the tenant automation snapshot
        /// </summary>
        public async Task<Sdkwork.Im.AppApi.Generated.Models.AutomationRetrieveResponse?> AutomationRetrieveAsync()
        {
            return await _client.GetAsync<Sdkwork.Im.AppApi.Generated.Models.AutomationRetrieveResponse>(ApiPaths.AppPath("/portal/automation"));
        }

        /// <summary>
        /// Read the tenant conversations snapshot
        /// </summary>
        public async Task<Sdkwork.Im.AppApi.Generated.Models.ConversationSnapshotRetrieveResponse?> ConversationSnapshotRetrieveAsync()
        {
            return await _client.GetAsync<Sdkwork.Im.AppApi.Generated.Models.ConversationSnapshotRetrieveResponse>(ApiPaths.AppPath("/portal/conversations"));
        }

        /// <summary>
        /// Read the tenant dashboard snapshot
        /// </summary>
        public async Task<Sdkwork.Im.AppApi.Generated.Models.DashboardRetrieveResponse?> DashboardRetrieveAsync()
        {
            return await _client.GetAsync<Sdkwork.Im.AppApi.Generated.Models.DashboardRetrieveResponse>(ApiPaths.AppPath("/portal/dashboard"));
        }

        /// <summary>
        /// Read the tenant governance snapshot
        /// </summary>
        public async Task<Sdkwork.Im.AppApi.Generated.Models.GovernanceRetrieveResponse?> GovernanceRetrieveAsync()
        {
            return await _client.GetAsync<Sdkwork.Im.AppApi.Generated.Models.GovernanceRetrieveResponse>(ApiPaths.AppPath("/portal/governance"));
        }

        /// <summary>
        /// Read the tenant portal home snapshot
        /// </summary>
        public async Task<Sdkwork.Im.AppApi.Generated.Models.HomeRetrieveResponse?> HomeRetrieveAsync()
        {
            return await _client.GetAsync<Sdkwork.Im.AppApi.Generated.Models.HomeRetrieveResponse>(ApiPaths.AppPath("/portal/home"));
        }

        /// <summary>
        /// Read the tenant media snapshot
        /// </summary>
        public async Task<Sdkwork.Im.AppApi.Generated.Models.MediaRetrieveResponse?> MediaRetrieveAsync()
        {
            return await _client.GetAsync<Sdkwork.Im.AppApi.Generated.Models.MediaRetrieveResponse>(ApiPaths.AppPath("/portal/media"));
        }

        /// <summary>
        /// Read the tenant realtime snapshot
        /// </summary>
        public async Task<Sdkwork.Im.AppApi.Generated.Models.RealtimeRetrieveResponse?> RealtimeRetrieveAsync()
        {
            return await _client.GetAsync<Sdkwork.Im.AppApi.Generated.Models.RealtimeRetrieveResponse>(ApiPaths.AppPath("/portal/realtime"));
        }

        /// <summary>
        /// Read the current tenant workspace snapshot
        /// </summary>
        public async Task<Sdkwork.Im.AppApi.Generated.Models.WorkspaceRetrieveResponse?> WorkspaceRetrieveAsync()
        {
            return await _client.GetAsync<Sdkwork.Im.AppApi.Generated.Models.WorkspaceRetrieveResponse>(ApiPaths.AppPath("/portal/workspace"));
        }



    }
}
