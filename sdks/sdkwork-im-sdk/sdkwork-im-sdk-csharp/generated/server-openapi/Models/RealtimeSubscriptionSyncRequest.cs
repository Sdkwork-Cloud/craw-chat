using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.Im.Sdk.Generated.Models
{
    public class RealtimeSubscriptionSyncRequest
    {
        public string? DeviceId { get; set; }
        public List<string>? Conversations { get; set; }
        public List<RealtimeSubscriptionItemInput>? Items { get; set; }
    }
}
