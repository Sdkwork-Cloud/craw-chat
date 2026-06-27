using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.Im.Sdk.Generated.Models
{
    public class CreateAgentDialogRequest
    {
        public string AgentId { get; set; }
        public string? ConversationId { get; set; }
        public string? Title { get; set; }
    }
}
