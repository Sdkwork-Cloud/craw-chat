using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.Im.AppApi.Generated.Models
{
    public class StartAgentResponseRequest
    {
        public string? ExecutionId { get; set; }
        public string? StreamId { get; set; }
        public string? StreamType { get; set; }
        public string? ConversationId { get; set; }
        public string? SchemaRef { get; set; }
        public string? MemberId { get; set; }
        public AgentSubject? Agent { get; set; }
    }
}
