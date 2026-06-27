using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.Im.AppApi.Generated.Models
{
    public class StreamSession
    {
        public string? TenantId { get; set; }
        public string? StreamId { get; set; }
        public string? StreamType { get; set; }
        public string? ScopeKind { get; set; }
        public string? ScopeId { get; set; }
        public string? DurabilityClass { get; set; }
        public string? OrderingScope { get; set; }
        public string? SchemaRef { get; set; }
        public string? State { get; set; }
        public int? LastFrameSeq { get; set; }
        public int? LastCheckpointSeq { get; set; }
        public string? ResultMessageId { get; set; }
        public string? OpenedAt { get; set; }
        public string? ClosedAt { get; set; }
        public string? ExpiresAt { get; set; }
    }
}
