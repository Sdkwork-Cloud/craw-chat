using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.Im.Sdk.Generated.Models
{
    public class UpdateConversationAgentsRequest
    {
        public int ExpectedGeneration { get; set; }
        public List<ConversationAgentAssignment> AgentAssignments { get; set; }
    }
}
