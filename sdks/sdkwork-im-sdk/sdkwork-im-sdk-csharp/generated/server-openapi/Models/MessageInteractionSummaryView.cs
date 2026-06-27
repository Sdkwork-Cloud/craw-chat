using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.Im.Sdk.Generated.Models
{
    public class MessageInteractionSummaryView
    {
        public string TenantId { get; set; }
        public string ConversationId { get; set; }
        public string MessageId { get; set; }
        public int MessageSeq { get; set; }
        public int TotalReactionCount { get; set; }
        public List<MessageReactionCountView> ReactionCounts { get; set; }
        public MessagePinView? Pin { get; set; }
    }
}
