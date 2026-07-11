using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.Im.Sdk.Generated.Models
{
    public class CreateThreadConversationRequest
    {
        public string ConversationId { get; set; }
        public string ParentConversationId { get; set; }
        public string RootMessageId { get; set; }
    }
}
