using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.Im.Sdk.Generated.Models
{
    public class RealtimeEventView
    {
        public string EventId { get; set; }
        public string Scope { get; set; }
        public string ScopeId { get; set; }
        public string EventType { get; set; }
        public string? Payload { get; set; }
        public string OccurredAt { get; set; }
    }
}
