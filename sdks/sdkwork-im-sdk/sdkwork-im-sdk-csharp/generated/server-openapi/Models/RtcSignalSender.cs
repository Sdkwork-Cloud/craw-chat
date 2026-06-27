using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.Im.Sdk.Generated.Models
{
    public class RtcSignalSender
    {
        public string Id { get; set; }
        public string Kind { get; set; }
        public string? MemberId { get; set; }
        public string? DeviceId { get; set; }
        public string? SessionId { get; set; }
        public Dictionary<string, object> Metadata { get; set; }
    }
}
