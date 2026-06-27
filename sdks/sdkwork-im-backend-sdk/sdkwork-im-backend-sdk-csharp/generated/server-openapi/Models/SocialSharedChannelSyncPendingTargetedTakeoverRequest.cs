using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.Im.BackendApi.Generated.Models
{
    public class SocialSharedChannelSyncPendingTargetedTakeoverRequest
    {
        public bool? AllowLegacyUntracked { get; set; }
        public List<string>? RequestKeys { get; set; }
    }
}
