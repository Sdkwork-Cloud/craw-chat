using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.Im.Sdk.Generated.Models
{
    public class MessageReactionMutationResult
    {
        public string TenantId { get; set; }
        public string ConversationId { get; set; }
        public string MessageId { get; set; }
        public string ReactionKey { get; set; }
        public int Count { get; set; }
        public string UpdatedAt { get; set; }
    }
}
