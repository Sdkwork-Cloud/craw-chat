using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.Im.Sdk.Generated.Models
{
    public class CommitEnvelopeResponse
    {
        public string EventId { get; set; }
        public string TenantId { get; set; }
        public string EventType { get; set; }
        public int EventVersion { get; set; }
        public string AggregateType { get; set; }
        public string AggregateId { get; set; }
        public string ScopeType { get; set; }
        public string ScopeId { get; set; }
        public string OrderingKey { get; set; }
        public int OrderingSeq { get; set; }
        public string? CausationId { get; set; }
        public string? CorrelationId { get; set; }
        public string? IdempotencyKey { get; set; }
        public EventActor Actor { get; set; }
        public string OccurredAt { get; set; }
        public string CommittedAt { get; set; }
        public string? PayloadSchema { get; set; }
        public string Payload { get; set; }
        public string RetentionClass { get; set; }
        public string AuditClass { get; set; }
    }
}
