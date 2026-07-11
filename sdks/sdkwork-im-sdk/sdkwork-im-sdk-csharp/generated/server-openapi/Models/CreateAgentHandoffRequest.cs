using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.Im.Sdk.Generated.Models
{
    public class CreateAgentHandoffRequest
    {
        public string ConversationId { get; set; }
        public string TargetId { get; set; }
        public string TargetKind { get; set; }
        public string HandoffSessionId { get; set; }
        public string? HandoffReason { get; set; }
    }
}
