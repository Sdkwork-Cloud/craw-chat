using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.Im.AppApi.Generated.Models
{
    public class StreamFrame
    {
        public string? TenantId { get; set; }
        public string? StreamId { get; set; }
        public string? StreamType { get; set; }
        public string? ScopeKind { get; set; }
        public string? ScopeId { get; set; }
        public int? FrameSeq { get; set; }
        public string? FrameType { get; set; }
        public string? SchemaRef { get; set; }
        public string? Encoding { get; set; }
        public string? Payload { get; set; }
        public Sender? Sender { get; set; }
        public Dictionary<string, string>? Attributes { get; set; }
        public string? OccurredAt { get; set; }
    }
}
