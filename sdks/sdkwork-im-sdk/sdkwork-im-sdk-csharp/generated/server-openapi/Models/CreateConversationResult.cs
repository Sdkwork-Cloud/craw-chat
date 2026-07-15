using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.Im.Sdk.Generated.Models
{
    public class CreateConversationResult
    {
        public string ConversationId { get; set; }
        public string EventId { get; set; }
        public string? RequestKey { get; set; }
        public string? DeliveryStatus { get; set; }
        public string? ProofVersion { get; set; }
        public string? KnowledgebaseInitialization { get; set; }
    }
}
