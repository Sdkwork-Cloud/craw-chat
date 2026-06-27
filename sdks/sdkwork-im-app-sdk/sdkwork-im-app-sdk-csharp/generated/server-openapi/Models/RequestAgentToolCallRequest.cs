using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.Im.AppApi.Generated.Models
{
    public class RequestAgentToolCallRequest
    {
        public string? ExecutionId { get; set; }
        public string? ToolCallId { get; set; }
        public string? ToolName { get; set; }
        public string? ArgumentsPayload { get; set; }
    }
}
