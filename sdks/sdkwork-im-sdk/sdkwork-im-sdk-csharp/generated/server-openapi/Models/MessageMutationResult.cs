using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.Im.Sdk.Generated.Models
{
    public class MessageMutationResult
    {
        public string ConversationId { get; set; }
        public string MessageId { get; set; }
        public int MessageSeq { get; set; }
        public string EventId { get; set; }
    }
}
