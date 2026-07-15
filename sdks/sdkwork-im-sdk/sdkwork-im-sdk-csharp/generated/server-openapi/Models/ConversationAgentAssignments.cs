using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.Im.Sdk.Generated.Models
{
    public class ConversationAgentAssignments
    {
        public int Generation { get; set; }
        public string Source { get; set; }
        public List<ConversationAgentAssignment> Agents { get; set; }
    }
}
