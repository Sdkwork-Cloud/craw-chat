using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.Im.AppApi.Generated.Models
{
    public class RequestAutomationExecution
    {
        public string? ExecutionId { get; set; }
        public string? TriggerType { get; set; }
        public string? TargetKind { get; set; }
        public string? TargetRef { get; set; }
        public string? InputPayload { get; set; }
    }
}
