using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.Im.BackendApi.Generated.Models
{
    public class QuotaProfileResponse
    {
        public string MaxConcurrentSessionsPerTenant { get; set; }
        public string MaxInflightMessages { get; set; }
        public string MaxPayloadBytes { get; set; }
        public string MaxSubscriptionsPerSession { get; set; }
        public string ProfileId { get; set; }
    }
}
