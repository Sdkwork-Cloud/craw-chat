using System;
using System.Collections.Generic;
using System.Threading.Tasks;
using Sdkwork.Im.Sdk.Generated.Models;
using SdkHttpClient = Sdkwork.Im.Sdk.Generated.Http.HttpClient;

namespace Sdkwork.Im.Sdk.Generated.Api
{
    public class PresenceApi
    {
        private readonly SdkHttpClient _client;

        public PresenceApi(SdkHttpClient client)
        {
            _client = client;
        }

        /// <summary>
        /// Publish current client route presence heartbeat
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.PresenceView?> HeartbeatCreateAsync(Sdkwork.Im.Sdk.Generated.Models.PresenceHeartbeatRequest body)
        {
            return await _client.PostAsync<Sdkwork.Im.Sdk.Generated.Models.PresenceView>(ApiPaths.ImPath("/presence/heartbeat"), body, null, null, "application/json");
        }

        /// <summary>
        /// Retrieve current principal presence
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.PresenceView?> MeRetrieveAsync()
        {
            return await _client.GetAsync<Sdkwork.Im.Sdk.Generated.Models.PresenceView>(ApiPaths.ImPath("/presence/me"));
        }



    }
}
