using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.Im.Sdk.Generated.Models
{
    public class CreateConversationRequest
    {
        public string? ConversationId { get; set; }
        public string ConversationType { get; set; }
        public string? GroupName { get; set; }
        public string? ClientRequestKey { get; set; }
        public bool? InitializeKnowledgebase { get; set; }
        public List<string>? MemberUserIds { get; set; }
        public List<ConversationAgentAssignment>? AgentAssignments { get; set; }
        public string? PolicyVersion { get; set; }
        public List<string>? CapabilityFlags { get; set; }
        public string? HistoryVisibility { get; set; }
        public string? RetentionPolicyRef { get; set; }
    }
}
