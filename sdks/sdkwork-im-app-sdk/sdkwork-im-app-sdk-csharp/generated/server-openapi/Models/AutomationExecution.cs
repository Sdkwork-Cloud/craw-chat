using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.Im.AppApi.Generated.Models
{
    public class AutomationExecution
    {
        public string? TenantId { get; set; }
        public string? PrincipalId { get; set; }
        public string? PrincipalKind { get; set; }
        public string? ExecutionId { get; set; }
        public string? TriggerType { get; set; }
        public string? TargetKind { get; set; }
        public string? TargetRef { get; set; }
        public string? InputPayload { get; set; }
        public string? OutputPayload { get; set; }
        public string? State { get; set; }
        public int? RetryCount { get; set; }
        public string? RequestedAt { get; set; }
        public string? CompletedAt { get; set; }
        public string? FailureReason { get; set; }
    }
}
