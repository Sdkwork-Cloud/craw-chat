using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.Im.Sdk.Generated.Models
{
    public class PresenceView
    {
        public string TenantId { get; set; }
        public string PrincipalId { get; set; }
        public string PrincipalKind { get; set; }
        public string DeviceId { get; set; }
        public string Status { get; set; }
        public string UpdatedAt { get; set; }
    }
}
