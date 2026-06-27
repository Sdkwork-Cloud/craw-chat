using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.Im.Sdk.Generated.Models
{
    public class StreamView
    {
        public string TenantId { get; set; }
        public string StreamId { get; set; }
        public string State { get; set; }
        public string OpenedAt { get; set; }
    }
}
