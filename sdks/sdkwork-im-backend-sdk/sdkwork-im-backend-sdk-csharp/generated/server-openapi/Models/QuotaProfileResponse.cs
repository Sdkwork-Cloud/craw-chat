using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.Im.BackendApi.Generated.Models
{
    public class QuotaProfileResponse
    {
        public int? MaxConcurrentSessionsPerTenant { get; set; }
        public int? MaxInflightMessages { get; set; }
        public int? MaxPayloadBytes { get; set; }
        public int? MaxSubscriptionsPerSession { get; set; }
        public string? ProfileId { get; set; }
    }
}
