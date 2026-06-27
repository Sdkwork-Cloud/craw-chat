using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.Im.AppApi.Generated.Models
{
    public class NotificationRequestResponse
    {
        public string? TenantId { get; set; }
        public string? NotificationId { get; set; }
        public string? SourceEventId { get; set; }
        public string? SourceEventType { get; set; }
        public string? Category { get; set; }
        public string? Channel { get; set; }
        public string? RecipientId { get; set; }
        public string? RecipientKind { get; set; }
        public string? Status { get; set; }
        public string? Title { get; set; }
        public string? Body { get; set; }
        public string? Payload { get; set; }
        public string? RequestedAt { get; set; }
        public string? DispatchedAt { get; set; }
        public string? FailureReason { get; set; }
        public string? RequestKey { get; set; }
        public string? DeliveryStatus { get; set; }
        public string? ProofVersion { get; set; }
    }
}
