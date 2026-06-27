using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.Im.AppApi.Generated.Models
{
    public class AgentToolCall
    {
        public string? TenantId { get; set; }
        public string? ExecutionId { get; set; }
        public string? AgentId { get; set; }
        public string? ToolCallId { get; set; }
        public string? ToolName { get; set; }
        public string? ArgumentsPayload { get; set; }
        public string? ResultPayload { get; set; }
        public string? State { get; set; }
        public string? RequestedAt { get; set; }
        public string? CompletedAt { get; set; }
    }
}
